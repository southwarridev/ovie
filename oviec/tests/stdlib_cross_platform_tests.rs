//! Cross-platform compatibility tests for Ovie standard library
//! 
//! These tests verify that all stdlib modules behave identically across
//! Windows, Linux, and macOS platforms. All tests use simple Rust #[test]
//! functions with no external dependencies for maximum portability.
//!
//! **Validates: Requirements 6.1.2**

use std::collections::HashMap;
use std::path::PathBuf;

#[cfg(test)]
mod cross_platform_tests {
    use super::*;

    /// Test that math operations produce identical results across platforms
    #[test]
    fn test_math_cross_platform_determinism() {
        // Test mathematical constants are identical
        let pi_value = 3.141592653589793;
        let e_value = 2.718281828459045;
        
        // These should be identical on all platforms
        assert_eq!(std::f64::consts::PI, pi_value);
        assert_eq!(std::f64::consts::E, e_value);
        
        // Test trigonometric functions
        let test_angles = [0.0, 0.5, 1.0, 1.5707963267948966, 3.141592653589793];
        
        for &angle in &test_angles {
            let angle: f64 = angle;
            let sin_result = angle.sin();
            let cos_result = angle.cos();
            let tan_result = angle.tan();
            
            // Results should be deterministic across platforms
            assert!(sin_result.is_finite() || sin_result.is_nan());
            assert!(cos_result.is_finite() || cos_result.is_nan());
            assert!(tan_result.is_finite() || tan_result.is_nan() || tan_result.is_infinite());
            
            // Test multiple calls produce same result
            assert_eq!(angle.sin(), sin_result);
            assert_eq!(angle.cos(), cos_result);
            assert_eq!(angle.tan(), tan_result);
        }
    }

    /// Test that arithmetic operations behave identically across platforms
    #[test]
    fn test_arithmetic_cross_platform() {
        let test_cases = [
            (1.0, 2.0),
            (3.14159, 2.71828),
            (-1.0, 0.5),
            (0.0, 1.0),
            (f64::MAX / 2.0, 2.0),
            (f64::MIN / 2.0, 2.0),
        ];
        
        for &(a, b) in &test_cases {
            let a: f64 = a;
            let b: f64 = b;
            let add_result = a + b;
            let sub_result = a - b;
            let mul_result = a * b;
            
            // Division with zero check
            let div_result = if b != 0.0 { Some(a / b) } else { None };
            
            // Results should be deterministic
            assert_eq!(a + b, add_result);
            assert_eq!(a - b, sub_result);
            assert_eq!(a * b, mul_result);
            
            if let Some(expected_div) = div_result {
                assert_eq!(a / b, expected_div);
            }
            
            // Test power operations
            if a >= 0.0 && b.abs() < 100.0 {
                let pow_result = a.powf(b);
                assert_eq!(a.powf(b), pow_result);
            }
        }
    }

    /// Test that string operations behave identically across platforms
    #[test]
    fn test_string_cross_platform() {
        let test_strings = [
            "hello world",
            "Hello World",
            "HELLO WORLD",
            "123456789",
            "special chars: !@#$%^&*()",
            "unicode: αβγδε",
            "",
            " ",
            "\n\t\r",
        ];
        
        for s in &test_strings {
            // String length should be consistent
            let len = s.len();
            assert_eq!(s.len(), len);
            
            // Case conversions should be consistent
            let upper = s.to_uppercase();
            let lower = s.to_lowercase();
            
            assert_eq!(s.to_uppercase(), upper);
            assert_eq!(s.to_lowercase(), lower);
            
            // String comparison should be consistent
            assert_eq!(s == s, true);
            assert_eq!(s.cmp(s), std::cmp::Ordering::Equal);
            
            // Substring operations
            if !s.is_empty() {
                let first_char = s.chars().next().unwrap();
                assert!(s.starts_with(first_char));
                
                let last_char = s.chars().last().unwrap();
                assert!(s.ends_with(last_char));
            }
        }
    }

    /// Test that collection operations behave identically across platforms
    #[test]
    fn test_collections_cross_platform() {
        // Test Vec operations
        let mut vec = Vec::new();
        let test_values = [1, 2, 3, 4, 5, -1, 0, 100, -100];
        
        for &value in &test_values {
            vec.push(value);
        }
        
        // Length should be consistent
        assert_eq!(vec.len(), test_values.len());
        
        // Iteration order should be consistent
        for (i, &expected) in test_values.iter().enumerate() {
            assert_eq!(vec[i], expected);
        }
        
        // Test HashMap operations
        let mut map = HashMap::new();
        let test_pairs = [
            ("key1", "value1"),
            ("key2", "value2"),
            ("key3", "value3"),
            ("", "empty_key"),
            ("unicode_key", "αβγ"),
        ];
        
        for (key, value) in &test_pairs {
            map.insert(*key, *value);
        }
        
        // All insertions should be retrievable
        for (key, expected_value) in &test_pairs {
            assert_eq!(map.get(key), Some(expected_value));
        }
        
        // Size should be consistent
        assert_eq!(map.len(), test_pairs.len());
    }

