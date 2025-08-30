use crate::const_vars::VALUE_DIR;
use crate::error::SysReason;
use crate::module::ModelSTD;
use crate::predule::*;

use crate::system::path::SysValuePaths;
use crate::system::spec::SysDefine;
use crate::{
    const_vars::SYS_MODEL_SPC_ROOT, error::MainResult, module::depend::DependencySet,
    types::SystemLocalizable, workflow::prj::GxlProject,
};

use super::conf::SysConf;
use super::path::SysOperatorPath;
use super::{
    init::{SYS_PRJ_ADM, SYS_PRJ_WORK, sys_init_gitignore},
    spec::SysModelSpec,
};
use crate::types::{Accessor, LocalizeOptions, RefUpdateable, ValuePath};
use async_trait::async_trait;
use orion_conf::{Configable, Persistable, Yamlable};
use orion_infra::path::{ensure_path, make_clean_path};
use orion_variate::update::DownloadOptions;
use orion_variate::vars::{VarCollection, VarToValue, find_project_define_base};

#[derive(Getters, Clone, Debug)]
#[getset(get = "pub")]
pub struct SysOperator {
    conf: SysConf,
    sys_spec: SysModelSpec,
    project: GxlProject,
    paths: SysOperatorPath,
}

impl SysOperator {
    pub fn new(spec: SysModelSpec, local_res: DependencySet, root_local: PathBuf) -> Self {
        let conf = SysConf::new(local_res);
        //let mut val_dict = ValueDict::default();
        //val_dict.insert("TEST_WORK_ROOT", ValueType::from("/home/galaxy"));
        Self {
            conf,
            sys_spec: spec,
            project: GxlProject::from((SYS_PRJ_WORK, SYS_PRJ_ADM, "define")),
            paths: SysOperatorPath::new(root_local),
            //val_dict,
        }
    }
    pub fn load(root_local: &Path) -> MainResult<Self> {
        let mut ctx = OperationContext::want("load sys-operator")
            .with_auto_log()
            .with_mod_path("sys/prj");

        let paths = SysOperatorPath::new(root_local);

        // 执行配置文件迁移
        paths.migrate_conf_file().with(&ctx).want("migrate conf")?;

        ctx.record("sys-conf", &paths.conf_file_v2());
        let conf = SysConf::from_conf(&paths.conf_file_v2())
            .owe_res()
            .with(&ctx)?;
        let sys_path = paths.sys_dir();
        ctx.record("sys_path", &sys_path);
        let sys_spec = SysModelSpec::load_from(&sys_path).with(&ctx)?;

        let project = GxlProject::load_from(paths.root())
            .owe(SysReason::Load.into())
            .with(&ctx)?;
        ensure_path(paths.value_dir()).owe_logic().with(&ctx)?;
        ctx.mark_suc();
        Ok(Self {
            conf,
            sys_spec,
            project,
            paths,
        })
    }
    pub fn save(&self) -> MainResult<()> {
        let mut ctx = OperationContext::want("save sys-prj")
            .with_auto_log()
            .with_mod_path("sys/prj");
        ctx.record("root", self.paths.root());
        let conf_file_v2 = self.paths.conf_file_v2();
        self.conf.save_conf(&conf_file_v2).owe_res().with(&ctx)?;
        self.sys_spec.save_local(self.paths.root(), "sys")?;
        self.project
            .save_to(self.paths.root(), None)
            .owe(SysReason::Save.into())
            .with(&ctx)?;

        // 保存 sys_local 配置

        ensure_path(self.paths.value_dir()).owe_logic().with(&ctx)?;
        sys_init_gitignore(self.paths.root()).with(&ctx)?;
        ctx.mark_suc();
        Ok(())
    }
}

#[async_trait]
impl RefUpdateable<()> for SysOperator {
    async fn update_local(
        &self,
        accessor: Accessor,
        path: &Path,
        options: &DownloadOptions,
    ) -> MainResult<()> {
        self.conf
            .update_local(accessor.clone(), path, options)
            .await?;
        self.sys_spec().update_local(accessor, path, options).await
    }
}

impl SysOperator {
    pub async fn localize(
        &self,
        val_path: SysValuePaths,
        options: LocalizeOptions,
    ) -> MainResult<()> {
        //let value_path = self.value_path().ensure_exist().owe_res()?;

        self.conf.sys_localize((), options.clone()).await?;
        self.sys_spec()
            .sys_localize(val_path, options.clone())
            .await?;
        Ok(())
    }
    pub fn value_path(&self) -> ValuePath {
        self.paths.to_value_path()
    }
}

impl SysOperator {
    /// 获取项目根路径，保持与原有 API 的兼容性
    pub fn root_local(&self) -> &Path {
        self.paths.root()
    }

