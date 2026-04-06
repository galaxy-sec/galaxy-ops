#![allow(deprecated)]

use std::path::Path;

pub use orion_conf::FilePersist as Persistable;
pub use orion_conf::LoadHook as StorageLoadEvent;
pub use orion_conf::error::{OrionConfResult as SerdeResult, SerdeReason};

// Public compatibility facade for downstream crates.
// Internal code should prefer the native orion_conf 0.5 traits directly.

#[deprecated(note = "use orion_conf::ConfigIO::load_conf/save_conf instead")]
pub trait Configable
where
    Self: serde::de::DeserializeOwned + serde::Serialize + Sized + orion_conf::ConfigIO<Self>,
{
    fn from_conf(path: &Path) -> SerdeResult<Self> {
        <Self as orion_conf::ConfigIO<Self>>::load_conf(path)
    }

    fn save_conf(&self, path: &Path) -> SerdeResult<()> {
        <Self as orion_conf::ConfigIO<Self>>::save_conf(self, path)
    }
}

impl<T> Configable for T where
    T: serde::de::DeserializeOwned + serde::Serialize + Sized + orion_conf::ConfigIO<T>
{
}

#[deprecated(note = "use orion_conf::JsonIO::load_json/save_json instead")]
pub trait JsonAble
where
    Self: serde::de::DeserializeOwned + serde::Serialize + Sized + orion_conf::JsonIO<Self>,
{
    fn from_json(path: &Path) -> SerdeResult<Self> {
        <Self as orion_conf::JsonIO<Self>>::load_json(path)
    }

    fn save_json(&self, path: &Path) -> SerdeResult<()> {
        <Self as orion_conf::JsonIO<Self>>::save_json(self, path)
    }
}

impl<T> JsonAble for T where
    T: serde::de::DeserializeOwned + serde::Serialize + Sized + orion_conf::JsonIO<T>
{
}

#[deprecated(note = "use orion_conf::YamlIO::load_yaml/save_yaml instead")]
pub trait Yamlable
where
    Self: serde::de::DeserializeOwned + serde::Serialize + Sized + orion_conf::YamlIO<Self>,
{
    fn from_yml(path: &Path) -> SerdeResult<Self> {
        <Self as orion_conf::YamlIO<Self>>::load_yaml(path)
    }

    fn save_yml(&self, path: &Path) -> SerdeResult<()> {
        <Self as orion_conf::YamlIO<Self>>::save_yaml(self, path)
    }
}

impl<T> Yamlable for T where
    T: serde::de::DeserializeOwned + serde::Serialize + Sized + orion_conf::YamlIO<T>
{
}

#[deprecated(note = "use orion_conf::TextConfigIO::load_valconf/save_valconf instead")]
pub trait ValueConfable
where
    Self: serde::de::DeserializeOwned + serde::Serialize + Sized + orion_conf::TextConfigIO<Self>,
{
    fn from_valconf(path: &Path) -> SerdeResult<Self> {
        <Self as orion_conf::TextConfigIO<Self>>::load_valconf(path)
    }

    fn save_valconf(&self, path: &Path) -> SerdeResult<()> {
        <Self as orion_conf::TextConfigIO<Self>>::save_valconf(self, path)
    }
}

impl<T> ValueConfable for T where
    T: serde::de::DeserializeOwned + serde::Serialize + Sized + orion_conf::TextConfigIO<T>
{
}

#[deprecated(note = "use orion_conf::YamlIO directly")]
pub trait YamlStorageExt: Yamlable {}

impl<T> YamlStorageExt for T where T: Yamlable {}

#[cfg(test)]
mod tests {
    use super::{Configable, Yamlable};
    use crate::system::setting::Setting;
    use orion_error::TestAssert;
    use tempfile::tempdir;

    #[allow(deprecated)]
    #[test]
    fn test_legacy_compat_traits_still_work() {
        let temp_dir = tempdir().unwrap();
        let conf_path = temp_dir.path().join("setting.yml");

        let setting = Setting::example();
        setting.save_conf(&conf_path).assert();
        let loaded = Setting::from_conf(&conf_path).assert();

        loaded.save_yml(&conf_path).assert();
    }
}
