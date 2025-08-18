use std::path::{Path, PathBuf};

use fs_extra::dir::{CopyOptions, move_dir};
use orion_common::serde::Configable;
use orion_error::{ErrorOwe, ErrorWith, UvsConfFrom};
use orion_infra::path::{ensure_path, make_clean_path};
use orion_variate::{
    addr::Address,
    archive::decompress,
    types::ResourceDownloader,
    update::DownloadOptions,
    vars::{EnvEvalable, ValueDict, VarCollection},
};

use crate::{
    artifact::types::{PackageType, build_pkg, convert_addr},
    error::{MainReason, MainResult, ToErr},
    ops_prj::{proj::OpsProject, system::OpsSystem},
    system::spec::SysModelSpec,
    types::Accessor,
};

impl OpsProject {
    pub async fn import_sys(
        &mut self,
        accessor: Accessor,
        path: &str,
        up_opt: &DownloadOptions,
    ) -> MainResult<SysModelSpec> {
        // 1. 解析地址
        let addr = convert_addr(path);

        // 2.更新到本地目路
        // 本地路径： ${HOME}/ds-build/
        let work_path = PathBuf::from(
            "${HOME}/ds-package"
                .to_string()
                .env_eval(&ValueDict::default()),
        );

        let pkg_path = if let Address::Local(local) = addr.clone() {
            PathBuf::from(local.path())
        } else {
            let up_unit = accessor
                .download_to_local(&addr, &work_path, up_opt)
                .await
                .owe_data()?;
            up_unit.position().clone()
        };
        let package = build_pkg(path);
        let sys_src = match package {
            //tar.gz ,tgz
            PackageType::Bin(bin_package) => {
                let out_path = work_path.join(bin_package.name());
                make_clean_path(&out_path).owe_res()?;
                decompress(&pkg_path, out_path.clone())
                    .owe_sys()
                    .want("decompress tar.gz")
                    .with(pkg_path.display().to_string())?;
                out_path
            }
            PackageType::Git(_git_package) => pkg_path.to_path_buf(),
        };
        let sys_spec = SysModelSpec::load_from(&sys_src.join("sys"))?;

        let ops_sys = OpsSystem::new(sys_spec.define().clone(), addr);
        self.import_ops_sys(ops_sys);
        // 3.获得sys pakage

        // 4. 导入到 工作目录
        let sys_dst_root = self.root_local();
        //if let Some(last_name) = sys_src.iter().last() {
        if let Some(last_name) = sys_src.iter().next_back() {
            let sys_dst_path = sys_dst_root.join(last_name);
            let sys_new_path = sys_dst_root.join(sys_spec.define().name());
            if sys_dst_path.exists() {
                std::fs::remove_dir_all(&sys_dst_path).owe_res()?;
            }
            if sys_new_path.exists() {
                std::fs::remove_dir_all(&sys_new_path).owe_res()?;
            }
            move_dir(sys_src, sys_dst_root, &CopyOptions::new()).owe_res()?;
            std::fs::rename(sys_dst_path, sys_new_path).owe_res()?;
        } else {
            MainReason::from_conf(format!(
                "import package failed, bad path: {}",
                sys_src.display()
            ))
            .to_err();
        }
        self.save()?;
        // 5. 提供系统包的信息， 包组所有组件。
        Ok(sys_spec)
    }
    pub fn ia_setting_interactive(&self) -> MainResult<()> {
        self.ia_setting(true)
    }

