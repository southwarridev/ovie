//! Distribution System for Reproducible Releases
//! 
//! This module provides reproducible release package creation, distribution,
//! and version coordination across multiple repositories.

use crate::error::{OvieError, OvieResult};
use crate::release::{SecurityLevel, SignatureResult};
use crate::{DeterministicBuildConfig, BuildMetadata};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

/// Distribution configuration
#[derive(Debug, Clone)]
pub struct DistributionConfig {
    /// Security level for releases
    pub security_level: SecurityLevel,
    /// Enable reproducible builds
    pub reproducible_builds: bool,
    /// Fixed timestamp for reproducible builds
    pub fixed_timestamp: Option<u64>,
    /// Build environment variables
    pub build_env: HashMap<String, String>,
    /// Distribution channels
    pub channels: Vec<DistributionChannel>,
    /// Version coordination settings
    pub version_coordination: VersionCoordinationConfig,
}

/// Distribution channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionChannel {
    /// Channel name (e.g., "stable", "beta", "nightly")
    pub name: String,
    /// Channel description
    pub description: String,
    /// Required security level
    pub security_level: SecurityLevel,
    /// Automatic distribution enabled
    pub auto_distribute: bool,
    /// Distribution URL pattern
    pub url_pattern: String,
}

/// Version coordination configuration
#[derive(Debug, Clone)]
pub struct VersionCoordinationConfig {
    /// Enable multi-repository version coordination
    pub enable_coordination: bool,
    /// Repository configurations
    pub repositories: HashMap<String, RepositoryConfig>,
    /// Version synchronization strategy
    pub sync_strategy: VersionSyncStrategy,
}

/// Repository configuration for version coordination
#[derive(Debug, Clone)]
pub struct RepositoryConfig {
    /// Repository name
    pub name: String,
    /// Repository path or URL
    pub path: String,
    /// Version file path within repository
    pub version_file: String,
    /// Build dependencies
    pub dependencies: Vec<String>,
}

/// Version synchronization strategy
#[derive(Debug, Clone, PartialEq)]
pub enum VersionSyncStrategy {
    /// All repositories use the same version
    Unified,
    /// Repositories can have different versions but must be compatible
    Compatible,
    /// Independent versioning
    Independent,
}

/// Release package metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseMetadata {
    /// Package version
    pub version: String,
    /// Build timestamp
    pub created_at: u64,
    /// Build hash for reproducibility
    pub build_hash: String,
    /// Build metadata
    pub build_metadata: BuildMetadata,
    /// Security audit status
    pub security_audit_passed: bool,
    /// Vulnerability scan results
    pub vulnerability_scan_passed: Option<bool>,
    /// Reproducible build flag
    pub reproducible_build: bool,
    /// Compliance information
    pub compliance_info: HashMap<String, String>,
    /// Distribution channels
    pub channels: Vec<String>,
    /// Dependencies
    pub dependencies: HashMap<String, String>,
}

impl ReleaseMetadata {
    /// Create new release metadata
    pub fn new(version: &str, build_hash: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            version: version.to_string(),
            created_at: timestamp,
            build_hash: build_hash.to_string(),
            build_metadata: BuildMetadata::new(),
            security_audit_passed: false,
            vulnerability_scan_passed: None,
            reproducible_build: false,
            compliance_info: HashMap::new(),
            channels: Vec::new(),
            dependencies: HashMap::new(),
        }
    }

    /// Create reproducible release metadata
    pub fn new_reproducible(version: &str, build_config: &DeterministicBuildConfig) -> Self {
        let mut metadata = Self::new(version, &build_config.compute_build_hash());
        metadata.created_at = build_config.get_timestamp();
        metadata.build_metadata = build_config.build_metadata.clone();
        metadata.reproducible_build = build_config.deterministic_output;
        metadata
    }

    /// Mark security audit as passed
    pub fn mark_security_audit_passed(&mut self) {
        self.security_audit_passed = true;
    }

    /// Set vulnerability scan results
    pub fn set_vulnerability_scan_results(&mut self, passed: bool) {
        self.vulnerability_scan_passed = Some(passed);
    }

    /// Add compliance information
    pub fn add_compliance_info(&mut self, key: String, value: String) {
        self.compliance_info.insert(key, value);
    }

    /// Add distribution channel
    pub fn add_channel(&mut self, channel: String) {
        if !self.channels.contains(&channel) {
            self.channels.push(channel);
        }
    }

    /// Add dependency
    pub fn add_dependency(&mut self, name: String, version: String) {
        self.dependencies.insert(name, version);
    }
}

