//! Unit tests for the Ovie runtime system

use crate::{Compiler, Backend, OvieResult};

/// Test basic program execution
pub fn test_basic_execution() -> OvieResult<()> {
    let mut compiler = Compiler::new_with_backend(Backend::Interpreter);
    let source = r#"
        fn main() {
            let x = 42;
            print(x);
        }
    "#;
    
    compiler.compile_and_run(source)?;
    
    Ok(())
}

/// Test arithmetic operations
pub fn test_arithmetic_operations() -> OvieResult<()> {
    let mut compiler = Compiler::new_with_backend(Backend::Interpreter);
    let source = r#"
        fn main() {
            let a = 10;
            let b = 5;
            let sum = a + b;
            let diff = a - b;
            let prod = a * b;
            let quot = a / b;
            
            assert_eq(sum, 15);
            assert_eq(diff, 5);
            assert_eq(prod, 50);
            assert_eq(quot, 2);
        }
    "#;
    
    compiler.compile_and_run(source)?;
    
    Ok(())
}

/// Test control flow execution
pub fn test_control_flow_execution() -> OvieResult<()> {
    let mut compiler = Compiler::new_with_backend(Backend::Interpreter);
    let source = r#"
        fn main() {
            let x = 10;
            
            if x > 5 {
                print("x is greater than 5");
            } else {
                print("x is not greater than 5");
            }
            
            let mut i = 0;
            while i < 3 {
                print(i);
                i = i + 1;
            }
        }
    "#;
    
    compiler.compile_and_run(source)?;
    
    Ok(())
}

/// Test function calls
pub fn test_function_calls() -> OvieResult<()> {
    let mut compiler = Compiler::new_with_backend(Backend::Interpreter);
    let source = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }
        
        fn main() {
            let result = add(3, 4);
            assert_eq(result, 7);
        }
    "#;
    
    compiler.compile_and_run(source)?;
    
    Ok(())
}

/// Test error handling
pub fn test_error_handling() -> OvieResult<()> {
    let mut compiler = Compiler::new_with_backend(Backend::Interpreter);
    let source = r#"
        fn divide(a: i32, b: i32) -> Result<i32, String> {
            if b == 0 {
                return Err("Division by zero");
            }
            return Ok(a / b);
        }
        
        fn main() {
            match divide(10, 2) {
                Ok(result) => print(result),
                Err(error) => print(error),
            }
        }
    "#;
    
    compiler.compile_and_run(source)?;
    
    Ok(())
}