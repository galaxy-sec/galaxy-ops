use std::ops::{Deref, DerefMut};

use derive_getters::Getters;
use derive_more::From;
use serde_derive::{Deserialize, Serialize};

use super::Artifact;

#[derive(Getters, Clone, Debug, Deserialize, Serialize, From, Default)]
#[serde(transparent)]
pub struct ArtifactPackage {
    items: Vec<Artifact>,
}
impl Deref for ArtifactPackage {
    type Target = Vec<Artifact>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
impl DerefMut for ArtifactPackage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artifact_package_creation() {
        let artifact1 = Artifact::new(
            "test-artifact1",
            "1.0.0",
            "https://example.com/repo1",
            "test-local1",
        );
        let artifact2 = Artifact::new(
            "test-artifact2",
            "2.0.0",
            "https://example.com/repo2",
            "test-local2",
        );

        let package = ArtifactPackage::from(vec![artifact1.clone(), artifact2.clone()]);

        assert_eq!(package.len(), 2);
        assert_eq!(package[0].name(), "test-artifact1");
        assert_eq!(package[1].name(), "test-artifact2");
        assert_eq!(package[0].version(), "1.0.0");
        assert_eq!(package[1].version(), "2.0.0");
    }

    #[test]
    fn test_artifact_package_deref() {
        let artifact = Artifact::new(
            "test-artifact",
            "1.0.0",
            "https://example.com/repo",
            "test-local",
        );
        let mut package = ArtifactPackage::from(vec![artifact.clone()]);

        // Test deref (read access)
        assert_eq!(package.len(), 1);
        assert_eq!(&package[0].name(), &artifact.name());

        // Test deref_mut (write access)
        package.push(Artifact::new(
            "new-artifact",
            "2.0.0",
            "https://example.com/new-repo",
            "new-local",
        ));
        assert_eq!(package.len(), 2);
        assert_eq!(package[1].name(), "new-artifact");
    }

    #[test]
    fn test_artifact_package_default() {
        let package = ArtifactPackage::default();

        assert_eq!(package.len(), 0);
        assert!(package.is_empty());
    }

    #[test]
    fn test_artifact_package_serialization() {
        let artifact1 = Artifact::new(
            "ser-artifact1",
            "1.0.0",
            "https://example.com/repo1",
            "ser-local1",
        );
        let artifact2 = Artifact::new(
            "ser-artifact2",
            "2.0.0",
            "https://example.com/repo2",
            "ser-local2",
        );
        let package = ArtifactPackage::from(vec![artifact1.clone(), artifact2.clone()]);

        // Test serialization
        let serialized = serde_json::to_string(&package).unwrap();
        let deserialized: ArtifactPackage = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.len(), 2);
        assert_eq!(deserialized[0].name(), "ser-artifact1");
        assert_eq!(deserialized[1].name(), "ser-artifact2");
        assert_eq!(deserialized[0].version(), "1.0.0");
        assert_eq!(deserialized[1].version(), "2.0.0");
    }
}