    pub fn make_new(prj_path: &Path, name: &str, model: ModelSTD) -> MainResult<Self> {
        let mod_spec = SysModelSpec::make_new(SysDefine::new(name, model))?;
        let res = DependencySet::default();
        Ok(SysOperator::new(mod_spec, res, prj_path.to_path_buf()))
    }
    pub fn make_test_prj(name: &str) -> MainResult<Self> {
        let prj_path = PathBuf::from(SYS_MODEL_SPC_ROOT).join(name);
        make_clean_path(&prj_path).owe_logic()?;
        let proj = SysOperator::make_new(&prj_path, name, ModelSTD::from_cur_sys())?;
        proj.save()?;
        Ok(proj)
    }
    pub fn init_setting_value(&self) -> MainResult<SysValuePaths> {
        let value_root = SysValuePaths::from(PathBuf::from(self.root_local()))
            .ensure_join(VALUE_DIR)
            .owe_res()?;
        //let mut all_vars = VarCollection::default();
        for x in self.sys_spec().mod_list().iter() {
            if let Some(mmo) = x.get_target_spec()? {
                let mm_path = value_root.clone().ensure_join(x.name()).owe_res()?;
                //all_vars = all_vars.merge(mmo.vars().clone());
                if !mm_path.mod_value_file().exists() {
                    let mod_vars = mmo.vars().module_vars().to_val();
                    mod_vars.save_yml(&mm_path.mod_value_file()).owe_res()?;
                }

                //mm.vars()
            }
        }
        if !value_root.sys_value_file().exists() {
            let sys_vars = VarCollection::from_yml(&self.paths.sys_vars_file())
                .owe_res()?
                .system_vars()
                .to_val();
            //all_vars.system_vars().to_val();
            sys_vars.save_yml(&value_root.sys_value_file()).owe_res()?;
        }
        Ok(value_root)
    }
}

pub fn setup_prj_root_env_vars(base: PathBuf) -> MainResult<()> {
    let prj_root = find_project_define_base(base).unwrap_or(PathBuf::from("UNDEFIN"));
    unsafe { std::env::set_var("GXL_PRJ_ROOT", format!("{}", prj_root.display())) };
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use std::path::{Path, PathBuf};

    use orion_conf::Yamlable;
    use orion_error::{ErrorOwe, TestAssertWithMsg};
    use orion_infra::path::make_clean_path;
    use orion_variate::{
        addr::{Address, HttpResource, types::PathTemplate},
        tools::test_init,
        update::DownloadOptions,
        vars::{OriginDict, ValueDict},
    };

    use crate::{
        accessor::accessor_for_test,
        const_vars::SYS_OPERATORS_ROOT,
        error::MainResult,
        module::{
            ModelSTD,
            depend::{Dependency, DependencySet},
        },
        system::{
            operator::{SysOperator, setup_prj_root_env_vars},
            spec::SysModelSpec,
        },
        types::{LocalizeOptions, RefUpdateable},
    };
    #[tokio::test]
    async fn test_mod_prj_new() -> MainResult<()> {
        test_init();
        let prj_path = PathBuf::from(SYS_OPERATORS_ROOT).join("sys_new");
        make_clean_path(&prj_path).owe_logic()?;
        let proj = SysOperator::make_new(&prj_path, "sys_new", ModelSTD::from_cur_sys())?;
        proj.save()?;
        Ok(())
    }

    #[tokio::test]
    async fn test_sys_prj_example() -> MainResult<()> {
        test_init();

        let prj_path = PathBuf::from(SYS_OPERATORS_ROOT).join("example_sys2");
        make_clean_path(&prj_path).owe_logic()?;
        let project = make_sys_prj_testins(&prj_path).assert("make cust");
        project.save().assert("save dss_prj");
        let project = SysOperator::load(&prj_path).assert("dss-project");
        let accessor = accessor_for_test();
        project
            .update_local(accessor, &prj_path, &DownloadOptions::default())
            .await
            .assert("spec.update_local");
        let value_path = project.init_setting_value()?;
        let mut dict =
            OriginDict::from(ValueDict::from_yml(&value_path.sys_value_file()).owe_res()?);
        dict.set_source("sys-setting");
        setup_prj_root_env_vars(prj_path.clone()).owe_sys()?;
        project
            .localize(value_path, LocalizeOptions::new(dict))
            .await
            .assert("spec.localize");
        Ok(())
    }

    fn make_sys_prj_testins(prj_path: &Path) -> MainResult<SysOperator> {
        let mod_spec = SysModelSpec::for_example("exmaple_sys2")?;
        let mut res = DependencySet::default();
        res.push(
            Dependency::new(
                Address::from(HttpResource::from(
                    "https://e.coding.net/dy-sec/galaxy-open/bitnami-common.git",
                )),
                PathTemplate::from(prj_path.join("test_res")),
            )
            .with_rename("bit-common"),
        );
        Ok(SysOperator::new(mod_spec, res, prj_path.to_path_buf()))
    }
}
