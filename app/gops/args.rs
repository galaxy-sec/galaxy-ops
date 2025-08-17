use clap::{ArgAction, Args, Parser};
use derive_getters::Getters;
use galaxy_ops::infra::DfxArgsGetter;

// Mod子命令参数结构
#[derive(Debug, Args, Getters)]
pub struct ModExampleArgs {
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
pub struct ModNewArgs {
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
pub struct ModUpdateArgs {
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

#[derive(Debug, Args, Getters)]
pub struct ModLocalizeArgs {
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

#[derive(Debug, Parser)]
pub enum ModCmd {
    /// Create example module structure
    #[command(
        about = "Create example module structure",
        long_about = "Create a complete example module structure with sample configurations and workflows to demonstrate module organization and best practices."
    )]
    Example(ModExampleArgs),
    /// Define new module specification
    #[command(
        about = "Define new module operator ",
        long_about = "Create a new module specification with the given name. This will initialize a new module directory structure with all necessary configuration files."
    )]
    New(ModNewArgs),
    /// Update existing module
    #[command(
        about = "Update existing module operator dependency",
        long_about = "Update an existing module's configuration, dependencies, or specifications. Supports force updates to override existing configurations."
    )]
    Update(ModUpdateArgs),
    /// Localize module configuration
    #[command(
        about = "Localize module configuration",
        long_about = "Generate localized configuration files for the module based on environment-specific values. Useful for adapting modules to different deployment environments."
    )]
    Localize(ModLocalizeArgs),
}

// Sys子命令参数结构
#[derive(Debug, Args, Getters)]
pub struct SysNewArgs {
    /// Name of the new system to create
    #[arg(
        short,
        long,
        help = "System name (alphanumeric with hyphens/underscores)"
    )]
    pub(crate) name: String,
}

#[derive(Debug, Args, Getters)]
pub struct SysUpdateArgs {
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

#[derive(Debug, Args, Getters)]
pub struct SysLocalizeArgs {
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

#[derive(Debug, Args, Getters)]
pub struct PrjNewArgs {
    /// 工程配置名称
    #[arg(short, long, help = "工程配置名称")]
    pub(crate) name: String,

    /// 调试输出级别
    #[arg(
        short = 'd',
        long = "debug",
        default_value = "0",
        help = "调试级别: 0=关闭, 1=基础, 2=详细, 3=完整"
    )]
    pub debug: usize,

    /// 日志配置
    #[arg(long = "log", help = "日志配置")]
    pub log: Option<String>,
}

#[derive(Debug, Args, Getters)]
pub struct PrjImportArgs {
    /// 系统导入路径
    #[arg(short = 'p', long = "path", help = "系统导入路径")]
    pub path: String,

    /// 调试输出级别
    #[arg(
        short = 'd',
        long = "debug",
        default_value = "0",
        help = "调试级别: 0=关闭, 1=基础, 2=详细, 3=完整"
    )]
    pub debug: usize,

    /// 日志配置
    #[arg(long = "log", help = "日志配置")]
    pub log: Option<String>,

    /// 强制更新级别
    #[arg(
        short = 'f',
        long = "force",
        default_value = "0",
        help = "强制更新级别: 0=正常, 1=跳过确认, 2=覆盖文件, 3=强制拉取"
    )]
    pub force: usize,
}

#[derive(Debug, Args, Getters)]
pub struct PrjUpdateArgs {
    /// 调试输出级别
    #[arg(
        short = 'd',
        long = "debug",
        default_value = "0",
        help = "调试级别: 0=关闭, 1=基础, 2=详细, 3=完整"
    )]
    pub debug: usize,

    /// 日志配置
    #[arg(long = "log", help = "日志配置")]
    pub log: Option<String>,

    /// 强制更新级别
    #[arg(
        short = 'f',
        long = "force",
        default_value = "0",
        help = "强制更新级别: 0=正常, 1=跳过确认, 2=覆盖文件, 3=强制拉取"
    )]
    pub force: usize,
}

#[derive(Debug, Args, Getters)]
pub struct PrjSettingArgs {
    /// 调试输出级别
    #[arg(
        short = 'd',
        long = "debug",
        default_value = "0",
        help = "调试级别: 0=关闭, 1=基础, 2=详细, 3=完整"
    )]
    pub debug: usize,

    /// 日志配置
    #[arg(long = "log", help = "日志配置")]
    pub log: Option<String>,
}

