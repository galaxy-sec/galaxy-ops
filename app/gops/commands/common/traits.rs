use galaxy_ops::infra::DfxArgsGetter;

// 为通用参数结构实现 DfxArgsGetter
use crate::commands::common::args::DebugLogArgs;

impl DfxArgsGetter for DebugLogArgs {
    fn debug_level(&self) -> usize {
        self.debug
    }

    fn log_setting(&self) -> Option<String> {
        self.log.clone()
    }
}
