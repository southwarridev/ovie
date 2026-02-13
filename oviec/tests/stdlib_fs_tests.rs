//! Integration tests for the std::fs module implementation
//! 
//! These tests verify that the fs module works correctly and follows
//! the offline-first design principles.

use oviec::stdlib::fs::{
    write_string, read_to_string, exists, is_file, is_dir,
    create_dir, remove_dir, create, open_with_mode, OvieFileMode,
    is_network_path, normalize_path
};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_basic_file_operations() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt").to_string_lossy().to_string();
    
    // Test write and read
    let content = "Hello, Ovie filesystem!".to_string();
    assert!(write_string(file_path.clone(), content.clone()).is_ok());
    
    let read_result = read_to_string(file_path.clone());
    assert!(read_result.is_ok());
    assert_eq!(read_result.unwrap(), content);
    
    // Test existence checks
    assert!(exists(file_path.clone()));
    assert!(is_file(file_path.clone()));
    assert!(!is_dir(file_path.clone()));
}

#[test]
fn test_directory_operations() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().join("test_dir").to_string_lossy().to_string();
    
    // Test directory creation
    assert!(create_dir(dir_path.clone()).is_ok());
    assert!(exists(dir_path.clone()));
    assert!(is_dir(dir_path.clone()));
    assert!(!is_file(dir_path.clone()));
    
    // Test directory removal
    assert!(remove_dir(dir_path.clone()).is_ok());
    assert!(!exists(dir_path));
}

#[test]
fn test_ovie_file_operations() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("ovie_test.txt").to_string_lossy().to_string();
    
    // Test file creation and writing
    let mut file = create(file_path.clone()).unwrap();
    assert!(file.write_string("Test content".to_string()).is_ok());
    assert!(file.flush().is_ok());
    assert!(file.close().is_ok());
    
    // Test file reading
    let mut file = open_with_mode(file_path.clone(), OvieFileMode::Read).unwrap();
    let content = file.read_to_string().unwrap();
    assert_eq!(content, "Test content");
    assert!(file.close().is_ok());
}

#[test]
fn test_security_validation() {
    // Test network path rejection
    assert!(is_network_path("http://example.com/file.txt"));
    assert!(is_network_path("https://example.com/file.txt"));
    assert!(is_network_path("ftp://example.com/file.txt"));
    assert!(is_network_path("\\\\server\\share\\file.txt"));
    assert!(!is_network_path("/local/file.txt"));
    assert!(!is_network_path("C:\\local\\file.txt"));
    
    // Test path normalization
    assert!(normalize_path("../etc/passwd").is_err());
    assert!(normalize_path("~/secret").is_err());
    assert!(normalize_path("").is_err());
    assert!(normalize_path("/valid/path").is_ok());
    
    // Test that network paths are rejected in file operations
    assert!(write_string("http://example.com/file.txt".to_string(), "test".to_string()).is_err());
    assert!(write_string("../../../etc/passwd".to_string(), "test".to_string()).is_err());
}

#[test]
fn test_error_handling() {
    // Test reading non-existent file
    assert!(read_to_string("/non/existent/file.txt".to_string()).is_err());
    
    // Test creating directory that already exists
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_string_lossy().to_string();
    assert!(create_dir(dir_path).is_err()); // Should fail because temp_dir already exists
}

#[test]
fn test_cross_platform_paths() {
    // Test that path normalization works consistently
    let normalized = normalize_path("some/path/file.txt").unwrap();
    assert!(normalized.contains("some"));
    assert!(normalized.contains("path"));
    assert!(normalized.contains("file.txt"));
    
    // Test that duplicate slashes are removed
    let normalized = normalize_path("some//path///file.txt").unwrap();
    assert!(!normalized.contains("//"));
    assert!(!normalized.contains("///"));
}

#[test]
fn test_deterministic_behavior() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("deterministic.txt").to_string_lossy().to_string();
    let content = "Deterministic content".to_string();
    
    // Write and read multiple times - should always produce same result
    for _ in 0..5 {
        assert!(write_string(file_path.clone(), content.clone()).is_ok());
        let read_content = read_to_string(file_path.clone()).unwrap();
        assert_eq!(read_content, content);
        assert!(exists(file_path.clone()));
        assert!(is_file(file_path.clone()));
    }
}