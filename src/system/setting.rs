use super::prelude::*;

use crate::{
    const_vars::VARS_YML,
    local::{LocalizeExecPath, LocalizeVarPath},
    project::mix_used_value,
    types::ModuleLocalizable,
};

use async_trait::async_trait;
use indexmap::IndexMap;
use orion_conf::YamlStorageExt;
use orion_variate::vars::EnvEvalable;

use crate::{error::MainResult, module::ModelSTD};

#[derive(Getters, Clone, Debug, Serialize, Deserialize)]
#[getset(get = "pub ")]
pub struct ModSetting {
    enable: bool,
    localize: LocalizeVarPath,
}
impl ModSetting {
    pub fn disaple_new(module: &str, model: &str) -> Self {
        Self {
            enable: false,
            localize: LocalizeVarPath::of_module(module, model),
        }
    }
    pub fn enable_new(module: &str, model: &str) -> Self {
        Self {
            enable: true,
            localize: LocalizeVarPath::of_module(module, model),
        }
    }
}
#[derive(Getters, Clone, Debug, Serialize, Deserialize)]
#[getset(get = "pub ")]
#[serde(transparent)]
pub struct LocalizeDict {
    dicts: IndexMap<String, ModSetting>,
}
impl LocalizeDict {
    pub fn example() -> Self {
        let mut dicts = IndexMap::new();
        dicts.insert(
            "example".to_string(),
            ModSetting::disaple_new("gflow", ModelSTD::x86_ubt22_host().to_string().as_str()),
        );

        Self { dicts }
    }
}
impl StorageLoadEvent for LocalizeDict {
    fn loaded_event_do(&mut self) {}
}

#[derive(Getters, Clone, Debug)]
#[getset(get = "pub ")]
pub struct SysSetting {
    vars: VarCollection,
    list: LocalizeDict,
    root: Option<PathBuf>,
}
impl StorageLoadEvent for SysSetting {
    fn loaded_event_do(&mut self) {
        self.vars.mark_vars_scope();
    }
}
impl SysSetting {
    pub fn new(vars: VarCollection) -> Self {
        SysSetting {
            vars,
            list: LocalizeDict::example(),
            root: None,
        }
    }
    pub fn add_mod_setting<S: Into<String>>(&mut self, mod_name: S, mod_setting: ModSetting) {
        self.list.dicts.insert(mod_name.into(), mod_setting);
    }

    pub fn example() -> Self {
        SysSetting {
            vars: VarCollection::define(vec![
                VarDefinition::from(("HOME", "${HOME}")).with_mut_immutable(),
                VarDefinition::from(("SYS_KEY1", "sys_value1")).with_mut_module(),
                VarDefinition::from(("SYS_KEY2", "sys_value2")).with_mut_system(),
            ]),
            list: LocalizeDict::example(),
            root: None,
        }
    }
    pub fn save_local(&self, path: &Path) -> MainResult<()> {
        let vars_file_name = path.join(VARS_YML);
        let list_file_name = path.join("list.yml");
        self.vars.save_yml(&vars_file_name).owe_res()?;
        self.list.save_yml(&list_file_name).owe_res()?;
        Ok(())
    }
    pub fn load_from(root: &Path) -> MainResult<Self> {
        let vars_file_name = root.join(VARS_YML);
        let list_file_name = root.join("list.yml");
        let vars = VarCollection::from_yml(&vars_file_name).owe_res()?;
        let list = LocalizeDict::from_yml(&list_file_name).owe_res()?;
        let root = Some(root.to_path_buf());
        Ok(SysSetting { vars, list, root })
    }
}

#[async_trait]
impl SystemLocalizable<SysValuePaths> for SysSetting {
    async fn sys_localize(
        &self,
        val_path: SysValuePaths,
        options: LocalizeOptions,
    ) -> MainResult<()> {
        let used_value_file = self
            .root()
            .clone()
            .expect("setting root miss")
            .join("_used.json");
        for (_k, v) in self.list.dicts() {
            if !v.enable {
                continue;
            }
            let cur_used_file = used_value_file.clone();
            let mut ctx = OperationContext::want("sys-setting localize")
                .with_auto_log()
                .with_mod_path("sys/setting");
            let dict = options.clone().evaled_value().export_dict();
            let exe_setting = LocalizeExecPath::from(v.localize.clone().env_eval(&dict));

            let used = mix_used_value(options.clone(), &self.vars, &val_path.mod_value_file())?;
            orion_conf::Yamlable::save_yml(&used.export_origin(), &val_path.used_with_origon())
                .owe_res()?;
            ctx.record("value_file", &cur_used_file);
            used.export_value().save_json(&cur_used_file).owe_res()?;

            exe_setting
                .mod_localize(cur_used_file, options.clone())
                .await?;
            ctx.mark_suc();
        }
        Ok(())
    }
}
