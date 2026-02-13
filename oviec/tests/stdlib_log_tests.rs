//! Comprehensive tests for the Ovie Standard Library Log Module
//! 
//! This test suite validates all functionality of the std::log module implementation,
//! including logging levels, structured logging, formatters, destinations, and
//! advanced features like filtering and rotation.

use oviec::stdlib::log::*;
use oviec::stdlib::core::{OvieResult, OvieOption, OvieVec, OvieHashMap, ok, err, some, none};
use std::sync::{Arc, Mutex};

#[test]
fn test_log_level_basic_operations() {
    // Test level ordering
    assert!(LogLevel::Trace < LogLevel::Debug);
    assert!(LogLevel::Debug < LogLevel::Info);
    assert!(LogLevel::Info < LogLevel::Warn);
    assert!(LogLevel::Warn < LogLevel::Error);
    assert!(LogLevel::Error < LogLevel::Fatal);
    
    // Test should_log
    assert!(LogLevel::Error.should_log(LogLevel::Info));
    assert!(LogLevel::Info.should_log(LogLevel::Info));
    assert!(!LogLevel::Debug.should_log(LogLevel::Info));
    
    // Test string conversion
    assert_eq!(LogLevel::Info.as_str(), "INFO");
    assert_eq!(LogLevel::from_str("ERROR"), some(LogLevel::Error));
    assert_eq!(LogLevel::from_str("invalid"), none());
}

#[test]
fn test_log_level_all_levels() {
    let levels = LogLevel::all_levels();
    assert_eq!(levels.len(), 6);
    
    // Verify all levels are present and in order
    assert_eq!(levels.get(0), some(LogLevel::Trace));
    assert_eq!(levels.get(1), some(LogLevel::Debug));
    assert_eq!(levels.get(2), some(LogLevel::Info));
    assert_eq!(levels.get(3), some(LogLevel::Warn));
    assert_eq!(levels.get(4), some(LogLevel::Error));
    assert_eq!(levels.get(5), some(LogLevel::Fatal));
}

#[test]
fn test_log_record_creation_and_modification() {
    let record = LogRecord::new(LogLevel::Info, "test message".to_string());
    
    assert_eq!(record.level, LogLevel::Info);
    assert_eq!(record.message, "test message");
    assert!(record.module.is_none());
    assert!(record.file.is_none());
    assert!(record.line.is_none());
    assert!(record.fields.is_empty());
    
    // Test builder pattern
    let enhanced_record = record
        .with_module("test_module".to_string())
        .with_file("test.rs".to_string())
        .with_line(42)
        .with_field("key1".to_string(), "value1".to_string())
        .with_field("key2".to_string(), "value2".to_string());
    
    assert_eq!(enhanced_record.module, some("test_module".to_string()));
    assert_eq!(enhanced_record.file, some("test.rs".to_string()));
    assert_eq!(enhanced_record.line, some(42));
    assert_eq!(enhanced_record.get_field("key1"), some("value1".to_string()));
    assert_eq!(enhanced_record.get_field("key2"), some("value2".to_string()));
    assert!(enhanced_record.has_field("key1"));
    assert!(!enhanced_record.has_field("nonexistent"));
}

#[test]
fn test_log_record_with_fields() {
    let mut fields = OvieHashMap::new();
    fields.insert("request_id".to_string(), "12345".to_string());
    fields.insert("user_id".to_string(), "67890".to_string());
    
    let record = LogRecord::new(LogLevel::Error, "Database error".to_string())
        .with_fields(fields);
    
    assert_eq!(record.get_field("request_id"), some("12345".to_string()));
    assert_eq!(record.get_field("user_id"), some("67890".to_string()));
    assert_eq!(record.fields.len(), 2);
}