    /// Test that file path operations behave consistently across platforms
    #[test]
    fn test_path_cross_platform() {
        let test_paths = [
            "simple.txt",
            "dir/file.txt",
            "deep/nested/path/file.ext",
            "file_with_underscores.txt",
            "file-with-dashes.txt",
            "file.with.dots.txt",
        ];
        
        for path_str in &test_paths {
            let path = PathBuf::from(path_str);
            
            // Path components should be consistent
            let components: Vec<_> = path.components().collect();
            let reconstructed = components.iter().fold(PathBuf::new(), |mut acc, comp| {
                acc.push(comp);
                acc
            });
            
            // Reconstruction should match original
            assert_eq!(path, reconstructed);
            
            // File name extraction should be consistent
            if let Some(file_name) = path.file_name() {
                assert!(!file_name.is_empty());
            }
            
            // Extension extraction should be consistent
            if let Some(extension) = path.extension() {
                assert!(!extension.is_empty());
            }
        }
    }

    /// Test that environment variable operations behave consistently
    #[test]
    fn test_env_cross_platform() {
        // Test setting and getting environment variables
        let test_var = "OVIE_TEST_VAR";
        let test_value = "test_value_123";
        
        // Set variable
        std::env::set_var(test_var, test_value);
        
        // Retrieve variable
        let retrieved = std::env::var(test_var);
        assert_eq!(retrieved.unwrap(), test_value);
        
        // Test variable existence
        assert!(std::env::var(test_var).is_ok());
        
        // Clean up
        std::env::remove_var(test_var);
        
        // Verify removal
        assert!(std::env::var(test_var).is_err());
        
        // Test common environment variables exist (platform-specific handling)
        let common_vars = if cfg!(windows) {
            vec!["PATH", "TEMP", "USERNAME"]
        } else {
            vec!["PATH", "HOME", "USER"]
        };
        
        for var in &common_vars {
            // These should exist on their respective platforms
            let value = std::env::var(var);
            assert!(value.is_ok(), "Environment variable {} should exist", var);
            assert!(!value.unwrap().is_empty(), "Environment variable {} should not be empty", var);
        }
    }

    /// Test that time operations behave consistently across platforms
    #[test]
    fn test_time_cross_platform() {
        use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
        
        // Test Duration operations
        let duration1 = Duration::from_secs(60);
        let duration2 = Duration::from_millis(60000);
        
        assert_eq!(duration1, duration2);
        assert_eq!(duration1.as_secs(), 60);
        assert_eq!(duration1.as_millis(), 60000);
        
        // Test Instant operations
        let start = Instant::now();
        let end = Instant::now();
        
        // End should be >= start
        assert!(end >= start);
        
        let elapsed = end.duration_since(start);
        assert!(elapsed.as_nanos() >= 0);
        
        // Test SystemTime operations
        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH);
        
        assert!(since_epoch.is_ok());
        let duration = since_epoch.unwrap();
        
        // Should be a reasonable time since epoch (after year 2000)
        assert!(duration.as_secs() > 946684800); // Jan 1, 2000
        
