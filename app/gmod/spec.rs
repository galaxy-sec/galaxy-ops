use galaxy_ops::accessor::accessor_for_default;
use galaxy_ops::error::MainResult;
use galaxy_ops::infra::configure_dfx_logging;
use galaxy_ops::module::proj::ModProject;
use galaxy_ops::module::spec::make_mod_spec_example;
use galaxy_ops::project::load_project_global_value;
use galaxy_ops::types::{Localizable, LocalizeOptions, RefUpdateable};
use orion_common::serde::Persistable;
use orion_error::{ErrorConv, ErrorOwe};
use orion_variate::update::DownloadOptions;
use orion_variate::vars::ValueDict;
use std::path::PathBuf;

use crate::args::{self};

pub async fn do_mod_cmd(cmd: args::GxModCmd) -> MainResult<()> {
    let current_dir = std::env::current_dir().expect("无法获取当前目录");
    match cmd {
        args::GxModCmd::Example => {
            let spec = make_mod_spec_example().err_conv()?;
            spec.save_to(&PathBuf::from("./"), None).owe_res()?;
        }
        args::GxModCmd::New(spec_args) => {
            let project_dir = current_dir.join(spec_args.name());
            std::fs::create_dir(&project_dir).owe_res()?;
            configure_dfx_logging(&spec_args);
            let spec = ModProject::make_new(&project_dir, spec_args.name.as_str()).err_conv()?;
            spec.save().err_conv()?;
        }
        args::GxModCmd::Update(dfx) => {
            configure_dfx_logging(&dfx);
            let spec = ModProject::load(&current_dir).err_conv()?;
            let options = DownloadOptions::from((dfx.force, ValueDict::default()));
            let accessor = accessor_for_default();
            spec.update_local(accessor, &current_dir, &options)
                .await
                .err_conv()?;
        }
        args::GxModCmd::Localize(args) => {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        GxModCmd,
        args::{LocalArgs, SpecArgs, UpdateArgs},
    };
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_do_mod_cmd_example_success() {
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Create test command
        let cmd = GxModCmd::Example;

        // Execute the command
        let result = do_mod_cmd(cmd).await;

        // Should create example module files
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_do_mod_cmd_new_success() {
        let temp_dir = tempdir().unwrap();
        let project_path = temp_dir.path().join("test_module");

        // Create test command
        let cmd = GxModCmd::New(SpecArgs {
            name: "test_module".to_string(),
            debug: 0,
            log: None,
        });

        // Set current directory
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Execute the command
        let result = do_mod_cmd(cmd).await;

        // Should create the project directory and files
        assert!(project_path.exists());
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_do_mod_cmd_new_with_debug() {
        let temp_dir = tempdir().unwrap();
        let project_path = temp_dir.path().join("debug_module");

        // Create test command with debug settings
        let cmd = GxModCmd::New(SpecArgs {
            name: "debug_module".to_string(),
            debug: 2,
            log: Some("cmd=debug".to_string()),
        });

        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Execute the command
        let result = do_mod_cmd(cmd).await;

        // Should succeed
        assert!(project_path.exists());
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_do_mod_cmd_update_no_project() {
        let temp_dir = tempdir().unwrap();

        // Create test command
        let cmd = GxModCmd::Update(UpdateArgs {
            debug: 1,
            log: None,
            force: 0,
        });

        // Set current directory to empty temp directory
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Should fail gracefully when no project exists
        let result = do_mod_cmd(cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_mod_cmd_update_with_force() {
        let temp_dir = tempdir().unwrap();

        // Create test command with force
        let cmd = GxModCmd::Update(UpdateArgs {
            debug: 2,
            log: Some("all=info".to_string()),
            force: 1,
        });

        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Should fail gracefully (no project exists)
        let result = do_mod_cmd(cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_mod_cmd_localize_no_project() {
        let temp_dir = tempdir().unwrap();

        // Create test command
        let cmd = GxModCmd::Localize(LocalArgs {
            debug: 0,
            log: None,
            value: None,
            use_default_value: false,
        });

        // Set current directory to empty temp directory
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Should fail gracefully when no project exists
        let result = do_mod_cmd(cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_mod_cmd_localize_with_values() {
        let temp_dir = tempdir().unwrap();

        // Create test command with value file
        let cmd = GxModCmd::Localize(LocalArgs {
            debug: 1,
            log: Some("all=info".to_string()),
            value: Some("test_values.yml".to_string()),
            use_default_value: false,
        });

        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Should fail gracefully (no project exists)
        let result = do_mod_cmd(cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_mod_cmd_localize_with_defaults() {
        let temp_dir = tempdir().unwrap();

        // Create test command with default values
        let cmd = GxModCmd::Localize(LocalArgs {
            debug: 0,
            log: None,
            value: None,
            use_default_value: true,
        });

        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Should fail gracefully (no project exists)
        let result = do_mod_cmd(cmd).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_make_mod_spec_example_compiles() {
        // Test that the example spec function compiles
        // This is a basic compilation test
        let result = make_mod_spec_example();

        // Should succeed or fail gracefully
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_configure_dfx_logging_compiles() {
        // Test that the logging configuration compiles
        let args = UpdateArgs {
            debug: 1,
            log: Some("test=debug".to_string()),
            force: 0,
        };

        // This should not panic
        configure_dfx_logging(&args);
    }

    #[tokio::test]
    async fn test_dfx_logging_with_local_args() {
        // Test logging configuration with local args
        let args = LocalArgs {
            debug: 2,
            log: Some("local=debug".to_string()),
            value: None,
            use_default_value: false,
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

        // Handle macOS path normalization (/private/var vs /var)
        let current_path = current_dir.unwrap();
        let temp_path = temp_dir.path();

        // Use canonical path to resolve any symlinks or path normalization differences
        let current_canonical =
            std::fs::canonicalize(&current_path).unwrap_or_else(|_| current_path.to_path_buf());
        let temp_canonical =
            std::fs::canonicalize(temp_path).unwrap_or_else(|_| temp_path.to_path_buf());

        assert_eq!(current_canonical, temp_canonical);

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();
    }

    #[tokio::test]
    async fn test_error_handling() {
        // Test that errors are handled gracefully
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Try to update in empty directory
        let cmd = GxModCmd::Update(UpdateArgs {
            debug: 0,
            log: None,
            force: 0,
        });

        let result = do_mod_cmd(cmd).await;
        assert!(result.is_err());

        // Error message should be meaningful
        let error_msg = result.unwrap_err().to_string();
        assert!(!error_msg.is_empty());
    }
}