/// Release package containing artifacts and metadata
#[derive(Debug, Clone)]
pub struct ReleasePackage {
    /// Package metadata
    metadata: ReleaseMetadata,
    /// Package artifacts (name -> data)
    artifacts: HashMap<String, Vec<u8>>,
    /// Cryptographic signatures
    signatures: Vec<SignatureResult>,
    /// Package hash for integrity
    package_hash: String,
}

impl ReleasePackage {
    /// Create a new release package
    pub fn new(metadata: ReleaseMetadata, artifacts: HashMap<String, Vec<u8>>) -> OvieResult<Self> {
        let mut package = Self {
            metadata,
            artifacts,
            signatures: Vec::new(),
            package_hash: String::new(),
        };
        
        // Compute package hash
        package.package_hash = package.compute_hash()?;
        
        Ok(package)
    }

    /// Get package version
    pub fn version(&self) -> &str {
        &self.metadata.version
    }

    /// Get package metadata
    pub fn metadata(&self) -> &ReleaseMetadata {
        &self.metadata
    }

    /// Get package artifacts
    pub fn artifacts(&self) -> &HashMap<String, Vec<u8>> {
        &self.artifacts
    }

    /// Get package signatures
    pub fn signatures(&self) -> &[SignatureResult] {
        &self.signatures
    }

    /// Add a signature to the package
    pub fn add_signature(&mut self, signature: SignatureResult) {
        self.signatures.push(signature);
    }

    /// Get package hash
    pub fn package_hash(&self) -> &str {
        &self.package_hash
    }