    pub fn process_system_vars(
        vars_path: &Path,
        value_path: &Path,
        value_link: &Path,
        system_name: &str,
        interactive: bool,
    ) -> MainResult<()> {
        use inquire::{Confirm, Text};

        let value_file = value_path.join("value.yml");
        if value_file.exists() {
            println!("value file exists ,use it");
            if !value_link.exists() {
                std::os::unix::fs::symlink(value_path, value_link)
                    .owe_res()
                    .with(value_link)?;
            }
            return Ok(());
        }

        let vars_vec = VarCollection::from_conf(vars_path).owe_res()?;
        let mut vals_dict = if value_file.exists() {
            ValueDict::from_conf(&value_file).owe_res()?
        } else {
            ValueDict::default()
        };

        // 通过交互模式设定vars的值
        println!("Setting variables for {system_name}");

        for var in vars_vec.vars() {
            if !var.is_mutable() {
                continue;
            }
            let prompt = if let Some(desp) = var.desp() {
                format!("{}\n{desp}", var.name())
            } else {
                var.name().to_string()
            };
            let mut default_value = var.value().clone();
            let value_str = if interactive {
                Text::new(&prompt)
                    .with_default(&var.value().to_string())
                    .prompt()
                    .owe_data()?
            } else {
                // 非交互模式，使用默认值
                var.value().to_string()
            };
            default_value.update_by_str(value_str.as_str()).owe_data()?;
            vals_dict.insert(var.name().to_string(), default_value);
        }

        // 如果用户确认保存更改
        let should_save = if interactive {
            Confirm::new("Do you want to save these changes?")
                .prompt()
                .owe_data()?
        } else {
            // 非交互模式，自动保存
            true
        };
        if should_save {
            // 保存修改后的vars到文件
            // vars.save_to_file(&vars_path)?; // 假设的方法
            println!("Changes saved to {}", value_file.display());
            vals_dict.save_conf(&value_file).owe_res()?;
        }
        if !value_link.exists() {
            std::os::unix::fs::symlink(value_path, value_link)
                .owe_res()
                .with(value_link)?;
        }

        Ok(())
    }

