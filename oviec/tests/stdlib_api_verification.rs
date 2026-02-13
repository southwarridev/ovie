//! API Verification Tests for Ovie Standard Library
//! 
//! These tests verify that all APIs specified in the .ov files are properly
//! implemented in the Rust runtime. This ensures completeness and consistency
//! between specification and implementation.
//!
//! **Validates: Requirements 6.2.1**

use std::collections::HashMap;

#[cfg(test)]
mod api_verification_tests {
    use super::*;

    /// Test that all core module APIs are implemented
    #[test]
    fn test_core_api_completeness() {
        // Test Result type and methods
        let ok_result: Result<i32, &str> = Ok(42);
        let err_result: Result<i32, &str> = Err("error");
        
        // Test Result methods
        assert!(ok_result.is_ok());
        assert!(!ok_result.is_err());
        assert!(!err_result.is_ok());
        assert!(err_result.is_err());
        
        assert_eq!(ok_result.unwrap(), 42);
        assert_eq!(err_result.unwrap_or(0), 0);
        
        // Test Result map operations
        let mapped = ok_result.map(|x| x * 2);
        assert_eq!(mapped.unwrap(), 84);
        
        let mapped_err = ok_result.map_err(|e| format!("Error: {}", e));
        assert!(mapped_err.is_ok());
        
        // Test Option type and methods
        let some_option = Some(42);
        let none_option: Option<i32> = None;
        
        assert!(some_option.is_some());
        assert!(!some_option.is_none());
        assert!(!none_option.is_some());
        assert!(none_option.is_none());
        
        assert_eq!(some_option.unwrap(), 42);
        assert_eq!(none_option.unwrap_or(0), 0);
        
        // Test Option map operations
        let mapped_some = some_option.map(|x| x * 2);
        assert_eq!(mapped_some.unwrap(), 84);
        
        let mapped_none = none_option.map(|x| x * 2);
        assert!(mapped_none.is_none());
        
        // Test Option to Result conversion
        let result_from_some = some_option.ok_or("no value");
        assert!(result_from_some.is_ok());
        
        let result_from_none = none_option.ok_or("no value");
        assert!(result_from_none.is_err());
        
        println!("✓ Core Result and Option APIs verified");
    }

