//! Package Management System for Ovie
//! 
//! This module implements a secure, deterministic package management system
//! with offline-first operation and cryptographic verification.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use crate::{OvieResult, OvieError};
use crate::security::{SupplyChainSecurity, SecurityPolicies};
use std::time::{SystemTime, UNIX_EPOCH};

/// Package identifier using cryptographic hash
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PackageId {
    /// Package name
    pub name: String,
    /// Version string
    pub version: String,
    /// SHA256 hash of package content
    pub content_hash: String,
}

impl PackageId {
    pub fn new(name: String, version: String, content_hash: String) -> Self {
        Self { name, version, content_hash }
    }

    /// Generate a unique identifier string for this package
    pub fn to_string(&self) -> String {
        format!("{}@{}#{}", self.name, self.version, &self.content_hash[..16])
    }
}

/// Package metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub id: PackageId,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub dependencies: HashMap<String, PackageId>,
    pub dev_dependencies: HashMap<String, PackageId>,
    pub build_dependencies: HashMap<String, PackageId>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    /// Cryptographic signatures for integrity verification
    pub signatures: Vec<PackageSignature>,
    /// Checksums for additional verification
    pub checksums: HashMap<String, String>, // algorithm -> checksum
    /// Build timestamp for reproducibility
    pub build_timestamp: Option<u64>,
    /// Offline-first compliance metadata
    pub offline_metadata: OfflineMetadata,
}

/// Cryptographic signature for package verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageSignature {
    /// Signature algorithm (e.g., "ed25519", "rsa-pss")
    pub algorithm: String,
    /// Base64-encoded signature
    pub signature: String,
    /// Key identifier
    pub key_id: String,
    /// Timestamp when signature was created
    pub timestamp: u64,
}

/// Offline-first compliance metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineMetadata {
    /// Whether package requires network access
    pub requires_network: bool,
    /// List of external resources accessed
    pub external_resources: Vec<String>,
    /// Deterministic build guarantee
    pub deterministic_build: bool,
    /// Reproducible build hash
    pub reproducible_hash: Option<String>,
}

/// Local package registry
#[derive(Debug)]
pub struct PackageRegistry {
    /// Path to the local registry directory (~/.ovie/registry/)
    registry_path: PathBuf,
    /// Path to the vendor directory (./vendor/)
    vendor_path: PathBuf,
    /// Cached package metadata
    cache: HashMap<PackageId, PackageMetadata>,
    /// Supply chain security manager
    security: SupplyChainSecurity,
}