#[test]
fn test_log_config_creation_and_modification() {
    // Test default config
    let config = LogConfig::new();
    assert_eq!(config.min_level, LogLevel::Info);
    assert!(config.enable_colors);
    assert!(config.enable_timestamps);
    
    // Test minimal config
    let minimal = LogConfig::minimal();
    assert_eq!(minimal.min_level, LogLevel::Warn);
    assert!(!minimal.enable_colors);
    assert!(minimal.max_message_length.is_some());
    
    // Test verbose config
    let verbose = LogConfig::verbose();
    assert_eq!(verbose.min_level, LogLevel::Trace);
    assert!(verbose.enable_file_locations);
    assert!(verbose.enable_thread_ids);
    
    // Test builder pattern
    let custom_config = LogConfig::new()
        .with_min_level(LogLevel::Debug)
        .with_colors(false)
        .with_max_message_length(500);
    
    assert_eq!(custom_config.min_level, LogLevel::Debug);
    assert!(!custom_config.enable_colors);
    assert_eq!(custom_config.max_message_length, some(500));
}

#[test]
fn test_log_destination_buffer() {
    let (destination, buffer) = LogDestination::buffer();
    
    // Test writing messages
    assert!(destination.write("message 1").is_ok());
    assert!(destination.write("message 2").is_ok());
    assert!(destination.write("message 3").is_ok());
    
    // Verify buffer contents
    let contents = buffer.lock().unwrap();
    assert_eq!(contents.len(), 3);
    assert_eq!(contents[0], "message 1");
    assert_eq!(contents[1], "message 2");
    assert_eq!(contents[2], "message 3");
}

#[test]
fn test_log_destination_multiple() {
    let (dest1, buffer1) = LogDestination::buffer();
    let (dest2, buffer2) = LogDestination::buffer();
    
    let mut destinations = OvieVec::new();
    destinations.push(dest1);
    destinations.push(dest2);
    
    let multi_dest = LogDestination::multiple(destinations);
    
    // Write to multiple destinations
    assert!(multi_dest.write("test message").is_ok());
    
    // Verify both buffers received the message
    assert_eq!(buffer1.lock().unwrap().len(), 1);
    assert_eq!(buffer2.lock().unwrap().len(), 1);
    assert_eq!(buffer1.lock().unwrap()[0], "test message");
    assert_eq!(buffer2.lock().unwrap()[0], "test message");
}

#[test]
fn test_log_destination_null() {
    let null_dest = LogDestination::Null;
    
    // Null destination should always succeed but discard output
    assert!(null_dest.write("discarded message").is_ok());
}

#[test]
fn test_simple_formatter() {
    let formatter = SimpleFormatter::new();
    let record = LogRecord::new(LogLevel::Info, "test message".to_string())
        .with_module("test_module".to_string())
        .with_field("key".to_string(), "value".to_string());
    
    let formatted = formatter.format(&record);
    
    // Should contain all expected components
    assert!(formatted.contains("INFO"));
    assert!(formatted.contains("test_module:"));
    assert!(formatted.contains("test message"));
    assert!(formatted.contains("[key=value]"));
    
    // Test minimal formatter
    let minimal_formatter = SimpleFormatter::minimal();
    let minimal_formatted = minimal_formatter.format(&record);
    
    // Should be more compact
    assert!(minimal_formatted.len() < formatted.len());
    assert!(minimal_formatted.contains("INFO"));
    assert!(minimal_formatted.contains("test message"));
}

#[test]
fn test_json_formatter() {
    let formatter = JsonFormatter::new();
    let record = LogRecord::new(LogLevel::Error, "error message".to_string())
        .with_module("error_module".to_string())
        .with_field("error_code".to_string(), "E001".to_string());
    
    let formatted = formatter.format(&record);
    
    // Should be JSON-like structure
    assert!(formatted.starts_with("{"));
    assert!(formatted.ends_with("}"));
    assert!(formatted.contains("\"level\":\"ERROR\""));
    assert!(formatted.contains("\"message\":\"error message\""));
    assert!(formatted.contains("\"module\":\"error_module\""));
    assert!(formatted.contains("\"error_code\":\"E001\""));
    
    // Test pretty formatter
    let pretty_formatter = JsonFormatter::pretty();
    let pretty_formatted = pretty_formatter.format(&record);
    
    // Should contain newlines for pretty printing
    assert!(pretty_formatted.contains("\n"));
    assert!(pretty_formatted.len() > formatted.len());
}

