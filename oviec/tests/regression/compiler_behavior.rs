//! Compiler behavior regression tests

use crate::{Compiler, OvieResult};
use std::collections::HashMap;

/// Test for lexer behavior regressions
pub fn test_lexer_behavior_regression() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    
    // Known good test cases with expected token counts
    let test_cases = vec![
        ("let x = 42;", 5),
        ("fn main() {}", 4),
        ("if true { print(\"hello\"); }", 7),
        ("1 + 2 * 3", 5),
    ];
    
    for (source, expected_tokens) in test_cases {
        let ast = compiler.compile_to_ast(source)?;
        // In a full implementation, would verify token count and types
        // For now, just ensure compilation succeeds
        assert!(matches!(ast, crate::AstNode::Program(_)));
    }
    
    Ok(())
}

/// Test for parser behavior regressions
pub fn test_parser_behavior_regression() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    
    // Test cases that should produce specific AST structures
    let test_cases = vec![
        r#"let x = 42;"#,
        r#"fn add(a: i32, b: i32) -> i32 { return a + b; }"#,
        r#"if x > 0 { print("positive"); }"#,
        r#"while i < 10 { i = i + 1; }"#,
    ];
    
    for source in test_cases {
        let ast = compiler.compile_to_ast(source)?;
        
        // Verify AST structure hasn't changed unexpectedly
        match ast {
            crate::AstNode::Program(statements) => {
                assert!(!statements.is_empty());
            }
            _ => panic!("Expected program node"),
        }
    }
    
    Ok(())
}

/// Test for type checker behavior regressions
pub fn test_type_checker_behavior_regression() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    
    // Test cases with known type inference results
    let valid_cases = vec![
        "let x = 42;",           // Should infer i32
        "let x = 3.14;",         // Should infer f64
        "let x = true;",         // Should infer bool
        "let x = \"hello\";",    // Should infer String
    ];
    
    for source in valid_cases {
        let hir = compiler.compile_to_hir(source)?;
        assert!(!hir.items.is_empty());
    }
    
    // Test cases that should produce type errors
    let error_cases = vec![
        "let x: i32 = \"string\";",  // Type mismatch
        "let x = 1 + \"hello\";",    // Invalid operation
    ];
    
    for source in error_cases {
        let result = compiler.compile_to_hir(source);
        assert!(result.is_err(), "Should produce type error: {}", source);
    }
    
    Ok(())
}

/// Test for code generation behavior regressions
pub fn test_codegen_behavior_regression() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    
    // Test cases with known code generation patterns
    let test_cases = vec![
        r#"fn main() { let x = 42; }"#,
        r#"fn add(a: i32, b: i32) -> i32 { return a + b; }"#,
        r#"fn main() { if true { print("hello"); } }"#,
    ];
    
    for source in test_cases {
        // Test WASM generation
        let wasm_result = compiler.compile_to_wasm(source)?;
        assert!(!wasm_result.is_empty());
        assert_eq!(&wasm_result[0..4], b"\0asm"); // WASM magic number
        
        // Test MIR generation
        let mir_result = compiler.compile_to_mir(source)?;
        assert!(!mir_result.functions.is_empty());
    }
    
    Ok(())
}

/// Test for deterministic behavior regressions
pub fn test_deterministic_behavior_regression() -> OvieResult<()> {
    let mut compiler = Compiler::new_deterministic();
    
    let test_cases = vec![
        "let x = 42;",
        "fn main() { print(\"hello\"); }",
        "let result = 1 + 2 * 3;",
    ];
    
    for source in test_cases {
        // Compile multiple times and verify identical output
        let result1 = compiler.compile_to_wasm(source)?;
        let result2 = compiler.compile_to_wasm(source)?;
        
        assert_eq!(result1, result2, "Deterministic compilation failed for: {}", source);
    }
    
    Ok(())
}

/// Test for error message consistency regressions
pub fn test_error_message_regression() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    
    // Test cases with known error patterns
    let error_cases = vec![
        ("let x: i32 = \"string\";", "type"),
        ("unknown_function();", "undefined"),
        ("let x = ;", "syntax"),
    ];
    
    for (source, expected_error_type) in error_cases {
        match compiler.compile_to_hir(source) {
            Ok(_) => panic!("Expected error for: {}", source),
            Err(error) => {
                let error_msg = error.to_string().to_lowercase();
                assert!(error_msg.contains(expected_error_type), 
                    "Error message should contain '{}' for source: {}\nActual error: {}", 
                    expected_error_type, source, error_msg);
            }
        }
    }
    
    Ok(())
}

/// Run all compiler behavior regression tests
pub fn run_all_behavior_regression_tests() -> OvieResult<Vec<String>> {
    let mut results = Vec::new();
    
    println!("Running compiler behavior regression tests...");
    
    // Run each test and collect results
    let tests = vec![
        ("lexer_behavior", test_lexer_behavior_regression),
        ("parser_behavior", test_parser_behavior_regression),
        ("type_checker_behavior", test_type_checker_behavior_regression),
        ("codegen_behavior", test_codegen_behavior_regression),
        ("deterministic_behavior", test_deterministic_behavior_regression),
        ("error_message_consistency", test_error_message_regression),
    ];
    
    for (test_name, test_fn) in tests {
        match test_fn() {
            Ok(_) => {
                println!("  ✓ {}", test_name);
                results.push(format!("PASS: {}", test_name));
            }
            Err(error) => {
                println!("  ✗ {}: {}", test_name, error);
                results.push(format!("FAIL: {}: {}", test_name, error));
            }
        }
    }
    
    Ok(results)
}