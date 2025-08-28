use std::path::{Path, PathBuf};

use derive_more::{Deref, DerefMut};
use getset::Getters;
use orion_variate::addr::Address;
use serde_derive::{Deserialize, Serialize};

use crate::system::spec::{SysDefine, SysModelSpec};

#[derive(Getters, Clone, Debug, Serialize, Deserialize, PartialEq)]
#[getset(get = "pub")]
pub struct OpsSystem {
    sys: SysDefine,
    addr: Address,
}

impl OpsSystem {
    pub fn new(sys: SysDefine, addr: Address) -> Self {
        Self { sys, addr }
    }
}

#[derive(Getters, Clone, Debug, Serialize, Deserialize, Default, Deref, DerefMut)]
pub struct OpsTarget {
    sys_models: Vec<OpsSystem>,
}

#[derive(Debug, Clone)]
pub struct OpsTargetSystem {
    pub installation_path: PathBuf,
    pub system_spec: SysModelSpec,
}

impl OpsTargetSystem {
    pub fn new(installation_path: PathBuf, system_spec: SysModelSpec) -> Self {
        Self {
            installation_path,
            system_spec,
        }
    }

    pub fn path(&self) -> &Path {
        &self.installation_path
    }

    pub fn spec(&self) -> &SysModelSpec {
        &self.system_spec
    }

    pub fn system_name(&self) -> &str {
        self.system_spec.define().name()
    }
}

#[cfg(test)]
mod tests {
    use crate::{const_vars::SYS_VALUE_FILE, ops_prj::project::OpsProject};
    use orion_variate::tools::test_init;
    use tempfile::TempDir;

    #[test]
    fn test_process_system_vars_symlink_already_exists() {
        test_init();
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create test paths
        let vars_path = root.join("sys/vars.yml");
        let value_path = root.join("values/test");
        let _value_file = root.join("values/test/").join(SYS_VALUE_FILE);
        let value_link = root.join("test/values");

        // Create necessary directories
        std::fs::create_dir_all(vars_path.parent().unwrap()).unwrap();
        std::fs::create_dir_all(&value_path).unwrap();
        std::fs::create_dir_all(value_link.parent().unwrap()).unwrap();

        // Create a sample vars.yml file
        let vars_content = r#"
system:
  - name: "test_var"
    value: "default_value"
    mutable: true
    desp: "A test variable"
  - name: "immutable_var"
    value: "immutable_value"
    mutable: false
    desp: "An immutable variable"
"#;
        std::fs::write(&vars_path, vars_content).unwrap();

        // Create existing value file
        let value_file = root.join("values/test").join(SYS_VALUE_FILE);
        std::fs::write(&value_file, "test_key: test_value").unwrap();

        // Pre-create the symlink
        std::os::unix::fs::symlink(&value_path, &value_link).unwrap();

        // Test function should not fail when symlink already exists
        OpsProject::process_system_vars(&vars_path, &value_path, &value_link, "test_system", false)
            .unwrap();

        // Verify symlink still exists
        assert!(value_link.exists());
        // Verify symlink points to correct target
        let link_target = std::fs::read_link(&value_link).unwrap();
        assert!(link_target.exists());
    }
}
