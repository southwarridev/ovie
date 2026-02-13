//! Integration tests for bootstrap verification scripts
//! 
//! These tests verify that the bootstrap_verify.sh and bootstrap_verify.ps1 scripts
//! work correctly with the Rust bootstrap verification infrastructure.

use std::process::Command;
use std::path::PathBuf;
use std::fs;

/// Test that bootstrap_verify.sh exists and is executable
#[test]
fn test_bootstrap_verify_sh_exists() {
    let script_path = PathBuf::from("scripts/bootstrap_verify.sh");
    assert!(script_path.exists(), "bootstrap_verify.sh should exist");
    
    // Check if file is readable
    let metadata = fs::metadata(&script_path).expect("Should be able to read script metadata");
    assert!(metadata.is_file(), "bootstrap_verify.sh should be a file");
}

/// Test that bootstrap_verify.ps1 exists
#[test]
fn test_bootstrap_verify_ps1_exists() {
    let script_path = PathBuf::from("scripts/bootstrap_verify.ps1");
    assert!(script_path.exists(), "bootstrap_verify.ps1 should exist");
    
    // Check if file is readable
    let metadata = fs::metadata(&script_path).expect("Should be able to read script metadata");
    assert!(metadata.is_file(), "bootstrap_verify.ps1 should be a file");
}

/// Test that bootstrap_verify.sh has correct shebang
#[test]
fn test_bootstrap_verify_sh_shebang() {
    let script_path = PathBuf::from("scripts/bootstrap_verify.sh");
    let content = fs::read_to_string(&script_path).expect("Should be able to read script");
    
    assert!(
        content.starts_with("#!/usr/bin/env bash") || content.starts_with("#!/bin/bash"),
        "Script should have correct bash shebang"
    );
}

/// Test that scripts contain required sections
#[test]
fn test_bootstrap_verify_sh_structure() {
    let script_path = PathBuf::from("scripts/bootstrap_verify.sh");
    let content = fs::read_to_string(&script_path).expect("Should be able to read script");
    
    // Check for required sections
    assert!(content.contains("Building Stage 0 Compiler"), "Should have Stage 0 build section");
    assert!(content.contains("Building Stage 1 Compiler"), "Should have Stage 1 build section");
    assert!(content.contains("Version Equivalence Test"), "Should have version test section");
    assert!(content.contains("Compilation Equivalence Test"), "Should have compilation test section");
    assert!(content.contains("Runtime Equivalence Test"), "Should have runtime test section");
    assert!(content.contains("Self-Check Diagnostics"), "Should have self-check section");
}

/// Test that PowerShell script contains required sections
#[test]
fn test_bootstrap_verify_ps1_structure() {
    let script_path = PathBuf::from("scripts/bootstrap_verify.ps1");
    let content = fs::read_to_string(&script_path).expect("Should be able to read script");
    
    // Check for required sections
    assert!(content.contains("Building Stage 0 Compiler"), "Should have Stage 0 build section");
    assert!(content.contains("Building Stage 1 Compiler"), "Should have Stage 1 build section");
    assert!(content.contains("Version Equivalence Test"), "Should have version test section");
    assert!(content.contains("Compilation Equivalence Test"), "Should have compilation test section");
    assert!(content.contains("Runtime Equivalence Test"), "Should have runtime test section");
    assert!(content.contains("Self-Check Diagnostics"), "Should have self-check section");
}

/// Test that scripts have cleanup functions
#[test]
fn test_scripts_have_cleanup() {
    let sh_content = fs::read_to_string("scripts/bootstrap_verify.sh")
        .expect("Should be able to read bash script");
    let ps1_content = fs::read_to_string("scripts/bootstrap_verify.ps1")
        .expect("Should be able to read PowerShell script");
    
    assert!(sh_content.contains("cleanup()"), "Bash script should have cleanup function");
    assert!(ps1_content.contains("function Cleanup"), "PowerShell script should have Cleanup function");
}