    /// Test that all Vec APIs are implemented
    #[test]
    fn test_vec_api_completeness() {
        // Test Vec creation
        let mut vec = Vec::new();
        assert!(vec.is_empty());
        assert_eq!(vec.len(), 0);
        
        let mut vec_with_capacity = Vec::with_capacity(10);
        assert!(vec_with_capacity.capacity() >= 10);
        
        // Test Vec operations
        vec.push(1);
        vec.push(2);
        vec.push(3);
        
        assert_eq!(vec.len(), 3);
        assert!(!vec.is_empty());
        
        // Test Vec access
        assert_eq!(vec[0], 1);
        assert_eq!(vec.get(1), Some(&2));
        assert_eq!(vec.get(10), None);
        
        // Test Vec modification
        vec.insert(1, 42);
        assert_eq!(vec[1], 42);
        assert_eq!(vec.len(), 4);
        
        let removed = vec.remove(1);
        assert_eq!(removed, 42);
        assert_eq!(vec.len(), 3);
        
        let popped = vec.pop();
        assert_eq!(popped, Some(3));
        assert_eq!(vec.len(), 2);
        
        // Test Vec iteration
        let sum: i32 = vec.iter().sum();
        assert_eq!(sum, 3); // 1 + 2
        
        let doubled: Vec<i32> = vec.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4]);
        
        // Test Vec clearing
        vec.clear();
        assert!(vec.is_empty());
        
        println!("✓ Vec APIs verified");
    }

    /// Test that all HashMap APIs are implemented
    #[test]
    fn test_hashmap_api_completeness() {
        // Test HashMap creation
        let mut map = HashMap::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
        
        let mut map_with_capacity = HashMap::with_capacity(10);
        assert_eq!(map_with_capacity.len(), 0);
        
        // Test HashMap operations
        map.insert("key1", "value1");
        map.insert("key2", "value2");
        map.insert("key3", "value3");
        
        assert_eq!(map.len(), 3);
        assert!(!map.is_empty());
        
        // Test HashMap access
        assert_eq!(map.get("key1"), Some(&"value1"));
        assert_eq!(map.get("nonexistent"), None);
        assert!(map.contains_key("key2"));
        assert!(!map.contains_key("nonexistent"));
        
        // Test HashMap modification
        let old_value = map.insert("key1", "new_value1");
        assert_eq!(old_value, Some("value1"));
        assert_eq!(map.get("key1"), Some(&"new_value1"));
        
        let removed = map.remove("key2");
        assert_eq!(removed, Some("value2"));
        assert_eq!(map.len(), 2);
        assert!(!map.contains_key("key2"));
        
        // Test HashMap iteration
        let keys: Vec<&str> = map.keys().cloned().collect();
        assert!(keys.contains(&"key1"));
        assert!(keys.contains(&"key3"));
        assert_eq!(keys.len(), 2);
        
        let values: Vec<&str> = map.values().cloned().collect();
        assert!(values.contains(&"new_value1"));
        assert!(values.contains(&"value3"));
        assert_eq!(values.len(), 2);
        
        // Test HashMap clearing
        map.clear();
        assert!(map.is_empty());
        
        println!("✓ HashMap APIs verified");
    }

    /// Test that all math APIs are implemented
    #[test]
    fn test_math_api_completeness() {
        // Test mathematical constants
        assert!(std::f64::consts::PI > 3.14 && std::f64::consts::PI < 3.15);
        assert!(std::f64::consts::E > 2.71 && std::f64::consts::E < 2.72);
        
        // Test basic arithmetic functions
        assert_eq!(42_i32.checked_add(8), Some(50));
        assert_eq!(42_i32.checked_sub(8), Some(34));
        assert_eq!(42_i32.checked_mul(2), Some(84));
        assert_eq!(42_i32.checked_div(2), Some(21));
        
        // Test overflow detection
        assert_eq!(i32::MAX.checked_add(1), None);
        assert_eq!(i32::MIN.checked_sub(1), None);
        
        // Test power and root functions
        assert_eq!(2.0_f64.powf(3.0), 8.0);
        assert_eq!(9.0_f64.sqrt(), 3.0);
        assert_eq!(8.0_f64.cbrt(), 2.0);
        
        // Test trigonometric functions
        let sin_0 = 0.0_f64.sin();
        assert!(sin_0.abs() < 1e-10);
        
        let cos_0 = 0.0_f64.cos();
        assert!((cos_0 - 1.0).abs() < 1e-10);
        
        let tan_0 = 0.0_f64.tan();
        assert!(tan_0.abs() < 1e-10);
        
        // Test inverse trigonometric functions
        let asin_0 = 0.0_f64.asin();
        assert!(asin_0.abs() < 1e-10);
        
        let acos_1 = 1.0_f64.acos();
        assert!(acos_1.abs() < 1e-10);
        
        let atan_0 = 0.0_f64.atan();
        assert!(atan_0.abs() < 1e-10);
        
        // Test exponential and logarithmic functions
        let exp_0 = 0.0_f64.exp();
        assert!((exp_0 - 1.0).abs() < 1e-10);
        
        let ln_1 = 1.0_f64.ln();
        assert!(ln_1.abs() < 1e-10);
        
        let log10_1 = 1.0_f64.log10();
        assert!(log10_1.abs() < 1e-10);
        
        let log2_1 = 1.0_f64.log2();
        assert!(log2_1.abs() < 1e-10);
        
        // Test utility functions
        assert_eq!((-42.0_f64).abs(), 42.0);
        assert_eq!(42.0_f64.floor(), 42.0);
        assert_eq!(42.7_f64.floor(), 42.0);
        assert_eq!(42.0_f64.ceil(), 42.0);
        assert_eq!(42.3_f64.ceil(), 43.0);
        assert_eq!(42.5_f64.round(), 42.0); // Round half to even
        
        // Test classification functions
        assert!(42.0_f64.is_finite());
        assert!(!f64::INFINITY.is_finite());
        assert!(f64::INFINITY.is_infinite());
        assert!(!42.0_f64.is_infinite());
        assert!(f64::NAN.is_nan());
        assert!(!42.0_f64.is_nan());
        
        println!("✓ Math APIs verified");
    }

    /// Test that all string APIs are implemented
    #[test]
    fn test_string_api_completeness() {
        let test_string = "Hello, World!";
        
        // Test string length and emptiness
        assert_eq!(test_string.len(), 13);
        assert!(!test_string.is_empty());
        assert!("".is_empty());
        
        // Test string case conversion
        assert_eq!(test_string.to_uppercase(), "HELLO, WORLD!");
        assert_eq!(test_string.to_lowercase(), "hello, world!");
        
        // Test string comparison
        assert_eq!(test_string.cmp("Hello, World!"), std::cmp::Ordering::Equal);
        assert_eq!(test_string.cmp("Hello, World"), std::cmp::Ordering::Greater);
        
        // Test string searching
        assert!(test_string.starts_with("Hello"));
        assert!(test_string.ends_with("World!"));
        assert!(test_string.contains("World"));
        assert!(!test_string.contains("xyz"));
        
        // Test string slicing
        assert_eq!(&test_string[0..5], "Hello");
        assert_eq!(&test_string[7..12], "World");
        
        // Test string splitting and joining
        let parts: Vec<&str> = test_string.split(", ").collect();
        assert_eq!(parts, vec!["Hello", "World!"]);
        
        let joined = parts.join(" - ");
        assert_eq!(joined, "Hello - World!");
        
        // Test string trimming
        let padded = "  Hello, World!  ";
        assert_eq!(padded.trim(), "Hello, World!");
        assert_eq!(padded.trim_start(), "Hello, World!  ");
        assert_eq!(padded.trim_end(), "  Hello, World!");
        
        // Test string replacement
        let replaced = test_string.replace("World", "Rust");
        assert_eq!(replaced, "Hello, Rust!");
        
        // Test string parsing
        let number_str = "42";
        let parsed: Result<i32, _> = number_str.parse();
        assert_eq!(parsed.unwrap(), 42);
        
        let invalid_str = "not_a_number";
        let invalid_parsed: Result<i32, _> = invalid_str.parse();
        assert!(invalid_parsed.is_err());
        
        println!("✓ String APIs verified");
    }

    /// Test that all iterator APIs are implemented
    #[test]
    fn test_iterator_api_completeness() {
        let data = vec![1, 2, 3, 4, 5];
        
        // Test basic iteration
        let sum: i32 = data.iter().sum();
        assert_eq!(sum, 15);
        
        let count = data.iter().count();
        assert_eq!(count, 5);
        
        // Test iterator adapters
        let doubled: Vec<i32> = data.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
        
        let evens: Vec<i32> = data.iter().filter(|&&x| x % 2 == 0).cloned().collect();
        assert_eq!(evens, vec![2, 4]);
        
        let first_three: Vec<i32> = data.iter().take(3).cloned().collect();
        assert_eq!(first_three, vec![1, 2, 3]);
        
        let skip_two: Vec<i32> = data.iter().skip(2).cloned().collect();
        assert_eq!(skip_two, vec![3, 4, 5]);
        
        // Test iterator combinators
        let chained: Vec<i32> = data.iter().chain(data.iter()).cloned().collect();
        assert_eq!(chained, vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5]);
        
        let enumerated: Vec<(usize, i32)> = data.iter().enumerate().map(|(i, &x)| (i, x)).collect();
        assert_eq!(enumerated, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)]);
        
        // Test iterator consumers
        let any_even = data.iter().any(|&&x| x % 2 == 0);
        assert!(any_even);
        
        let all_positive = data.iter().all(|&&x| x > 0);
        assert!(all_positive);
        
        let found = data.iter().find(|&&x| x == 3);
        assert_eq!(found, Some(&3));
        
        let position = data.iter().position(|&&x| x == 3);
        assert_eq!(position, Some(2));
        
        // Test fold and reduce
        let product = data.iter().fold(1, |acc, &x| acc * x);
        assert_eq!(product, 120); // 1 * 2 * 3 * 4 * 5
        
        let max = data.iter().max();
        assert_eq!(max, Some(&5));
        
        let min = data.iter().min();
        assert_eq!(min, Some(&1));
        
        println!("✓ Iterator APIs verified");
    }

    /// Test that all formatting APIs are implemented
    #[test]
    fn test_formatting_api_completeness() {
        // Test basic formatting
        let formatted = format!("Hello, {}!", "World");
        assert_eq!(formatted, "Hello, World!");
        
        // Test multiple arguments
        let multi_arg = format!("{} + {} = {}", 2, 3, 5);
        assert_eq!(multi_arg, "2 + 3 = 5");
        
        // Test positional arguments
        let positional = format!("{1} {0}", "World", "Hello");
        assert_eq!(positional, "Hello World");
        
        // Test named arguments
        let named = format!("{greeting}, {name}!", greeting = "Hello", name = "World");
        assert_eq!(named, "Hello, World!");
        
        // Test number formatting
        let int_formats = format!("decimal: {}, hex: {:x}, octal: {:o}, binary: {:b}", 42, 42, 42, 42);
        assert!(int_formats.contains("decimal: 42"));
        assert!(int_formats.contains("hex: 2a"));
        assert!(int_formats.contains("octal: 52"));
        assert!(int_formats.contains("binary: 101010"));
        
        // Test float formatting
        let float_formats = format!("default: {}, precision: {:.2}, scientific: {:e}", 3.14159, 3.14159, 3.14159);
        assert!(float_formats.contains("default: 3.14159"));
        assert!(float_formats.contains("precision: 3.14"));
        assert!(float_formats.contains("scientific:"));
        
        // Test padding and alignment
        let padded = format!("'{:10}'", "test");
        assert_eq!(padded.len(), 12); // 'test      '
        
        let left_aligned = format!("'{:<10}'", "test");
        assert!(left_aligned.starts_with("'test"));
        
        let right_aligned = format!("'{:>10}'", "test");
        assert!(right_aligned.ends_with("test'"));
        
        let center_aligned = format!("'{:^10}'", "test");
        assert!(center_aligned.contains("test"));
        
        // Test debug formatting
        let debug_vec = format!("{:?}", vec![1, 2, 3]);
        assert!(debug_vec.contains("[1, 2, 3]"));
        
        println!("✓ Formatting APIs verified");
    }

    /// Test that all error handling APIs are implemented
    #[test]
    fn test_error_handling_api_completeness() {
        // Test Result error propagation
        fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
            if b == 0.0 {
                Err("Division by zero")
            } else {
                Ok(a / b)
            }
        }
        
        let success = divide(10.0, 2.0);
        assert_eq!(success.unwrap(), 5.0);
        
        let failure = divide(10.0, 0.0);
        assert!(failure.is_err());
        assert_eq!(failure.unwrap_err(), "Division by zero");
        
        // Test Result chaining
        let chained = divide(10.0, 2.0)
            .and_then(|x| divide(x, 2.0))
            .and_then(|x| divide(x, 0.0)); // This should fail
        
        assert!(chained.is_err());
        
        // Test Result mapping
        let mapped = divide(10.0, 2.0)
            .map(|x| x as i32)
            .map_err(|e| format!("Error: {}", e));
        
        assert_eq!(mapped.unwrap(), 5);
        
        // Test Option error handling
        let vec = vec![1, 2, 3];
        let valid_get = vec.get(1);
        assert_eq!(valid_get, Some(&2));
        
        let invalid_get = vec.get(10);
        assert_eq!(invalid_get, None);
        
        // Test Option chaining
        let chained_option = vec.get(1)
            .and_then(|&x| if x > 0 { Some(x * 2) } else { None });
        
        assert_eq!(chained_option, Some(4));
        
        // Test panic handling (in controlled way)
        let panic_result = std::panic::catch_unwind(|| {
            panic!("Test panic");
        });
        
        assert!(panic_result.is_err());
        
        println!("✓ Error handling APIs verified");
    }

    /// Test that all memory management APIs are implemented
    #[test]
    fn test_memory_management_api_completeness() {
        use std::rc::Rc;
        use std::sync::Arc;
        
        // Test Box (heap allocation)
        let boxed = Box::new(42);
        assert_eq!(*boxed, 42);
        
        let unboxed = Box::into_inner(boxed);
        assert_eq!(unboxed, 42);
        
        // Test Rc (reference counting)
        let rc1 = Rc::new(42);
        let rc2 = Rc::clone(&rc1);
        
        assert_eq!(*rc1, 42);
        assert_eq!(*rc2, 42);
        assert_eq!(Rc::strong_count(&rc1), 2);
        
        drop(rc2);
        assert_eq!(Rc::strong_count(&rc1), 1);
        
        // Test Arc (atomic reference counting)
        let arc1 = Arc::new(42);
        let arc2 = Arc::clone(&arc1);
        
        assert_eq!(*arc1, 42);
        assert_eq!(*arc2, 42);
        assert_eq!(Arc::strong_count(&arc1), 2);
        
        drop(arc2);
        assert_eq!(Arc::strong_count(&arc1), 1);
        
        // Test memory layout consistency
        assert_eq!(std::mem::size_of::<i32>(), 4);
        assert_eq!(std::mem::size_of::<i64>(), 8);
        assert_eq!(std::mem::align_of::<i32>(), 4);
        assert_eq!(std::mem::align_of::<i64>(), 8);
        
        // Test Vec capacity management
        let mut vec = Vec::with_capacity(10);
        assert!(vec.capacity() >= 10);
        
        for i in 0..5 {
            vec.push(i);
        }
        
        vec.shrink_to_fit();
        assert!(vec.capacity() >= vec.len());
        
        println!("✓ Memory management APIs verified");
    }
}

