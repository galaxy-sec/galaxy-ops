use clap::{ArgAction, Parser};
use derive_getters::Getters;
use galaxy_ops::infra::DfxArgsGetter;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "gmod")]
#[command(
    version,
    about = "Galaxy Module Management Tool",
    long_about = "A comprehensive tool for managing Galaxy modules including creating new modules, updating existing ones, and localizing configurations."
)]
pub enum GxModCmd {
    /// Create example module structure
    #[command(
        about = "Create example module structure",
        long_about = "Create a complete example module structure with sample configurations and workflows to demonstrate module organization and best practices."
    )]
    Example,
    /// Define new module specification
    #[command(
        about = "Define new module operator ",
        long_about = "Create a new module specification with the given name. This will initialize a new module directory structure with all necessary configuration files."
    )]
    New(SpecArgs),
    /// Update existing module
    #[command(
        about = "Update existing module operator dependency",
        long_about = "Update an existing module's configuration, dependencies, or specifications. Supports force updates to override existing configurations."
    )]
    Update(UpdateArgs),
    /// Localize module configuration
    #[command(
        about = "Localize module configuration",
        long_about = "Generate localized configuration files for the module based on environment-specific values. Useful for adapting modules to different deployment environments."
    )]
    Localize(LocalArgs),
}

#[derive(Debug, Args, Getters)]
pub struct SpecArgs {
    /// Name of the new module to create
    #[arg(
        short,
        long,
        help = "Module name (alphanumeric with hyphens/underscores)"
    )]
    pub(crate) name: String,

    /// Enable debug output with specified level (0-3)
    #[arg(
        short = 'd',
        long = "debug",
        default_value = "0",
        help = "Debug level: 0=off, 1=basic, 2=verbose, 3=trace"
    )]
    pub debug: usize,
    /// Set logging level and format
    #[arg(long = "log", help = "Log level: error, warn, info, debug, trace")]
    pub log: Option<String>,
}
#[derive(Debug, Args, Getters)]
pub struct UpdateArgs {
    /// Enable debug output with specified level (0-3)
    #[arg(
        short = 'd',
        long = "debug",
        default_value = "0",
        help = "Debug level: 0=off, 1=basic, 2=verbose, 3=trace"
    )]
    pub debug: usize,
    /// Set logging level and format
    #[arg(long = "log", help = "Log level: error, warn, info, debug, trace")]
    pub log: Option<String>,

    /// Force update even if conflicts exist
    #[arg(
        short = 'f',
        long = "force",
        default_value = "0",
        help = "Force update: skip confirmation, overwrite existing files"
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
    /// Enable debug output with specified level (0-3)
    #[arg(
        short = 'd',
        long = "debug",
        default_value = "0",
        help = "Debug level: 0=off, 1=basic, 2=verbose, 3=trace"
    )]
    pub debug: usize,
    /// Set logging level and format
    #[arg(long = "log", help = "Log level: error, warn, info, debug, trace")]
    pub log: Option<String>,

    /// Path to values file for localization
    #[arg(
        long = "value",
        help = "Path to YAML/JSON file containing environment-specific values"
    )]
    pub value: Option<String>,
    /// Use default values instead of user-provided value.yml
    #[arg(long = "default", default_value = "false" , action = ArgAction::SetTrue, help = "Use default values instead of user-provided value.yml")]
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

impl DfxArgsGetter for SpecArgs {
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
    fn test_gxmod_cmd_app_creation() {
        let app = GxModCmd::command();
        assert_eq!(app.get_name(), "gmod");
        assert!(app.get_about().is_some());
        assert!(app.get_long_about().is_some());
    }

    #[test]
    fn test_example_command_parsing() {
        let args = vec!["gmod", "example"];
        let cmd = GxModCmd::try_parse_from(args).unwrap();

        match cmd {
            GxModCmd::Example => {
                // Example command has no arguments
            }
            _ => panic!("Expected Example command"),
        }
    }

    #[test]
    fn test_new_command_parsing() {
        let args = vec!["gmod", "new", "--name", "test-module", "--debug", "2"];
        let cmd = GxModCmd::try_parse_from(args).unwrap();

        match cmd {
            GxModCmd::New(new_args) => {
                assert_eq!(new_args.name(), "test-module");
                assert_eq!(*new_args.debug(), 2);
                assert_eq!(new_args.log(), &None);
            }
            _ => panic!("Expected New command"),
        }
    }