/// Test that scripts check for Rust/Cargo
#[test]
fn test_scripts_check_for_rust() {
    let sh_content = fs::read_to_string("scripts/bootstrap_verify.sh")
        .expect("Should be able to read bash script");
    let ps1_content = fs::read_to_string("scripts/bootstrap_verify.ps1")
        .expect("Should be able to read PowerShell script");
    
    assert!(sh_content.contains("cargo"), "Bash script should check for cargo");
    assert!(ps1_content.contains("cargo"), "PowerShell script should check for cargo");
}

/// Test that scripts build with cargo
#[test]
fn test_scripts_use_cargo_build() {
    let sh_content = fs::read_to_string("scripts/bootstrap_verify.sh")
        .expect("Should be able to read bash script");
    let ps1_content = fs::read_to_string("scripts/bootstrap_verify.ps1")
        .expect("Should be able to read PowerShell script");
    
    assert!(sh_content.contains("cargo build --release"), "Bash script should use cargo build");
    assert!(ps1_content.contains("cargo build --release"), "PowerShell script should use cargo build");
}

/// Test that scripts create test directories
#[test]
fn test_scripts_create_test_directories() {
    let sh_content = fs::read_to_string("scripts/bootstrap_verify.sh")
        .expect("Should be able to read bash script");
    let ps1_content = fs::read_to_string("scripts/bootstrap_verify.ps1")
        .expect("Should be able to read PowerShell script");
    
    assert!(sh_content.contains("TEST_DIR") || sh_content.contains("bootstrap_test"), 
        "Bash script should create test directory");
    assert!(ps1_content.contains("TestDir") || ps1_content.contains("bootstrap_test"), 
        "PowerShell script should create test directory");
}

/// Test that scripts handle errors properly
#[test]
fn test_scripts_have_error_handling() {
    let sh_content = fs::read_to_string("scripts/bootstrap_verify.sh")
        .expect("Should be able to read bash script");
    let ps1_content = fs::read_to_string("scripts/bootstrap_verify.ps1")
        .expect("Should be able to read PowerShell script");
    
    assert!(sh_content.contains("set -e") || sh_content.contains("exit 1"), 
        "Bash script should have error handling");
    assert!(ps1_content.contains("ErrorActionPreference") || ps1_content.contains("exit 1"), 
        "PowerShell script should have error handling");
}

/// Test that scripts output colored messages
#[test]
fn test_scripts_have_colored_output() {
    let sh_content = fs::read_to_string("scripts/bootstrap_verify.sh")
        .expect("Should be able to read bash script");
    let ps1_content = fs::read_to_string("scripts/bootstrap_verify.ps1")
        .expect("Should be able to read PowerShell script");
    
    // Bash uses ANSI color codes
    assert!(sh_content.contains("\\033[") || sh_content.contains("RED=") || sh_content.contains("GREEN="), 
        "Bash script should have colored output");
    
    // PowerShell uses Write-Host with -ForegroundColor
    assert!(ps1_content.contains("ForegroundColor") || ps1_content.contains("Write-ColorOutput"), 
        "PowerShell script should have colored output");
}

/// Test that scripts compare compiler outputs
#[test]
fn test_scripts_compare_outputs() {
    let sh_content = fs::read_to_string("scripts/bootstrap_verify.sh")
        .expect("Should be able to read bash script");
    let ps1_content = fs::read_to_string("scripts/bootstrap_verify.ps1")
        .expect("Should be able to read PowerShell script");
    
    assert!(sh_content.contains("diff") || sh_content.contains("compare"), 
        "Bash script should compare outputs");
    assert!(ps1_content.contains("Compare") || ps1_content.contains("-eq"), 
        "PowerShell script should compare outputs");
}

/// Test that scripts test with example files
#[test]
fn test_scripts_use_examples() {
    let sh_content = fs::read_to_string("scripts/bootstrap_verify.sh")
        .expect("Should be able to read bash script");
    let ps1_content = fs::read_to_string("scripts/bootstrap_verify.ps1")
        .expect("Should be able to read PowerShell script");
    
    assert!(sh_content.contains("examples/hello.ov") || sh_content.contains("examples/"), 
        "Bash script should use example files");
    assert!(ps1_content.contains("examples\\hello.ov") || ps1_content.contains("examples/hello.ov"), 
        "PowerShell script should use example files");
}

