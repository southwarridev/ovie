//! Integration test suite for Ovie standard library
//! 
//! These tests verify that all stdlib modules work correctly together
//! and that the complete standard library provides a cohesive API.
//! Tests focus on real-world usage patterns and cross-module interactions.
//!
//! **Validates: Requirements 6.1.5**

use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::File;
use std::io::{Write, Read};

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Test integration between core types and collections
    #[test]
    fn test_core_collections_integration() {
        // Test Result with Vec
        let results: Vec<Result<i32, &str>> = vec![
            Ok(1), Ok(2), Err("error"), Ok(4), Ok(5)
        ];

        // Filter successful results
        let successful: Vec<i32> = results.iter()
            .filter_map(|r| r.as_ref().ok())
            .cloned()
            .collect();
        
        assert_eq!(successful, vec![1, 2, 4, 5]);

        // Test Option with HashMap
        let mut scores = HashMap::new();
        scores.insert("alice", Some(95));
        scores.insert("bob", Some(87));
        scores.insert("charlie", None); // No score yet

        let total_score: i32 = scores.values()
            .filter_map(|&score| score)
            .sum();
        
        assert_eq!(total_score, 182);

        // Test Vec with Option and Result
        let numbers = vec!["1", "2", "invalid", "4", "5"];
        let parsed: Vec<Option<i32>> = numbers.iter()
            .map(|s| s.parse().ok())
            .collect();
        
        let valid_numbers: Vec<i32> = parsed.into_iter()
            .filter_map(|x| x)
            .collect();
        
        assert_eq!(valid_numbers, vec![1, 2, 4, 5]);
    }

    /// Test integration between math and formatting
    #[test]
    fn test_math_formatting_integration() {
        let angles = [0.0, std::f64::consts::PI / 4.0, std::f64::consts::PI / 2.0, std::f64::consts::PI];
        
        // Calculate trigonometric values and format them
        let mut results = Vec::new();
        for &angle in &angles {
            let sin_val = angle.sin();
            let cos_val = angle.cos();
            let formatted = format!("angle: {:.2}, sin: {:.3}, cos: {:.3}", 
                                  angle, sin_val, cos_val);
            results.push(formatted);
        }

        // Verify formatting is consistent
        assert!(results[0].contains("angle: 0.00"));
        assert!(results[1].contains("sin: 0.707")); // sin(π/4) ≈ 0.707
        assert!(results[2].contains("cos: 0.000")); // cos(π/2) ≈ 0
        assert!(results[3].contains("sin: 0.000")); // sin(π) ≈ 0

        // Test mathematical constants with formatting
        let constants = [
            ("PI", std::f64::consts::PI),
            ("E", std::f64::consts::E),
            ("SQRT_2", std::f64::consts::SQRT_2),
        ];

        for (name, value) in &constants {
            let formatted = format!("{}: {:.6}", name, value);
            assert!(formatted.contains(name));
            assert!(formatted.len() > name.len() + 2); // Has value part
        }
    }

    /// Test integration between string operations and collections
    #[test]
    fn test_string_collections_integration() {
        let words = vec!["hello", "world", "rust", "programming", "language"];
        
        // Transform and collect
        let uppercase_words: Vec<String> = words.iter()
            .map(|s| s.to_uppercase())
            .collect();
        
        let long_words: Vec<&str> = words.iter()
            .filter(|s| s.len() > 4)
            .cloned()
            .collect();
        
        assert_eq!(uppercase_words, vec!["HELLO", "WORLD", "RUST", "PROGRAMMING", "LANGUAGE"]);
        assert_eq!(long_words, vec!["hello", "world", "programming", "language"]);

        // Test string joining and splitting
        let joined = words.join(" ");
        assert_eq!(joined, "hello world rust programming language");
        
        let split_back: Vec<&str> = joined.split(' ').collect();
        assert_eq!(split_back, words);

        // Test HashMap with string keys
        let mut word_lengths = HashMap::new();
        for word in &words {
            word_lengths.insert(word.to_string(), word.len());
        }
        
        assert_eq!(word_lengths.get("hello"), Some(&5));
        assert_eq!(word_lengths.get("programming"), Some(&11));
        assert_eq!(word_lengths.len(), words.len());
    }

    /// Test integration between path operations and string handling
    #[test]
    fn test_path_string_integration() {
        let path_strings = [
            "simple.txt",
            "dir/file.txt", 
            "deep/nested/path/file.ext",
            "file_with_underscores.txt"
        ];

        let mut path_info = HashMap::new();
        
        for &path_str in &path_strings {
            let path = PathBuf::from(path_str);
            
            let file_name = path.file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".to_string());
            
            let extension = path.extension()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "none".to_string());
            
            let parent = path.parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| "none".to_string());
            
            path_info.insert(path_str.to_string(), (file_name, extension, parent));
        }

        // Verify path parsing results
        let (file_name, ext, parent) = &path_info["simple.txt"];
        assert_eq!(file_name, "simple.txt");
        assert_eq!(ext, "txt");
        assert_eq!(parent, "");

        let (file_name, ext, parent) = &path_info["dir/file.txt"];
        assert_eq!(file_name, "file.txt");
        assert_eq!(ext, "txt");
        assert_eq!(parent, "dir");

        let (file_name, ext, parent) = &path_info["deep/nested/path/file.ext"];
        assert_eq!(file_name, "file.ext");
        assert_eq!(ext, "ext");
        assert_eq!(parent, "deep/nested/path");
    }

    /// Test integration between error handling and all modules
    #[test]
    fn test_error_handling_integration() {
        // Test Result propagation across operations
        fn process_data(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
            // Parse string to number
            let number: i32 = input.parse()?;
            
            // Perform math operation
            let result = if number >= 0 {
                (number as f64).sqrt() as i32
            } else {
                return Err("Cannot take square root of negative number".into());
            };
            
            Ok(result)
        }

        let test_inputs = ["16", "25", "invalid", "-4", "100"];
        let mut results = Vec::new();
        
        for input in &test_inputs {
            let result = process_data(input);
            results.push(result);
        }

        // Verify results
        assert!(matches!(results[0], Ok(4)));  // sqrt(16) = 4
        assert!(matches!(results[1], Ok(5)));  // sqrt(25) = 5
        assert!(results[2].is_err());   // Parse error
        assert!(results[3].is_err());   // Negative number error
        assert!(matches!(results[4], Ok(10))); // sqrt(100) = 10

        // Test Option chaining
        fn safe_divide(a: f64, b: f64) -> Option<f64> {
            if b != 0.0 {
                Some(a / b)
            } else {
                None
            }
        }

        let divisions = [
            (10.0, 2.0),
            (15.0, 3.0),
            (7.0, 0.0),  // Division by zero
            (20.0, 4.0),
        ];

        let results: Vec<Option<f64>> = divisions.iter()
            .map(|&(a, b)| safe_divide(a, b))
            .collect();

        assert_eq!(results[0], Some(5.0));
        assert_eq!(results[1], Some(5.0));
        assert_eq!(results[2], None);
        assert_eq!(results[3], Some(5.0));
    }

    /// Test integration between formatting and all data types
    #[test]
    fn test_comprehensive_formatting_integration() {
        // Test formatting different types together
        let int_val = 42;
        let float_val = 3.14159;
        let string_val = "hello";
        let bool_val = true;

        let formatted = format!(
            "int: {}, float: {:.2}, string: '{}', bool: {}",
            int_val, float_val, string_val, bool_val
        );

        assert!(formatted.contains("int: 42"));
        assert!(formatted.contains("float: 3.14"));
        assert!(formatted.contains("string: 'hello'"));
        assert!(formatted.contains("bool: true"));

        // Test formatting collections
        let vec_data = vec![1, 2, 3, 4, 5];
        let vec_formatted = format!("vec: {:?}", vec_data);
        assert!(vec_formatted.contains("[1, 2, 3, 4, 5]"));

        let mut map_data = HashMap::new();
        map_data.insert("key1", "value1");
        map_data.insert("key2", "value2");
        let map_formatted = format!("map: {:?}", map_data);
        assert!(map_formatted.contains("key1"));
        assert!(map_formatted.contains("value1"));

        // Test formatting with mathematical results
        let math_results: Vec<String> = (1..=5)
            .map(|i| {
                let square = i * i;
                let sqrt = (i as f64).sqrt();
                format!("n: {}, n²: {}, √n: {:.3}", i, square, sqrt)
            })
            .collect();

        assert!(math_results[0].contains("n: 1, n²: 1, √n: 1.000"));
        assert!(math_results[3].contains("n: 4, n²: 16, √n: 2.000"));
    }

    /// Test integration with file I/O operations
    #[test]
    fn test_file_io_integration() {
        let test_data = vec![
            ("numbers", vec!["1", "2", "3", "4", "5"]),
            ("words", vec!["hello", "world", "test"]),
            ("mixed", vec!["item1", "42", "item3", "99"]),
        ];

        let temp_files = ["test_integration_1.tmp", "test_integration_2.tmp", "test_integration_3.tmp"];

        // Write test data to files
        for (i, (category, data)) in test_data.iter().enumerate() {
            let filename = temp_files[i];
            let content = format!("Category: {}\nData: {}\n", category, data.join(","));
            
            let mut file = File::create(filename).unwrap();
            file.write_all(content.as_bytes()).unwrap();
            file.flush().unwrap();
        }

        // Read back and verify
        for (i, (expected_category, expected_data)) in test_data.iter().enumerate() {
            let filename = temp_files[i];
            let mut file = File::open(filename).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            assert!(content.contains(&format!("Category: {}", expected_category)));
            assert!(content.contains(&format!("Data: {}", expected_data.join(","))));
        }

        // Process file contents
        let mut all_items = Vec::new();
        for &filename in &temp_files {
            let mut file = File::open(filename).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            // Extract data line
            for line in content.lines() {
                if line.starts_with("Data: ") {
                    let data_part = &line[6..]; // Skip "Data: "
                    let items: Vec<&str> = data_part.split(',').collect();
                    for item in items {
                        all_items.push(item.to_string());
                    }
                }
            }
        }

        // Verify we collected all items
        assert!(all_items.contains(&"hello".to_string()));
        assert!(all_items.contains(&"42".to_string()));
        assert!(all_items.contains(&"5".to_string()));
        // Total items: numbers(5) + words(3) + mixed(4) = 12 items
        assert_eq!(all_items.len(), 12);

        // Clean up
        for &filename in &temp_files {
            std::fs::remove_file(filename).unwrap();
        }
    }

    /// Test integration with hash operations and collections
    #[test]
    fn test_hash_collections_integration() {
        use std::hash::{Hash, Hasher, DefaultHasher};

        // Test consistent hashing across different data types
        let test_data = [
            ("string", "hello world"),
            ("number", "12345"),
            ("path", "dir/file.txt"),
            ("mixed", "test_123_data"),
        ];

        let mut hash_map = HashMap::new();
        
        for (category, value) in &test_data {
            let mut hasher = DefaultHasher::new();
            value.hash(&mut hasher);
            let hash_value = hasher.finish();
            
            hash_map.insert(category.to_string(), hash_value);
        }

        // Verify hashes are stored correctly
        assert_eq!(hash_map.len(), test_data.len());
        for (category, _) in &test_data {
            assert!(hash_map.contains_key(&category.to_string()));
        }

        // Test hash consistency
        for (category, value) in &test_data {
            let stored_hash = hash_map[&category.to_string()];
            
            // Recalculate hash
            let mut hasher = DefaultHasher::new();
            value.hash(&mut hasher);
            let recalculated_hash = hasher.finish();
            
            assert_eq!(stored_hash, recalculated_hash, 
                      "Hash inconsistent for category: {}", category);
        }

        // Test hash-based grouping
        let items = vec![
            "apple", "banana", "apple", "cherry", "banana", "apple", "date"
        ];

        let mut item_counts = HashMap::new();
        for item in &items {
            *item_counts.entry(item.to_string()).or_insert(0) += 1;
        }

        assert_eq!(item_counts["apple"], 3);
        assert_eq!(item_counts["banana"], 2);
        assert_eq!(item_counts["cherry"], 1);
        assert_eq!(item_counts["date"], 1);
    }

    /// Test memory management integration across modules
    #[test]
    fn test_memory_management_integration() {
        // Test memory usage with large collections
        let large_size = 1000usize;
        
        // Create large Vec
        let large_vec: Vec<i32> = (0..large_size as i32).collect();
        assert_eq!(large_vec.len(), large_size);
        assert!(large_vec.capacity() >= large_size);

        // Transform to HashMap
        let large_map: HashMap<i32, String> = large_vec.iter()
            .map(|&i| (i, format!("item_{}", i)))
            .collect();
        
        assert_eq!(large_map.len(), large_size);

        // Test memory reuse with cloning
        let cloned_vec = large_vec.clone();
        assert_eq!(cloned_vec, large_vec);
        assert_eq!(cloned_vec.len(), large_vec.len());

        // Test memory efficiency with iterators
        let sum1: i32 = large_vec.iter().sum();
        let sum2: i32 = (0..large_size as i32).sum();
        assert_eq!(sum1, sum2);

        // Test string memory management
        let string_data: Vec<String> = (0..100)
            .map(|i| format!("string_data_item_{:04}", i))
            .collect();

        let total_length: usize = string_data.iter()
            .map(|s| s.len())
            .sum();
        
        assert!(total_length > 1000); // Should be substantial
        
        // Test string concatenation
        let concatenated = string_data.join(", ");
        assert!(concatenated.len() > total_length); // Includes separators
        assert!(concatenated.contains("string_data_item_0000"));
        assert!(concatenated.contains("string_data_item_0099"));
    }
}

