use galaxy_ops::accessor::accessor_for_default;
use galaxy_ops::error::MainResult;
use galaxy_ops::infra::configure_dfx_logging;
use galaxy_ops::module::proj::ModProject;
use galaxy_ops::module::spec::make_mod_spec_example;
use galaxy_ops::ops_prj::proj::OpsProject;
use galaxy_ops::project::load_project_global_value;
use galaxy_ops::system::proj::SysProject;
use galaxy_ops::types::{InsUpdateable, Localizable, LocalizeOptions, RefUpdateable};
use orion_common::serde::Persistable;
use orion_error::{ErrorConv, ErrorOwe};
use orion_infra::path::make_new_path;
use orion_variate::update::DownloadOptions;
use orion_variate::vars::ValueDict;

use crate::args::{GInsCmd, ModCmd, SysCmd};
use galaxy_ops::module::ModelSTD;
use inquire::Select;

fn ia_model_std() -> MainResult<ModelSTD> {
    let support_models = ModelSTD::support();

    // 准备选项列表
    let options: Vec<String> = support_models
        .iter()
        .map(|model| format!("{model}"))
        .collect();

    // 添加使用当前系统的选项
    let all_options = options;

    // 检查是否在测试环境中
    if std::env::var("TEST_MODE").is_ok() {
        // 在测试环境中，自动选择第一个支持的模式
        if let Some(first_model) = support_models.first() {
            return Ok(first_model.clone());
        } else {
            return Ok(ModelSTD::from_cur_sys());
        }
    }

    // 检查是否在测试环境中
    if std::env::var("TEST_MODE").is_ok() {
        // 在测试环境中，自动选择第一个支持的模式
        if let Some(first_model) = support_models.first() {
            return Ok(first_model.clone());
        } else {
            return Ok(ModelSTD::from_cur_sys());
        }
    }

    let selection = Select::new("请选择系统型号配置:", all_options.clone())
        .prompt()
        .unwrap();

    // 从预定义选项中选择
    let index = all_options.iter().position(|s| s == &selection).unwrap();
    if index < support_models.len() {
        Ok(support_models[index].clone())
    } else {
        Ok(ModelSTD::from_cur_sys()) // 兜底处理
    }
}

pub async fn do_mod_cmd(cmd: ModCmd) -> MainResult<()> {
    let current_dir = std::env::current_dir().expect("无法获取当前目录");
    match cmd {
        ModCmd::Example(_args) => {
            let spec = make_mod_spec_example().err_conv()?;
            spec.save_to(&std::path::PathBuf::from("./"), None)
                .owe_res()?;
        }
        ModCmd::New(args) => {
            let project_dir = current_dir.join(args.name());
            std::fs::create_dir(&project_dir).owe_res()?;
            configure_dfx_logging(&args);
            let spec = ModProject::make_new(&project_dir, args.name.as_str()).err_conv()?;
            spec.save().err_conv()?;
        }
        ModCmd::Update(args) => {
            configure_dfx_logging(&args);
            let spec = ModProject::load(&current_dir).err_conv()?;
            let options = DownloadOptions::from((args.force, ValueDict::default()));
            let accessor = accessor_for_default();
            spec.update_local(accessor, &current_dir, &options)
                .await
                .err_conv()?;
        }
        ModCmd::Localize(args) => {
            configure_dfx_logging(&args);
            let spec = ModProject::load(&current_dir).err_conv()?;
            let dict = load_project_global_value(spec.root_local(), args.value())?;
            spec.localize(None, LocalizeOptions::new(dict, args.use_default_value))
                .await
                .err_conv()?;
        }
    }
    Ok(())
}