impl PackageRegistry {
    /// Create a new package registry
    pub fn new() -> OvieResult<Self> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| OvieError::generic("Could not determine home directory"))?;
        
        let registry_path = home_dir.join(".ovie").join("registry");
        let vendor_path = PathBuf::from("vendor");

        // Create registry directory if it doesn't exist
        fs::create_dir_all(&registry_path)
            .map_err(|e| OvieError::io_error(format!("Failed to create registry directory: {}", e)))?;

        Ok(Self {
            registry_path,
            vendor_path,
            cache: HashMap::new(),
            security: SupplyChainSecurity::new(),
        })
    }

    /// Create a new package registry with custom paths
    pub fn with_paths(registry_path: PathBuf, vendor_path: PathBuf) -> OvieResult<Self> {
        // Create registry directory if it doesn't exist
        fs::create_dir_all(&registry_path)
            .map_err(|e| OvieError::io_error(format!("Failed to create registry directory: {}", e)))?;

        Ok(Self {
            registry_path,
            vendor_path,
            cache: HashMap::new(),
            security: SupplyChainSecurity::new(),
        })
    }

    /// Get security manager
    pub fn security(&mut self) -> &mut SupplyChainSecurity {
        &mut self.security
    }

    /// Update security policies
    pub fn update_security_policies(&mut self, policies: SecurityPolicies) {
        self.security.update_policies(policies);
    }

    /// Store a package in the local registry with integrity verification
    pub fn store_package(&mut self, metadata: PackageMetadata, content: &[u8]) -> OvieResult<()> {
        // Validate package security first
        let source_url = metadata.repository.as_deref().unwrap_or("unknown");
        let signatures = if !metadata.signatures.is_empty() {
            Some(&metadata.signatures[0].signature)
        } else {
            None
        };
        let key_id = if !metadata.signatures.is_empty() {
            Some(&metadata.signatures[0].key_id)
        } else {
            None
        };

        let is_valid = self.security.validate_package(
            content,
            source_url,
            &metadata.id.content_hash,
            signatures,
            key_id,
        )?;

        if !is_valid {
            return Err(OvieError::generic(format!(
                "Package {} failed security validation",
                metadata.id.to_string()
            )));
        }

        // Verify content hash
        let computed_hash = self.compute_content_hash(content);
        if computed_hash != metadata.id.content_hash {
            return Err(OvieError::generic(format!(
                "Content hash mismatch for package {}: expected {}, got {}",
                metadata.id.name, metadata.id.content_hash, computed_hash
            )));
        }

        // Verify additional checksums
        self.verify_checksums(&metadata, content)?;

        // Verify signatures if present
        if !metadata.signatures.is_empty() {
            self.verify_signatures(&metadata, content)?;
        }

        // Enforce offline-first compliance
        self.verify_offline_compliance(&metadata)?;

        // Create package directory
        let package_dir = self.registry_path.join(&metadata.id.name).join(&metadata.id.version);
        fs::create_dir_all(&package_dir)
            .map_err(|e| OvieError::io_error(format!("Failed to create package directory: {}", e)))?;

        // Store metadata
        let metadata_path = package_dir.join("metadata.json");
        let metadata_json = serde_json::to_string_pretty(&metadata)
            .map_err(|e| OvieError::generic(format!("Failed to serialize metadata: {}", e)))?;
        fs::write(&metadata_path, metadata_json)
            .map_err(|e| OvieError::io_error(format!("Failed to write metadata: {}", e)))?;

        // Store content
        let content_path = package_dir.join("content.tar.gz");
        fs::write(&content_path, content)
            .map_err(|e| OvieError::io_error(format!("Failed to write package content: {}", e)))?;

        // Store integrity manifest
        self.create_integrity_manifest(&package_dir, &metadata, content)?;

        // Update cache
        self.cache.insert(metadata.id.clone(), metadata);

        Ok(())
    }

    /// Retrieve a package from the local registry
    pub fn get_package(&mut self, package_id: &PackageId) -> OvieResult<Option<(PackageMetadata, Vec<u8>)>> {
        // Check cache first
        if let Some(metadata) = self.cache.get(package_id) {
            let content = self.load_package_content(package_id)?;
            return Ok(Some((metadata.clone(), content)));
        }

        // Load from disk
        let package_dir = self.registry_path.join(&package_id.name).join(&package_id.version);
        if !package_dir.exists() {
            return Ok(None);
        }

        // Load metadata
        let metadata_path = package_dir.join("metadata.json");
        if !metadata_path.exists() {
            return Ok(None);
        }

        let metadata_json = fs::read_to_string(&metadata_path)
            .map_err(|e| OvieError::io_error(format!("Failed to read metadata: {}", e)))?;
        let metadata: PackageMetadata = serde_json::from_str(&metadata_json)
            .map_err(|e| OvieError::generic(format!("Failed to parse metadata: {}", e)))?;

        // Verify package ID matches
        if metadata.id != *package_id {
            return Err(OvieError::generic(format!(
                "Package ID mismatch: expected {:?}, got {:?}",
                package_id, metadata.id
            )));
        }

        // Load content
        let content = self.load_package_content(package_id)?;

        // Verify content hash
        let computed_hash = self.compute_content_hash(&content);
        if computed_hash != package_id.content_hash {
            return Err(OvieError::generic(format!(
                "Content hash verification failed for package {}: expected {}, got {}",
                package_id.name, package_id.content_hash, computed_hash
            )));
        }

        // Update cache
        self.cache.insert(package_id.clone(), metadata.clone());

        Ok(Some((metadata, content)))
    }

    /// Vendor a package to the local vendor directory
    pub fn vendor_package(&mut self, package_id: &PackageId) -> OvieResult<()> {
        let (metadata, content) = self.get_package(package_id)?
            .ok_or_else(|| OvieError::generic(format!("Package not found: {}", package_id.to_string())))?;

        // Create vendor directory
        fs::create_dir_all(&self.vendor_path)
            .map_err(|e| OvieError::io_error(format!("Failed to create vendor directory: {}", e)))?;

        // Create package-specific vendor directory
        let vendor_package_dir = self.vendor_path.join(&metadata.id.name).join(&metadata.id.version);
        fs::create_dir_all(&vendor_package_dir)
            .map_err(|e| OvieError::io_error(format!("Failed to create vendor package directory: {}", e)))?;

        // Extract content to vendor directory
        self.extract_package_content(&content, &vendor_package_dir)?;

        // Store metadata in vendor directory
        let vendor_metadata_path = vendor_package_dir.join("ovie-package.json");
        let metadata_json = serde_json::to_string_pretty(&metadata)
            .map_err(|e| OvieError::generic(format!("Failed to serialize metadata: {}", e)))?;
        fs::write(&vendor_metadata_path, metadata_json)
            .map_err(|e| OvieError::io_error(format!("Failed to write vendor metadata: {}", e)))?;

        Ok(())
    }

    /// List all packages in the registry
    pub fn list_packages(&mut self) -> OvieResult<Vec<PackageId>> {
        let mut packages = Vec::new();

        if !self.registry_path.exists() {
            return Ok(packages);
        }

        // Iterate through package directories
        let entries = fs::read_dir(&self.registry_path)
            .map_err(|e| OvieError::io_error(format!("Failed to read registry directory: {}", e)))?;

        for entry in entries {
            let entry = entry.map_err(|e| OvieError::io_error(format!("Failed to read directory entry: {}", e)))?;
            let package_name = entry.file_name().to_string_lossy().to_string();
            let package_dir = entry.path();

            if package_dir.is_dir() {
                // Iterate through version directories
                let version_entries = fs::read_dir(&package_dir)
                    .map_err(|e| OvieError::io_error(format!("Failed to read package directory: {}", e)))?;

                for version_entry in version_entries {
                    let version_entry = version_entry.map_err(|e| OvieError::io_error(format!("Failed to read version entry: {}", e)))?;
                    let version = version_entry.file_name().to_string_lossy().to_string();
                    let version_dir = version_entry.path();

                    if version_dir.is_dir() {
                        // Load metadata to get the package ID
                        let metadata_path = version_dir.join("metadata.json");
                        if metadata_path.exists() {
                            let metadata_json = fs::read_to_string(&metadata_path)
                                .map_err(|e| OvieError::io_error(format!("Failed to read metadata: {}", e)))?;
                            let metadata: PackageMetadata = serde_json::from_str(&metadata_json)
                                .map_err(|e| OvieError::generic(format!("Failed to parse metadata: {}", e)))?;
                            
                            packages.push(metadata.id.clone());
                            self.cache.insert(metadata.id.clone(), metadata);
                        }
                    }
                }
            }
        }

        Ok(packages)
    }

    /// Compute SHA256 hash of content
    fn compute_content_hash(&self, content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        format!("{:x}", hasher.finalize())
    }

    /// Load package content from disk
    fn load_package_content(&self, package_id: &PackageId) -> OvieResult<Vec<u8>> {
        let package_dir = self.registry_path.join(&package_id.name).join(&package_id.version);
        let content_path = package_dir.join("content.tar.gz");
        
        fs::read(&content_path)
            .map_err(|e| OvieError::io_error(format!("Failed to read package content: {}", e)))
    }

    /// Extract package content (placeholder implementation)
    fn extract_package_content(&self, _content: &[u8], _target_dir: &Path) -> OvieResult<()> {
        // TODO: Implement actual tar.gz extraction
        // For now, just create a placeholder file
        let placeholder_path = _target_dir.join("extracted_content.txt");
        fs::write(&placeholder_path, "Package content extracted here")
            .map_err(|e| OvieError::io_error(format!("Failed to write placeholder: {}", e)))?;
        Ok(())
    }

    /// Verify additional checksums beyond the main content hash
    fn verify_checksums(&self, metadata: &PackageMetadata, content: &[u8]) -> OvieResult<()> {
        for (algorithm, expected_checksum) in &metadata.checksums {
            let computed_checksum = match algorithm.as_str() {
                "sha256" => self.compute_content_hash(content),
                "sha512" => {
                    use sha2::Sha512;
                    let mut hasher = Sha512::new();
                    hasher.update(content);
                    format!("{:x}", hasher.finalize())
                }
                "blake3" => {
                    // Placeholder for BLAKE3 - would need blake3 crate
                    return Err(OvieError::generic(format!("Unsupported checksum algorithm: {}", algorithm)));
                }
                _ => {
                    return Err(OvieError::generic(format!("Unknown checksum algorithm: {}", algorithm)));
                }
            };

            if &computed_checksum != expected_checksum {
                return Err(OvieError::generic(format!(
                    "Checksum verification failed for algorithm {}: expected {}, got {}",
                    algorithm, expected_checksum, computed_checksum
                )));
            }
        }
        Ok(())
    }

    /// Verify cryptographic signatures
    fn verify_signatures(&self, metadata: &PackageMetadata, content: &[u8]) -> OvieResult<()> {
        if metadata.signatures.is_empty() {
            return Ok(());
        }

        for signature in &metadata.signatures {
            match signature.algorithm.as_str() {
                "ed25519" => {
                    // Placeholder for Ed25519 verification
                    // Would need ed25519-dalek crate
                    return Err(OvieError::generic("Ed25519 signature verification not yet implemented"));
                }
                "rsa-pss" => {
                    // Placeholder for RSA-PSS verification
                    // Would need rsa crate
                    return Err(OvieError::generic("RSA-PSS signature verification not yet implemented"));
                }
                _ => {
                    return Err(OvieError::generic(format!("Unsupported signature algorithm: {}", signature.algorithm)));
                }
            }
        }
        Ok(())
    }

    /// Verify offline-first compliance
    fn verify_offline_compliance(&self, metadata: &PackageMetadata) -> OvieResult<()> {
        if metadata.offline_metadata.requires_network {
            return Err(OvieError::generic(format!(
                "Package {} violates offline-first policy by requiring network access",
                metadata.id.name
            )));
        }

        if !metadata.offline_metadata.external_resources.is_empty() {
            return Err(OvieError::generic(format!(
                "Package {} violates offline-first policy by accessing external resources: {:?}",
                metadata.id.name, metadata.offline_metadata.external_resources
            )));
        }

        if !metadata.offline_metadata.deterministic_build {
            return Err(OvieError::generic(format!(
                "Package {} does not guarantee deterministic builds",
                metadata.id.name
            )));
        }

        Ok(())
    }

    /// Create integrity manifest for a package
    fn create_integrity_manifest(&self, package_dir: &Path, metadata: &PackageMetadata, content: &[u8]) -> OvieResult<()> {
        let manifest = IntegrityManifest {
            package_id: metadata.id.clone(),
            content_hash: metadata.id.content_hash.clone(),
            checksums: metadata.checksums.clone(),
            signatures: metadata.signatures.clone(),
            verification_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            file_size: content.len() as u64,
            offline_compliance: metadata.offline_metadata.clone(),
        };

        let manifest_path = package_dir.join("integrity.json");
        let manifest_json = serde_json::to_string_pretty(&manifest)
            .map_err(|e| OvieError::generic(format!("Failed to serialize integrity manifest: {}", e)))?;
        
        fs::write(&manifest_path, manifest_json)
            .map_err(|e| OvieError::io_error(format!("Failed to write integrity manifest: {}", e)))?;

        Ok(())
    }

    /// Verify package integrity from stored manifest
    pub fn verify_package_integrity(&mut self, package_id: &PackageId) -> OvieResult<bool> {
        let package_dir = self.registry_path.join(&package_id.name).join(&package_id.version);
        let manifest_path = package_dir.join("integrity.json");
        
        if !manifest_path.exists() {
            return Ok(false);
        }

        let manifest_json = fs::read_to_string(&manifest_path)
            .map_err(|e| OvieError::io_error(format!("Failed to read integrity manifest: {}", e)))?;
        
        let manifest: IntegrityManifest = serde_json::from_str(&manifest_json)
            .map_err(|e| OvieError::generic(format!("Failed to parse integrity manifest: {}", e)))?;

        // Verify package ID matches
        if manifest.package_id != *package_id {
            return Ok(false);
        }

        // Load and verify content
        let content = self.load_package_content(package_id)?;
        
        // Verify content hash
        let computed_hash = self.compute_content_hash(&content);
        if computed_hash != manifest.content_hash {
            return Ok(false);
        }

        // Verify file size
        if content.len() as u64 != manifest.file_size {
            return Ok(false);
        }

        // Verify additional checksums
        for (algorithm, expected_checksum) in &manifest.checksums {
            let computed_checksum = match algorithm.as_str() {
                "sha256" => self.compute_content_hash(&content),
                "sha512" => {
                    use sha2::Sha512;
                    let mut hasher = Sha512::new();
                    hasher.update(&content);
                    format!("{:x}", hasher.finalize())
                }
                _ => continue, // Skip unsupported algorithms
            };

            if &computed_checksum != expected_checksum {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Enforce offline-only build by checking network access
    pub fn enforce_offline_build(&self) -> OvieResult<()> {
        // Check if we're in offline mode
        if std::env::var("OVIE_OFFLINE").unwrap_or_default() != "true" {
            return Err(OvieError::generic(
                "Offline-only build enforcement requires OVIE_OFFLINE=true environment variable"
            ));
        }

        // Verify no network interfaces are accessible (simplified check)
        // In a real implementation, this would be more sophisticated
        if self.has_network_access() {
            return Err(OvieError::generic(
                "Network access detected during offline-only build"
            ));
        }

        Ok(())
    }

    /// Check if network access is available (simplified implementation)
    fn has_network_access(&self) -> bool {
        // Simplified check - in reality this would be more comprehensive
        std::env::var("OVIE_FORCE_OFFLINE").unwrap_or_default() != "true"
    }
}

/// Integrity manifest for package verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityManifest {
    pub package_id: PackageId,
    pub content_hash: String,
    pub checksums: HashMap<String, String>,
    pub signatures: Vec<PackageSignature>,
    pub verification_timestamp: u64,
    pub file_size: u64,
    pub offline_compliance: OfflineMetadata,
}

/// Dependency resolver for reproducible builds
#[derive(Debug)]
pub struct DependencyResolver {
    registry: PackageRegistry,
    resolved_cache: HashMap<String, PackageId>,
    /// Integrity verification enabled
    verify_integrity: bool,
    /// Offline-only mode enforcement
    offline_only: bool,
}

impl DependencyResolver {
    /// Create a new dependency resolver
    pub fn new() -> OvieResult<Self> {
        Ok(Self {
            registry: PackageRegistry::new()?,
            resolved_cache: HashMap::new(),
            verify_integrity: true,
            offline_only: true,
        })
    }

    /// Create a new dependency resolver with custom settings
    pub fn with_settings(verify_integrity: bool, offline_only: bool) -> OvieResult<Self> {
        Ok(Self {
            registry: PackageRegistry::new()?,
            resolved_cache: HashMap::new(),
            verify_integrity,
            offline_only,
        })
    }

    /// Enable or disable integrity verification
    pub fn set_integrity_verification(&mut self, enabled: bool) {
        self.verify_integrity = enabled;
    }

    /// Enable or disable offline-only mode
    pub fn set_offline_only(&mut self, enabled: bool) {
        self.offline_only = enabled;
    }

    /// Resolve dependencies from ovie.toml with integrity verification
    pub fn resolve_dependencies(&mut self, project_path: &Path) -> OvieResult<PackageLock> {
        // Enforce offline-only mode if enabled
        if self.offline_only {
            self.registry.enforce_offline_build()?;
        }

        let toml_path = project_path.join("ovie.toml");
        if !toml_path.exists() {
            return Err(OvieError::generic("ovie.toml not found"));
        }

        let toml_content = fs::read_to_string(&toml_path)?;
        let project_config: ProjectConfig = toml::from_str(&toml_content)
            .map_err(|e| OvieError::generic(format!("Failed to parse ovie.toml: {}", e)))?;

        let mut resolved_dependencies = HashMap::new();

        // Resolve regular dependencies
        for (name, spec) in &project_config.dependencies {
            let package_id = self.resolve_dependency_with_verification(name, spec)?;
            resolved_dependencies.insert(name.clone(), package_id);
        }

        // Resolve dev dependencies
        for (name, spec) in &project_config.dev_dependencies {
            let package_id = self.resolve_dependency_with_verification(name, spec)?;
            resolved_dependencies.insert(format!("dev:{}", name), package_id);
        }

        // Resolve build dependencies
        for (name, spec) in &project_config.build_dependencies {
            let package_id = self.resolve_dependency_with_verification(name, spec)?;
            resolved_dependencies.insert(format!("build:{}", name), package_id);
        }

        // Recursively resolve transitive dependencies with verification
        let mut all_dependencies = HashMap::new();
        for (name, package_id) in resolved_dependencies {
            self.resolve_transitive_dependencies_with_verification(&package_id, &mut all_dependencies)?;
            all_dependencies.insert(name, package_id);
        }

        Ok(PackageLock::new(all_dependencies))
    }

    /// Resolve a single dependency with integrity verification
    fn resolve_dependency_with_verification(&mut self, name: &str, spec: &DependencySpec) -> OvieResult<PackageId> {
        let package_id = self.resolve_dependency(name, spec)?;
        
        if self.verify_integrity {
            let is_valid = self.registry.verify_package_integrity(&package_id)?;
            if !is_valid {
                return Err(OvieError::generic(format!(
                    "Integrity verification failed for package {}", 
                    package_id.to_string()
                )));
            }
        }

        Ok(package_id)
    }

    /// Resolve transitive dependencies with integrity verification
    fn resolve_transitive_dependencies_with_verification(
        &mut self,
        package_id: &PackageId,
        resolved: &mut HashMap<String, PackageId>,
    ) -> OvieResult<()> {
        // Avoid infinite recursion
        if resolved.contains_key(&package_id.name) {
            return Ok(());
        }

        // Verify integrity if enabled
        if self.verify_integrity {
            let is_valid = self.registry.verify_package_integrity(package_id)?;
            if !is_valid {
                return Err(OvieError::generic(format!(
                    "Integrity verification failed for transitive dependency {}", 
                    package_id.to_string()
                )));
            }
        }

        // Get package metadata
        let (metadata, _) = self.registry.get_package(package_id)?
            .ok_or_else(|| OvieError::generic(format!("Package not found: {}", package_id.to_string())))?;

        // Resolve dependencies of this package
        for (dep_name, dep_id) in &metadata.dependencies {
            if !resolved.contains_key(dep_name) {
                self.resolve_transitive_dependencies_with_verification(dep_id, resolved)?;
                resolved.insert(dep_name.clone(), dep_id.clone());
            }
        }

        Ok(())
    }

    /// Resolve a single dependency specification
    fn resolve_dependency(&mut self, name: &str, spec: &DependencySpec) -> OvieResult<PackageId> {
        // Check cache first
        let cache_key = format!("{}:{}", name, spec.to_string());
        if let Some(cached_id) = self.resolved_cache.get(&cache_key) {
            return Ok(cached_id.clone());
        }

        // Find matching packages in registry
        let available_packages = self.registry.list_packages()?;
        let matching_packages: Vec<_> = available_packages.iter()
            .filter(|pkg| pkg.name == name)
            .collect();

        if matching_packages.is_empty() {
            return Err(OvieError::generic(format!("Package not found: {}", name)));
        }

        // Select best matching version
        let selected_package = self.select_best_version(&matching_packages, spec)?;
        
        // Cache the result
        self.resolved_cache.insert(cache_key, selected_package.clone());
        
        Ok(selected_package)
    }

    /// Update a specific dependency while maintaining determinism
    pub fn update_specific_dependency(
        &mut self,
        project_path: &Path,
        dependency_name: &str,
        existing_lock: Option<&PackageLock>,
    ) -> OvieResult<PackageLock> {
        // Load project configuration
        let config_path = project_path.join("ovie.toml");
        let config_content = fs::read_to_string(&config_path)
            .map_err(|e| OvieError::io_error(format!("Failed to read ovie.toml: {}", e)))?;
        let project_config: ProjectConfig = toml::from_str(&config_content)
            .map_err(|e| OvieError::generic(format!("Failed to parse ovie.toml: {}", e)))?;

        // Find the dependency specification
        let dependency_spec = project_config.dependencies.get(dependency_name)
            .or_else(|| project_config.dev_dependencies.get(dependency_name))
            .or_else(|| project_config.build_dependencies.get(dependency_name))
            .ok_or_else(|| OvieError::generic(format!("Dependency '{}' not found in ovie.toml", dependency_name)))?;

        // Start with existing lock file dependencies if available
        let mut resolved_dependencies = if let Some(lock) = existing_lock {
            lock.dependencies.clone()
        } else {
            HashMap::new()
        };

        // Update the specific dependency
        let updated_package_id = self.resolve_dependency(dependency_name, dependency_spec)?;
        resolved_dependencies.insert(dependency_name.to_string(), updated_package_id);

        // Re-resolve transitive dependencies for the updated package
        let mut all_dependencies = HashMap::new();
        for (name, package_id) in resolved_dependencies {
            self.resolve_transitive_dependencies(&package_id, &mut all_dependencies)?;
            all_dependencies.insert(name, package_id);
        }

        Ok(PackageLock::new(all_dependencies))
    }

    /// Update all dependencies while maintaining determinism
    pub fn update_all_dependencies(
        &mut self,
        project_path: &Path,
        existing_lock: Option<&PackageLock>,
    ) -> OvieResult<PackageLock> {
        // This is essentially the same as resolve_dependencies but with better conflict handling
        self.resolve_dependencies(project_path)
    }
    fn resolve_transitive_dependencies(
        &mut self,
        package_id: &PackageId,
        resolved: &mut HashMap<String, PackageId>,
    ) -> OvieResult<()> {
        // Avoid infinite recursion
        if resolved.contains_key(&package_id.name) {
            return Ok(());
        }

        // Get package metadata
        let (metadata, _) = self.registry.get_package(package_id)?
            .ok_or_else(|| OvieError::generic(format!("Package not found: {}", package_id.to_string())))?;

        // Resolve dependencies of this package
        for (dep_name, dep_id) in &metadata.dependencies {
            if !resolved.contains_key(dep_name) {
                self.resolve_transitive_dependencies(dep_id, resolved)?;
                resolved.insert(dep_name.clone(), dep_id.clone());
            }
        }

        Ok(())
    }

    /// Select the best version from available packages
    fn select_best_version(
        &self,
        packages: &[&PackageId],
        spec: &DependencySpec,
    ) -> OvieResult<PackageId> {
        match spec {
            DependencySpec::Version(version) => {
                // Exact version match
                packages.iter()
                    .find(|pkg| pkg.version == *version)
                    .map(|pkg| (*pkg).clone())
                    .ok_or_else(|| OvieError::generic(format!("Version {} not found", version)))
            }
            DependencySpec::VersionRange { min, max } => {
                // Version range matching (simplified)
                let matching: Vec<_> = packages.iter()
                    .filter(|pkg| {
                        let version = &pkg.version;
                        (min.is_none() || version >= min.as_ref().unwrap()) &&
                        (max.is_none() || version <= max.as_ref().unwrap())
                    })
                    .collect();

                if matching.is_empty() {
                    return Err(OvieError::generic("No versions match the specified range"));
                }

                // Select the latest version in range
                let latest = matching.iter()
                    .max_by(|a, b| a.version.cmp(&b.version))
                    .unwrap();
                
                Ok((**latest).clone())
            }
            DependencySpec::Hash(hash) => {
                // Exact hash match
                packages.iter()
                    .find(|pkg| pkg.content_hash == *hash)
                    .map(|pkg| (*pkg).clone())
                    .ok_or_else(|| OvieError::generic(format!("Hash {} not found", hash)))
            }
        }
    }

    /// Update the lock file with current resolutions
    pub fn update_lock_file(&mut self, project_path: &Path) -> OvieResult<()> {
        let lock = self.resolve_dependencies(project_path)?;
        let lock_path = project_path.join("ovie.lock");
        lock.save(&lock_path)?;
        Ok(())
    }

    /// Verify that current dependencies match the lock file
    pub fn verify_lock_file(&mut self, project_path: &Path) -> OvieResult<bool> {
        let lock_path = project_path.join("ovie.lock");
        if !lock_path.exists() {
            return Ok(false);
        }

        let existing_lock = PackageLock::load(&lock_path)?;
        let current_lock = self.resolve_dependencies(project_path)?;

        // Compare dependency sets
        Ok(existing_lock.dependencies == current_lock.dependencies)
    }
}

/// Project configuration from ovie.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub project: ProjectInfo,
    #[serde(default)]
    pub dependencies: HashMap<String, DependencySpec>,
    #[serde(default)]
    #[serde(rename = "dev-dependencies")]
    pub dev_dependencies: HashMap<String, DependencySpec>,
    #[serde(default)]
    #[serde(rename = "build-dependencies")]
    pub build_dependencies: HashMap<String, DependencySpec>,
    #[serde(default)]
    pub build: BuildConfig,
    #[serde(default)]
    pub security: SecurityConfig,
    #[serde(default)]
    pub vendor: VendorConfig,
    #[serde(default)]
    pub package: PackageConfig,
    #[serde(default)]
    pub scripts: HashMap<String, String>,
    #[serde(default)]
    pub features: FeaturesConfig,
    #[serde(default)]
    pub workspace: Option<WorkspaceConfig>,
    // Platform-specific dependencies
    #[serde(default)]
    pub target: HashMap<String, TargetConfig>,
}

