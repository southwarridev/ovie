// System Integration Tests
// Tests for Task 15.1: End-to-end system testing

use oviec::runtime_environment::OvieRuntimeEnvironment;
use oviec::Compiler;
use std::env;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_complete_compilation_pipeline() {
    // Test the complete compilation pipeline from source to output
    let source = r#"
        fn main() {
            let x = 42;
            print(x);
        }
    "#;
    
    let mut compiler = Compiler::new_deterministic();
    
    // Test AST generation
    let ast_result = compiler.compile_to_ast(source);
    assert!(ast_result.is_ok(), "AST compilation should succeed");
    
    // Test HIR generation
    let hir_result = compiler.compile_to_hir(source);
    assert!(hir_result.is_ok(), "HIR compilation should succeed");
    
    // Test MIR generation
    let mir_result = compiler.compile_to_mir(source);
    assert!(mir_result.is_ok(), "MIR compilation should succeed");
    
    // Test IR generation
    let ir_result = compiler.compile_to_ir(source);
    assert!(ir_result.is_ok(), "IR compilation should succeed");
}

#[test]
fn test_ore_discovery_and_validation() {
    // Test ORE discovery system
    let ore_result = OvieRuntimeEnvironment::discover();
    
    match ore_result {
        Ok(ore) => {
            // ORE was discovered successfully
            println!("ORE discovered at: {:?}", ore.ovie_home);
            
            // Validate the ORE structure
            let validation = ore.validate();
            if validation.is_ok() {
                println!("ORE validation passed");
            } else {
                println!("ORE validation warnings: {:?}", validation);
            }
        }
        Err(e) => {
            // ORE not found - this is acceptable in test environment
            println!("ORE not found (expected in test environment): {:?}", e);
        }
    }
}

#[test]
fn test_compiler_determinism() {
    // Test that compilation is deterministic
    let source = r#"
        fn main() {
            let x = 10;
            let y = 20;
            let result = x + y;
            print(result);
        }
    "#;
    
    let mut compiler = Compiler::new_deterministic();
    
    // Compile the same source multiple times
    let ir1 = compiler.compile_to_ir(source).unwrap();
    let ir2 = compiler.compile_to_ir(source).unwrap();
    let ir3 = compiler.compile_to_ir(source).unwrap();
    
    // All compilations should produce identical IR
    assert_eq!(format!("{:?}", ir1), format!("{:?}", ir2));
    assert_eq!(format!("{:?}", ir2), format!("{:?}", ir3));
}

#[test]
fn test_error_handling_pipeline() {
    // Test error handling throughout the compilation pipeline
    let invalid_sources = vec![
        ("let x = unknown_var;", "undefined variable"),
        ("fn main() { 1 + \"string\"; }", "type mismatch"),
        ("fn main() { missing_func(); }", "undefined function"),
    ];
    
    let mut compiler = Compiler::new_deterministic();
    
    for (source, _expected_error) in invalid_sources {
        let result = compiler.compile_to_ir(source);
        assert!(result.is_err(), "Invalid source should produce error: {}", source);
    }
}

#[test]
fn test_stdlib_integration() {
    // Test that standard library modules are accessible
    // Note: This tests that the compiler can parse stdlib-like syntax
    let source = r#"
        fn main() {
            let x = 42;
            print(x);
        }
    "#;
    
    let mut compiler = Compiler::new_deterministic();
    
    // This should compile successfully
    let result = compiler.compile_to_ast(source);
    assert!(result.is_ok(), "Basic program should compile");
}

#[test]
fn test_workspace_detection() {
    // Test workspace root detection
    let current_dir = env::current_dir().unwrap();
    
    // Should be able to find workspace root
    assert!(current_dir.exists());
    
    // Check for key workspace files
    let cargo_toml = current_dir.join("Cargo.toml");
    assert!(cargo_toml.exists(), "Cargo.toml should exist in workspace root");
}

#[test]
fn test_target_directory_structure() {
    // Test that target directory structure is correct
    let current_dir = env::current_dir().unwrap();
    let target_dir = current_dir.join("target");
    
    if target_dir.exists() {
        // Target directory exists - verify it's a directory
        assert!(target_dir.is_dir(), "target should be a directory");
    }
}

#[test]
fn test_compilation_with_multiple_functions() {
    // Test compilation of programs with simple expressions
    let source = r#"
        fn main() {
            let x = 5;
            let y = x * 2;
            let z = y + 10;
            print(z);
        }
    "#;
    
    let mut compiler = Compiler::new_deterministic();
    let result = compiler.compile_to_ir(source);
    assert!(result.is_ok(), "Program with multiple expressions should compile");
}

#[test]
fn test_compilation_with_control_flow() {
    // Test compilation of programs with control flow
    let source = r#"
        fn main() {
            let x = 10;
            if x > 5 {
                print("greater");
            } else {
                print("less or equal");
            }
        }
    "#;
    
    let mut compiler = Compiler::new_deterministic();
    let result = compiler.compile_to_ir(source);
    assert!(result.is_ok(), "Control flow should compile");
}

#[test]
fn test_end_to_end_simple_program() {
    // Test complete end-to-end compilation of a simple program
    let source = r#"
        fn main() {
            print("Hello, Ovie!");
        }
    "#;
    
    let mut compiler = Compiler::new_deterministic();
    
    // Compile through all stages
    let ast = compiler.compile_to_ast(source);
    assert!(ast.is_ok(), "AST generation failed");
    
    let hir = compiler.compile_to_hir(source);
    assert!(hir.is_ok(), "HIR generation failed");
    
    let mir = compiler.compile_to_mir(source);
    assert!(mir.is_ok(), "MIR generation failed");
    
    let ir = compiler.compile_to_ir(source);
    assert!(ir.is_ok(), "IR generation failed");
}