    pub fn ia_setting(&self, interactive: bool) -> MainResult<()> {
        for i in self.ops_target().iter() {
            let vars_path = self.root_local().join(i.sys().name()).join("sys/vars.yml");

            let value_path = self.root_local().join("values").join(i.sys().name());
            ensure_path(&value_path).owe_res()?;
            //let value_file = value_path.join("value.yml");

            let value_link = self.root_local().join(i.sys().name()).join("values");
            //.join("value.yml");

            Self::process_system_vars(
                &vars_path,
                &value_path,
                &value_link,
                i.sys().name(),
                interactive,
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use orion_error::TestAssert;
    use orion_variate::{tools::test_init, update::DownloadOptions, vars::ValueDict};
    use std::path::PathBuf;
    use tempfile::TempDir;

    use crate::{accessor::accessor_for_test, const_vars::EXAMPLE_ROOT};

    use super::*;

    #[ignore = "need interactive run"]
    #[tokio::test]
    async fn import_pkg() {
        test_init();
        let prj_path = PathBuf::from(EXAMPLE_ROOT).join("dev-mac-env");
        let mut project = OpsProject::load(&prj_path).assert();
        let path = "${HOME}/ds-build/mac-devkit-0.1.6.tar.gz"
            .to_string()
            .env_eval(&ValueDict::default());
        let accessor = accessor_for_test();
        let sys_spec = project
            .import_sys(accessor, path.as_str(), &DownloadOptions::for_test())
            .await
            .assert();
        println!("{}", serde_json::to_string(&sys_spec).assert());
    }

    #[test]
    fn test_process_system_vars_non_interactive() {
        test_init();
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create test paths
        let vars_path = root.join("sys/vars.yml");
        let value_path = root.join("values/test");
        let value_file = root.join("values/test/value.yml");
        let value_link = root.join("test/values");

        // Create necessary directories
        std::fs::create_dir_all(vars_path.parent().unwrap()).unwrap();
        std::fs::create_dir_all(&value_path).unwrap();
        std::fs::create_dir_all(value_link.parent().unwrap()).unwrap();

        // Create a sample vars.yml file
        let vars_content = r#"
vars:
  - name: "test_var"
    value: "default_value"
    mutable: true
    desp: "A test variable"
  - name: "immutable_var"
    value: "immutable_value"
    mutable: false
    desp: "An immutable variable"
"#;
        std::fs::write(&vars_path, vars_content).unwrap();

        // Create a sample value.yml file
        let value_content = r#"
test_var: "existing_value"
immutable_var: "existing_immutable"
"#;
        std::fs::write(&value_file, value_content).unwrap();

        // Test the function in non-interactive mode
        let result = super::OpsProject::process_system_vars(
            &vars_path,
            &value_path,
            &value_link,
            "test_system",
            false,
        );

        // Verify the function succeeds
        result.assert();

        // Verify the symlink was created
        assert!(value_link.exists());

        // Read and verify the value file was not modified (since we used the existing one)
        let updated_vals = ValueDict::from_conf(&value_file).unwrap();
        assert_eq!(
            updated_vals.get("test_var").unwrap().to_string(),
            "existing_value"
        );
        assert_eq!(
            updated_vals.get("immutable_var").unwrap().to_string(),
            "existing_immutable"
        );
    }

    #[test]
    fn test_process_system_vars_no_existing_file() {
        test_init();
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create test paths
        let vars_path = root.join("sys/vars.yml");
        let value_path = root.join("values/test");
        let value_file = root.join("values/test/value.yml");
        let value_link = root.join("test/values");

        // Create necessary directories
        std::fs::create_dir_all(vars_path.parent().unwrap()).unwrap();
        std::fs::create_dir_all(&value_path).unwrap();
        std::fs::create_dir_all(value_link.parent().unwrap()).unwrap();

        // Create a sample vars.yml file
        let vars_content = r#"
vars:
  - name: "test_var"
    value: "default_value"
    mutable: true
    desp: "A test variable"
  - name: "immutable_var"
    value: "immutable_value"
    mutable: false
    desp: "An immutable variable"
"#;
        std::fs::write(&vars_path, vars_content).unwrap();

        // DO NOT create the value file initially to test variable processing

        // Test the function in non-interactive mode with no existing value file
        super::OpsProject::process_system_vars(
            &vars_path,
            &value_path,
            &value_link,
            "test_system",
            false,
        )
        .unwrap();

        // The value file should be created by the function
        assert!(value_path.exists());
        // The symlink should be created
        assert!(value_link.exists());

        // Read and verify the value file has default values
        let updated_vals = ValueDict::from_conf(&value_file).unwrap();
        assert_eq!(
            updated_vals.get("test_var").unwrap().to_string(),
            "default_value"
        );
        assert_eq!(
            updated_vals.get("immutable_var").unwrap().to_string(),
            "immutable_value"
        );
    }

    #[test]
    fn test_process_system_vars_existing_value_file() {
        test_init();
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create test paths
        let vars_path = root.join("sys/vars.yml");
        let value_path = root.join("values/test");
        let _value_file = root.join("values/test/value.yml");
        let value_link = root.join("test/values");

        // Create necessary directories
        std::fs::create_dir_all(vars_path.parent().unwrap()).unwrap();
        std::fs::create_dir_all(&value_path).unwrap();
        std::fs::create_dir_all(value_link.parent().unwrap()).unwrap();

        // Create a sample vars.yml file
        // Create existing value file
        let value_file = root.join("values/test/value.yml");
        std::fs::write(&value_file, "test_key: test_value").unwrap();

        // Test the function - it should return early due to existing value file
        let result = super::OpsProject::process_system_vars(
            &vars_path,
            &value_path,
            &value_link,
            "test_system",
            false,
        );

        // Verify the function succeeds
        result.assert();

        // Verify the symlink was created
        assert!(value_link.exists());

        // Verify vars.yml wasn't created (since we returned early)
        assert!(!vars_path.exists());
    }

    #[test]
    fn test_process_system_vars_symlink_creation() {
        test_init();
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create test paths
        let vars_path = root.join("sys/vars.yml");
        let value_path = root.join("values/test");
        let value_file = root.join("values/test/value.yml");
        let value_link = root.join("test/values");

        // Create necessary directories
        std::fs::create_dir_all(vars_path.parent().unwrap()).unwrap();
        std::fs::create_dir_all(&value_path).unwrap();
        std::fs::create_dir_all(value_link.parent().unwrap()).unwrap();

        // Create a sample vars.yml file
        let vars_content = r#"
vars:
  - name: "test_var"
    value: "default_value"
    mutable: true
    desp: "A test variable"
  - name: "immutable_var"
    value: "immutable_value"
    mutable: false
    desp: "An immutable variable"
"#;
        std::fs::write(&vars_path, vars_content).unwrap();

        // Create a sample value.yml file
        let value_content = r#"
test_var: "existing_value"
immutable_var: "existing_immutable"
"#;
        std::fs::write(&value_file, value_content).unwrap();

        // Test function
        super::OpsProject::process_system_vars(
            &vars_path,
            &value_path,
            &value_link,
            "test_system",
            false,
        )
        .unwrap();

        // Verify symlink was created
        assert!(value_link.exists());

        // Verify symlink points to the correct target
        let link_target = std::fs::read_link(&value_link).unwrap();
        assert_eq!(link_target, value_path);
    }

    #[test]
    fn test_process_system_vars_empty_vars_file() {
        test_init();
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create test paths
        let vars_path = root.join("sys/vars.yml");
        let value_path = root.join("values/test");
        let _value_file = root.join("values/test/value.yml");
        let value_link = root.join("test/values");

        // Create necessary directories
        std::fs::create_dir_all(vars_path.parent().unwrap()).unwrap();
        std::fs::create_dir_all(&value_path).unwrap();
        std::fs::create_dir_all(value_link.parent().unwrap()).unwrap();

        // Create a sample vars.yml file
        // Create an empty vars.yml file (no variables)
        let vars_content = r#"
vars: []
"#;
        std::fs::write(&vars_path, vars_content).unwrap();

        // Test function should work even with empty vars
        super::OpsProject::process_system_vars(
            &vars_path,
            &value_path,
            &value_link,
            "test_system",
            false,
        )
        .unwrap();

        // Verify value file was created
        assert!(value_path.exists());
        // Verify symlink was created
        assert!(value_link.exists());
    }

    #[test]
    fn test_process_system_vars_all_immutable_vars() {
        test_init();
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create test paths
        let vars_path = root.join("sys/vars.yml");
        let value_path = root.join("values/test");
        let value_link = root.join("test/values");

        // Create necessary directories
        std::fs::create_dir_all(vars_path.parent().unwrap()).unwrap();
        std::fs::create_dir_all(&value_path).unwrap();
        std::fs::create_dir_all(value_link.parent().unwrap()).unwrap();

        // Create a sample vars.yml file
        // Create vars.yml file with only immutable variables
        let vars_content = r#"
vars:
  - name: "immutable_var_1"
    value: "immutable_value_1"
    mutable: false
    desp: "An immutable variable"
  - name: "immutable_var_2"
    value: "immutable_value_2"
    mutable: false
    desp: "Another immutable variable"
"#;
        std::fs::write(&vars_path, vars_content).unwrap();

        // Test function
        super::OpsProject::process_system_vars(
            &vars_path,
            &value_path,
            &value_link,
            "test_system",
            false,
        )
        .unwrap();

        // Verify value file was created with default values
        let value_file = root.join("values/test/value.yml");
        assert!(value_file.exists());
        let vals_dict = ValueDict::from_conf(&value_file).unwrap();

        // Should contain default values from vars file
        assert_eq!(
            vals_dict.get("immutable_var_1").unwrap().to_string(),
            "immutable_value_1"
        );
        assert_eq!(
            vals_dict.get("immutable_var_2").unwrap().to_string(),
            "immutable_value_2"
        );
    }

    #[test]
    fn test_process_system_vars_error_handling() {
        test_init();
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create test paths
        let vars_path = root.join("sys/vars.yml");
        let value_path = root.join("values/test/value.yml");
        let value_link = root.join("test/values");

        // Create only parent directories
        std::fs::create_dir_all(value_path.parent().unwrap()).unwrap();
        std::fs::create_dir_all(value_link.parent().unwrap()).unwrap();
        // Intentionally do NOT create vars_path parent directory

        // Function should handle missing vars file gracefully
        let result = super::OpsProject::process_system_vars(
            &vars_path,
            &value_path,
            &value_link,
            "test_system",
            false,
        );

        // Should return an error
        assert!(result.is_err());
    }

    #[test]
    fn test_process_system_vars_symlink_already_exists() {
        test_init();
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create test paths
        let vars_path = root.join("sys/vars.yml");
        let value_path = root.join("values/test");
        let value_file = root.join("values/test/value.yml");
        let value_link = root.join("test/values");

        // Create necessary directories
        std::fs::create_dir_all(vars_path.parent().unwrap()).unwrap();
        std::fs::create_dir_all(&value_path).unwrap();
        std::fs::create_dir_all(value_link.parent().unwrap()).unwrap();

        // Create a sample vars.yml file
        let vars_content = r#"
vars:
  - name: "test_var"
    value: "default_value"
    mutable: true
    desp: "A test variable"
"#;
        std::fs::write(&vars_path, vars_content).unwrap();

        let value_content = r#"
test_var: "existing_value"
"#;
        std::fs::write(&value_file, value_content).unwrap();

        // Pre-create the symlink
        std::os::unix::fs::symlink(&value_path, &value_link).unwrap();

        // Test function should not fail when symlink already exists
        super::OpsProject::process_system_vars(
            &vars_path,
            &value_path,
            &value_link,
            "test_system",
            false,
        )
        .unwrap();

        // Verify symlink still exists
        assert!(value_link.exists());
        // Verify symlink points to correct target
        let link_target = std::fs::read_link(&value_link).unwrap();
        assert!(link_target.exists());
    }
}
