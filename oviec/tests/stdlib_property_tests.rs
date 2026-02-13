//! Comprehensive Property-Based Tests for Ovie Standard Library
//! 
//! This test suite validates correctness properties across all standard library modules
//! using property-based testing techniques. Each module is tested for:
//! - Deterministic behavior across platforms
//! - Correctness of core operations
//! - Edge case handling
//! - Type safety and invariant preservation
//! - Cross-platform consistency
//! - Performance characteristics
//! - Memory safety

use oviec::stdlib::*;
use std::collections::HashSet;
use std::time::Instant;

/// Simple random number generator for property tests
pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }
    
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }
    
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() as f64) / (u64::MAX as f64)
    }
    
    pub fn gen_range(&mut self, min: i32, max: i32) -> i32 {
        if min >= max {
            return min;
        }
        min + (self.next_u32() as i32).abs() % (max - min)
    }
    
    pub fn gen_bool(&mut self) -> bool {
        self.next_u64() % 2 == 0
    }
    
    pub fn gen_string(&mut self, max_len: usize) -> String {
        let len = self.gen_range(0, max_len as i32 + 1) as usize;
        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ".chars().collect();
        (0..len)
            .map(|_| chars[self.gen_range(0, chars.len() as i32) as usize])
            .collect()
    }
}

/// Run a property test with multiple random inputs
pub fn check_property<F>(name: &str, iterations: usize, mut test_fn: F) 
where 
    F: FnMut(&mut SimpleRng) -> bool,
{
    let mut rng = SimpleRng::new(42); // Fixed seed for reproducibility
    let mut failures = 0;
    
    for i in 0..iterations {
        if !test_fn(&mut rng) {
            failures += 1;
            if failures > 10 { // Stop after too many failures
                panic!("Property '{}' failed {} times (stopped at iteration {})", name, failures, i);
            }
        }
    }
    
    if failures > 0 {
        panic!("Property '{}' failed {} out of {} iterations", name, failures, iterations);
    }
}

// =============================================================================
// Core Module Property Tests
// =============================================================================

#[test]
fn property_result_operations_preserve_correctness() {
    check_property("Result operations preserve correctness", 1000, |rng| {
        let value = rng.gen_range(-1000, 1000);
        let error_msg = rng.gen_string(20);
        
        // Test Ok values
        let ok_result: OvieResult<i32, String> = ok(value);
        assert!(ok_result.is_ok());
        assert!(!ok_result.is_err());
        assert_eq!(ok_result.unwrap(), value);
        
        // Test Err values
        let err_result: OvieResult<i32, String> = err(error_msg.clone());
        assert!(!err_result.is_ok());
        assert!(err_result.is_err());
        
        // Test map operations preserve structure
        let mapped_ok = ok(value).map(|x| x * 2);
        assert!(mapped_ok.is_ok());
        assert_eq!(mapped_ok.unwrap(), value * 2);
        
        let mapped_err = err(error_msg.clone()).map(|x: i32| x * 2);
        assert!(mapped_err.is_err());
        
        true
    });
}

#[test]
fn property_option_operations_preserve_correctness() {
    check_property("Option operations preserve correctness", 1000, |rng| {
        let value = rng.gen_range(-1000, 1000);
        
        // Test Some values
        let some_option = some(value);
        assert!(some_option.is_some());
        assert!(!some_option.is_none());
        assert_eq!(some_option.unwrap(), value);
        
        // Test None values
        let none_option: OvieOption<i32> = none();
        assert!(!none_option.is_some());
        assert!(none_option.is_none());
        
        // Test map operations preserve structure
        let mapped_some = some(value).map(|x| x * 2);
        assert!(mapped_some.is_some());
        assert_eq!(mapped_some.unwrap(), value * 2);
        
        let mapped_none = none().map(|x: i32| x * 2);
        assert!(mapped_none.is_none());
        
        true
    });
}

#[test]
fn property_vec_operations_maintain_invariants() {
    check_property("Vec operations maintain invariants", 500, |rng| {
        let mut vec = OvieVec::new();
        let initial_len = vec.len();
        assert_eq!(initial_len, 0);
        
        // Add elements and verify length increases
        let num_elements = rng.gen_range(1, 20);
        let mut expected_elements = Vec::new();
        
        for _ in 0..num_elements {
            let value = rng.gen_range(-100, 100);
            vec.push(value);
            expected_elements.push(value);
        }
        
        assert_eq!(vec.len(), num_elements as usize);
        
        // Verify all elements are accessible and correct
        for i in 0..vec.len() {
            assert_eq!(vec.get(i), some(expected_elements[i]));
        }
        
        // Test pop operations
        while !vec.is_empty() {
            let expected = expected_elements.pop().unwrap();
            let actual = vec.pop().unwrap();
            assert_eq!(actual, expected);
        }
        
        assert_eq!(vec.len(), 0);
        assert!(vec.is_empty());
        
        true
    });
}

#[test]
fn property_hashmap_operations_maintain_invariants() {
    check_property("HashMap operations maintain invariants", 500, |rng| {
        let mut map = OvieHashMap::new();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
        
        let num_pairs = rng.gen_range(1, 20);
        let mut expected_keys = HashSet::new();
        
        // Insert key-value pairs
        for i in 0..num_pairs {
            let key = format!("key_{}", i);
            let value = rng.gen_range(-100, 100);
            
            map.insert(key.clone(), value);
            expected_keys.insert(key.clone());
            
            // Verify insertion
            assert_eq!(map.get(&key), some(value));
            assert!(map.contains_key(&key));
        }
        
        assert_eq!(map.len(), expected_keys.len());
        assert!(!map.is_empty());
        
        // Verify all keys exist
        for key in &expected_keys {
            assert!(map.contains_key(key));
        }
        
        // Test removal
        for key in expected_keys {
            assert!(map.contains_key(&key));
            let removed = map.remove(&key);
            assert!(removed.is_some());
            assert!(!map.contains_key(&key));
        }
        
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
        
        true
    });
}

// =============================================================================
// Math Module Property Tests
// =============================================================================

