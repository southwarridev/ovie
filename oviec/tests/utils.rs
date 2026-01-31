//! Test utilities and helper functions

use crate::{Compiler, OvieResult};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Test execution context with timing and metadata
pub struct TestContext {
    pub name: String,
    pub start_time: Instant,
    pub metadata: HashMap<String, String>,
}

impl TestContext {
    /// Create a new test context
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            start_time: Instant::now(),
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to the test context
    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }

    /// Get elapsed time since test start
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
}

/// Test assertion utilities
pub struct TestAssert;

impl TestAssert {
    /// Assert that compilation succeeds
    pub fn compilation_succeeds(source: &str) -> OvieResult<()> {
        let mut compiler = Compiler::new();
        let _result = compiler.compile_to_ast(source)?;
        Ok(())
    }

    /// Assert that compilation fails
    pub fn compilation_fails(source: &str) -> OvieResult<()> {
        let mut compiler = Compiler::new();
        match compiler.compile_to_ast(source) {
            Ok(_) => Err(crate::OvieError::compile_error("Expected compilation to fail")),
            Err(_) => Ok(()),
        }
    }

    /// Assert that type checking succeeds
    pub fn type_checking_succeeds(source: &str) -> OvieResult<()> {
        let mut compiler = Compiler::new();
        let _result = compiler.compile_to_hir(source)?;
        Ok(())
    }

    /// Assert that type checking fails
    pub fn type_checking_fails(source: &str) -> OvieResult<()> {
        let mut compiler = Compiler::new();
        match compiler.compile_to_hir(source) {
            Ok(_) => Err(crate::OvieError::compile_error("Expected type checking to fail")),
            Err(_) => Ok(()),
        }
    }

    /// Assert that two compilation results are identical (deterministic)
    pub fn compilation_deterministic(source: &str) -> OvieResult<()> {
        let mut compiler = Compiler::new_deterministic();
        
        let result1 = compiler.compile_to_wasm(source)?;
        let result2 = compiler.compile_to_wasm(source)?;
        
        if result1 == result2 {
            Ok(())
        } else {
            Err(crate::OvieError::compile_error("Compilation is not deterministic"))
        }
    }
}

/// Test data generators for property-based testing
pub struct TestDataGenerator;

impl TestDataGenerator {
    /// Generate simple arithmetic expressions
    pub fn arithmetic_expressions() -> Vec<String> {
        vec![
            "1 + 2".to_string(),
            "3 * 4".to_string(),
            "10 - 5".to_string(),
            "8 / 2".to_string(),
            "(1 + 2) * 3".to_string(),
            "2 + 3 * 4".to_string(),
        ]
    }

    /// Generate variable declarations
    pub fn variable_declarations() -> Vec<String> {
        vec![
            "let x = 42;".to_string(),
            "let y: i32 = 100;".to_string(),
            "let z = true;".to_string(),
            "let s = \"hello\";".to_string(),
            "let f = 3.14;".to_string(),
        ]
    }

    /// Generate function declarations
    pub fn function_declarations() -> Vec<String> {
        vec![
            "fn main() {}".to_string(),
            "fn add(a: i32, b: i32) -> i32 { return a + b; }".to_string(),
            "fn greet(name: String) { print(name); }".to_string(),
            "fn factorial(n: i32) -> i32 { if n <= 1 { return 1; } else { return n * factorial(n - 1); } }".to_string(),
        ]
    }

    /// Generate control flow statements
    pub fn control_flow_statements() -> Vec<String> {
        vec![
            "if true { print(\"yes\"); }".to_string(),
            "if x > 0 { print(\"positive\"); } else { print(\"non-positive\"); }".to_string(),
            "while i < 10 { i = i + 1; }".to_string(),
            "for i in 0..10 { print(i); }".to_string(),
        ]
    }

    /// Generate type error cases
    pub fn type_error_cases() -> Vec<String> {
        vec![
            "let x: i32 = \"string\";".to_string(),
            "let y: bool = 42;".to_string(),
            "1 + \"hello\"".to_string(),
            "true * false".to_string(),
        ]
    }

    /// Generate syntax error cases
    pub fn syntax_error_cases() -> Vec<String> {
        vec![
            "let = 42;".to_string(),
            "fn () {}".to_string(),
            "if { print(\"hello\"); }".to_string(),
            "let x = ;".to_string(),
        ]
    }

    /// Generate all test cases
    pub fn all_test_cases() -> Vec<String> {
        let mut cases = Vec::new();
        cases.extend(Self::arithmetic_expressions());
        cases.extend(Self::variable_declarations());
        cases.extend(Self::function_declarations());
        cases.extend(Self::control_flow_statements());
        cases
    }
}