#[derive(Debug, Parser)]
pub enum PrjCmd {
    /// 创建维护工程
    #[command(
        about = "创建维护工程",
        long_about = "创建新的维护工程，包含所有必要的配置文件和目录结构。"
    )]
    New(PrjNewArgs),

    /// 导入系统到工程
    #[command(
        about = "导入系统到工程",
        long_about = "从指定路径导入系统到当前工程，集成到现有的项目结构中。"
    )]
    Import(PrjImportArgs),

    /// 维护工程
    #[command(
        about = "维护工程",
        long_about = "更新现有工程的配置、依赖关系和引用信息。支持强制更新。"
    )]
    Update(PrjUpdateArgs),

    /// 为设置工程
    #[command(
        about = "为设置工程",
        long_about = "管理系统级别的配置设置，提供交互式配置界面。"
    )]
    Setting(PrjSettingArgs),
}

#[derive(Debug, Parser)]
pub enum SysCmd {
    /// Create new system operator
    #[command(
        about = "Create new system operator ",
        long_about = "Create a new system specification with the given name. This will initialize a new system directory structure with all necessary configuration files and templates."
    )]
    New(SysNewArgs),
    /// Update existing system configuration
    #[command(
        about = "Update system configuration",
        long_about = "Update an existing system's configuration, specifications, or dependencies. Supports force updates to override existing configurations without confirmation."
    )]
    Update(SysUpdateArgs),
    /// Localize system configuration for environment
    #[command(
        about = "Localize system configuration",
        long_about = "Generate localized configuration files for the system based on environment-specific values. Useful for adapting system configurations to different deployment environments."
    )]
    Localize(SysLocalizeArgs),
}

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "gops")]
#[command(
    version,
    about = "Galaxy Operations System - 系统操作管理工具",
    long_about = "Galaxy Operations System - 系统操作管理工具

用于管理系统配置、导入模块、更新引用等操作的核心工具。"
)]
pub enum GInsCmd {
    /// 创建新的维护工程
    New(NewArgs),
    /// 导入系统到当前工程
    Import(ImportArgs),
    /// 更新系统模块和引用
    ///
    /// 更新系统模块的引用、依赖关系等配置信息
    Update(UpdateArgs),
    /// 本地化模块配置
    ///
    /// 管理系统级别的配置设置
    Setting(SettingArgs),

    /// Module management commands
    #[command(subcommand, about = "Module management commands")]
    Mod(ModCmd),

    /// System management commands
    #[command(subcommand, about = "System management commands")]
    Sys(SysCmd),

    /// Project management commands
    #[command(subcommand, about = "工程管理命令")]
    Prj(PrjCmd),
}

#[derive(Debug, Args, Getters)]
pub struct SettingArgs {
    /// 调试输出级别
    ///
    /// 设置调试信息的详细程度：
    /// - 0: 无调试输出
    /// - 1: 基础调试信息
    /// - 2: 详细调试信息
    /// - 3: 完整调试信息
    #[arg(short = 'd', long = "debug", default_value = "0")]
    pub debug: usize,
    /// 日志配置
    ///
    /// 配置日志输出格式和级别，格式：模块=级别,模块=级别
    /// 例如：--log cmd=debug,parse=info
    #[arg(long = "log")]
    pub log: Option<String>,
}
impl DfxArgsGetter for SettingArgs {
    fn debug_level(&self) -> usize {
        self.debug
    }

    fn log_setting(&self) -> Option<String> {
        self.log.clone()
    }
}

#[derive(Debug, Args, Getters)]
pub struct NewArgs {
    /// 系统配置名称
    ///
    /// 新创建的系统配置的唯一标识名称
    #[arg(short, long, help = "系统配置名称", required = true)]
    #[arg(value_parser = clap::builder::NonEmptyStringValueParser::new())]
    pub(crate) name: String,
}

