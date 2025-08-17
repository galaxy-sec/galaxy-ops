use clap::{Args, Parser};
use derive_getters::Getters;
use galaxy_ops::error::MainResult;
use galaxy_ops::infra::DfxArgsGetter;
use galaxy_ops::module::ModelSTD;
use galaxy_ops::project::load_project_global_value;
use galaxy_ops::system::proj::SysProject;
use galaxy_ops::types::{LocalizeOptions, RefUpdateable};
use inquire::Select;
use orion_error::{ErrorConv, ErrorOwe};
use orion_infra::path::make_new_path;
use orion_variate::update::DownloadOptions;
use orion_variate::vars::ValueDict;

use crate::commands::common::{DebugLogArgs, ForceArgs, LocalizeArgs};

// === 参数定义 ===

#[derive(Debug, Args, Getters)]
pub struct SysNewArgs {
    #[arg(
        short,
        long,
        help = "系统名称 (System name): 字母数字，可包含连字符和下划线\nalphanumeric with hyphens/underscores"
    )]
    pub(crate) name: String,
}

#[derive(Debug, Args, Getters)]
pub struct SysUpdateArgs {
    #[clap(flatten)]
    pub debug_log: DebugLogArgs,

    #[clap(flatten)]
    pub force: ForceArgs,
}

#[derive(Debug, Args, Getters)]
pub struct SysLocalizeArgs {
    #[clap(flatten)]
    pub debug_log: DebugLogArgs,

    #[clap(flatten)]
    pub localize: LocalizeArgs,
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

// === DfxArgsGetter 实现 ===

impl DfxArgsGetter for SysNewArgs {
    fn debug_level(&self) -> usize {
        0
    }
    fn log_setting(&self) -> Option<String> {
        None
    }
}

impl DfxArgsGetter for SysUpdateArgs {
    fn debug_level(&self) -> usize {
        self.debug_log.debug_level()
    }
    fn log_setting(&self) -> Option<String> {
        self.debug_log.log_setting()
    }
}

impl DfxArgsGetter for SysLocalizeArgs {
    fn debug_level(&self) -> usize {
        self.debug_log.debug_level()
    }
    fn log_setting(&self) -> Option<String> {
        self.debug_log.log_setting()
    }
}

// === 命令处理器 ===

pub struct SysCommandHandler;

impl SysCommandHandler {
    fn ia_model_std() -> MainResult<ModelSTD> {
        let support_models = ModelSTD::support();
        let options: Vec<String> = support_models
            .iter()
            .map(|model| format!("{model}"))
            .collect();

        // 检查是否在测试环境中
        if std::env::var("TEST_MODE").is_ok() {
            // 在测试环境中，自动选择第一个支持的模式
            if let Some(first_model) = support_models.first() {
                return Ok(first_model.clone());
            } else {
                return Ok(ModelSTD::from_cur_sys());
            }
        }

        let selection = Select::new("请选择系统型号配置:", options.clone())
            .prompt()
            .unwrap();

        // 从预定义选项中选择
        let index = options.iter().position(|s| s == &selection).unwrap();
        if index < support_models.len() {
            Ok(support_models[index].clone())
        } else {
            Ok(ModelSTD::from_cur_sys()) // 兜底处理
        }
    }

    pub async fn handle_new(args: SysNewArgs) -> MainResult<()> {
        let current_dir = std::env::current_dir().expect("无法获取当前目录");
        let new_prj = current_dir.join(args.name());
        make_new_path(&new_prj).owe_res()?;

        let model_in = Self::ia_model_std()?;
        let spec = SysProject::make_new(&new_prj, args.name(), model_in).err_conv()?;
        spec.save().err_conv()?;
        Ok(())
    }

    pub async fn handle_update(args: SysUpdateArgs) -> MainResult<()> {
        let current_dir = std::env::current_dir().expect("无法获取当前目录");
        galaxy_ops::infra::configure_dfx_logging(&args);

        let options = DownloadOptions::from((*args.force.force(), ValueDict::default()));
        let spec = SysProject::load(&current_dir).err_conv()?;
        let accessor = galaxy_ops::accessor::accessor_for_default();

        spec.update_local(accessor, &current_dir, &options)
            .await
            .err_conv()?;
        Ok(())
    }

