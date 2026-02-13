// Installation System Tests
// Tests for Task 13.3: Installation system

use oviec::release::installer::{Installer, InstallConfig};
use std::path::PathBuf;

#[test]
fn test_install_config_creation() {
    let config = InstallConfig::default();
    assert!(config.add_to_path);
    assert!(config.create_ore);
    assert!(config.validate);
    assert!(!config.install_dir.as_os_str().is_empty());
}

#[test]
fn test_user_local_install_config() {
    let config = InstallConfig::user_local();
    
    // Should contain .ovie in the path
    assert!(config.install_dir.to_string_lossy().contains(".ovie"));
    assert!(config.add_to_path);
    assert!(config.create_ore);
    assert!(config.validate);
}

#[test]
fn test_installer_creation() {
    let config = InstallConfig::user_local();
    let installer = Installer::new(config.clone());
    
    // Verify installer was created with correct config
    assert_eq!(installer.config.install_dir, config.install_dir);
}

#[test]
fn test_default_install_dir_not_empty() {
    let config = InstallConfig::default();
    assert!(!config.install_dir.as_os_str().is_empty());
    
    // Should be an absolute path
    assert!(config.install_dir.is_absolute());
}

#[test]
fn test_user_local_install_dir_contains_home() {
    let config = InstallConfig::user_local();
    
    // User local should be in home directory
    if let Some(home) = dirs::home_dir() {
        assert!(config.install_dir.starts_with(home));
    }
}

#[test]
fn test_install_config_customization() {
    let mut config = InstallConfig::default();
    
    // Customize configuration
    config.add_to_path = false;
    config.create_ore = false;
    config.validate = false;
    config.install_dir = PathBuf::from("/custom/path");
    
    assert!(!config.add_to_path);
    assert!(!config.create_ore);
    assert!(!config.validate);
    assert_eq!(config.install_dir, PathBuf::from("/custom/path"));
}
