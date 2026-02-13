//! HIR Invariant Tests
//! 
//! Tests for HIR validation according to Stage 2.2 compiler invariants.
//! These tests ensure that HIR nodes maintain their invariants throughout
//! the compilation pipeline.

use crate::{Compiler, OvieResult, OvieError};

/// Test that HIR validation passes for valid HIR structures
#[test]
fn test_hir_invariants_valid_structures() -> OvieResult<()> {
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
        let hir = compiler.compile_to_hir(program)?;
        
        // HIR validation should pass for all valid programs
        hir.validate().map_err(|e| OvieError::CompileError {
            message: format!("HIR validation failed for '{}': {}", program, e)
        })?;
        
        // HIR invariant validation should also pass
        hir.validate_invariants().map_err(|e| OvieError::CompileError {
            message: format!("HIR invariant validation failed for '{}': {}", program, e)
        })?;
    }
    
    Ok(())
}

/// Test that all names are resolved in HIR (invariant 2.2.2)
#[test]
fn test_hir_all_names_resolved_invariant() -> OvieResult<()> {
    let test_programs = [
        "let x = 42; print(x);",
        "fn add(a: i32, b: i32) -> i32 { a + b } fn main() { let result = add(1, 2); }",
        "struct Point { x: i32, y: i32 } fn main() { let p = Point { x: 1, y: 2 }; print(p.x); }",
    ];
    
    let mut compiler = Compiler::new();
    
    for program in &test_programs {
        let hir = compiler.compile_to_hir(program)?;
        
        // HIR invariant validation includes checking that all names are resolved
        hir.validate_invariants().map_err(|e| OvieError::CompileError {
            message: format!("HIR all-names-resolved invariant failed for '{}': {}", program, e)
        })?;
    }
    
    Ok(())
}

/// Test that all types are known in HIR (invariant 2.2.3)
#[test]
fn test_hir_all_types_known_invariant() -> OvieResult<()> {
    let test_programs = [
        "let x: i32 = 42;",
        "fn add(a: i32, b: i32) -> i32 { a + b }",
        "struct Point { x: f64, y: f64 } fn main() { let p = Point { x: 1.0, y: 2.0 }; }",
        "let array: [i32; 5] = [1, 2, 3, 4, 5];",
    ];
    
    let mut compiler = Compiler::new();
    
    for program in &test_programs {
        let hir = compiler.compile_to_hir(program)?;
        
        // HIR invariant validation includes checking that all types are known
        hir.validate_invariants().map_err(|e| OvieError::CompileError {
            message: format!("HIR all-types-known invariant failed for '{}': {}", program, e)
        })?;
    }
    
    Ok(())
}

/// Test that HIR contains no lowering artifacts (invariant 2.2.4)
#[test]
fn test_hir_no_lowering_artifacts_invariant() -> OvieResult<()> {
    let test_programs = [
        "if true { print(\"yes\"); } else { print(\"no\"); }",
        "while x < 10 { x = x + 1; }",
        "for i in 0..10 { print(i); }",
        "match value { Some(x) => x, None => 0 }",
    ];
    
    let mut compiler = Compiler::new();
    
    for program in &test_programs {
        let hir = compiler.compile_to_hir(program)?;
        
        // HIR invariant validation includes checking for no lowering artifacts
        hir.validate_invariants().map_err(|e| OvieError::CompileError {
            message: format!("HIR no-lowering-artifacts invariant failed for '{}': {}", program, e)
        })?;
    }
    
    Ok(())
}

/// Test HIR invariant validation with complex programs
#[test]
fn test_comprehensive_hir_validation() -> OvieResult<()> {
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
            
            if dist > 5.0 {
                print("Far away");
            } else {
                print("Close");
            }
            
            mut counter = 0;
            while counter < 10 {
                print(counter);
                counter = counter + 1;
            }
        }
    "#;
    
    let mut compiler = Compiler::new();
    let hir = compiler.compile_to_hir(complex_program)?;
    
    // Comprehensive validation
    hir.validate()?;
    hir.validate_invariants()?;
    
    Ok(())
}

/// Test HIR validation with type inference
#[test]
fn test_hir_type_inference_validation() -> OvieResult<()> {
    let test_programs = [
        "let x = 42;",  // Should infer i32
        "let y = 3.14;", // Should infer f64
        "let z = true;", // Should infer bool
        "let s = \"hello\";", // Should infer string
        "let arr = [1, 2, 3];", // Should infer [i32; 3]
    ];
    
    let mut compiler = Compiler::new();
    
    for program in &test_programs {
        let hir = compiler.compile_to_hir(program)?;
        
        // After type inference, all types should be concrete
        hir.validate_invariants()?;
    }
    
    Ok(())
}

/// Test HIR validation with function calls
#[test]
fn test_hir_function_call_validation() -> OvieResult<()> {
    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        
        fn multiply(x: i32, y: i32) -> i32 {
            x * y
        }
        
        fn main() {
            let sum = add(1, 2);
            let product = multiply(sum, 3);
            print(product);
        }
    "#;
    
    let mut compiler = Compiler::new();
    let hir = compiler.compile_to_hir(program)?;
    
    // All function calls should be resolved
    hir.validate_invariants()?;
    
    Ok(())
}

/// Test HIR validation with struct operations
#[test]
fn test_hir_struct_validation() -> OvieResult<()> {
    let program = r#"
        struct Rectangle {
            width: f64,
            height: f64,
        }
        
        fn area(rect: Rectangle) -> f64 {
            rect.width * rect.height
        }
        
        fn main() {
            let rect = Rectangle { width: 10.0, height: 5.0 };
            let a = area(rect);
            print(a);
            
            // Field access
            print(rect.width);
            print(rect.height);
        }
    "#;
    
    let mut compiler = Compiler::new();
    let hir = compiler.compile_to_hir(program)?;
    
    // All struct operations should be resolved
    hir.validate_invariants()?;
    
    Ok(())
}

