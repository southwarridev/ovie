//! AST Invariant Tests
//! 
//! Tests for AST validation according to Stage 2.1 compiler invariants.
//! These tests ensure that AST nodes maintain their invariants throughout
//! the compilation pipeline.

use crate::{Compiler, OvieResult, OvieError};

/// Test that AST validation passes for valid AST structures
#[test]
fn test_ast_invariants_valid_structures() -> OvieResult<()> {
    let test_programs = [
        "let x = 42;",
        "fn main() { print(\"hello\"); }",
        "struct Point { x: i32, y: i32 }",
        "if true { print(\"yes\"); } else { print(\"no\"); }",
        "while x < 10 { x = x + 1; }",
        "for i in 0..10 { print(i); }",
        "let result = add(1, 2);",
        "mut counter = 0;",
        "return 42;",
    ];
    
    let mut compiler = Compiler::new();
    
    for program in &test_programs {
        let ast = compiler.compile_to_ast(program)?;
        
        // AST validation should pass for all valid programs
        ast.validate().map_err(|e| OvieError::CompileError {
            message: format!("AST invariant validation failed for '{}': {}", program, e)
        })?;
    }
    
    Ok(())
}

/// Test that AST contains no resolved types (invariant 2.1.2)
#[test]
fn test_ast_no_type_info_invariant() -> OvieResult<()> {
    let test_programs = [
        "let x: i32 = 42;",  // Type annotations are syntax, not resolved types
        "fn add(a: i32, b: i32) -> i32 { a + b }",
        "struct Point { x: f64, y: f64 }",
        "let array: [i32; 5] = [1, 2, 3, 4, 5];",
    ];
    
    let mut compiler = Compiler::new();
    
    for program in &test_programs {
        let ast = compiler.compile_to_ast(program)?;
        
        // AST validation includes checking for no type info
        ast.validate().map_err(|e| OvieError::CompileError {
            message: format!("AST no-type-info invariant failed for '{}': {}", program, e)
        })?;
    }
    
    Ok(())
}

/// Test that AST contains no symbol IDs (invariant 2.1.3)
#[test]
fn test_ast_no_symbol_id_invariant() -> OvieResult<()> {
    let test_programs = [
        "let x = 42; print(x);",
        "fn main() { let y = add(1, 2); }",
        "struct Point { x: i32, y: i32 } let p = Point { x: 1, y: 2 };",
    ];
    
    let mut compiler = Compiler::new();
    
    for program in &test_programs {
        let ast = compiler.compile_to_ast(program)?;
        
        // AST validation includes checking for no symbol IDs
        ast.validate().map_err(|e| OvieError::CompileError {
            message: format!("AST no-symbol-id invariant failed for '{}': {}", program, e)
        })?;
    }
    
    Ok(())
}

/// Test that AST preserves source spans (invariant 2.1.4)
#[test]
fn test_ast_source_span_preservation() -> OvieResult<()> {
    let test_programs = [
        "let x = 42;",
        "fn main() {\n    print(\"hello\");\n}",
        "if true {\n    let x = 1;\n} else {\n    let x = 2;\n}",
    ];
    
    let mut compiler = Compiler::new();
    
    for program in &test_programs {
        let ast = compiler.compile_to_ast(program)?;
        
        // AST validation includes checking source spans
        ast.validate().map_err(|e| OvieError::CompileError {
            message: format!("AST source-span invariant failed for '{}': {}", program, e)
        })?;
    }
    
    Ok(())
}

/// Test AST invariant validation with invalid structures
#[test]
fn test_ast_invariant_violations() {
    // This test would create invalid AST structures and verify they fail validation
    // For now, we test that the validation framework exists
    
    let mut compiler = Compiler::new();
    let ast = compiler.compile_to_ast("let x = 42;").unwrap();
    
    // Valid AST should pass validation
    assert!(ast.validate().is_ok());
}

/// Test comprehensive AST validation
#[test]
fn test_comprehensive_ast_validation() -> OvieResult<()> {
    let complex_program = r#"
        struct Point {
            x: f64,
            y: f64,
        }
        
        fn distance(p1: Point, p2: Point) -> f64 {
            let dx = p1.x - p2.x;
            let dy = p1.y - p2.y;
            sqrt(dx * dx + dy * dy)
        }
        
        fn main() {
            let origin = Point { x: 0.0, y: 0.0 };
            let point = Point { x: 3.0, y: 4.0 };
            let dist = distance(origin, point);
            print(dist);
        }
    "#;
    
    let mut compiler = Compiler::new();
    let ast = compiler.compile_to_ast(complex_program)?;
    
    // Comprehensive validation - the validate() method checks all invariants
    ast.validate().map_err(|e| OvieError::CompileError {
        message: format!("Comprehensive AST validation failed: {}", e)
    })?;
    
    Ok(())
}

