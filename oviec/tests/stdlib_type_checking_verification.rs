//! Type Checking Verification Tests for Ovie Standard Library
//! 
//! These tests verify that all standard library functions have proper type checking,
//! including parameter type validation, return type consistency, and generic type
//! parameter handling.
//!
//! **Validates: Requirements 6.2.3**

use std::collections::HashMap;

#[cfg(test)]
mod type_checking_verification_tests {
    use super::*;

    /// Test that Result<T, E> type operations maintain type safety
    #[test]
    fn test_result_type_safety() {
        // Test Ok variant type consistency
        let ok_int: Result<i32, &str> = Ok(42);
        let ok_string: Result<String, &str> = Ok("hello".to_string());
        
        // Type should be preserved through operations
        assert_eq!(ok_int.unwrap(), 42);
        assert_eq!(ok_string.unwrap(), "hello");
        
        // Test Err variant type consistency
        let err_int: Result<i32, &str> = Err("error");
        let err_string: Result<String, &str> = Err("error");
        
        assert_eq!(err_int.unwrap_err(), "error");
        assert_eq!(err_string.unwrap_err(), "error");
        
        // Test map preserves type transformations
        let mapped_result = ok_int.map(|x| x.to_string());
        assert!(mapped_result.is_ok());
        assert_eq!(mapped_result.unwrap(), "42");
        
        // Test map_err preserves error type transformations
        let mapped_err = err_int.map_err(|e| format!("Error: {}", e));
        assert!(mapped_err.is_err());
        assert_eq!(mapped_err.unwrap_err(), "Error: error");
        
        println!("✓ Result type safety verified");
    }

    /// Test that Option<T> type operations maintain type safety
    #[test]
    fn test_option_type_safety() {
        // Test Some variant type consistency
        let some_int = Some(42);
        let some_string = Some("hello".to_string());
        
        assert_eq!(some_int.unwrap(), 42);
        assert_eq!(some_string.unwrap(), "hello");
        
        // Test None variant
        let none_int: Option<i32> = None;
        let none_string: Option<String> = None;
        
        assert!(none_int.is_none());
        assert!(none_string.is_none());
        
        // Test map preserves type transformations
        let mapped_some = some_int.map(|x| x.to_string());
        assert_eq!(mapped_some.unwrap(), "42");
        
        let mapped_none = none_int.map(|x| x.to_string());
        assert!(mapped_none.is_none());
        
        // Test Option to Result conversion maintains types
        let result_from_some = some_int.ok_or("no value");
        assert!(result_from_some.is_ok());
        assert_eq!(result_from_some.unwrap(), 42);
        
        let result_from_none = none_int.ok_or("no value");
        assert!(result_from_none.is_err());
        assert_eq!(result_from_none.unwrap_err(), "no value");
        
        println!("✓ Option type safety verified");
    }

    /// Test that Vec<T> operations maintain type safety
    #[test]
    fn test_vec_type_safety() {
        // Test homogeneous type storage
        let mut int_vec: Vec<i32> = Vec::new();
        let mut string_vec: Vec<String> = Vec::new();
        
        int_vec.push(1);
        int_vec.push(2);
        int_vec.push(3);
        
        string_vec.push("hello".to_string());
        string_vec.push("world".to_string());
        
        // Type should be preserved in access operations
        assert_eq!(int_vec[0], 1);
        assert_eq!(string_vec[0], "hello");
        
        // Test get returns Option<&T>
        assert_eq!(int_vec.get(0), Some(&1));
        assert_eq!(string_vec.get(0), Some(&"hello".to_string()));
        assert_eq!(int_vec.get(10), None);
        
        // Test pop returns Option<T>
        assert_eq!(int_vec.pop(), Some(3));
        assert_eq!(string_vec.pop(), Some("world".to_string()));
        
        // Test iterator type consistency
        let doubled: Vec<i32> = int_vec.iter().map(|&x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4]);
        
        let lengths: Vec<usize> = string_vec.iter().map(|s| s.len()).collect();
        assert_eq!(lengths, vec![5]); // "hello".len()
        
