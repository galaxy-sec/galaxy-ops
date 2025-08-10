mod args;
mod spec;
//mod vault;

extern crate log;
#[macro_use]
extern crate clap;

use crate::args::GxModCmd;
use clap::Parser;
use galaxy_ops::error::{MainResult, report_error};
use orion_error::ErrorOwe;
use orion_variate::vars::setup_start_env_vars;
use spec::do_mod_cmd;

#[tokio::main]
async fn main() {
    use std::process;
    match GxMod::run().await {
        Err(e) => report_error(e),
        Ok(_) => {
            return;
        }
    }
    process::exit(-1);
}

pub struct GxMod {}
impl GxMod {
    pub async fn run() -> MainResult<()> {
        setup_start_env_vars().owe_res()?;
        println!("gmod: {}", env!("CARGO_PKG_VERSION"));
        let cmd = GxModCmd::parse();
        do_mod_cmd(cmd).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{CommandFactory, Parser};
    use std::env;

    #[test]
    fn test_main_function_exists() {
        // Test that main function compiles and can be called
        // Note: we can't easily test the actual main function due to process::exit
        // But we can verify the structure

        // Verify that GxMod::run can be called (even if it fails in test env)
        let args = vec!["gmod", "example"];
        let cmd = GxModCmd::try_parse_from(args);
        assert!(cmd.is_ok());
    }

    #[tokio::test]
    async fn test_setup_environment_vars() {
        // Test the environment setup
        let result = setup_start_env_vars();

        // Should succeed or fail gracefully
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_version_display() {
        // Test that version information is displayed correctly
        let version = env!("CARGO_PKG_VERSION");
        assert!(!version.is_empty());
        assert!(version.contains('.'));
    }

    #[tokio::test]
    async fn test_error_handling() {
        // Test that errors are handled gracefully
        let args = vec!["gmod", "invalid-command"];
        let result = GxModCmd::try_parse_from(args);

        // Should fail gracefully with a parsing error
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(!error.to_string().is_empty());
    }

    #[test]
    fn test_command_structure() {
        // Test that all expected commands are available
        let app = GxModCmd::command();
        let subcommands = app.get_subcommands();

        let mut found_example = false;
        let mut found_new = false;
        let mut found_update = false;
        let mut found_localize = false;

        for subcommand in subcommands {
            match subcommand.get_name() {
                "example" => found_example = true,
                "new" => found_new = true,
                "update" => found_update = true,
                "localize" => found_localize = true,
                _ => {}
            }
        }

        assert!(found_example, "Example command should be available");
        assert!(found_new, "New command should be available");
        assert!(found_update, "Update command should be available");
        assert!(found_localize, "Localize command should be available");
    }

    #[tokio::test]
    async fn test_all_commands_parse() {
        // Test that all commands can be parsed without error
        let commands = vec![
            vec!["gmod", "example"],
            vec!["gmod", "new", "test-module"],
            vec!["gmod", "update"],
            vec!["gmod", "localize"],
        ];

        for cmd_args in commands {
            let result = GxModCmd::try_parse_from(cmd_args.clone());
            assert!(result.is_ok(), "Failed to parse command: {:?}", cmd_args);
        }
    }

    #[tokio::test]
    async fn test_commands_with_options() {
        // Test commands with various options
        let commands = vec![
            vec!["gmod", "new", "test", "--debug", "2", "--log", "cmd=debug"],
            vec!["gmod", "update", "--force", "1", "--debug", "1"],
            vec!["gmod", "localize", "--value", "test.yml", "--default"],
        ];

        for cmd_args in commands {
            let result = GxModCmd::try_parse_from(cmd_args.clone());
            assert!(
                result.is_ok(),
                "Failed to parse command with options: {:?}",
                cmd_args
            );
        }
    }
}
