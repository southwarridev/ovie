//! Performance integration tests

use crate::{Compiler, OvieResult};
use std::time::{Duration, Instant};

/// Test compilation performance under load
pub fn test_compilation_performance_under_load() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = include_str!("../../../examples/employee_management.ov");
    
    let iterations = 100;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _result = compiler.compile_to_ast(source)?;
    }
    
    let duration = start.elapsed();
    let avg_time = duration / iterations;
    
    println!("Average compilation time: {:.2}ms", avg_time.as_millis());
    
    // Performance should be reasonable (less than 1 second per compilation)
    assert!(avg_time < Duration::from_secs(1));
    
    Ok(())
}

/// Test memory usage during large compilation
pub fn test_memory_usage_large_compilation() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    
    // Generate a large program
    let mut source = String::new();
    source.push_str("fn main() {\n");
    
    for i in 0..1000 {
        source.push_str(&format!("    let var_{} = {};\n", i, i));
    }
    
    source.push_str("}\n");
    
    // Should handle large programs without excessive memory usage
    let _result = compiler.compile_to_ast(&source)?;
    
    Ok(())
}

/// Test concurrent compilation performance
pub fn test_concurrent_compilation_performance() -> OvieResult<()> {
    use std::thread;
    use std::sync::Arc;
    
    let source = Arc::new(r#"
        fn fibonacci(n: i32) -> i32 {
            if n <= 1 {
                return n;
            }
            return fibonacci(n - 1) + fibonacci(n - 2);
        }
    "#.to_string());
    
    let mut handles = Vec::new();
    
    // Spawn multiple compilation threads
    for _ in 0..4 {
        let source_clone = Arc::clone(&source);
        let handle = thread::spawn(move || {
            let mut compiler = Compiler::new();
            compiler.compile_to_ast(&source_clone)
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        let _result = handle.join().unwrap()?;
    }
    
    Ok(())
}