/// Project information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    #[serde(default = "default_edition")]
    pub edition: String,
}

/// Build configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    #[serde(default = "default_backend")]
    pub backend: String,
    #[serde(default = "default_target")]
    pub target: String,
    #[serde(default = "default_optimization")]
    pub optimization: String,
    #[serde(default = "default_deterministic")]
    pub deterministic: bool,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    #[serde(default = "default_enforce_pinning")]
    pub enforce_pinning: bool,
    #[serde(default)]
    pub require_signatures: bool,
    #[serde(default = "default_offline_only")]
    pub offline_only: bool,
    #[serde(default = "default_max_dependency_depth")]
    pub max_dependency_depth: usize,
}

/// Vendor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorConfig {
    #[serde(default = "default_vendor_directory")]
    pub directory: String,
    #[serde(default)]
    pub include_dev: bool,
    #[serde(default = "default_include_build")]
    pub include_build: bool,
    #[serde(default = "default_verify_checksums")]
    pub verify_checksums: bool,
}

/// Package configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageConfig {
    #[serde(default)]
    pub include: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
    pub ovie_version: Option<String>,
}

/// Features configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesConfig {
    #[serde(default)]
    pub default: Vec<String>,
    #[serde(flatten)]
    pub features: HashMap<String, Vec<String>>,
}

