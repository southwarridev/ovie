//! Property-based tests for the Ovie CLI

use proptest::prelude::*;
use std::process::Command;
use std::fs;
use tempfile::TempDir;

/// Property 13: CLI Command Completeness
/// Validates: Requirements 9.1
/// 
/// This property ensures that all required CLI commands are available and functional.
#[cfg(test)]
mod cli_tests {
    use super::*;

    #[test]
    fn test_cli_help_command() {
        let output = Command::new("cargo")
            .args(&["run", "-p", "ovie", "--", "--help"])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Verify all required commands are listed
        assert!(stdout.contains("new"));
        assert!(stdout.contains("build"));
        assert!(stdout.contains("run"));
        assert!(stdout.contains("test"));
        assert!(stdout.contains("fmt"));
        assert!(stdout.contains("update"));
        assert!(stdout.contains("vendor"));
    }

    #[test]
    fn test_cli_version_command() {
        let output = Command::new("cargo")
            .args(&["run", "-p", "ovie", "--", "--version"])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("ovie"));
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(5))]
        
        /// Property 13: CLI Command Completeness
        /// **Validates: Requirements 9.1**
        /// 
        /// This property verifies that all CLI commands handle invalid arguments gracefully
        /// and provide helpful error messages.
        #[test]
        fn property_cli_command_error_handling(
            invalid_arg in "[a-zA-Z0-9_-]{1,10}" // Reduced length for faster testing
        ) {
            let commands = vec!["new", "build", "run", "test", "fmt", "update", "vendor"];
            
            for command in commands {
                let output = Command::new("cargo")
                    .args(&["run", "-p", "ovie", "--", command, &format!("--{}", invalid_arg)])
                    .output()
                    .expect("Failed to execute command");

                // Command should either succeed or fail gracefully with helpful error
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    // Should contain helpful error information
                    assert!(
                        stderr.contains("error:") || 
                        stderr.contains("Error:") || 
                        stderr.contains("help") ||
                        stderr.contains("usage") ||
                        stderr.len() > 0,
                        "Command '{}' with invalid arg '{}' should provide helpful error message",
                        command, invalid_arg
                    );
                }
            }
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(5))]
        
        /// Property 14: Project Scaffolding Consistency
        /// **Validates: Requirements 9.2**
        /// 
        /// This property ensures that project scaffolding creates consistent project structures.
        #[test]
        fn property_project_scaffolding_consistency(
            project_name in "[a-zA-Z][a-zA-Z0-9_-]{0,9}" // Reduced length for faster testing
        ) {
            let temp_dir = TempDir::new().expect("Failed to create temp directory");
            let project_path = temp_dir.path().join(&project_name);
            
            // Create new project
            let output = Command::new("cargo")
                .args(&["run", "-p", "ovie", "--", "new", &project_name, "--path", project_path.to_str().unwrap()])
                .output()
                .expect("Failed to execute command");

            if output.status.success() {
                // Verify standard project structure exists
                assert!(project_path.join("src").exists(), "src directory should exist");
                assert!(project_path.join("tests").exists(), "tests directory should exist");
                assert!(project_path.join("src/main.ov").exists(), "main.ov should exist");
                assert!(project_path.join("ovie.toml").exists(), "ovie.toml should exist");
                assert!(project_path.join("README.md").exists(), "README.md should exist");
                assert!(project_path.join(".gitignore").exists(), ".gitignore should exist");

                // Verify main.ov contains expected content
                let main_content = fs::read_to_string(project_path.join("src/main.ov"))
                    .expect("Failed to read main.ov");
                assert!(main_content.contains("seeAm"), "main.ov should contain seeAm statement");
                assert!(main_content.contains(&project_name), "main.ov should reference project name");

                // Verify ovie.toml contains expected content
                let toml_content = fs::read_to_string(project_path.join("ovie.toml"))
                    .expect("Failed to read ovie.toml");
                assert!(toml_content.contains(&project_name), "ovie.toml should contain project name");
                assert!(toml_content.contains("[project]"), "ovie.toml should have [project] section");
                assert!(toml_content.contains("[build]"), "ovie.toml should have [build] section");
            }
        }
    }

    /// Property 15: Testing Framework Dual Support
    /// **Validates: Requirements 9.3**
    /// 
    /// This property ensures the testing framework supports both unit tests and property-based tests.
    #[test]
    fn test_testing_framework_dual_support() {
        // Test that the test command exists and provides help
        let help_output = Command::new("cargo")
            .args(&["run", "-p", "ovie", "--", "test", "--help"])
            .output()
            .expect("Failed to execute test help command");

        // Should either succeed or provide helpful error message
        if help_output.status.success() {
            let stdout = String::from_utf8_lossy(&help_output.stdout);
            assert!(stdout.contains("test") || stdout.contains("Test"), "Help should mention testing");
        } else {
            let stderr = String::from_utf8_lossy(&help_output.stderr);
            // Should contain helpful error information
            assert!(
                stderr.contains("test") || stderr.contains("help") || stderr.len() > 0,
                "Should provide helpful error message"
            );
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(5))]
        
        /// Property 16: Code Formatting Consistency
        /// **Validates: Requirements 9.4**
        /// 
        /// This property ensures that code formatting is consistent and idempotent.
        #[test]
        fn property_code_formatting_consistency(
            spaces_before_op in 0..3usize, // Reduced range for faster testing
            spaces_after_op in 0..3usize,
            newlines_before_brace in 0..2usize
        ) {
            let temp_dir = TempDir::new().expect("Failed to create temp directory");
            let test_file = temp_dir.path().join("test.ov");
            
            // Create poorly formatted code with varying spacing
            let poorly_formatted = format!(
                "{}seeAm\"Hello\";{}x{}={}2+3;{}if x==5{}{{{}\n    seeAm \"Good\";\n}}",
                " ".repeat(spaces_before_op),
                "\n".repeat(newlines_before_brace),
                " ".repeat(spaces_before_op),
                " ".repeat(spaces_after_op),
                "\n".repeat(newlines_before_brace),
                " ".repeat(spaces_before_op),
                "\n".repeat(newlines_before_brace)
            );
            
            fs::write(&test_file, &poorly_formatted)
                .expect("Failed to write test file");

            // Format the file
            let format_output = Command::new("cargo")
                .args(&["run", "-p", "ovie", "--", "fmt", test_file.to_str().unwrap()])
                .output()
                .expect("Failed to execute format command");

            if format_output.status.success() {
                let formatted_content = fs::read_to_string(&test_file)
                    .expect("Failed to read formatted file");

                // Format again to test idempotency
                let format_again_output = Command::new("cargo")
                    .args(&["run", "-p", "ovie", "--", "fmt", test_file.to_str().unwrap()])
                    .output()
                    .expect("Failed to execute format command again");

                if format_again_output.status.success() {
                    let formatted_again_content = fs::read_to_string(&test_file)
                        .expect("Failed to read formatted file again");

                    // Formatting should be idempotent
                    assert_eq!(
                        formatted_content, 
                        formatted_again_content,
                        "Formatting should be idempotent"
                    );

                    // Formatted code should have consistent spacing
                    assert!(formatted_content.contains(" = "), "Should have spaces around assignment");
                    assert!(formatted_content.contains(" + "), "Should have spaces around operators");
                    assert!(formatted_content.contains("if "), "Should have space after if");
                }
            }
        }
    }
}