#[test]
fn test_json_formatter_escaping() {
    let formatter = JsonFormatter::new();
    let record = LogRecord::new(LogLevel::Info, "message with \"quotes\" and \n newlines".to_string());
    
    let formatted = formatter.format(&record);
    
    // Should properly escape special characters
    assert!(formatted.contains("\\\""));
    assert!(formatted.contains("\\n"));
    assert!(!formatted.contains("\n")); // Raw newlines should be escaped
}

#[test]
fn test_key_value_formatter() {
    let formatter = KeyValueFormatter::new();
    let record = LogRecord::new(LogLevel::Warn, "warning message".to_string())
        .with_field("component".to_string(), "database".to_string())
        .with_field("operation".to_string(), "query".to_string());
    
    let formatted = formatter.format(&record);
    
    // Should contain key=value pairs
    assert!(formatted.contains("level=\"WARN\""));
    assert!(formatted.contains("msg=\"warning message\""));
    assert!(formatted.contains("component=\"database\""));
    assert!(formatted.contains("operation=\"query\""));
    
    // Test simple formatter (no quotes)
    let simple_formatter = KeyValueFormatter::simple();
    let simple_formatted = simple_formatter.format(&record);
    
    assert!(simple_formatted.contains("level=WARN"));
    assert!(simple_formatted.contains("message=warning message"));
    assert!(!simple_formatted.contains("\""));
}

#[test]
fn test_compact_formatter() {
    let formatter = CompactFormatter::new();
    let record = LogRecord::new(LogLevel::Debug, "debug message".to_string());
    
    let formatted = formatter.format(&record);
    
    // Should be compact
    assert!(formatted.contains("D")); // Abbreviated level
    assert!(formatted.contains("debug message"));
    assert!(formatted.len() < 50); // Should be reasonably short
    
    // Test with long message
    let long_record = LogRecord::new(LogLevel::Info, "a".repeat(200));
    let long_formatted = formatter.format(&long_record);
    
    // Should be truncated
    assert!(long_formatted.contains("..."));
    assert!(long_formatted.len() < 200);
}

#[test]
fn test_template_formatter() {
    let formatter = TemplateFormatter::new("[{level}] {message}".to_string());
    let record = LogRecord::new(LogLevel::Info, "info message".to_string());
    
    let formatted = formatter.format(&record);
    assert_eq!(formatted, "[INFO] info message");
    
    // Test with fields
    let field_formatter = TemplateFormatter::new("{level}: {message} {fields}".to_string());
    let field_record = LogRecord::new(LogLevel::Error, "error occurred".to_string())
        .with_field("code".to_string(), "500".to_string());
    
    let field_formatted = field_formatter.format(&field_record);
    assert!(field_formatted.contains("ERROR: error occurred"));
    assert!(field_formatted.contains("[code=500]"));
    
    // Test with pattern
    let pattern_formatter = TemplateFormatter::with_pattern("simple");
    let pattern_formatted = pattern_formatter.format(&record);
    assert!(pattern_formatted.contains("[INFO]"));
    assert!(pattern_formatted.contains("info message"));
}

#[test]
fn test_structured_logger() {
    let logger = StructuredLogger::new()
        .with_module("test_module".to_string())
        .with_field("service".to_string(), "web".to_string())
        .with_field("version".to_string(), "1.0.0".to_string());
    
    let record = logger.create_record(LogLevel::Info, "test message".to_string());
    
    assert_eq!(record.level, LogLevel::Info);
    assert_eq!(record.message, "test message");
    assert_eq!(record.module, some("test_module".to_string()));
    assert_eq!(record.get_field("service"), some("web".to_string()));
    assert_eq!(record.get_field("version"), some("1.0.0".to_string()));
    
    // Test with target
    let target_logger = StructuredLogger::with_target("api".to_string());
    let target_record = target_logger.create_record(LogLevel::Debug, "debug message".to_string());
    assert_eq!(target_record.target, some("api".to_string()));
}

