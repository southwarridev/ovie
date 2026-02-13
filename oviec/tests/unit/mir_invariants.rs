//! MIR Invariant Tests
//! 
//! Tests for MIR validation according to Stage 2.3 compiler invariants.
//! These tests ensure that MIR nodes maintain their invariants throughout
//! the compilation pipeline.

use crate::{Compiler, OvieResult, OvieError};

/// Test that MIR validation passes for valid MIR structures
#[test]
fn test_mir_invariants_valid_structures() -> OvieResult<()> {
    let test_programs = [
        "let x = 42;",
        "fn main() { print(\"hello\"); }",
        "if true { print(\"yes\"); } else { print(\"no\"); }",
        "while x < 10 { x = x + 1; }",
        "for i in 0..10 { print(i); }",
        "let result = add(1, 2);",
        "return 42;",
    ];
    
    let mut compiler = Compiler::new();
    
    for program in &test_programs {
        let mir = compiler.compile_to_mir(program)?;
        
        // MIR validation should pass for all valid programs
        mir.validate().map_err(|e| OvieError::CompileError {
            message: format!("MIR validation failed for '{}': {}", program, e)
        })?;
        
        // MIR invariant validation should also pass
        mir.validate_invariants().map_err(|e| OvieError::CompileError {
            message: format!("MIR invariant validation failed for '{}': {}", program, e)
        })?;
    }
    
    Ok(())
}

/// Test that MIR has explicit control flow (invariant 2.3.2)
#[test]
fn test_mir_explicit_control_flow_invariant() -> OvieResult<()> {
    let test_programs = [
        "if true { print(\"yes\"); } else { print(\"no\"); }",
        "while x < 10 { x = x + 1; }",
        "for i in 0..10 { print(i); }",
        "match value { Some(x) => x, None => 0 }",
    ];
    
    let mut compiler = Compiler::new();
    
    for program in &test_programs {
        let mir = compiler.compile_to_mir(program)?;
        
        // MIR invariant validation includes checking for explicit control flow
        mir.validate_invariants().map_err(|e| OvieError::CompileError {
            message: format!("MIR explicit-control-flow invariant failed for '{}': {}", program, e)
        })?;
    }
    
    Ok(())
}

/// Test that MIR contains no high-level constructs (invariant 2.3.3)
#[test]
fn test_mir_no_high_level_constructs_invariant() -> OvieResult<()> {
    let test_programs = [
        "if true { print(\"yes\"); } else { print(\"no\"); }",
        "while x < 10 { x = x + 1; }",
        "for i in 0..10 { print(i); }",
        "let array = [1, 2, 3, 4, 5];",
        "struct Point { x: i32, y: i32 } let p = Point { x: 1, y: 2 };",
    ];
    
    let mut compiler = Compiler::new();
    
    for program in &test_programs {
        let mir = compiler.compile_to_mir(program)?;
        
        // MIR invariant validation includes checking for no high-level constructs
        mir.validate_invariants().map_err(|e| OvieError::CompileError {
            message: format!("MIR no-high-level-constructs invariant failed for '{}': {}", program, e)
        })?;
    }
    
    Ok(())
}

/// Test that MIR basic blocks are well-formed (invariant 2.3.4)
#[test]
fn test_mir_basic_block_invariant() -> OvieResult<()> {
    let test_programs = [
        "let x = 42; print(x);",
        "if true { print(\"yes\"); } else { print(\"no\"); }",
        "while x < 10 { x = x + 1; }",
        "fn add(a: i32, b: i32) -> i32 { a + b } fn main() { let result = add(1, 2); }",
    ];
    
    let mut compiler = Compiler::new();
    
    for program in &test_programs {
        let mir = compiler.compile_to_mir(program)?;
        
        // MIR invariant validation includes checking that basic blocks are well-formed
        mir.validate_invariants().map_err(|e| OvieError::CompileError {
            message: format!("MIR basic-block invariant failed for '{}': {}", program, e)
        })?;
    }
    
    Ok(())
}