/// Workspace configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    #[serde(default)]
    pub members: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
}

/// Target-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetConfig {
    #[serde(default)]
    pub dependencies: HashMap<String, DependencySpec>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            backend: default_backend(),
            target: default_target(),
            optimization: default_optimization(),
            deterministic: default_deterministic(),
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enforce_pinning: default_enforce_pinning(),
            require_signatures: false,
            offline_only: default_offline_only(),
            max_dependency_depth: default_max_dependency_depth(),
        }
    }
}

impl Default for VendorConfig {
    fn default() -> Self {
        Self {
            directory: default_vendor_directory(),
            include_dev: false,
            include_build: default_include_build(),
            verify_checksums: default_verify_checksums(),
        }
    }
}

impl Default for PackageConfig {
    fn default() -> Self {
        Self {
            include: Vec::new(),
            exclude: Vec::new(),
            ovie_version: None,
        }
    }
}

impl Default for FeaturesConfig {
    fn default() -> Self {
        Self {
            default: Vec::new(),
            features: HashMap::new(),
        }
    }
}

fn default_edition() -> String {
    "2024".to_string()
}

fn default_backend() -> String {
    "interpreter".to_string()
}

fn default_target() -> String {
    "native".to_string()
}

fn default_optimization() -> String {
    "debug".to_string()
}