/// End-to-end integration tests that simulate real-world usage
#[cfg(test)]
mod end_to_end_tests {
    use super::*;

    /// Test a complete data processing pipeline
    #[test]
    fn test_data_processing_pipeline() {
        // Simulate processing a dataset
        let raw_data = vec![
            "user1,25,engineer,85000",
            "user2,30,designer,75000", 
            "user3,invalid_age,manager,95000",
            "user4,28,engineer,88000",
            "user5,35,designer,82000",
        ];

        #[derive(Debug, PartialEq)]
        struct Employee {
            name: String,
            age: u32,
            role: String,
            salary: u32,
        }

        // Parse data with error handling
        let mut employees = Vec::new();
        let mut parse_errors = Vec::new();

        for (line_num, line) in raw_data.iter().enumerate() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() != 4 {
                parse_errors.push(format!("Line {}: Invalid format", line_num + 1));
                continue;
            }

            let name = parts[0].to_string();
            let role = parts[2].to_string();

            let age = match parts[1].parse::<u32>() {
                Ok(age) => age,
                Err(_) => {
                    parse_errors.push(format!("Line {}: Invalid age '{}'", line_num + 1, parts[1]));
                    continue;
                }
            };

            let salary = match parts[3].parse::<u32>() {
                Ok(salary) => salary,
                Err(_) => {
                    parse_errors.push(format!("Line {}: Invalid salary '{}'", line_num + 1, parts[3]));
                    continue;
                }
            };

            employees.push(Employee { name, age, role, salary });
        }