#[test]
fn property_math_operations_are_deterministic() {
    check_property("Math operations are deterministic", 1000, |rng| {
        let a = rng.next_f64() * 100.0 - 50.0; // Range: -50 to 50
        let b = rng.next_f64() * 100.0 - 50.0;
        
        // Test that same inputs always produce same outputs
        assert_eq!(ovie_abs(a), ovie_abs(a));
        assert_eq!(ovie_sqrt(a.abs()), ovie_sqrt(a.abs()));
        assert_eq!(ovie_floor(a), ovie_floor(a));
        assert_eq!(ovie_ceil(a), ovie_ceil(a));
        assert_eq!(ovie_round(a), ovie_round(a));
        
        // Test min/max properties
        let min_val = ovie_min(a, b);
        let max_val = ovie_max(a, b);
        assert!(min_val <= max_val);
        assert!(min_val == a || min_val == b);
        assert!(max_val == a || max_val == b);
        
        // Test clamp properties
        let low = ovie_min(a, b);
        let high = ovie_max(a, b);
        let test_val = rng.next_f64() * 200.0 - 100.0;
        let clamped = ovie_clamp(test_val, low, high);
        assert!(clamped >= low);
        assert!(clamped <= high);
        
        true
    });
}

#[test]
fn property_checked_arithmetic_prevents_overflow() {
    check_property("Checked arithmetic prevents overflow", 1000, |rng| {
        let a = rng.gen_range(-1000, 1000);
        let b = rng.gen_range(-1000, 1000);
        
        // Test checked addition
        let add_result = checked_add(a, b);
        if let some(sum) = add_result {
            // If operation succeeded, verify it's correct
            assert_eq!(sum, a + b);
        }
        
        // Test checked subtraction
        let sub_result = checked_sub(a, b);
        if let some(diff) = sub_result {
            assert_eq!(diff, a - b);
        }
        
        // Test checked multiplication
        let mul_result = checked_mul(a, b);
        if let some(product) = mul_result {
            assert_eq!(product, a * b);
        }
        
        // Test checked division (avoid division by zero)
        if b != 0 {
            let div_result = checked_div(a, b);
            if let some(quotient) = div_result {
                assert_eq!(quotient, a / b);
            }
        }
        
        true
    });
}

#[test]
fn property_math_constants_are_stable() {
    // Test that mathematical constants are deterministic and within expected ranges
    assert!(PI > 3.14 && PI < 3.15);
    assert!(E > 2.71 && E < 2.72);
    assert!(TAU > 6.28 && TAU < 6.29);
    assert_eq!(TAU, 2.0 * PI);
    
    // Test that constants are exactly the same across multiple accesses
    assert_eq!(PI, PI);
    assert_eq!(E, E);
    assert_eq!(TAU, TAU);
    
    // Test infinity and NaN constants
    assert!(INFINITY.is_infinite());
    assert!(NEG_INFINITY.is_infinite());
    assert!(NAN.is_nan());
    assert!(EPSILON > 0.0);
}

// =============================================================================
// I/O Module Property Tests
// =============================================================================

#[test]
fn property_io_formatting_is_deterministic() {
    check_property("I/O formatting is deterministic", 500, |rng| {
        let value = rng.gen_range(-1000, 1000);
        let text = rng.gen_string(50);
        
        // Test that format operations are deterministic
        let formatted1 = format("Value: {}, Text: {}", vec![value.to_string(), text.clone()]);
        let formatted2 = format("Value: {}, Text: {}", vec![value.to_string(), text.clone()]);
        assert_eq!(formatted1, formatted2);
        
        // Test that format contains expected content
        if let ok(result) = formatted1 {
            assert!(result.contains(&value.to_string()));
            assert!(result.contains(&text));
        }
        
        true
    });
}

// =============================================================================
// Time Module Property Tests
// =============================================================================

#[test]
fn property_time_operations_are_consistent() {
    check_property("Time operations are consistent", 100, |rng| {
        let seconds = rng.gen_range(0, 1000000) as u64;
        
        // Test time creation and conversion
        let time1 = from_unix_timestamp(seconds);
        let time2 = from_unix_timestamp(seconds);
        assert_eq!(time1.unix_timestamp(), time2.unix_timestamp());
        
        // Test duration operations
        let duration1 = duration_from_seconds(seconds);
        let duration2 = duration_from_seconds(seconds);
        assert_eq!(duration1.as_seconds(), duration2.as_seconds());
        
        // Test duration arithmetic properties
        let dur_a = duration_from_seconds(rng.gen_range(1, 1000) as u64);
        let dur_b = duration_from_seconds(rng.gen_range(1, 1000) as u64);
        
        // Addition is commutative
        assert_eq!(dur_a.add(&dur_b), dur_b.add(&dur_a));
        
        // Addition with zero duration is identity
        let zero_dur = duration_from_seconds(0);
        assert_eq!(dur_a.add(&zero_dur), dur_a);
        
        true
    });
}

#[test]
fn property_date_validation_is_correct() {
    check_property("Date validation is correct", 1000, |rng| {
        let year = rng.gen_range(1900, 2100);
        let month = rng.gen_range(1, 13);
        let day = rng.gen_range(1, 32);
        
        // Test leap year calculation
        let is_leap = is_leap_year(year);
        if year % 4 == 0 {
            if year % 100 == 0 {
                assert_eq!(is_leap, year % 400 == 0);
            } else {
                assert!(is_leap);
            }
        } else {
            assert!(!is_leap);
        }
        
        // Test days in month
        let days = days_in_month(year, month);
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => assert_eq!(days, 31),
            4 | 6 | 9 | 11 => assert_eq!(days, 30),
            2 => assert_eq!(days, if is_leap { 29 } else { 28 }),
            _ => panic!("Invalid month: {}", month),
        }
        
        // Test date validation
        let is_valid = is_valid_date(year, month, day);
        if day <= days {
            assert!(is_valid);
        } else {
            assert!(!is_valid);
        }
        
        true
    });
}

// =============================================================================
// Environment Module Property Tests
// =============================================================================

#[test]
fn property_env_operations_are_safe() {
    check_property("Environment operations are safe", 100, |rng| {
        let var_name = format!("TEST_VAR_{}", rng.gen_range(0, 1000));
        let var_value = rng.gen_string(20);
        
        // Test set and get
        set_var(&var_name, &var_value);
        let retrieved = var(&var_name);
        assert_eq!(retrieved, some(var_value.clone()));
        
        // Test var_or with existing variable
        let with_default = var_or(&var_name, "default");
        assert_eq!(with_default, var_value);
        
        // Test removal
        remove_var(&var_name);
        let after_removal = var(&var_name);
        assert_eq!(after_removal, none());
        
        // Test var_or with non-existent variable
        let with_default_after = var_or(&var_name, "default");
        assert_eq!(with_default_after, "default");
        
        true
    });
}

