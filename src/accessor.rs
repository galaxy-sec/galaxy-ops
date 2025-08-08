use log::error;
use orion_common::serde::Yamlable;
use orion_variate::{
    addr::{
        accessor::{UniversalAccessor, UniversalConfig},
        redirect::serv::RedirectService,
    },
    vars::{EnvDict, EnvEvalable},
};
use std::{env::home_dir, sync::Arc};

use crate::const_vars::REDIRECT_FILE;

pub fn build_accessor(dict: &EnvDict) -> UniversalAccessor {
    if let Some(path) = home_dir().map(|x| x.join(REDIRECT_FILE)) {
        if path.exists() {
            match RedirectService::from_yml(&path) {
                Ok(redirect) => {
                    let redirect = redirect.env_eval(dict);
                    return UniversalAccessor::new(
                        UniversalConfig::default().with_redirect(redirect),
                    );
                }
                Err(e) => {
                    error!("load redirect conf failed!\npath:{} \n{e}", path.display());
                }
            }
        }
    }
    UniversalAccessor::new(UniversalConfig::default())
}
pub fn accessor_for_test() -> Arc<UniversalAccessor> {
    Arc::new(build_accessor(&EnvDict::default()))
}

pub fn accessor_for_default() -> Arc<UniversalAccessor> {
    Arc::new(build_accessor(&EnvDict::default()))
}
