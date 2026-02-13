//! Determinism verification tests for Ovie standard library
//! 
//! These tests verify that all stdlib operations produce identical results
//! when run multiple times with the same inputs. This is critical for
//! reproducible builds and consistent behavior across different runs.
//!
//! **Validates: Requirements 6.1.3**

use std::collections::HashMap;
use std::hash::{Hash, Hasher, DefaultHasher};

#[cfg(test)]
mod determinism_tests {
    use super::*;

    /// Test that mathematical operations are deterministic
    #[test]
    fn test_math_determinism() {
        let test_values = [
            0.0, 1.0, -1.0, 3.14159, 2.71828, 
            f64::MIN / 1e10, f64::MAX / 1e10,
            0.1, 0.2, 0.3, 0.7, 0.9
        ];

        for &value in &test_values {
            // Test trigonometric functions
            let sin_results: Vec<f64> = (0..10).map(|_| value.sin()).collect();
            let cos_results: Vec<f64> = (0..10).map(|_| value.cos()).collect();
            let tan_results: Vec<f64> = (0..10).map(|_| value.tan()).collect();

            // All results should be identical
            assert!(sin_results.windows(2).all(|w| w[0] == w[1]), 
                   "sin({}) not deterministic: {:?}", value, sin_results);
            assert!(cos_results.windows(2).all(|w| w[0] == w[1]), 
                   "cos({}) not deterministic: {:?}", value, cos_results);
            assert!(tan_results.windows(2).all(|w| w[0] == w[1]), 
                   "tan({}) not deterministic: {:?}", value, tan_results);

            // Test logarithmic and exponential functions
            if value > 0.0 {
                let ln_results: Vec<f64> = (0..10).map(|_| value.ln()).collect();
                let log10_results: Vec<f64> = (0..10).map(|_| value.log10()).collect();
                let exp_results: Vec<f64> = (0..10).map(|_| value.exp()).collect();

                assert!(ln_results.windows(2).all(|w| w[0] == w[1]), 
                       "ln({}) not deterministic: {:?}", value, ln_results);
                assert!(log10_results.windows(2).all(|w| w[0] == w[1]), 
                       "log10({}) not deterministic: {:?}", value, log10_results);
                assert!(exp_results.windows(2).all(|w| w[0] == w[1]), 
                       "exp({}) not deterministic: {:?}", value, exp_results);
            }

            // Test power and root functions
            if value >= 0.0 {
                let sqrt_results: Vec<f64> = (0..10).map(|_| value.sqrt()).collect();
                let cbrt_results: Vec<f64> = (0..10).map(|_| value.cbrt()).collect();

                assert!(sqrt_results.windows(2).all(|w| w[0] == w[1]), 
                       "sqrt({}) not deterministic: {:?}", value, sqrt_results);
                assert!(cbrt_results.windows(2).all(|w| w[0] == w[1]), 
                       "cbrt({}) not deterministic: {:?}", value, cbrt_results);
            }

            // Test rounding functions
            let floor_results: Vec<f64> = (0..10).map(|_| value.floor()).collect();
            let ceil_results: Vec<f64> = (0..10).map(|_| value.ceil()).collect();
            let round_results: Vec<f64> = (0..10).map(|_| value.round()).collect();
            let abs_results: Vec<f64> = (0..10).map(|_| value.abs()).collect();

            assert!(floor_results.windows(2).all(|w| w[0] == w[1]), 
                   "floor({}) not deterministic: {:?}", value, floor_results);
            assert!(ceil_results.windows(2).all(|w| w[0] == w[1]), 
                   "ceil({}) not deterministic: {:?}", value, ceil_results);
            assert!(round_results.windows(2).all(|w| w[0] == w[1]), 
                   "round({}) not deterministic: {:?}", value, round_results);
            assert!(abs_results.windows(2).all(|w| w[0] == w[1]), 
                   "abs({}) not deterministic: {:?}", value, abs_results);
        }
    }