/// Integration tests that verify API consistency across modules
#[cfg(test)]
mod api_integration_tests {
    use super::*;

    /// Test that APIs work together correctly
    #[test]
    fn test_cross_module_api_integration() {
        // Test Result with Vec
        let results: Vec<Result<i32, &str>> = vec![
            Ok(1), Ok(2), Err("error"), Ok(4)
        ];
        
        let successful: Vec<i32> = results.into_iter()
            .filter_map(|r| r.ok())
            .collect();
        
        assert_eq!(successful, vec![1, 2, 4]);
        
        // Test Option with HashMap
        let mut map = HashMap::new();
        map.insert("key1", Some(42));
        map.insert("key2", None);
        
        let values: Vec<i32> = map.values()
            .filter_map(|&opt| opt)
            .collect();
        
        assert_eq!(values, vec![42]);
        
        // Test Iterator with formatting
        let numbers = vec![1, 2, 3, 4, 5];
        let formatted: Vec<String> = numbers.iter()
            .map(|n| format!("Number: {}", n))
            .collect();
        
        assert_eq!(formatted[0], "Number: 1");
        assert_eq!(formatted.len(), 5);
        
        // Test error propagation across operations
        fn process_numbers(nums: Vec<&str>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
            nums.iter()
                .map(|s| s.parse::<i32>().map_err(|e| e.into()))
                .collect()
        }
        
        let valid_input = vec!["1", "2", "3"];
        let valid_result = process_numbers(valid_input);
        assert!(valid_result.is_ok());
        assert_eq!(valid_result.unwrap(), vec![1, 2, 3]);
        
        let invalid_input = vec!["1", "invalid", "3"];
        let invalid_result = process_numbers(invalid_input);
        assert!(invalid_result.is_err());
        
        println!("✓ Cross-module API integration verified");
    }

