use std::path::{Path, PathBuf};

use getset::Getters;
use orion_conf::ErrorOwe;
use orion_infra::path::ensure_path;

use crate::{error::MainResult, types::ValuePath};

#[derive(Debug, Clone)]
pub struct InstallPaths {
    pub source_path: PathBuf,
    pub temp_target_path: PathBuf,
    pub final_target_path: PathBuf,
    pub project_root: PathBuf,
}

#[derive(Getters, Clone, Debug)]
pub struct ProjectPath {
    #[getset(get = "pub")]
    root: PathBuf,
}

impl ProjectPath {
    /// 创建新的 ProjectPath 实例
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self {
            root: PathBuf::from(root.as_ref()),
        }
    }

    /// 获取项目配置文件路径 (ops-prj.yml)
    pub fn conf_file(&self) -> PathBuf {
        self.root.join("ops-prj.yml")
    }

    /// 获取目标配置文件路径 (ops-systems.yml)
    pub fn target_file(&self) -> PathBuf {
        self.root.join("ops-systems.yml")
    }

    /// 获取值目录路径 (values/)
    pub fn value_dir(&self) -> PathBuf {
        self.root.join("values")
    }

    /// 获取值文件路径 (values/_value.yml)
    pub fn value_file(&self) -> PathBuf {
        self.value_dir().join("_value.yml")
    }

    /// 转换为 ValuePath，与现有 API 兼容
    pub fn to_value_path(&self) -> ValuePath {
        ValuePath::from_root(self.value_dir())
    }

    /// 确保项目根目录存在
    pub fn ensure_root_exists(&self) -> MainResult<()> {
        ensure_path(&self.root).owe_logic()?;
        Ok(())
    }
}
