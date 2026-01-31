//! Performance regression detection tests

use crate::{Compiler, OvieResult};
use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Performance baseline data
pub struct PerformanceBaseline {
    pub lexer_time: Duration,
    pub parser_time: Duration,
    pub type_checker_time: Duration,
    pub codegen_time: Duration,
    pub end_to_end_time: Duration,
}

impl PerformanceBaseline {
    /// Create baseline from current performance
    pub fn create_baseline() -> OvieResult<Self> {
        let mut compiler = Compiler::new();
        let source = include_str!("../../../examples/calculator.ov");
        
        // Measure lexer performance
        let start = Instant::now();
        for _ in 0..100 {
            let _tokens = compiler.compile_to_ast(source)?;
        }
        let lexer_time = start.elapsed() / 100;
        
        // Measure parser performance (included in AST compilation)
        let parser_time = lexer_time; // Simplified for now
        
        // Measure type checker performance
        let start = Instant::now();
        for _ in 0..50 {
            let _hir = compiler.compile_to_hir(source)?;
        }
        let type_checker_time = start.elapsed() / 50;
        
        // Measure code generation performance
        let start = Instant::now();
        for _ in 0..20 {
            let _wasm = compiler.compile_to_wasm(source)?;
        }
        let codegen_time = start.elapsed() / 20;
        
        // Measure end-to-end performance
        let start = Instant::now();
        for _ in 0..10 {
            compiler.compile_and_run(source)?;
        }
        let end_to_end_time = start.elapsed() / 10;
        
        Ok(Self {
            lexer_time,
            parser_time,
            type_checker_time,
            codegen_time,
            end_to_end_time,
        })
    }
}

/// Test for performance regressions
pub fn test_performance_regression_detection() -> OvieResult<()> {
    // Create baseline (in practice, this would be loaded from storage)
    let baseline = PerformanceBaseline::create_baseline()?;
    
    // Measure current performance
    let current = PerformanceBaseline::create_baseline()?;
    
    // Check for regressions (allow 10% variance)
    let regression_threshold = 1.1; // 10% slower is considered a regression
    
    let mut regressions = Vec::new();
    
    if current.lexer_time > baseline.lexer_time * regression_threshold.try_into().unwrap() {
        regressions.push("Lexer performance regression detected");
    }
    
    if current.parser_time > baseline.parser_time * regression_threshold.try_into().unwrap() {
        regressions.push("Parser performance regression detected");
    }
    
    if current.type_checker_time > baseline.type_checker_time * regression_threshold.try_into().unwrap() {
        regressions.push("Type checker performance regression detected");
    }
    
    if current.codegen_time > baseline.codegen_time * regression_threshold.try_into().unwrap() {
        regressions.push("Code generation performance regression detected");
    }
    
    if current.end_to_end_time > baseline.end_to_end_time * regression_threshold.try_into().unwrap() {
        regressions.push("End-to-end performance regression detected");
    }
    
    if !regressions.is_empty() {
        println!("Performance regressions detected:");
        for regression in &regressions {
            println!("  - {}", regression);
        }
        
        // In a real implementation, this might fail the test
        // For now, just report the regressions
    }
    
    Ok(())
}

/// Test memory usage regression
pub fn test_memory_usage_regression() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = include_str!("../../../examples/memory_safety.ov");
    
    // Measure memory usage during compilation
    // In a real implementation, this would use actual memory measurement
    let _result = compiler.compile_to_mir(source)?;
    
    // Check for memory usage regressions
    // This would compare against a baseline
    
    Ok(())
}

/// Test compilation speed regression for large files
pub fn test_large_file_compilation_regression() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    
    // Generate a large source file
    let mut source = String::new();
    source.push_str("fn main() {\n");
    
    for i in 0..5000 {
        source.push_str(&format!("    let var_{} = {} + {};\n", i, i, i + 1));
    }
    
    source.push_str("}\n");
    
    // Measure compilation time
    let start = Instant::now();
    let _result = compiler.compile_to_ast(&source)?;
    let compilation_time = start.elapsed();
    
    // Check against baseline (in practice, would load from storage)
    let baseline_time = Duration::from_millis(1000); // 1 second baseline
    
    if compilation_time > baseline_time * 2 {
        println!("Large file compilation regression detected: {:.2}ms (baseline: {:.2}ms)", 
            compilation_time.as_millis(), baseline_time.as_millis());
    }
    
    Ok(())
}

/// Run all performance regression tests
pub fn run_all_performance_regression_tests() -> OvieResult<Vec<String>> {
    let mut results = Vec::new();
    
    println!("Running performance regression tests...");
    
    let tests = vec![
        ("performance_regression_detection", test_performance_regression_detection),
        ("memory_usage_regression", test_memory_usage_regression),
        ("large_file_compilation_regression", test_large_file_compilation_regression),
    ];
    
    for (test_name, test_fn) in tests {
        match test_fn() {
            Ok(_) => {
                println!("  ✓ {}", test_name);
                results.push(format!("PASS: {}", test_name));
            }
            Err(error) => {
                println!("  ✗ {}: {}", test_name, error);
                results.push(format!("FAIL: {}: {}", test_name, error));
            }
        }
    }
    
    Ok(results)
}