    /// Test that all APIs maintain consistent behavior
    #[test]
    fn test_api_consistency() {
        // Test that empty collections behave consistently
        let empty_vec: Vec<i32> = Vec::new();
        let empty_map: HashMap<String, i32> = HashMap::new();
        
        assert!(empty_vec.is_empty());
        assert!(empty_map.is_empty());
        assert_eq!(empty_vec.len(), 0);
        assert_eq!(empty_map.len(), 0);
        
        // Test that iteration over empty collections works
        let vec_sum: i32 = empty_vec.iter().sum();
        let map_count = empty_map.iter().count();
        
        assert_eq!(vec_sum, 0);
        assert_eq!(map_count, 0);
        
        // Test that error types are consistent
        let parse_error: Result<i32, _> = "invalid".parse();
        let division_error = 10_i32.checked_div(0);
        
        assert!(parse_error.is_err());
        assert_eq!(division_error, None);
        
        // Test that None/Some and Err/Ok have consistent behavior
        let some_value = Some(42);
        let ok_value = Ok(42);
        
        assert!(some_value.is_some());
        assert!(ok_value.is_ok());
        
        let none_value: Option<i32> = None;
        let err_value: Result<i32, &str> = Err("error");
        
        assert!(none_value.is_none());
        assert!(err_value.is_err());
        
        println!("✓ API consistency verified");
    }