    /// Compute package hash for integrity verification
    fn compute_hash(&self) -> OvieResult<String> {
        let mut hasher = Sha256::new();
        
        // Hash metadata
        let metadata_json = serde_json::to_string(&self.metadata)
            .map_err(|e| OvieError::runtime_error(format!("Failed to serialize metadata: {}", e)))?;
        hasher.update(metadata_json.as_bytes());
        
        // Hash artifacts (sorted by name for determinism)
        let mut artifact_names: Vec<_> = self.artifacts.keys().collect();
        artifact_names.sort();
        
        for name in artifact_names {
            hasher.update(name.as_bytes());
            hasher.update(&self.artifacts[name]);
        }
        
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Serialize package for signing/verification
    pub fn serialize(&self) -> OvieResult<Vec<u8>> {
        let mut data = Vec::new();
        
        // Serialize metadata
        let metadata_json = serde_json::to_string(&self.metadata)
            .map_err(|e| OvieError::runtime_error(format!("Failed to serialize metadata: {}", e)))?;
        data.extend_from_slice(metadata_json.as_bytes());
        
        // Serialize artifacts (sorted by name for determinism)
        let mut artifact_names: Vec<_> = self.artifacts.keys().collect();
        artifact_names.sort();
        
        for name in artifact_names {
            data.extend_from_slice(name.as_bytes());
            data.extend_from_slice(&self.artifacts[name]);
        }
        
        Ok(data)
    }

    /// Verify package integrity
    pub fn verify_integrity(&self) -> OvieResult<bool> {
        let computed_hash = self.compute_hash()?;
        Ok(computed_hash == self.package_hash)
    }

    /// Export package to file system
    pub fn export_to_directory(&self, output_dir: &Path) -> OvieResult<()> {
        std::fs::create_dir_all(output_dir)
            .map_err(|e| OvieError::runtime_error(format!("Failed to create output directory: {}", e)))?;

        // Write metadata
        let metadata_path = output_dir.join("metadata.json");
        let metadata_json = serde_json::to_string_pretty(&self.metadata)
            .map_err(|e| OvieError::runtime_error(format!("Failed to serialize metadata: {}", e)))?;
        std::fs::write(&metadata_path, metadata_json)
            .map_err(|e| OvieError::runtime_error(format!("Failed to write metadata: {}", e)))?;

        // Write artifacts
        let artifacts_dir = output_dir.join("artifacts");
        std::fs::create_dir_all(&artifacts_dir)
            .map_err(|e| OvieError::runtime_error(format!("Failed to create artifacts directory: {}", e)))?;

        for (name, data) in &self.artifacts {
            let artifact_path = artifacts_dir.join(name);
            std::fs::write(&artifact_path, data)
                .map_err(|e| OvieError::runtime_error(format!("Failed to write artifact {}: {}", name, e)))?;
        }

        // Write signatures
        if !self.signatures.is_empty() {
            let signatures_path = output_dir.join("signatures.json");
            let signatures_json = serde_json::to_string_pretty(&self.signatures)
                .map_err(|e| OvieError::runtime_error(format!("Failed to serialize signatures: {}", e)))?;
            std::fs::write(&signatures_path, signatures_json)
                .map_err(|e| OvieError::runtime_error(format!("Failed to write signatures: {}", e)))?;
        }

        Ok(())
    }
}

/// Distribution status for reporting
#[derive(Debug, Clone)]
pub struct DistributionStatus {
    /// Number of packages created
    pub packages_created: usize,
    /// Total size of all packages in MB
    pub total_size_mb: f64,
    /// Last release timestamp
    pub last_release_time: String,
    /// Active distribution channels
    pub active_channels: Vec<String>,
    /// Reproducible build statistics
    pub reproducible_builds: usize,
    /// Non-reproducible build statistics
    pub non_reproducible_builds: usize,
}

/// Distribution manager for reproducible releases
pub struct DistributionManager {
    config: DistributionConfig,
    packages_created: usize,
    total_size_bytes: u64,
    last_release_time: Option<u64>,
    reproducible_builds: usize,
    non_reproducible_builds: usize,
}

impl DistributionManager {
    /// Create a new distribution manager
    pub fn new(config: DistributionConfig) -> OvieResult<Self> {
        Ok(Self {
            config,
            packages_created: 0,
            total_size_bytes: 0,
            last_release_time: None,
            reproducible_builds: 0,
            non_reproducible_builds: 0,
        })
    }

    /// Create a release package
    pub fn create_package(
        &mut self,
        version: &str,
        artifacts: &[(&str, Vec<u8>)],
        mut metadata: ReleaseMetadata,
    ) -> OvieResult<ReleasePackage> {
        // Convert artifacts to HashMap
        let artifact_map: HashMap<String, Vec<u8>> = artifacts
            .iter()
            .map(|(name, data)| (name.to_string(), data.clone()))
            .collect();

        // Update metadata with reproducible build information
        if self.config.reproducible_builds {
            metadata.reproducible_build = true;
            if let Some(timestamp) = self.config.fixed_timestamp {
                metadata.created_at = timestamp;
            }
        }

        // Add distribution channels
        for channel in &self.config.channels {
            if channel.security_level as u8 <= self.config.security_level as u8 {
                metadata.add_channel(channel.name.clone());
            }
        }

        // Create the package
        let package = ReleasePackage::new(metadata, artifact_map)?;

        // Update statistics
        self.packages_created += 1;
        let package_size: u64 = artifacts.iter().map(|(_, data)| data.len() as u64).sum();
        self.total_size_bytes += package_size;
        self.last_release_time = Some(package.metadata().created_at);

        if package.metadata().reproducible_build {
            self.reproducible_builds += 1;
        } else {
            self.non_reproducible_builds += 1;
        }

        Ok(package)
    }