    pub async fn handle_localize(args: SysLocalizeArgs) -> MainResult<()> {
        let current_dir = std::env::current_dir().expect("无法获取当前目录");
        galaxy_ops::infra::configure_dfx_logging(&args);

        let spec = SysProject::load(&current_dir).err_conv()?;
        let dict = load_project_global_value(spec.root_local(), args.localize.value())?;
        spec.localize(LocalizeOptions::new(
            dict,
            *args.localize.use_default_value(),
        ))
        .await
        .err_conv()?;
        Ok(())
    }

    pub async fn execute(cmd: SysCmd) -> MainResult<()> {
        match cmd {
            SysCmd::New(args) => Self::handle_new(args).await,
            SysCmd::Update(args) => Self::handle_update(args).await,
            SysCmd::Localize(args) => Self::handle_localize(args).await,
        }
    }
}

// === 测试 ===

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_sys_new_command() {
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        unsafe {
            std::env::set_var("TEST_MODE", "true");
        }

        let args = SysNewArgs {
            name: "test_system".to_string(),
        };

        let result = SysCommandHandler::handle_new(args).await;
        unsafe {
            std::env::remove_var("TEST_MODE");
        }

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sys_update_command() {
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let args = SysUpdateArgs {
            debug_log: DebugLogArgs {
                debug: 0,
                log: None,
            },
            force: ForceArgs { force: 0 },
        };

        let result = SysCommandHandler::handle_update(args).await;
        // 预期会失败，因为没有现有的系统项目
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_sys_localize_command() {
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let args = SysLocalizeArgs {
            debug_log: DebugLogArgs {
                debug: 0,
                log: None,
            },
            localize: LocalizeArgs {
                value: None,
                use_default_value: true,
            },
        };

        let result = SysCommandHandler::handle_localize(args).await;
        // 预期会失败，因为没有现有的系统项目
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ia_model_std() {
        unsafe {
            std::env::set_var("TEST_MODE", "true");
        }

        let result = SysCommandHandler::ia_model_std();
        unsafe {
            std::env::remove_var("TEST_MODE");
        }

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_sys_commands() {
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        unsafe {
            std::env::set_var("TEST_MODE", "true");
        }

        // 测试 new 命令
        let new_cmd = SysCmd::New(SysNewArgs {
            name: "test_system".to_string(),
        });
        let result = SysCommandHandler::execute(new_cmd).await;
        assert!(result.is_ok());

        unsafe {
            std::env::remove_var("TEST_MODE");
        }
    }

    #[test]
    fn test_sys_new_args_getter() {
        let args = SysNewArgs {
            name: "test_system".to_string(),
        };

        assert_eq!(args.debug_level(), 0);
        assert_eq!(args.log_setting(), None);
        assert_eq!(args.name(), "test_system");
    }

    #[test]
    fn test_sys_update_args_getter() {
        let args = SysUpdateArgs {
            debug_log: DebugLogArgs {
                debug: 2,
                log: Some("info".to_string()),
            },
            force: ForceArgs { force: 1 },
        };

        assert_eq!(args.debug_level(), 2);
        assert_eq!(args.log_setting(), Some("info".to_string()));
        assert_eq!(*args.force.force(), 1);
    }

    #[test]
    fn test_sys_localize_args_getter() {
        let args = SysLocalizeArgs {
            debug_log: DebugLogArgs {
                debug: 1,
                log: Some("debug".to_string()),
            },
            localize: LocalizeArgs {
                value: Some("test.yml".to_string()),
                use_default_value: false,
            },
        };

        assert_eq!(args.debug_level(), 1);
        assert_eq!(args.log_setting(), Some("debug".to_string()));
        assert_eq!(*args.localize.value(), Some("test.yml".to_string()));
        assert!(!*args.localize.use_default_value());
    }
}
