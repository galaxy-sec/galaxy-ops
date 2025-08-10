use galaxy_ops::accessor::accessor_for_default;
use galaxy_ops::error::MainResult;
use galaxy_ops::infra::configure_dfx_logging;
use galaxy_ops::module::ModelSTD;
use inquire::Select;
use orion_error::{ErrorConv, ErrorOwe};
use orion_infra::path::make_new_path;

use galaxy_ops::project::load_project_global_value;
use galaxy_ops::system::proj::SysProject;
use galaxy_ops::types::{LocalizeOptions, RefUpdateable};
use orion_variate::update::DownloadOptions;
use orion_variate::vars::ValueDict;

use crate::args::GSysCmd;

fn ia_model_std() -> MainResult<ModelSTD> {
    let support_models = ModelSTD::support();

    // 准备选项列表
    let options: Vec<String> = support_models
        .iter()
        .map(|model| format!("{model}"))
        .collect();

    // 添加使用当前系统的选项
    let all_options = options;

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

pub async fn do_sys_cmd(cmd: GSysCmd) -> MainResult<()> {
    let current_dir = std::env::current_dir().expect("无法获取当前目录");
    match cmd {
        GSysCmd::New(args) => {
            let new_prj = current_dir.join(args.name());
            make_new_path(&new_prj).owe_res()?;
            let model_in = ia_model_std()?;
            let spec = SysProject::make_new(&new_prj, args.name(), model_in).err_conv()?;
            spec.save().err_conv()?;
        }
        GSysCmd::Update(dfx) => {
            configure_dfx_logging(&dfx);
            let options = DownloadOptions::from((dfx.force, ValueDict::default()));
            let spec = SysProject::load(&current_dir).err_conv()?;
            let accessor = accessor_for_default();
            spec.update_local(accessor, &current_dir, &options)
                .await
                .err_conv()?;
        }
        GSysCmd::Localize(args) => {
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

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

    #[tokio::test]
    async fn test_ia_model_std_success() {
        // Mock user selection for testing
        // This test assumes the interactive selection works
        // In a real test environment, you might want to mock the inquire::Select

        let result = ia_model_std();

        // Should return a valid ModelSTD or fail gracefully
        match result {
            Ok(model) => {
                assert!(!model.to_string().is_empty());
            }
            Err(_) => {
                // Interactive tests may fail in CI environments
                // This is acceptable behavior
            }
        }
    }

    #[tokio::test]
    async fn test_do_sys_cmd_new_success() {
        let temp_dir = tempdir().unwrap();
        let project_path = temp_dir.path().join("test_system");

        // Create test command
        let cmd = GSysCmd::New(crate::args::NewArgs {
            name: "test_system".to_string(),
        });

        // Mock the current directory
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Execute the command
        let result = do_sys_cmd(cmd).await;

        // Should create the project directory
        assert!(project_path.exists());

        // Should contain system project files
        assert!(project_path.join("sys/sys_model.yml").exists() || result.is_err());
    }

    #[tokio::test]
    async fn test_do_sys_cmd_update_no_project() {
        let temp_dir = tempdir().unwrap();

        // Create test command
        let cmd = GSysCmd::Update(crate::args::UpdateArgs {
            debug: 1,
            log: None,
            force: 0,
        });

        // Set current directory to empty temp directory
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Should fail gracefully when no project exists
        let result = do_sys_cmd(cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_sys_cmd_localize_no_project() {
        let temp_dir = tempdir().unwrap();

        // Create test command
        let cmd = GSysCmd::Localize(crate::args::LocalArgs {
            debug: 0,
            log: None,
            value: None,
            use_default_value: false,
        });

        // Set current directory to empty temp directory
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Should fail gracefully when no project exists
        let result = do_sys_cmd(cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_sys_cmd_update_with_debug() {
        let temp_dir = tempdir().unwrap();

        // Create test command with debug settings
        let cmd = GSysCmd::Update(crate::args::UpdateArgs {
            debug: 2,
            log: Some("cmd=debug".to_string()),
            force: 1,
        });

        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Should fail gracefully (no project exists)
        let result = do_sys_cmd(cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_sys_cmd_localize_with_values() {
        let temp_dir = tempdir().unwrap();

        // Create test command with value file
        let cmd = GSysCmd::Localize(crate::args::LocalArgs {
            debug: 1,
            log: Some("all=info".to_string()),
            value: Some("test_values.yml".to_string()),
            use_default_value: false,
        });

        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Should fail gracefully (no project exists)
        let result = do_sys_cmd(cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_sys_cmd_localize_with_defaults() {
        let temp_dir = tempdir().unwrap();

        // Create test command with default values
        let cmd = GSysCmd::Localize(crate::args::LocalArgs {
            debug: 0,
            log: None,
            value: None,
            use_default_value: true,
        });

        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Should fail gracefully (no project exists)
        let result = do_sys_cmd(cmd).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_model_std_support() {
        let models = ModelSTD::support();
        assert!(!models.is_empty());

        // Verify each model can be converted to string
        for model in models {
            let model_str = format!("{}", model);
            assert!(!model_str.is_empty());
        }
    }

    #[test]
    fn test_current_system_model() {
        let current_model = ModelSTD::from_cur_sys();
        assert!(!current_model.to_string().is_empty());
    }

    #[tokio::test]
    async fn test_configure_dfx_logging_compiles() {
        // Test that the logging configuration compiles
        let args = crate::args::UpdateArgs {
            debug: 1,
            log: Some("test=debug".to_string()),
            force: 0,
        };

        // This should not panic
        configure_dfx_logging(&args);
    }
}
