use std::path::{Path, PathBuf};

use galaxy_ops::{
    accessor::accessor_for_test,
    const_vars::{SYS_MODEL_PRJ_ROOT, WORKINS_PRJ_ROOT},
    error::MainResult,
    module::depend::{Dependency, DependencySet},
    ops_prj::project::OpsProject,
    system::{operator::SysOperator, spec::SysModelSpec},
    types::{InsUpdateable, LocalizeOptions, RefUpdateable},
};
use orion_error::{ErrorOwe, TestAssertWithMsg};
use orion_infra::path::make_clean_path;
use orion_variate::{
    addr::{Address, HttpResource, types::PathTemplate},
    archive::compress,
    tools::test_init,
    update::DownloadOptions,
};
#[tokio::test]
async fn test_full_flow() -> MainResult<()> {
    test_init();
    let sys_proj = make_sys_opr_example().await?;
    let out_path = PathBuf::from(SYS_MODEL_PRJ_ROOT).join("example_sys_x.tar.gz");
    compress(sys_proj.root_local(), &out_path).owe_sys()?;
    let mut ops_proj = make_workins_example().await?;
    let accessor = accessor_for_test();
    ops_proj
        .import_sys(
            accessor.clone(),
            out_path.display().to_string().as_str(),
            &DownloadOptions::for_test(),
        )
        .await?;
    ops_proj.ia_setting(false)?;
    let sys_path = ops_proj.root_local().join("example_sys_x");
    let sys_proj = SysOperator::load(&sys_path)?;
    sys_proj
        .update_local(accessor, &sys_path, &DownloadOptions::default())
        .await?;
    //sys_proj.localize(LocalizeOptions::for_test()).await?;
    Ok(())
    //sys_proj.
}
async fn make_workins_example() -> MainResult<OpsProject> {
    test_init();
    let prj_name = "obs_prj_x";
    let prj_path = PathBuf::from(WORKINS_PRJ_ROOT).join(prj_name);
    make_clean_path(&prj_path).owe_logic()?;
    let project = OpsProject::for_test(prj_name).assert("make workins");
    project.save().assert("save workins_prj");
    let project = OpsProject::load(&prj_path).assert("workins-prj");
    let accessor = accessor_for_test();
    let project = project
        .update_local(accessor, &prj_path, &DownloadOptions::default())
        .await
        .assert("spec.update_local");
    Ok(project)
}

async fn make_sys_opr_example() -> MainResult<SysOperator> {
    let name = "example_sys_x";
    let prj_path = PathBuf::from(SYS_MODEL_PRJ_ROOT).join(name);
    make_clean_path(&prj_path).owe_logic()?;
    let sys_opr = make_sys_operator(&prj_path, name).assert("make cust");
    if prj_path.exists() {
        std::fs::remove_dir_all(&prj_path).assert("ok");
    }
    std::fs::create_dir_all(&prj_path).assert("yes");
    sys_opr.save().assert("save dss_prj");
    let loaded_sys_opr = SysOperator::load(&prj_path).assert("dss-project");
    let accessor = accessor_for_test();
    loaded_sys_opr
        .update_local(accessor, &prj_path, &DownloadOptions::default())
        .await
        .assert("spec.update_local");
    std::fs::remove_dir_all(loaded_sys_opr.paths().value_dir()).owe_res()?;
    /*
    loaded_sys_opr
        .localize(LocalizeOptions::for_test())
        .await
        .assert("spec.localize");
        */
    Ok(loaded_sys_opr)
}

fn make_sys_operator(prj_path: &Path, name: &str) -> MainResult<SysOperator> {
    let mod_spec = SysModelSpec::for_example(name)?;
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