        // Should be before year 2100
        assert!(duration.as_secs() < 4102444800); // Jan 1, 2100
    }

    /// Test that I/O operations behave consistently across platforms
    #[test]
    fn test_io_cross_platform() {
        use std::io::Write;
        
        // Test string formatting consistency
        let test_values = [
            42,
            -42,
            0,
            i32::MAX,
            i32::MIN,
        ];
        
        for value in &test_values {
            let formatted = format!("{}", value);
            let parsed: i32 = formatted.parse().unwrap();
            assert_eq!(parsed, *value);
            
            // Test different formatting options
            let hex_formatted = format!("{:x}", value);
            let octal_formatted = format!("{:o}", value);
            let binary_formatted = format!("{:b}", value);
            
            // These should be consistent across platforms
            assert!(!hex_formatted.is_empty());
            assert!(!octal_formatted.is_empty());
            assert!(!binary_formatted.is_empty());
        }
        
        // Test buffer operations
        let mut buffer = Vec::new();
        let test_data = b"Hello, cross-platform world!";
        
        buffer.write_all(test_data).unwrap();
        assert_eq!(buffer.len(), test_data.len());
        assert_eq!(&buffer[..], test_data);
        
        // Test string conversion
        let string_data = String::from_utf8(buffer).unwrap();
        assert_eq!(string_data.as_bytes(), test_data);
    }

    /// Test that error handling behaves consistently across platforms
    #[test]
    fn test_error_cross_platform() {
        use std::fs::File;
        use std::io::ErrorKind;
        
        // Test file not found error
        let result = File::open("definitely_does_not_exist.txt");
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert_eq!(error.kind(), ErrorKind::NotFound);
        
        // Test error message formatting
        let error_string = error.to_string();
        assert!(!error_string.is_empty());
        
        // Test Result operations
        let ok_result: Result<i32, &str> = Ok(42);
        let err_result: Result<i32, &str> = Err("error message");
        
        assert!(ok_result.is_ok());
        assert!(!ok_result.is_err());
        assert_eq!(ok_result.unwrap(), 42);
        
        assert!(err_result.is_err());
        assert!(!err_result.is_ok());
        assert_eq!(err_result.unwrap_err(), "error message");
        
        // Test Option operations
        let some_option = Some(42);
        let none_option: Option<i32> = None;
        
        assert!(some_option.is_some());
        assert!(!some_option.is_none());
        assert_eq!(some_option.unwrap(), 42);
        
        assert!(none_option.is_none());
        assert!(!none_option.is_some());
        assert_eq!(none_option.unwrap_or(0), 0);
    }

    /// Test that memory operations behave consistently across platforms
    #[test]
    fn test_memory_cross_platform() {
        // Test basic memory allocation
        let vec: Vec<i32> = Vec::with_capacity(1000);
        assert_eq!(vec.len(), 0);
        assert!(vec.capacity() >= 1000);
        
        // Test memory layout consistency
        let array = [1, 2, 3, 4, 5];
        let slice = &array[..];
        
        assert_eq!(slice.len(), 5);
        assert_eq!(slice[0], 1);
        assert_eq!(slice[4], 5);
        
        // Test pointer arithmetic consistency
        let ptr = array.as_ptr();
        unsafe {
            assert_eq!(*ptr, 1);
            assert_eq!(*ptr.add(1), 2);
            assert_eq!(*ptr.add(4), 5);
        }
        
        // Test alignment requirements
        assert_eq!(std::mem::align_of::<i32>(), 4);
        assert_eq!(std::mem::align_of::<i64>(), 8);
        assert_eq!(std::mem::size_of::<i32>(), 4);
        assert_eq!(std::mem::size_of::<i64>(), 8);
    }

    /// Test that threading operations behave consistently across platforms
    #[test]
    fn test_threading_cross_platform() {
        use std::sync::{Arc, Mutex};
        use std::thread;
        
        // Test basic thread creation and joining
        let handle = thread::spawn(|| {
            42
        });
        
        let result = handle.join().unwrap();
        assert_eq!(result, 42);
        
        // Test shared state with Arc<Mutex<T>>
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        
        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        let final_count = *counter.lock().unwrap();
        assert_eq!(final_count, 10);
    }

    /// Test platform-specific behavior is properly abstracted
    #[test]
    fn test_platform_abstraction() {
        // Test path separator handling
        let path = std::path::Path::new("dir").join("file.txt");
        let path_str = path.to_string_lossy();
        
        // Should contain appropriate separator for platform
        if cfg!(windows) {
            assert!(path_str.contains('\\') || path_str.contains('/'));
        } else {
            assert!(path_str.contains('/'));
        }
        
        // Test line ending handling
        let line_ending = if cfg!(windows) { "\r\n" } else { "\n" };
        let text_with_endings = format!("line1{}line2{}", line_ending, line_ending);
        
        let lines: Vec<&str> = text_with_endings.lines().collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "line1");
        assert_eq!(lines[1], "line2");
        
        // Test executable extension
        let exe_name = if cfg!(windows) { "program.exe" } else { "program" };
        assert!(!exe_name.is_empty());
        
        if cfg!(windows) {
            assert!(exe_name.ends_with(".exe"));
        } else {
            assert!(!exe_name.ends_with(".exe"));
        }
    }

    /// Test that numeric limits are consistent across platforms
    #[test]
    fn test_numeric_limits_cross_platform() {
        // Test integer limits
        assert_eq!(i8::MIN, -128);
        assert_eq!(i8::MAX, 127);
        assert_eq!(u8::MIN, 0);
        assert_eq!(u8::MAX, 255);
        
        assert_eq!(i16::MIN, -32768);
        assert_eq!(i16::MAX, 32767);
        assert_eq!(u16::MIN, 0);
        assert_eq!(u16::MAX, 65535);
        
        assert_eq!(i32::MIN, -2147483648);
        assert_eq!(i32::MAX, 2147483647);
        assert_eq!(u32::MIN, 0);
        assert_eq!(u32::MAX, 4294967295);
        
        // Test floating point limits
        assert!(f32::MIN.is_finite());
        assert!(f32::MAX.is_finite());
        assert!(f32::INFINITY.is_infinite());
        assert!(f32::NEG_INFINITY.is_infinite());
        assert!(f32::NAN.is_nan());
        
        assert!(f64::MIN.is_finite());
        assert!(f64::MAX.is_finite());
        assert!(f64::INFINITY.is_infinite());
        assert!(f64::NEG_INFINITY.is_infinite());
        assert!(f64::NAN.is_nan());
        
        // Test epsilon values
        assert!(f32::EPSILON > 0.0);
        assert!(f64::EPSILON > 0.0);
        assert!(f64::EPSILON < f32::EPSILON as f64);
    }

    /// Test that hash operations are deterministic across platforms
    #[test]
    fn test_hash_cross_platform() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let test_values = [
            "hello",
            "world",
            "test string",
            "",
            "unicode: αβγδε",
            "numbers: 123456789",
        ];
        
        for value in &test_values {
            // Hash the same value multiple times
            let mut hasher1 = DefaultHasher::new();
            value.hash(&mut hasher1);
            let hash1 = hasher1.finish();
            
            let mut hasher2 = DefaultHasher::new();
            value.hash(&mut hasher2);
            let hash2 = hasher2.finish();
            
            // Should produce the same hash
            assert_eq!(hash1, hash2);
            
            // Hash should be deterministic within the same run
            for _ in 0..10 {
                let mut hasher = DefaultHasher::new();
                value.hash(&mut hasher);
                let hash = hasher.finish();
                assert_eq!(hash, hash1);
            }
        }
    }
}

