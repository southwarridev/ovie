// Distribution Builder Tests
// Tests for Task 13.2: Create platform-specific packages

use oviec::release::builder::{DistributionBuilder, Platform, PackageStructure};
use std::env;

#[test]
fn test_package_structure_for_all_platforms() {
    let platforms = vec![
        Platform::WindowsX64,
        Platform::LinuxX64,
        Platform::MacOSArm64,
        Platform::MacOSX64,
    ];
    
    for platform in platforms {
        let structure = PackageStructure::new("2.2.0", &platform);
        
        // Verify root directory name
        assert_eq!(structure.root_dir, format!("ovie-2.2.0-{}", platform.name()));
        
        // Verify binaries have correct extensions
        for binary in &structure.binaries {
            if platform == Platform::WindowsX64 {
                assert!(binary.ends_with(".exe"), "Windows binary should have .exe extension");
            } else {
                assert!(!binary.ends_with(".exe"), "Unix binary should not have .exe extension");
            }
        }
        
        // Verify install scripts are platform-specific
        for script in &structure.install_scripts {
            if platform == Platform::WindowsX64 {
                assert!(script.ends_with(".bat"), "Windows should use .bat install script");
            } else {
                assert!(script.ends_with(".sh"), "Unix should use .sh install script");
            }
        }
    }
}

#[test]
fn test_distribution_builder_creation() {
    let workspace_root = env::current_dir().unwrap();
    let output_dir = workspace_root.join("target").join("test-dist");
    
    let _builder = DistributionBuilder::new(
        "2.2.0".to_string(),
        Platform::LinuxX64,
        workspace_root.clone(),
        output_dir.clone(),
    );
    
    // Builder created successfully - internal fields are private implementation details
    // The builder's functionality is tested through its public methods
}

#[test]
fn test_platform_archive_extensions() {
    assert_eq!(Platform::WindowsX64.archive_extension(), ".zip");
    assert_eq!(Platform::LinuxX64.archive_extension(), ".tar.gz");
    assert_eq!(Platform::MacOSArm64.archive_extension(), ".tar.gz");
    assert_eq!(Platform::MacOSX64.archive_extension(), ".tar.gz");
}

#[test]
fn test_platform_binary_extensions() {
    assert_eq!(Platform::WindowsX64.binary_extension(), ".exe");
    assert_eq!(Platform::LinuxX64.binary_extension(), "");
    assert_eq!(Platform::MacOSArm64.binary_extension(), "");
    assert_eq!(Platform::MacOSX64.binary_extension(), "");
}

#[test]
fn test_platform_names() {
    assert_eq!(Platform::WindowsX64.name(), "windows-x64");
    assert_eq!(Platform::LinuxX64.name(), "linux-x64");
    assert_eq!(Platform::MacOSArm64.name(), "macos-arm64");
    assert_eq!(Platform::MacOSX64.name(), "macos-x64");
}