/// Test that scripts report success/failure
#[test]
fn test_scripts_report_results() {
    let sh_content = fs::read_to_string("scripts/bootstrap_verify.sh")
        .expect("Should be able to read bash script");
    let ps1_content = fs::read_to_string("scripts/bootstrap_verify.ps1")
        .expect("Should be able to read PowerShell script");
    
    assert!(sh_content.contains("PASSED") || sh_content.contains("✅"), 
        "Bash script should report success");
    assert!(ps1_content.contains("PASSED") || ps1_content.contains("✅"), 
        "PowerShell script should report success");
}

/// Test that scripts have proper documentation
#[test]
fn test_scripts_have_documentation() {
    let sh_content = fs::read_to_string("scripts/bootstrap_verify.sh")
        .expect("Should be able to read bash script");
    let ps1_content = fs::read_to_string("scripts/bootstrap_verify.ps1")
        .expect("Should be able to read PowerShell script");
    
    // Check for header comments
    assert!(sh_content.contains("Bootstrap Verification"), 
        "Bash script should have documentation header");
    assert!(ps1_content.contains("Bootstrap Verification"), 
        "PowerShell script should have documentation header");
}

/// Test that scripts handle missing compilers gracefully
#[test]
fn test_scripts_handle_missing_compilers() {
    let sh_content = fs::read_to_string("scripts/bootstrap_verify.sh")
        .expect("Should be able to read bash script");
    let ps1_content = fs::read_to_string("scripts/bootstrap_verify.ps1")
        .expect("Should be able to read PowerShell script");
    
    assert!(sh_content.contains("not found") || sh_content.contains("Error"), 
        "Bash script should handle missing compilers");
    assert!(ps1_content.contains("not found") || ps1_content.contains("Error"), 
        "PowerShell script should handle missing compilers");
}

/// Test that scripts create stage 0 and stage 1 compilers
#[test]
fn test_scripts_create_compiler_stages() {
    let sh_content = fs::read_to_string("scripts/bootstrap_verify.sh")
        .expect("Should be able to read bash script");
    let ps1_content = fs::read_to_string("scripts/bootstrap_verify.ps1")
        .expect("Should be able to read PowerShell script");
    
    assert!(sh_content.contains("STAGE0_COMPILER") && sh_content.contains("STAGE1_COMPILER"), 
        "Bash script should define stage 0 and stage 1 compilers");
    assert!(ps1_content.contains("Stage0Compiler") && ps1_content.contains("Stage1Compiler"), 
        "PowerShell script should define stage 0 and stage 1 compilers");
}

/// Test that scripts run self-check commands
#[test]
fn test_scripts_run_self_check() {
    let sh_content = fs::read_to_string("scripts/bootstrap_verify.sh")
        .expect("Should be able to read bash script");
    let ps1_content = fs::read_to_string("scripts/bootstrap_verify.ps1")
        .expect("Should be able to read PowerShell script");
    
    assert!(sh_content.contains("--self-check"), 
        "Bash script should run self-check");
    assert!(ps1_content.contains("--self-check"), 
        "PowerShell script should run self-check");
}

/// Test that scripts provide next steps
#[test]
fn test_scripts_provide_next_steps() {
    let sh_content = fs::read_to_string("scripts/bootstrap_verify.sh")
        .expect("Should be able to read bash script");
    let ps1_content = fs::read_to_string("scripts/bootstrap_verify.ps1")
        .expect("Should be able to read PowerShell script");
    
    assert!(sh_content.contains("Next Steps"), 
        "Bash script should provide next steps");
    assert!(ps1_content.contains("Next Steps"), 
        "PowerShell script should provide next steps");
}

// NOTE: The following tests are currently disabled because they require
// a working Ovie-in-Ovie compiler. They will be enabled once Task 7.1 is complete.

#[test]
#[ignore = "Requires working Ovie-in-Ovie compiler (Task 7.1)"]
fn test_bash_script_execution() {
    // This test will actually run the bash script once the compiler is ready
    let output = Command::new("bash")
        .arg("scripts/bootstrap_verify.sh")
        .output()
        .expect("Failed to execute bash script");
    
    assert!(output.status.success(), "Bash script should execute successfully");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("PASSED"), "Script should report success");
}