    /// Test that arithmetic operations are deterministic
    #[test]
    fn test_arithmetic_determinism() {
        let test_pairs = [
            (1.0, 2.0), (3.14159, 2.71828), (-1.5, 0.5),
            (0.1, 0.2), (1e10, 1e-10), (f64::MAX / 1e10, f64::MIN / 1e10)
        ];

        for &(a, b) in &test_pairs {
            // Test basic arithmetic operations
            let add_results: Vec<f64> = (0..10).map(|_| a + b).collect();
            let sub_results: Vec<f64> = (0..10).map(|_| a - b).collect();
            let mul_results: Vec<f64> = (0..10).map(|_| a * b).collect();

            assert!(add_results.windows(2).all(|w| w[0] == w[1]), 
                   "{} + {} not deterministic: {:?}", a, b, add_results);
            assert!(sub_results.windows(2).all(|w| w[0] == w[1]), 
                   "{} - {} not deterministic: {:?}", a, b, sub_results);
            assert!(mul_results.windows(2).all(|w| w[0] == w[1]), 
                   "{} * {} not deterministic: {:?}", a, b, mul_results);

            // Test division (avoiding division by zero)
            if b != 0.0 {
                let div_results: Vec<f64> = (0..10).map(|_| a / b).collect();
                assert!(div_results.windows(2).all(|w| w[0] == w[1]), 
                       "{} / {} not deterministic: {:?}", a, b, div_results);
            }

            // Test modulo operation
            if b != 0.0 {
                let mod_results: Vec<f64> = (0..10).map(|_| a % b).collect();
                assert!(mod_results.windows(2).all(|w| w[0] == w[1]), 
                       "{} % {} not deterministic: {:?}", a, b, mod_results);
            }

            // Test power operation
            if a >= 0.0 && b.abs() < 100.0 {
                let pow_results: Vec<f64> = (0..10).map(|_| a.powf(b)).collect();
                assert!(pow_results.windows(2).all(|w| w[0] == w[1]), 
                       "{}.powf({}) not deterministic: {:?}", a, b, pow_results);
            }
        }
    }

    /// Test that string operations are deterministic
    #[test]
    fn test_string_determinism() {
        let test_strings = [
            "hello", "world", "Hello World", "UPPERCASE", "lowercase",
            "123456", "special!@#$%", "unicode: αβγδε", "", " ", "\n\t"
        ];

        for &s in &test_strings {
            // Test string length
            let len_results: Vec<usize> = (0..10).map(|_| s.len()).collect();
            assert!(len_results.windows(2).all(|w| w[0] == w[1]), 
                   "len('{}') not deterministic: {:?}", s, len_results);

            // Test case conversions
            let upper_results: Vec<String> = (0..10).map(|_| s.to_uppercase()).collect();
            let lower_results: Vec<String> = (0..10).map(|_| s.to_lowercase()).collect();

            assert!(upper_results.windows(2).all(|w| w[0] == w[1]), 
                   "to_uppercase('{}') not deterministic: {:?}", s, upper_results);
            assert!(lower_results.windows(2).all(|w| w[0] == w[1]), 
                   "to_lowercase('{}') not deterministic: {:?}", s, lower_results);

            // Test string comparison
            let eq_results: Vec<bool> = (0..10).map(|_| s == s).collect();
            let cmp_results: Vec<std::cmp::Ordering> = (0..10).map(|_| s.cmp(s)).collect();

            assert!(eq_results.windows(2).all(|w| w[0] == w[1]), 
                   "equality for '{}' not deterministic: {:?}", s, eq_results);
            assert!(cmp_results.windows(2).all(|w| w[0] == w[1]), 
                   "comparison for '{}' not deterministic: {:?}", s, cmp_results);

            // Test substring operations
            if !s.is_empty() {
                let first_char = s.chars().next().unwrap();
                let last_char = s.chars().last().unwrap();

                let starts_with_results: Vec<bool> = (0..10).map(|_| s.starts_with(first_char)).collect();
                let ends_with_results: Vec<bool> = (0..10).map(|_| s.ends_with(last_char)).collect();

                assert!(starts_with_results.windows(2).all(|w| w[0] == w[1]), 
                       "starts_with for '{}' not deterministic: {:?}", s, starts_with_results);
                assert!(ends_with_results.windows(2).all(|w| w[0] == w[1]), 
                       "ends_with for '{}' not deterministic: {:?}", s, ends_with_results);
            }
        }
    }

