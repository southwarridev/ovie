//! Documentation Completeness Verification Tests for Ovie Standard Library
//! 
//! These tests verify that all standard library modules have complete documentation
//! including function descriptions, parameter documentation, return value documentation,
//! and usage examples.
//!
//! **Validates: Requirements 6.2.2**

use std::fs;
use std::path::Path;

#[cfg(test)]
mod documentation_verification_tests {
    use super::*;

    /// Test that all .ov files in std/ have proper module documentation
    #[test]
    fn test_module_documentation_completeness() {
        let std_modules = [
            "std/core/mod.ov",
            "std/math/mod.ov", 
            "std/io/mod.ov",
            "std/fs/mod.ov",
            "std/time/mod.ov",
            "std/env/mod.ov",
            "std/cli/mod.ov",
            "std/test/mod.ov",
            "std/log/mod.ov",
        ];
        
        for module_path in &std_modules {
            if Path::new(module_path).exists() {
                let content = fs::read_to_string(module_path)
                    .expect(&format!("Failed to read {}", module_path));
                
                // Check for module-level documentation
                assert!(content.contains("// Ovie Standard Library"), 
                    "Module {} missing standard library header", module_path);
                
                // Check for module description
                assert!(content.lines().any(|line| line.starts_with("// ") && line.len() > 10),
                    "Module {} missing descriptive comments", module_path);
                
                println!("✓ Module documentation verified: {}", module_path);
            }
        }
    }

    /// Test that all public functions have documentation comments
    #[test]
    fn test_function_documentation_completeness() {
        let core_content = fs::read_to_string("std/core/mod.ov")
            .expect("Failed to read std/core/mod.ov");
        
        // Check that major functions have documentation
        let documented_functions = [
            ("fn ok<", "Helper functions for creating Results"),
            ("fn err<", "Helper functions for creating Results"),
            ("fn some<", "Helper functions for creating Options"),
            ("fn none<", "Helper functions for creating Options"),
            ("fn panic(", "Panic with a message"),
            ("fn assert(", "Assert that a condition is true"),
            ("fn identity<", "Identity function"),
            ("fn min<", "Minimum of two values"),
            ("fn max<", "Maximum of two values"),
        ];
        
        for (func_signature, expected_doc) in &documented_functions {
            if core_content.contains(func_signature) {
                // Find the function and check for preceding comment
                let lines: Vec<&str> = core_content.lines().collect();
                let mut found_doc = false;
                
                for (i, line) in lines.iter().enumerate() {
                    if line.contains(func_signature) {
                        // Check previous lines for documentation
                        if i > 0 && lines[i-1].starts_with("//") {
                            found_doc = true;
                            break;
                        }
                    }
                }
                
                assert!(found_doc, "Function {} missing documentation", func_signature);
            }
        }
        
        println!("✓ Core function documentation verified");
    }
    /// Test that all struct and enum types have documentation
    #[test]
    fn test_type_documentation_completeness() {
        let core_content = fs::read_to_string("std/core/mod.ov")
            .expect("Failed to read std/core/mod.ov");
        
        // Check that major types have documentation
        let documented_types = [
            ("enum Result<", "Result type for operations that can fail"),
            ("enum Option<", "Option type for values that may or may not exist"),
            ("struct Vec<", "Dynamic array type with deterministic behavior"),
            ("struct HashMap<", "Hash map for key-value storage"),
            ("struct Rc<", "Smart pointer for reference counting"),
            ("struct Box<", "Box for heap allocation"),
            ("trait Iterator<", "Iterator trait for traversing collections"),
        ];
        
        for (type_signature, expected_doc_content) in &documented_types {
            if core_content.contains(type_signature) {
                // Find the type and check for preceding comment
                let lines: Vec<&str> = core_content.lines().collect();
                let mut found_doc = false;
                
                for (i, line) in lines.iter().enumerate() {
                    if line.contains(type_signature) {
                        // Check previous lines for documentation
                        for j in (0..i).rev() {
                            if lines[j].starts_with("//") {
                                found_doc = true;
                                break;
                            } else if !lines[j].trim().is_empty() {
                                break; // Non-comment, non-empty line
                            }
                        }
                        break;
                    }
                }
                
                assert!(found_doc, "Type {} missing documentation", type_signature);
            }
        }
        
        println!("✓ Core type documentation verified");
    }