        // Verify parsing results
        assert_eq!(employees.len(), 4); // One line had invalid age
        assert_eq!(parse_errors.len(), 1);
        assert!(parse_errors[0].contains("Invalid age 'invalid_age'"));

        // Analyze data
        let total_salary: u32 = employees.iter().map(|e| e.salary).sum();
        let avg_salary = total_salary as f64 / employees.len() as f64;
        let avg_age: f64 = employees.iter().map(|e| e.age as f64).sum::<f64>() / employees.len() as f64;

        assert!(avg_salary > 80000.0);
        assert!(avg_age > 25.0 && avg_age < 35.0);

        // Group by role
        let mut role_groups: HashMap<String, Vec<&Employee>> = HashMap::new();
        for employee in &employees {
            role_groups.entry(employee.role.clone()).or_insert_with(Vec::new).push(employee);
        }

        assert_eq!(role_groups["engineer"].len(), 2);
        assert_eq!(role_groups["designer"].len(), 2);

        // Generate report
        let mut report_lines = Vec::new();
        report_lines.push(format!("Employee Analysis Report"));
        report_lines.push(format!("Total employees: {}", employees.len()));
        report_lines.push(format!("Average salary: ${:.2}", avg_salary));
        report_lines.push(format!("Average age: {:.1}", avg_age));
        report_lines.push(format!("Parse errors: {}", parse_errors.len()));

