use super::prelude::*;
use crate::error::{MainError, ModReason};
use crate::local::{LocalizeExecPath, LocalizeVarPath};
use crate::predule::*;

use orion_error::UvsLogicFrom;
use orion_variate::types::ResourceDownloader;
use orion_variate::vars::EnvEvalable;

use super::ModelSTD;
use crate::types::{Localizable, LocalizeOptions, RefUpdateable, ValuePath};
use crate::{const_vars::MOD_DIR, error::MainResult, module::model::ModModelSpec};

#[derive(Getters, Clone, Debug, Serialize, Deserialize)]
pub struct ModuleSpecRef {
    name: String,
    addr: Address,
    #[serde(alias = "node")]
    model: ModelSTD,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    enable: Option<bool>,
    #[serde(skip)]
    local: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    setting: Option<LocalizeVarPath>,
}

impl ModuleSpecRef {
    pub fn from<S: Into<String>, A: Into<Address>>(
        name: S,
        addr: A,
        node: ModelSTD,
    ) -> ModuleSpecRef {
        Self {
            name: name.into(),
            addr: addr.into(),
            model: node,
            enable: None,
            local: None,
            setting: None,
        }
    }
    pub fn with_enable(mut self, effective: bool) -> Self {
        self.enable = Some(effective);
        self
    }

    pub fn with_setting(mut self, setting: LocalizeVarPath) -> Self {
        self.setting = Some(setting);
        self
    }

    pub fn with_local(mut self, local: PathBuf) -> Self {
        self.local = Some(local);
        self
    }

    pub fn is_enable(&self) -> bool {
        self.enable.unwrap_or(true)
    }
    pub fn spec_path(&self, root: &Path) -> PathBuf {
        root.join("mods").join(self.name.as_str())
    }
    pub fn set_local(&mut self, local: PathBuf) {
        self.local = Some(local);
    }
    pub fn get_target_spec(&self) -> MainResult<Option<ModModelSpec>> {
        if self.is_enable()
            && let Some(local) = &self.local
        {
            let target_root = local.join(self.name());
            let target_path = target_root.join(self.model().to_string());
            if target_path.exists() {
                let spec = ModModelSpec::load_from(&target_path)
                    .with(&target_root)
                    .owe(MainReason::from(ModReason::Load))?;
                return Ok(Some(spec));
            }
        }
        Ok(None)
    }
}
#[async_trait]
impl RefUpdateable<UpdateUnit> for ModuleSpecRef {
    //#[requires(self.local.is_some())]
    async fn update_local(
        &self,
        accessor: Accessor,
        _sys_root: &Path,
        options: &DownloadOptions,
    ) -> MainResult<UpdateUnit> {
        //trace!(target: "spec/mod/",  "{:?}",self );
        if let Some(local) = &self.local {
            let mut flag = auto_exit_log!(
                info!(target: "/mod/ref",  "update mod ref {} success!", self.name ),
                error!(target: "/mod/ref", "update mod ref {} fail!", self.name )
            );
            std::fs::create_dir_all(local).owe_res().with(local)?;
            let target_root = local.join(self.name());
            let target_path = target_root.join(self.model().to_string());
            if !target_path.exists() || options.clean_cache() {
                let tmp_name = "__mod";
                let prj_path = accessor
                    .download_rename(self.addr(), local, tmp_name, options)
                    .await
                    .owe(MainReason::from(ModReason::Update))?;
                let mod_path = prj_path.position().join(MOD_DIR);
                let tmp_path = local.join(tmp_name);
                make_clean_path(&target_root).owe_res()?;

                std::fs::rename(&mod_path, &target_root)
                    .owe_logic()
                    .with(("from", &mod_path))
                    .with(("to", &target_root))?;
                if tmp_path.exists() {
                    std::fs::remove_dir_all(tmp_path).owe_sys()?;
                }
            }

            debug!(target: "mod/ref",  "update target success!" );
            //let target_path = target_root.join(self.node().to_string());
            let spec = ModModelSpec::load_from(&target_path)
                .with(&target_root)
                .owe(MainReason::from(ModReason::Load))?;
            let unit = spec
                .update_local(accessor, &target_path, options)
                .await
                .owe(MainReason::from(ModReason::Update))?;
            ModModelSpec::clean_other(&target_root, self.model())?;
            flag.mark_suc();
            return Ok(unit);
        } else {
            Err(MainError::from_logic(
                "no local value in ModuleSpecRef ".into(),
            ))
        }
    }
}