#[derive(Debug, Args, Getters)]
pub struct UpdateArgs {
    /// 调试输出级别
    ///
    /// 设置调试信息的详细程度：
    /// - 0: 无调试输出
    /// - 1: 基础调试信息
    /// - 2: 详细调试信息
    /// - 3: 完整调试信息
    #[arg(short = 'd', long = "debug", default_value = "0")]
    pub debug: usize,
    /// 日志配置
    ///
    /// 配置日志输出格式和级别，格式：模块=级别,模块=级别
    /// 例如：--log cmd=debug,parse=info
    #[arg(long = "log")]
    pub log: Option<String>,

    /// 强制更新级别
    ///
    /// 强制更新远程git仓库：
    /// - 0: 不强制更新
    /// - 1: 强制更新引用
    /// - 2: 强制更新依赖
    /// - 3: 强制更新所有内容
    #[arg(short = 'f', long = "force", default_value = "0")]
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
pub struct ImportArgs {
    /// 调试输出级别
    ///
    /// 设置调试信息的详细程度：
    /// - 0: 无调试输出
    /// - 1: 基础调试信息
    /// - 2: 详细调试信息
    /// - 3: 完整调试信息
    #[arg(short = 'd', long = "debug", default_value = "0")]
    pub debug: usize,
    /// 日志配置
    ///
    /// 配置日志输出格式和级别，格式：模块=级别,模块=级别
    /// 例如：--log cmd=debug,parse=info
    #[arg(long = "log")]
    pub log: Option<String>,

    /// 强制更新级别
    ///
    /// 强制更新远程git仓库：
    /// - 0: 不强制更新
    /// - 1: 强制更新引用
    /// - 2: 强制更新依赖
    /// - 3: 强制更新所有内容
    #[arg(short = 'f', long = "force", default_value = "0")]
    pub force: usize,

    /// 导入路径
    ///
    /// 要导入的模块所在的路径，可以是相对路径或绝对路径
    #[arg(short = 'p', long = "path", help = "模块导入路径")]
    pub path: String,
}

impl DfxArgsGetter for ImportArgs {
    fn debug_level(&self) -> usize {
        self.debug
    }

    fn log_setting(&self) -> Option<String> {
        self.log.clone()
    }
}

#[derive(Debug, Args, Getters)]
pub struct LocalArgs {
    /// 调试输出级别
    ///
    /// 设置调试信息的详细程度：
    /// - 0: 无调试输出
    /// - 1: 基础调试信息
    /// - 2: 详细调试信息
    /// - 3: 完整调试信息
    #[arg(short = 'd', long = "debug", default_value = "0")]
    pub debug: usize,
    /// 日志配置
    ///
    /// 配置日志输出格式和级别，格式：模块=级别,模块=级别
    /// 例如：--log cmd=debug,parse=info
    #[arg(long = "log")]
    pub log: Option<String>,

    /// 值文件路径
    ///
    /// 指定用于本地化的值文件路径，通常为YAML格式
    /// 例如：--value cicd_value.yml
    #[arg(long = "value", help = "本地化值文件路径")]
    pub value: Option<String>,

    /// 使用默认模块配置
    ///
    /// 启用默认模块模式，不使用用户自定义的value.yml文件
    #[arg(long = "default", default_value = "false" , action = ArgAction::SetTrue, help = "使用默认模块配置")]
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

// Mod命令参数的DfxArgsGetter实现
impl DfxArgsGetter for ModExampleArgs {
    fn debug_level(&self) -> usize {
        self.debug
    }

    fn log_setting(&self) -> Option<String> {
        self.log.clone()
    }
}

impl DfxArgsGetter for ModNewArgs {
    fn debug_level(&self) -> usize {
        self.debug
    }

    fn log_setting(&self) -> Option<String> {
        self.log.clone()
    }
}

impl DfxArgsGetter for ModUpdateArgs {
    fn debug_level(&self) -> usize {
        self.debug
    }

    fn log_setting(&self) -> Option<String> {
        self.log.clone()
    }
}

impl DfxArgsGetter for ModLocalizeArgs {
    fn debug_level(&self) -> usize {
        self.debug
    }

    fn log_setting(&self) -> Option<String> {
        self.log.clone()
    }
}

// Prj命令参数的DfxArgsGetter实现
impl DfxArgsGetter for PrjNewArgs {
    fn debug_level(&self) -> usize {
        self.debug
    }

    fn log_setting(&self) -> Option<String> {
        self.log.clone()
    }
}

impl DfxArgsGetter for PrjImportArgs {
    fn debug_level(&self) -> usize {
        self.debug
    }

