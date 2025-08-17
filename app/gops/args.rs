use clap::{ArgAction, Args, Parser};
use derive_getters::Getters;
use galaxy_ops::infra::DfxArgsGetter;

// Mod子命令参数结构
#[derive(Debug, Args, Getters)]
pub struct ModExampleArgs {
    /// 启用调试输出并指定级别 (0-3) (Enable debug output with specified level (0-3))
    #[arg(
        short = 'd',
        long = "debug",
        default_value = "0",
        help = "调试级别 (Debug level): 0=关闭, 1=基础, 2=详细, 3=跟踪\n0=off, 1=basic, 2=verbose, 3=trace"
    )]
    pub debug: usize,
    /// 设置日志级别和格式 (Set logging level and format)
    #[arg(
        long = "log",
        help = "日志级别 (Log level): error, warn, info, debug, trace"
    )]
    pub log: Option<String>,
}

#[derive(Debug, Args, Getters)]
pub struct ModNewArgs {
    /// 要创建的新模块名称 (Name of new module to create)
    #[arg(
        short,
        long,
        help = "模块名称 (Module name): 字母数字，可包含连字符和下划线\nalphanumeric with hyphens/underscores"
    )]
    pub(crate) name: String,

    /// 启用调试输出并指定级别 (0-3) (Enable debug output with specified level (0-3))
    #[arg(
        short = 'd',
        long = "debug",
        default_value = "0",
        help = "调试级别 (Debug level): 0=关闭, 1=基础, 2=详细, 3=跟踪\n0=off, 1=basic, 2=verbose, 3=trace"
    )]
    pub debug: usize,
    /// 设置日志级别和格式 (Set logging level and format)
    #[arg(
        long = "log",
        help = "日志级别 (Log level): error, warn, info, debug, trace"
    )]
    pub log: Option<String>,
}

#[derive(Debug, Args, Getters)]
pub struct ModUpdateArgs {
    /// 启用调试输出并指定级别 (0-3) (Enable debug output with specified level (0-3))
    #[arg(
        short = 'd',
        long = "debug",
        default_value = "0",
        help = "调试级别 (Debug level): 0=关闭, 1=基础, 2=详细, 3=跟踪\n0=off, 1=basic, 2=verbose, 3=trace"
    )]
    pub debug: usize,
    /// 设置日志级别和格式 (Set logging level and format)
    #[arg(
        long = "log",
        help = "日志级别 (Log level): error, warn, info, debug, trace"
    )]
    pub log: Option<String>,

    /// 即使存在冲突也强制更新 (Force update even if conflicts exist)
    #[arg(
        short = 'f',
        long = "force",
        default_value = "0",
        help = "强制更新 (Force update): 跳过确认，覆盖现有文件\nskip confirmation, overwrite existing files"
    )]
    pub force: usize,
}

#[derive(Debug, Args, Getters)]
pub struct ModLocalizeArgs {
    /// 启用调试输出并指定级别 (0-3) (Enable debug output with specified level (0-3))
    #[arg(
        short = 'd',
        long = "debug",
        default_value = "0",
        help = "调试级别 (Debug level): 0=关闭, 1=基础, 2=详细, 3=跟踪\n0=off, 1=basic, 2=verbose, 3=trace"
    )]
    pub debug: usize,
    /// 设置日志级别和格式 (Set logging level and format)
    #[arg(
        long = "log",
        help = "日志级别 (Log level): error, warn, info, debug, trace"
    )]
    pub log: Option<String>,

    /// 用于本地化的值文件路径 (Path to values file for localization)
    #[arg(
        long = "value",
        help = "包含环境特定值的 YAML/JSON 文件路径 (Path to YAML/JSON file containing environment-specific values)"
    )]
    pub value: Option<String>,
    /// 使用默认值而不是用户提供的 value.yml (Use default values instead of user-provided value.yml)
    #[arg(long = "default", default_value = "false" , action = ArgAction::SetTrue, help = "使用内置默认值而不是用户提供的 value.yml (Use built-in default values instead of user-provided value.yml)")]
    pub use_default_value: bool,
}

