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

    /// Store a package in the local registry
    /// Store a package in the local registry
    pub fn store_package(&mut self, metadata: PackageMetadata, content: &[u8]) -> OvieResult<()> {
        // Validate package security first
        let source_url = metadata.repository.as_deref().unwrap_or("unknown");
        let is_valid = self.security.validate_package(
            content,
            source_url,
            &metadata.id.content_hash,
            None, // TODO: Add signature support
            None, // TODO: Add key ID support
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
}

/// Dependency resolver for reproducible builds
#[derive(Debug)]
pub struct DependencyResolver {
    registry: PackageRegistry,
    resolved_cache: HashMap<String, PackageId>,
}

impl DependencyResolver {
    /// Create a new dependency resolver
    pub fn new() -> OvieResult<Self> {
        Ok(Self {
            registry: PackageRegistry::new()?,
            resolved_cache: HashMap::new(),
        })
    }

    /// Resolve dependencies from ovie.toml
    pub fn resolve_dependencies(&mut self, project_path: &Path) -> OvieResult<PackageLock> {
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
            let package_id = self.resolve_dependency(name, spec)?;
            resolved_dependencies.insert(name.clone(), package_id);
        }

        // Resolve dev dependencies
        for (name, spec) in &project_config.dev_dependencies {
            let package_id = self.resolve_dependency(name, spec)?;
            resolved_dependencies.insert(format!("dev:{}", name), package_id);
        }

        // Resolve build dependencies
        for (name, spec) in &project_config.build_dependencies {
            let package_id = self.resolve_dependency(name, spec)?;
            resolved_dependencies.insert(format!("build:{}", name), package_id);
        }

        // Recursively resolve transitive dependencies
        let mut all_dependencies = HashMap::new();
        for (name, package_id) in resolved_dependencies {
            self.resolve_transitive_dependencies(&package_id, &mut all_dependencies)?;
            all_dependencies.insert(name, package_id);
        }

        Ok(PackageLock::new(all_dependencies))
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
}

/// Build configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    #[serde(default = "default_backend")]
    pub backend: String,
    #[serde(default = "default_target")]
    pub target: String,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            backend: default_backend(),
            target: default_target(),
        }
    }
}

fn default_backend() -> String {
    "interpreter".to_string()
}

fn default_target() -> String {
    "native".to_string()
}

/// Dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
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
        let temp_dir = TempDir::new().unwrap();
        let registry_path = temp_dir.path().join("registry");
        let vendor_path = temp_dir.path().join("vendor");
        
        let _resolver = DependencyResolver::new();
        // Should not panic
    }
}