use crate::prelude::*;

use crate::system::setting::Setting;

#[derive(Getters, Clone, Debug, Serialize, Deserialize)]
#[getset(get = "pub")]
pub struct LocalizeVarPath {
    src: String,
    dst: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    setting: Option<Setting>,
}
impl EnvEvalable<LocalizeVarPath> for LocalizeVarPath {
    fn env_eval(self, dict: &orion_variate::vars::EnvDict) -> Self {
        Self {
            src: self.src.env_eval(dict),
            dst: self.dst.env_eval(dict),
            setting: self.setting.map(|x| x.env_eval(dict)),
        }
    }
}
impl LocalizeVarPath {
    pub fn of_module(module: &str, model: &str) -> Self {
        Self {
            src: format!("${{GXL_PRJ_ROOT}}/sys/setting/{module}"),
            dst: format!("${{GXL_PRJ_ROOT}}/sys/mods/{module}/{model}/local/",),
            setting: None,
        }
    }
}
