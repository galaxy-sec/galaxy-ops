use std::path::{Path, PathBuf};

use crate::ops_prj::install::SystemPackageInstaller;
use orion_conf::Configable;
use orion_error::{ErrorOwe, ErrorWith};
use orion_infra::path::ensure_path;
use orion_variate::{
    addr::Address,
    types::ResourceDownloader,
    update::DownloadOptions,
    vars::{EnvEvalable, ValueDict, VarCollection},
};

use crate::{
    artifact::types::{build_pkg, convert_addr},
    const_vars::{SYS_VALUE_FILE, SYS_VARS_YML},
    error::MainResult,
    ops_prj::{project::OpsProject, system::OpsSystem},
    types::Accessor,
};

impl OpsProject {
    pub async fn import_sys(
        &mut self,
        accessor: Accessor,
        path: &str,
        up_opt: &DownloadOptions,
    ) -> MainResult<()> {
        // 1. 解析地址
        let addr = convert_addr(path);

        // 2. 更新到本地目录
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

        // 3. 创建安装器并准备包
        let installer = SystemPackageInstaller::new(self.paths().clone()).with_pkg_path(pkg_path);

        let package = build_pkg(path);
        let sys_src = installer.prepare_package(package)?;

        // 4. 导入到工作目录
        let ops_target_system = installer.install_system_package(&sys_src)?;

        let ops_sys = OpsSystem::new(ops_target_system.spec().define().clone(), addr);
        self.import_ops_sys(ops_sys);
        self.save()?;

        // 5. 提供系统包的信息， 包组所有组件。
        Ok(())
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

        let value_file = value_path.join(SYS_VALUE_FILE);
        if value_file.exists() && value_link.exists() {
            std::fs::remove_file(value_link).owe_res()?;
        }

        let vars_vec = VarCollection::from_conf(vars_path).owe_res()?;
        let mut vals_dict = if value_file.exists() {
            ValueDict::from_conf(&value_file).owe_res()?
        } else {
            ValueDict::default()
        };

        // 通过交互模式设定vars的值
        println!("Setting variables for {system_name}");

        for var in vars_vec.system_vars() {
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
                // 非交互模式，如果已有值则保留，否则使用默认值
                if let Some(existing_value) = vals_dict.get(var.name()) {
                    existing_value.to_string()
                } else {
                    var.value().to_string()
                }
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
            let vars_path = self
                .root_local()
                .join(i.sys().name())
                .join("sys")
                .join(SYS_VARS_YML);

            let value_path = self.root_local().join("values").join(i.sys().name());
            ensure_path(&value_path).owe_res()?;

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
