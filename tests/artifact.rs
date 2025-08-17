use std::fs;
use std::path::Path;

use galaxy_ops::artifact::Artifact;
use serde_yaml;

#[test]
fn test_parse_artifact_yml() {
    let test_data = r#"- name: postgresql
  version: 0.1.0
  origin_addr:
    url: https://mirrors.aliyun.com/postgresql/latest/postgresql-17.4.tar.gz
  cache_enable: false
  local: postgresql-17.4.tar.gz"#;

    let result: Result<Vec<Artifact>, serde_yaml::Error> = serde_yaml::from_str(test_data);
    assert!(result.is_ok(), "Failed to parse artifact.yml");

    let artifacts = result.unwrap();
    assert_eq!(artifacts.len(), 1);

    let artifact = &artifacts[0];
    assert_eq!(artifact.name(), "postgresql");
    assert_eq!(artifact.version(), "0.1.0");
    assert_eq!(artifact.local(), "postgresql-17.4.tar.gz");
    assert!(!artifact.cache_enable());
}

#[test]
fn test_parse_artifact_package_with_cache() {
    let test_data = r#"- name: test-artifact
  version: 1.0.0
  origin_addr:
    url: https://example.com/repo.git
  cache_addr:
    url: https://example.com/cache.zip
  cache_enable: true
  local: test-artifact-local"#;

    let result: Result<Vec<Artifact>, serde_yaml::Error> = serde_yaml::from_str(test_data);
    assert!(result.is_ok(), "Failed to parse artifact.yml with cache");

    let artifacts = result.unwrap();
    assert_eq!(artifacts.len(), 1);

    let artifact = &artifacts[0];
    assert_eq!(artifact.name(), "test-artifact");
    assert_eq!(artifact.version(), "1.0.0");
    assert_eq!(artifact.local(), "test-artifact-local");
    assert!(artifact.cache_enable());
}

#[test]
fn test_parse_actual_artifact_files() {
    let test_cases = vec![
        "example/targets/arm-mac14-host/spec/artifact.yml",
        "example/targets/x86-ubt22-k8s/spec/artifact.yml",
        "example/modules/postgresql/mod/arm-mac14-host/spec/artifact.yml",
    ];

    for file_path in test_cases {
        let path = Path::new(file_path);
        if path.exists() {
            let content = fs::read_to_string(path).expect(&format!("Failed to read {}", file_path));

            let result: Result<Vec<Artifact>, serde_yaml::Error> = serde_yaml::from_str(&content);
            assert!(
                result.is_ok(),
                "Failed to parse {} with error: {:?}",
                file_path,
                result.err()
            );

            let artifacts = result.unwrap();
            assert!(
                !artifacts.is_empty(),
                "Parsed empty artifact list from {}",
                file_path
            );

            for artifact in &artifacts {
                assert!(!artifact.name().is_empty(), "Artifact name cannot be empty");
                assert!(
                    !artifact.version().is_empty(),
                    "Artifact version cannot be empty"
                );
                assert!(
                    !artifact.local().is_empty(),
                    "Artifact local name cannot be empty"
                );
            }
        } else {
            panic!("Test file does not exist: {}", file_path);
        }
    }
}

#[test]
fn test_artifact_creation_consistency() {
    // This test ensures that the parsed artifacts can be created
    // with the same parameters as the original implementation
    let test_data = r#"- name: postgresql
  version: 0.1.0
  origin_addr:
    url: https://mirrors.aliyun.com/postgresql/latest/postgresql-17.4.tar.gz
  cache_enable: false
  local: postgresql-17.4.tar.gz"#;

    let artifacts: Vec<Artifact> =
        serde_yaml::from_str(test_data).expect("Failed to parse test data");

    let artifact = &artifacts[0];

    // Clone the address instead of trying to convert &Address to Address
    let new_artifact = Artifact::new(
        artifact.name(),
        artifact.version(),
        artifact.origin_addr().clone(),
        artifact.local(),
    );

    // Verify all properties match
    assert_eq!(new_artifact.name(), artifact.name());
    assert_eq!(new_artifact.version(), artifact.version());
    assert_eq!(new_artifact.origin_addr(), artifact.origin_addr());
    assert_eq!(new_artifact.local(), artifact.local());
    assert_eq!(new_artifact.cache_enable(), artifact.cache_enable());
    assert_eq!(new_artifact.cache_addr(), artifact.cache_addr());
}
