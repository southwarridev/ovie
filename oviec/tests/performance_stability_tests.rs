// Performance and Stability Tests
// Tests for Task 15.3: Performance and stability testing

use oviec::Compiler;
use std::time::Instant;

#[test]
fn test_compilation_performance() {
    // Test that compilation completes in reasonable time
    let source = r#"
        fn main() {
            let a = 1;
            let b = 2;
            let c = a + b;
            let d = c * 3;
            let e = d - 1;
            let f = e + 10;
            let g = f * 2;
            let h = g + 5;
            let result = h - 3;
            print(result);
        }
    "#;
    
    let mut compiler = Compiler::new_deterministic();
    
    let start = Instant::now();
    let result = compiler.compile_to_ir(source);
    let duration = start.elapsed();
    
    assert!(result.is_ok(), "Compilation should succeed");
    assert!(duration.as_secs() < 5, "Compilation should complete within 5 seconds");
    
    println!("Compilation took: {:?}", duration);
}

#[test]
fn test_repeated_compilation_stability() {
    // Test that repeated compilations don't degrade performance
    let source = r#"
        fn main() {
            let x = 10;
            let y = 20;
            let result = x + y;
            print(result);
        }
    "#;
    
    let mut compiler = Compiler::new_deterministic();
    let iterations = 100;
    
    let mut durations = Vec::new();
    
    for _ in 0..iterations {
        let start = Instant::now();
        let result = compiler.compile_to_ir(source);
        let duration = start.elapsed();
        
        assert!(result.is_ok(), "Each compilation should succeed");
        durations.push(duration);
    }
    
    // Calculate average duration
    let total: std::time::Duration = durations.iter().sum();
    let average = total / iterations as u32;
    
    println!("Average compilation time over {} iterations: {:?}", iterations, average);
    
    // Count how many compilations are within reasonable bounds (using microseconds to avoid 0 values)
    let avg_micros = average.as_micros().max(1); // Ensure at least 1 to avoid division by zero
    let reasonable_count = durations.iter()
        .filter(|d| {
            let micros = d.as_micros();
            micros < avg_micros * 50 // Very lenient threshold for Windows
        })
        .count();
    
    // At least 90% of compilations should be within reasonable bounds
    let threshold = (iterations as f64 * 0.90) as usize;
    assert!(
        reasonable_count >= threshold,
        "At least 90% of compilations should be within reasonable bounds (got {}/{}, avg: {}µs)",
        reasonable_count,
        iterations,
        avg_micros
    );
}

#[test]
fn test_large_program_compilation() {
    // Test compilation of a larger program
    let mut source = String::from("fn main() {\n");
    
    // Generate a program with many statements
    for i in 0..100 {
        source.push_str(&format!("    let x{} = {};\n", i, i));
    }
    
    source.push_str("}\n");
    
    let mut compiler = Compiler::new_deterministic();
    
    let start = Instant::now();
    let result = compiler.compile_to_ir(&source);
    let duration = start.elapsed();
    
    assert!(result.is_ok(), "Large program should compile");
    assert!(duration.as_secs() < 10, "Large program should compile within 10 seconds");
    
    println!("Large program compilation took: {:?}", duration);
}

#[test]
fn test_deeply_nested_expressions() {
    // Test compilation of deeply nested expressions
    let mut source = String::from("fn main() {\n    let x = ");
    
    // Create deeply nested expression: ((((1 + 1) + 1) + 1) + 1)...
    for _ in 0..20 {
        source.push('(');
    }
    source.push_str("1");
    for _ in 0..20 {
        source.push_str(" + 1)");
    }
    source.push_str(";\n}\n");
    
    let mut compiler = Compiler::new_deterministic();
    let result = compiler.compile_to_ir(&source);
    
    assert!(result.is_ok(), "Deeply nested expressions should compile");
}

#[test]
fn test_many_functions_compilation() {
    // Test compilation of programs with many statements
    let mut source = String::from("fn main() {\n");
    
    // Generate many statements
    for i in 0..50 {
        source.push_str(&format!("    let var{} = {};\n", i, i * 2));
    }
    
    source.push_str("    print(var0);\n");
    source.push_str("}\n");
    
    let mut compiler = Compiler::new_deterministic();
    
    let start = Instant::now();
    let result = compiler.compile_to_ir(&source);
    let duration = start.elapsed();
    
    assert!(result.is_ok(), "Program with many statements should compile");
    assert!(duration.as_secs() < 10, "Should compile within 10 seconds");
    
    println!("Many statements compilation took: {:?}", duration);
}