impl ModuleSpecRef {
    pub fn spec_value_path(&self, parent: ValuePath) -> ValuePath {
        let value = PathBuf::from(self.name());
        parent.join(value)
    }
}

#[async_trait]
impl Localizable for ModuleSpecRef {
    async fn localize(
        &self,
        val_path: Option<ValuePath>,
        options: LocalizeOptions,
    ) -> MainResult<()> {
        if self.enable.is_none_or(|x| x) {
            if let Some(local) = &self.local {
                let mut flag = auto_exit_log!(
                    info!(target: "spec/mod/", "localize mod {} success!", self.name ),
                    error!(target: "spec/mod/", "localize mod {} fail!", self.name )
                );
                let mod_path = local.join(self.name.as_str());
                let target_path = mod_path.join(self.model().to_string());
                let spec =
                    ModModelSpec::load_from(&target_path).owe(MainReason::from(ModReason::Load))?;
                //if let Some(dst) = &dst_path {
                //    spec.save_main(dst.local(), None)?;
                //}
                let value = PathBuf::from(self.name());
                //let local = PathBuf::from(self.name()).join("local");
                let cur_dst_path = val_path.map(|x| x.join(value));
                spec.localize(cur_dst_path.clone(), options.clone()).await?;
                flag.mark_suc();
                if let Some(setting) = &self.setting {
                    let used_value_file = ValuePath::new(spec.used_value_path()?);
                    let exe_setting =
                        LocalizeExecPath::from(setting.clone().env_eval(options.evaled_value()));
                    exe_setting.localize(Some(used_value_file), options).await?;
                }
            }
            Ok(())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::module::{ModelSTD, refs::ModuleSpecRef};

    #[test]
    fn test_module_spec_ref_builder() {
        let model_std = ModelSTD::x86_ubt22_k8s();
        let module_ref = ModuleSpecRef::from(
            "test-module",
            "https://github.com/example/test-module.git",
            model_std.clone(),
        )
        .with_enable(true)
        .with_local(PathBuf::from("/tmp/test"));

        assert_eq!(module_ref.name(), "test-module");
        assert!(!module_ref.addr().to_string().is_empty());
        assert_eq!(module_ref.model(), &model_std);
        assert!(module_ref.is_enable());
        assert!(module_ref.local().is_some());
    }

    #[test]
    fn test_module_spec_ref_enable_flag() {
        let model_std = ModelSTD::x86_ubt22_k8s();

        let enabled_ref =
            ModuleSpecRef::from("test", "https://example.com", model_std.clone()).with_enable(true);
        let disabled_ref = ModuleSpecRef::from("test", "https://example.com", model_std.clone())
            .with_enable(false);
        let default_ref = ModuleSpecRef::from("test", "https://example.com", model_std);

        assert!(enabled_ref.is_enable());
        assert!(!disabled_ref.is_enable());
        assert!(default_ref.is_enable()); // Default should be true
    }

    #[tokio::test]
    async fn test_module_spec_ref_spec_path() {
        let model_std = ModelSTD::x86_ubt22_k8s();
        let module_ref = ModuleSpecRef::from("test-module", "https://example.com", model_std);

        let root = PathBuf::from("/project/root");
        let spec_path = module_ref.spec_path(&root);

        assert_eq!(spec_path, root.join("mods").join("test-module"));
    }
}
