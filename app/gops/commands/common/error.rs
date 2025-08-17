//! 通用错误处理模块
//!
//! 这个模块重新导出 `MainResult` 类型，供命令域统一使用。

// 重新导出 MainResult 类型，供命令域使用
#[allow(unused_imports)]
pub use galaxy_ops::error::MainResult;

// 通用的错误处理函数 - 暂时注释掉，因为当前未使用
// pub fn handle_main_result<T>(result: MainResult<T>) -> MainResult<T> {
//     result
// }

// 注意：configure_dfx_logging 函数需要 DfxArgsGetter trait，
// 但为了避免循环依赖，我们在每个命令模块中直接使用：
// galaxy_ops::infra::configure_dfx_logging
