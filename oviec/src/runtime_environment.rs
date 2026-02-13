// Ovie Runtime Environment (ORE) Discovery System
// This module implements the canonical directory structure discovery and validation
// for Ovie v2.2 Complete Language Consolidation

use std::path::{Path, PathBuf};
use std::env;
use std::fs;
use std::fmt;

/// Represents the Ovie Runtime Environment with canonical directory structure
#[derive(Debug, Clone, PartialEq)]
pub struct OvieRuntimeEnvironment {
    pub ovie_home: PathBuf,
    pub bin_dir: PathBuf,
    pub std_dir: PathBuf,
    pub aproko_dir: PathBuf,
    pub targets_dir: PathBuf,
    pub config_dir: PathBuf,
    pub logs_dir: PathBuf,
}

/// Errors that can occur during ORE discovery and validation
#[derive(Debug, Clone)]
pub enum OreError {
    NotFound(String),
    InvalidStructure(String),
    MissingDirectory(PathBuf),
    MissingFile(PathBuf),
    PermissionDenied(PathBuf),
    IoError(String),
}

impl fmt::Display for OreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OreError::NotFound(msg) => write!(f, "ORE not found: {}", msg),
            OreError::InvalidStructure(msg) => write!(f, "Invalid ORE structure: {}", msg),
            OreError::MissingDirectory(path) => write!(f, "Missing required directory: {}", path.display()),
            OreError::MissingFile(path) => write!(f, "Missing required file: {}", path.display()),
            OreError::PermissionDenied(path) => write!(f, "Permission denied: {}", path.display()),
            OreError::IoError(msg) => write!(f, "I/O error: {}", msg),
        }
    }
}

impl std::error::Error for OreError {}

/// Health report for ORE self-check
#[derive(Debug, Clone)]
pub struct HealthReport {
    pub overall_status: HealthStatus,
    pub components: Vec<ComponentHealth>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct ComponentHealth {
    pub name: String,
    pub status: HealthStatus,
    pub message: String,
}

impl OvieRuntimeEnvironment {
    /// Discover the Ovie Runtime Environment using precedence ordering
    /// 
    /// Discovery order:
    /// 1. OVIE_HOME environment variable
    /// 2. Current directory .ovie/ subdirectory
    /// 3. Executable directory
    /// 4. System-wide locations
    pub fn discover() -> Result<Self, OreError> {
        // 1. Check OVIE_HOME environment variable
        if let Ok(ovie_home) = env::var("OVIE_HOME") {
            let path = PathBuf::from(ovie_home);
            if let Ok(ore) = Self::from_path(&path) {
                return Ok(ore);
            }
        }

        // 2. Check current directory for .ovie/
        if let Ok(current_dir) = env::current_dir() {
            let ovie_dir = current_dir.join(".ovie");
            if ovie_dir.exists() {
                if let Ok(ore) = Self::from_path(&ovie_dir) {
                    return Ok(ore);
                }
            }
        }

        // 3. Check executable directory
        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                // Check if executable is in a bin/ directory
                if exe_dir.file_name() == Some(std::ffi::OsStr::new("bin")) {
                    if let Some(parent) = exe_dir.parent() {
                        if let Ok(ore) = Self::from_path(parent) {
                            return Ok(ore);
                        }
                    }
                }
                
                // Check executable directory directly
                if let Ok(ore) = Self::from_path(exe_dir) {
                    return Ok(ore);
                }
            }
        }

        // 4. Check system-wide locations
        let system_paths = Self::get_system_paths();
        for path in system_paths {
            if path.exists() {
                if let Ok(ore) = Self::from_path(&path) {
                    return Ok(ore);
                }
            }
        }