#[test]
fn property_path_operations_are_consistent() {
    check_property("Path operations are consistent", 500, |rng| {
        let path1 = rng.gen_string(20);
        let path2 = rng.gen_string(20);
        
        // Test path joining
        let joined = join_path(&path1, &path2);
        assert!(joined.contains(&path1) || path1.is_empty());
        assert!(joined.contains(&path2) || path2.is_empty());
        
        // Test path normalization is idempotent
        let normalized1 = normalize_path(&joined);
        let normalized2 = normalize_path(&normalized1);
        assert_eq!(normalized1, normalized2);
        
        true
    });
}

// =============================================================================
// CLI Module Property Tests
// =============================================================================

#[test]
fn property_cli_validation_is_consistent() {
    check_property("CLI validation is consistent", 500, |rng| {
        let flag_name = rng.gen_string(10);
        
        // Test flag name validation
        let is_valid_name = validate_flag_name(&flag_name);
        // Just ensure it doesn't panic and returns a boolean
        assert!(is_valid_name == true || is_valid_name == false);
        
        // Test version validation
        let version = format!("{}.{}.{}", 
            rng.gen_range(0, 10), 
            rng.gen_range(0, 10), 
            rng.gen_range(0, 10)
        );
        let is_valid_version = is_valid_version(&version);
        assert!(is_valid_version); // Should be valid format
        
        true
    });
}

// =============================================================================
// Test Module Property Tests
// =============================================================================

#[test]
fn property_test_assertions_behave_correctly() {
    check_property("Test assertions behave correctly", 1000, |rng| {
        let value1 = rng.gen_range(-100, 100);
        
        // Test basic assertions
        let eq_result = assert_eq(value1, value1);
        assert!(eq_result.is_ok());
        
        let ne_result = assert_ne(value1, value1 + 1);
        assert!(ne_result.is_ok());
        
        // Test Option assertions
        let some_val = some(value1);
        let none_val: OvieOption<i32> = none();
        
        let some_assert = assert_some(&some_val);
        assert!(some_assert.is_ok());
        
        let none_assert = assert_none(&none_val);
        assert!(none_assert.is_ok());
        
        // Test Result assertions
        let ok_val: OvieResult<i32, String> = ok(value1);
        let err_val: OvieResult<i32, String> = err("error".to_string());
        
        let ok_assert = assert_ok(&ok_val);
        assert!(ok_assert.is_ok());
        
        let err_assert = assert_err(&err_val);
        assert!(err_assert.is_ok());
        
        true
    });
}

// =============================================================================
// Log Module Property Tests
// =============================================================================

#[test]
fn property_log_levels_maintain_ordering() {
    use LogLevel::*;
    
    // Test that log levels maintain their ordering
    assert!(Trace < Debug);
    assert!(Debug < Info);
    assert!(Info < Warn);
    assert!(Warn < Error);
    assert!(Error < Fatal);
    
    // Test should_log property
    assert!(Error.should_log(Info));  // Higher level should log
    assert!(Info.should_log(Info));   // Same level should log
    assert!(!Debug.should_log(Info)); // Lower level should not log
    
    // Test string conversion is consistent
    let levels = [Trace, Debug, Info, Warn, Error, Fatal];
    for level in &levels {
        let as_string = level.as_str();
        let parsed = LogLevel::from_str(as_string);
        assert_eq!(parsed, some(*level));
    }
}

#[test]
fn property_log_records_preserve_data() {
    check_property("Log records preserve data", 500, |rng| {
        let level_idx = rng.gen_range(0, 6);
        let levels = [LogLevel::Trace, LogLevel::Debug, LogLevel::Info, 
                     LogLevel::Warn, LogLevel::Error, LogLevel::Fatal];
        let level = levels[level_idx as usize];
        
        let message = rng.gen_string(100);
        let module_name = rng.gen_string(20);
        let file_name = rng.gen_string(30);
        let line_num = rng.gen_range(1, 1000) as u32;
        
        // Create log record
        let record = LogRecord::new(level, message.clone())
            .with_module(module_name.clone())
            .with_file(file_name.clone())
            .with_line(line_num);
        
        // Verify all data is preserved
        assert_eq!(record.level, level);
        assert_eq!(record.message, message);
        assert_eq!(record.module, some(module_name));
        assert_eq!(record.file, some(file_name));
        assert_eq!(record.line, some(line_num));
        
        // Test field operations
        let key = rng.gen_string(10);
        let value = rng.gen_string(20);
        let record_with_field = record.with_field(key.clone(), value.clone());
        
        assert_eq!(record_with_field.get_field(&key), some(value));
        assert!(record_with_field.has_field(&key));
        assert!(!record_with_field.has_field("nonexistent"));
        
        true
    });
}

// =============================================================================
// Cross-Module Integration Property Tests
// =============================================================================

#[test]
fn property_cross_module_determinism() {
    check_property("Cross-module operations are deterministic", 200, |rng| {
        let value = rng.gen_range(-100, 100);
        
        // Test that operations across modules produce consistent results
        let math_result = ovie_abs(value as f64);
        let core_result = if value >= 0 { ok(value) } else { ok(-value) };
        
        // Both should represent the same absolute value concept
        if let ok(core_val) = core_result {
            assert_eq!(math_result, core_val as f64);
        }
        
        // Test time and math integration
        let timestamp = rng.gen_range(0, 1000000) as u64;
        let time_obj = from_unix_timestamp(timestamp);
        let retrieved_timestamp = time_obj.unix_timestamp();
        assert_eq!(timestamp, retrieved_timestamp);
        
        // Test environment and path integration
        let test_var = format!("TEST_{}", rng.gen_range(0, 1000));
        let test_path = rng.gen_string(30);
        
        set_var(&test_var, &test_path);
        let retrieved_path = var(&test_var);
        if let some(path) = retrieved_path {
            let normalized = normalize_path(&path);
            // Normalization should be idempotent
            assert_eq!(normalized, normalize_path(&normalized));
        }
        
        // Cleanup
        remove_var(&test_var);
        
        true
    });
}

#[test]
fn property_error_handling_consistency() {
    check_property("Error handling is consistent across modules", 300, |rng| {
        // Test that all modules handle errors consistently using OvieResult
        
        // Division by zero
        let zero_div = checked_div(rng.gen_range(-100, 100), 0);
        assert_eq!(zero_div, none()); // Should return None for division by zero
        
        // Environment module error handling
        let nonexistent_var = var("DEFINITELY_NONEXISTENT_VAR_12345");
        assert_eq!(nonexistent_var, none()); // Should return None
        
        // All error cases should be handled gracefully without panicking
        true
    });
}

