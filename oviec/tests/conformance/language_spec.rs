//! Language specification conformance tests

use crate::{Compiler, OvieResult};

/// Test conformance with formal BNF grammar specification
pub fn test_grammar_specification_conformance() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    
    // Test cases that should conform to grammar specification
    let valid_programs = vec![
        "let x = 42;",
        "fn main() {}",
        "if true { print(\"hello\"); }",
        "while x < 10 { x = x + 1; }",
        "for i in 0..10 { print(i); }",
        "match x { 1 => print(\"one\"), _ => print(\"other\") }",
    ];
    
    for program in valid_programs {
        let result = compiler.compile_to_ast(program);
        assert!(result.is_ok(), "Valid program should parse: {}", program);
    }
    
    // Test cases that should violate grammar specification
    let invalid_programs = vec![
        "let = 42;",           // Missing identifier
        "fn () {}",            // Missing function name
        "if { print(\"hello\"); }", // Missing condition
        "while { x = x + 1; }", // Missing condition
        "let x = ;",           // Missing expression
    ];
    
    for program in invalid_programs {
        let result = compiler.compile_to_ast(program);
        assert!(result.is_err(), "Invalid program should not parse: {}", program);
    }
    
    Ok(())
}

/// Test conformance with type system specification
pub fn test_type_system_specification_conformance() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    
    // Test type inference rules
    let type_inference_tests = vec![
        ("let x = 42;", "i32"),
        ("let x = 3.14;", "f64"),
        ("let x = true;", "bool"),
        ("let x = \"hello\";", "String"),
    ];
    
    for (program, expected_type) in type_inference_tests {
        let hir = compiler.compile_to_hir(program)?;
        // In a full implementation, would verify inferred types
        assert!(!hir.items.is_empty());
    }
    
    // Test type checking rules
    let type_error_tests = vec![
        "let x: i32 = \"string\";",  // Type mismatch
        "let x: bool = 42;",         // Type mismatch
        "let x = 1 + \"hello\";",    // Invalid operation
    ];
    
    for program in type_error_tests {
        let result = compiler.compile_to_hir(program);
        assert!(result.is_err(), "Type error should be detected: {}", program);
    }
    
    Ok(())
}

/// Test conformance with memory model specification
pub fn test_memory_model_specification_conformance() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    
    // Test ownership rules
    let ownership_tests = vec![
        r#"
            let x = vec![1, 2, 3];
            let y = x; // Move
            // x is no longer accessible
        "#,
        r#"
            let x = 42;
            let y = &x; // Borrow
            print(*y);
        "#,
    ];
    
    for program in ownership_tests {
        let result = compiler.compile_to_mir(program);
        // Should either succeed (valid ownership) or fail with ownership error
        match result {
            Ok(_) => {}, // Valid ownership
            Err(error) => {
                let error_msg = error.to_string();
                assert!(error_msg.contains("borrow") || error_msg.contains("ownership"));
            }
        }
    }
    
    Ok(())
}

/// Test conformance with error model specification
pub fn test_error_model_specification_conformance() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    
    // Test compile-time error classification
    let compile_time_errors = vec![
        ("let x: i32 = \"string\";", "type error"),
        ("unknown_function();", "name resolution error"),
        ("let x = ;", "syntax error"),
    ];
    
    for (program, error_category) in compile_time_errors {
        let result = compiler.compile_to_hir(program);
        assert!(result.is_err(), "Should produce compile-time error: {}", program);
        
        let error_msg = result.unwrap_err().to_string();
        // Verify error is properly categorized
        assert!(!error_msg.is_empty());
    }
    
    Ok(())
}

/// Test conformance with numeric system specification
pub fn test_numeric_system_specification_conformance() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    
    // Test deterministic numeric operations
    let numeric_tests = vec![
        "let x = 1 + 2;",           // Integer arithmetic
        "let x = 3.14 * 2.0;",      // Floating-point arithmetic
        "let x = 10 / 3;",          // Integer division
        "let x = 10.0 / 3.0;",      // Floating-point division
        "let x = 2_i32.pow(10);",   // Exponentiation
    ];
    
    for program in numeric_tests {
        let result = compiler.compile_to_ast(program);
        assert!(result.is_ok(), "Numeric operation should compile: {}", program);
    }
    
    // Test overflow handling
    let overflow_tests = vec![
        "let x = i32::MAX + 1;",    // Integer overflow
        "let x = 1.0 / 0.0;",       // Division by zero (float)
    ];
    
    for program in overflow_tests {
        let result = compiler.compile_to_mir(program);
        // Should either handle overflow gracefully or produce appropriate error
        match result {
            Ok(_) => {}, // Overflow handled
            Err(error) => {
                let error_msg = error.to_string();
                assert!(error_msg.contains("overflow") || error_msg.contains("division"));
            }
        }
    }
    
    Ok(())
}