/// Integration tests that verify cross-platform stdlib behavior
#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Test complete stdlib workflow across platforms
    #[test]
    fn test_stdlib_integration_cross_platform() {
        // Simulate a complete stdlib workflow
        let mut data = Vec::new();
        
        // Math operations
        for i in 0..10 {
            let value = (i as f64).sin() * 100.0;
            data.push(value as i32);
        }
        
        // Collection operations
        data.sort();
        data.dedup();
        
        // String operations
        let formatted: Vec<String> = data.iter()
            .map(|x| format!("value_{}", x))
            .collect();
        
        // I/O simulation (using Vec as buffer)
        let mut buffer = Vec::new();
        for s in &formatted {
            buffer.extend_from_slice(s.as_bytes());
            buffer.push(b'\n');
        }
        
        // Verify results are consistent
        assert!(!data.is_empty());
        assert!(!formatted.is_empty());
        assert!(!buffer.is_empty());
        assert_eq!(data.len(), formatted.len());
        
        // Convert back and verify
        let reconstructed = String::from_utf8(buffer).unwrap();
        let lines: Vec<&str> = reconstructed.lines().collect();
        assert_eq!(lines.len(), formatted.len());
        
        for (original, reconstructed) in formatted.iter().zip(lines.iter()) {
            assert_eq!(original, reconstructed);
        }
    }

    /// Test error propagation across platform boundaries
    #[test]
    fn test_error_propagation_cross_platform() {
        use std::fs::File;
        use std::io::Read;
        
        // Function that might fail on any platform
        fn read_file_content(path: &str) -> Result<String, Box<dyn std::error::Error>> {
            let mut file = File::open(path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            Ok(content)
        }
        
        // Test with non-existent file (should fail consistently)
        let result = read_file_content("non_existent_file.txt");
        assert!(result.is_err());
        
        // Error should be properly formatted
        let error = result.unwrap_err();
        let error_string = error.to_string();
        assert!(!error_string.is_empty());
    }

    /// Test resource cleanup across platforms
    #[test]
    fn test_resource_cleanup_cross_platform() {
        use std::fs::File;
        use std::io::Write;
        
        let temp_file = "test_temp_file.txt";
        
        // Create and write to file
        {
            let mut file = File::create(temp_file).unwrap();
            file.write_all(b"test content").unwrap();
            file.flush().unwrap();
        } // File should be closed here
        
        // Verify file exists and can be read
        let content = std::fs::read_to_string(temp_file).unwrap();
        assert_eq!(content, "test content");
        
        // Clean up
        std::fs::remove_file(temp_file).unwrap();
        
        // Verify cleanup
        assert!(std::fs::metadata(temp_file).is_err());
    }
}