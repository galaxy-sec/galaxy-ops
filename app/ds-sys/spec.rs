use std::path::PathBuf;

use orion_error::ErrorConv;
use orion_syspec::addr::GitAddr;
use orion_syspec::error::SpecResult;
use orion_syspec::infra::configure_dfx_logging;
use orion_syspec::system::refs::SysModelSpecRef;
use orion_syspec::system::spec::{SysModelSpec, make_sys_spec_example, make_sys_spec_new};
use orion_syspec::types::{AsyncUpdateable, Localizable, UpdateOptions};

use crate::args::GSysCmd;

pub async fn do_sys_cmd(cmd: GSysCmd) -> SpecResult<()> {
    let current_dir = std::env::current_dir().expect("无法获取当前目录");
    match cmd {
        GSysCmd::Example => {
            let spec = make_sys_spec_example().err_conv()?;
            spec.save_to(&PathBuf::from("./")).err_conv()?;
        }
        GSysCmd::New(args) => {
            let spec = make_sys_spec_new(args.name(), "https://").err_conv()?;
            spec.save_to(&PathBuf::from("./")).err_conv()?;
        }
        GSysCmd::Load(load_args) => {
            configure_dfx_logging(&load_args);
            let target = load_args.path();
            let spec_ref =
                SysModelSpecRef::from(target, GitAddr::from(load_args.repo()).path(target));
            spec_ref
                .update_local(&current_dir, &UpdateOptions::default())
                .await
                .err_conv()?;
        }
        GSysCmd::Update(dfx) => {
            configure_dfx_logging(&dfx);
            let spec = SysModelSpec::load_from(&current_dir).err_conv()?;
            spec.update_local(&UpdateOptions::default())
                .await
                .err_conv()?;
        }
        GSysCmd::Localize(dfx) => {
            configure_dfx_logging(&dfx);
            let spec = SysModelSpec::load_from(&current_dir).err_conv()?;
            spec.localize(None).await.err_conv()?;
        }
    }
    Ok(())
}