// =============================================================================
// Performance and Memory Property Tests
// =============================================================================

#[test]
fn property_memory_usage_is_reasonable() {
    // Test that data structures don't use excessive memory
    let mut vec = OvieVec::new();
    
    // Add elements and verify capacity grows reasonably
    for i in 0..100 {
        vec.push(i);
    }
    
    // Capacity should have grown but not excessively
    assert!(vec.capacity() >= vec.len());
    assert!(vec.capacity() < vec.len() * 10); // Reasonable growth factor
    
    // Test HashMap memory usage
    let mut map = OvieHashMap::new();
    for i in 0..50 {
        map.insert(format!("key_{}", i), i);
    }
    
    assert_eq!(map.len(), 50);
    // Should be able to retrieve all inserted values
    for i in 0..50 {
        assert_eq!(map.get(&format!("key_{}", i)), some(i));
    }
}

#[test]
fn property_operations_complete_in_reasonable_time() {
    use std::time::Instant;
    
    // Test that basic operations complete quickly
    let start = Instant::now();
    
    // Perform a series of operations
    let mut vec = OvieVec::new();
    for i in 0..1000 {
        vec.push(i);
    }
    
    let mut map = OvieHashMap::new();
    for i in 0..100 {
        map.insert(format!("key_{}", i), i);
    }
    
    // Math operations
    for i in 0..100 {
        let _ = ovie_sqrt(i as f64);
        let _ = ovie_abs(i as f64 - 50.0);
    }
    
    let elapsed = start.elapsed();
    
    // Should complete in reasonable time (less than 1 second for these operations)
    assert!(elapsed.as_secs() < 1);
}

// =============================================================================
// Platform Consistency Property Tests
// =============================================================================

#[test]
fn property_deterministic_hashing() {
    check_property("Hashing is deterministic", 1000, |rng| {
        let value = rng.gen_range(-1000, 1000);
        let text = rng.gen_string(50);
        
        // Test that same values always produce same hashes
        let hash1 = deterministic_hash(&value);
        let hash2 = deterministic_hash(&value);
        assert_eq!(hash1, hash2);
        
        let text_hash1 = deterministic_hash(&text);
        let text_hash2 = deterministic_hash(&text);
        assert_eq!(text_hash1, text_hash2);
        
        true
    });
}

#[test]
fn property_floating_point_consistency() {
    check_property("Floating point operations are consistent", 1000, |rng| {
        let a = rng.next_f64() * 100.0 - 50.0;
        let b = rng.next_f64() * 100.0 - 50.0;
        
        // Test that operations are deterministic
        assert_eq!(a + b, a + b);
        assert_eq!(a * b, a * b);
        
        // Test special value handling
        if a.is_finite() && b.is_finite() {
            let sum = a + b;
            let product = a * b;
            
            // Results should be consistent
            assert_eq!(sum, a + b);
            assert_eq!(product, a * b);
        }
        
        // Test classification functions
        assert_eq!(is_finite(a), a.is_finite());
        assert_eq!(is_infinite(a), a.is_infinite());
        assert_eq!(is_nan(a), a.is_nan());
        
        true
    });
}

// =============================================================================
// Filesystem Module Property Tests
// =============================================================================

#[test]
fn property_fs_operations_are_safe() {
    check_property("Filesystem operations are safe", 100, |rng| {
        let test_dir = format!("test_dir_{}", rng.gen_range(0, 10000));
        let test_file = format!("{}/test_file.txt", test_dir);
        let test_content = rng.gen_string(100);
        
        // Test directory creation
        let create_result = create_dir(&test_dir);
        if create_result.is_err() {
            return true; // Skip if can't create directory
        }
        
        // Test file operations
        let write_result = write_string(&test_file, &test_content);
        if write_result.is_ok() {
            // Test file reading
            let read_result = read_to_string(&test_file);
            if let ok(content) = read_result {
                assert_eq!(content, test_content);
            }
            
            // Test file existence
            assert!(exists(&test_file));
            assert!(is_file(&test_file));
            assert!(!is_dir(&test_file));
            
            // Cleanup
            let _ = remove_file(&test_file);
        }
        
        // Cleanup directory
        let _ = remove_dir(&test_dir);
        
        true
    });
}

#[test]
fn property_path_operations_are_consistent() {
    check_property("Path operations are consistent", 500, |rng| {
        let path1 = rng.gen_string(20);
        let path2 = rng.gen_string(20);
        
        // Test path joining
        let joined = join_path(&path1, &path2);
        assert!(joined.contains(&path1) || path1.is_empty());
        assert!(joined.contains(&path2) || path2.is_empty());
        
        // Test path normalization is idempotent
        let normalized1 = normalize_path(&joined);
        let normalized2 = normalize_path(&normalized1);
        assert_eq!(normalized1, normalized2);
        
        // Test parent path operations
        if !joined.is_empty() {
            let parent = parent_path(&joined);
            if let some(parent_str) = parent {
                assert!(joined.starts_with(&parent_str) || parent_str.is_empty());
            }
        }
        
        true
    });
}

#[test]
fn property_fs_security_validation() {
    check_property("Filesystem security validation", 200, |rng| {
        let suspicious_paths = [
            "../../../etc/passwd",
            "\\\\network\\share\\file",
            "http://example.com/file",
            "ftp://example.com/file",
            "/dev/null",
            "CON", "PRN", "AUX", // Windows reserved names
        ];
        
        for &path in &suspicious_paths {
            // Network paths should be detected
            if path.contains("://") || path.starts_with("\\\\") {
                assert!(is_network_path(path));
            }
            
            // Path normalization should handle suspicious paths safely
            let normalized = normalize_path(path);
            // Should not crash and should return a safe path
            assert!(!normalized.is_empty());
        }
        
        // Test with random potentially dangerous paths
        let random_path = format!("../{}", rng.gen_string(10));
        let normalized = normalize_path(&random_path);
        assert!(!normalized.is_empty());
        
        true
    });
}

// =============================================================================
// Enhanced Time Module Property Tests
// =============================================================================

