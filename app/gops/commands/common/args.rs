use clap::{ArgAction, Args};
use derive_getters::Getters;

// 通用调试和日志参数结构
#[derive(Debug, Args, Getters)]
pub struct DebugLogArgs {
    #[arg(
        short = 'd',
        long = "debug",
        default_value = "0",
        help = "调试级别 (Debug level): 0=关闭, 1=基础, 2=详细, 3=跟踪\n0=off, 1=basic, 2=verbose, 3=trace"
    )]
    pub debug: usize,

    #[arg(
        long = "log",
        help = "日志级别 (Log level): error, warn, info, debug, trace"
    )]
    pub log: Option<String>,
}

// 通用强制更新参数结构
#[derive(Debug, Args, Getters)]
pub struct ForceArgs {
    #[arg(
        short = 'f',
        long = "force",
        default_value = "0",
        help = "强制更新 (Force update): 0=正常, 1=跳过确认, 2=覆盖文件, 3=强制拉取\n0=normal, 1=skip confirmation, 2=overwrite files, 3=force git pull"
    )]
    pub force: usize,
}

// 通用本地化参数结构
#[derive(Debug, Args, Getters)]
pub struct LocalizeArgs {
    #[arg(
        long = "value",
        help = "包含环境特定值的 YAML/JSON 文件路径 (Path to YAML/JSON file containing environment-specific values)"
    )]
    pub value: Option<String>,

    #[arg(
        long = "default",
        default_value = "false",
        action = ArgAction::SetTrue,
        help = "使用内置默认值而不是用户提供的 value.yml (Use built-in default values instead of user-provided value.yml)"
    )]
    pub use_default_value: bool,
}
