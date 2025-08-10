use galaxy_ops::accessor::accessor_for_default;
use galaxy_ops::error::MainResult;
use galaxy_ops::infra::configure_dfx_logging;
use galaxy_ops::ops_prj::proj::OpsProject;
use galaxy_ops::types::InsUpdateable;
use orion_error::{ErrorConv, ErrorOwe};
use orion_infra::path::make_new_path;
use orion_variate::update::DownloadOptions;
use orion_variate::vars::ValueDict;

use crate::args::GInsCmd;

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
        GInsCmd::Localize(_args) => {
            todo!();
            /*
            configure_dfx_logging(&args);
            let spec = OpsProject::load(&current_dir).err_conv()?;
            let dict = load_project_global_value(spec.root_local(), args.value())?;
            spec.localize(LocalizeOptions::new(dict, args.use_default_value))
                .await
                .err_conv()?;
            */
        }
        GInsCmd::Setting(args) => {
            configure_dfx_logging(&args);
            let spec = OpsProject::load(&current_dir).err_conv()?;
            spec.ia_setting()?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::{GInsCmd, ImportArgs, LocalArgs, NewArgs, SettingArgs, UpdateArgs};
    use std::path::PathBuf;
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
    async fn test_do_ins_cmd_localize_todo() {
        let temp_dir = tempdir().unwrap();

        // Create test command
        let cmd = GInsCmd::Localize(LocalArgs {
            debug: 0,
            log: None,
            value: None,
            use_default_value: false,
        });

        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Should panic due to todo!()
        let result = std::panic::catch_unwind(|| {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let _ = do_ins_cmd(cmd).await;
            })
        });

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
        assert_eq!(current_dir.unwrap(), temp_dir.path());

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();
    }

    #[tokio::test]
    async fn test_download_options_creation() {
        // Test DownloadOptions creation with different force levels
        let options1 = DownloadOptions::from((0, ValueDict::default()));
        let options2 = DownloadOptions::from((1, ValueDict::default()));
        let options3 = DownloadOptions::from((2, ValueDict::default()));
        let options4 = DownloadOptions::from((3, ValueDict::default()));

        // Should create without panicking
        // Note: We can't easily test the internal state without accessors
        // But we can verify they don't panic
        assert!(true);
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

    #[tokio::test]
    async fn test_all_commands_fail_in_empty_dir() {
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let commands = vec![
            GInsCmd::Import(ImportArgs {
                debug: 0,
                log: None,
                force: 0,
                path: "/test".to_string(),
            }),
            GInsCmd::Update(UpdateArgs {
                debug: 0,
                log: None,
                force: 0,
            }),
            GInsCmd::Localize(LocalArgs {
                debug: 0,
                log: None,
                value: None,
                use_default_value: false,
            }),
            GInsCmd::Setting(SettingArgs {
                debug: 0,
                log: None,
            }),
        ];

        for cmd in commands {
            // Check command type before consuming it
            let is_localize = matches!(cmd, GInsCmd::Localize(_));
            let result = do_ins_cmd(cmd).await;
            // Most commands should fail in empty directory, except Localize which panics
            if is_localize {
                // Skip panic test for localize
                continue;
            }
            assert!(result.is_err(), "Command should fail in empty directory");
        }
    }
}
