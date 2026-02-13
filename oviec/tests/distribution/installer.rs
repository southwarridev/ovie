// Installer Tests
// Tests for installation system

use oviec::release::installer::{Installer, InstallConfig};
use std::path::PathBuf;

#[test]
fn test_default_install_config() {
    let config = InstallConfig::default();
    
    assert!(config.add_to_path);
    assert!(config.create_ore);
    assert!(config.validate);
    assert!(!config.install_dir.as_os_str().is_empty());
    assert!(config.install_dir.is_absolute());
}

#[test]
fn test_user_local_install_config() {
    let config = InstallConfig::user_local();
    
    assert!(config.install_dir.to_string_lossy().contains(".ovie"));
    assert!(config.add_to_path);
    assert!(config.create_ore);
    assert!(config.validate);
}

#[test]
fn test_installer_with_custom_config() {
    let mut config = InstallConfig::default();
    config.install_dir = PathBuf::from("/custom/install/path");
    config.add_to_path = false;
    config.create_ore = false;
    
    let installer = Installer::new(config.clone());
    assert_eq!(installer.config.install_dir, PathBuf::from("/custom/install/path"));
    assert!(!installer.config.add_to_path);
    assert!(!installer.config.create_ore);
}

#[test]
fn test_install_dir_contains_home_for_user_local() {
    let config = InstallConfig::user_local();
    
    if let Some(home) = dirs::home_dir() {
        assert!(config.install_dir.starts_with(home));
    }
}

#[test]
fn test_platform_specific_install_dirs() {
    let config = InstallConfig::default();
    
    if cfg!(windows) {
        assert!(config.install_dir.to_string_lossy().contains("Program Files") ||
                config.install_dir.to_string_lossy().contains("Ovie"));
    } else {
        assert!(config.install_dir.to_string_lossy().contains("/usr/local") ||
                config.install_dir.to_string_lossy().contains("ovie"));
    }
}