#[test]
fn test_context_logger() {
    let mut logger = ContextLogger::new();
    
    // Test context stacking
    let mut context1 = OvieHashMap::new();
    context1.insert("request_id".to_string(), "123".to_string());
    context1.insert("session_id".to_string(), "abc".to_string());
    logger.push_context(context1);
    
    let mut context2 = OvieHashMap::new();
    context2.insert("user_id".to_string(), "456".to_string());
    context2.insert("request_id".to_string(), "124".to_string()); // Override
    logger.push_context(context2);
    
    let merged = logger.get_merged_context();
    assert_eq!(merged.get(&"request_id".to_string()), some("124".to_string())); // Later context wins
    assert_eq!(merged.get(&"session_id".to_string()), some("abc".to_string()));
    assert_eq!(merged.get(&"user_id".to_string()), some("456".to_string()));
    
    // Test context popping
    let popped = logger.pop_context();
    assert!(popped.is_some());
    
    let merged_after_pop = logger.get_merged_context();
    assert_eq!(merged_after_pop.get(&"request_id".to_string()), some("123".to_string())); // Back to original
    assert_eq!(merged_after_pop.get(&"user_id".to_string()), none()); // Removed
    
    // Test adding fields to current context
    logger.add_context_field("new_field".to_string(), "new_value".to_string());
    let updated_context = logger.get_merged_context();
    assert_eq!(updated_context.get(&"new_field".to_string()), some("new_value".to_string()));
}

#[test]
fn test_log_builder() {
    let builder = info_builder("test message".to_string())
        .field("key1".to_string(), "value1".to_string())
        .field("key2".to_string(), "value2".to_string())
        .module("test_module".to_string())
        .location("test.rs".to_string(), 100);
    
    // Verify builder state
    assert_eq!(builder.level, LogLevel::Info);
    assert_eq!(builder.message, "test message");
    assert_eq!(builder.module, some("test_module".to_string()));
    assert_eq!(builder.file, some("test.rs".to_string()));
    assert_eq!(builder.line, some(100));
    assert_eq!(builder.fields.get(&"key1".to_string()), some("value1".to_string()));
    assert_eq!(builder.fields.get(&"key2".to_string()), some("value2".to_string()));
    
    // Test all builder types
    let trace_builder = trace_builder("trace".to_string());
    assert_eq!(trace_builder.level, LogLevel::Trace);
    
    let debug_builder = debug_builder("debug".to_string());
    assert_eq!(debug_builder.level, LogLevel::Debug);
    
    let warn_builder = warn_builder("warn".to_string());
    assert_eq!(warn_builder.level, LogLevel::Warn);
    
    let error_builder = error_builder("error".to_string());
    assert_eq!(error_builder.level, LogLevel::Error);
    
    let fatal_builder = fatal_builder("fatal".to_string());
    assert_eq!(fatal_builder.level, LogLevel::Fatal);
}

#[test]
fn test_advanced_log_destination_filtering() {
    let (dest, buffer) = LogDestination::buffer();
    let advanced_dest = AdvancedLogDestination::new(
        dest,
        Box::new(SimpleFormatter::new()),
    )
    .with_min_level(LogLevel::Warn)
    .with_max_level(LogLevel::Error)
    .with_module_filter("allowed_module".to_string())
    .with_field_filter("environment".to_string(), "production".to_string());
    
    // Should log: correct level, module, and field
    let record1 = LogRecord::new(LogLevel::Error, "error message".to_string())
        .with_module("allowed_module".to_string())
        .with_field("environment".to_string(), "production".to_string());
    assert!(advanced_dest.should_log(&record1));
    assert!(advanced_dest.log(&record1).is_ok());
    
    // Should not log: level too low
    let record2 = LogRecord::new(LogLevel::Info, "info message".to_string())
        .with_module("allowed_module".to_string())
        .with_field("environment".to_string(), "production".to_string());
    assert!(!advanced_dest.should_log(&record2));
    
    // Should not log: level too high
    let record3 = LogRecord::new(LogLevel::Fatal, "fatal message".to_string())
        .with_module("allowed_module".to_string())
        .with_field("environment".to_string(), "production".to_string());
    assert!(!advanced_dest.should_log(&record3));
    
    // Should not log: wrong module
    let record4 = LogRecord::new(LogLevel::Error, "error message".to_string())
        .with_module("other_module".to_string())
        .with_field("environment".to_string(), "production".to_string());
    assert!(!advanced_dest.should_log(&record4));
    
    // Should not log: wrong field value
    let record5 = LogRecord::new(LogLevel::Error, "error message".to_string())
        .with_module("allowed_module".to_string())
        .with_field("environment".to_string(), "development".to_string());
    assert!(!advanced_dest.should_log(&record5));
    
    // Verify only one message was logged
    assert_eq!(buffer.lock().unwrap().len(), 1);
}

