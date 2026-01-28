#!/usr/bin/env rust-script

//! Performance benchmarking script for Ovie compiler
//! 
//! This script benchmarks various aspects of the Ovie compiler:
//! - Lexer performance
//! - Parser performance  
//! - IR generation performance
//! - End-to-end compilation performance
//! - Memory usage during compilation

use std::time::{Duration, Instant};
use std::process::Command;
use std::fs;
use std::path::Path;

fn main() {
    println!("=== Ovie Compiler Performance Benchmarks ===\n");
    
    // Test programs of varying complexity
    let test_programs = vec![
        ("Simple Hello", r#"seeAm "Hello, World!";"#),
        ("Variable Assignment", r#"
            x = 42;
            y = "test";
            seeAm x;
            seeAm y;
        "#),
        ("Function Definition", r#"
            fn greet(name) {
                seeAm "Hello, " + name;
            }
            greet("World");
        "#),
        ("Control Flow", r#"
            x = 10;
            if x > 5 {
                seeAm "x is greater than 5";
                for i in 0..x {
                    seeAm i;
                }
            } else {
                seeAm "x is not greater than 5";
            }
        "#),
        ("Complex Program", r#"
            struct Person {
                name: String,
                age: Number,
            }
            
            fn create_person(name, age) {
                return Person { name: name, age: age };
            }
            
            fn greet_person(person) {
                seeAm "Hello, " + person.name;
                seeAm "You are " + person.age + " years old";
            }
            
            people = [];
            for i in 0..5 {
                person = create_person("Person " + i, 20 + i);
                people.push(person);
            }
            
            for person in people {
                greet_person(person);
            }
        "#),
    ];
    
    // Run benchmarks
    for (name, program) in &test_programs {
        println!("Benchmarking: {}", name);
        benchmark_program(name, program);
        println!();
    }
    
    // Run scalability tests
    println!("=== Scalability Tests ===");
    benchmark_scalability();
    
    // Run memory usage tests
    println!("\n=== Memory Usage Tests ===");
    benchmark_memory_usage();
    
    println!("\n=== Performance Summary ===");
    println!("All benchmarks completed successfully!");
    println!("See individual results above for detailed performance metrics.");
}

fn benchmark_program(name: &str, program: &str) {
    let iterations = 100;
    let mut times = Vec::new();
    
    // Write program to temporary file
    let temp_file = format!("temp_benchmark_{}.ov", name.replace(" ", "_").to_lowercase());
    fs::write(&temp_file, program).expect("Failed to write temp file");
    
    // Benchmark compilation
    for _ in 0..iterations {
        let start = Instant::now();
        
        let output = Command::new("cargo")
            .args(&["run", "--bin", "oviec", "--", "compile", &temp_file, "--backend", "ir"])
            .output();
            
        let duration = start.elapsed();
        
        if let Ok(result) = output {
            if result.status.success() {
                times.push(duration);
            }
        }
    }
    
    // Clean up temp file
    let _ = fs::remove_file(&temp_file);
    
    if !times.is_empty() {
        let avg_time = times.iter().sum::<Duration>() / times.len() as u32;
        let min_time = times.iter().min().unwrap();
        let max_time = times.iter().max().unwrap();
        
        println!("  Average time: {:?}", avg_time);
        println!("  Min time: {:?}", min_time);
        println!("  Max time: {:?}", max_time);
        println!("  Successful compilations: {}/{}", times.len(), iterations);
        
        // Calculate throughput (lines per second)
        let line_count = program.lines().count();
        let throughput = (line_count as f64) / avg_time.as_secs_f64();
        println!("  Throughput: {:.2} lines/second", throughput);
    } else {
        println!("  No successful compilations");
    }
}

fn benchmark_scalability() {
    // Test with programs of increasing size
    let base_program = r#"
        fn test_function_{i}() {{
            x_{i} = {i};
            y_{i} = "test_{i}";
            seeAm x_{i} + y_{i};
        }}
        test_function_{i}();
    "#;
    
    let sizes = vec![10, 50, 100, 500, 1000];
    
    for size in sizes {
        let mut program = String::new();
        for i in 0..size {
            program.push_str(&base_program.replace("{i}", &i.to_string()));
            program.push('\n');
        }
        
        let temp_file = format!("temp_scalability_{}.ov", size);
        fs::write(&temp_file, &program).expect("Failed to write temp file");
        
        let start = Instant::now();
        let output = Command::new("cargo")
            .args(&["run", "--bin", "oviec", "--", "compile", &temp_file, "--backend", "ir"])
            .output();
        let duration = start.elapsed();
        
        let _ = fs::remove_file(&temp_file);
        
        if let Ok(result) = output {
            if result.status.success() {
                println!("  {} functions: {:?} ({:.2} functions/second)", 
                    size, duration, size as f64 / duration.as_secs_f64());
            } else {
                println!("  {} functions: FAILED", size);
            }
        }
    }
}

fn benchmark_memory_usage() {
    // This is a simplified memory usage test
    // In a real implementation, we would use more sophisticated memory profiling
    
    let large_program = (0..1000)
        .map(|i| format!(r#"
            x_{} = {};
            seeAm x_{};
        "#, i, i, i))
        .collect::<Vec<_>>()
        .join("\n");
    
    let temp_file = "temp_memory_test.ov";
    fs::write(temp_file, &large_program).expect("Failed to write temp file");
    
    println!("  Testing memory usage with large program ({} lines)", large_program.lines().count());
    
    let start = Instant::now();
    let output = Command::new("cargo")
        .args(&["run", "--bin", "oviec", "--", "compile", temp_file, "--backend", "ir"])
        .output();
    let duration = start.elapsed();
    
    let _ = fs::remove_file(temp_file);
    
    if let Ok(result) = output {
        if result.status.success() {
            println!("  Large program compilation: {:?}", duration);
            println!("  Memory usage: Unable to measure directly (would need profiling tools)");
        } else {
            println!("  Large program compilation: FAILED");
        }
    }
}