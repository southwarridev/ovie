//! Unit tests for the Ovie code generation components

use crate::{Compiler, Backend, OvieResult};

/// Test WebAssembly code generation
pub fn test_wasm_codegen() -> OvieResult<()> {
    let mut compiler = Compiler::new_with_backend(Backend::Wasm);
    let source = r#"
        fn main() {
            let x = 42;
            print(x);
        }
    "#;
    
    let wasm_bytes = compiler.compile_to_wasm(source)?;
    
    // Verify WASM output
    assert!(!wasm_bytes.is_empty());
    assert_eq!(&wasm_bytes[0..4], b"\0asm"); // WASM magic number
    
    Ok(())
}

/// Test LLVM code generation
#[cfg(feature = "llvm")]
pub fn test_llvm_codegen() -> OvieResult<()> {
    let mut compiler = Compiler::new_with_backend(Backend::Llvm);
    let source = r#"
        fn main() {
            let x = 42;
            print(x);
        }
    "#;
    
    let llvm_ir = compiler.compile_to_llvm(source)?;
    
    // Verify LLVM IR output
    assert!(!llvm_ir.is_empty());
    assert!(llvm_ir.contains("define"));
    
    Ok(())
}

/// Test HIR generation
pub fn test_hir_generation() -> OvieResult<()> {
    let mut compiler = Compiler::new_with_backend(Backend::Hir);
    let source = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }
    "#;
    
    let hir = compiler.compile_to_hir(source)?;
    
    // Verify HIR structure
    assert!(!hir.items.is_empty());
    
    Ok(())
}

/// Test MIR generation
pub fn test_mir_generation() -> OvieResult<()> {
    let mut compiler = Compiler::new_with_backend(Backend::Mir);
    let source = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }
    "#;
    
    let mir = compiler.compile_to_mir(source)?;
    
    // Verify MIR structure
    assert!(!mir.functions.is_empty());
    
    Ok(())
}

/// Test deterministic code generation
pub fn test_deterministic_codegen() -> OvieResult<()> {
    let mut compiler = Compiler::new_deterministic();
    let source = r#"
        fn main() {
            let x = 42;
            print(x);
        }
    "#;
    
    // Compile twice and verify identical output
    let result1 = compiler.verify_build_reproducibility(source, Backend::Wasm)?;
    assert!(result1, "Build should be reproducible");
    
    Ok(())
}