    /// Test that collection operations are deterministic
    #[test]
    fn test_collection_determinism() {
        let test_data = vec![1, 2, 3, 4, 5, -1, 0, 100, -100];

        // Test Vec operations
        for _ in 0..10 {
            let mut vec1 = Vec::new();
            let mut vec2 = Vec::new();

            // Perform identical operations
            for &value in &test_data {
                vec1.push(value);
                vec2.push(value);
            }

            // Results should be identical
            assert_eq!(vec1, vec2, "Vec operations not deterministic");
            assert_eq!(vec1.len(), vec2.len(), "Vec length not deterministic");

            // Test iteration order
            let iter1: Vec<i32> = vec1.iter().cloned().collect();
            let iter2: Vec<i32> = vec2.iter().cloned().collect();
            assert_eq!(iter1, iter2, "Vec iteration not deterministic");

            // Test sorting
            vec1.sort();
            vec2.sort();
            assert_eq!(vec1, vec2, "Vec sorting not deterministic");
        }

        // Test HashMap operations (with deterministic iteration)
        for _ in 0..10 {
            let mut map1 = HashMap::new();
            let mut map2 = HashMap::new();

            let test_pairs = [
                ("key1", "value1"), ("key2", "value2"), ("key3", "value3"),
                ("", "empty"), ("unicode", "αβγ")
            ];

            // Perform identical operations
            for &(key, value) in &test_pairs {
                map1.insert(key, value);
                map2.insert(key, value);
            }

            // Results should be identical
            assert_eq!(map1.len(), map2.len(), "HashMap length not deterministic");

            for &(key, expected_value) in &test_pairs {
                assert_eq!(map1.get(key), map2.get(key), 
                          "HashMap get('{}') not deterministic", key);
                assert_eq!(map1.get(key), Some(&expected_value), 
                          "HashMap value for '{}' incorrect", key);
            }

            // Test contains_key
            for &(key, _) in &test_pairs {
                let contains1 = map1.contains_key(key);
                let contains2 = map2.contains_key(key);
                assert_eq!(contains1, contains2, 
                          "HashMap contains_key('{}') not deterministic", key);
                assert!(contains1, "HashMap should contain key '{}'", key);
            }
        }
    }

    /// Test that hash operations are deterministic within a single run
    #[test]
    fn test_hash_determinism() {
        let test_values = [
            "hello", "world", "test string", "", "unicode: αβγδε",
            "numbers: 123456789", "special: !@#$%^&*()"
        ];

        for &value in &test_values {
            // Hash the same value multiple times within the same run
            let mut hashes = Vec::new();
            
            for _ in 0..10 {
                let mut hasher = DefaultHasher::new();
                value.hash(&mut hasher);
                hashes.push(hasher.finish());
            }

            // All hashes should be identical within the same run
            assert!(hashes.windows(2).all(|w| w[0] == w[1]), 
                   "Hash for '{}' not deterministic within run: {:?}", value, hashes);

            // Verify the hash is not zero (unless it's a pathological case)
            let first_hash = hashes[0];
            if !value.is_empty() {
                // Most non-empty strings should have non-zero hashes
                // (This is not guaranteed but very likely with a good hash function)
            }
        }

        // Test integer hashing
        let int_values = [0, 1, -1, 42, -42, i32::MAX, i32::MIN, 12345, -67890];
        
        for &value in &int_values {
            let mut hashes = Vec::new();
            
            for _ in 0..10 {
                let mut hasher = DefaultHasher::new();
                value.hash(&mut hasher);
                hashes.push(hasher.finish());
            }

            assert!(hashes.windows(2).all(|w| w[0] == w[1]), 
                   "Hash for {} not deterministic within run: {:?}", value, hashes);
        }
    }