    fn log_setting(&self) -> Option<String> {
        self.log.clone()
    }
}

impl DfxArgsGetter for PrjUpdateArgs {
    fn debug_level(&self) -> usize {
        self.debug
    }

    fn log_setting(&self) -> Option<String> {
        self.log.clone()
    }
}

impl DfxArgsGetter for PrjSettingArgs {
    fn debug_level(&self) -> usize {
        self.debug
    }

    fn log_setting(&self) -> Option<String> {
        self.log.clone()
    }
}

// Sys命令参数的DfxArgsGetter实现
impl DfxArgsGetter for SysNewArgs {
    // SysNewArgs 没有debug和log字段，提供默认实现
    fn debug_level(&self) -> usize {
        0
    }

    fn log_setting(&self) -> Option<String> {
        None
    }
}

impl DfxArgsGetter for SysUpdateArgs {
    fn debug_level(&self) -> usize {
        self.debug
    }

    fn log_setting(&self) -> Option<String> {
        self.log.clone()
    }
}

impl DfxArgsGetter for SysLocalizeArgs {
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
    fn test_gins_cmd_app_creation() {
        let app = GInsCmd::command();
        assert_eq!(app.get_name(), "gops");
        assert!(app.get_about().is_some());
        assert!(app.get_long_about().is_some());
    }

    #[test]
    fn test_new_command_parsing() {
        let args = vec!["gops", "new", "--name", "test-system"];
        let cmd = GInsCmd::try_parse_from(args).unwrap();

        match cmd {
            GInsCmd::New(new_args) => {
                assert_eq!(new_args.name(), "test-system");
            }
            _ => panic!("Expected New command"),
        }
    }

    #[test]
    fn test_import_command_parsing() {
        let args = vec!["gops", "import", "--path", "/path/to/module"];
        let cmd = GInsCmd::try_parse_from(args).unwrap();

        match cmd {
            GInsCmd::Import(import_args) => {
                assert_eq!(import_args.path(), "/path/to/module");
                assert_eq!(*import_args.debug(), 0);
                assert_eq!(import_args.force, 0);
            }
            _ => panic!("Expected Import command"),
        }
    }

    #[test]
    fn test_import_command_with_options() {
        let args = vec![
            "gops",
            "import",
            "--debug",
            "2",
            "--force",
            "1",
            "--path",
            "/path/to/module",
        ];
        let cmd = GInsCmd::try_parse_from(args).unwrap();

        match cmd {
            GInsCmd::Import(import_args) => {
                assert_eq!(import_args.path(), "/path/to/module");
                assert_eq!(*import_args.debug(), 2);
                assert_eq!(import_args.force, 1);
            }
            _ => panic!("Expected Import command"),
        }
    }

    #[test]
    fn test_update_command_parsing() {
        let args = vec!["gops", "update", "--debug", "1", "--log", "cmd=debug"];
        let cmd = GInsCmd::try_parse_from(args).unwrap();

        match cmd {
            GInsCmd::Update(update_args) => {
                assert_eq!(*update_args.debug(), 1);
                assert_eq!(*update_args.log(), Some("cmd=debug".to_string()));
                assert_eq!(update_args.force, 0);
            }
            _ => panic!("Expected Update command"),
        }
    }

    #[test]
    fn test_update_command_with_force() {
        let args = vec!["gops", "update", "-f", "3", "-d", "2"];
        let cmd = GInsCmd::try_parse_from(args).unwrap();

        match cmd {
            GInsCmd::Update(update_args) => {
                assert_eq!(*update_args.debug(), 2);
                assert_eq!(update_args.force, 3);
                assert_eq!(update_args.log(), &None);
            }
            _ => panic!("Expected Update command"),
        }
    }

    #[test]
    fn test_setting_command_parsing() {
        let args = vec!["gops", "setting", "--debug", "2", "--log", "system=debug"];
        let cmd = GInsCmd::try_parse_from(args).unwrap();

        match cmd {
            GInsCmd::Setting(setting_args) => {
                assert_eq!(*setting_args.debug(), 2);
                assert_eq!(*setting_args.log(), Some("system=debug".to_string()));
            }
            _ => panic!("Expected Setting command"),
        }
    }