#[test]
fn test_destination_manager() {
    let mut manager = DestinationManager::new();
    
    // Create destinations with different filters
    let (dest1, buffer1) = LogDestination::buffer();
    let advanced_dest1 = AdvancedLogDestination::new(
        dest1,
        Box::new(SimpleFormatter::new()),
    ).with_min_level(LogLevel::Info);
    
    let (dest2, buffer2) = LogDestination::buffer();
    let advanced_dest2 = AdvancedLogDestination::new(
        dest2,
        Box::new(JsonFormatter::new()),
    ).with_min_level(LogLevel::Error);
    
    let (dest3, buffer3) = LogDestination::buffer();
    let advanced_dest3 = AdvancedLogDestination::new(
        dest3,
        Box::new(KeyValueFormatter::new()),
    ).with_module_filter("specific_module".to_string());
    
    manager.add_destination(advanced_dest1);
    manager.add_destination(advanced_dest2);
    manager.add_destination(advanced_dest3);
    
    assert_eq!(manager.destination_count(), 3);
    
    // Log info message - should only go to first destination
    let info_record = LogRecord::new(LogLevel::Info, "info message".to_string());
    assert!(manager.log(&info_record).is_ok());
    
    assert_eq!(buffer1.lock().unwrap().len(), 1);
    assert_eq!(buffer2.lock().unwrap().len(), 0);
    assert_eq!(buffer3.lock().unwrap().len(), 0);
    
    // Log error message - should go to first two destinations
    let error_record = LogRecord::new(LogLevel::Error, "error message".to_string());
    assert!(manager.log(&error_record).is_ok());
    
    assert_eq!(buffer1.lock().unwrap().len(), 2);
    assert_eq!(buffer2.lock().unwrap().len(), 1);
    assert_eq!(buffer3.lock().unwrap().len(), 0);
    
    // Log message with specific module - should go to first and third destinations
    let module_record = LogRecord::new(LogLevel::Info, "module message".to_string())
        .with_module("specific_module".to_string());
    assert!(manager.log(&module_record).is_ok());
    
    assert_eq!(buffer1.lock().unwrap().len(), 3);
    assert_eq!(buffer2.lock().unwrap().len(), 1);
    assert_eq!(buffer3.lock().unwrap().len(), 1);
    
    // Test clearing destinations
    manager.clear();
    assert_eq!(manager.destination_count(), 0);
}

#[test]
fn test_formatter_registry() {
    let mut registry = FormatterRegistry::new();
    
    // Test default formatters
    assert!(registry.get("simple").is_some());
    assert!(registry.get("json").is_some());
    assert!(registry.get("keyvalue").is_some());
    assert!(registry.get("compact").is_some());
    assert!(registry.get("nonexistent").is_none());
    
    // Test custom formatter registration
    registry.register("custom", Box::new(SimpleFormatter::minimal()));
    assert!(registry.get("custom").is_some());
    
    // Test formatter names
    let names = registry.list_formatters();
    assert!(names.len() >= 5); // At least the 4 defaults + 1 custom
    
    // Test formatter usage
    if let some(formatter) = registry.get("simple") {
        let record = LogRecord::new(LogLevel::Info, "test".to_string());
        let formatted = formatter.format(&record);
        assert!(!formatted.is_empty());
        assert!(formatted.contains("INFO"));
    }
}

