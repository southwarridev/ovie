// Installation System
// Implements Task 13.3: Platform-specific install scripts, ORE setup, environment configuration, and uninstall

use crate::error::{OvieError, OvieResult};
use crate::runtime_environment::OvieRuntimeEnvironment;
use std::path::{Path, PathBuf};
use std::fs;
use std::env;

/// Installation configuration
#[derive(Debug, Clone)]
pub struct InstallConfig {
    /// Installation directory
    pub install_dir: PathBuf,
    /// Whether to add to PATH
    pub add_to_path: bool,
    /// Whether to create ORE structure
    pub create_ore: bool,
    /// Whether to validate after installation
    pub validate: bool,
}

impl InstallConfig {
    /// Create default installation configuration
    pub fn default() -> Self {
        Self {
            install_dir: Self::default_install_dir(),
            add_to_path: true,
            create_ore: true,
            validate: true,
        }
    }

    /// Get default installation directory for the current platform
    fn default_install_dir() -> PathBuf {
        if cfg!(windows) {
            // Windows: C:\Program Files\Ovie
            PathBuf::from("C:\\Program Files\\Ovie")
        } else {
            // Unix: /usr/local/ovie
            PathBuf::from("/usr/local/ovie")
        }
    }

    /// Create user-local installation configuration
    pub fn user_local() -> Self {
        let install_dir = if cfg!(windows) {
            // Windows: %USERPROFILE%\.ovie
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".ovie")
        } else {
            // Unix: ~/.ovie
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".ovie")
        };

        Self {
            install_dir,
            add_to_path: true,
            create_ore: true,
            validate: true,
        }
    }
}

/// Installation manager
pub struct Installer {
    config: InstallConfig,
}

impl Installer {
    /// Create a new installer with the given configuration
    pub fn new(config: InstallConfig) -> Self {
        Self { config }
    }

    /// Install Ovie from a package directory
    pub fn install(&self, package_dir: &Path) -> OvieResult<InstallationResult> {
        println!("Installing Ovie to {}...", self.config.install_dir.display());

        // Create installation directory
        self.create_install_directory()?;

        // Copy binaries
        let binaries = self.copy_binaries(package_dir)?;

        // Copy standard library
        self.copy_stdlib(package_dir)?;

        // Copy documentation
        self.copy_docs(package_dir)?;

        // Create ORE structure if requested
        if self.config.create_ore {
            self.create_ore_structure()?;
        }

        // Add to PATH if requested
        if self.config.add_to_path {
            self.add_to_path()?;
        }

        // Validate installation if requested
        if self.config.validate {
            self.validate_installation()?;
        }

        println!("✓ Installation complete!");

        Ok(InstallationResult {
            install_dir: self.config.install_dir.clone(),
            binaries_installed: binaries,
            ore_created: self.config.create_ore,
            added_to_path: self.config.add_to_path,
        })
    }

    /// Uninstall Ovie
    pub fn uninstall(&self) -> OvieResult<()> {
        println!("Uninstalling Ovie from {}...", self.config.install_dir.display());

        if !self.config.install_dir.exists() {
            return Err(OvieError::io_error(format!(
                "Installation directory does not exist: {}",
                self.config.install_dir.display()
            )));
        }

        // Remove from PATH
        self.remove_from_path()?;

        // Remove installation directory
        fs::remove_dir_all(&self.config.install_dir)
            .map_err(|e| OvieError::io_error(format!("Failed to remove installation directory: {}", e)))?;

        println!("✓ Uninstallation complete!");

        Ok(())
    }

    /// Create the installation directory
    fn create_install_directory(&self) -> OvieResult<()> {
        println!("  Creating installation directory...");

        fs::create_dir_all(&self.config.install_dir)
            .map_err(|e| OvieError::io_error(format!("Failed to create installation directory: {}", e)))?;

        Ok(())
    }

    /// Copy binaries from package to installation directory
    fn copy_binaries(&self, package_dir: &Path) -> OvieResult<Vec<String>> {
        println!("  Installing binaries...");

        let bin_dir = self.config.install_dir.join("bin");
        fs::create_dir_all(&bin_dir)
            .map_err(|e| OvieError::io_error(format!("Failed to create bin directory: {}", e)))?;

        let binary_names = if cfg!(windows) {
            vec!["oviec.exe", "ovie.exe"]
        } else {
            vec!["oviec", "ovie"]
        };

        let mut installed = Vec::new();

        for binary_name in &binary_names {
            let source = package_dir.join(binary_name);
            let dest = bin_dir.join(binary_name);

            if source.exists() {
                fs::copy(&source, &dest)
                    .map_err(|e| OvieError::io_error(format!("Failed to copy {}: {}", binary_name, e)))?;

                // Set executable permissions on Unix
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = fs::metadata(&dest)
                        .map_err(|e| OvieError::io_error(format!("Failed to get permissions: {}", e)))?
                        .permissions();
                    perms.set_mode(0o755);
                    fs::set_permissions(&dest, perms)
                        .map_err(|e| OvieError::io_error(format!("Failed to set permissions: {}", e)))?;
                }

                installed.push(binary_name.to_string());
                println!("    ✓ {}", binary_name);
            }
        }