    #[test]
    fn test_new_command_with_log() {
        let args = vec!["gmod", "new", "--name", "test-module", "--log", "cmd=debug"];
        let cmd = GxModCmd::try_parse_from(args).unwrap();

        match cmd {
            GxModCmd::New(new_args) => {
                assert_eq!(new_args.name(), "test-module");
                assert_eq!(*new_args.debug(), 0);
                assert_eq!(*new_args.log(), Some("cmd=debug".to_string()));
            }
            _ => panic!("Expected New command"),
        }
    }

    #[test]
    fn test_update_command_parsing() {
        let args = vec!["gmod", "update", "--debug", "1", "--force", "2"];
        let cmd = GxModCmd::try_parse_from(args).unwrap();

        match cmd {
            GxModCmd::Update(update_args) => {
                assert_eq!(*update_args.debug(), 1);
                assert_eq!(update_args.force, 2);
                assert_eq!(update_args.log(), &None);
            }
            _ => panic!("Expected Update command"),
        }
    }

    #[test]
    fn test_update_command_with_log() {
        let args = vec!["gmod", "update", "--log", "all=info"];
        let cmd = GxModCmd::try_parse_from(args).unwrap();

        match cmd {
            GxModCmd::Update(update_args) => {
                assert_eq!(*update_args.debug(), 0);
                assert_eq!(update_args.force, 0);
                assert_eq!(*update_args.log(), Some("all=info".to_string()));
            }
            _ => panic!("Expected Update command"),
        }
    }

    #[test]
    fn test_localize_command_parsing() {
        let args = vec![
            "gmod",
            "localize",
            "--value",
            "prod-values.yml",
            "--default",
        ];
        let cmd = GxModCmd::try_parse_from(args).unwrap();

        match cmd {
            GxModCmd::Localize(local_args) => {
                assert_eq!(*local_args.debug(), 0);
                assert_eq!(local_args.value(), &Some("prod-values.yml".to_string()));
                assert!(local_args.use_default_value);
                assert_eq!(local_args.log(), &None);
            }
            _ => panic!("Expected Localize command"),
        }
    }

    #[test]
    fn test_localize_command_with_debug() {
        let args = vec!["gmod", "localize", "-d", "3", "--log", "test=debug"];
        let cmd = GxModCmd::try_parse_from(args).unwrap();

        match cmd {
            GxModCmd::Localize(local_args) => {
                assert_eq!(*local_args.debug(), 3);
                assert_eq!(*local_args.log(), Some("test=debug".to_string()));
                assert!(!local_args.use_default_value);
                assert_eq!(local_args.value(), &None);
            }
            _ => panic!("Expected Localize command"),
        }
    }

    #[test]
    fn test_dfx_args_getter_spec() {
        let spec_args = SpecArgs {
            name: "test-module".to_string(),
            debug: 1,
            log: Some("cmd=debug".to_string()),
        };

        assert_eq!(spec_args.debug_level(), 1);
        assert_eq!(spec_args.log_setting(), Some("cmd=debug".to_string()));
    }

    #[test]
    fn test_dfx_args_getter_update() {
        let update_args = UpdateArgs {
            debug: 2,
            log: Some("all=info".to_string()),
            force: 1,
        };

        assert_eq!(update_args.debug_level(), 2);
        assert_eq!(update_args.log_setting(), Some("all=info".to_string()));
    }

    #[test]
    fn test_dfx_args_getter_localize() {
        let local_args = LocalArgs {
            debug: 1,
            log: Some("test=debug".to_string()),
            value: Some("values.yml".to_string()),
            use_default_value: true,
        };

        assert_eq!(local_args.debug_level(), 1);
        assert_eq!(local_args.log_setting(), Some("test=debug".to_string()));
    }

    #[test]
    fn test_invalid_module_name() {
        let args = vec!["gmod", "new", ""];
        let result = GxModCmd::try_parse_from(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_unknown_command() {
        let args = vec!["gmod", "unknown"];
        let result = GxModCmd::try_parse_from(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_force_flag_validation() {
        let args = vec!["gmod", "update", "--force", "5"];
        let cmd = GxModCmd::try_parse_from(args);

        // Note: clap doesn't validate ranges by default, so this should succeed
        assert!(cmd.is_ok());
    }

    #[test]
    fn test_debug_flag_validation() {
        let args = vec!["gmod", "new", "--name", "test", "--debug", "1"];
        let cmd = GxModCmd::try_parse_from(args);

        // Note: clap doesn't validate ranges by default, so this should succeed
        assert!(cmd.is_ok());
    }
}