#[derive(Debug, Parser)]
pub enum ModCmd {
    /// 创建示例模块结构 (Create Example Module Structure)
    #[command(
        about = "创建示例模块结构 (Create Example Module Structure)",
        long_about = "创建一个完整的示例模块结构，包含示例配置和工作流，以展示模块组织和最佳实践。\n\
                     Create a complete example module structure with sample configurations and workflows to demonstrate module organization and best practices."
    )]
    Example(ModExampleArgs),
    /// 定义新的模块操作符 (Define New Module Operator)
    #[command(
        about = "定义新的模块操作符 (Define New Module Operator)",
        long_about = "使用给定的名称创建新的模块规范。这将初始化一个新的模块目录结构，其中包含所有必要的配置文件。\n\
                     Create a new module specification with the given name. This will initialize a new module directory structure with all necessary configuration files."
    )]
    New(ModNewArgs),
    /// 更新现有模块操作符 (Update Existing Module Operator)
    #[command(
        about = "更新现有模块操作符 (Update Existing Module Operator)",
        long_about = "更新现有模块的配置、依赖关系或规范。支持强制更新以覆盖现有配置。\n\
                     Update an existing module's configuration, dependencies, or specifications. Supports force updates to override existing configurations."
    )]
    Update(ModUpdateArgs),
    /// 本地化模块配置 (Localize Module Configuration)
    #[command(
        about = "本地化模块配置 (Localize Module Configuration)",
        long_about = "基于环境特定值为模块生成本地化配置文件。适用于将模块适配到不同的部署环境。\n\
                     Generate localized configuration files for the module based on environment-specific values. Useful for adapting modules to different deployment environments."
    )]
    Localize(ModLocalizeArgs),
}

// Sys子命令参数结构
#[derive(Debug, Args, Getters)]
pub struct SysNewArgs {
    /// 要创建的新系统名称 (Name of the new system to create)
    #[arg(
        short,
        long,
        help = "系统名称 (System name): 字母数字，可包含连字符和下划线\nalphanumeric with hyphens/underscores"
    )]
    pub(crate) name: String,
}

#[derive(Debug, Args, Getters)]
pub struct SysUpdateArgs {
    /// 启用调试输出并指定级别 (0-4) (Enable debug output with specified level (0-4))
    #[arg(
        short = 'd',
        long = "debug",
        default_value = "0",
        help = "调试级别 (Debug level): 0=关闭, 1=基础, 2=详细, 3=跟踪, 4=完整\n0=off, 1=basic, 2=verbose, 3=trace, 4=full"
    )]
    pub debug: usize,
    /// 配置日志输出格式和级别 (Configure logging output format and levels)
    #[arg(
        long = "log",
        help = "配置日志 (Configure logging): 例如 --log cmd=debug,parse=info\neg --log cmd=debug,parse=info"
    )]
    pub log: Option<String>,

    /// 强制更新级别 (0-3) (Force update level (0-3))
    #[arg(
        short = 'f',
        long = "force",
        default_value = "0",
        help = "强制更新 (Force update): 0=正常, 1=跳过确认, 2=覆盖文件, 3=强制拉取\n0=normal, 1=skip confirmation, 2=overwrite files, 3=force git pull"
    )]
    pub force: usize,
}

#[derive(Debug, Args, Getters)]
pub struct SysLocalizeArgs {
    /// 启用调试输出并指定级别 (0-4) (Enable debug output with specified level (0-4))
    #[arg(
        short = 'd',
        long = "debug",
        default_value = "0",
        help = "调试级别 (Debug level): 0=关闭, 1=基础, 2=详细, 3=跟踪, 4=完整\n0=off, 1=basic, 2=verbose, 3=trace, 4=full"
    )]
    pub debug: usize,
    /// 配置日志输出格式和级别 (Configure logging output format and levels)
    #[arg(
        long = "log",
        help = "配置日志 (Configure logging): 例如 --log cmd=debug,parse=info\neg --log cmd=debug,parse=info"
    )]
    pub log: Option<String>,

    /// 用于本地化的值文件路径 (Path to values file for localization)
    #[arg(
        long = "value",
        help = "包含环境特定值的 YAML/JSON 文件路径 (Path to YAML/JSON file containing environment-specific values)"
    )]
    pub value: Option<String>,

    /// 使用默认值而不是用户提供的 value.yml (Use default values instead of user-provided value.yml)
    #[arg(long = "default", default_value = "false" , action = ArgAction::SetTrue, help = "使用内置默认值而不是用户提供的 value.yml (Use built-in default values instead of user-provided value.yml)")]
    pub use_default_value: bool,
}

#[derive(Debug, Args, Getters)]
pub struct PrjNewArgs {
    /// 工程配置名称 (Project Configuration Name)
    #[arg(short, long, help = "工程配置名称 (Project configuration name)")]
    pub(crate) name: String,

    /// 调试输出级别 (Debug Output Level)
    #[arg(
        short = 'd',
        long = "debug",
        default_value = "0",
        help = "调试级别 (Debug level): 0=关闭, 1=基础, 2=详细, 3=完整\n\
                 0=off, 1=basic, 2=verbose, 3=full"
    )]
    pub debug: usize,

    /// 日志配置 (Logging Configuration)
    #[arg(
        long = "log",
        help = "日志配置 (Log configuration): error, warn, info, debug, trace"
    )]
    pub log: Option<String>,
}