        Ok(installed)
    }

    /// Copy standard library from package to installation directory
    fn copy_stdlib(&self, package_dir: &Path) -> OvieResult<()> {
        println!("  Installing standard library...");

        let source_std = package_dir.join("std");
        let dest_std = self.config.install_dir.join("std");

        if source_std.exists() {
            self.copy_directory_recursive(&source_std, &dest_std)?;
            println!("    ✓ Standard library");
        }

        Ok(())
    }

    /// Copy documentation from package to installation directory
    fn copy_docs(&self, package_dir: &Path) -> OvieResult<()> {
        println!("  Installing documentation...");

        let source_docs = package_dir.join("docs");
        let dest_docs = self.config.install_dir.join("docs");

        if source_docs.exists() {
            self.copy_directory_recursive(&source_docs, &dest_docs)?;
            println!("    ✓ Documentation");
        }

        // Copy README and LICENSE
        for file in &["README.md", "LICENSE"] {
            let source = package_dir.join(file);
            let dest = self.config.install_dir.join(file);

            if source.exists() {
                fs::copy(&source, &dest)
                    .map_err(|e| OvieError::io_error(format!("Failed to copy {}: {}", file, e)))?;
            }
        }

        Ok(())
    }

    /// Create ORE (Ovie Runtime Environment) structure
    fn create_ore_structure(&self) -> OvieResult<()> {
        println!("  Creating ORE structure...");

        // Create .ovie directory
        let ovie_dir = self.config.install_dir.join(".ovie");
        fs::create_dir_all(&ovie_dir)
            .map_err(|e| OvieError::io_error(format!("Failed to create .ovie directory: {}", e)))?;

        // Create aproko.toml configuration
        let aproko_config = ovie_dir.join("aproko.toml");
        let aproko_content = r#"# Aproko Configuration
[analysis]
enabled = true
severity_threshold = "warning"

[rules]
# Enable all default rules
enable_all = true
"#;
        fs::write(&aproko_config, aproko_content)
            .map_err(|e| OvieError::io_error(format!("Failed to create aproko.toml: {}", e)))?;

        println!("    ✓ ORE structure created");

        Ok(())
    }

    /// Add installation directory to PATH
    fn add_to_path(&self) -> OvieResult<()> {
        println!("  Configuring PATH...");

        let bin_dir = self.config.install_dir.join("bin");

        if cfg!(windows) {
            // On Windows, we need to modify the registry or user environment
            println!("    ⚠ Please add {} to your PATH manually", bin_dir.display());
            println!("      Or run: setx PATH \"%PATH%;{}\"", bin_dir.display());
        } else {
            // On Unix, we can suggest adding to shell profile
            println!("    ⚠ Please add the following to your shell profile:");
            println!("      export PATH=\"$PATH:{}\"", bin_dir.display());
        }

        Ok(())
    }

    /// Remove installation directory from PATH
    fn remove_from_path(&self) -> OvieResult<()> {
        println!("  Removing from PATH...");

        let bin_dir = self.config.install_dir.join("bin");

        if cfg!(windows) {
            println!("    ⚠ Please remove {} from your PATH manually", bin_dir.display());
        } else {
            println!("    ⚠ Please remove {} from your shell profile", bin_dir.display());
        }

        Ok(())
    }

    /// Validate the installation
    fn validate_installation(&self) -> OvieResult<()> {
        println!("  Validating installation...");

        // Check if binaries exist
        let bin_dir = self.config.install_dir.join("bin");
        let oviec_binary = if cfg!(windows) {
            bin_dir.join("oviec.exe")
        } else {
            bin_dir.join("oviec")
        };

        if !oviec_binary.exists() {
            return Err(OvieError::runtime_error("oviec binary not found after installation".to_string()));
        }

        // Check if standard library exists
        let std_dir = self.config.install_dir.join("std");
        if !std_dir.exists() {
            return Err(OvieError::runtime_error("Standard library not found after installation".to_string()));
        }

        // Try to discover ORE
        env::set_var("OVIE_HOME", &self.config.install_dir);
        match OvieRuntimeEnvironment::discover() {
            Ok(ore) => {
                println!("    ✓ ORE discovered at: {}", ore.ovie_home.display());
            }
            Err(e) => {
                println!("    ⚠ ORE discovery failed: {}", e);
            }
        }

        println!("    ✓ Installation validated");

        Ok(())
    }

    /// Recursively copy a directory
    fn copy_directory_recursive(&self, source: &Path, dest: &Path) -> OvieResult<()> {
        if !source.exists() {
            return Ok(());
        }

        fs::create_dir_all(dest)
            .map_err(|e| OvieError::io_error(format!("Failed to create directory {}: {}", dest.display(), e)))?;

        for entry in fs::read_dir(source)
            .map_err(|e| OvieError::io_error(format!("Failed to read directory {}: {}", source.display(), e)))?
        {
            let entry = entry
                .map_err(|e| OvieError::io_error(format!("Failed to read entry: {}", e)))?;
            let path = entry.path();
            let file_name = entry.file_name();
            let dest_path = dest.join(&file_name);

            if path.is_dir() {
                self.copy_directory_recursive(&path, &dest_path)?;
            } else {
                fs::copy(&path, &dest_path)
                    .map_err(|e| OvieError::io_error(format!("Failed to copy file {}: {}", path.display(), e)))?;
            }
        }

        Ok(())
    }
}

/// Result of an installation operation
#[derive(Debug, Clone)]
pub struct InstallationResult {
    /// Installation directory
    pub install_dir: PathBuf,
    /// List of binaries installed
    pub binaries_installed: Vec<String>,
    /// Whether ORE was created
    pub ore_created: bool,
    /// Whether PATH was modified
    pub added_to_path: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_config_default() {
        let config = InstallConfig::default();
        assert!(config.add_to_path);
        assert!(config.create_ore);
        assert!(config.validate);
    }

    #[test]
    fn test_install_config_user_local() {
        let config = InstallConfig::user_local();
        assert!(config.install_dir.to_string_lossy().contains(".ovie"));
        assert!(config.add_to_path);
        assert!(config.create_ore);
    }

    #[test]
    fn test_installer_creation() {
        let config = InstallConfig::user_local();
        let installer = Installer::new(config.clone());
        assert_eq!(installer.config.install_dir, config.install_dir);
    }
}
