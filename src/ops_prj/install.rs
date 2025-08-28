use std::path::{Path, PathBuf};

use fs_extra::dir::{CopyOptions, move_dir};
use orion_error::{ErrorOwe, UvsConfFrom};

use crate::{
    error::{MainReason, MainResult, ToErr},
    ops_prj::{
        path::{InstallPaths, ProjectPath},
        system::OpsTargetSystem,
    },
    system::spec::SysModelSpec,
};
#[derive(Clone)]
pub struct PackageInstaller {
    project_paths: ProjectPath,
    copy_options: CopyOptions,
}

impl PackageInstaller {
    pub fn new(project_paths: ProjectPath) -> Self {
        Self {
            project_paths,
            copy_options: CopyOptions::new(),
        }
    }

    pub fn install_package(
        &self,
        sys_src: PathBuf,
        sys_spec: &SysModelSpec,
    ) -> MainResult<OpsTargetSystem> {
        let paths = self.prepare_paths(&sys_src, sys_spec)?;
        self.cleanup_existing(&paths)?;
        self.move_and_rename_system(&sys_src, &paths)?;
        Ok(OpsTargetSystem::new(
            paths.final_target_path,
            sys_spec.clone(),
        ))
    }

    fn prepare_paths(&self, sys_src: &Path, sys_spec: &SysModelSpec) -> MainResult<InstallPaths> {
        if let Some(last_name) = sys_src.iter().next_back() {
            let sys_dst_path = self.project_paths.root().join(last_name);
            let sys_new_path = self.project_paths.root().join(sys_spec.define().name());
            Ok(InstallPaths {
                source_path: sys_src.to_path_buf(),
                temp_target_path: sys_dst_path,
                final_target_path: sys_new_path,
                project_root: self.project_paths.root().to_path_buf(),
            })
        } else {
            Err(MainReason::from_conf(format!(
                "import package failed, bad path: {}",
                sys_src.display()
            ))
            .to_err())
        }
    }

    fn cleanup_existing(&self, paths: &InstallPaths) -> MainResult<()> {
        if paths.temp_target_path.exists() {
            std::fs::remove_dir_all(&paths.temp_target_path).owe_res()?;
        }
        if paths.final_target_path.exists() {
            std::fs::remove_dir_all(&paths.final_target_path).owe_res()?;
        }
        Ok(())
    }

    fn move_and_rename_system(&self, sys_src: &Path, paths: &InstallPaths) -> MainResult<()> {
        move_dir(sys_src, &paths.project_root, &self.copy_options).owe_res()?;
        std::fs::rename(&paths.temp_target_path, &paths.final_target_path).owe_res()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ops_prj::path::ProjectPath,
        system::spec::{SysDefine, SysModelSpec},
        workflow::act::SysWorkflows,
    };
    use orion_error::TestAssertWithMsg;

    use orion_variate::tools::test_init;

    use tempfile::TempDir;

    #[test]
    fn test_system_package_installer() {
        test_init();
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();
        let project_paths = ProjectPath::new(root);

        let installer = PackageInstaller::new(project_paths);

        // Test that installer was created successfully
        assert_eq!(installer.project_paths.root(), root);
    }

    #[test]
    fn test_installation_paths() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();
        let sys_src = root.join("source");
        let sys_dst_path = root.join("temp_name");
        let sys_new_path = root.join("final_name");

        let paths = InstallPaths {
            source_path: sys_src.clone(),
            temp_target_path: sys_dst_path.clone(),
            final_target_path: sys_new_path.clone(),
            project_root: root.to_path_buf(),
        };

        // Test that all paths are accessible
        assert_eq!(paths.source_path, sys_src);
        assert_eq!(paths.temp_target_path, sys_dst_path);
        assert_eq!(paths.final_target_path, sys_new_path);
        assert_eq!(paths.project_root, root);
    }

    #[test]
    fn test_prepare_installation_paths_success() {
        test_init();
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();
        let project_paths = ProjectPath::new(root);
        let installer = PackageInstaller::new(project_paths);

        let sys_src = root.join("test_system");
        let sys_spec = create_test_system_spec("test_system");

        let result = installer.prepare_paths(&sys_src, &sys_spec);

        // Should succeed
        let paths = result.assert("prepare_installation_paths should succeed");

        // Verify paths are correct
        assert_eq!(paths.source_path, sys_src);
        assert_eq!(paths.temp_target_path, root.join("test_system"));
        assert_eq!(paths.final_target_path, root.join("test_system")); // Same name in this case
        assert_eq!(paths.project_root, root);
    }

    #[test]
    fn test_prepare_installation_paths_no_last_name() {
        test_init();
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();
        let project_paths = ProjectPath::new(root);
        let installer = PackageInstaller::new(project_paths);

        let sys_src = PathBuf::new(); // Use empty path with no components
        let sys_spec = create_test_system_spec("test_system");

        let result = installer.prepare_paths(&sys_src, &sys_spec);

        // Should fail with appropriate error
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("bad path"));
    }

    #[test]
    fn test_cleanup_existing_paths() {
        test_init();
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();
        let project_paths = ProjectPath::new(root);
        let installer = PackageInstaller::new(project_paths);

        // Create test directories
        let temp_path = root.join("temp_test");
        let final_path = root.join("final_test");
        std::fs::create_dir_all(&temp_path).unwrap();
        std::fs::create_dir_all(&final_path).unwrap();

        let paths = InstallPaths {
            source_path: root.join("source"),
            temp_target_path: temp_path.clone(),
            final_target_path: final_path.clone(),
            project_root: root.to_path_buf(),
        };

        // Should succeed and remove existing directories
        installer
            .cleanup_existing(&paths)
            .assert("cleanup_existing_paths should succeed");

        // Verify directories were removed
        assert!(!temp_path.exists());
        assert!(!final_path.exists());
    }

    #[test]
    fn test_cleanup_existing_paths_nonexistent() {
        test_init();
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();
        let project_paths = ProjectPath::new(root);
        let installer = PackageInstaller::new(project_paths);

        // Create InstallationPaths with non-existent directories
        let paths = InstallPaths {
            source_path: root.join("source"),
            temp_target_path: root.join("nonexistent_temp"),
            final_target_path: root.join("nonexistent_final"),
            project_root: root.to_path_buf(),
        };

        // Should succeed even when directories don't exist
        installer
            .cleanup_existing(&paths)
            .assert("cleanup_existing_paths should succeed with nonexistent directories");
    }

    // Helper function to create a test SysModelSpec
    fn create_test_system_spec(name: &str) -> SysModelSpec {
        let define = SysDefine::new(name, crate::module::ModelSTD::from_cur_sys());
        let actions = SysWorkflows::new(vec![]);
        SysModelSpec::new(define, actions)
    }
}