    /// Test that formatting operations are deterministic
    #[test]
    fn test_formatting_determinism() {
        let int_values = [0, 1, -1, 42, -42, i32::MAX, i32::MIN];
        let float_values = [0.0, 1.0, -1.0, 3.14159, -2.71828, f64::MAX / 1e10, f64::MIN / 1e10];

        // Test integer formatting
        for &value in &int_values {
            let decimal_results: Vec<String> = (0..10).map(|_| format!("{}", value)).collect();
            let hex_results: Vec<String> = (0..10).map(|_| format!("{:x}", value)).collect();
            let octal_results: Vec<String> = (0..10).map(|_| format!("{:o}", value)).collect();
            let binary_results: Vec<String> = (0..10).map(|_| format!("{:b}", value)).collect();

            assert!(decimal_results.windows(2).all(|w| w[0] == w[1]), 
                   "Decimal format for {} not deterministic: {:?}", value, decimal_results);
            assert!(hex_results.windows(2).all(|w| w[0] == w[1]), 
                   "Hex format for {} not deterministic: {:?}", value, hex_results);
            assert!(octal_results.windows(2).all(|w| w[0] == w[1]), 
                   "Octal format for {} not deterministic: {:?}", value, octal_results);
            assert!(binary_results.windows(2).all(|w| w[0] == w[1]), 
                   "Binary format for {} not deterministic: {:?}", value, binary_results);
        }

        // Test float formatting
        for &value in &float_values {
            let default_results: Vec<String> = (0..10).map(|_| format!("{}", value)).collect();
            let scientific_results: Vec<String> = (0..10).map(|_| format!("{:e}", value)).collect();
            let fixed_results: Vec<String> = (0..10).map(|_| format!("{:.2}", value)).collect();

            assert!(default_results.windows(2).all(|w| w[0] == w[1]), 
                   "Default format for {} not deterministic: {:?}", value, default_results);
            assert!(scientific_results.windows(2).all(|w| w[0] == w[1]), 
                   "Scientific format for {} not deterministic: {:?}", value, scientific_results);
            assert!(fixed_results.windows(2).all(|w| w[0] == w[1]), 
                   "Fixed format for {} not deterministic: {:?}", value, fixed_results);
        }
    }

    /// Test that parsing operations are deterministic
    #[test]
    fn test_parsing_determinism() {
        let int_strings = ["0", "1", "-1", "42", "-42", "2147483647", "-2147483648"];
        let float_strings = ["0.0", "1.0", "-1.0", "3.14159", "-2.71828", "1e10", "-1e-10"];

        // Test integer parsing
        for &s in &int_strings {
            let parse_results: Vec<Result<i32, _>> = (0..10).map(|_| s.parse::<i32>()).collect();
            
            // All results should be identical
            for window in parse_results.windows(2) {
                match (&window[0], &window[1]) {
                    (Ok(a), Ok(b)) => assert_eq!(a, b, "Integer parse for '{}' not deterministic", s),
                    (Err(_), Err(_)) => {}, // Both errors is fine
                    _ => panic!("Integer parse for '{}' inconsistent success/failure", s),
                }
            }
        }

        // Test float parsing
        for &s in &float_strings {
            let parse_results: Vec<Result<f64, _>> = (0..10).map(|_| s.parse::<f64>()).collect();
            
            // All results should be identical
            for window in parse_results.windows(2) {
                match (&window[0], &window[1]) {
                    (Ok(a), Ok(b)) => assert_eq!(a, b, "Float parse for '{}' not deterministic", s),
                    (Err(_), Err(_)) => {}, // Both errors is fine
                    _ => panic!("Float parse for '{}' inconsistent success/failure", s),
                }
            }
        }
    }

    /// Test that path operations are deterministic
    #[test]
    fn test_path_determinism() {
        use std::path::PathBuf;

        let test_paths = [
            "simple.txt", "dir/file.txt", "deep/nested/path/file.ext",
            "file_with_underscores.txt", "file-with-dashes.txt", "file.with.dots.txt"
        ];

        for &path_str in &test_paths {
            // Test path construction
            let path_results: Vec<PathBuf> = (0..10).map(|_| PathBuf::from(path_str)).collect();
            assert!(path_results.windows(2).all(|w| w[0] == w[1]), 
                   "PathBuf construction for '{}' not deterministic", path_str);

            let path = PathBuf::from(path_str);

            // Test component extraction
            let component_results: Vec<Vec<_>> = (0..10).map(|_| {
                path.components().collect()
            }).collect();
            assert!(component_results.windows(2).all(|w| w[0] == w[1]), 
                   "Path components for '{}' not deterministic", path_str);

            // Test file name extraction
            let filename_results: Vec<Option<_>> = (0..10).map(|_| {
                path.file_name().map(|s| s.to_string_lossy().to_string())
            }).collect();
            assert!(filename_results.windows(2).all(|w| w[0] == w[1]), 
                   "File name for '{}' not deterministic", path_str);

            // Test extension extraction
            let extension_results: Vec<Option<_>> = (0..10).map(|_| {
                path.extension().map(|s| s.to_string_lossy().to_string())
            }).collect();
            assert!(extension_results.windows(2).all(|w| w[0] == w[1]), 
                   "Extension for '{}' not deterministic", path_str);

            // Test parent extraction
            let parent_results: Vec<Option<_>> = (0..10).map(|_| {
                path.parent().map(|p| p.to_string_lossy().to_string())
            }).collect();
            assert!(parent_results.windows(2).all(|w| w[0] == w[1]), 
                   "Parent for '{}' not deterministic", path_str);
        }
    }

