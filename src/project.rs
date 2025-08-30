use std::path::{Path, PathBuf};

use orion_conf::{Configable, Yamlable};
use orion_error::{ContextRecord, ErrorOwe, OperationContext};
use orion_infra::path::ensure_path;
use orion_variate::vars::{EnvDict, EnvEvalable, OriginDict, ValueDict, VarCollection};

use crate::{
    const_vars::{MOD_VALUE_FILE, SYS_VALUE_FILE, SYS_VARS_YML, VALUE_DIR},
    error::MainResult,
    types::LocalizeOptions,
};

pub fn load_mod_opr_value(root: &Path, model: &str) -> MainResult<OriginDict> {
    let value_root = ensure_path(root.join(VALUE_DIR)).owe_logic()?;
    let sys_v_file = value_root.join(SYS_VALUE_FILE);
    if !sys_v_file.exists() {
        let mut ctx = OperationContext::want("build sys-value.yml").with_auto_log();
        let vars_file = root.join("mod").join(model).join("vars.yml");
        let vars_vec = VarCollection::from_conf(&vars_file).owe_res()?;
        let sys_value = vars_vec.system_vars();
        ctx.record("sys-value", &sys_v_file);
        sys_value.save_conf(&sys_v_file).owe_res()?;
        ctx.mark_suc();
    }
    let mut sys_dict = OriginDict::from(ValueDict::from_yml(&sys_v_file).owe_logic()?);
    sys_dict.set_source("sys-setting");

    let mod_v_file = value_root.join(model).join(MOD_VALUE_FILE);
    if !mod_v_file.exists() {
        ensure_path(&value_root.join(model)).owe_res()?;
        let vars_file = root.join("mod").join(model).join("vars.yml");
        let vars_vec = VarCollection::from_conf(&vars_file).owe_res()?;
        let sys_value = vars_vec.module_vars();
        sys_value.save_conf(&mod_v_file).owe_res()?;
    }
    let mut mod_dict = OriginDict::from(ValueDict::from_yml(&mod_v_file).owe_logic()?);
    mod_dict.set_source("mod-setting");
    sys_dict.merge(&mod_dict);
    Ok(sys_dict)
}

pub fn load_sys_opr_value(prj_root: &Path) -> MainResult<OriginDict> {
    let value_root = ensure_path(prj_root.join(VALUE_DIR)).owe_logic()?;
    let sys_v_file = value_root.join(SYS_VALUE_FILE);
    if !sys_v_file.exists() {
        let mut ctx = OperationContext::want("build sys-value.yml").with_auto_log();
        let vars_file = prj_root.join("sys").join(SYS_VARS_YML);
        let vars_vec = VarCollection::from_conf(&vars_file).owe_res()?;
        let sys_value = vars_vec.system_vars();
        ctx.record("sys-value", &sys_v_file);
        sys_value.save_conf(&sys_v_file).owe_res()?;
        ctx.mark_suc();
    }
    let mut sys_dict = OriginDict::from(ValueDict::from_yml(&sys_v_file).owe_logic()?);
    sys_dict.set_source("sys-setting");
    Ok(sys_dict)
}

pub fn mix_used_value(
    options: LocalizeOptions,
    vars: &VarCollection,
    mod_value: &PathBuf,
) -> MainResult<OriginDict> {
    let mut used = OriginDict::default();
    let mut default = OriginDict::from(vars.clone());
    default.set_source("mod-default");
    used.merge(&default);
    let mut mod_dict = OriginDict::from(ValueDict::from_yml(mod_value).owe_res()?);
    mod_dict.set_source("mod-setting");
    let global = options.raw_value().clone();
    used.merge(&mod_dict);
    used.merge(&global);
    let used = used.clone().env_eval(&EnvDict::default());
    Ok(used)
}

#[cfg(test)]
mod tests {
    use crate::const_vars::USER_VALUE_FILE;

    use super::*;
    use orion_variate::vars::{Mutability, OriginValue, ValueType, VarDefinition};
    use tempfile::tempdir;