#[derive(Debug, Args, Getters)]
pub struct PrjImportArgs {
    /// 系统导入路径 (System Import Path)
    #[arg(short = 'p', long = "path", help = "系统导入路径 (System import path)")]
    pub path: String,

    /// 调试输出级别 (Debug Output Level)
    #[arg(
        short = 'd',
        long = "debug",
        default_value = "0",
        help = "调试级别 (Debug level): 0=关闭, 1=基础, 2=详细, 3=完整\n\
                 0=off, 1=basic, 2=verbose, 3=full"
    )]
    pub debug: usize,

    /// 日志配置 (Logging Configuration)
    #[arg(
        long = "log",
        help = "日志配置 (Log configuration): error, warn, info, debug, trace"
    )]
    pub log: Option<String>,

    /// 强制更新级别 (Force Update Level)
    #[arg(
        short = 'f',
        long = "force",
        default_value = "0",
        help = "强制更新级别 (Force update level): 0=正常, 1=跳过确认, 2=覆盖文件, 3=强制拉取\n\
                 0=normal, 1=skip confirmation, 2=overwrite files, 3=force git pull"
    )]
    pub force: usize,
}

#[derive(Debug, Args, Getters)]
pub struct PrjUpdateArgs {
    /// 调试输出级别 (Debug Output Level)
    #[arg(
        short = 'd',
        long = "debug",
        default_value = "0",
        help = "调试级别 (Debug level): 0=关闭, 1=基础, 2=详细, 3=完整\n\
                 0=off, 1=basic, 2=verbose, 3=full"
    )]
    pub debug: usize,

    /// 日志配置 (Logging Configuration)
    #[arg(
        long = "log",
        help = "日志配置 (Log configuration): error, warn, info, debug, trace"
    )]
    pub log: Option<String>,

    /// 强制更新级别 (Force Update Level)
    #[arg(
        short = 'f',
        long = "force",
        default_value = "0",
        help = "强制更新级别 (Force update level): 0=正常, 1=跳过确认, 2=覆盖文件, 3=强制拉取\n\
                 0=normal, 1=skip confirmation, 2=overwrite files, 3=force git pull"
    )]
    pub force: usize,
}

#[derive(Debug, Args, Getters)]
pub struct PrjSettingArgs {
    /// 调试输出级别 (Debug Output Level)
    #[arg(
        short = 'd',
        long = "debug",
        default_value = "0",
        help = "调试级别 (Debug level): 0=关闭, 1=基础, 2=详细, 3=完整\n\
                 0=off, 1=basic, 2=verbose, 3=full"
    )]
    pub debug: usize,

    /// 日志配置 (Logging Configuration)
    #[arg(
        long = "log",
        help = "日志配置 (Log configuration): error, warn, info, debug, trace"
    )]
    pub log: Option<String>,
}

#[derive(Debug, Parser)]
pub enum PrjCmd {
    /// 创建维护工程 (Create Maintenance Project)
    #[command(
        about = "创建维护工程 (Create Maintenance Project)",
        long_about = "创建新的维护工程，包含所有必要的配置文件和目录结构。\n\
                     Create a new maintenance project with all necessary configuration files and directory structure."
    )]
    New(PrjNewArgs),

    /// 导入系统到工程 (Import System to Project)
    #[command(
        about = "导入系统到工程 (Import System to Project)",
        long_about = "从指定路径导入系统到当前工程，集成到现有的项目结构中。\n\
                     Import a system from specified path to current project, integrating it into the existing project structure."
    )]
    Import(PrjImportArgs),

    /// 维护工程 (Maintain Project)
    #[command(
        about = "维护工程 (Maintain Project)",
        long_about = "更新现有工程的配置、依赖关系和引用信息。支持强制更新。\n\
                     Update existing project configuration, dependencies, and references. Supports force updates."
    )]
    Update(PrjUpdateArgs),

    /// 工程设置 (Project Settings)
    #[command(
        about = "工程设置 (Project Settings)",
        long_about = "管理系统级别的配置设置，提供交互式配置界面。\n\
                     Manage system-level configuration settings, providing interactive configuration interface."
    )]
    Setting(PrjSettingArgs),
}

