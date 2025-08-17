use clap::{Args, Parser};
use derive_getters::Getters;
use galaxy_ops::error::MainResult;
use galaxy_ops::infra::DfxArgsGetter;

use crate::commands::common::DebugLogArgs;

#[derive(Debug, Args, Getters)]
pub struct PrjNewArgs {
    #[arg(short, long, help = "工程配置名称")]
    pub(crate) name: String,
    #[clap(flatten)]
    pub debug_log: DebugLogArgs,
}

#[derive(Debug, Parser)]
pub enum PrjCmd {
    #[command(about = "创建维护工程")]
    New(PrjNewArgs),
}

impl DfxArgsGetter for PrjNewArgs {
    fn debug_level(&self) -> usize {
        self.debug_log.debug_level()
    }
    fn log_setting(&self) -> Option<String> {
        self.debug_log.log_setting()
    }
}

pub struct PrjCommandHandler;

impl PrjCommandHandler {
    pub async fn handle_new(_args: PrjNewArgs) -> MainResult<()> {
        println!("创建维护工程功能待实现");
        Ok(())
    }

    pub async fn execute(cmd: PrjCmd) -> MainResult<()> {
        match cmd {
            PrjCmd::New(args) => Self::handle_new(args).await,
        }
    }
}
