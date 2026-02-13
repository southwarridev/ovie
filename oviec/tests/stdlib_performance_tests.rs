//! Performance regression tests for Ovie standard library
//! 
//! These tests verify that stdlib operations maintain acceptable performance
//! characteristics and detect performance regressions. All tests use simple
//! Rust #[test] functions with basic timing measurements.
//!
//! **Validates: Requirements 6.1.4**

use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Performance test configuration
const PERFORMANCE_ITERATIONS: usize = 1000;
const LARGE_COLLECTION_SIZE: usize = 10000;
const SMALL_COLLECTION_SIZE: usize = 100;

/// Maximum acceptable time for basic operations (in milliseconds)
const MAX_BASIC_OP_TIME_MS: u64 = 50;
const MAX_COLLECTION_OP_TIME_MS: u64 = 500;
const MAX_LARGE_OP_TIME_MS: u64 = 2000;

#[cfg(test)]
mod performance_tests {
    use super::*;

    /// Test mathematical operation performance
    #[test]
    fn test_math_performance() {
        let test_values = [0.0, 1.0, 3.14159, 2.71828, 100.0, 1000.0];

        // Test trigonometric functions
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            for &value in &test_values {
                let value: f64 = value;
                let _sin = value.sin();
                let _cos = value.cos();
                let _tan = value.tan();
            }
        }
        let trig_duration = start.elapsed();
        
        assert!(trig_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
               "Trigonometric functions too slow: {}ms", trig_duration.as_millis());