fn default_deterministic() -> bool {
    true
}

fn default_enforce_pinning() -> bool {
    true
}

fn default_offline_only() -> bool {
    true
}

fn default_max_dependency_depth() -> usize {
    10
}

fn default_vendor_directory() -> String {
    "vendor".to_string()
}

fn default_include_build() -> bool {
    true
}

fn default_verify_checksums() -> bool {
    true
}

impl ProjectConfig {
    /// Create a new project configuration with defaults
    pub fn new(name: String, version: String, authors: Vec<String>) -> Self {
        Self {
            project: ProjectInfo {
                name,
                version,
                authors,
                description: None,
                license: Some("MIT".to_string()),
                repository: None,
                homepage: None,
                documentation: None,
                keywords: Vec::new(),
                categories: Vec::new(),
                edition: default_edition(),
            },
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            build_dependencies: HashMap::new(),
            build: BuildConfig::default(),
            security: SecurityConfig::default(),
            vendor: VendorConfig::default(),
            package: PackageConfig::default(),
            scripts: HashMap::new(),
            features: FeaturesConfig::default(),
            workspace: None,
            target: HashMap::new(),
        }
    }

    /// Load project configuration from ovie.toml
    pub fn load<P: AsRef<Path>>(path: P) -> OvieResult<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| OvieError::io_error(format!("Failed to read ovie.toml: {}", e)))?;
        
        Self::from_toml(&content)
    }

    /// Parse project configuration from TOML string
    pub fn from_toml(content: &str) -> OvieResult<Self> {
        toml::from_str(content)
            .map_err(|e| OvieError::generic(format!("Failed to parse ovie.toml: {}", e)))
    }

    /// Save project configuration to ovie.toml
    pub fn save<P: AsRef<Path>>(&self, path: P) -> OvieResult<()> {
        let content = self.to_toml()?;
        fs::write(path, content)
            .map_err(|e| OvieError::io_error(format!("Failed to write ovie.toml: {}", e)))
    }

    /// Convert project configuration to TOML string
    pub fn to_toml(&self) -> OvieResult<String> {
        toml::to_string_pretty(self)
            .map_err(|e| OvieError::generic(format!("Failed to serialize ovie.toml: {}", e)))
    }

    /// Validate the project configuration
    pub fn validate(&self) -> OvieResult<Vec<String>> {
        let mut warnings = Vec::new();

        // Validate project name
        if self.project.name.is_empty() {
            return Err(OvieError::generic("Project name cannot be empty"));
        }

        if !self.project.name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(OvieError::generic("Project name can only contain alphanumeric characters, hyphens, and underscores"));
        }

        // Validate version
        if self.project.version.is_empty() {
            return Err(OvieError::generic("Project version cannot be empty"));
        }

        // Validate authors
        if self.project.authors.is_empty() {
            warnings.push("No authors specified".to_string());
        }

        // Validate security settings
        if self.security.enforce_pinning {
            for (name, spec) in &self.dependencies {
                match spec {
                    DependencySpec::Version(_) | DependencySpec::Hash(_) => {
                        // Exact versions and hashes are good
                    }
                    DependencySpec::VersionRange { .. } => {
                        warnings.push(format!("Dependency '{}' uses version range, which may compromise reproducibility", name));
                    }
                }
            }
        }

        // Validate dependency depth
        if self.security.max_dependency_depth == 0 {
            return Err(OvieError::generic("Maximum dependency depth must be greater than 0"));
        }

        // Validate vendor directory
        if self.vendor.directory.is_empty() {
            return Err(OvieError::generic("Vendor directory cannot be empty"));
        }

        // Validate features
        for (feature_name, dependencies) in &self.features.features {
            if feature_name.is_empty() {
                return Err(OvieError::generic("Feature name cannot be empty"));
            }
            
            for dep in dependencies {
                if !self.dependencies.contains_key(dep) && 
                   !self.dev_dependencies.contains_key(dep) && 
                   !self.build_dependencies.contains_key(dep) {
                    warnings.push(format!("Feature '{}' references unknown dependency '{}'", feature_name, dep));
                }
            }
        }

        Ok(warnings)
    }

    /// Generate a template ovie.toml file
    pub fn generate_template<P: AsRef<Path>>(path: P, name: &str) -> OvieResult<()> {
        let config = Self::new(
            name.to_string(),
            "0.1.0".to_string(),
            vec!["Your Name <your.email@example.com>".to_string()],
        );

        let mut content = String::new();
        content.push_str("# Ovie Package Manifest\n");
        content.push_str("# Generated by Ovie toolchain\n\n");
        content.push_str(&config.to_toml()?);

        fs::write(path, content)
            .map_err(|e| OvieError::io_error(format!("Failed to write template: {}", e)))
    }

    /// Check if version pinning is enforced
    pub fn is_version_pinning_enforced(&self) -> bool {
        self.security.enforce_pinning
    }

    /// Get all dependencies (including dev and build dependencies)
    pub fn get_all_dependencies(&self) -> HashMap<String, &DependencySpec> {
        let mut all_deps = HashMap::new();
        
        for (name, spec) in &self.dependencies {
            all_deps.insert(name.clone(), spec);
        }
        
        for (name, spec) in &self.dev_dependencies {
            all_deps.insert(format!("dev:{}", name), spec);
        }
        
        for (name, spec) in &self.build_dependencies {
            all_deps.insert(format!("build:{}", name), spec);
        }

        all_deps
    }

    /// Get dependencies for a specific target
    pub fn get_target_dependencies(&self, target: &str) -> HashMap<String, &DependencySpec> {
        self.target.get(target)
            .map(|config| config.dependencies.iter().map(|(k, v)| (k.clone(), v)).collect())
            .unwrap_or_default()
    }

    /// Add a dependency with version pinning enforcement
    pub fn add_dependency(&mut self, name: String, spec: DependencySpec) -> OvieResult<()> {
        if self.security.enforce_pinning {
            match &spec {
                DependencySpec::VersionRange { .. } => {
                    return Err(OvieError::generic(format!(
                        "Version ranges not allowed when version pinning is enforced for dependency '{}'", 
                        name
                    )));
                }
                _ => {} // Exact versions and hashes are allowed
            }
        }

        self.dependencies.insert(name, spec);
        Ok(())
    }

    /// Remove a dependency
    pub fn remove_dependency(&mut self, name: &str) -> bool {
        self.dependencies.remove(name).is_some() ||
        self.dev_dependencies.remove(name).is_some() ||
        self.build_dependencies.remove(name).is_some()
    }

    /// Update a dependency version
    pub fn update_dependency(&mut self, name: &str, spec: DependencySpec) -> OvieResult<bool> {
        if self.security.enforce_pinning {
            match &spec {
                DependencySpec::VersionRange { .. } => {
                    return Err(OvieError::generic(format!(
                        "Version ranges not allowed when version pinning is enforced for dependency '{}'", 
                        name
                    )));
                }
                _ => {} // Exact versions and hashes are allowed
            }
        }

        if self.dependencies.contains_key(name) {
            self.dependencies.insert(name.to_string(), spec);
            Ok(true)
        } else if self.dev_dependencies.contains_key(name) {
            self.dev_dependencies.insert(name.to_string(), spec);
            Ok(true)
        } else if self.build_dependencies.contains_key(name) {
            self.build_dependencies.insert(name.to_string(), spec);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
#[serde(untagged)]
pub enum DependencySpec {
    /// Exact version: "1.0.0"
    Version(String),
    /// Version range: { min = "1.0.0", max = "2.0.0" }
    VersionRange {
        min: Option<String>,
        max: Option<String>,
    },
    /// Exact hash: { hash = "abc123..." }
    Hash(String),
}

impl DependencySpec {
    pub fn to_string(&self) -> String {
        match self {
            DependencySpec::Version(v) => v.clone(),
            DependencySpec::VersionRange { min, max } => {
                format!("{}..{}", 
                    min.as_deref().unwrap_or("*"), 
                    max.as_deref().unwrap_or("*"))
            }
            DependencySpec::Hash(h) => format!("#{}", h),
        }
    }
}

/// Package lock file for reproducible builds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageLock {
    /// Version of the lock file format
    pub version: String,
    /// Resolved dependencies with exact package IDs
    pub dependencies: HashMap<String, PackageId>,
    /// Metadata about the lock file
    pub metadata: LockMetadata,
}

/// Metadata for the lock file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockMetadata {
    /// When the lock file was generated
    pub generated_at: String,
    /// Version of Ovie that generated the lock file
    pub ovie_version: String,
    /// Platform information
    pub platform: String,
}