#[test]
fn test_utility_destination_functions() {
    // Test console destination
    let console_dest = console_destination();
    // Can't easily test console output, but ensure it doesn't panic
    
    // Test file destination
    let file_dest = file_destination("test.log".to_string());
    // File operations would require filesystem access
    
    // Test development destination
    let dev_dest = development_destination();
    let record = LogRecord::new(LogLevel::Debug, "dev message".to_string());
    assert!(dev_dest.should_log(&record)); // Should log debug in development
    
    // Test production destination
    let prod_dest = production_destination("prod.log".to_string());
    let debug_record = LogRecord::new(LogLevel::Debug, "debug message".to_string());
    let info_record = LogRecord::new(LogLevel::Info, "info message".to_string());
    assert!(!prod_dest.should_log(&debug_record)); // Should not log debug in production
    assert!(prod_dest.should_log(&info_record)); // Should log info in production
    
    // Test testing destination
    let (test_dest, test_buffer) = testing_destination();
    let test_record = LogRecord::new(LogLevel::Info, "test message".to_string());
    assert!(test_dest.log(&test_record).is_ok());
    assert_eq!(test_buffer.lock().unwrap().len(), 1);
}

// Property-based tests for logging system
#[test]
fn test_log_level_properties() {
    let levels = LogLevel::all_levels();
    
    // Property: All levels should parse from their string representation
    for level in levels.iter() {
        let level_str = level.as_str();
        let parsed = LogLevel::from_str(level_str);
        assert_eq!(parsed, some(*level));
        
        // Case insensitive parsing
        let lower_parsed = LogLevel::from_str(&level_str.to_lowercase());
        assert_eq!(lower_parsed, some(*level));
    }
    
    // Property: Level ordering should be consistent
    for i in 0..levels.len() {
        for j in i + 1..levels.len() {
            let level1 = levels.get(i).unwrap();
            let level2 = levels.get(j).unwrap();
            assert!(level1 < level2);
            assert!(level2.should_log(level1));
            assert!(!level1.should_log(level2));
        }
    }
    
    // Property: should_log is reflexive
    for level in levels.iter() {
        assert!(level.should_log(*level));
    }
}

#[test]
fn test_log_record_properties() {
    let test_messages = ["", "short", "a very long message with lots of text and special characters !@#$%^&*()"];
    let test_levels = LogLevel::all_levels();
    
    // Property: Record creation should preserve all input data
    for message in &test_messages {
        for level in test_levels.iter() {
            let record = LogRecord::new(*level, message.to_string());
            assert_eq!(record.level, *level);
            assert_eq!(record.message, *message);
            assert!(record.module.is_none());
            assert!(record.fields.is_empty());
        }
    }
    
    // Property: Field operations should be consistent
    let mut record = LogRecord::new(LogLevel::Info, "test".to_string());
    let test_fields = [
        ("key1", "value1"),
        ("key2", "value2"),
        ("key3", "value3"),
        ("empty", ""),
        ("special", "!@#$%^&*()"),
    ];
    
    for (key, value) in &test_fields {
        record = record.with_field(key.to_string(), value.to_string());
        assert_eq!(record.get_field(key), some(value.to_string()));
        assert!(record.has_field(key));
    }
    
    assert_eq!(record.fields.len(), test_fields.len());
    
    // Property: Fields with same key should override
    let overridden_record = record.with_field("key1".to_string(), "new_value".to_string());
    assert_eq!(overridden_record.get_field("key1"), some("new_value".to_string()));
    assert_eq!(overridden_record.fields.len(), test_fields.len()); // Same number of fields
}