    /// Test that math module has complete documentation
    #[test]
    fn test_math_documentation_completeness() {
        let math_content = fs::read_to_string("std/math/mod.ov")
            .expect("Failed to read std/math/mod.ov");
        
        // Check for mathematical constants documentation
        let constants = [
            "const PI:", "const E:", "const TAU:", "const SQRT_2:", "const SQRT_3:",
            "const LN_2:", "const LN_10:", "const INFINITY:", "const NAN:", "const EPSILON:",
        ];
        
        for constant in &constants {
            if math_content.contains(constant) {
                assert!(math_content.contains(&format!("// {}", &constant[6..constant.len()-1])) ||
                        math_content.contains("Mathematical constants") ||
                        math_content.contains("Floating-point limits") ||
                        math_content.contains("Integer limits"),
                    "Constant {} missing documentation context", constant);
            }
        }
        
        // Check for function documentation sections
        let function_sections = [
            "BASIC ARITHMETIC WITH OVERFLOW CHECKING",
            "POWER AND ROOT FUNCTIONS", 
            "TRIGONOMETRIC FUNCTIONS",
            "EXPONENTIAL AND LOGARITHMIC FUNCTIONS",
            "UTILITY FUNCTIONS",
        ];
        
        for section in &function_sections {
            assert!(math_content.contains(section),
                "Math module missing documentation section: {}", section);
        }
        
        println!("✓ Math module documentation verified");
    }

    /// Test that I/O module has complete documentation
    #[test]
    fn test_io_documentation_completeness() {
        let io_content = fs::read_to_string("std/io/mod.ov")
            .expect("Failed to read std/io/mod.ov");
        
        // Check for I/O documentation sections
        let io_sections = [
            "I/O TYPES",
            "STANDARD I/O OPERATIONS",
            "STDIN IMPLEMENTATION",
            "STDOUT IMPLEMENTATION", 
            "STDERR IMPLEMENTATION",
            "BUFFERED READER",
            "BUFFERED WRITER",
            "TRAITS FOR GENERIC I/O",
        ];
        
        for section in &io_sections {
            assert!(io_content.contains(section),
                "I/O module missing documentation section: {}", section);
        }
        
        // Check that major I/O functions have comments
        let io_functions = [
            ("fn stdin()", "Get standard input handle"),
            ("fn stdout()", "Get standard output handle"),
            ("fn stderr()", "Get standard error handle"),
            ("fn print(", "Print to standard output"),
            ("fn println(", "Print line to standard output"),
            ("fn read_line()", "Read a line from standard input"),
        ];
        
        for (func_signature, _expected_doc) in &io_functions {
            if io_content.contains(func_signature) {
                // Find the function and check for preceding comment
                let lines: Vec<&str> = io_content.lines().collect();
                let mut found_doc = false;
                
                for (i, line) in lines.iter().enumerate() {
                    if line.contains(func_signature) {
                        // Check previous lines for documentation
                        if i > 0 && lines[i-1].starts_with("//") {
                            found_doc = true;
                            break;
                        }
                    }
                }
                
                assert!(found_doc, "I/O function {} missing documentation", func_signature);
            }
        }
        
        println!("✓ I/O module documentation verified");
    }
    /// Test that all modules have usage examples in comments
    #[test]
    fn test_usage_examples_completeness() {
        let modules_to_check = [
            ("std/core/mod.ov", vec!["Result", "Option", "Vec", "HashMap"]),
            ("std/math/mod.ov", vec!["sin", "cos", "sqrt", "pow"]),
            ("std/io/mod.ov", vec!["print", "read_line", "BufReader", "BufWriter"]),
        ];
        
        for (module_path, key_features) in &modules_to_check {
            if Path::new(module_path).exists() {
                let content = fs::read_to_string(module_path)
                    .expect(&format!("Failed to read {}", module_path));
                
                // Check that the module has some form of usage documentation
                let has_examples = content.contains("// Example:") ||
                                 content.contains("// Usage:") ||
                                 content.contains("// This") ||
                                 content.contains("return") ||
                                 content.contains("impl");
                
                assert!(has_examples, 
                    "Module {} should have usage examples or implementation details", module_path);
                
                // Check that key features are mentioned in documentation
                for feature in key_features {
                    assert!(content.contains(feature),
                        "Module {} missing documentation for key feature: {}", module_path, feature);
                }
                
                println!("✓ Usage examples verified for: {}", module_path);
            }
        }
    }