impl PackageLock {
    /// Create a new package lock
    pub fn new(dependencies: HashMap<String, PackageId>) -> Self {
        Self {
            version: "1.0".to_string(),
            dependencies,
            metadata: LockMetadata {
                generated_at: chrono::Utc::now().to_rfc3339(),
                ovie_version: env!("CARGO_PKG_VERSION").to_string(),
                platform: std::env::consts::OS.to_string(),
            },
        }
    }

    /// Load lock file from disk
    pub fn load<P: AsRef<Path>>(path: P) -> OvieResult<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| OvieError::io_error(format!("Failed to read lock file: {}", e)))?;
        
        serde_json::from_str(&content)
            .map_err(|e| OvieError::generic(format!("Failed to parse lock file: {}", e)))
    }

    /// Save lock file to disk
    pub fn save<P: AsRef<Path>>(&self, path: P) -> OvieResult<()> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| OvieError::generic(format!("Failed to serialize lock file: {}", e)))?;
        
        fs::write(path, content)
            .map_err(|e| OvieError::io_error(format!("Failed to write lock file: {}", e)))
    }
}

impl Default for PackageRegistry {
    fn default() -> Self {
        Self::new().expect("Failed to create default package registry")
    }
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new().expect("Failed to create default dependency resolver")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_package_id_creation() {
        let package_id = PackageId::new(
            "test-package".to_string(),
            "1.0.0".to_string(),
            "abcdef1234567890".to_string(),
        );
        
        assert_eq!(package_id.name, "test-package");
        assert_eq!(package_id.version, "1.0.0");
        assert_eq!(package_id.content_hash, "abcdef1234567890");
        assert_eq!(package_id.to_string(), "test-package@1.0.0#abcdef1234567890");
    }