    fn test_init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_build_used_value_with_default_only() {
        test_init();
        let vars = VarCollection::define(vec![VarDefinition::from(("TEST_KEY", "default_value"))]);
        let options = LocalizeOptions::new(OriginDict::new());
        let temp_dir = tempdir().unwrap();
        let value_paths = temp_dir.path().to_path_buf();

        let result = mix_used_value(options, &vars, &value_paths).unwrap();
        assert_eq!(
            result.get("TEST_KEY"),
            Some(&OriginValue::from("default_value").with_origin("mod-default"))
        );
    }

    #[test]
    fn test_build_used_value_with_global_value() {
        test_init();
        let mut global_dict = OriginDict::new();
        global_dict.insert("TEST_KEY".to_string(), ValueType::from("global_value"));
        global_dict.insert("PRJ_SPACE".to_string(), ValueType::from("galaxy"));
        let vars = VarCollection::define(vec![
            VarDefinition::from(("TEST_KEY", "default_value"))
                .with_mutability(Mutability::Immutable),
            VarDefinition::from(("PRJ_SPACE", "${HOME}")),
            VarDefinition::from(("SVR_NAME", "gflow")),
            VarDefinition::from(("MOD_SPACE", "${PRJ_SPACE}/${SVR_NAME}")),
            VarDefinition::from(("SVR_SPACE", "/home/${SVR_NAME}")),
        ]);
        let options = LocalizeOptions::new(global_dict);
        let temp_dir = tempdir().unwrap();
        let value_paths = temp_dir.path().to_path_buf();

        let result = mix_used_value(options, &vars, &value_paths).unwrap();
        assert_eq!(
            result.get("TEST_KEY"),
            Some(
                &OriginValue::from("default_value")
                    .with_origin("mod-default")
                    .with_mutability(Mutability::Immutable),
            )
        );
        assert_eq!(
            result.get("PRJ_SPACE"),
            Some(&OriginValue::from("galaxy").with_origin("global"))
        );
        assert_eq!(
            result.get("SVR_SPACE"),
            Some(&OriginValue::from("/home/gflow").with_origin("mod-default"))
        );
        assert_eq!(
            result.get("MOD_SPACE"),
            Some(&OriginValue::from("galaxy/gflow").with_origin("mod-default"))
        );
    }

    #[test]
    fn test_build_used_value_with_user_value() {
        test_init();
        let temp_dir = tempdir().unwrap();
        let user_value_path = temp_dir.path().join(USER_VALUE_FILE);
        std::fs::write(&user_value_path, "TEST_KEY: user_value").unwrap();

        let vars = VarCollection::define(vec![VarDefinition::from(("TEST_KEY", "default_value"))]);
        let options = LocalizeOptions::new(OriginDict::new());
        let value_paths = temp_dir.path().to_path_buf();

        let result = mix_used_value(options, &vars, &value_paths).unwrap();
        assert_eq!(
            result.get("TEST_KEY"),
            Some(&OriginValue::from("user_value").with_origin("mod-cust"))
        );
    }

    #[test]
    fn test_build_used_value_merge_precedence() {
        test_init();
        let temp_dir = tempdir().unwrap();
        let cust_value_path = temp_dir.path().join(USER_VALUE_FILE);
        std::fs::write(
            &cust_value_path,
            "TEST_KEY: user_value\nUSER_ONLY: user_only",
        )
        .unwrap();

        let mut global_dict = OriginDict::new();
        global_dict.insert("TEST_KEY".to_string(), ValueType::from("global_value"));
        global_dict.insert("GLOBAL_ONLY".to_string(), ValueType::from("global_only"));

        let vars = VarCollection::define(vec![
            VarDefinition::from(("TEST_KEY", "default_value")),
            VarDefinition::from(("DEFAULT_ONLY", "default_only")),
        ]);
        let options = LocalizeOptions::new(global_dict);
        let value_paths = temp_dir.path().to_path_buf();

        let result = mix_used_value(options, &vars, &value_paths).unwrap();
        // 验证优先级: global > cust  > default
        assert_eq!(
            result.get("TEST_KEY"),
            Some(&OriginValue::from("global_value").with_origin("global"))
        );
        // 验证各层特有键都存在
        assert_eq!(
            result.get("GLOBAL_ONLY"),
            Some(&OriginValue::from("global_only").with_origin("global"))
        );
        assert_eq!(
            result.get("USER_ONLY"),
            Some(&OriginValue::from("user_only").with_origin("mod-cust"))
        );
        assert_eq!(
            result.get("DEFAULT_ONLY"),
            Some(&OriginValue::from("default_only").with_origin("mod-default"))
        );
    }