/// Test MIR invariant validation with complex control flow
#[test]
fn test_comprehensive_mir_validation() -> OvieResult<()> {
    let complex_program = r#"
        fn fibonacci(n: i32) -> i32 {
            if n <= 1 {
                return n;
            }
            
            mut a = 0;
            mut b = 1;
            mut i = 2;
            
            while i <= n {
                let temp = a + b;
                a = b;
                b = temp;
                i = i + 1;
            }
            
            return b;
        }
        
        fn main() {
            for i in 0..10 {
                let fib = fibonacci(i);
                print(fib);
            }
            
            let result = if fibonacci(5) > 5 {
                "large"
            } else {
                "small"
            };
            
            print(result);
        }
    "#;
    
    let mut compiler = Compiler::new();
    let mir = compiler.compile_to_mir(complex_program)?;
    
    // Comprehensive validation
    mir.validate()?;
    mir.validate_invariants()?;
    
    Ok(())
}

/// Test MIR validation with nested control structures
#[test]
fn test_mir_nested_control_structures() -> OvieResult<()> {
    let program = r#"
        fn main() {
            mut i = 0;
            while i < 10 {
                if i % 2 == 0 {
                    print("even");
                    
                    mut j = 0;
                    while j < i {
                        print(j);
                        j = j + 1;
                    }
                } else {
                    print("odd");
                    
                    for k in 0..i {
                        if k > 5 {
                            break;
                        }
                        print(k);
                    }
                }
                
                i = i + 1;
            }
        }
    "#;
    
    let mut compiler = Compiler::new();
    let mir = compiler.compile_to_mir(program)?;
    
    // Nested structures should be properly lowered to basic blocks
    mir.validate_invariants()?;
    
    Ok(())
}

/// Test MIR validation with function calls and returns
#[test]
fn test_mir_function_calls_validation() -> OvieResult<()> {
    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }
        
        fn multiply(x: i32, y: i32) -> i32 {
            let result = x * y;
            return result;
        }
        
        fn complex_calculation(n: i32) -> i32 {
            let sum = add(n, 1);
            let product = multiply(sum, 2);
            
            if product > 10 {
                return product;
            } else {
                return add(product, 5);
            }
        }
        
        fn main() {
            let result = complex_calculation(5);
            print(result);
        }
    "#;
    
    let mut compiler = Compiler::new();
    let mir = compiler.compile_to_mir(program)?;
    
    // Function calls should be properly represented in MIR
    mir.validate_invariants()?;
    
    Ok(())
}

/// Test MIR validation with error handling
#[test]
fn test_mir_error_handling_validation() -> OvieResult<()> {
    let program = r#"
        fn divide(a: i32, b: i32) -> Result<i32, String> {
            if b == 0 {
                return Err("Division by zero");
            } else {
                return Ok(a / b);
            }
        }
        
        fn main() {
            let result = divide(10, 2);
            match result {
                Ok(value) => print(value),
                Err(error) => print(error),
            }
        }
    "#;
    
    let mut compiler = Compiler::new();
    let mir = compiler.compile_to_mir(program)?;
    
    // Error handling should be lowered to explicit control flow
    mir.validate_invariants()?;
    
    Ok(())
}

/// Test MIR control flow graph analysis
#[test]
fn test_mir_cfg_analysis() -> OvieResult<()> {
    let program = r#"
        fn main() {
            let x = 42;
            if x > 0 {
                print("positive");
            } else {
                print("non-positive");
            }
            print("done");
        }
    "#;
    
    let mut compiler = Compiler::new();
    let mir = compiler.compile_to_mir(program)?;
    
    // Test control flow graph analysis
    let cfg_analysis = mir.analyze_cfg()?;
    
    // Should have analysis for at least one function
    assert!(!cfg_analysis.function_analyses.is_empty());
    
    Ok(())
}

/// Test MIR report generation
#[test]
fn test_mir_report_generation() -> OvieResult<()> {
    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        
        fn main() {
            let result = add(1, 2);
            print(result);
        }
    "#;
    
    let mut compiler = Compiler::new();
    let mir = compiler.compile_to_mir(program)?;
    
    // Test report generation
    let report = mir.generate_ir_report()?;
    
    // Report should contain basic information
    assert!(report.contains("MIR Program Analysis Report"));
    assert!(report.contains("Functions:"));
    
    Ok(())
}

/// Test MIR DOT export for visualization
#[test]
fn test_mir_dot_export() -> OvieResult<()> {
    let program = r#"
        fn main() {
            let x = 42;
            print(x);
        }
    "#;
    
    let mut compiler = Compiler::new();
    let mir = compiler.compile_to_mir(program)?;
    
    // Test DOT export
    let dot = mir.to_dot()?;
    
    // DOT should contain basic graph structure
    assert!(dot.contains("digraph MIR"));
    assert!(dot.contains("Function: main"));
    
    Ok(())
}