#[test]
fn test_compilation_memory_stability() {
    // Test that repeated compilations don't leak memory
    // (This is a basic test - proper memory profiling would require external tools)
    let source = r#"
        fn main() {
            let a = 1;
            let b = 2;
            let c = 3;
            let d = 4;
            let e = 5;
            let sum = a + b + c + d + e;
            print(sum);
        }
    "#;
    
    let mut compiler = Compiler::new_deterministic();
    
    // Compile many times
    for i in 0..1000 {
        let result = compiler.compile_to_ir(source);
        assert!(result.is_ok(), "Compilation {} should succeed", i);
    }
    
    // If we got here without crashing or running out of memory, the test passes
    println!("Completed 1000 compilations without memory issues");
}

#[test]
fn test_error_recovery_stability() {
    // Test that error handling doesn't cause instability
    let invalid_sources = vec![
        "fn main() { let x = ; }",
        "fn main() { unknown_function(); }",
        "fn main() { 1 + \"string\"; }",
        "fn main() { let x: UnknownType = 1; }",
        "fn { }",
    ];
    
    let mut compiler = Compiler::new_deterministic();
    
    for source in invalid_sources {
        let result = compiler.compile_to_ir(source);
        assert!(result.is_err(), "Invalid source should produce error");
        
        // Compiler should still be usable after error
        let valid_source = "fn main() { print(42); }";
        let valid_result = compiler.compile_to_ir(valid_source);
        assert!(valid_result.is_ok(), "Compiler should recover from errors");
    }
}

#[test]
fn test_concurrent_compilation_safety() {
    // Test that multiple compilers can work independently
    use std::thread;
    
    let source = r#"
        fn main() {
            let x = 5;
            let y = x * x;
            let z = y + x;
            print(z);
        }
    "#;
    
    let handles: Vec<_> = (0..4)
        .map(|i| {
            let source = source.to_string();
            thread::spawn(move || {
                let mut compiler = Compiler::new_deterministic();
                let result = compiler.compile_to_ir(&source);
                assert!(result.is_ok(), "Thread {} compilation should succeed", i);
                result
            })
        })
        .collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }
    
    println!("Concurrent compilation test passed");
}

#[test]
fn test_compilation_speed_baseline() {
    // Establish a baseline for compilation speed
    let test_cases = vec![
        ("empty", "fn main() { }"),
        ("simple", "fn main() { print(42); }"),
        ("arithmetic", "fn main() { let x = 1 + 2 * 3; print(x); }"),
        ("multiple_vars", "fn main() { let a = 1; let b = 2; let c = a + b; print(c); }"),
    ];
    
    let mut compiler = Compiler::new_deterministic();
    
    for (name, source) in test_cases {
        let start = Instant::now();
        let result = compiler.compile_to_ir(source);
        let duration = start.elapsed();
        
        assert!(result.is_ok(), "{} should compile", name);
        println!("{} compilation: {:?}", name, duration);
        
        // All should complete quickly
        assert!(duration.as_millis() < 1000, "{} should compile in under 1 second", name);
    }
}

#[test]
fn test_deterministic_performance() {
    // Test that performance is consistent across runs
    let source = r#"
        fn main() {
            let n = 8;
            let a = n * 7;
            let b = a * 6;
            let c = b * 5;
            let d = c * 4;
            let e = d * 3;
            let f = e * 2;
            let result = f * 1;
            print(result);
        }
    "#;
    
    let mut compiler = Compiler::new_deterministic();
    let runs = 10;
    let mut durations = Vec::new();
    
    for _ in 0..runs {
        let start = Instant::now();
        let result = compiler.compile_to_ir(source);
        let duration = start.elapsed();
        
        assert!(result.is_ok(), "Compilation should succeed");
        durations.push(duration.as_micros());
    }
    
    // Calculate statistics
    let sum: u128 = durations.iter().sum();
    let avg = sum / runs as u128;
    let max = *durations.iter().max().unwrap();
    let min = *durations.iter().min().unwrap();
    
    println!("Performance stats (microseconds):");
    println!("  Average: {}", avg);
    println!("  Min: {}", min);
    println!("  Max: {}", max);
    println!("  Range: {}", max - min);
    
    // Performance should be relatively consistent
    // Max should not be more than 20x min (very lenient for Windows system variance)
    let min_threshold = min.max(1); // Avoid division by zero
    assert!(
        max < min_threshold * 20,
        "Performance should be reasonably consistent (max: {}, min: {})",
        max,
        min
    );
}