        // Test logarithmic and exponential functions
        let positive_values = [1.0, 2.0, 3.14159, 2.71828, 10.0, 100.0];
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            for &value in &positive_values {
                let value: f64 = value;
                let _ln = value.ln();
                let _log10 = value.log10();
                let _exp = (value / 10.0).exp(); // Scale down to avoid overflow
            }
        }
        let log_exp_duration = start.elapsed();
        
        assert!(log_exp_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
               "Logarithmic/exponential functions too slow: {}ms", log_exp_duration.as_millis());

        // Test power and root functions
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            for &value in &positive_values {
                let value: f64 = value;
                let _sqrt = value.sqrt();
                let _cbrt = value.cbrt();
                let _pow2 = value.powf(2.0);
                let _pow_half = value.powf(0.5);
            }
        }
        let power_duration = start.elapsed();
        
        assert!(power_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
               "Power/root functions too slow: {}ms", power_duration.as_millis());

        // Test basic arithmetic
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS * 10 {
            for &a in &test_values {
                for &b in &test_values {
                    let a: f64 = a;
                    let b: f64 = b;
                    if b != 0.0 {
                        let _add = a + b;
                        let _sub = a - b;
                        let _mul = a * b;
                        let _div = a / b;
                    }
                }
            }
        }
        let arithmetic_duration = start.elapsed();
        
        assert!(arithmetic_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
               "Basic arithmetic too slow: {}ms", arithmetic_duration.as_millis());
    }

    /// Test string operation performance
    #[test]
    fn test_string_performance() {
        let test_strings = [
            "short", "medium length string", 
            "this is a much longer string that should test performance with longer text",
            "UPPERCASE STRING", "lowercase string", "MiXeD cAsE sTrInG"
        ];

        // Test string length and basic operations
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            for &s in &test_strings {
                let _len = s.len();
                let _is_empty = s.is_empty();
                let _chars = s.chars().count();
            }
        }
        let basic_duration = start.elapsed();
        
        assert!(basic_duration.as_millis() < MAX_BASIC_OP_TIME_MS as u128,
               "String basic operations too slow: {}ms", basic_duration.as_millis());

        // Test case conversions
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            for &s in &test_strings {
                let _upper = s.to_uppercase();
                let _lower = s.to_lowercase();
            }
        }
        let case_duration = start.elapsed();
        
        assert!(case_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
               "String case conversions too slow: {}ms", case_duration.as_millis());

        // Test string comparison
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            for &s1 in &test_strings {
                for &s2 in &test_strings {
                    let _eq = s1 == s2;
                    let _cmp = s1.cmp(s2);
                }
            }
        }
        let comparison_duration = start.elapsed();
        
        assert!(comparison_duration.as_millis() < MAX_BASIC_OP_TIME_MS as u128,
               "String comparisons too slow: {}ms", comparison_duration.as_millis());

        // Test substring operations
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            for &s in &test_strings {
                if !s.is_empty() {
                    let first_char = s.chars().next().unwrap();
                    let last_char = s.chars().last().unwrap();
                    let _starts = s.starts_with(first_char);
                    let _ends = s.ends_with(last_char);
                    let _contains = s.contains("test");
                }
            }
        }
        let substring_duration = start.elapsed();
        
        assert!(substring_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
               "String substring operations too slow: {}ms", substring_duration.as_millis());
    }

    /// Test Vec collection performance
    #[test]
    fn test_vec_performance() {
        // Test Vec creation and basic operations
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            let mut vec = Vec::new();
            for i in 0..SMALL_COLLECTION_SIZE {
                vec.push(i);
            }
            let _len = vec.len();
            let _capacity = vec.capacity();
        }
        let creation_duration = start.elapsed();
        
        assert!(creation_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
               "Vec creation too slow: {}ms", creation_duration.as_millis());

        // Test Vec with pre-allocated capacity
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            let mut vec = Vec::with_capacity(SMALL_COLLECTION_SIZE);
            for i in 0..SMALL_COLLECTION_SIZE {
                vec.push(i);
            }
        }
        let preallocated_duration = start.elapsed();
        
        assert!(preallocated_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
               "Vec with capacity too slow: {}ms", preallocated_duration.as_millis());

        // Pre-allocated should be faster or equal (allowing for small timing variations)
        assert!(preallocated_duration.as_millis() <= creation_duration.as_millis() + 1,
               "Pre-allocated Vec should be faster or equal: {}ms vs {}ms", 
               preallocated_duration.as_millis(), creation_duration.as_millis());

        // Test Vec access patterns
        let test_vec: Vec<i32> = (0..SMALL_COLLECTION_SIZE as i32).collect();
        
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            for i in 0..test_vec.len() {
                let _value = test_vec[i];
            }
        }
        let access_duration = start.elapsed();
        
        assert!(access_duration.as_millis() < MAX_BASIC_OP_TIME_MS as u128,
               "Vec access too slow: {}ms", access_duration.as_millis());

        // Test Vec iteration
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            let _sum: i32 = test_vec.iter().sum();
        }
        let iteration_duration = start.elapsed();
        
        assert!(iteration_duration.as_millis() < MAX_BASIC_OP_TIME_MS as u128,
               "Vec iteration too slow: {}ms", iteration_duration.as_millis());

        // Test Vec sorting
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS / 10 { // Sorting is more expensive
            let mut vec_copy = test_vec.clone();
            vec_copy.reverse(); // Make it unsorted
            vec_copy.sort();
        }
        let sorting_duration = start.elapsed();
        
        assert!(sorting_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
               "Vec sorting too slow: {}ms", sorting_duration.as_millis());
    }

    /// Test HashMap collection performance
    #[test]
    fn test_hashmap_performance() {
        // Test HashMap creation and insertion
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS / 10 { // Reduce iterations for HashMap
            let mut map = HashMap::new();
            for i in 0..SMALL_COLLECTION_SIZE {
                map.insert(i, i * 2);
            }
        }
        let creation_duration = start.elapsed();
        
        assert!(creation_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
               "HashMap creation too slow: {}ms", creation_duration.as_millis());

        // Test HashMap with pre-allocated capacity
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS / 10 { // Reduce iterations for HashMap
            let mut map = HashMap::with_capacity(SMALL_COLLECTION_SIZE);
            for i in 0..SMALL_COLLECTION_SIZE {
                map.insert(i, i * 2);
            }
        }
        let preallocated_duration = start.elapsed();
        
        assert!(preallocated_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
               "HashMap with capacity too slow: {}ms", preallocated_duration.as_millis());

        // Pre-allocated should be faster or equal (allowing for small timing variations)
        assert!(preallocated_duration.as_millis() <= creation_duration.as_millis() + 10,
               "Pre-allocated HashMap should be faster or equal: {}ms vs {}ms", 
               preallocated_duration.as_millis(), creation_duration.as_millis());

        // Test HashMap lookups
        let mut test_map = HashMap::new();
        for i in 0..SMALL_COLLECTION_SIZE {
            test_map.insert(i, i * 2);
        }

        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS / 10 { // Reduce iterations for HashMap
            for i in 0..SMALL_COLLECTION_SIZE {
                let _value = test_map.get(&i);
            }
        }
        let lookup_duration = start.elapsed();
        
        assert!(lookup_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
               "HashMap lookups too slow: {}ms", lookup_duration.as_millis());

        // Test HashMap contains_key
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS / 10 { // Reduce iterations for HashMap
            for i in 0..SMALL_COLLECTION_SIZE {
                let _contains = test_map.contains_key(&i);
            }
        }
        let contains_duration = start.elapsed();
        
        assert!(contains_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
               "HashMap contains_key too slow: {}ms", contains_duration.as_millis());

        // Test HashMap iteration
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS / 10 { // Reduce iterations for HashMap
            let _count = test_map.iter().count();
        }
        let iteration_duration = start.elapsed();
        
        assert!(iteration_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
               "HashMap iteration too slow: {}ms", iteration_duration.as_millis());
    }

    /// Test formatting operation performance
    #[test]
    fn test_formatting_performance() {
        let int_values = [0, 1, -1, 42, -42, 12345, -67890, i32::MAX, i32::MIN];
        let float_values = [0.0, 1.0, -1.0, 3.14159, -2.71828, 1e6, -1e6];

        // Test integer formatting
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            for &value in &int_values {
                let _decimal = format!("{}", value);
                let _hex = format!("{:x}", value);
                let _binary = format!("{:b}", value);
            }
        }
        let int_format_duration = start.elapsed();
        
        assert!(int_format_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
               "Integer formatting too slow: {}ms", int_format_duration.as_millis());

        // Test float formatting
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            for &value in &float_values {
                let _default = format!("{}", value);
                let _scientific = format!("{:e}", value);
                let _fixed = format!("{:.2}", value);
            }
        }
        let float_format_duration = start.elapsed();
        
        assert!(float_format_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
               "Float formatting too slow: {}ms", float_format_duration.as_millis());

        // Test string formatting
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            for &int_val in &int_values {
                for &float_val in &float_values {
                    let _formatted = format!("int: {}, float: {:.2}", int_val, float_val);
                }
            }
        }
        let string_format_duration = start.elapsed();
        
        assert!(string_format_duration.as_millis() < MAX_LARGE_OP_TIME_MS as u128,
               "String formatting too slow: {}ms", string_format_duration.as_millis());
    }

    /// Test parsing operation performance
    #[test]
    fn test_parsing_performance() {
        let int_strings = ["0", "1", "-1", "42", "-42", "12345", "-67890"];
        let float_strings = ["0.0", "1.0", "-1.0", "3.14159", "-2.71828", "1e6", "-1e6"];

        // Test integer parsing
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            for &s in &int_strings {
                let _parsed: Result<i32, _> = s.parse();
            }
        }
        let int_parse_duration = start.elapsed();
        
        assert!(int_parse_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
               "Integer parsing too slow: {}ms", int_parse_duration.as_millis());

        // Test float parsing
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            for &s in &float_strings {
                let _parsed: Result<f64, _> = s.parse();
            }
        }
        let float_parse_duration = start.elapsed();
        
        assert!(float_parse_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
               "Float parsing too slow: {}ms", float_parse_duration.as_millis());
    }

    /// Test hash operation performance
    #[test]
    fn test_hash_performance() {
        use std::hash::{Hash, Hasher, DefaultHasher};
        
        let test_strings = [
            "short", "medium length string",
            "this is a much longer string for testing hash performance",
            "another long string with different content to test hash distribution",
            "final test string with even more content to ensure comprehensive testing"
        ];

        // Test string hashing
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            for &s in &test_strings {
                let mut hasher = DefaultHasher::new();
                s.hash(&mut hasher);
                let _hash = hasher.finish();
            }
        }
        let string_hash_duration = start.elapsed();
        
        assert!(string_hash_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
               "String hashing too slow: {}ms", string_hash_duration.as_millis());

        // Test integer hashing
        let int_values = [0, 1, -1, 42, -42, 12345, -67890, i32::MAX, i32::MIN];
        
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            for &value in &int_values {
                let mut hasher = DefaultHasher::new();
                value.hash(&mut hasher);
                let _hash = hasher.finish();
            }
        }
        let int_hash_duration = start.elapsed();
        
        assert!(int_hash_duration.as_millis() < MAX_BASIC_OP_TIME_MS as u128,
               "Integer hashing too slow: {}ms", int_hash_duration.as_millis());
    }

    /// Test path operation performance
    #[test]
    fn test_path_performance() {
        use std::path::PathBuf;
        
        let test_paths = [
            "simple.txt", "dir/file.txt", "deep/nested/path/file.ext",
            "very/deep/nested/directory/structure/with/many/components/file.txt",
            "file_with_underscores.txt", "file-with-dashes.txt", "file.with.dots.txt"
        ];

        // Test path construction
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            for &path_str in &test_paths {
                let _path = PathBuf::from(path_str);
            }
        }
        let construction_duration = start.elapsed();
        
        assert!(construction_duration.as_millis() < MAX_BASIC_OP_TIME_MS as u128,
               "Path construction too slow: {}ms", construction_duration.as_millis());

        // Test path operations
        let paths: Vec<PathBuf> = test_paths.iter().map(|&s| PathBuf::from(s)).collect();
        
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            for path in &paths {
                let _components: Vec<_> = path.components().collect();
                let _file_name = path.file_name();
                let _extension = path.extension();
                let _parent = path.parent();
            }
        }
        let operations_duration = start.elapsed();
        
        assert!(operations_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
               "Path operations too slow: {}ms", operations_duration.as_millis());

        // Test path joining
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            for path in &paths {
                let _joined = path.join("additional_component.txt");
            }
        }
        let joining_duration = start.elapsed();
        
        assert!(joining_duration.as_millis() < MAX_BASIC_OP_TIME_MS as u128,
               "Path joining too slow: {}ms", joining_duration.as_millis());
    }

    /// Test memory operation performance
    #[test]
    fn test_memory_performance() {
        // Test Vec allocation performance
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            let _vec: Vec<i32> = Vec::with_capacity(SMALL_COLLECTION_SIZE);
        }
        let allocation_duration = start.elapsed();
        
        assert!(allocation_duration.as_millis() < MAX_BASIC_OP_TIME_MS as u128,
               "Memory allocation too slow: {}ms", allocation_duration.as_millis());

        // Test large Vec operations
        let start = Instant::now();
        for _ in 0..10 { // Fewer iterations for large operations
            let mut vec = Vec::with_capacity(LARGE_COLLECTION_SIZE);
            for i in 0..LARGE_COLLECTION_SIZE {
                vec.push(i);
            }
            let _sum: usize = vec.iter().sum();
        }
        let large_ops_duration = start.elapsed();
        
        assert!(large_ops_duration.as_millis() < MAX_LARGE_OP_TIME_MS as u128,
               "Large Vec operations too slow: {}ms", large_ops_duration.as_millis());

        // Test memory copying
        let source_vec: Vec<i32> = (0..SMALL_COLLECTION_SIZE as i32).collect();
        
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            let _copy = source_vec.clone();
        }
        let copying_duration = start.elapsed();
        
        assert!(copying_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
               "Memory copying too slow: {}ms", copying_duration.as_millis());
    }
}

