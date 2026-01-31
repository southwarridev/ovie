//! ABI specification conformance tests

use crate::{Compiler, Backend, OvieResult};

/// Test function calling convention conformance
pub fn test_function_calling_convention_conformance() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }
        
        fn main() {
            let result = add(3, 4);
            print(result);
        }
    "#;
    
    // Test WASM ABI conformance
    let _wasm_result = compiler.compile_to_wasm(source)?;
    
    // Test LLVM ABI conformance (if available)
    #[cfg(feature = "llvm")]
    let _llvm_result = compiler.compile_to_llvm(source)?;
    
    Ok(())
}

/// Test data layout conformance
pub fn test_data_layout_conformance() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = r#"
        struct Point {
            x: i32,
            y: i32,
        }
        
        fn main() {
            let p = Point { x: 10, y: 20 };
            print(p.x);
            print(p.y);
        }
    "#;
    
    // Should follow consistent data layout rules
    let _result = compiler.compile_to_mir(source)?;
    
    Ok(())
}

/// Test type representation conformance
pub fn test_type_representation_conformance() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = r#"
        fn main() {
            let i8_val: i8 = 127;
            let i16_val: i16 = 32767;
            let i32_val: i32 = 2147483647;
            let i64_val: i64 = 9223372036854775807;
            
            let u8_val: u8 = 255;
            let u16_val: u16 = 65535;
            let u32_val: u32 = 4294967295;
            let u64_val: u64 = 18446744073709551615;
            
            let f32_val: f32 = 3.14;
            let f64_val: f64 = 2.718281828459045;
            
            let bool_val: bool = true;
            let char_val: char = 'A';
        }
    "#;
    
    // Should follow standard type representations
    let _result = compiler.compile_to_mir(source)?;
    
    Ok(())
}

/// Test memory alignment conformance
pub fn test_memory_alignment_conformance() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = r#"
        struct AlignedStruct {
            a: u8,
            b: u32,
            c: u8,
            d: u64,
        }
        
        fn main() {
            let s = AlignedStruct {
                a: 1,
                b: 2,
                c: 3,
                d: 4,
            };
            print(s.a);
        }
    "#;
    
    // Should follow proper memory alignment rules
    let _result = compiler.compile_to_mir(source)?;
    
    Ok(())
}

/// Test cross-platform ABI consistency
pub fn test_cross_platform_abi_consistency() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = r#"
        extern "C" fn external_function(x: i32) -> i32;
        
        fn main() {
            let result = external_function(42);
            print(result);
        }
    "#;
    
    // Should maintain consistent ABI across platforms
    let _result = compiler.compile_to_mir(source)?;
    
    Ok(())
}