#[test]
fn test_formatter_properties() {
    let formatters: Vec<Box<dyn LogFormatter>> = vec![
        Box::new(SimpleFormatter::new()),
        Box::new(JsonFormatter::new()),
        Box::new(KeyValueFormatter::new()),
        Box::new(CompactFormatter::new()),
        Box::new(TemplateFormatter::new("{level}: {message}".to_string())),
    ];
    
    let test_records = vec![
        LogRecord::new(LogLevel::Info, "simple message".to_string()),
        LogRecord::new(LogLevel::Error, "error with fields".to_string())
            .with_field("code".to_string(), "E001".to_string())
            .with_field("details".to_string(), "Database connection failed".to_string()),
        LogRecord::new(LogLevel::Debug, "debug with module".to_string())
            .with_module("test_module".to_string())
            .with_file("test.rs".to_string())
            .with_line(123),
        LogRecord::new(LogLevel::Trace, "".to_string()), // Empty message
        LogRecord::new(LogLevel::Fatal, "message with special chars: !@#$%^&*()".to_string()),
    ];
    
    // Property: All formatters should produce non-empty output for valid records
    for formatter in &formatters {
        for record in &test_records {
            let formatted = formatter.format(record);
            if !record.message.is_empty() {
                assert!(!formatted.is_empty());
            }
            // Message should appear in formatted output (unless empty)
            if !record.message.is_empty() {
                assert!(formatted.contains(&record.message));
            }
        }
    }
    
    // Property: Formatters should be deterministic
    for formatter in &formatters {
        for record in &test_records {
            let formatted1 = formatter.format(record);
            let formatted2 = formatter.format(record);
            assert_eq!(formatted1, formatted2);
        }
    }
    
    // Property: Formatters should handle all log levels
    for formatter in &formatters {
        for level in LogLevel::all_levels().iter() {
            let record = LogRecord::new(*level, "test message".to_string());
            let formatted = formatter.format(&record);
            assert!(formatted.contains(level.as_str()));
        }
    }
}

#[test]
fn test_destination_properties() {
    let (dest, buffer) = LogDestination::buffer();
    let test_messages = ["message1", "message2", "message3", "", "special chars: !@#$%^&*()"];
    
    // Property: All writes should succeed and be stored in order
    for (i, message) in test_messages.iter().enumerate() {
        assert!(dest.write(message).is_ok());
        let buffer_contents = buffer.lock().unwrap();
        assert_eq!(buffer_contents.len(), i + 1);
        assert_eq!(buffer_contents[i], *message);
    }
    
    // Property: Multiple destination should write to all sub-destinations
    let (dest1, buffer1) = LogDestination::buffer();
    let (dest2, buffer2) = LogDestination::buffer();
    let (dest3, buffer3) = LogDestination::buffer();
    
    let mut destinations = OvieVec::new();
    destinations.push(dest1);
    destinations.push(dest2);
    destinations.push(dest3);
    let multi_dest = LogDestination::multiple(destinations);
    
    for message in &test_messages {
        assert!(multi_dest.write(message).is_ok());
    }
    
    // All buffers should have all messages
    for buffer in [&buffer1, &buffer2, &buffer3] {
        let contents = buffer.lock().unwrap();
        assert_eq!(contents.len(), test_messages.len());
        for (i, message) in test_messages.iter().enumerate() {
            assert_eq!(contents[i], *message);
        }
    }
    
    // Property: Null destination should always succeed
    let null_dest = LogDestination::Null;
    for message in &test_messages {
        assert!(null_dest.write(message).is_ok());
    }
}

#[test]
fn test_structured_logging_properties() {
    let base_fields = [
        ("service", "web"),
        ("version", "1.0.0"),
        ("environment", "test"),
    ];
    
    let mut base_field_map = OvieHashMap::new();
    for (key, value) in &base_fields {
        base_field_map.insert(key.to_string(), value.to_string());
    }
    
    let logger = StructuredLogger::new()
        .with_module("test_module".to_string())
        .with_fields(base_field_map);
    
    let test_levels = LogLevel::all_levels();
    let test_messages = ["info", "debug", "error", ""];
    
    // Property: All records should inherit base fields and module
    for level in test_levels.iter() {
        for message in &test_messages {
            let record = logger.create_record(*level, message.to_string());
            
            assert_eq!(record.level, *level);
            assert_eq!(record.message, *message);
            assert_eq!(record.module, some("test_module".to_string()));
            
            // All base fields should be present
            for (key, value) in &base_fields {
                assert_eq!(record.get_field(key), some(value.to_string()));
            }
        }
    }
    
    // Property: Additional fields should be merged with base fields
    let additional_fields = [("request_id", "123"), ("user_id", "456")];
    let mut additional_field_map = OvieHashMap::new();
    for (key, value) in &additional_fields {
        additional_field_map.insert(key.to_string(), value.to_string());
    }
    
    let enhanced_record = logger.create_record(LogLevel::Info, "test".to_string())
        .with_fields(additional_field_map);
    
    // Should have both base and additional fields
    for (key, value) in &base_fields {
        assert_eq!(enhanced_record.get_field(key), some(value.to_string()));
    }
    for (key, value) in &additional_fields {
        assert_eq!(enhanced_record.get_field(key), some(value.to_string()));
    }
}

