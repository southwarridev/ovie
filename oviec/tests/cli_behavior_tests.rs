// CLI Behavior Tests for Task 12.2
// Tests exit codes, output format, error handling, and help documentation

use std::process::{Command, Stdio};
use std::fs;
use std::path::Path;

#[test]
fn test_help_command_exits_successfully() {
    let output = Command::new("cargo")
        .args(&["run", "--package", "oviec", "--", "help"])
        .output()
        .expect("Failed to execute command");
    
    assert_eq!(output.status.code(), Some(0), "Help command should exit with code 0");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("USAGE:"), "Help should contain USAGE section");
    assert!(stdout.contains("COMMANDS:"), "Help should contain COMMANDS section");
    assert!(stdout.contains("OPTIONS:"), "Help should contain OPTIONS section");
    assert!(stdout.contains("EXIT CODES:"), "Help should contain EXIT CODES section");
}

#[test]
fn test_version_command_exits_successfully() {
    let output = Command::new("cargo")
        .args(&["run", "--package", "oviec", "--", "version"])
        .output()
        .expect("Failed to execute command");
    
    assert_eq!(output.status.code(), Some(0), "Version command should exit with code 0");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Ovie Compiler"), "Version should contain compiler name");
}

#[test]
fn test_missing_input_file_exits_with_error() {
    let output = Command::new("cargo")
        .args(&["run", "--package", "oviec", "--", "run"])
        .output()
        .expect("Failed to execute command");
    
    assert_eq!(output.status.code(), Some(1), "Missing input file should exit with code 1");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error:") || stderr.contains("No input file"), 
            "Should report missing input file error");
}

#[test]
fn test_nonexistent_file_exits_with_error() {
    let output = Command::new("cargo")
        .args(&["run", "--package", "oviec", "--", "run", "nonexistent_file_12345.ov"])
        .output()
        .expect("Failed to execute command");
    
    assert_eq!(output.status.code(), Some(1), "Nonexistent file should exit with code 1");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error:") || stderr.contains("Could not read"), 
            "Should report file not found error");
}

#[test]
fn test_check_command_with_valid_file() {
    // Create a temporary valid Ovie file
    let test_file = "test_cli_valid.ov";
    fs::write(test_file, "seeAm \"Hello, World!\"").expect("Failed to write test file");
    
    let output = Command::new("cargo")
        .args(&["run", "--package", "oviec", "--", "check", test_file])
        .output()
        .expect("Failed to execute command");
    
    // Clean up
    let _ = fs::remove_file(test_file);
    
    assert_eq!(output.status.code(), Some(0), "Valid file should exit with code 0");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No errors found") || stdout.contains("✓"), 
            "Should report no errors for valid file");
}

#[test]
fn test_check_command_with_invalid_syntax() {
    // Create a temporary invalid Ovie file
    let test_file = "test_cli_invalid.ov";
    fs::write(test_file, "this is not valid ovie syntax @#$%").expect("Failed to write test file");
    
    let output = Command::new("cargo")
        .args(&["run", "--package", "oviec", "--", "check", test_file])
        .output()
        .expect("Failed to execute command");
    
    // Clean up
    let _ = fs::remove_file(test_file);
    
    assert_eq!(output.status.code(), Some(1), "Invalid syntax should exit with code 1");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error:"), "Should report syntax error");
}

#[test]
fn test_new_command_creates_project() {
    let project_name = "test_cli_project_12345";
    
    // Clean up if exists
    if Path::new(project_name).exists() {
        let _ = fs::remove_dir_all(project_name);
    }
    
    let output = Command::new("cargo")
        .args(&["run", "--package", "oviec", "--", "new", project_name])
        .output()
        .expect("Failed to execute command");
    
    assert_eq!(output.status.code(), Some(0), "New command should exit with code 0");
    
    // Verify project structure
    assert!(Path::new(project_name).exists(), "Project directory should be created");
    assert!(Path::new(&format!("{}/main.ov", project_name)).exists(), "main.ov should be created");
    assert!(Path::new(&format!("{}/ovie.toml", project_name)).exists(), "ovie.toml should be created");
    
    // Clean up
    let _ = fs::remove_dir_all(project_name);
}

