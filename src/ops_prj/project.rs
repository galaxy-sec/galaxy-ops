use crate::const_vars::WORKINS_PRJ_ROOT;
use crate::error::OpsReason;
use crate::ops_prj::path::ProjectPath;
use crate::ops_prj::system::{OpsSystem, OpsTarget};
use crate::predule::*;

use crate::{error::MainResult, module::depend::DependencySet, workflow::prj::GxlProject};
const OPS_PRJ_WORK: &str = include_str!("init/_gal/work.gxl");
const OPS_PRJ_ADM: &str = include_str!("init/_gal/adm.gxl");
pub const OPS_PRJ_FILE: &str = "ops-prj.yml";
pub const PRJ_OPS_TARGET: &str = "ops-systems.yml";

use crate::types::{Accessor, InsUpdateable, ValuePath};
use async_trait::async_trait;
use getset::MutGetters;
use orion_conf::{Configable, Persistable};
use orion_infra::auto_exit_log;
use orion_infra::path::make_clean_path;
use orion_variate::update::DownloadOptions;

use super::conf::ProjectConf;
use super::init::workins_init_gitignore;

#[derive(Getters, Clone, Debug, MutGetters)]
pub struct OpsProject {
    conf: ProjectConf,
    project: GxlProject,
    paths: ProjectPath,
    #[getset(get = "pub", get_mut = "pub")]
    ops_target: OpsTarget,
}
impl OpsProject {
    pub fn new(conf: ProjectConf, root_local: PathBuf) -> Self {
        Self {
            conf,
            project: GxlProject::from((OPS_PRJ_WORK, OPS_PRJ_ADM)),
            paths: ProjectPath::new(root_local),
            ops_target: OpsTarget::default(),
        }
    }
    pub fn import_ops_sys(&mut self, ops_sys: OpsSystem) {
        if !self.ops_target.contains(&ops_sys) {
            self.ops_target.push(ops_sys);
        }
    }
    pub fn load(root_local: &Path) -> MainResult<Self> {
        let mut flag = auto_exit_log!(
            info!(
                target : "ops-prj",
                "load project from {} success!", root_local.display()
            ),
            error!(
                target : "ops-prj",
                "load project  from {} fail!", root_local.display()
            )
        );

        let paths = ProjectPath::new(root_local);
        let conf = ProjectConf::load(paths.root())?;

        let ops_target = OpsTarget::from_conf(&paths.target_file()).owe_conf()?;
        let project = GxlProject::load_from(paths.root()).owe(OpsReason::Load.into())?;
        flag.mark_suc();
        Ok(Self {
            conf,
            project,
            paths,
            ops_target,
        })
    }
    pub fn save(&self) -> MainResult<()> {
        let mut flag = auto_exit_log!(
            info!(
                target : "workprj",
                "save project to {} success!", self.paths.root().display()
            ),
            error!(
                target : "workprj",
                "save project  to {} fail!", self.paths.root().display()
            )
        );
        self.ops_target
            .save_conf(&self.paths.target_file())
            .owe_res()?;
        self.conf.save_conf(&self.paths.conf_file()).owe_res()?;
        self.project.save_to(self.paths.root(), None).owe_logic()?;

        workins_init_gitignore(self.paths.root())?;
        flag.mark_suc();
        Ok(())
    }
}

#[async_trait]
impl InsUpdateable<OpsProject> for OpsProject {
    async fn update_local(
        mut self,
        accessor: Accessor,
        path: &Path,
        options: &DownloadOptions,
    ) -> MainResult<Self> {
        self.conf = self.conf.update_local(accessor, path, options).await?;
        self.save()?;
        Ok(self)
    }
}

impl OpsProject {
    pub fn value_path(&self) -> ValuePath {
        self.paths.to_value_path()
    }

    /// 获取项目根路径，保持与原有 API 的兼容性
    pub fn root_local(&self) -> &Path {
        self.paths.root()
    }
}
impl OpsProject {
    pub fn make_new(prj_path: &Path, name: &str) -> MainResult<Self> {
        let conf = ProjectConf::new(name, DependencySet::default());
        Ok(OpsProject::new(conf, prj_path.to_path_buf()))
    }
    pub fn for_test(name: &str) -> MainResult<Self> {
        let prj_path = PathBuf::from(WORKINS_PRJ_ROOT).join(name);
        make_clean_path(&prj_path).owe_logic()?;

        let conf = ProjectConf::for_test();
        let proj = OpsProject::new(conf, prj_path);
        Ok(proj)
    }
}

#[cfg(test)]
pub mod tests {
    use std::path::PathBuf;

    use orion_error::{ErrorOwe, TestAssertWithMsg};
    use orion_infra::path::make_clean_path;
    use orion_variate::{tools::test_init, update::DownloadOptions};

    use crate::{
        accessor::accessor_for_test, const_vars::WORKINS_PRJ_ROOT, error::MainResult,
        ops_prj::project::OpsProject, types::InsUpdateable,
    };

    #[tokio::test]
    async fn test_workins_example() -> MainResult<()> {
        test_init();
        let prj_path = PathBuf::from(WORKINS_PRJ_ROOT).join("workins_sys_1");
        make_clean_path(&prj_path).owe_logic()?;
        let project = OpsProject::for_test("workins_sys_1").assert("make workins");
        project.save().assert("save workins_prj");
        let project = OpsProject::load(&prj_path).assert("workins-prj");
        let accessor = accessor_for_test();
        project
            .update_local(accessor, &prj_path, &DownloadOptions::default())
            .await
            .assert("spec.update_local");
        Ok(())
    }
}