    #[test]
    fn test_dfx_args_getter_setting() {
        let setting_args = SettingArgs {
            debug: 2,
            log: Some("system=debug".to_string()),
        };

        assert_eq!(setting_args.debug_level(), 2);
        assert_eq!(setting_args.log_setting(), Some("system=debug".to_string()));
    }

    #[test]
    fn test_dfx_args_getter_update() {
        let update_args = UpdateArgs {
            debug: 1,
            log: Some("cmd=debug".to_string()),
            force: 2,
        };

        assert_eq!(update_args.debug_level(), 1);
        assert_eq!(update_args.log_setting(), Some("cmd=debug".to_string()));
    }

    #[test]
    fn test_dfx_args_getter_import() {
        let import_args = ImportArgs {
            debug: 3,
            log: Some("import=debug".to_string()),
            force: 1,
            path: "/test/path".to_string(),
        };

        assert_eq!(import_args.debug_level(), 3);
        assert_eq!(import_args.log_setting(), Some("import=debug".to_string()));
    }

    #[test]
    fn test_dfx_args_getter_localize() {
        let local_args = LocalArgs {
            debug: 1,
            log: Some("local=info".to_string()),
            value: Some("test.yml".to_string()),
            use_default_value: true,
        };

        assert_eq!(local_args.debug_level(), 1);
        assert_eq!(local_args.log_setting(), Some("local=info".to_string()));
    }

    #[test]
    fn test_invalid_system_name() {
        let args = vec!["gops", "new", "--name", ""];
        let result = GInsCmd::try_parse_from(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_import_without_path() {
        let args = vec!["gops", "import"];
        let result = GInsCmd::try_parse_from(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_help_output() {
        let _app = GInsCmd::command();
    }

    #[test]
    fn test_subcommand_help() {
        // Test that help is available without triggering process exit
        let mut app = GInsCmd::command();
        let new_cmd = app.find_subcommand_mut("new").unwrap();

        // Verify that the subcommand has help text
        assert!(new_cmd.get_about().is_some());
        let about_text = new_cmd.get_about().unwrap().to_string();
        assert!(!about_text.is_empty());

        // Test that we can render help without process exit
        let help_text = new_cmd.render_help().to_string();
        assert!(help_text.contains("new"));
        assert!(help_text.contains("--name"));
    }

    #[test]
    fn test_all_commands_parse() {
        // Test that all commands can be parsed without error
        let commands = vec![
            vec!["gops", "new", "--name", "test-system"],
            vec!["gops", "import", "--path", "/test/path"],
            vec!["gops", "update"],
            vec!["gops", "mod", "localize"],
            vec!["gops", "sys", "localize"],
            vec!["gops", "mod", "example"],
            vec!["gops", "mod", "new", "--name", "test-module"],
            vec!["gops", "mod", "update"],
            vec!["gops", "sys", "new", "--name", "test-system"],
            vec!["gops", "sys", "update"],
            vec!["gops", "setting"],
        ];

        for cmd_args in commands {
            let result = GInsCmd::try_parse_from(cmd_args.clone());
            assert!(result.is_ok(), "Failed to parse command: {:?}", cmd_args);
        }
    }

    #[test]
    fn test_commands_with_all_options() {
        let commands = vec![
            vec!["gops", "new", "--name", "test"],
            vec![
                "gops",
                "import",
                "--debug",
                "1",
                "--log",
                "import=debug",
                "--force",
                "2",
                "--path",
                "/test/path",
            ],
            vec![
                "gops",
                "update",
                "--debug",
                "2",
                "--log",
                "update=debug",
                "--force",
                "3",
            ],
            vec![
                "gops",
                "mod",
                "localize",
                "--debug",
                "1",
                "--log",
                "local=debug",
                "--value",
                "test.yml",
                "--default",
            ],
            vec![
                "gops",
                "sys",
                "localize",
                "--debug",
                "1",
                "--log",
                "local=debug",
                "--value",
                "test.yml",
                "--default",
            ],
            vec!["gops", "setting", "--debug", "2", "--log", "setting=debug"],
        ];

        for cmd_args in commands {
            let result = GInsCmd::try_parse_from(cmd_args.clone());
            assert!(
                result.is_ok(),
                "Failed to parse command with options: {:?}",
                cmd_args
            );
        }
    }
}