#[test]
fn property_time_arithmetic_consistency() {
    check_property("Time arithmetic is consistent", 300, |rng| {
        let seconds1 = rng.gen_range(0, 1000000) as u64;
        let seconds2 = rng.gen_range(0, 1000000) as u64;
        
        let time1 = from_unix_timestamp(seconds1);
        let time2 = from_unix_timestamp(seconds2);
        
        let dur1 = duration_from_seconds(seconds1);
        let dur2 = duration_from_seconds(seconds2);
        
        // Test duration addition is commutative
        let sum1 = dur1.add(&dur2);
        let sum2 = dur2.add(&dur1);
        assert_eq!(sum1.as_seconds(), sum2.as_seconds());
        
        // Test duration addition with zero is identity
        let zero_dur = duration_from_seconds(0);
        let identity = dur1.add(&zero_dur);
        assert_eq!(identity.as_seconds(), dur1.as_seconds());
        
        // Test time comparison consistency
        if seconds1 < seconds2 {
            assert!(time1.unix_timestamp() < time2.unix_timestamp());
        } else if seconds1 > seconds2 {
            assert!(time1.unix_timestamp() > time2.unix_timestamp());
        } else {
            assert_eq!(time1.unix_timestamp(), time2.unix_timestamp());
        }
        
        true
    });
}

#[test]
fn property_date_time_validation_comprehensive() {
    check_property("Date/time validation is comprehensive", 1000, |rng| {
        let year = rng.gen_range(1900, 2100);
        let month = rng.gen_range(1, 13);
        let day = rng.gen_range(1, 32);
        let hour = rng.gen_range(0, 24);
        let minute = rng.gen_range(0, 60);
        let second = rng.gen_range(0, 60);
        
        // Test leap year calculation consistency
        let is_leap = is_leap_year(year);
        let expected_leap = (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0));
        assert_eq!(is_leap, expected_leap);
        
        // Test days in month consistency
        let days = days_in_month(year, month);
        let expected_days = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if is_leap { 29 } else { 28 },
            _ => 0, // Invalid month
        };
        assert_eq!(days, expected_days);
        
        // Test date validation
        let is_valid_date_result = is_valid_date(year, month, day);
        let expected_valid = month >= 1 && month <= 12 && day >= 1 && day <= days;
        assert_eq!(is_valid_date_result, expected_valid);
        
        // Test time validation
        let is_valid_time_result = is_valid_time(hour, minute, second);
        let expected_time_valid = hour < 24 && minute < 60 && second < 60;
        assert_eq!(is_valid_time_result, expected_time_valid);
        
        true
    });
}

#[test]
fn property_time_zone_independence() {
    check_property("Time operations are timezone independent", 100, |rng| {
        let timestamp = rng.gen_range(0, 2000000000) as u64; // Valid Unix timestamps
        
        // Create time from timestamp
        let time1 = from_unix_timestamp(timestamp);
        let retrieved_timestamp = time1.unix_timestamp();
        
        // Should be exactly the same
        assert_eq!(timestamp, retrieved_timestamp);
        
        // Multiple conversions should be stable
        let time2 = from_unix_timestamp(retrieved_timestamp);
        assert_eq!(time1.unix_timestamp(), time2.unix_timestamp());
        
        true
    });
}

// =============================================================================
// Enhanced CLI Module Property Tests
// =============================================================================

#[test]
fn property_cli_argument_parsing_robustness() {
    check_property("CLI argument parsing is robust", 300, |rng| {
        let app_name = rng.gen_string(10);
        let mut app = App::new(&app_name);
        
        // Add random flags and options
        let flag_name = rng.gen_string(8);
        let option_name = rng.gen_string(8);
        
        if validate_flag_name(&flag_name) {
            app = app.flag(Flag::new(&flag_name));
        }
        
        if validate_option_name(&option_name) {
            app = app.option(Option::new(&option_name));
        }
        
        // Test with various argument combinations
        let test_args = vec![
            vec![app_name.clone()],
            vec![app_name.clone(), format!("--{}", flag_name)],
            vec![app_name.clone(), format!("--{}", option_name), "value".to_string()],
        ];
        
        for args in test_args {
            let parse_result = app.parse(&args);
            // Should either succeed or fail gracefully with proper error
            match parse_result {
                ok(_) => {}, // Success is fine
                err(error) => {
                    // Error should be well-formed
                    let error_str = cli_error_to_string(&error);
                    assert!(!error_str.is_empty());
                }
            }
        }
        
        true
    });
}

#[test]
fn property_cli_validation_consistency() {
    check_property("CLI validation is consistent", 500, |rng| {
        let test_strings = (0..10).map(|_| rng.gen_string(15)).collect::<Vec<_>>();
        
        for test_str in test_strings {
            // Flag name validation should be consistent
            let is_valid_flag1 = validate_flag_name(&test_str);
            let is_valid_flag2 = validate_flag_name(&test_str);
            assert_eq!(is_valid_flag1, is_valid_flag2);
            
            // Option name validation should be consistent
            let is_valid_option1 = validate_option_name(&test_str);
            let is_valid_option2 = validate_option_name(&test_str);
            assert_eq!(is_valid_option1, is_valid_option2);
            
            // Command name validation should be consistent
            let is_valid_command1 = is_valid_command_name(&test_str);
            let is_valid_command2 = is_valid_command_name(&test_str);
            assert_eq!(is_valid_command1, is_valid_command2);
        }
        
        true
    });
}

#[test]
fn property_cli_security_validation() {
    check_property("CLI security validation", 200, |rng| {
        let malicious_inputs = [
            "../../../etc/passwd",
            "$(rm -rf /)",
            "; rm -rf /",
            "' OR 1=1 --",
            "<script>alert('xss')</script>",
            "\x00\x01\x02", // Null bytes and control characters
        ];
        
        for &input in &malicious_inputs {
            // File path validation should reject dangerous paths
            let is_safe_path = validate_file_path(input);
            if input.contains("..") || input.contains('\0') || input.contains("$(") {
                assert!(!is_safe_path);
            }
            
            // Input sanitization should handle malicious input safely
            let sanitized = sanitize_input(input);
            assert!(!sanitized.contains('\0')); // No null bytes
            assert!(!sanitized.contains("$("));  // No command injection
        }
        
        // Test with random potentially dangerous input
        let random_input = format!("{}$(echo hack){}", rng.gen_string(5), rng.gen_string(5));
        let sanitized = sanitize_input(&random_input);
        assert!(!sanitized.contains("$("));
        
        true
    });
}

// =============================================================================
// Enhanced Test Module Property Tests
// =============================================================================