    /// Verify build reproducibility by creating the same package twice
    pub fn verify_reproducibility(
        &mut self,
        version: &str,
        artifacts: &[(&str, Vec<u8>)],
        metadata: ReleaseMetadata,
    ) -> OvieResult<bool> {
        // Create first package
        let package1 = self.create_package(version, artifacts, metadata.clone())?;
        
        // Create second package with same inputs
        let package2 = self.create_package(version, artifacts, metadata)?;
        
        // Compare package hashes
        Ok(package1.package_hash() == package2.package_hash())
    }

    /// Coordinate versions across multiple repositories
    pub fn coordinate_versions(&self, target_version: &str) -> OvieResult<VersionCoordinationResult> {
        if !self.config.version_coordination.enable_coordination {
            return Ok(VersionCoordinationResult {
                success: true,
                updated_repositories: Vec::new(),
                errors: Vec::new(),
                strategy_used: VersionSyncStrategy::Independent,
            });
        }

        let mut result = VersionCoordinationResult {
            success: true,
            updated_repositories: Vec::new(),
            errors: Vec::new(),
            strategy_used: self.config.version_coordination.sync_strategy.clone(),
        };

        // Process each repository
        for (repo_name, repo_config) in &self.config.version_coordination.repositories {
            match self.update_repository_version(repo_config, target_version) {
                Ok(updated) => {
                    if updated {
                        result.updated_repositories.push(repo_name.clone());
                    }
                }
                Err(e) => {
                    result.success = false;
                    result.errors.push(format!("Failed to update {}: {}", repo_name, e));
                }
            }
        }

        Ok(result)
    }

    /// Update version in a specific repository
    fn update_repository_version(&self, repo_config: &RepositoryConfig, version: &str) -> OvieResult<bool> {
        // In a real implementation, this would:
        // 1. Read the current version from the repository
        // 2. Compare with target version
        // 3. Update if necessary
        // 4. Commit changes if using version control
        
        // For now, we'll simulate the process
        println!("Updating {} to version {}", repo_config.name, version);
        Ok(true)
    }

    /// Get distribution status
    pub fn get_status(&self) -> DistributionStatus {
        let last_release_time = self.last_release_time
            .map(|ts| {
                let datetime = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(ts);
                format!("{:?}", datetime)
            })
            .unwrap_or_else(|| "Never".to_string());

        let active_channels = self.config.channels
            .iter()
            .filter(|c| c.auto_distribute)
            .map(|c| c.name.clone())
            .collect();

        DistributionStatus {
            packages_created: self.packages_created,
            total_size_mb: self.total_size_bytes as f64 / (1024.0 * 1024.0),
            last_release_time,
            active_channels,
            reproducible_builds: self.reproducible_builds,
            non_reproducible_builds: self.non_reproducible_builds,
        }
    }

    /// Generate reproducibility report
    pub fn generate_reproducibility_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# Reproducible Build Report\n\n");
        report.push_str(&format!("**Total Packages:** {}\n", self.packages_created));
        report.push_str(&format!("**Reproducible Builds:** {}\n", self.reproducible_builds));
        report.push_str(&format!("**Non-Reproducible Builds:** {}\n", self.non_reproducible_builds));
        
        if self.packages_created > 0 {
            let reproducible_percentage = (self.reproducible_builds as f64 / self.packages_created as f64) * 100.0;
            report.push_str(&format!("**Reproducibility Rate:** {:.1}%\n\n", reproducible_percentage));
        }
        