    #[test]
    fn test_empty_vars_returns_empty_dict() {
        test_init();
        let vars = VarCollection::define(vec![]);
        let options = LocalizeOptions::new(OriginDict::new());
        let temp_dir = tempdir().unwrap();
        let value_paths = temp_dir.path().to_path_buf();

        let result = mix_used_value(options, &vars, &value_paths).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_complex_value_types() {
        test_init();

        let vars = VarCollection::define(vec![
            VarDefinition::from(("STRING_VAR", ValueType::from("default_string"))),
            VarDefinition::from(("NUMBER_VAR", ValueType::from(42))),
            VarDefinition::from(("BOOL_VAR", ValueType::from(true))),
        ]);
        let options = LocalizeOptions::new(OriginDict::new());
        let temp_dir = tempdir().unwrap();
        let value_paths = temp_dir.path().to_path_buf();

        let result = mix_used_value(options, &vars, &value_paths).unwrap();

        assert_eq!(
            result.get("STRING_VAR"),
            Some(&OriginValue::from(ValueType::from("default_string")).with_origin("mod-default"))
        );
        assert_eq!(
            result.get("NUMBER_VAR"),
            Some(&OriginValue::from(ValueType::from(42)).with_origin("mod-default"))
        );
        assert_eq!(
            result.get("BOOL_VAR"),
            Some(&OriginValue::from(ValueType::from(true)).with_origin("mod-default"))
        );
    }

    #[test]
    fn test_env_variable_substitution() {
        test_init();
        unsafe {
            std::env::set_var("TEST_ENV_VAR", "substituted_value");
        }

        let vars = VarCollection::define(vec![
            VarDefinition::from(("ENV_VAR", "${TEST_ENV_VAR}")),
            VarDefinition::from(("MIXED_VAR", "prefix_${TEST_ENV_VAR}_suffix")),
        ]);
        let options = LocalizeOptions::new(OriginDict::new());
        let temp_dir = tempdir().unwrap();
        let value_paths = temp_dir.path().to_path_buf();

        let result = mix_used_value(options, &vars, &value_paths).unwrap();

        assert_eq!(
            result.get("ENV_VAR"),
            Some(&OriginValue::from("substituted_value").with_origin("mod-default"))
        );
        assert_eq!(
            result.get("MIXED_VAR"),
            Some(&OriginValue::from("prefix_substituted_value_suffix").with_origin("mod-default"))
        );

        unsafe {
            std::env::remove_var("TEST_ENV_VAR");
        }
    }

    #[test]
    fn test_use_default_value_flag() {
        test_init();
        let temp_dir = tempdir().unwrap();
        let user_value_path = temp_dir.path().join(USER_VALUE_FILE);
        std::fs::write(&user_value_path, "TEST_KEY: user_value").unwrap();

        let vars = VarCollection::define(vec![VarDefinition::from(("TEST_KEY", "default_value"))]);
        let options = LocalizeOptions::new(OriginDict::new());
        let value_paths = temp_dir.path().to_path_buf();

        let result = mix_used_value(options, &vars, &value_paths).unwrap();
        assert_eq!(
            result.get("TEST_KEY"),
            Some(&OriginValue::from("default_value").with_origin("mod-default"))
        );
    }

    #[test]
    fn test_global_value_override_precedence() {
        test_init();
        let temp_dir = tempdir().unwrap();
        let user_value_path = temp_dir.path().join(USER_VALUE_FILE);
        std::fs::write(&user_value_path, "TEST_KEY: user_value").unwrap();

        let mut global_dict = OriginDict::new();
        global_dict.insert("TEST_KEY".to_string(), ValueType::from("global_value"));

        let vars = VarCollection::define(vec![VarDefinition::from(("TEST_KEY", "default_value"))]);
        let options = LocalizeOptions::new(global_dict);
        let value_paths = &temp_dir.path().to_path_buf();

        let result = mix_used_value(options, &vars, &value_paths).unwrap();
        // 全局值应该覆盖用户值和默认值
        assert_eq!(
            result.get("TEST_KEY"),
            Some(&OriginValue::from("global_value").with_origin("global"))
        );
    }
}
