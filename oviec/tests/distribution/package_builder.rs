// Package Builder Tests
// Tests for distribution package creation

use oviec::release::builder::{DistributionBuilder, Platform, PackageStructure};
use std::env;

#[test]
fn test_all_platforms_have_correct_structure() {
    let platforms = vec![
        Platform::WindowsX64,
        Platform::LinuxX64,
        Platform::MacOSArm64,
        Platform::MacOSX64,
    ];
    
    for platform in platforms {
        let structure = PackageStructure::new("2.2.0", &platform);
        
        // Verify root directory naming
        assert!(structure.root_dir.starts_with("ovie-2.2.0-"));
        assert!(structure.root_dir.contains(platform.name()));
        
        // Verify binaries exist
        assert!(!structure.binaries.is_empty());
        
        // Verify docs exist
        assert!(!structure.docs.is_empty());
        
        // Verify stdlib exists
        assert!(!structure.stdlib.is_empty());
        
        // Verify legal files exist
        assert!(!structure.legal.is_empty());
        
        // Verify install scripts exist
        assert!(!structure.install_scripts.is_empty());
    }
}

#[test]
fn test_windows_package_has_exe_extension() {
    let structure = PackageStructure::new("2.2.0", &Platform::WindowsX64);
    
    for binary in &structure.binaries {
        assert!(binary.ends_with(".exe"), "Windows binary {} should have .exe extension", binary);
    }
    
    for script in &structure.install_scripts {
        assert!(script.ends_with(".bat"), "Windows install script {} should have .bat extension", script);
    }
}

#[test]
fn test_unix_packages_have_no_extension() {
    let platforms = vec![Platform::LinuxX64, Platform::MacOSArm64, Platform::MacOSX64];
    
    for platform in platforms {
        let structure = PackageStructure::new("2.2.0", &platform);
        
        for binary in &structure.binaries {
            assert!(!binary.ends_with(".exe"), "Unix binary {} should not have .exe extension", binary);
        }
        
        for script in &structure.install_scripts {
            assert!(script.ends_with(".sh"), "Unix install script {} should have .sh extension", script);
        }
    }
}

#[test]
fn test_distribution_builder_initialization() {
    let workspace_root = env::current_dir().unwrap();
    let output_dir = workspace_root.join("target").join("test-dist");
    
    let builder = DistributionBuilder::new(
        "2.2.0".to_string(),
        Platform::LinuxX64,
        workspace_root.clone(),
        output_dir.clone(),
    );
    
    assert_eq!(builder.version, "2.2.0");
    assert_eq!(builder.platform, Platform::LinuxX64);
}

#[test]
fn test_archive_extensions_are_correct() {
    assert_eq!(Platform::WindowsX64.archive_extension(), ".zip");
    assert_eq!(Platform::LinuxX64.archive_extension(), ".tar.gz");
    assert_eq!(Platform::MacOSArm64.archive_extension(), ".tar.gz");
    assert_eq!(Platform::MacOSX64.archive_extension(), ".tar.gz");
}