#[test]
fn test_new_command_fails_on_existing_directory() {
    let project_name = "test_cli_existing_12345";
    
    // Create directory first
    fs::create_dir(project_name).expect("Failed to create test directory");
    
    let output = Command::new("cargo")
        .args(&["run", "--package", "oviec", "--", "new", project_name])
        .output()
        .expect("Failed to execute command");
    
    // Clean up
    let _ = fs::remove_dir_all(project_name);
    
    assert_eq!(output.status.code(), Some(1), "Should fail when directory exists");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("already exists"), "Should report directory already exists");
}

#[test]
fn test_fmt_command_validates_syntax() {
    // Create a temporary valid Ovie file
    let test_file = "test_cli_fmt.ov";
    fs::write(test_file, "seeAm \"Hello\"").expect("Failed to write test file");
    
    let output = Command::new("cargo")
        .args(&["run", "--package", "oviec", "--", "fmt", test_file])
        .output()
        .expect("Failed to execute command");
    
    // Clean up
    let _ = fs::remove_file(test_file);
    
    assert_eq!(output.status.code(), Some(0), "Fmt command should exit with code 0 for valid file");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("valid") || stdout.contains("✓"), 
            "Should report file is valid");
}

#[test]
fn test_env_command_shows_environment_info() {
    let output = Command::new("cargo")
        .args(&["run", "--package", "oviec", "--", "env"])
        .output()
        .expect("Failed to execute command");
    
    assert_eq!(output.status.code(), Some(0), "Env command should exit with code 0");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Ovie Runtime Environment") || stdout.contains("ORE"), 
            "Should show environment information");
}

#[test]
fn test_self_check_validates_compiler() {
    let output = Command::new("cargo")
        .args(&["run", "--package", "oviec", "--", "self-check"])
        .output()
        .expect("Failed to execute command");
    
    // Self-check should pass with exit code 0
    assert_eq!(output.status.code(), Some(0), "Self-check should exit with code 0");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Self-Check") || stdout.contains("PASS"), 
            "Should show self-check results");
}

#[test]
fn test_deterministic_output_format() {
    // Create a test file
    let test_file = "test_cli_deterministic.ov";
    fs::write(test_file, "mut x = 42").expect("Failed to write test file");
    
    // Run check command multiple times
    let mut outputs = Vec::new();
    for _ in 0..3 {
        let output = Command::new("cargo")
            .args(&["run", "--package", "oviec", "--", "check", test_file])
            .output()
            .expect("Failed to execute command");
        
        outputs.push(String::from_utf8_lossy(&output.stdout).to_string());
    }
    
    // Clean up
    let _ = fs::remove_file(test_file);
    
    // All outputs should be identical
    assert_eq!(outputs[0], outputs[1], "Output should be deterministic (run 1 vs 2)");
    assert_eq!(outputs[1], outputs[2], "Output should be deterministic (run 2 vs 3)");
}

#[test]
fn test_unknown_command_shows_error() {
    let output = Command::new("cargo")
        .args(&["run", "--package", "oviec", "--", "unknown_command_xyz"])
        .output()
        .expect("Failed to execute command");
    
    assert_eq!(output.status.code(), Some(1), "Unknown command should exit with code 1");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown") || stderr.contains("Error:"), 
            "Should report unknown command error");
}

#[test]
fn test_test_command_with_no_tests() {
    // Create a temporary directory with no test files
    let test_dir = "test_cli_no_tests_12345";
    fs::create_dir(test_dir).expect("Failed to create test directory");
    
    let output = Command::new("cargo")
        .args(&["run", "--package", "oviec", "--", "test", test_dir])
        .output()
        .expect("Failed to execute command");
    
    // Clean up
    let _ = fs::remove_dir_all(test_dir);
    
    assert_eq!(output.status.code(), Some(0), "Test command with no tests should exit with code 0");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No test files found"), "Should report no tests found");
}

#[test]
fn test_exit_code_documentation_in_help() {
    let output = Command::new("cargo")
        .args(&["run", "--package", "oviec", "--", "help"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Verify exit codes are documented
    assert!(stdout.contains("EXIT CODES:"), "Help should document exit codes");
    assert!(stdout.contains("0"), "Should document success exit code");
    assert!(stdout.contains("1"), "Should document error exit code");
    assert!(stdout.contains("2"), "Should document invariant violation exit code");
}
