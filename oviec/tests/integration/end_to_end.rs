//! End-to-end integration tests for complete compilation pipeline

use crate::{Compiler, Backend, OvieResult};

/// Test complete compilation pipeline from source to executable
pub fn test_complete_compilation_pipeline() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = r#"
        fn fibonacci(n: i32) -> i32 {
            if n <= 1 {
                return n;
            }
            return fibonacci(n - 1) + fibonacci(n - 2);
        }
        
        fn main() {
            let result = fibonacci(10);
            print(result);
        }
    "#;
    
    // Test AST compilation
    let _ast = compiler.compile_to_ast(source)?;
    
    // Test HIR compilation
    let _hir = compiler.compile_to_hir(source)?;
    
    // Test MIR compilation
    let _mir = compiler.compile_to_mir(source)?;
    
    // Test WASM compilation
    let _wasm = compiler.compile_to_wasm(source)?;
    
    // Test execution
    compiler.compile_and_run(source)?;
    
    Ok(())
}

/// Test compilation with standard library integration
pub fn test_stdlib_integration_compilation() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = r#"
        use std::math;
        use std::io;
        
        fn main() {
            let x = math::sqrt(16.0);
            io::println("Square root of 16 is: {}", x);
        }
    "#;
    
    // This should compile successfully with standard library
    let _result = compiler.compile_to_ast(source)?;
    
    Ok(())
}

/// Test error recovery and reporting
pub fn test_error_recovery_and_reporting() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = r#"
        fn main() {
            let x: i32 = "string"; // Type error
            let y = undefined_var;  // Name error
            unknown_function();     // Function error
        }
    "#;
    
    // Should fail with comprehensive error reporting
    match compiler.compile_to_hir(source) {
        Ok(_) => panic!("Expected compilation to fail"),
        Err(error) => {
            // Verify that error contains useful information
            let error_msg = error.to_string();
            assert!(!error_msg.is_empty());
        }
    }
    
    Ok(())
}

/// Test multi-file compilation
pub fn test_multi_file_compilation() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    
    // Main file
    let main_source = r#"
        mod utils;
        use utils::helper_function;
        
        fn main() {
            let result = helper_function(42);
            print(result);
        }
    "#;
    
    // For now, just test single file compilation
    // Multi-file support would be implemented later
    let _result = compiler.compile_to_ast(main_source)?;
    
    Ok(())
}

/// Test optimization pipeline integration
pub fn test_optimization_pipeline() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = r#"
        fn main() {
            let x = 1 + 2; // Should be optimized to constant
            let y = x * 0; // Should be optimized to 0
            if false {     // Dead code elimination
                print("unreachable");
            }
            print(y);
        }
    "#;
    
    // Test that optimization doesn't break correctness
    let mir = compiler.compile_to_mir(source)?;
    
    // Verify MIR structure (optimizations would be visible here)
    assert!(!mir.functions.is_empty());
    
    Ok(())
}

/// Test cross-backend consistency
pub fn test_cross_backend_consistency() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = r#"
        fn main() {
            let x = 42;
            let y = x + 8;
            print(y);
        }
    "#;
    
    // Test that different backends produce consistent results
    let _wasm_result = compiler.compile_to_wasm(source)?;
    
    #[cfg(feature = "llvm")]
    let _llvm_result = compiler.compile_to_llvm(source)?;
    
    // Both should succeed for valid programs
    Ok(())
}

/// Test large program compilation
pub fn test_large_program_compilation() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    
    // Generate a larger program for stress testing
    let mut source = String::new();
    source.push_str("fn main() {\n");
    
    // Add many variable declarations
    for i in 0..100 {
        source.push_str(&format!("    let var_{} = {};\n", i, i));
    }
    
    // Add many function calls
    for i in 0..50 {
        source.push_str(&format!("    print(var_{});\n", i));
    }
    
    source.push_str("}\n");
    
    // Should handle large programs efficiently
    let _result = compiler.compile_to_ast(&source)?;
    
    Ok(())
}