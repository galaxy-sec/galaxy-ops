use super::prelude::*;
use std::net::Ipv4Addr;

use crate::types::{SysUpdateValue, ValuePath};
use derive_more::Deref;
use orion_variate::vars::{ValueDict, ValueType, VarCollection};

use crate::error::MainResult;
use crate::module::refs::ModuleSpecRef;
use crate::module::spec::ModuleSpec;

#[derive(Getters, Clone, Debug, Default, Serialize, Deserialize, Deref)]
#[getset(get = "pub")]
#[serde(transparent)]
pub struct ModulesList {
    mods: Vec<ModuleSpecRef>,
    //#[serde(skip)]
    //mod_map: HashMap<String, ModuleSpec>,
}
impl ModulesList {
    pub fn add_ref(&mut self, spec_ref: ModuleSpecRef) {
        self.mods.push(spec_ref);
    }
    pub fn export(&self) -> ValueDict {
        let mut dict = ValueDict::new();
        for item in self.mods().iter() {
            if item.is_enable() {
                dict.insert(item.name(), ValueType::from(item.name().as_str()));
            }
        }
        dict
    }

    pub fn set_mods_local(&mut self, spec_path: PathBuf) {
        self.mods
            .iter_mut()
            .for_each(|x| x.set_local(spec_path.join("mods")));
    }

    pub fn find(&self, arg: &str) -> Option<&ModuleSpecRef> {
        self.mods.iter().find(|x| x.name() == arg)
    }
}

#[async_trait]
impl RefUpdateable<SysUpdateValue> for ModulesList {
    async fn update_local(
        &self,
        accessor: Accessor,
        sys_root: &Path,
        options: &DownloadOptions,
    ) -> MainResult<SysUpdateValue> {
        let mut vars = VarCollection::default();
        for m in &self.mods {
            if m.is_enable() {
                let update_v = m.update_local(accessor.clone(), sys_root, options).await?;
                if let Some(v) = update_v.vars {
                    vars = vars.merge_system(v);
                }
            }
        }
        Ok(SysUpdateValue::new(vars))
    }
}

impl ModulesList {
    pub fn value_path(&self, parent: ValuePath) -> ValuePath {
        parent.join_all("mods")
    }
}
#[async_trait]
impl SystemLocalizable<SysValuePaths> for ModulesList {
    async fn sys_localize(
        &self,
        val_path: SysValuePaths,
        options: LocalizeOptions,
    ) -> MainResult<()> {
        //let root = val_path.join("mods");
        for m in &self.mods {
            if m.is_enable() && options.allow_module(m.name()) {
                m.sys_localize(val_path.clone(), options.clone()).await?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NoneValue<T> {
    None,
    Value(T),
}
impl ModulesList {
    pub fn add_mod(&mut self, _modx: ModuleSpec) {
        todo!();
        //self.mod_map.insert(modx.name().clone(), modx);
    }
}

#[derive(Getters, Clone, Debug, Serialize, Deserialize)]
#[getset(get = "pub")]
pub struct NetResSpace {
    master: Ipv4Addr,
    node_scope: (Ipv4Addr, Ipv4Addr),
}
impl NetResSpace {
    pub fn new(master: Ipv4Addr, node_scope: (Ipv4Addr, Ipv4Addr)) -> Self {
        Self { master, node_scope }
    }
}