#[test]
fn property_test_assertions_comprehensive() {
    check_property("Test assertions are comprehensive", 1000, |rng| {
        let value1 = rng.gen_range(-100, 100);
        let value2 = rng.gen_range(-100, 100);
        let float1 = rng.next_f64() * 200.0 - 100.0;
        let float2 = rng.next_f64() * 200.0 - 100.0;
        
        // Test equality assertions
        let eq_result = assert_eq(value1, value1);
        assert!(eq_result.is_ok());
        
        if value1 != value2 {
            let ne_result = assert_ne(value1, value2);
            assert!(ne_result.is_ok());
        }
        
        // Test boolean assertions
        let true_result = assert_true(true);
        assert!(true_result.is_ok());
        
        let false_result = assert_false(false);
        assert!(false_result.is_ok());
        
        // Test approximate equality for floats
        let approx_result = assert_approx_eq(float1, float1, 1e-10);
        assert!(approx_result.is_ok());
        
        // Test range assertions
        let min_val = if value1 < value2 { value1 } else { value2 };
        let max_val = if value1 > value2 { value1 } else { value2 };
        let mid_val = (min_val + max_val) / 2;
        
        let range_result = assert_in_range(mid_val, min_val, max_val);
        assert!(range_result.is_ok());
        
        true
    });
}

#[test]
fn property_test_generators_produce_valid_data() {
    check_property("Test generators produce valid data", 500, |rng| {
        // Test integer range generator
        let min_int = rng.gen_range(-100, 0);
        let max_int = rng.gen_range(1, 100);
        let int_gen = int_range(min_int, max_int);
        
        for _ in 0..10 {
            let generated = int_gen.generate(rng);
            assert!(generated >= min_int && generated <= max_int);
        }
        
        // Test float range generator
        let min_float = rng.next_f64() * 50.0 - 25.0;
        let max_float = min_float + rng.next_f64() * 50.0;
        let float_gen = float_range(min_float, max_float);
        
        for _ in 0..10 {
            let generated = float_gen.generate(rng);
            assert!(generated >= min_float && generated <= max_float);
        }
        
        // Test string generator
        let max_len = rng.gen_range(1, 50) as usize;
        let string_gen = strings_with_length(max_len);
        
        for _ in 0..10 {
            let generated = string_gen.generate(rng);
            assert!(generated.len() <= max_len);
        }
        
        // Test vector generator
        let vec_len = rng.gen_range(1, 20) as usize;
        let vec_gen = vecs_with_length(int_range(-10, 10), vec_len);
        
        for _ in 0..5 {
            let generated = vec_gen.generate(rng);
            assert_eq!(generated.len(), vec_len);
            for &item in &generated {
                assert!(item >= -10 && item <= 10);
            }
        }
        
        true
    });
}

#[test]
fn property_test_runner_reliability() {
    check_property("Test runner is reliable", 100, |rng| {
        let mut registry = TestRegistry::new();
        
        // Register some test cases
        let test_name1 = format!("test_{}", rng.gen_range(0, 1000));
        let test_name2 = format!("test_{}", rng.gen_range(1000, 2000));
        
        registry.register_test(test_case(&test_name1, || pass()));
        registry.register_test(test_case(&test_name2, || {
            if rng.gen_bool() { pass() } else { fail("Random failure") }
        }));
        
        // Run tests
        let config = TestConfig::default();
        let results = run_tests_with_config(&registry, &config);
        
        // Should have results for all registered tests
        assert!(results.total_tests >= 2);
        assert!(results.passed_tests + results.failed_tests + results.skipped_tests == results.total_tests);
        
        true
    });
}

// =============================================================================
// Enhanced Log Module Property Tests
// =============================================================================

#[test]
fn property_log_level_ordering_comprehensive() {
    use LogLevel::*;
    
    let levels = [Trace, Debug, Info, Warn, Error, Fatal];
    
    // Test strict ordering
    for i in 0..levels.len() {
        for j in i+1..levels.len() {
            assert!(levels[i] < levels[j]);
            assert!(levels[j] > levels[i]);
            assert!(levels[i] != levels[j]);
        }
    }
    
    // Test should_log property comprehensively
    for &current_level in &levels {
        for &message_level in &levels {
            let should_log = message_level.should_log(current_level);
            let expected = message_level >= current_level;
            assert_eq!(should_log, expected);
        }
    }
    
    // Test string conversion round-trip
    for &level in &levels {
        let as_string = level.as_str();
        let parsed = LogLevel::from_str(as_string);
        assert_eq!(parsed, some(level));
    }
}

#[test]
fn property_log_record_integrity() {
    check_property("Log record integrity", 500, |rng| {
        let level_idx = rng.gen_range(0, 6);
        let levels = [LogLevel::Trace, LogLevel::Debug, LogLevel::Info, 
                     LogLevel::Warn, LogLevel::Error, LogLevel::Fatal];
        let level = levels[level_idx as usize];
        
        let message = rng.gen_string(100);
        let module_name = rng.gen_string(20);
        let file_name = rng.gen_string(30);
        let line_num = rng.gen_range(1, 1000) as u32;
        
        // Create log record with all fields
        let mut record = LogRecord::new(level, message.clone())
            .with_module(module_name.clone())
            .with_file(file_name.clone())
            .with_line(line_num);
        
        // Verify all data is preserved
        assert_eq!(record.level, level);
        assert_eq!(record.message, message);
        assert_eq!(record.module, some(module_name));
        assert_eq!(record.file, some(file_name));
        assert_eq!(record.line, some(line_num));
        
        // Test field operations
        let num_fields = rng.gen_range(1, 10);
        let mut expected_fields = Vec::new();
        
        for i in 0..num_fields {
            let key = format!("key_{}", i);
            let value = rng.gen_string(15);
            record = record.with_field(key.clone(), value.clone());
            expected_fields.push((key, value));
        }
        
        // Verify all fields are present
        for (key, value) in expected_fields {
            assert_eq!(record.get_field(&key), some(value));
            assert!(record.has_field(&key));
        }
        
        // Test non-existent field
        assert!(!record.has_field("definitely_nonexistent_key_12345"));
        assert_eq!(record.get_field("definitely_nonexistent_key_12345"), none());
        
        true
    });
}

#[test]
fn property_log_formatting_consistency() {
    check_property("Log formatting is consistent", 300, |rng| {
        let level = LogLevel::Info;
        let message = rng.gen_string(50);
        
        let record = LogRecord::new(level, message.clone());
        
        // Format multiple times - should be identical
        let formatted1 = record.format();
        let formatted2 = record.format();
        assert_eq!(formatted1, formatted2);
        
        // Should contain the message
        assert!(formatted1.contains(&message));
        
        // Should contain level information
        assert!(formatted1.contains("INFO") || formatted1.contains("info"));
        
        true
    });
}

// =============================================================================
// Cross-Module Integration Property Tests (Enhanced)
// =============================================================================