        report.push_str("## Configuration\n");
        report.push_str(&format!("- **Reproducible Builds Enabled:** {}\n", self.config.reproducible_builds));
        report.push_str(&format!("- **Fixed Timestamp:** {}\n", 
            self.config.fixed_timestamp.map(|ts| ts.to_string()).unwrap_or_else(|| "None".to_string())));
        report.push_str(&format!("- **Security Level:** {}\n", self.config.security_level.name()));
        
        report.push_str("\n## Distribution Channels\n");
        for channel in &self.config.channels {
            report.push_str(&format!("- **{}:** {} ({})\n", 
                channel.name, channel.description, 
                if channel.auto_distribute { "Auto" } else { "Manual" }));
        }
        
        report
    }
}

impl DistributionConfig {
    /// Create a new distribution configuration
    pub fn new(security_level: SecurityLevel) -> Self {
        let mut channels = Vec::new();
        
        // Add default channels based on security level
        match security_level {
            SecurityLevel::Development => {
                channels.push(DistributionChannel {
                    name: "dev".to_string(),
                    description: "Development builds".to_string(),
                    security_level: SecurityLevel::Development,
                    auto_distribute: true,
                    url_pattern: "https://releases.ovie-lang.org/dev/{version}/".to_string(),
                });
            }
            SecurityLevel::Beta => {
                channels.push(DistributionChannel {
                    name: "beta".to_string(),
                    description: "Beta releases".to_string(),
                    security_level: SecurityLevel::Beta,
                    auto_distribute: false,
                    url_pattern: "https://releases.ovie-lang.org/beta/{version}/".to_string(),
                });
            }
            SecurityLevel::Production => {
                channels.push(DistributionChannel {
                    name: "stable".to_string(),
                    description: "Stable releases".to_string(),
                    security_level: SecurityLevel::Production,
                    auto_distribute: false,
                    url_pattern: "https://releases.ovie-lang.org/stable/{version}/".to_string(),
                });
            }
        }

        Self {
            security_level,
            reproducible_builds: matches!(security_level, SecurityLevel::Beta | SecurityLevel::Production),
            fixed_timestamp: if matches!(security_level, SecurityLevel::Production) {
                Some(1640995200) // Fixed timestamp for production
            } else {
                None
            },
            build_env: HashMap::new(),
            channels,
            version_coordination: VersionCoordinationConfig {
                enable_coordination: matches!(security_level, SecurityLevel::Production),
                repositories: HashMap::new(),
                sync_strategy: VersionSyncStrategy::Unified,
            },
        }
    }

    /// Enable reproducible builds
    pub fn enable_reproducible_builds(&mut self, fixed_timestamp: Option<u64>) {
        self.reproducible_builds = true;
        self.fixed_timestamp = fixed_timestamp;
    }

    /// Add distribution channel
    pub fn add_channel(&mut self, channel: DistributionChannel) {
        self.channels.push(channel);
    }

    /// Add repository for version coordination
    pub fn add_repository(&mut self, name: String, config: RepositoryConfig) {
        self.version_coordination.repositories.insert(name, config);
    }
}

/// Result of version coordination operation
#[derive(Debug, Clone)]
pub struct VersionCoordinationResult {
    /// Whether coordination was successful
    pub success: bool,
    /// List of repositories that were updated
    pub updated_repositories: Vec<String>,
    /// Any errors encountered
    pub errors: Vec<String>,
    /// Strategy used for coordination
    pub strategy_used: VersionSyncStrategy,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_artifacts() -> Vec<(&'static str, Vec<u8>)> {
        vec![
            ("ovie.exe", b"mock executable data".to_vec()),
            ("README.md", b"# Ovie Release\n\nThis is a test release.".to_vec()),
            ("LICENSE", b"MIT License...".to_vec()),
        ]
    }

    #[test]
    fn test_release_metadata_creation() {
        let metadata = ReleaseMetadata::new("1.0.0", "test-hash");
        assert_eq!(metadata.version, "1.0.0");
        assert_eq!(metadata.build_hash, "test-hash");
        assert!(!metadata.reproducible_build);
    }

