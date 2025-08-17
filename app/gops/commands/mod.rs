pub mod common;
pub mod mod_cmd;
pub mod prj_cmd;
pub mod sys_cmd;

use clap::Parser;
use galaxy_ops::error::MainResult;

pub use mod_cmd::{ModCmd, ModCommandHandler};
pub use prj_cmd::{PrjCmd, PrjCommandHandler};
pub use sys_cmd::{SysCmd, SysCommandHandler};

#[derive(Debug, Parser)]
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

pub struct CommandDispatcher;

impl CommandDispatcher {
    pub async fn dispatch(cmd: GInsCmd) -> MainResult<()> {
        match cmd {
            GInsCmd::Mod(mod_cmd) => ModCommandHandler::execute(mod_cmd).await,
            GInsCmd::Prj(prj_cmd) => PrjCommandHandler::execute(prj_cmd).await,
            GInsCmd::Sys(sys_cmd) => SysCommandHandler::execute(sys_cmd).await,
        }
    }
}
