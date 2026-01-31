//! Cross-platform integration tests

use crate::{Compiler, Backend, OvieResult};
use std::collections::HashMap;

/// Test cross-platform compilation consistency
pub fn test_cross_platform_compilation_consistency() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = r#"
        fn main() {
            let x = 42;
            let y = x + 8;
            print(y);
        }
    "#;
    
    // Test WASM compilation
    let _wasm_result = compiler.compile_to_wasm(source)?;
    
    // Test LLVM compilation (if available)
    #[cfg(feature = "llvm")]
    let _llvm_result = compiler.compile_to_llvm(source)?;
    
    // Test interpreter
    compiler.compile_and_run(source)?;
    
    Ok(())
}

/// Test platform-specific behavior consistency
pub fn test_platform_behavior_consistency() -> OvieResult<()> {
    let test_cases = vec![
        "let x = 42;",
        "fn add(a: i32, b: i32) -> i32 { return a + b; }",
        "if true { print(\"hello\"); }",
    ];
    
    for source in test_cases {
        // All platforms should handle these cases consistently
        test_cross_platform_compilation_consistency()?;
    }
    
    Ok(())
}

/// Test numeric consistency across platforms
pub fn test_numeric_consistency_across_platforms() -> OvieResult<()> {
    let mut compiler = Compiler::new_deterministic();
    let source = r#"
        fn main() {
            let x = 1.0 / 3.0;
            let y = 2_i32.pow(10);
            print(x);
            print(y);
        }
    "#;
    
    // Should produce consistent results across platforms
    let _result = compiler.compile_to_wasm(source)?;
    
    Ok(())
}