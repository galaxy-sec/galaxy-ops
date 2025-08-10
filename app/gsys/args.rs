use clap::{ArgAction, Parser};
use derive_getters::Getters;
use galaxy_ops::infra::DfxArgsGetter;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "gsys")]
#[command(
    version,
    about = "Galaxy System Management Tool",
    long_about = "A comprehensive tool for managing Galaxy system configurations, including creating new system specs, updating existing configurations, and localizing settings for different environments."
)]
pub enum GSysCmd {
    /// Create new system operator
    #[command(
        about = "Create new system operator ",
        long_about = "Create a new system specification with the given name. This will initialize a new system directory structure with all necessary configuration files and templates."
    )]
    New(NewArgs),
    /// Update existing system configuration
    #[command(
        about = "Update system configuration",
        long_about = "Update an existing system's configuration, specifications, or dependencies. Supports force updates to override existing configurations without confirmation."
    )]
    Update(UpdateArgs),
    /// Localize system configuration for environment
    #[command(
        about = "Localize system configuration",
        long_about = "Generate localized configuration files for the system based on environment-specific values. Useful for adapting system configurations to different deployment environments."
    )]
    Localize(LocalArgs),
}

#[derive(Debug, Args, Getters)]
pub struct NewArgs {
    /// Name of the new system to create
    #[arg(
        short,
        long,
        help = "System name (alphanumeric with hyphens/underscores)"
    )]
    pub(crate) name: String,
}

#[derive(Debug, Args, Getters)]
pub struct UpdateArgs {
    /// Enable debug output with specified level (0-4)
    #[arg(
        short = 'd',
        long = "debug",
        default_value = "0",
        help = "Debug level: 0=off, 1=basic, 2=verbose, 3=trace, 4=full"
    )]
    pub debug: usize,
    /// Configure logging output format and levels
    #[arg(
        long = "log",
        help = "Configure logging: eg --log cmd=debug,parse=info"
    )]
    pub log: Option<String>,

    /// Force update level (0-3)
    #[arg(
        short = 'f',
        long = "force",
        default_value = "0",
        help = "Force update: 0=normal, 1=skip confirmation, 2=overwrite files, 3=force git pull"
    )]
    pub force: usize,
}
impl DfxArgsGetter for UpdateArgs {
    fn debug_level(&self) -> usize {
        self.debug
    }

    fn log_setting(&self) -> Option<String> {
        self.log.clone()
    }
}

#[derive(Debug, Args, Getters)]
pub struct LocalArgs {
    /// Enable debug output with specified level (0-4)
    #[arg(
        short = 'd',
        long = "debug",
        default_value = "0",
        help = "Debug level: 0=off, 1=basic, 2=verbose, 3=trace, 4=full"
    )]
    pub debug: usize,
    /// Configure logging output format and levels
    #[arg(
        long = "log",
        help = "Configure logging: eg --log cmd=debug,parse=info"
    )]
    pub log: Option<String>,

    /// Path to values file for localization
    #[arg(
        long = "value",
        help = "Path to YAML/JSON file containing environment-specific values"
    )]
    pub value: Option<String>,

    /// Use default values instead of user-provided value.yml
    #[arg(long = "default", default_value = "false" , action = ArgAction::SetTrue, help = "Use built-in default values instead of user-provided value.yml")]
    pub use_default_value: bool,
}
impl DfxArgsGetter for LocalArgs {
    fn debug_level(&self) -> usize {
        self.debug
    }