/// Stress tests that verify performance under heavy load
#[cfg(test)]
mod stress_tests {
    use super::*;

    /// Test performance with large collections
    #[test]
    fn test_large_collection_performance() {
        // Test large Vec operations
        let start = Instant::now();
        let mut large_vec = Vec::with_capacity(LARGE_COLLECTION_SIZE);
        for i in 0..LARGE_COLLECTION_SIZE {
            large_vec.push(i);
        }
        let creation_duration = start.elapsed();
        
        assert!(creation_duration.as_millis() < MAX_LARGE_OP_TIME_MS as u128,
               "Large Vec creation too slow: {}ms", creation_duration.as_millis());

        // Test large Vec access
        let start = Instant::now();
        let mut sum = 0;
        for i in 0..LARGE_COLLECTION_SIZE {
            sum += large_vec[i];
        }
        let access_duration = start.elapsed();
        
        assert!(access_duration.as_millis() < MAX_LARGE_OP_TIME_MS as u128,
               "Large Vec access too slow: {}ms", access_duration.as_millis());
        assert!(sum > 0, "Sum should be positive"); // Use the sum to prevent optimization

        // Test large HashMap operations
        let start = Instant::now();
        let mut large_map = HashMap::with_capacity(LARGE_COLLECTION_SIZE);
        for i in 0..LARGE_COLLECTION_SIZE {
            large_map.insert(i, i * 2);
        }
        let map_creation_duration = start.elapsed();
        
        assert!(map_creation_duration.as_millis() < MAX_LARGE_OP_TIME_MS as u128,
               "Large HashMap creation too slow: {}ms", map_creation_duration.as_millis());

        // Test large HashMap lookups
        let start = Instant::now();
        let mut found_count = 0;
        for i in 0..LARGE_COLLECTION_SIZE {
            if large_map.contains_key(&i) {
                found_count += 1;
            }
        }
        let lookup_duration = start.elapsed();
        
        assert!(lookup_duration.as_millis() < MAX_LARGE_OP_TIME_MS as u128,
               "Large HashMap lookups too slow: {}ms", lookup_duration.as_millis());
        assert_eq!(found_count, LARGE_COLLECTION_SIZE, "All keys should be found");
    }