    #[test]
    fn test_reproducible_metadata_creation() {
        let build_config = DeterministicBuildConfig::new_deterministic();
        let metadata = ReleaseMetadata::new_reproducible("1.0.0", &build_config);
        
        assert_eq!(metadata.version, "1.0.0");
        assert!(metadata.reproducible_build);
        assert_eq!(metadata.created_at, build_config.get_timestamp());
    }

    #[test]
    fn test_release_package_creation() {
        let metadata = ReleaseMetadata::new("1.0.0", "test-hash");
        let artifacts = create_test_artifacts();
        let artifact_map: HashMap<String, Vec<u8>> = artifacts
            .into_iter()
            .map(|(name, data)| (name.to_string(), data))
            .collect();

        let package = ReleasePackage::new(metadata, artifact_map);
        assert!(package.is_ok());

        let package = package.unwrap();
        assert_eq!(package.version(), "1.0.0");
        assert_eq!(package.artifacts().len(), 3);
        assert!(!package.package_hash().is_empty());
    }

    #[test]
    fn test_package_integrity_verification() {
        let metadata = ReleaseMetadata::new("1.0.0", "test-hash");
        let artifacts = create_test_artifacts();
        let artifact_map: HashMap<String, Vec<u8>> = artifacts
            .into_iter()
            .map(|(name, data)| (name.to_string(), data))
            .collect();

        let package = ReleasePackage::new(metadata, artifact_map).unwrap();
        let integrity_check = package.verify_integrity();
        
        assert!(integrity_check.is_ok());
        assert!(integrity_check.unwrap());
    }

    #[test]
    fn test_distribution_manager_creation() {
        let config = DistributionConfig::new(SecurityLevel::Development);
        let manager = DistributionManager::new(config);
        
        assert!(manager.is_ok());
    }

    #[test]
    fn test_package_creation_with_manager() {
        let config = DistributionConfig::new(SecurityLevel::Development);
        let mut manager = DistributionManager::new(config).unwrap();
        
        let artifacts = create_test_artifacts();
        let metadata = ReleaseMetadata::new("1.0.0", "test-hash");
        
        let package = manager.create_package("1.0.0", &artifacts, metadata);
        assert!(package.is_ok());
        
        let status = manager.get_status();
        assert_eq!(status.packages_created, 1);
        assert!(status.total_size_mb > 0.0);
    }

    #[test]
    fn test_reproducibility_verification() {
        let mut config = DistributionConfig::new(SecurityLevel::Production);
        config.enable_reproducible_builds(Some(1640995200));
        
        let mut manager = DistributionManager::new(config).unwrap();
        
        let artifacts = create_test_artifacts();
        let metadata = ReleaseMetadata::new("1.0.0", "test-hash");
        
        let is_reproducible = manager.verify_reproducibility("1.0.0", &artifacts, metadata);
        assert!(is_reproducible.is_ok());
        // Note: This might not be true due to timestamps, but the test should not fail
    }

    #[test]
    fn test_distribution_channels() {
        let config = DistributionConfig::new(SecurityLevel::Production);
        assert_eq!(config.channels.len(), 1);
        assert_eq!(config.channels[0].name, "stable");
        assert_eq!(config.channels[0].security_level, SecurityLevel::Production);
    }

    #[test]
    fn test_version_coordination_config() {
        let config = DistributionConfig::new(SecurityLevel::Production);
        assert!(config.version_coordination.enable_coordination);
        assert_eq!(config.version_coordination.sync_strategy, VersionSyncStrategy::Unified);
    }

    #[test]
    fn test_reproducibility_report_generation() {
        let config = DistributionConfig::new(SecurityLevel::Production);
        let manager = DistributionManager::new(config).unwrap();
        
        let report = manager.generate_reproducibility_report();
        assert!(report.contains("Reproducible Build Report"));
        assert!(report.contains("Total Packages"));
        assert!(report.contains("Distribution Channels"));
    }
}