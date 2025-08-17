use clap::{ArgAction, Args, Parser};
use derive_getters::Getters;
use galaxy_ops::infra::DfxArgsGetter;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "gops")]
#[command(
    version,
    about = "Galaxy Operations System - 系统操作管理工具",
    long_about = "Galaxy Operations System - 系统操作管理工具

用于管理系统配置、导入模块、更新引用等操作的核心工具。"
)]
pub enum GInsCmd {
    /// 创建新的系统配置
    ///
    /// 根据提供的参数创建新的系统配置模板
    New(NewArgs),
    /// 导入外部模块到当前系统
    ///
    /// 从指定路径导入模块配置并集成到当前系统
    Import(ImportArgs),
    /// 更新系统模块和引用
    ///
    /// 更新系统模块的引用、依赖关系等配置信息
    Update(UpdateArgs),
    /// 本地化模块配置
    ///
    /// 管理系统级别的配置设置
    Setting(SettingArgs),
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
        let commands = vec![
            vec!["gops", "new", "--name", "test"],
            vec!["gops", "import", "--path", "/test"],
            vec!["gops", "update"],
            vec!["gops", "localize"],
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
