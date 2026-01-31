//! Standard library specification conformance tests

use crate::{Compiler, OvieResult};

/// Test core module conformance
pub fn test_core_module_conformance() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = r#"
        use std::core;
        
        fn main() {
            let result: Result<i32, String> = Ok(42);
            let option: Option<i32> = Some(10);
            
            match result {
                Ok(value) => print(value),
                Err(error) => print(error),
            }
            
            match option {
                Some(value) => print(value),
                None => print("none"),
            }
        }
    "#;
    
    // Should compile successfully with core types
    let _result = compiler.compile_to_hir(source)?;
    
    Ok(())
}

/// Test math module conformance
pub fn test_math_module_conformance() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = r#"
        use std::math;
        
        fn main() {
            let x = math::sqrt(16.0);
            let y = math::pow(2.0, 3.0);
            let z = math::PI;
            
            print(x);
            print(y);
            print(z);
        }
    "#;
    
    // Should compile successfully with math functions
    let _result = compiler.compile_to_hir(source)?;
    
    Ok(())
}

/// Test I/O module conformance
pub fn test_io_module_conformance() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = r#"
        use std::io;
        
        fn main() {
            io::println("Hello, world!");
            io::print("Number: ");
            io::println(42);
        }
    "#;
    
    // Should compile successfully with I/O functions
    let _result = compiler.compile_to_hir(source)?;
    
    Ok(())
}

/// Test file system module conformance
pub fn test_fs_module_conformance() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = r#"
        use std::fs;
        
        fn main() {
            let content = fs::read_to_string("test.txt");
            match content {
                Ok(text) => print(text),
                Err(error) => print("File error"),
            }
        }
    "#;
    
    // Should compile successfully with file system functions
    let _result = compiler.compile_to_hir(source)?;
    
    Ok(())
}

/// Test testing module conformance
pub fn test_testing_module_conformance() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = r#"
        use std::testing;
        
        fn test_addition() {
            testing::assert_eq(2 + 2, 4);
            testing::assert_ne(2 + 2, 5);
            testing::assert(true);
        }
    "#;
    
    // Should compile successfully with testing functions
    let _result = compiler.compile_to_hir(source)?;
    
    Ok(())
}