#[test]
#[ignore = "Requires working Ovie-in-Ovie compiler (Task 7.1)"]
fn test_powershell_script_execution() {
    // This test will actually run the PowerShell script once the compiler is ready
    let output = Command::new("powershell")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg("scripts/bootstrap_verify.ps1")
        .output()
        .expect("Failed to execute PowerShell script");
    
    assert!(output.status.success(), "PowerShell script should execute successfully");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("PASSED"), "Script should report success");
}

#[test]
#[ignore = "Requires working Ovie-in-Ovie compiler (Task 7.1)"]
fn test_script_creates_stage_compilers() {
    // Test that the script actually creates stage 0 and stage 1 compilers
    let output = Command::new("bash")
        .arg("scripts/bootstrap_verify.sh")
        .output()
        .expect("Failed to execute bash script");
    
    assert!(output.status.success(), "Script should execute successfully");
    
    // Check that stage compilers were created (they should be cleaned up after)
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Stage 0 compiler ready"), "Should create stage 0 compiler");
    assert!(stdout.contains("Stage 1 compiler ready"), "Should create stage 1 compiler");
}

#[test]
#[ignore = "Requires working Ovie-in-Ovie compiler (Task 7.1)"]
fn test_script_compares_versions() {
    // Test that the script compares compiler versions
    let output = Command::new("bash")
        .arg("scripts/bootstrap_verify.sh")
        .output()
        .expect("Failed to execute bash script");
    
    assert!(output.status.success(), "Script should execute successfully");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Version Equivalence Test"), "Should run version test");
    assert!(stdout.contains("Stage 0 version"), "Should show stage 0 version");
    assert!(stdout.contains("Stage 1 version"), "Should show stage 1 version");
}

#[test]
#[ignore = "Requires working Ovie-in-Ovie compiler (Task 7.1)"]
fn test_script_compiles_examples() {
    // Test that the script compiles example programs
    let output = Command::new("bash")
        .arg("scripts/bootstrap_verify.sh")
        .output()
        .expect("Failed to execute bash script");
    
    assert!(output.status.success(), "Script should execute successfully");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Compilation Equivalence Test"), "Should run compilation test");
    assert!(stdout.contains("examples/hello.ov"), "Should compile hello.ov");
}

#[test]
#[ignore = "Requires working Ovie-in-Ovie compiler (Task 7.1)"]
fn test_script_runs_compiled_programs() {
    // Test that the script runs compiled programs and compares outputs
    let output = Command::new("bash")
        .arg("scripts/bootstrap_verify.sh")
        .output()
        .expect("Failed to execute bash script");
    
    assert!(output.status.success(), "Script should execute successfully");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Runtime Equivalence Test"), "Should run runtime test");
    assert!(stdout.contains("Stage 0 output") || stdout.contains("Stage 1 output"), 
        "Should show program outputs");
}

#[test]
#[ignore = "Requires working Ovie-in-Ovie compiler (Task 7.1)"]
fn test_script_runs_self_check() {
    // Test that the script runs self-check diagnostics
    let output = Command::new("bash")
        .arg("scripts/bootstrap_verify.sh")
        .output()
        .expect("Failed to execute bash script");
    
    assert!(output.status.success(), "Script should execute successfully");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Self-Check Diagnostics"), "Should run self-check");
}

#[test]
#[ignore = "Requires working Ovie-in-Ovie compiler (Task 7.1)"]
fn test_script_cleans_up() {
    // Test that the script cleans up temporary files
    let output = Command::new("bash")
        .arg("scripts/bootstrap_verify.sh")
        .output()
        .expect("Failed to execute bash script");
    
    assert!(output.status.success(), "Script should execute successfully");
    
    // Check that temporary files don't exist after script completes
    assert!(!PathBuf::from("oviec_stage0").exists(), "Stage 0 compiler should be cleaned up");
    assert!(!PathBuf::from("oviec_stage1").exists(), "Stage 1 compiler should be cleaned up");
    assert!(!PathBuf::from("bootstrap_test").exists(), "Test directory should be cleaned up");
}
