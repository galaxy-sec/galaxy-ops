use super::prelude::*;

use orion_variate::addr::accessor::UniversalAccessor;

pub type AnyResult<T> = anyhow::Result<T>;
#[derive(Clone)]
pub struct SysUpdateValue {
    pub vars: VarCollection,
}
impl SysUpdateValue {
    pub fn new(vars: VarCollection) -> Self {
        Self { vars }
    }
    pub fn vars(&self) -> &VarCollection {
        &self.vars
    }
}

pub type Accessor = Arc<UniversalAccessor>;
#[async_trait]
pub trait InsUpdateable<T> {
    //pub type UpdateObj = T;
    async fn update_local(
        self,
        accessor: Accessor,
        path: &Path,
        options: &DownloadOptions,
    ) -> MainResult<T>;
}

#[async_trait]
pub trait RefUpdateable<T> {
    //pub type UpdateObj = T;
    async fn update_local(
        &self,
        accessor: Accessor,
        path: &Path,
        options: &DownloadOptions,
    ) -> MainResult<T>;
}

#[derive(Clone, Debug, Default, Getters, WithSetters)]
#[getset(get = "pub")]
pub struct LocalizeOptions {
    eval_dict: OriginDict,
    raw_dict: OriginDict,
    #[getset(set_with = "pub")]
    only_mod: Option<String>,
}
impl LocalizeOptions {
    pub fn new(raw_dict: OriginDict) -> Self {
        Self {
            eval_dict: raw_dict.clone().env_eval(&EnvDict::default()),
            raw_dict,
            only_mod: None,
        }
    }
    pub fn evaled_value(&self) -> &OriginDict {
        &self.eval_dict
    }
    pub fn raw_value(&self) -> &OriginDict {
        &self.raw_dict
    }
    pub fn allow_module(&self, mod_name: &str) -> bool {
        if let Some(name) = &self.only_mod {
            return name == mod_name;
        }
        true
    }

    pub fn for_test() -> Self {
        Self {
            eval_dict: OriginDict::new(),
            raw_dict: OriginDict::new(),
            only_mod: None,
        }
    }
}

#[async_trait]
pub trait SystemLocalizable<T> {
    async fn sys_localize(&self, val_path: T, options: LocalizeOptions) -> MainResult<()>;
}

#[async_trait]
pub trait ModuleLocalizable<T> {
    async fn mod_localize(&self, val_path: T, options: LocalizeOptions) -> MainResult<()>;
}
#[derive(Clone, Debug)]
pub enum Value2Path {
    ModOperator(PathBuf),
    SysSetting(PathBuf),
}
impl Value2Path {
    pub fn path(&self) -> &PathBuf {
        match self {
            Value2Path::ModOperator(path_buf) => path_buf,
            Value2Path::SysSetting(path_buf) => path_buf,
        }
    }
    pub fn module_join<S: AsRef<str>>(self, sub: S) {
        match self {
            Value2Path::ModOperator(x) => Value2Path::ModOperator(x.join(sub.as_ref())),
            Value2Path::SysSetting(_) => self,
        };
    }
}

#[derive(Clone, Debug, Getters)]
pub struct ValuePath {
    #[getset(get = "pub")]
    path: PathBuf,
}
pub const VALUE_FILE: &str = "value.yml";
impl ValuePath {
    pub fn new<P: AsRef<Path>>(value: P) -> Self {
        Self {
            //local: PathBuf::from(local.as_ref()),
            path: PathBuf::from(value.as_ref()),
        }
    }
    pub fn from_root(root: PathBuf) -> Self {
        Self { path: root }
    }
    pub fn join_all<P: AsRef<Path>>(&self, path: P) -> Self {
        Self {
            //local: self.local.join(&path),
            path: self.path.join(&path),
        }
    }
    pub fn join<P: AsRef<Path>>(&self, value: P) -> Self {
        Self {
            //local: self.local.join(&local),
            path: self.path.join(&value),
        }
    }
    pub fn value_file(&self) -> PathBuf {
        self.path.join(VALUE_FILE)
    }
    pub fn ensure_exist(self) -> PathResult<Self> {
        ensure_path(&self.path)?;
        Ok(self)
    }
}