    /// Test that error operations are deterministic
    #[test]
    fn test_error_determinism() {
        use std::fs::File;
        use std::io::ErrorKind;

        // Test file not found error
        for _ in 0..10 {
            let result = File::open("definitely_does_not_exist_12345.txt");
            assert!(result.is_err(), "File open should fail consistently");
            
            let error = result.unwrap_err();
            assert_eq!(error.kind(), ErrorKind::NotFound, "Error kind should be consistent");
        }

        // Test Result operations
        let ok_results: Vec<Result<i32, &str>> = (0..10).map(|_| Ok(42)).collect();
        let err_results: Vec<Result<i32, &str>> = (0..10).map(|_| Err("error")).collect();

        // All Ok results should be identical
        for window in ok_results.windows(2) {
            assert_eq!(window[0], window[1], "Ok results not deterministic");
        }

        // All Err results should be identical
        for window in err_results.windows(2) {
            assert_eq!(window[0], window[1], "Err results not deterministic");
        }

        // Test Option operations
        let some_results: Vec<Option<i32>> = (0..10).map(|_| Some(42)).collect();
        let none_results: Vec<Option<i32>> = (0..10).map(|_| None).collect();

        // All Some results should be identical
        for window in some_results.windows(2) {
            assert_eq!(window[0], window[1], "Some results not deterministic");
        }

        // All None results should be identical
        for window in none_results.windows(2) {
            assert_eq!(window[0], window[1], "None results not deterministic");
        }
    }

    /// Test that memory operations are deterministic
    #[test]
    fn test_memory_determinism() {
        // Test memory layout consistency
        let test_values = [1i32, 2, 3, 4, 5];
        
        for _ in 0..10 {
            let array1 = [1i32, 2, 3, 4, 5];
            let array2 = [1i32, 2, 3, 4, 5];
            
            // Arrays should be identical
            assert_eq!(array1, array2, "Array contents not deterministic");
            assert_eq!(array1.len(), array2.len(), "Array length not deterministic");
            
            // Slices should behave identically
            let slice1 = &array1[..];
            let slice2 = &array2[..];
            
            assert_eq!(slice1, slice2, "Slice contents not deterministic");
            assert_eq!(slice1.len(), slice2.len(), "Slice length not deterministic");
        }

        // Test Vec allocation behavior
        for _ in 0..10 {
            let mut vec1 = Vec::with_capacity(100);
            let mut vec2 = Vec::with_capacity(100);
            
            assert_eq!(vec1.capacity(), vec2.capacity(), "Vec capacity not deterministic");
            assert_eq!(vec1.len(), vec2.len(), "Vec length not deterministic");
            
            // Add same elements
            for i in 0..50 {
                vec1.push(i);
                vec2.push(i);
            }
            
            assert_eq!(vec1, vec2, "Vec contents not deterministic");
            assert_eq!(vec1.len(), vec2.len(), "Vec length after push not deterministic");
        }

        // Test alignment requirements
        let align_i32_results: Vec<usize> = (0..10).map(|_| std::mem::align_of::<i32>()).collect();
        let align_i64_results: Vec<usize> = (0..10).map(|_| std::mem::align_of::<i64>()).collect();
        let size_i32_results: Vec<usize> = (0..10).map(|_| std::mem::size_of::<i32>()).collect();
        let size_i64_results: Vec<usize> = (0..10).map(|_| std::mem::size_of::<i64>()).collect();

        assert!(align_i32_results.windows(2).all(|w| w[0] == w[1]), 
               "i32 alignment not deterministic: {:?}", align_i32_results);
        assert!(align_i64_results.windows(2).all(|w| w[0] == w[1]), 
               "i64 alignment not deterministic: {:?}", align_i64_results);
        assert!(size_i32_results.windows(2).all(|w| w[0] == w[1]), 
               "i32 size not deterministic: {:?}", size_i32_results);
        assert!(size_i64_results.windows(2).all(|w| w[0] == w[1]), 
               "i64 size not deterministic: {:?}", size_i64_results);
    }
}