/// Performance measurement utilities
pub struct PerformanceMeasurement;

impl PerformanceMeasurement {
    /// Measure compilation time
    pub fn measure_compilation_time(source: &str) -> OvieResult<Duration> {
        let mut compiler = Compiler::new();
        let start = Instant::now();
        let _result = compiler.compile_to_ast(source)?;
        Ok(start.elapsed())
    }

    /// Measure memory usage during compilation
    pub fn measure_memory_usage(source: &str) -> OvieResult<u64> {
        let mut compiler = Compiler::new();
        
        // In a full implementation, this would measure actual memory usage
        // For now, return a placeholder
        let _result = compiler.compile_to_ast(source)?;
        Ok(0) // Placeholder memory usage
    }

    /// Compare performance between two implementations
    pub fn compare_performance<F1, F2>(name: &str, impl1: F1, impl2: F2) -> OvieResult<(Duration, Duration)>
    where
        F1: FnOnce() -> OvieResult<()>,
        F2: FnOnce() -> OvieResult<()>,
    {
        let start1 = Instant::now();
        impl1()?;
        let time1 = start1.elapsed();

        let start2 = Instant::now();
        impl2()?;
        let time2 = start2.elapsed();

        println!("Performance comparison for {}:", name);
        println!("  Implementation 1: {:.2}ms", time1.as_millis());
        println!("  Implementation 2: {:.2}ms", time2.as_millis());
        
        if time1 < time2 {
            println!("  Implementation 1 is {:.2}x faster", time2.as_secs_f64() / time1.as_secs_f64());
        } else {
            println!("  Implementation 2 is {:.2}x faster", time1.as_secs_f64() / time2.as_secs_f64());
        }

        Ok((time1, time2))
    }
}

/// Cross-platform testing utilities
pub struct CrossPlatformTesting;

impl CrossPlatformTesting {
    /// Test compilation across different targets
    pub fn test_cross_platform_compilation(source: &str) -> OvieResult<HashMap<String, bool>> {
        let mut results = HashMap::new();
        let mut compiler = Compiler::new();

        // Test WASM target
        match compiler.compile_to_wasm(source) {
            Ok(_) => { results.insert("wasm32-unknown-unknown".to_string(), true); }
            Err(_) => { results.insert("wasm32-unknown-unknown".to_string(), false); }
        }

        // Test LLVM target (if available)
        #[cfg(feature = "llvm")]
        {
            match compiler.compile_to_llvm(source) {
                Ok(_) => { results.insert("x86_64-pc-windows-gnu".to_string(), true); }
                Err(_) => { results.insert("x86_64-pc-windows-gnu".to_string(), false); }
            }
        }

        // Test interpreter
        match compiler.compile_and_run(source) {
            Ok(_) => { results.insert("interpreter".to_string(), true); }
            Err(_) => { results.insert("interpreter".to_string(), false); }
        }

        Ok(results)
    }

    /// Verify consistent behavior across platforms
    pub fn verify_cross_platform_consistency(source: &str) -> OvieResult<bool> {
        let results = Self::test_cross_platform_compilation(source)?;
        
        // All platforms should have the same result (all pass or all fail)
        let first_result = results.values().next().copied().unwrap_or(false);
        let all_consistent = results.values().all(|&result| result == first_result);
        
        Ok(all_consistent)
    }
}

/// Test result formatting utilities
pub struct TestResultFormatter;

impl TestResultFormatter {
    /// Format test results as a table
    pub fn format_results_table(results: &[(String, bool, Duration)]) -> String {
        let mut output = String::new();
        output.push_str("┌─────────────────────────────────┬────────┬──────────────┐\n");
        output.push_str("│ Test Name                       │ Status │ Duration     │\n");
        output.push_str("├─────────────────────────────────┼────────┼──────────────┤\n");
        
        for (name, passed, duration) in results {
            let status = if *passed { "PASS" } else { "FAIL" };
            let duration_str = format!("{:.2}ms", duration.as_millis());
            output.push_str(&format!("│ {:<31} │ {:<6} │ {:<12} │\n", 
                name, status, duration_str));
        }
        
        output.push_str("└─────────────────────────────────┴────────┴──────────────┘\n");
        output
    }

    /// Format summary statistics
    pub fn format_summary(total: usize, passed: usize, failed: usize, total_time: Duration) -> String {
        let success_rate = if total > 0 { (passed as f64 / total as f64) * 100.0 } else { 0.0 };
        
        format!(
            "Summary: {}/{} tests passed ({:.1}%), {} failed, total time: {:.2}s",
            passed, total, success_rate, failed, total_time.as_secs_f64()
        )
    }
}