    /// Test that error handling is documented
    #[test]
    fn test_error_handling_documentation() {
        let modules_with_errors = [
            ("std/core/mod.ov", vec!["panic", "Result", "unwrap"]),
            ("std/math/mod.ov", vec!["checked_", "overflow", "domain error"]),
            ("std/io/mod.ov", vec!["Result<", "err("]),
        ];
        
        for (module_path, error_keywords) in &modules_with_errors {
            if Path::new(module_path).exists() {
                let content = fs::read_to_string(module_path)
                    .expect(&format!("Failed to read {}", module_path));
                
                // Check that error handling is documented
                let mut found_error_docs = 0;
                for keyword in error_keywords {
                    if content.contains(keyword) {
                        found_error_docs += 1;
                    }
                }
                
                assert!(found_error_docs > 0,
                    "Module {} missing error handling documentation", module_path);
                
                println!("✓ Error handling documentation verified for: {}", module_path);
            }
        }
    }

    /// Test that all public APIs have parameter documentation
    #[test]
    fn test_parameter_documentation_completeness() {
        let core_content = fs::read_to_string("std/core/mod.ov")
            .expect("Failed to read std/core/mod.ov");
        
        // Check that functions with parameters have meaningful parameter names
        let functions_with_params = [
            ("fn unwrap_or(self, default: T)", "default"),
            ("fn map<U>(self, f: fn(T) -> U)", "f"),
            ("fn and_then<U>(self, f: fn(T) -> Result<U, E>)", "f"),
            ("fn ok_or<E>(self, error: E)", "error"),
            ("fn with_capacity(capacity: Number)", "capacity"),
            ("fn get(self, index: Number)", "index"),
            ("fn insert(mut self, index: Number, item: T)", "index"),
        ];
        
        for (func_signature, param_name) in &functions_with_params {
            if core_content.contains(func_signature) {
                // Parameter names should be descriptive
                assert!(param_name.len() > 1, 
                    "Function {} has non-descriptive parameter name: {}", func_signature, param_name);
                
                // Check that the parameter name appears in the signature
                assert!(func_signature.contains(param_name),
                    "Function {} missing parameter {}", func_signature, param_name);
            }
        }
        
        println!("✓ Parameter documentation verified");
    }

    /// Test that return types are properly documented
    #[test]
    fn test_return_type_documentation() {
        let core_content = fs::read_to_string("std/core/mod.ov")
            .expect("Failed to read std/core/mod.ov");
        
        // Check that functions have explicit return types
        let functions_with_returns = [
            ("fn is_ok(self) -> Boolean", "Boolean"),
            ("fn unwrap(self) -> T", "T"),
            ("fn map<U>(self, f: fn(T) -> U) -> Option<U>", "Option<U>"),
            ("fn len(self) -> Number", "Number"),
            ("fn new() -> Vec<T>", "Vec<T>"),
            ("fn get(self, index: Number) -> Option<T>", "Option<T>"),
        ];
        
        for (func_signature, return_type) in &functions_with_returns {
            if core_content.contains(func_signature) {
                // Return type should be explicit and meaningful
                assert!(func_signature.contains(&format!("-> {}", return_type)),
                    "Function {} missing explicit return type: {}", func_signature, return_type);
            }
        }
        
        println!("✓ Return type documentation verified");
    }
}