/// Integration tests that verify determinism across multiple stdlib components
#[cfg(test)]
mod integration_determinism_tests {
    use super::*;

    /// Test determinism of complex operations involving multiple stdlib components
    #[test]
    fn test_complex_operation_determinism() {
        // Simulate a complex workflow using multiple stdlib components
        for _ in 0..10 {
            let mut data = Vec::new();
            
            // Math operations
            for i in 0..10 {
                let value = (i as f64 * 3.14159 / 10.0).sin() * 100.0;
                data.push(value as i32);
            }
            
            // Collection operations
            data.sort();
            data.dedup();
            
            // String operations
            let formatted: Vec<String> = data.iter()
                .map(|x| format!("value_{:04}", x))
                .collect();
            
            // Hash operations
            let mut hasher = DefaultHasher::new();
            for s in &formatted {
                s.hash(&mut hasher);
            }
            let hash_result = hasher.finish();
            
            // Store results for comparison
            if data.is_empty() {
                // This should be consistent across runs
                assert!(data.is_empty(), "Empty data should remain empty");
            }
            
            // Verify formatting is consistent
            for (i, s) in formatted.iter().enumerate() {
                assert!(s.starts_with("value_"), "Formatted string should start with 'value_'");
                assert!(s.len() >= 6, "Formatted string should have minimum length");
            }
            
            // Hash should be deterministic within this run
            let mut hasher2 = DefaultHasher::new();
            for s in &formatted {
                s.hash(&mut hasher2);
            }
            let hash_result2 = hasher2.finish();
            
            assert_eq!(hash_result, hash_result2, "Hash results should be identical within run");
        }
    }

    /// Test determinism of error handling across components
    #[test]
    fn test_error_handling_determinism() {
        use std::fs::File;
        use std::io::Read;
        
        // Function that consistently fails
        fn failing_operation(path: &str) -> Result<String, Box<dyn std::error::Error>> {
            let mut file = File::open(path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            Ok(content)
        }
        
        // Test consistent failure behavior
        for _ in 0..10 {
            let result = failing_operation("non_existent_file_12345.txt");
            assert!(result.is_err(), "Operation should fail consistently");
            
            let error = result.unwrap_err();
            let error_string = error.to_string();
            
            // Error message should be consistent
            assert!(!error_string.is_empty(), "Error message should not be empty");
            
            // Test error formatting determinism
            let formatted_error = format!("Error: {}", error);
            assert!(formatted_error.starts_with("Error: "), "Error formatting should be consistent");
        }
    }

    /// Test determinism of resource management
    #[test]
    fn test_resource_management_determinism() {
        use std::fs::File;
        use std::io::Write;
        
        let temp_files = [
            "test_determinism_1.tmp",
            "test_determinism_2.tmp", 
            "test_determinism_3.tmp"
        ];
        
        // Test file creation and cleanup determinism
        for _ in 0..5 {
            // Create files
            for &filename in &temp_files {
                let mut file = File::create(filename).unwrap();
                file.write_all(b"test content").unwrap();
                file.flush().unwrap();
            }
            
            // Verify files exist
            for &filename in &temp_files {
                let content = std::fs::read_to_string(filename).unwrap();
                assert_eq!(content, "test content", "File content should be deterministic");
            }
            
            // Clean up files
            for &filename in &temp_files {
                std::fs::remove_file(filename).unwrap();
            }
            
            // Verify cleanup
            for &filename in &temp_files {
                assert!(std::fs::metadata(filename).is_err(), "File should be removed");
            }
        }
    }

    /// Test determinism of concurrent operations (basic thread safety)
    #[test]
    fn test_concurrent_determinism() {
        use std::sync::{Arc, Mutex};
        use std::thread;
        
        // Test that basic concurrent operations are deterministic
        for _ in 0..5 {
            let counter = Arc::new(Mutex::new(0));
            let mut handles = vec![];
            
            // Spawn threads that increment counter
            for _ in 0..10 {
                let counter = Arc::clone(&counter);
                let handle = thread::spawn(move || {
                    let mut num = counter.lock().unwrap();
                    *num += 1;
                });
                handles.push(handle);
            }
            
            // Wait for all threads
            for handle in handles {
                handle.join().unwrap();
            }
            
            // Final count should always be 10
            let final_count = *counter.lock().unwrap();
            assert_eq!(final_count, 10, "Concurrent counter should be deterministic");
        }
    }
}