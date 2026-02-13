// Release Manager Tests
// Tests for the complete release management system

use oviec::release::{ReleaseManager, SecurityLevel};

#[test]
fn test_release_manager_creation() {
    let manager = ReleaseManager::new(SecurityLevel::Development);
    assert!(manager.is_ok());
    
    let manager = manager.unwrap();
    assert_eq!(manager.security_level(), SecurityLevel::Development);
}

#[test]
fn test_security_levels() {
    assert_eq!(SecurityLevel::Development.required_signatures(), 1);
    assert_eq!(SecurityLevel::Beta.required_signatures(), 2);
    assert_eq!(SecurityLevel::Production.required_signatures(), 3);
    
    assert_eq!(SecurityLevel::Development.required_key_strength(), 2048);
    assert_eq!(SecurityLevel::Beta.required_key_strength(), 3072);
    assert_eq!(SecurityLevel::Production.required_key_strength(), 4096);
}

#[test]
fn test_release_report_generation() {
    let manager = ReleaseManager::new(SecurityLevel::Production).unwrap();
    let report = manager.generate_release_report();
    
    assert!(report.contains("Ovie Release System Status"));
    assert!(report.contains("Production"));
    assert!(report.contains("Cryptographic Signing"));
    assert!(report.contains("Verification System"));
    assert!(report.contains("Distribution System"));
}

#[test]
fn test_all_security_levels_can_create_manager() {
    let levels = vec![
        SecurityLevel::Development,
        SecurityLevel::Beta,
        SecurityLevel::Production,
    ];
    
    for level in levels {
        let manager = ReleaseManager::new(level);
        assert!(manager.is_ok(), "Failed to create manager for {:?}", level);
    }
}