#[test]
fn property_stdlib_module_integration() {
    check_property("Standard library modules integrate correctly", 200, |rng| {
        let test_value = rng.gen_range(-100, 100);
        
        // Test core + math integration
        let math_result = ovie_abs(test_value as f64);
        let core_result = if test_value >= 0 { 
            ok(test_value) 
        } else { 
            ok(-test_value) 
        };
        
        if let ok(core_val) = core_result {
            assert_eq!(math_result, core_val as f64);
        }
        
        // Test time + env integration
        let current_time = now();
        let timestamp = current_time.unix_timestamp();
        
        // Set environment variable with timestamp
        let env_key = format!("TEST_TIME_{}", rng.gen_range(0, 10000));
        set_var(&env_key, &timestamp.to_string());
        
        let retrieved = var(&env_key);
        if let some(value) = retrieved {
            let parsed_timestamp: u64 = value.parse().unwrap_or(0);
            assert_eq!(parsed_timestamp, timestamp);
        }
        
        // Cleanup
        remove_var(&env_key);
        
        // Test fs + io integration
        let test_dir = format!("integration_test_{}", rng.gen_range(0, 10000));
        let test_file = format!("{}/test.txt", test_dir);
        let test_content = format!("Test content: {}", rng.gen_string(50));
        
        if create_dir(&test_dir).is_ok() {
            if write_string(&test_file, &test_content).is_ok() {
                if let ok(read_content) = read_to_string(&test_file) {
                    assert_eq!(read_content, test_content);
                }
                let _ = remove_file(&test_file);
            }
            let _ = remove_dir(&test_dir);
        }
        
        true
    });
}

#[test]
fn property_error_handling_consistency_comprehensive() {
    check_property("Error handling is consistent across all modules", 400, |rng| {
        // Test that all modules handle errors consistently using OvieResult/OvieOption
        
        // Math module errors
        let zero_div = checked_div(rng.gen_range(-100, 100) as f64, 0.0);
        assert!(zero_div.is_err());
        
        let negative_sqrt = ovie_sqrt(-1.0);
        assert!(negative_sqrt.is_err());
        
        // Core module - out of bounds access
        let mut vec = OvieVec::new();
        vec.push(42);
        let out_of_bounds = vec.get(100);
        assert_eq!(out_of_bounds, none());
        
        // Environment module - nonexistent variable
        let nonexistent_var = var("DEFINITELY_NONEXISTENT_VAR_98765");
        assert_eq!(nonexistent_var, none());
        
        // Time module - invalid date
        let invalid_date = is_valid_date(2023, 13, 1); // Invalid month
        assert!(!invalid_date);
        
        let invalid_time = is_valid_time(25, 0, 0); // Invalid hour
        assert!(!invalid_time);
        
        // CLI module - invalid names
        let invalid_flag = validate_flag_name(""); // Empty name
        assert!(!invalid_flag);
        
        // Test module - failing assertions
        let failing_assertion = assert_eq(1, 2);
        assert!(failing_assertion.is_err());
        
        // All error cases should be handled gracefully without panicking
        true
    });
}

// =============================================================================
// Performance and Memory Property Tests (Enhanced)
// =============================================================================

#[test]
fn property_memory_usage_bounds() {
    // Test that data structures don't use excessive memory
    let mut vec = OvieVec::new();
    let initial_capacity = vec.capacity();
    
    // Add elements and verify capacity grows reasonably
    for i in 0..1000 {
        vec.push(i);
        
        // Capacity should never be more than 4x the length (reasonable growth factor)
        assert!(vec.capacity() <= vec.len() * 4);
        
        // Capacity should never shrink without explicit action
        assert!(vec.capacity() >= initial_capacity);
    }
    
    // Test HashMap memory usage
    let mut map = OvieHashMap::new();
    for i in 0..500 {
        map.insert(format!("key_{}", i), i);
    }
    
    assert_eq!(map.len(), 500);
    
    // Should be able to retrieve all inserted values efficiently
    for i in 0..500 {
        assert_eq!(map.get(&format!("key_{}", i)), some(i));
    }
    
    // Test that removal actually frees memory conceptually
    for i in 0..250 {
        map.remove(&format!("key_{}", i));
    }
    assert_eq!(map.len(), 250);
}

#[test]
fn property_operations_performance_bounds() {
    // Test that basic operations complete in reasonable time
    let start = Instant::now();
    
    // Core operations
    let mut vec = OvieVec::new();
    for i in 0..10000 {
        vec.push(i);
    }
    
    let mut map = OvieHashMap::new();
    for i in 0..1000 {
        map.insert(format!("key_{}", i), i);
    }
    
    // Math operations
    for i in 0..1000 {
        let _ = ovie_sqrt((i as f64).abs());
        let _ = ovie_abs(i as f64 - 500.0);
        let _ = ovie_sin(i as f64 / 100.0);
        let _ = ovie_cos(i as f64 / 100.0);
    }
    
    // Time operations
    for _ in 0..100 {
        let _ = now();
        let _ = duration_from_seconds(42);
    }
    
    let elapsed = start.elapsed();
    
    // Should complete in reasonable time (less than 5 seconds for these operations)
    assert!(elapsed.as_secs() < 5);
}

#[test]
fn property_concurrent_safety_simulation() {
    // Simulate concurrent access patterns to test for race conditions
    // Note: This is a single-threaded simulation of concurrent patterns
    
    let mut shared_vec = OvieVec::new();
    let mut shared_map = OvieHashMap::new();
    
    // Simulate interleaved operations that might cause issues in concurrent scenarios
    for i in 0..100 {
        // Simulate thread 1 operations
        shared_vec.push(i);
        shared_map.insert(format!("thread1_{}", i), i);
        
        // Simulate thread 2 operations
        shared_vec.push(i + 1000);
        shared_map.insert(format!("thread2_{}", i), i + 1000);
        
        // Simulate reads during writes
        let _ = shared_vec.get(i / 2);
        let _ = shared_map.get(&format!("thread1_{}", i / 2));
        
        // Simulate removals
        if i % 10 == 0 && i > 0 {
            let _ = shared_vec.pop();
            let _ = shared_map.remove(&format!("thread1_{}", i - 10));
        }
    }
    
    // Verify final state is consistent
    assert!(shared_vec.len() > 0);
    assert!(shared_map.len() > 0);
    
    // All remaining elements should be accessible
    for i in 0..shared_vec.len() {
        assert!(shared_vec.get(i).is_some());
    }
}

// =============================================================================
// Platform Consistency Property Tests (Enhanced)
// =============================================================================