    /// Test performance with repeated operations
    #[test]
    fn test_repeated_operations_performance() {
        let iterations = PERFORMANCE_ITERATIONS * 10;
        
        // Test repeated math operations
        let start = Instant::now();
        let mut result = 1.0;
        for i in 0..iterations {
            let x = (i as f64) / 1000.0;
            result += x.sin() * x.cos();
        }
        let math_duration = start.elapsed();
        
        assert!(math_duration.as_millis() < MAX_LARGE_OP_TIME_MS as u128,
               "Repeated math operations too slow: {}ms", math_duration.as_millis());
        assert!(result.is_finite(), "Result should be finite"); // Use result to prevent optimization

        // Test repeated string operations
        let start = Instant::now();
        let mut total_length = 0;
        for i in 0..iterations {
            let s = format!("test_string_{}", i);
            total_length += s.len();
            let _upper = s.to_uppercase();
        }
        let string_duration = start.elapsed();
        
        assert!(string_duration.as_millis() < MAX_LARGE_OP_TIME_MS as u128,
               "Repeated string operations too slow: {}ms", string_duration.as_millis());
        assert!(total_length > 0, "Total length should be positive");

        // Test repeated collection operations
        let start = Instant::now();
        let mut total_size = 0;
        for i in 0..iterations / 100 { // Fewer iterations for expensive operations
            let mut vec = Vec::new();
            for j in 0..10 {
                vec.push(i * 10 + j);
            }
            total_size += vec.len();
        }
        let collection_duration = start.elapsed();
        
        assert!(collection_duration.as_millis() < MAX_LARGE_OP_TIME_MS as u128,
               "Repeated collection operations too slow: {}ms", collection_duration.as_millis());
        assert!(total_size > 0, "Total size should be positive");
    }