pub async fn do_sys_cmd(cmd: SysCmd) -> MainResult<()> {
    let current_dir = std::env::current_dir().expect("无法获取当前目录");
    match cmd {
        SysCmd::New(args) => {
            let new_prj = current_dir.join(args.name());
            make_new_path(&new_prj).owe_res()?;
            let model_in = ia_model_std()?;
            let spec = SysProject::make_new(&new_prj, args.name(), model_in).err_conv()?;
            spec.save().err_conv()?;
        }
        SysCmd::Update(args) => {
            configure_dfx_logging(&args);
            let options = DownloadOptions::from((args.force, ValueDict::default()));
            let spec = SysProject::load(&current_dir).err_conv()?;
            let accessor = accessor_for_default();
            spec.update_local(accessor, &current_dir, &options)
                .await
                .err_conv()?;
        }
        SysCmd::Localize(args) => {
            configure_dfx_logging(&args);
            let spec = SysProject::load(&current_dir).err_conv()?;
            let dict = load_project_global_value(spec.root_local(), args.value())?;
            spec.localize(LocalizeOptions::new(dict, args.use_default_value))
                .await
                .err_conv()?;
        }
    }
    Ok(())
}

pub async fn do_ins_cmd(cmd: GInsCmd) -> MainResult<()> {
    let current_dir = std::env::current_dir().expect("无法获取当前目录");
    match cmd {
        GInsCmd::New(args) => {
            let new_prj = current_dir.join(args.name());
            make_new_path(&new_prj).owe_res()?;
            let spec = OpsProject::make_new(&new_prj, args.name()).err_conv()?;
            spec.save().err_conv()?;
        }
        GInsCmd::Import(args) => {
            configure_dfx_logging(&args);
            let options = DownloadOptions::from((args.force, ValueDict::default()));
            let mut prj = OpsProject::load(&current_dir).err_conv()?;
            let accessor = accessor_for_default();
            prj.import_sys(accessor, args.path(), &options)
                .await
                .err_conv()?;
        }
        GInsCmd::Update(dfx) => {
            configure_dfx_logging(&dfx);
            let options = DownloadOptions::from((dfx.force, ValueDict::default()));
            let spec = OpsProject::load(&current_dir).err_conv()?;
            let accessor = accessor_for_default();
            spec.update_local(accessor, &current_dir, &options)
                .await
                .err_conv()?;
        }
        GInsCmd::Setting(args) => {
            configure_dfx_logging(&args);
            let spec = OpsProject::load(&current_dir).err_conv()?;
            spec.ia_setting()?;
        }
        GInsCmd::Mod(mod_cmd) => {
            do_mod_cmd(mod_cmd).await?;
        }
        GInsCmd::Sys(sys_cmd) => {
            do_sys_cmd(sys_cmd).await?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::{GInsCmd, ImportArgs, NewArgs, SettingArgs, UpdateArgs};
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_do_ins_cmd_new_success() {
        let temp_dir = tempdir().unwrap();
        let project_path = temp_dir.path().join("test_project");

        // Create test command
        let cmd = GInsCmd::New(NewArgs {
            name: "test_project".to_string(),
        });

        // Set current directory
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Execute the command
        let result = do_ins_cmd(cmd).await;

        // Should create the project directory and files
        assert!(project_path.exists());
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_do_ins_cmd_import_no_project() {
        let temp_dir = tempdir().unwrap();

        // Create test command
        let cmd = GInsCmd::Import(ImportArgs {
            debug: 0,
            log: None,
            force: 0,
            path: "/test/path".to_string(),
        });

        // Set current directory to empty temp directory
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Should fail gracefully when no project exists
        let result = do_ins_cmd(cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_ins_cmd_import_with_force() {
        let temp_dir = tempdir().unwrap();

        // Create test command with force
        let cmd = GInsCmd::Import(ImportArgs {
            debug: 2,
            log: Some("import=debug".to_string()),
            force: 1,
            path: "/test/path".to_string(),
        });

        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Should fail gracefully (no project exists)
        let result = do_ins_cmd(cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_ins_cmd_update_no_project() {
        let temp_dir = tempdir().unwrap();

        // Create test command
        let cmd = GInsCmd::Update(UpdateArgs {
            debug: 1,
            log: None,
            force: 0,
        });

        // Set current directory to empty temp directory
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Should fail gracefully when no project exists
        let result = do_ins_cmd(cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_ins_cmd_update_with_force() {
        let temp_dir = tempdir().unwrap();

        // Create test command with force
        let cmd = GInsCmd::Update(UpdateArgs {
            debug: 2,
            log: Some("all=info".to_string()),
            force: 3,
        });

        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Should fail gracefully (no project exists)
        let result = do_ins_cmd(cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_ins_cmd_setting_no_project() {
        let temp_dir = tempdir().unwrap();

        // Create test command
        let cmd = GInsCmd::Setting(SettingArgs {
            debug: 1,
            log: Some("setting=debug".to_string()),
        });

        // Set current directory to empty temp directory
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Should fail gracefully when no project exists
        let result = do_ins_cmd(cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_ins_cmd_setting_with_debug() {
        let temp_dir = tempdir().unwrap();

        // Create test command with debug
        let cmd = GInsCmd::Setting(SettingArgs {
            debug: 2,
            log: Some("system=debug".to_string()),
        });

        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Should fail gracefully (no project exists)
        let result = do_ins_cmd(cmd).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_configure_dfx_logging_compiles() {
        // Test that the logging configuration compiles
        let args = UpdateArgs {
            debug: 1,
            log: Some("test=debug".to_string()),
            force: 0,
        };

        // This should not panic
        configure_dfx_logging(&args);
    }

    #[test]
    fn test_configure_dfx_logging_with_import_args() {
        // Test logging configuration with import args
        let args = ImportArgs {
            debug: 2,
            log: Some("import=debug".to_string()),
            force: 1,
            path: "/test/path".to_string(),
        };

        // This should not panic
        configure_dfx_logging(&args);
    }

    #[test]
    fn test_configure_dfx_logging_with_setting_args() {
        // Test logging configuration with setting args
        let args = SettingArgs {
            debug: 3,
            log: Some("setting=debug".to_string()),
        };

        // This should not panic
        configure_dfx_logging(&args);
    }

    #[tokio::test]
    async fn test_current_dir_handling() {
        // Test that current directory handling works
        let temp_dir = tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        // Change to temp directory
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Try to get current directory
        let current_dir = std::env::current_dir();
        assert!(current_dir.is_ok());

        // Normalize both paths for comparison (handles macOS /private prefix issue)
        let current_path = current_dir.unwrap();
        let temp_path = temp_dir.path();

        // Use std::fs::canonicalize to get the real path for both
        let normalized_current = std::fs::canonicalize(&current_path).unwrap_or(current_path);
        let normalized_temp = std::fs::canonicalize(temp_path).unwrap_or(temp_path.to_path_buf());

        assert_eq!(normalized_current, normalized_temp);

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();
    }

    #[tokio::test]
    async fn test_download_options_creation() {
        // Test DownloadOptions creation with different force levels
        let _options1 = DownloadOptions::from((0, ValueDict::default()));
        let _options2 = DownloadOptions::from((1, ValueDict::default()));
        let _options3 = DownloadOptions::from((2, ValueDict::default()));
        let _options4 = DownloadOptions::from((3, ValueDict::default()));

        // Should create without panicking
        // Note: We can't easily test the internal state without accessors
        // But we can verify they don't panic
        // This test validates DownloadOptions creation - no assertion needed
    }

    #[tokio::test]
    async fn test_error_handling() {
        // Test that errors are handled gracefully
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Try to update in empty directory
        let cmd = GInsCmd::Update(UpdateArgs {
            debug: 0,
            log: None,
            force: 0,
        });

        let result = do_ins_cmd(cmd).await;
        assert!(result.is_err());

        // Error message should be meaningful
        let error_msg = result.unwrap_err().to_string();
        assert!(!error_msg.is_empty());
    }
}