#[derive(Debug, Parser)]
pub enum SysCmd {
    /// 创建新的系统操作符 (Create New System Operator)
    #[command(
        about = "创建新的系统操作符 (Create New System Operator)",
        long_about = "使用给定的名称创建新的系统规范。这将初始化一个新的系统目录结构，其中包含所有必要的配置文件和模板。\n\
                     Create a new system specification with the given name. This will initialize a new system directory structure with all necessary configuration files and templates."
    )]
    New(SysNewArgs),
    /// 更新系统配置 (Update System Configuration)
    #[command(
        about = "更新系统配置 (Update System Configuration)",
        long_about = "更新现有系统的配置、规范或依赖关系。支持强制更新以在不确认的情况下覆盖现有配置。\n\
                     Update an existing system's configuration, specifications, or dependencies. Supports force updates to override existing configurations without confirmation."
    )]
    Update(SysUpdateArgs),
    /// 为环境本地化系统配置 (Localize System Configuration for Environment)
    #[command(
        about = "为环境本地化系统配置 (Localize System Configuration for Environment)",
        long_about = "基于环境特定值为系统生成本地化配置文件。适用于将系统配置适配到不同的部署环境。\n\
                     Generate localized configuration files for the system based on environment-specific values. Useful for adapting system configurations to different deployment environments."
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
    /// 工程管理命令 (Project Management Commands)
    #[command(subcommand, about = "工程管理命令 (Project Management Commands)")]
    Prj(PrjCmd),

    /// 模块管理命令 (Module Management Commands)
    #[command(subcommand, about = "模块管理命令 (Module Management Commands)")]
    Mod(ModCmd),

    /// 系统管理命令 (System Management Commands)
    #[command(subcommand, about = "系统管理命令 (System Management Commands)")]
    Sys(SysCmd),
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
    fn test_prj_new_command_parsing() {
        let args = vec!["gops", "prj", "new", "--name", "test-system"];
        let cmd = GInsCmd::try_parse_from(args).unwrap();

        match cmd {
            GInsCmd::Prj(PrjCmd::New(new_args)) => {
                assert_eq!(new_args.name(), "test-system");
            }
            _ => panic!("Expected Prj New command"),
        }
    }

    #[test]
    fn test_prj_import_command_parsing() {
        let args = vec!["gops", "prj", "import", "--path", "/path/to/module"];
        let cmd = GInsCmd::try_parse_from(args).unwrap();

        match cmd {
            GInsCmd::Prj(PrjCmd::Import(import_args)) => {
                assert_eq!(import_args.path(), "/path/to/module");
                assert_eq!(*import_args.debug(), 0);
                assert_eq!(import_args.force, 0);
            }
            _ => panic!("Expected Prj Import command"),
        }
    }

    #[test]
    fn test_prj_import_command_with_options() {
        let args = vec![
            "gops",
            "prj",
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
            GInsCmd::Prj(PrjCmd::Import(import_args)) => {
                assert_eq!(import_args.path(), "/path/to/module");
                assert_eq!(*import_args.debug(), 2);
                assert_eq!(import_args.force, 1);
            }
            _ => panic!("Expected Prj Import command"),
        }
    }

    #[test]
    fn test_prj_update_command_parsing() {
        let args = vec!["gops", "prj", "update"];
        let cmd = GInsCmd::try_parse_from(args).unwrap();

        match cmd {
            GInsCmd::Prj(PrjCmd::Update(update_args)) => {
                assert_eq!(*update_args.debug(), 0);
                assert_eq!(update_args.force, 0);
            }
            _ => panic!("Expected Prj Update command"),
        }
    }

    #[test]
    fn test_prj_update_with_force() {
        let args = vec!["gops", "prj", "update", "--force", "2"];
        let cmd = GInsCmd::try_parse_from(args).unwrap();

        match cmd {
            GInsCmd::Prj(PrjCmd::Update(update_args)) => {
                assert_eq!(update_args.force, 2);
            }
            _ => panic!("Expected Prj Update command"),
        }
    }

    #[test]
    fn test_prj_setting_command_parsing() {
        let args = vec!["gops", "prj", "setting"];
        let cmd = GInsCmd::try_parse_from(args).unwrap();

        match cmd {
            GInsCmd::Prj(PrjCmd::Setting(setting_args)) => {
                assert_eq!(*setting_args.debug(), 0);
            }
            _ => panic!("Expected Prj Setting command"),
        }
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
        let args = vec!["gops", "prj", "import"];
        let cmd = GInsCmd::try_parse_from(args);
        assert!(cmd.is_err());
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
        let commands = vec![
            vec!["gops", "prj", "new", "--name", "test-project"],
            vec!["gops", "prj", "import", "--path", "/test/path"],
            vec!["gops", "prj", "update"],
            vec!["gops", "prj", "setting"],
            vec!["gops", "mod", "example"],
            vec!["gops", "mod", "new", "--name", "test-module"],
            vec!["gops", "mod", "update"],
            vec!["gops", "mod", "localize"],
            vec!["gops", "sys", "new", "--name", "test-system"],
            vec!["gops", "sys", "update"],
            vec!["gops", "sys", "localize"],
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