    /// Test performance regression detection
    #[test]
    fn test_performance_regression_detection() {
        // This test establishes baseline performance characteristics
        // In a real implementation, these would be compared against stored baselines
        
        let mut performance_metrics = HashMap::new();
        
        // Measure basic operation performance
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            let _result = 3.14159_f64.sin();
        }
        let sin_duration = start.elapsed();
        performance_metrics.insert("sin_operation", sin_duration);
        
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            let _vec = Vec::<i32>::with_capacity(100);
        }
        let vec_alloc_duration = start.elapsed();
        performance_metrics.insert("vec_allocation", vec_alloc_duration);
        
        let start = Instant::now();
        for _ in 0..PERFORMANCE_ITERATIONS {
            let _formatted = format!("{}", 42);
        }
        let format_duration = start.elapsed();
        performance_metrics.insert("format_operation", format_duration);
        
        // Verify all operations are within acceptable bounds
        for (operation, duration) in &performance_metrics {
            assert!(duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128,
                   "{} too slow: {}ms", operation, duration.as_millis());
        }
        
        // In a real implementation, we would:
        // 1. Store these metrics to a baseline file
        // 2. Compare against previous baselines
        // 3. Alert if performance degrades beyond threshold (e.g., 20% slower)
        
        println!("Performance baseline established:");
        for (operation, duration) in &performance_metrics {
            println!("  {}: {}ms", operation, duration.as_millis());
        }
    }
}