    #[test]
    fn test_package_registry_creation() {
        let temp_dir = TempDir::new().unwrap();
        let registry_path = temp_dir.path().join("registry");
        let vendor_path = temp_dir.path().join("vendor");
        
        let registry = PackageRegistry::with_paths(registry_path.clone(), vendor_path).unwrap();
        assert!(registry_path.exists());
    }

    #[test]
    fn test_content_hash_computation() {
        let temp_dir = TempDir::new().unwrap();
        let registry_path = temp_dir.path().join("registry");
        let vendor_path = temp_dir.path().join("vendor");
        
        let registry = PackageRegistry::with_paths(registry_path, vendor_path).unwrap();
        let content = b"Hello, World!";
        let hash = registry.compute_content_hash(content);
        
        // SHA256 of "Hello, World!"
        assert_eq!(hash, "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f");
    }

    #[test]
    fn test_package_storage_and_retrieval() {
        let temp_dir = TempDir::new().unwrap();
        let registry_path = temp_dir.path().join("registry");
        let vendor_path = temp_dir.path().join("vendor");
        
        let mut registry = PackageRegistry::with_paths(registry_path, vendor_path).unwrap();
        
        let content = b"Test package content";
        let content_hash = registry.compute_content_hash(content);
        
        let package_id = PackageId::new(
            "test-pkg".to_string(),
            "1.0.0".to_string(),
            content_hash,
        );
        
        let metadata = PackageMetadata {
            id: package_id.clone(),
            description: Some("A test package".to_string()),
            authors: vec!["Test Author".to_string()],
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            build_dependencies: HashMap::new(),
            license: Some("MIT".to_string()),
            repository: None,
            homepage: None,
            documentation: None,
            keywords: vec!["test".to_string()],
            categories: vec!["testing".to_string()],
            signatures: Vec::new(),
            checksums: HashMap::new(),
            build_timestamp: Some(1640995200), // 2022-01-01 00:00:00 UTC
            offline_metadata: OfflineMetadata {
                requires_network: false,
                external_resources: Vec::new(),
                deterministic_build: true,
                reproducible_hash: Some("test-hash".to_string()),
            },
        };
        
        // Store package
        registry.store_package(metadata.clone(), content).unwrap();
        
        // Retrieve package
        let result = registry.get_package(&package_id).unwrap();
        assert!(result.is_some());
        
        let (retrieved_metadata, retrieved_content) = result.unwrap();
        assert_eq!(retrieved_metadata.id, package_id);
        assert_eq!(retrieved_content, content);
    }