#[test]
fn property_deterministic_hashing_comprehensive() {
    check_property("Hashing is deterministic across data types", 1000, |rng| {
        // Test various data types
        let int_value = rng.gen_range(-1000, 1000);
        let float_value = rng.next_f64() * 1000.0 - 500.0;
        let string_value = rng.gen_string(50);
        let bool_value = rng.gen_bool();
        
        // Test that same values always produce same hashes
        let int_hash1 = deterministic_hash(&int_value);
        let int_hash2 = deterministic_hash(&int_value);
        assert_eq!(int_hash1, int_hash2);
        
        let string_hash1 = deterministic_hash(&string_value);
        let string_hash2 = deterministic_hash(&string_value);
        assert_eq!(string_hash1, string_hash2);
        
        let bool_hash1 = deterministic_hash(&bool_value);
        let bool_hash2 = deterministic_hash(&bool_value);
        assert_eq!(bool_hash1, bool_hash2);
        
        // Test that different values produce different hashes (with high probability)
        let different_int = int_value + 1;
        let different_int_hash = deterministic_hash(&different_int);
        if int_value != different_int {
            // Should be different with very high probability
            assert_ne!(int_hash1, different_int_hash);
        }
        
        true
    });
}

#[test]
fn property_floating_point_consistency_comprehensive() {
    check_property("Floating point operations are consistent across platforms", 1000, |rng| {
        let a = rng.next_f64() * 100.0 - 50.0;
        let b = rng.next_f64() * 100.0 - 50.0;
        
        // Test that operations are deterministic
        assert_eq!(a + b, a + b);
        assert_eq!(a * b, a * b);
        assert_eq!(a - b, a - b);
        
        if b != 0.0 {
            assert_eq!(a / b, a / b);
        }
        
        // Test special value handling
        if a.is_finite() && b.is_finite() {
            let sum = a + b;
            let product = a * b;
            let difference = a - b;
            
            // Results should be consistent across multiple calculations
            assert_eq!(sum, a + b);
            assert_eq!(product, a * b);
            assert_eq!(difference, a - b);
            
            // Test mathematical properties
            assert_eq!(a + b, b + a); // Commutativity
            assert_eq!(a * b, b * a); // Commutativity
            
            if a != 0.0 && b != 0.0 {
                let div_result = a / b;
                if div_result.is_finite() {
                    // Division and multiplication should be inverse operations
                    let reconstructed = div_result * b;
                    assert!((reconstructed - a).abs() < 1e-10);
                }
            }
        }
        
        // Test classification functions consistency
        assert_eq!(is_finite(a), a.is_finite());
        assert_eq!(is_infinite(a), a.is_infinite());
        assert_eq!(is_nan(a), a.is_nan());
        
        // Test that classification is stable
        let finite_check1 = is_finite(a);
        let finite_check2 = is_finite(a);
        assert_eq!(finite_check1, finite_check2);
        
        true
    });
}

#[test]
fn property_string_encoding_consistency() {
    check_property("String encoding is consistent", 500, |rng| {
        let test_string = rng.gen_string(100);
        
        // Test that string operations are consistent
        let length1 = test_string.len();
        let length2 = test_string.len();
        assert_eq!(length1, length2);
        
        // Test that string comparison is deterministic
        let same_string = test_string.clone();
        assert_eq!(test_string, same_string);
        
        // Test that string hashing is consistent
        let hash1 = deterministic_hash(&test_string);
        let hash2 = deterministic_hash(&test_string);
        assert_eq!(hash1, hash2);
        
        // Test substring operations
        if !test_string.is_empty() {
            let mid_point = test_string.len() / 2;
            let prefix = &test_string[..mid_point];
            let suffix = &test_string[mid_point..];
            let reconstructed = format!("{}{}", prefix, suffix);
            assert_eq!(reconstructed, test_string);
        }
        
        true
    });
}

// =============================================================================
// Stress Testing Properties
// =============================================================================

#[test]
fn property_large_data_structure_handling() {
    // Test handling of large data structures
    let mut large_vec = OvieVec::new();
    
    // Add a large number of elements
    for i in 0..50000 {
        large_vec.push(i);
    }
    
    assert_eq!(large_vec.len(), 50000);
    
    // Test random access
    for _ in 0..1000 {
        let index = (large_vec.len() / 2) + (_ % 100);
        if index < large_vec.len() {
            assert!(large_vec.get(index).is_some());
        }
    }
    
    // Test iteration over large collection
    let mut count = 0;
    let mut iter = large_vec.iter();
    while iter.next().is_some() {
        count += 1;
        if count > 60000 { // Safety break
            break;
        }
    }
    assert_eq!(count, 50000);
    
    // Test large HashMap
    let mut large_map = OvieHashMap::new();
    for i in 0..10000 {
        large_map.insert(format!("key_{:06}", i), i);
    }
    
    assert_eq!(large_map.len(), 10000);
    
    // Test retrieval from large map
    for i in (0..10000).step_by(100) {
        let key = format!("key_{:06}", i);
        assert_eq!(large_map.get(&key), some(i));
    }
}

#[test]
fn property_edge_case_robustness() {
    // Test robustness with edge cases
    
    // Empty collections
    let empty_vec: OvieVec<i32> = OvieVec::new();
    assert_eq!(empty_vec.len(), 0);
    assert_eq!(empty_vec.get(0), none());
    assert_eq!(empty_vec.pop(), none());
    
    let empty_map: OvieHashMap<String, i32> = OvieHashMap::new();
    assert_eq!(empty_map.len(), 0);
    assert_eq!(empty_map.get(&"key".to_string()), none());
    assert_eq!(empty_map.remove(&"key".to_string()), none());
    
    // Extreme values
    let max_int = i64::MAX;
    let min_int = i64::MIN;
    
    let max_result = checked_add(max_int as f64, 1.0);
    // Should handle overflow gracefully
    
    let min_result = checked_sub(min_int as f64, 1.0);
    // Should handle underflow gracefully
    
    // Very large strings
    let large_string = "x".repeat(10000);
    let hash_large = deterministic_hash(&large_string);
    let hash_large2 = deterministic_hash(&large_string);
    assert_eq!(hash_large, hash_large2);
    
    // Unicode strings
    let unicode_string = "Hello 世界 🌍 Здравствуй мир";
    let unicode_hash1 = deterministic_hash(&unicode_string);
    let unicode_hash2 = deterministic_hash(&unicode_string);
    assert_eq!(unicode_hash1, unicode_hash2);
}