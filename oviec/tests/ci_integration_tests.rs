//! CI Integration Tests
//! 
//! These tests verify that the CI pipeline is correctly configured and can run
//! bootstrap verification successfully.

use std::path::PathBuf;
use std::fs;

/// Test that the bootstrap verification workflow exists
#[test]
fn test_bootstrap_workflow_exists() {
    // Tests run from the workspace root
    let cwd = std::env::current_dir().unwrap();
    eprintln!("Current directory: {:?}", cwd);
    
    // Try multiple possible paths
    let possible_paths = vec![
        "../.github/workflows/bootstrap-verification.yml",
        "../../.github/workflows/bootstrap-verification.yml",
        "../../../.github/workflows/bootstrap-verification.yml",
    ];
    
    let mut workflow_path = None;
    for path in &possible_paths {
        let p = PathBuf::from(path);
        eprintln!("Trying path: {:?} - exists: {}", p, p.exists());
        if p.exists() {
            workflow_path = Some(p);
            break;
        }
    }
    
    let workflow_path = workflow_path.expect("Bootstrap verification workflow should exist");
    let metadata = fs::metadata(&workflow_path).expect("Should be able to read workflow file");
    assert!(metadata.is_file(), "Workflow should be a file");
}

/// Test that the workflow file is valid YAML
#[test]
fn test_bootstrap_workflow_valid_yaml() {
    let workflow_path = PathBuf::from("../.github/workflows/bootstrap-verification.yml");
    let content = fs::read_to_string(&workflow_path).unwrap_or_else(|e| {
        panic!("Failed to read workflow file: {:?}. Error: {}", workflow_path, e);
    });
    
    eprintln!("Content length: {}", content.len());
    eprintln!("First 100 chars: {:?}", &content.chars().take(100).collect::<String>());
    
    // Basic YAML validation - check for required keys
    assert!(content.contains("name:"), "Workflow should have a name. Content length: {}", content.len());
    assert!(content.contains("on:"), "Workflow should have triggers");
    assert!(content.contains("jobs:"), "Workflow should have jobs");
}

/// Test that workflow includes all required jobs
#[test]
fn test_bootstrap_workflow_has_required_jobs() {
    let workflow_path = PathBuf::from("../.github/workflows/bootstrap-verification.yml");
    let content = fs::read_to_string(&workflow_path).expect("Should be able to read workflow");
    
    // Check for required jobs
    assert!(content.contains("bootstrap-verification:"), "Should have bootstrap-verification job");
    assert!(content.contains("performance-monitoring:"), "Should have performance-monitoring job");
    assert!(content.contains("ci-integration-tests:"), "Should have ci-integration-tests job");
    assert!(content.contains("summary:"), "Should have summary job");
}

/// Test that workflow runs on multiple platforms
#[test]
fn test_bootstrap_workflow_multi_platform() {
    let workflow_path = PathBuf::from("../.github/workflows/bootstrap-verification.yml");
    let content = fs::read_to_string(&workflow_path).expect("Should be able to read workflow");
    
    // Check for platform matrix
    assert!(content.contains("matrix:"), "Should have platform matrix");
    assert!(content.contains("ubuntu-latest"), "Should test on Ubuntu");
    assert!(content.contains("windows-latest"), "Should test on Windows");
    assert!(content.contains("macos-latest"), "Should test on macOS");
}

/// Test that workflow uses bootstrap scripts
#[test]
fn test_bootstrap_workflow_uses_scripts() {
    let workflow_path = PathBuf::from("../.github/workflows/bootstrap-verification.yml");
    let content = fs::read_to_string(&workflow_path).expect("Should be able to read workflow");
    
    // Check that workflow references bootstrap scripts
    assert!(content.contains("bootstrap_verify.sh"), "Should use bash script");
    assert!(content.contains("bootstrap_verify.ps1"), "Should use PowerShell script");
}

/// Test that workflow uploads artifacts
#[test]
fn test_bootstrap_workflow_uploads_artifacts() {
    let workflow_path = PathBuf::from("../.github/workflows/bootstrap-verification.yml");
    let content = fs::read_to_string(&workflow_path).expect("Should be able to read workflow");
    
    // Check for artifact uploads
    assert!(content.contains("upload-artifact"), "Should upload artifacts");
    assert!(content.contains("bootstrap-report"), "Should upload bootstrap reports");
    assert!(content.contains("bootstrap-artifacts"), "Should upload bootstrap artifacts");
}

/// Test that workflow has performance monitoring
#[test]
fn test_bootstrap_workflow_has_performance_monitoring() {
    let workflow_path = PathBuf::from("../.github/workflows/bootstrap-verification.yml");
    let content = fs::read_to_string(&workflow_path).expect("Should be able to read workflow");
    
    // Check for performance monitoring
    assert!(content.contains("performance-monitoring"), "Should have performance monitoring job");
    assert!(content.contains("Performance"), "Should mention performance");
}

/// Test that workflow has failure handling
#[test]
fn test_bootstrap_workflow_has_failure_handling() {
    let workflow_path = PathBuf::from("../.github/workflows/bootstrap-verification.yml");
    let content = fs::read_to_string(&workflow_path).expect("Should be able to read workflow");
    
    // Check for failure handling
    assert!(content.contains("continue-on-error") || content.contains("if:"), "Should have failure handling");
    assert!(content.contains("always()"), "Should run cleanup steps always");
}

