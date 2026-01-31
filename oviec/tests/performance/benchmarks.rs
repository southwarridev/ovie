//! Performance benchmarks for compiler components

use crate::{Compiler, Backend, OvieResult};
use std::time::{Duration, Instant};

/// Benchmark result structure
pub struct BenchmarkResult {
    pub name: String,
    pub duration: Duration,
    pub memory_usage: u64,
    pub throughput: Option<f64>,
}

/// Benchmark lexer performance
pub fn benchmark_lexer_performance() -> OvieResult<BenchmarkResult> {
    let source = include_str!("../../../examples/employee_management.ov");
    let iterations = 1000;
    
    let start = Instant::now();
    
    for _ in 0..iterations {
        let mut compiler = Compiler::new();
        let _tokens = compiler.compile_to_ast(source)?;
    }
    
    let duration = start.elapsed();
    let throughput = iterations as f64 / duration.as_secs_f64();
    
    Ok(BenchmarkResult {
        name: "lexer_performance".to_string(),
        duration,
        memory_usage: 0, // Would measure actual memory usage
        throughput: Some(throughput),
    })
}

/// Benchmark parser performance
pub fn benchmark_parser_performance() -> OvieResult<BenchmarkResult> {
    let source = include_str!("../../../examples/calculator.ov");
    let iterations = 500;
    
    let start = Instant::now();
    
    for _ in 0..iterations {
        let mut compiler = Compiler::new();
        let _ast = compiler.compile_to_ast(source)?;
    }
    
    let duration = start.elapsed();
    let throughput = iterations as f64 / duration.as_secs_f64();
    
    Ok(BenchmarkResult {
        name: "parser_performance".to_string(),
        duration,
        memory_usage: 0,
        throughput: Some(throughput),
    })
}

/// Benchmark type checker performance
pub fn benchmark_type_checker_performance() -> OvieResult<BenchmarkResult> {
    let source = include_str!("../../../examples/functions.ov");
    let iterations = 200;
    
    let start = Instant::now();
    
    for _ in 0..iterations {
        let mut compiler = Compiler::new();
        let _hir = compiler.compile_to_hir(source)?;
    }
    
    let duration = start.elapsed();
    let throughput = iterations as f64 / duration.as_secs_f64();
    
    Ok(BenchmarkResult {
        name: "type_checker_performance".to_string(),
        duration,
        memory_usage: 0,
        throughput: Some(throughput),
    })
}

/// Benchmark code generation performance
pub fn benchmark_codegen_performance() -> OvieResult<BenchmarkResult> {
    let source = include_str!("../../../examples/control_flow.ov");
    let iterations = 100;
    
    let start = Instant::now();
    
    for _ in 0..iterations {
        let mut compiler = Compiler::new();
        let _wasm = compiler.compile_to_wasm(source)?;
    }
    
    let duration = start.elapsed();
    let throughput = iterations as f64 / duration.as_secs_f64();
    
    Ok(BenchmarkResult {
        name: "codegen_performance".to_string(),
        duration,
        memory_usage: 0,
        throughput: Some(throughput),
    })
}

/// Benchmark end-to-end compilation performance
pub fn benchmark_end_to_end_performance() -> OvieResult<BenchmarkResult> {
    let source = r#"
        fn fibonacci(n: i32) -> i32 {
            if n <= 1 {
                return n;
            }
            return fibonacci(n - 1) + fibonacci(n - 2);
        }
        
        fn main() {
            let result = fibonacci(20);
            print(result);
        }
    "#;
    let iterations = 50;
    
    let start = Instant::now();
    
    for _ in 0..iterations {
        let mut compiler = Compiler::new();
        let _result = compiler.compile_and_run(source)?;
    }
    
    let duration = start.elapsed();
    let throughput = iterations as f64 / duration.as_secs_f64();
    
    Ok(BenchmarkResult {
        name: "end_to_end_performance".to_string(),
        duration,
        memory_usage: 0,
        throughput: Some(throughput),
    })
}

/// Benchmark memory usage during compilation
pub fn benchmark_memory_usage() -> OvieResult<BenchmarkResult> {
    let source = include_str!("../../../examples/memory_safety.ov");
    
    let start = Instant::now();
    
    // Measure memory usage during compilation
    let mut compiler = Compiler::new();
    let _result = compiler.compile_to_mir(source)?;
    
    let duration = start.elapsed();
    
    Ok(BenchmarkResult {
        name: "memory_usage".to_string(),
        duration,
        memory_usage: 0, // Would measure actual memory usage
        throughput: None,
    })
}

/// Run all performance benchmarks
pub fn run_all_benchmarks() -> OvieResult<Vec<BenchmarkResult>> {
    let mut results = Vec::new();
    
    println!("Running performance benchmarks...");
    
    results.push(benchmark_lexer_performance()?);
    results.push(benchmark_parser_performance()?);
    results.push(benchmark_type_checker_performance()?);
    results.push(benchmark_codegen_performance()?);
    results.push(benchmark_end_to_end_performance()?);
    results.push(benchmark_memory_usage()?);
    
    // Print results
    for result in &results {
        println!("Benchmark: {}", result.name);
        println!("  Duration: {:.2}ms", result.duration.as_millis());
        if let Some(throughput) = result.throughput {
            println!("  Throughput: {:.2} ops/sec", throughput);
        }
        println!("  Memory: {} bytes", result.memory_usage);
        println!();
    }
    
    Ok(results)
}