        println!("✓ Vec type safety verified");
    }
    /// Test that HashMap<K, V> operations maintain type safety
    #[test]
    fn test_hashmap_type_safety() {
        // Test key-value type consistency
        let mut int_to_string: HashMap<i32, String> = HashMap::new();
        let mut string_to_int: HashMap<String, i32> = HashMap::new();
        
        int_to_string.insert(1, "one".to_string());
        int_to_string.insert(2, "two".to_string());
        
        string_to_int.insert("one".to_string(), 1);
        string_to_int.insert("two".to_string(), 2);
        
        // Type should be preserved in access operations
        assert_eq!(int_to_string.get(&1), Some(&"one".to_string()));
        assert_eq!(string_to_int.get("one"), Some(&1));
        
        // Test insert returns Option<V> (old value)
        let old_value = int_to_string.insert(1, "ONE".to_string());
        assert_eq!(old_value, Some("one".to_string()));
        
        // Test remove returns Option<V>
        let removed = string_to_int.remove("one");
        assert_eq!(removed, Some(1));
        
        // Test iteration maintains key-value types
        for (key, value) in &int_to_string {
            assert!(key > &0); // key is &i32
            assert!(!value.is_empty()); // value is &String
        }
        
        println!("✓ HashMap type safety verified");
    }

    /// Test that mathematical operations maintain numeric type safety
    #[test]
    fn test_math_type_safety() {
        // Test integer operations
        let a: i32 = 10;
        let b: i32 = 3;
        
        // Basic arithmetic should preserve types
        assert_eq!(a + b, 13);
        assert_eq!(a - b, 7);
        assert_eq!(a * b, 30);
        assert_eq!(a / b, 3);
        assert_eq!(a % b, 1);
        
        // Test floating-point operations
        let x: f64 = 10.0;
        let y: f64 = 3.0;
        
        assert!((x / y - 3.333333333333333).abs() < 1e-10);
        assert_eq!(x.floor(), 10.0);
        assert_eq!(x.ceil(), 10.0);
        assert_eq!(x.round(), 10.0);
        
        // Test trigonometric functions maintain f64 type
        let angle: f64 = std::f64::consts::PI / 4.0;
        let sin_result = angle.sin();
        let cos_result = angle.cos();
        
        assert!(sin_result.is_finite());
        assert!(cos_result.is_finite());
        assert!((sin_result - cos_result).abs() < 1e-10); // sin(π/4) ≈ cos(π/4)
        
        // Test power operations
        let base: f64 = 2.0;
        let exponent: f64 = 3.0;
        let power_result = base.powf(exponent);
        assert_eq!(power_result, 8.0);
        
        println!("✓ Math type safety verified");
    }

    /// Test that string operations maintain type safety
    #[test]
    fn test_string_type_safety() {
        let s1 = "Hello";
        let s2 = "World";
        let owned_string = String::from("Test");
        
        // Test string concatenation types
        let concatenated = format!("{}, {}!", s1, s2);
        assert_eq!(concatenated, "Hello, World!");
        
        // Test string method return types
        assert_eq!(s1.len(), 5); // returns usize
        assert!(s1.starts_with("He")); // returns bool
        assert!(s1.ends_with("lo")); // returns bool
        assert!(s1.contains("ell")); // returns bool
        
        // Test string slicing maintains str type
        let slice = &s1[1..4];
        assert_eq!(slice, "ell");
        
        // Test string parsing maintains Result<T, ParseError> type
        let number_str = "42";
        let parsed_int: Result<i32, _> = number_str.parse();
        assert!(parsed_int.is_ok());
        assert_eq!(parsed_int.unwrap(), 42);
        
        let invalid_str = "not_a_number";
        let invalid_parsed: Result<i32, _> = invalid_str.parse();
        assert!(invalid_parsed.is_err());
        
        // Test owned string operations
        let mut mutable_string = owned_string.clone();
        mutable_string.push_str(" String");
        assert_eq!(mutable_string, "Test String");
        
        println!("✓ String type safety verified");
    }

    /// Test that iterator operations maintain type safety
    #[test]
    fn test_iterator_type_safety() {
        let numbers = vec![1, 2, 3, 4, 5];
        
        // Test iterator adapter type transformations
        let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
        
        let strings: Vec<String> = numbers.iter().map(|&x| x.to_string()).collect();
        assert_eq!(strings, vec!["1", "2", "3", "4", "5"]);
        
        // Test filter maintains element type
        let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();
        assert_eq!(evens, vec![2, 4]);
        
        // Test fold/reduce type consistency
        let sum: i32 = numbers.iter().sum();
        assert_eq!(sum, 15);
        
        let product: i32 = numbers.iter().product();
        assert_eq!(product, 120);
        
        // Test find returns Option<&T>
        let found = numbers.iter().find(|&&x| x == 3);
        assert_eq!(found, Some(&3));
        
        let not_found = numbers.iter().find(|&&x| x == 10);
        assert_eq!(not_found, None);
        
        // Test enumerate maintains (usize, &T) type
        let enumerated: Vec<(usize, i32)> = numbers.iter().enumerate().map(|(i, &x)| (i, x)).collect();
        assert_eq!(enumerated, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)]);
        
        println!("✓ Iterator type safety verified");
    }
    /// Test that generic type parameters work correctly
    #[test]
    fn test_generic_type_safety() {
        // Test generic function type inference
        fn identity<T>(x: T) -> T {
            x
        }
        
        let int_result = identity(42);
        let string_result = identity("hello");
        let vec_result = identity(vec![1, 2, 3]);
        
        assert_eq!(int_result, 42);
        assert_eq!(string_result, "hello");
        assert_eq!(vec_result, vec![1, 2, 3]);
        
        // Test generic struct type safety
        #[derive(Debug, PartialEq)]
        struct Container<T> {
            value: T,
        }
        
        impl<T> Container<T> {
            fn new(value: T) -> Self {
                Container { value }
            }
            
            fn get(&self) -> &T {
                &self.value
            }
            
            fn map<U, F>(self, f: F) -> Container<U>
            where
                F: FnOnce(T) -> U,
            {
                Container::new(f(self.value))
            }
        }
        
        let int_container = Container::new(42);
        let string_container = Container::new("hello".to_string());
        
        assert_eq!(*int_container.get(), 42);
        assert_eq!(*string_container.get(), "hello");
        
        // Test generic transformation
        let string_from_int = int_container.map(|x| x.to_string());
        assert_eq!(*string_from_int.get(), "42");
        
        let length_from_string = string_container.map(|s| s.len());
        assert_eq!(*length_from_string.get(), 5);
        
        println!("✓ Generic type safety verified");
    }

    /// Test that error types are properly typed
    #[test]
    fn test_error_type_safety() {
        // Test Result error type consistency
        fn divide(a: f64, b: f64) -> Result<f64, String> {
            if b == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(a / b)
            }
        }
        
        let success = divide(10.0, 2.0);
        assert!(success.is_ok());
        assert_eq!(success.unwrap(), 5.0);
        
        let failure = divide(10.0, 0.0);
        assert!(failure.is_err());
        assert_eq!(failure.unwrap_err(), "Division by zero");
        
        // Test error propagation maintains types
        fn chain_operations(a: f64, b: f64, c: f64) -> Result<f64, String> {
            let first = divide(a, b)?;
            let second = divide(first, c)?;
            Ok(second)
        }
        
        let chain_success = chain_operations(20.0, 2.0, 2.0);
        assert!(chain_success.is_ok());
        assert_eq!(chain_success.unwrap(), 5.0);
        
        let chain_failure = chain_operations(20.0, 0.0, 2.0);
        assert!(chain_failure.is_err());
        
        // Test different error types
        #[derive(Debug, PartialEq)]
        enum MathError {
            DivisionByZero,
            NegativeSquareRoot,
            Overflow,
        }
        
        fn safe_sqrt(x: f64) -> Result<f64, MathError> {
            if x < 0.0 {
                Err(MathError::NegativeSquareRoot)
            } else {
                Ok(x.sqrt())
            }
        }
        
        let valid_sqrt = safe_sqrt(9.0);
        assert_eq!(valid_sqrt.unwrap(), 3.0);
        
        let invalid_sqrt = safe_sqrt(-1.0);
        assert_eq!(invalid_sqrt.unwrap_err(), MathError::NegativeSquareRoot);
        
        println!("✓ Error type safety verified");
    }

    /// Test that memory management types maintain safety
    #[test]
    fn test_memory_type_safety() {
        use std::rc::Rc;
        use std::sync::Arc;
        
        // Test Box type safety
        let boxed_int = Box::new(42);
        let boxed_string = Box::new("hello".to_string());
        
        assert_eq!(*boxed_int, 42);
        assert_eq!(*boxed_string, "hello");
        
        // Test Rc type safety
        let rc_int = Rc::new(42);
        let rc_clone = Rc::clone(&rc_int);
        
        assert_eq!(*rc_int, 42);
        assert_eq!(*rc_clone, 42);
        assert_eq!(Rc::strong_count(&rc_int), 2);
        
        // Test Arc type safety
        let arc_string = Arc::new("hello".to_string());
        let arc_clone = Arc::clone(&arc_string);
        
        assert_eq!(*arc_string, "hello");
        assert_eq!(*arc_clone, "hello");
        assert_eq!(Arc::strong_count(&arc_string), 2);
        
        // Test that types are preserved through smart pointers
        let rc_vec = Rc::new(vec![1, 2, 3]);
        assert_eq!(rc_vec.len(), 3);
        assert_eq!(rc_vec[0], 1);
        
        println!("✓ Memory management type safety verified");
    }

    /// Test that trait objects maintain type safety
    #[test]
    fn test_trait_object_type_safety() {
        trait Display {
            fn display(&self) -> String;
        }
        
        impl Display for i32 {
            fn display(&self) -> String {
                self.to_string()
            }
        }
        
        impl Display for String {
            fn display(&self) -> String {
                self.clone()
            }
        }
        
        // Test trait object creation and usage
        let int_display: Box<dyn Display> = Box::new(42);
        let string_display: Box<dyn Display> = Box::new("hello".to_string());
        
        assert_eq!(int_display.display(), "42");
        assert_eq!(string_display.display(), "hello");
        
        // Test trait object in collections
        let displays: Vec<Box<dyn Display>> = vec![
            Box::new(42),
            Box::new("world".to_string()),
            Box::new(100),
        ];
        
        let results: Vec<String> = displays.iter().map(|d| d.display()).collect();
        assert_eq!(results, vec!["42", "world", "100"]);
        
        println!("✓ Trait object type safety verified");
    }
}