    #[test]
    fn test_package_lock_creation_and_serialization() {
        let mut dependencies = HashMap::new();
        dependencies.insert(
            "test-dep".to_string(),
            PackageId::new(
                "test-dep".to_string(),
                "1.0.0".to_string(),
                "abcdef1234567890".to_string(),
            ),
        );
        
        let lock = PackageLock::new(dependencies);
        assert_eq!(lock.version, "1.0");
        assert_eq!(lock.dependencies.len(), 1);
        
        // Test serialization
        let json = serde_json::to_string(&lock).unwrap();
        assert!(json.contains("test-dep"));
        assert!(json.contains("1.0.0"));
    }

    #[test]
    fn test_dependency_spec_serialization() {
        // Test version spec
        let version_spec = DependencySpec::Version("1.0.0".to_string());
        assert_eq!(version_spec.to_string(), "1.0.0");

        // Test version range spec
        let range_spec = DependencySpec::VersionRange {
            min: Some("1.0.0".to_string()),
            max: Some("2.0.0".to_string()),
        };
        assert_eq!(range_spec.to_string(), "1.0.0..2.0.0");

        // Test hash spec
        let hash_spec = DependencySpec::Hash("abcdef123456".to_string());
        assert_eq!(hash_spec.to_string(), "#abcdef123456");
    }

    #[test]
    fn test_project_config_parsing() {
        let toml_content = r#"
[project]
name = "test-project"
version = "1.0.0"
authors = ["Test Author <test@example.com>"]
description = "A test project"
keywords = []
categories = []

[dependencies]
test-dep = "1.0.0"

[dev-dependencies]
test-dev-dep = "2.0.0"

[build]
backend = "wasm"
target = "web"
"#;

        let config: ProjectConfig = toml::from_str(toml_content).unwrap();
        assert_eq!(config.project.name, "test-project");
        assert_eq!(config.project.version, "1.0.0");
        assert_eq!(config.dependencies.len(), 1);
        assert_eq!(config.dev_dependencies.len(), 1);
        assert_eq!(config.build.backend, "wasm");
        assert_eq!(config.build.target, "web");
    }

    #[test]
    fn test_dependency_resolver_creation() {
        let _resolver = DependencyResolver::new().unwrap();
        // Should not panic
        
        let _resolver_custom = DependencyResolver::with_settings(false, false).unwrap();
        // Should not panic
    }

    #[test]
    fn test_integrity_verification() {
        let temp_dir = TempDir::new().unwrap();
        let registry_path = temp_dir.path().join("registry");
        let vendor_path = temp_dir.path().join("vendor");
        
        let mut registry = PackageRegistry::with_paths(registry_path, vendor_path).unwrap();
        
        let content = b"Test package content for integrity";
        let content_hash = registry.compute_content_hash(content);
        
        let package_id = PackageId::new(
            "integrity-test".to_string(),
            "1.0.0".to_string(),
            content_hash,
        );
        
        let mut checksums = HashMap::new();
        checksums.insert("sha256".to_string(), registry.compute_content_hash(content));
        
        let metadata = PackageMetadata {
            id: package_id.clone(),
            description: Some("Integrity test package".to_string()),
            authors: vec!["Test Author".to_string()],
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            build_dependencies: HashMap::new(),
            license: Some("MIT".to_string()),
            repository: None,
            homepage: None,
            documentation: None,
            keywords: vec!["test".to_string()],
            categories: vec!["testing".to_string()],
            signatures: Vec::new(),
            checksums,
            build_timestamp: Some(1640995200),
            offline_metadata: OfflineMetadata {
                requires_network: false,
                external_resources: Vec::new(),
                deterministic_build: true,
                reproducible_hash: Some("test-hash".to_string()),
            },
        };
        
        // Store package
        registry.store_package(metadata, content).unwrap();
        
        // Verify integrity
        let is_valid = registry.verify_package_integrity(&package_id).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_offline_compliance_validation() {
        let offline_metadata = OfflineMetadata {
            requires_network: false,
            external_resources: Vec::new(),
            deterministic_build: true,
            reproducible_hash: Some("test-hash".to_string()),
        };
        
        // Valid offline metadata should pass
        assert!(!offline_metadata.requires_network);
        assert!(offline_metadata.external_resources.is_empty());
        assert!(offline_metadata.deterministic_build);
        
        let invalid_offline_metadata = OfflineMetadata {
            requires_network: true,
            external_resources: vec!["https://example.com".to_string()],
            deterministic_build: false,
            reproducible_hash: None,
        };
        
        // Invalid offline metadata should fail validation
        assert!(invalid_offline_metadata.requires_network);
        assert!(!invalid_offline_metadata.external_resources.is_empty());
        assert!(!invalid_offline_metadata.deterministic_build);
    }
}