        Err(OreError::NotFound(
            "Could not find Ovie Runtime Environment. Please set OVIE_HOME or ensure proper installation.".to_string()
        ))
    }

    /// Create ORE from a given path
    fn from_path(ovie_home: &Path) -> Result<Self, OreError> {
        if !ovie_home.exists() {
            return Err(OreError::MissingDirectory(ovie_home.to_path_buf()));
        }

        let ore = Self {
            ovie_home: ovie_home.to_path_buf(),
            bin_dir: ovie_home.join("bin"),
            std_dir: ovie_home.join("std"),
            aproko_dir: ovie_home.join("aproko"),
            targets_dir: ovie_home.join("targets"),
            config_dir: ovie_home.join("config"),
            logs_dir: ovie_home.join("logs"),
        };

        // Basic validation - at least std/ should exist for a valid ORE
        if !ore.std_dir.exists() {
            return Err(OreError::InvalidStructure(
                format!("Missing std/ directory in {}", ovie_home.display())
            ));
        }

        Ok(ore)
    }

    /// Get system-wide search paths for different platforms
    fn get_system_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();

        #[cfg(unix)]
        {
            paths.push(PathBuf::from("/usr/local/ovie"));
            paths.push(PathBuf::from("/opt/ovie"));
            paths.push(PathBuf::from("/usr/share/ovie"));
            
            // Check user home directory
            if let Ok(home) = env::var("HOME") {
                paths.push(PathBuf::from(home).join(".ovie"));
                paths.push(PathBuf::from(home).join(".local/share/ovie"));
            }
        }

        #[cfg(windows)]
        {
            // Windows system paths
            if let Ok(program_files) = env::var("PROGRAMFILES") {
                paths.push(PathBuf::from(program_files).join("Ovie"));
            }
            
            if let Ok(appdata) = env::var("APPDATA") {
                paths.push(PathBuf::from(appdata).join("Ovie"));
            }
            
            if let Ok(localappdata) = env::var("LOCALAPPDATA") {
                paths.push(PathBuf::from(localappdata).join("Ovie"));
            }
        }

        #[cfg(target_os = "macos")]
        {
            paths.push(PathBuf::from("/Applications/Ovie.app/Contents/Resources"));
            
            if let Ok(home) = env::var("HOME") {
                paths.push(PathBuf::from(home).join("Library/Application Support/Ovie"));
            }
        }

        paths
    }

    /// Validate the ORE structure and components
    pub fn validate(&self) -> Result<(), OreError> {
        // Check required directories exist
        let required_dirs = [
            (&self.bin_dir, "bin"),
            (&self.std_dir, "std"),
            (&self.aproko_dir, "aproko"),
            (&self.targets_dir, "targets"),
            (&self.config_dir, "config"),
            (&self.logs_dir, "logs"),
        ];

        for (dir, name) in &required_dirs {
            if !dir.exists() {
                return Err(OreError::MissingDirectory(dir.to_path_buf()));
            }
            
            // Check if directory is readable
            if let Err(_) = fs::read_dir(dir) {
                return Err(OreError::PermissionDenied(dir.to_path_buf()));
            }
        }

        // Validate std/ contains required modules
        self.validate_std_modules()?;

        // Validate aproko/ configuration
        self.validate_aproko_config()?;

        // Validate targets/ has at least one backend
        self.validate_targets()?;

        Ok(())
    }

    /// Validate standard library modules
    fn validate_std_modules(&self) -> Result<(), OreError> {
        let required_modules = [
            "core", "math", "io", "fs", "time", "env", "cli", "test", "log"
        ];

        for module in &required_modules {
            let module_path = self.std_dir.join(format!("{}/mod.ov", module));
            if !module_path.exists() {
                return Err(OreError::MissingFile(module_path));
            }
        }

        Ok(())
    }

    /// Validate aproko configuration
    fn validate_aproko_config(&self) -> Result<(), OreError> {
        // Check for aproko configuration file
        let config_file = self.aproko_dir.join("aproko.toml");
        if !config_file.exists() {
            return Err(OreError::MissingFile(config_file));
        }

        Ok(())
    }

    /// Validate targets directory has at least one backend
    fn validate_targets(&self) -> Result<(), OreError> {
        let targets_dir = &self.targets_dir;
        
        if let Ok(entries) = fs::read_dir(targets_dir) {
            let mut has_target = false;
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.path().is_dir() {
                        has_target = true;
                        break;
                    }
                }
            }
            
            if !has_target {
                return Err(OreError::InvalidStructure(
                    "No target backends found in targets/ directory".to_string()
                ));
            }
        }

        Ok(())
    }

    /// Perform comprehensive health check of the ORE
    pub fn self_check(&self) -> HealthReport {
        let mut components = Vec::new();
        let mut warnings = Vec::new();
        let mut errors = Vec::new();

        // Check each component
        self.check_component("OVIE_HOME", &self.ovie_home, &mut components, &mut warnings, &mut errors);
        self.check_component("bin", &self.bin_dir, &mut components, &mut warnings, &mut errors);
        self.check_component("std", &self.std_dir, &mut components, &mut warnings, &mut errors);
        self.check_component("aproko", &self.aproko_dir, &mut components, &mut warnings, &mut errors);
        self.check_component("targets", &self.targets_dir, &mut components, &mut warnings, &mut errors);
        self.check_component("config", &self.config_dir, &mut components, &mut warnings, &mut errors);
        self.check_component("logs", &self.logs_dir, &mut components, &mut warnings, &mut errors);

        // Check standard library modules
        self.check_std_modules(&mut components, &mut warnings, &mut errors);

        // Determine overall status
        let overall_status = if !errors.is_empty() {
            HealthStatus::Error
        } else if !warnings.is_empty() {
            HealthStatus::Warning
        } else {
            HealthStatus::Healthy
        };

        HealthReport {
            overall_status,
            components,
            warnings,
            errors,
        }
    }

    fn check_component(
        &self,
        name: &str,
        path: &Path,
        components: &mut Vec<ComponentHealth>,
        warnings: &mut Vec<String>,
        errors: &mut Vec<String>,
    ) {
        if path.exists() {
            if let Err(_) = fs::read_dir(path) {
                components.push(ComponentHealth {
                    name: name.to_string(),
                    status: HealthStatus::Error,
                    message: "Permission denied".to_string(),
                });
                errors.push(format!("{}: Permission denied", name));
            } else {
                components.push(ComponentHealth {
                    name: name.to_string(),
                    status: HealthStatus::Healthy,
                    message: "OK".to_string(),
                });
            }
        } else {
            components.push(ComponentHealth {
                name: name.to_string(),
                status: HealthStatus::Error,
                message: "Missing".to_string(),
            });
            errors.push(format!("{}: Directory missing", name));
        }
    }

    fn check_std_modules(
        &self,
        components: &mut Vec<ComponentHealth>,
        warnings: &mut Vec<String>,
        errors: &mut Vec<String>,
    ) {
        let required_modules = [
            "core", "math", "io", "fs", "time", "env", "cli", "test", "log"
        ];

        let mut missing_modules = Vec::new();
        
        for module in &required_modules {
            let module_path = self.std_dir.join(format!("{}/mod.ov", module));
            if !module_path.exists() {
                missing_modules.push(*module);
            }
        }

        if missing_modules.is_empty() {
            components.push(ComponentHealth {
                name: "std modules".to_string(),
                status: HealthStatus::Healthy,
                message: "All required modules present".to_string(),
            });
        } else {
            components.push(ComponentHealth {
                name: "std modules".to_string(),
                status: HealthStatus::Error,
                message: format!("Missing modules: {}", missing_modules.join(", ")),
            });
            errors.push(format!("Missing std modules: {}", missing_modules.join(", ")));
        }
    }

    /// Get environment status as a formatted string
    pub fn env_status(&self) -> String {
        format!(
            "Ovie Runtime Environment Status:\n\
             OVIE_HOME: {}\n\
             bin/: {}\n\
             std/: {}\n\
             aproko/: {}\n\
             targets/: {}\n\
             config/: {}\n\
             logs/: {}",
            self.ovie_home.display(),
            if self.bin_dir.exists() { "✓" } else { "✗" },
            if self.std_dir.exists() { "✓" } else { "✗" },
            if self.aproko_dir.exists() { "✓" } else { "✗" },
            if self.targets_dir.exists() { "✓" } else { "✗" },
            if self.config_dir.exists() { "✓" } else { "✗" },
            if self.logs_dir.exists() { "✓" } else { "✗" },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_ore() -> (TempDir, OvieRuntimeEnvironment) {
        let temp_dir = TempDir::new().unwrap();
        let ovie_home = temp_dir.path();

        // Create required directories
        fs::create_dir_all(ovie_home.join("bin")).unwrap();
        fs::create_dir_all(ovie_home.join("std")).unwrap();
        fs::create_dir_all(ovie_home.join("aproko")).unwrap();
        fs::create_dir_all(ovie_home.join("targets")).unwrap();
        fs::create_dir_all(ovie_home.join("config")).unwrap();
        fs::create_dir_all(ovie_home.join("logs")).unwrap();

        // Create required std modules
        let required_modules = ["core", "math", "io", "fs", "time", "env", "cli", "test", "log"];
        for module in &required_modules {
            let module_dir = ovie_home.join("std").join(module);
            fs::create_dir_all(&module_dir).unwrap();
            fs::write(module_dir.join("mod.ov"), "// Test module").unwrap();
        }

        // Create aproko config
        fs::write(ovie_home.join("aproko/aproko.toml"), "# Test config").unwrap();

        // Create a target backend
        fs::create_dir_all(ovie_home.join("targets/native")).unwrap();

        let ore = OvieRuntimeEnvironment::from_path(ovie_home).unwrap();
        (temp_dir, ore)
    }

    #[test]
    fn test_ore_discovery_ovie_home() {
        let (_temp_dir, ore) = create_test_ore();
        
        // Test OVIE_HOME detection
        env::set_var("OVIE_HOME", ore.ovie_home.to_str().unwrap());
        let discovered = OvieRuntimeEnvironment::discover().unwrap();
        assert_eq!(discovered.ovie_home, ore.ovie_home);
        
        env::remove_var("OVIE_HOME");
    }

    #[test]
    fn test_ore_validation_complete() {
        let (_temp_dir, ore) = create_test_ore();
        
        // Complete ORE should validate successfully
        ore.validate().unwrap();
    }

    #[test]
    fn test_ore_validation_incomplete() {
        let temp_dir = TempDir::new().unwrap();
        let ovie_home = temp_dir.path();
        
        // Create minimal structure (missing required components)
        fs::create_dir_all(ovie_home.join("std")).unwrap();
        
        let ore = OvieRuntimeEnvironment::from_path(ovie_home).unwrap();
        let result = ore.validate();
        
        assert!(result.is_err());
        match result.unwrap_err() {
            OreError::MissingDirectory(_) => {}, // Expected
            other => panic!("Expected MissingDirectory error, got: {:?}", other),
        }
    }

    #[test]
    fn test_ore_self_check() {
        let (_temp_dir, ore) = create_test_ore();
        
        let health_report = ore.self_check();
        assert_eq!(health_report.overall_status, HealthStatus::Healthy);
        assert!(health_report.errors.is_empty());
    }

    #[test]
    fn test_ore_env_status() {
        let (_temp_dir, ore) = create_test_ore();
        
        let status = ore.env_status();
        assert!(status.contains("Ovie Runtime Environment Status"));
        assert!(status.contains("✓")); // Should have checkmarks for existing dirs
    }

    #[test]
    fn test_ore_missing_std_modules() {
        let temp_dir = TempDir::new().unwrap();
        let ovie_home = temp_dir.path();
        
        // Create structure but missing some std modules
        fs::create_dir_all(ovie_home.join("bin")).unwrap();
        fs::create_dir_all(ovie_home.join("std/core")).unwrap();
        fs::write(ovie_home.join("std/core/mod.ov"), "// Test").unwrap();
        fs::create_dir_all(ovie_home.join("aproko")).unwrap();
        fs::create_dir_all(ovie_home.join("targets/native")).unwrap();
        fs::create_dir_all(ovie_home.join("config")).unwrap();
        fs::create_dir_all(ovie_home.join("logs")).unwrap();
        fs::write(ovie_home.join("aproko/aproko.toml"), "# Test").unwrap();
        
        let ore = OvieRuntimeEnvironment::from_path(ovie_home).unwrap();
        let result = ore.validate();
        
        assert!(result.is_err());
        match result.unwrap_err() {
            OreError::MissingFile(path) => {
                assert!(path.to_string_lossy().contains("mod.ov"));
            },
            other => panic!("Expected MissingFile error, got: {:?}", other),
        }
    }

    #[test]
    fn test_system_paths_not_empty() {
        let paths = OvieRuntimeEnvironment::get_system_paths();
        assert!(!paths.is_empty(), "System paths should not be empty");
    }

    // Task 1.4.2: Test validation with incomplete environments
    #[test]
    fn test_ore_validation_missing_bin_directory() {
        let temp_dir = TempDir::new().unwrap();
        let ovie_home = temp_dir.path();
        
        // Create structure missing bin/ directory
        fs::create_dir_all(ovie_home.join("std/core")).unwrap();
        fs::write(ovie_home.join("std/core/mod.ov"), "// Test").unwrap();
        fs::create_dir_all(ovie_home.join("aproko")).unwrap();
        fs::create_dir_all(ovie_home.join("targets/native")).unwrap();
        fs::create_dir_all(ovie_home.join("config")).unwrap();
        fs::create_dir_all(ovie_home.join("logs")).unwrap();
        fs::write(ovie_home.join("aproko/aproko.toml"), "# Test").unwrap();
        
        let ore = OvieRuntimeEnvironment::from_path(ovie_home).unwrap();
        let result = ore.validate();
        
        assert!(result.is_err());
        match result.unwrap_err() {
            OreError::MissingDirectory(path) => {
                assert!(path.to_string_lossy().contains("bin"));
            },
            other => panic!("Expected MissingDirectory error for bin/, got: {:?}", other),
        }
    }

    #[test]
    fn test_ore_validation_missing_aproko_config() {
        let temp_dir = TempDir::new().unwrap();
        let ovie_home = temp_dir.path();
        
        // Create structure missing aproko.toml
        fs::create_dir_all(ovie_home.join("bin")).unwrap();
        let required_modules = ["core", "math", "io", "fs", "time", "env", "cli", "test", "log"];
        for module in &required_modules {
            let module_dir = ovie_home.join("std").join(module);
            fs::create_dir_all(&module_dir).unwrap();
            fs::write(module_dir.join("mod.ov"), "// Test module").unwrap();
        }
        fs::create_dir_all(ovie_home.join("aproko")).unwrap();
        fs::create_dir_all(ovie_home.join("targets/native")).unwrap();
        fs::create_dir_all(ovie_home.join("config")).unwrap();
        fs::create_dir_all(ovie_home.join("logs")).unwrap();
        // Note: NOT creating aproko.toml
        
        let ore = OvieRuntimeEnvironment::from_path(ovie_home).unwrap();
        let result = ore.validate();
        
        assert!(result.is_err());
        match result.unwrap_err() {
            OreError::MissingFile(path) => {
                assert!(path.to_string_lossy().contains("aproko.toml"));
            },
            other => panic!("Expected MissingFile error for aproko.toml, got: {:?}", other),
        }
    }

    #[test]
    fn test_ore_validation_missing_targets() {
        let temp_dir = TempDir::new().unwrap();
        let ovie_home = temp_dir.path();
        
        // Create structure with empty targets/ directory
        fs::create_dir_all(ovie_home.join("bin")).unwrap();
        let required_modules = ["core", "math", "io", "fs", "time", "env", "cli", "test", "log"];
        for module in &required_modules {
            let module_dir = ovie_home.join("std").join(module);
            fs::create_dir_all(&module_dir).unwrap();
            fs::write(module_dir.join("mod.ov"), "// Test module").unwrap();
        }
        fs::create_dir_all(ovie_home.join("aproko")).unwrap();
        fs::create_dir_all(ovie_home.join("targets")).unwrap(); // Empty targets dir
        fs::create_dir_all(ovie_home.join("config")).unwrap();
        fs::create_dir_all(ovie_home.join("logs")).unwrap();
        fs::write(ovie_home.join("aproko/aproko.toml"), "# Test").unwrap();
        
        let ore = OvieRuntimeEnvironment::from_path(ovie_home).unwrap();
        let result = ore.validate();
        
        assert!(result.is_err());
        match result.unwrap_err() {
            OreError::InvalidStructure(msg) => {
                assert!(msg.contains("No target backends found"));
            },
            other => panic!("Expected InvalidStructure error for empty targets/, got: {:?}", other),
        }
    }

    // Task 1.4.3: Test error reporting for missing components
    #[test]
    fn test_ore_error_reporting_missing_std_directory() {
        let temp_dir = TempDir::new().unwrap();
        let ovie_home = temp_dir.path();
        
        // Create minimal structure without std/ directory
        fs::create_dir_all(ovie_home.join("bin")).unwrap();
        
        let result = OvieRuntimeEnvironment::from_path(ovie_home);
        
        assert!(result.is_err());
        match result.unwrap_err() {
            OreError::InvalidStructure(msg) => {
                assert!(msg.contains("Missing std/ directory"));
                assert!(msg.contains(&*ovie_home.to_string_lossy()));
            },
            other => panic!("Expected InvalidStructure error, got: {:?}", other),
        }
    }

    #[test]
    fn test_ore_error_reporting_permission_denied() {
        let (_temp_dir, ore) = create_test_ore();
        
        // Test error message formatting
        let error = OreError::PermissionDenied(ore.bin_dir.clone());
        let error_msg = error.to_string();
        
        assert!(error_msg.contains("Permission denied"));
        assert!(error_msg.contains("bin"));
    }

    #[test]
    fn test_ore_error_reporting_not_found() {
        // Test discovery failure error message
        env::remove_var("OVIE_HOME");
        
        // Temporarily change to a directory without .ovie/
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        
        let result = OvieRuntimeEnvironment::discover();
        
        // Restore original directory
        env::set_current_dir(original_dir).unwrap();
        
        assert!(result.is_err());
        match result.unwrap_err() {
            OreError::NotFound(msg) => {
                assert!(msg.contains("Could not find Ovie Runtime Environment"));
                assert!(msg.contains("OVIE_HOME"));
            },
            other => panic!("Expected NotFound error, got: {:?}", other),
        }
    }

    // Task 1.4.4: Test self-check command functionality
    #[test]
    fn test_self_check_healthy_environment() {
        let (_temp_dir, ore) = create_test_ore();
        
        let health_report = ore.self_check();
        
        assert_eq!(health_report.overall_status, HealthStatus::Healthy);
        assert!(health_report.errors.is_empty());
        assert!(health_report.warnings.is_empty());
        
        // Check that all components are reported as healthy
        let component_names: Vec<&str> = health_report.components.iter()
            .map(|c| c.name.as_str())
            .collect();
        
        assert!(component_names.contains(&"OVIE_HOME"));
        assert!(component_names.contains(&"bin"));
        assert!(component_names.contains(&"std"));
        assert!(component_names.contains(&"aproko"));
        assert!(component_names.contains(&"targets"));
        assert!(component_names.contains(&"config"));
        assert!(component_names.contains(&"logs"));
        assert!(component_names.contains(&"std modules"));
        
        // All components should be healthy
        for component in &health_report.components {
            assert_eq!(component.status, HealthStatus::Healthy);
        }
    }

    #[test]
    fn test_self_check_missing_directory() {
        let temp_dir = TempDir::new().unwrap();
        let ovie_home = temp_dir.path();
        
        // Create incomplete structure (missing logs directory)
        fs::create_dir_all(ovie_home.join("bin")).unwrap();
        fs::create_dir_all(ovie_home.join("std/core")).unwrap();
        fs::write(ovie_home.join("std/core/mod.ov"), "// Test").unwrap();
        fs::create_dir_all(ovie_home.join("aproko")).unwrap();
        fs::create_dir_all(ovie_home.join("targets/native")).unwrap();
        fs::create_dir_all(ovie_home.join("config")).unwrap();
        // Missing logs/ directory
        fs::write(ovie_home.join("aproko/aproko.toml"), "# Test").unwrap();
        
        let ore = OvieRuntimeEnvironment::from_path(ovie_home).unwrap();
        let health_report = ore.self_check();
        
        assert_eq!(health_report.overall_status, HealthStatus::Error);
        assert!(!health_report.errors.is_empty());
        
        // Should report logs directory as missing
        assert!(health_report.errors.iter().any(|e| e.contains("logs")));
        
        // Find the logs component and verify it's marked as error
        let logs_component = health_report.components.iter()
            .find(|c| c.name == "logs")
            .expect("Should have logs component");
        assert_eq!(logs_component.status, HealthStatus::Error);
        assert_eq!(logs_component.message, "Missing");
    }

    #[test]
    fn test_self_check_missing_std_modules() {
        let temp_dir = TempDir::new().unwrap();
        let ovie_home = temp_dir.path();
        
        // Create structure with only some std modules
        fs::create_dir_all(ovie_home.join("bin")).unwrap();
        fs::create_dir_all(ovie_home.join("std/core")).unwrap();
        fs::write(ovie_home.join("std/core/mod.ov"), "// Test").unwrap();
        fs::create_dir_all(ovie_home.join("std/math")).unwrap();
        fs::write(ovie_home.join("std/math/mod.ov"), "// Test").unwrap();
        // Missing other required modules
        fs::create_dir_all(ovie_home.join("aproko")).unwrap();
        fs::create_dir_all(ovie_home.join("targets/native")).unwrap();
        fs::create_dir_all(ovie_home.join("config")).unwrap();
        fs::create_dir_all(ovie_home.join("logs")).unwrap();
        fs::write(ovie_home.join("aproko/aproko.toml"), "# Test").unwrap();
        
        let ore = OvieRuntimeEnvironment::from_path(ovie_home).unwrap();
        let health_report = ore.self_check();
        
        assert_eq!(health_report.overall_status, HealthStatus::Error);
        assert!(!health_report.errors.is_empty());
        
        // Should report missing std modules
        assert!(health_report.errors.iter().any(|e| e.contains("Missing std modules")));
        
        // Find the std modules component and verify it's marked as error
        let std_modules_component = health_report.components.iter()
            .find(|c| c.name == "std modules")
            .expect("Should have std modules component");
        assert_eq!(std_modules_component.status, HealthStatus::Error);
        assert!(std_modules_component.message.contains("Missing modules"));
    }

    #[test]
    fn test_env_status_formatting() {
        let (_temp_dir, ore) = create_test_ore();
        
        let status = ore.env_status();
        
        // Check that status contains expected sections
        assert!(status.contains("Ovie Runtime Environment Status:"));
        assert!(status.contains("OVIE_HOME:"));
        assert!(status.contains("bin/:"));
        assert!(status.contains("std/:"));
        assert!(status.contains("aproko/:"));
        assert!(status.contains("targets/:"));
        assert!(status.contains("config/:"));
        assert!(status.contains("logs/:"));
        
        // Should contain checkmarks for existing directories
        assert!(status.contains("✓"));
        
        // Should show the actual path
        assert!(status.contains(&*ore.ovie_home.to_string_lossy()));
    }
}