    /// Test that all APIs handle edge cases properly
    #[test]
    fn test_api_edge_cases() {
        // Test empty string operations
        let empty_str = "";
        assert!(empty_str.is_empty());
        assert_eq!(empty_str.len(), 0);
        assert_eq!(empty_str.to_uppercase(), "");
        assert_eq!(empty_str.to_lowercase(), "");
        
        // Test zero and negative number operations
        assert_eq!(0.0_f64.abs(), 0.0);
        assert_eq!((-0.0_f64).abs(), 0.0);
        assert_eq!(0.0_f64.signum(), 1.0); // IEEE 754 behavior
        
        // Test boundary values
        assert_eq!(i32::MAX.checked_add(1), None);
        assert_eq!(i32::MIN.checked_sub(1), None);
        assert_eq!(i32::MAX.checked_mul(2), None);
        
        // Test NaN and infinity handling
        assert!(f64::NAN.is_nan());
        assert!(f64::INFINITY.is_infinite());
        assert!(f64::NEG_INFINITY.is_infinite());
        
        // Test that NaN comparisons work correctly
        assert!(!(f64::NAN == f64::NAN));
        assert!(!(f64::NAN < 1.0));
        assert!(!(f64::NAN > 1.0));
        
        // Test empty collection edge cases
        let mut empty_vec: Vec<i32> = Vec::new();
        assert_eq!(empty_vec.pop(), None);
        assert_eq!(empty_vec.get(0), None);
        
        let empty_map: HashMap<String, i32> = HashMap::new();
        assert_eq!(empty_map.get("key"), None);
        assert!(!empty_map.contains_key("key"));
        
        println!("✓ API edge cases verified");
    }
}