/// Test that workflow caches dependencies
#[test]
fn test_bootstrap_workflow_caches_dependencies() {
    let workflow_path = PathBuf::from("../.github/workflows/bootstrap-verification.yml");
    let content = fs::read_to_string(&workflow_path).expect("Should be able to read workflow");
    
    // Check for caching
    assert!(content.contains("cache"), "Should cache dependencies");
    assert!(content.contains("cargo"), "Should cache Cargo dependencies");
}

/// Test that workflow has manual trigger
#[test]
fn test_bootstrap_workflow_has_manual_trigger() {
    let workflow_path = PathBuf::from("../.github/workflows/bootstrap-verification.yml");
    let content = fs::read_to_string(&workflow_path).expect("Should be able to read workflow");
    
    // Check for manual trigger (workflow_dispatch)
    assert!(content.contains("workflow_dispatch"), "Should have manual trigger");
}

/// Test that workflow builds the compiler
#[test]
fn test_bootstrap_workflow_builds_compiler() {
    let workflow_path = PathBuf::from("../.github/workflows/bootstrap-verification.yml");
    let content = fs::read_to_string(&workflow_path).expect("Should be able to read workflow");
    
    // Check that workflow builds the compiler
    assert!(content.contains("cargo build"), "Should build with cargo");
    assert!(content.contains("--release"), "Should build in release mode");
}

/// Test that workflow generates summary
#[test]
fn test_bootstrap_workflow_generates_summary() {
    let workflow_path = PathBuf::from("../.github/workflows/bootstrap-verification.yml");
    let content = fs::read_to_string(&workflow_path).expect("Should be able to read workflow");
    
    // Check for summary generation
    assert!(content.contains("GITHUB_STEP_SUMMARY"), "Should generate GitHub summary");
    assert!(content.contains("summary:"), "Should have summary job");
}

/// Test that workflow has environment variables
#[test]
fn test_bootstrap_workflow_has_env_vars() {
    let workflow_path = PathBuf::from("../.github/workflows/bootstrap-verification.yml");
    let content = fs::read_to_string(&workflow_path).expect("Should be able to read workflow");
    
    // Check for environment variables
    assert!(content.contains("env:"), "Should have environment variables");
    assert!(content.contains("RUST_BACKTRACE"), "Should set RUST_BACKTRACE");
}

/// Test that workflow uses Rust toolchain action
#[test]
fn test_bootstrap_workflow_uses_rust_toolchain() {
    let workflow_path = PathBuf::from("../.github/workflows/bootstrap-verification.yml");
    let content = fs::read_to_string(&workflow_path).expect("Should be able to read workflow");
    
    // Check for Rust toolchain setup
    assert!(content.contains("setup-rust-toolchain") || content.contains("rust-toolchain"), 
        "Should setup Rust toolchain");
}

/// Test that workflow has retention policy for artifacts
#[test]
fn test_bootstrap_workflow_has_retention_policy() {
    let workflow_path = PathBuf::from("../.github/workflows/bootstrap-verification.yml");
    let content = fs::read_to_string(&workflow_path).expect("Should be able to read workflow");
    
    // Check for retention policy
    assert!(content.contains("retention-days"), "Should have retention policy for artifacts");
}

/// Test that workflow checks verification results
#[test]
fn test_bootstrap_workflow_checks_results() {
    let workflow_path = PathBuf::from("../.github/workflows/bootstrap-verification.yml");
    let content = fs::read_to_string(&workflow_path).expect("Should be able to read workflow");
    
    // Check that workflow validates results
    assert!(content.contains("result"), "Should check job results");
    assert!(content.contains("success") || content.contains("failure"), "Should handle success/failure");
}

/// Test that workflow has proper job dependencies
#[test]
fn test_bootstrap_workflow_has_job_dependencies() {
    let workflow_path = PathBuf::from("../.github/workflows/bootstrap-verification.yml");
    let content = fs::read_to_string(&workflow_path).expect("Should be able to read workflow");
    
    // Check for job dependencies
    assert!(content.contains("needs:"), "Should have job dependencies");
}

/// Test that workflow downloads artifacts for analysis
#[test]
fn test_bootstrap_workflow_downloads_artifacts() {
    let workflow_path = PathBuf::from("../.github/workflows/bootstrap-verification.yml");
    let content = fs::read_to_string(&workflow_path).expect("Should be able to read workflow");
    
    // Check for artifact downloads
    assert!(content.contains("download-artifact"), "Should download artifacts for analysis");
}

/// Test that workflow has descriptive job names
#[test]
fn test_bootstrap_workflow_has_descriptive_names() {
    let workflow_path = PathBuf::from("../.github/workflows/bootstrap-verification.yml");
    let content = fs::read_to_string(&workflow_path).expect("Should be able to read workflow");
    
    // Check for descriptive names
    assert!(content.contains("name:"), "Should have job names");
    assert!(content.contains("Bootstrap"), "Should mention Bootstrap");
    assert!(content.contains("Verification"), "Should mention Verification");
}

/// Test that workflow is documented
#[test]
fn test_bootstrap_workflow_is_documented() {
    let workflow_path = PathBuf::from("../.github/workflows/bootstrap-verification.yml");
    let content = fs::read_to_string(&workflow_path).expect("Should be able to read workflow");
    
    // Check for documentation comments
    assert!(content.contains("#"), "Should have comments");
    assert!(content.contains("CURRENT STATUS") || content.contains("This workflow"), 
        "Should have status documentation");
}
