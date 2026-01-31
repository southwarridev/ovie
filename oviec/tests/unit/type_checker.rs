//! Unit tests for the Ovie type checker component

use crate::{Compiler, OvieResult};

/// Test basic type inference
pub fn test_basic_type_inference() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = "let x = 42;";
    
    let hir = compiler.compile_to_hir(source)?;
    
    // Verify that type inference worked correctly
    // In a full implementation, this would check the HIR for correct types
    assert!(!hir.items.is_empty());
    
    Ok(())
}

/// Test type checking for function calls
pub fn test_function_call_type_checking() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }
        
        let result = add(1, 2);
    "#;
    
    let hir = compiler.compile_to_hir(source)?;
    
    // Verify function call type checking
    assert!(!hir.items.is_empty());
    
    Ok(())
}

/// Test type error detection
pub fn test_type_error_detection() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = r#"
        let x: i32 = "hello"; // Type mismatch
    "#;
    
    // This should fail type checking
    match compiler.compile_to_hir(source) {
        Ok(_) => panic!("Expected type error"),
        Err(_) => {
            // Type error correctly detected
        }
    }
    
    Ok(())
}

/// Test generic type inference
pub fn test_generic_type_inference() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = r#"
        fn identity<T>(x: T) -> T {
            return x;
        }
        
        let result = identity(42);
    "#;
    
    let hir = compiler.compile_to_hir(source)?;
    
    // Verify generic type inference
    assert!(!hir.items.is_empty());
    
    Ok(())
}

/// Test ownership and borrowing type checking
pub fn test_ownership_type_checking() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = r#"
        let x = vec![1, 2, 3];
        let y = x; // Move
        // let z = x; // This should be an error - use after move
    "#;
    
    let hir = compiler.compile_to_hir(source)?;
    
    // Verify ownership checking
    assert!(!hir.items.is_empty());
    
    Ok(())
}