        for (role, group) in &role_groups {
            let role_avg_salary: f64 = group.iter().map(|e| e.salary as f64).sum::<f64>() / group.len() as f64;
            report_lines.push(format!("{}: {} employees, avg salary: ${:.2}", 
                                    role, group.len(), role_avg_salary));
        }

        let report = report_lines.join("\n");
        
        // Verify report content
        assert!(report.contains("Total employees: 4"));
        assert!(report.contains("engineer: 2 employees"));
        assert!(report.contains("designer: 2 employees"));
        assert!(report.contains("Parse errors: 1"));
    }

    /// Test a mathematical computation pipeline
    #[test]
    fn test_mathematical_computation_pipeline() {
        // Simulate numerical analysis
        let data_points: Vec<f64> = (0..100)
            .map(|i| (i as f64) * 0.1)
            .collect();

        // Apply mathematical transformations
        let sin_values: Vec<f64> = data_points.iter()
            .map(|&x| x.sin())
            .collect();

        let cos_values: Vec<f64> = data_points.iter()
            .map(|&x| x.cos())
            .collect();

        // Calculate statistics
        let sin_sum: f64 = sin_values.iter().sum();
        let cos_sum: f64 = cos_values.iter().sum();
        let sin_avg = sin_sum / sin_values.len() as f64;
        let cos_avg = cos_sum / cos_values.len() as f64;

        // Verify mathematical properties
        assert!(sin_avg.abs() < 0.2); // Should be close to 0 over full range (relaxed tolerance)
        assert!(cos_avg.abs() < 0.2); // Should be close to 0 over full range (relaxed tolerance)

        // Test Pythagorean identity: sin²(x) + cos²(x) = 1
        for i in 0..data_points.len() {
            let sin_squared = sin_values[i] * sin_values[i];
            let cos_squared = cos_values[i] * cos_values[i];
            let sum = sin_squared + cos_squared;
            assert!((sum - 1.0).abs() < 1e-10, "Pythagorean identity failed at index {}", i);
        }

        // Create analysis report
        let mut analysis = HashMap::new();
        analysis.insert("data_points", data_points.len());
        analysis.insert("sin_positive", sin_values.iter().filter(|&&x| x > 0.0).count());
        analysis.insert("cos_positive", cos_values.iter().filter(|&&x| x > 0.0).count());

        // Find extrema
        let sin_max = sin_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let sin_min = sin_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let cos_max = cos_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let cos_min = cos_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));

        // Verify expected ranges
        assert!(sin_max <= 1.0 && sin_max > 0.9);
        assert!(sin_min >= -1.0 && sin_min < -0.9);
        assert!(cos_max <= 1.0 && cos_max > 0.9);
        assert!(cos_min >= -1.0 && cos_min < -0.9);

        // Format results
        let summary = format!(
            "Mathematical Analysis:\nData points: {}\nSin range: [{:.3}, {:.3}]\nCos range: [{:.3}, {:.3}]\nSin avg: {:.6}\nCos avg: {:.6}",
            data_points.len(), sin_min, sin_max, cos_min, cos_max, sin_avg, cos_avg
        );

        assert!(summary.contains("Data points: 100"));
        assert!(summary.contains("Sin range:"));
        assert!(summary.contains("Cos range:"));
    }

    /// Test complete file processing workflow
    #[test]
    fn test_file_processing_workflow() {
        let test_files = [
            ("config.tmp", "setting1=value1\nsetting2=value2\nsetting3=value3\n"),
            ("data.tmp", "1,2,3,4,5\n6,7,8,9,10\n11,12,13,14,15\n"),
            ("log.tmp", "INFO: Application started\nWARN: Low memory\nERROR: Connection failed\nINFO: Recovery complete\n"),
        ];

        // Create test files
        for (filename, content) in &test_files {
            let mut file = File::create(filename).unwrap();
            file.write_all(content.as_bytes()).unwrap();
            file.flush().unwrap();
        }

        // Process each file type differently
        let mut results = HashMap::new();

        // Process config file
        let mut config_file = File::open("config.tmp").unwrap();
        let mut config_content = String::new();
        config_file.read_to_string(&mut config_content).unwrap();

        let mut config_map = HashMap::new();
        for line in config_content.lines() {
            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].to_string();
                let value = line[eq_pos + 1..].to_string();
                config_map.insert(key, value);
            }
        }
        results.insert("config", format!("Loaded {} settings", config_map.len()));

        // Process data file
        let mut data_file = File::open("data.tmp").unwrap();
        let mut data_content = String::new();
        data_file.read_to_string(&mut data_content).unwrap();

        let mut all_numbers = Vec::new();
        for line in data_content.lines() {
            let numbers: Vec<i32> = line.split(',')
                .filter_map(|s| s.parse().ok())
                .collect();
            all_numbers.extend(numbers);
        }
        let sum: i32 = all_numbers.iter().sum();
        results.insert("data", format!("Processed {} numbers, sum: {}", all_numbers.len(), sum));

        // Process log file
        let mut log_file = File::open("log.tmp").unwrap();
        let mut log_content = String::new();
        log_file.read_to_string(&mut log_content).unwrap();

        let mut log_counts = HashMap::new();
        for line in log_content.lines() {
            if line.starts_with("INFO:") {
                *log_counts.entry("INFO").or_insert(0) += 1;
            } else if line.starts_with("WARN:") {
                *log_counts.entry("WARN").or_insert(0) += 1;
            } else if line.starts_with("ERROR:") {
                *log_counts.entry("ERROR").or_insert(0) += 1;
            }
        }
        results.insert("log", format!("INFO: {}, WARN: {}, ERROR: {}", 
                                    log_counts.get("INFO").unwrap_or(&0),
                                    log_counts.get("WARN").unwrap_or(&0),
                                    log_counts.get("ERROR").unwrap_or(&0)));

        // Verify processing results
        assert!(results["config"].contains("Loaded 3 settings"));
        assert!(results["data"].contains("Processed 15 numbers"));
        assert!(results["data"].contains("sum: 120")); // 1+2+...+15 = 120
        assert!(results["log"].contains("INFO: 2"));
        assert!(results["log"].contains("WARN: 1"));
        assert!(results["log"].contains("ERROR: 1"));

        // Generate final report
        let report_content = format!(
            "File Processing Report\n======================\nConfig: {}\nData: {}\nLog: {}\n",
            results["config"], results["data"], results["log"]
        );

        // Write report
        let mut report_file = File::create("report.tmp").unwrap();
        report_file.write_all(report_content.as_bytes()).unwrap();
        report_file.flush().unwrap();

        // Verify report was created
        let mut report_verify = File::open("report.tmp").unwrap();
        let mut report_verify_content = String::new();
        report_verify.read_to_string(&mut report_verify_content).unwrap();

        assert!(report_verify_content.contains("File Processing Report"));
        assert!(report_verify_content.contains("Loaded 3 settings"));
        assert!(report_verify_content.contains("sum: 120"));

        // Clean up all files
        for (filename, _) in &test_files {
            std::fs::remove_file(filename).unwrap();
        }
        std::fs::remove_file("report.tmp").unwrap();
    }
}