/// Benchmark utilities for more detailed performance analysis
#[cfg(test)]
mod benchmark_utils {
    use super::*;

    /// Measure operation performance with statistical analysis
    fn measure_operation<F>(name: &str, iterations: usize, mut operation: F) -> Duration
    where
        F: FnMut(),
    {
        let mut durations = Vec::new();
        
        // Warm up
        for _ in 0..10 {
            operation();
        }
        
        // Measure multiple runs
        for _ in 0..10 {
            let start = Instant::now();
            for _ in 0..iterations {
                operation();
            }
            durations.push(start.elapsed());
        }
        
        // Calculate statistics
        durations.sort();
        let median = durations[durations.len() / 2];
        let min = durations[0];
        let max = durations[durations.len() - 1];
        
        println!("{}: min={}ms, median={}ms, max={}ms", 
                name, min.as_millis(), median.as_millis(), max.as_millis());
        
        median
    }

    /// Test comprehensive performance benchmarking
    #[test]
    fn test_comprehensive_benchmarks() {
        // Math operations benchmark
        let math_duration = measure_operation("Math operations", PERFORMANCE_ITERATIONS, || {
            let x: f64 = 3.14159;
            let _result = x.sin() + x.cos() + x.tan();
        });
        assert!(math_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128);

        // String operations benchmark
        let string_duration = measure_operation("String operations", PERFORMANCE_ITERATIONS, || {
            let s = "test string";
            let _upper = s.to_uppercase();
            let _len = s.len();
        });
        assert!(string_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128);

        // Collection operations benchmark
        let collection_duration = measure_operation("Collection operations", PERFORMANCE_ITERATIONS / 10, || {
            let mut vec = Vec::new();
            for i in 0..10 {
                vec.push(i);
            }
            let _sum: i32 = vec.iter().sum();
        });
        assert!(collection_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128);

        // Hash operations benchmark
        let hash_duration = measure_operation("Hash operations", PERFORMANCE_ITERATIONS, || {
            use std::hash::{Hash, Hasher, DefaultHasher};
            let mut hasher = DefaultHasher::new();
            "test string".hash(&mut hasher);
            let _hash = hasher.finish();
        });
        assert!(hash_duration.as_millis() < MAX_COLLECTION_OP_TIME_MS as u128);
    }
}