#[test]
fn test_context_logger_properties() {
    let mut logger = ContextLogger::new();
    
    // Property: Empty context should work
    let empty_context = logger.get_merged_context();
    assert!(empty_context.is_empty());
    
    // Property: Context stacking should preserve order
    let contexts = [
        vec![("ctx1_key1", "ctx1_val1"), ("shared", "ctx1_shared")],
        vec![("ctx2_key1", "ctx2_val1"), ("shared", "ctx2_shared")],
        vec![("ctx3_key1", "ctx3_val1"), ("shared", "ctx3_shared")],
    ];
    
    for context_data in &contexts {
        let mut context = OvieHashMap::new();
        for (key, value) in context_data {
            context.insert(key.to_string(), value.to_string());
        }
        logger.push_context(context);
    }
    
    let merged = logger.get_merged_context();
    
    // Later contexts should override earlier ones
    assert_eq!(merged.get(&"shared".to_string()), some("ctx3_shared".to_string()));
    
    // All unique keys should be present
    assert_eq!(merged.get(&"ctx1_key1".to_string()), some("ctx1_val1".to_string()));
    assert_eq!(merged.get(&"ctx2_key1".to_string()), some("ctx2_val1".to_string()));
    assert_eq!(merged.get(&"ctx3_key1".to_string()), some("ctx3_val1".to_string()));
    
    // Property: Popping should restore previous context
    logger.pop_context();
    let after_pop = logger.get_merged_context();
    assert_eq!(after_pop.get(&"shared".to_string()), some("ctx2_shared".to_string()));
    assert_eq!(after_pop.get(&"ctx3_key1".to_string()), none());
    
    // Property: Popping all contexts should result in empty context
    logger.pop_context();
    logger.pop_context();
    let final_context = logger.get_merged_context();
    assert!(final_context.is_empty());
    
    // Property: Popping from empty context should return None
    let empty_pop = logger.pop_context();
    assert_eq!(empty_pop, none());
}

#[test]
fn test_filtering_properties() {
    let (dest, buffer) = LogDestination::buffer();
    let advanced_dest = AdvancedLogDestination::new(
        dest,
        Box::new(SimpleFormatter::new()),
    )
    .with_min_level(LogLevel::Info)
    .with_max_level(LogLevel::Error);
    
    let all_levels = LogLevel::all_levels();
    let test_message = "test message".to_string();
    
    // Property: Only levels within range should be logged
    for level in all_levels.iter() {
        let record = LogRecord::new(*level, test_message.clone());
        let should_log = advanced_dest.should_log(&record);
        let expected = *level >= LogLevel::Info && *level <= LogLevel::Error;
        assert_eq!(should_log, expected);
    }
    
    // Property: Field filters should be exact matches
    let field_filtered_dest = AdvancedLogDestination::new(
        LogDestination::Null,
        Box::new(SimpleFormatter::new()),
    )
    .with_field_filter("environment".to_string(), "production".to_string());
    
    let test_field_values = ["production", "Production", "prod", "development", ""];
    for value in &test_field_values {
        let record = LogRecord::new(LogLevel::Info, test_message.clone())
            .with_field("environment".to_string(), value.to_string());
        let should_log = field_filtered_dest.should_log(&record);
        let expected = *value == "production"; // Exact match only
        assert_eq!(should_log, expected);
    }
}