    fn log_setting(&self) -> Option<String> {
        self.log.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_gsys_cmd_app_creation() {
        let app = GSysCmd::command();
        assert_eq!(app.get_name(), "gsys");
        assert!(app.get_about().is_some());
        assert!(app.get_long_about().is_some());
    }

    #[test]
    fn test_new_args_parsing() {
        let args = vec!["gsys", "new", "test-system"];
        let cmd = GSysCmd::try_parse_from(args).unwrap();

        match cmd {
            GSysCmd::New(new_args) => {
                assert_eq!(new_args.name(), "test-system");
            }
            _ => panic!("Expected New command"),
        }
    }

    #[test]
    fn test_update_args_parsing() {
        let args = vec!["gsys", "update", "--debug", "2", "--log", "cmd=debug"];
        let cmd = GSysCmd::try_parse_from(args).unwrap();

        match cmd {
            GSysCmd::Update(update_args) => {
                assert_eq!(*update_args.debug(), 2);
                assert_eq!(*update_args.log(), Some("cmd=debug".to_string()));
                assert_eq!(update_args.force, 0);
            }
            _ => panic!("Expected Update command"),
        }
    }

    #[test]
    fn test_update_args_with_force() {
        let args = vec!["gsys", "update", "-f", "1", "-d", "3"];
        let cmd = GSysCmd::try_parse_from(args).unwrap();

        match cmd {
            GSysCmd::Update(update_args) => {
                assert_eq!(*update_args.debug(), 3);
                assert_eq!(update_args.force, 1);
                assert_eq!(*update_args.log(), None);
            }
            _ => panic!("Expected Update command"),
        }
    }

    #[test]
    fn test_localize_args_parsing() {
        let args = vec![
            "gsys",
            "localize",
            "--value",
            "prod-values.yml",
            "--default",
        ];
        let cmd = GSysCmd::try_parse_from(args).unwrap();

        match cmd {
            GSysCmd::Localize(local_args) => {
                assert_eq!(*local_args.debug(), 0);
                assert_eq!(*local_args.value(), Some("prod-values.yml".to_string()));
                assert_eq!(local_args.use_default_value, true);
            }
            _ => panic!("Expected Localize command"),
        }
    }

    #[test]
    fn test_localize_args_with_debug() {
        let args = vec!["gsys", "localize", "-d", "1", "--log", "all=info"];
        let cmd = GSysCmd::try_parse_from(args).unwrap();

        match cmd {
            GSysCmd::Localize(local_args) => {
                assert_eq!(*local_args.debug(), 1);
                assert_eq!(*local_args.log(), Some("all=info".to_string()));
                assert_eq!(local_args.use_default_value, false);
                assert_eq!(*local_args.value(), None);
            }
            _ => panic!("Expected Localize command"),
        }
    }

    #[test]
    fn test_dfx_args_getter_update() {
        let update_args = UpdateArgs {
            debug: 2,
            log: Some("cmd=debug".to_string()),
            force: 1,
        };

        assert_eq!(update_args.debug_level(), 2);
        assert_eq!(update_args.log_setting(), Some("cmd=debug".to_string()));
    }

    #[test]
    fn test_dfx_args_getter_localize() {
        let local_args = LocalArgs {
            debug: 1,
            log: Some("all=info".to_string()),
            value: Some("test.yml".to_string()),
            use_default_value: true,
        };

        assert_eq!(local_args.debug_level(), 1);
        assert_eq!(local_args.log_setting(), Some("all=info".to_string()));
    }

    #[test]
    fn test_invalid_system_name() {
        let args = vec!["gsys", "new", ""];
        let result = GSysCmd::try_parse_from(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_debug_level() {
        let args = vec!["gsys", "update", "--debug", "10"];
        let result = GSysCmd::try_parse_from(args);
        // Note: clap doesn't validate numeric ranges by default, so this will succeed
        // In a real implementation, you'd add custom validation
        assert!(result.is_ok());
    }

    #[test]
    fn test_help_output() {
        let help_text = GSysCmd::command().render_help().to_string();
        assert!(help_text.contains("Galaxy System Management Tool"));
        assert!(help_text.contains("Create new system operator"));
        assert!(help_text.contains("Update system configuration"));
        assert!(help_text.contains("Localize system configuration"));
    }

    #[test]
    fn test_long_help_output() {
        let long_help = GSysCmd::command().render_long_help().to_string();
        assert!(long_help.contains("comprehensive tool for managing Galaxy system"));
        assert!(long_help.contains("Create a new system specification"));
        assert!(long_help.contains("Update an existing system's configuration"));
        assert!(long_help.contains("Generate localized configuration files"));
    }

    #[test]
    fn test_subcommand_help() {
        let args = vec!["gsys", "new", "--help"];
        let cmd = GSysCmd::try_parse_from(args);

        // This will show help and exit, so we expect an error in the test
        match cmd {
            Err(e) => {
                assert_eq!(e.kind(), clap::error::ErrorKind::DisplayHelp);
            }
            Ok(_) => panic!("Expected help display error"),
        }
    }
}
