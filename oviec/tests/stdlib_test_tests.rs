//! Comprehensive tests for the Ovie standard library test module
//! 
//! This test suite validates the testing framework implementation to ensure
//! it provides reliable and deterministic testing capabilities.

use oviec::stdlib::test::*;
use oviec::stdlib::core::{OvieResult, OvieOption, OvieVec, ok, err, some, none};
use std::time::Duration;

#[test]
fn test_test_result_basic_operations() {
    let pass = TestResult::Pass;
    let fail = TestResult::Fail("error message".to_string());
    let skip = TestResult::Skip("not implemented".to_string());
    
    // Test is_pass, is_fail, is_skip
    assert!(pass.is_pass());
    assert!(!pass.is_fail());
    assert!(!pass.is_skip());
    
    assert!(!fail.is_pass());
    assert!(fail.is_fail());
    assert!(!fail.is_skip());
    
    assert!(!skip.is_pass());
    assert!(!skip.is_fail());
    assert!(skip.is_skip());
    
    // Test message extraction
    assert_eq!(pass.failure_message(), none());
    assert_eq!(fail.failure_message(), some("error message".to_string()));
    assert_eq!(skip.failure_message(), none());
    
    assert_eq!(pass.skip_reason(), none());
    assert_eq!(fail.skip_reason(), none());
    assert_eq!(skip.skip_reason(), some("not implemented".to_string()));
}

#[test]
fn test_test_case_builder_pattern() {
    let test_case = TestCase::new("example_test".to_string())
        .with_description("This is a test description".to_string())
        .with_timeout(5000)
        .should_panic()
        .ignore()
        .with_tag("unit".to_string())
        .with_tag("fast".to_string());
    
    assert_eq!(test_case.name, "example_test");
    assert_eq!(test_case.description, some("This is a test description".to_string()));
    assert_eq!(test_case.timeout_ms, some(5000));
    assert!(test_case.should_panic);
    assert!(test_case.ignore);
    assert_eq!(test_case.tags.len(), 2);
    assert!(test_case.has_tag("unit"));
    assert!(test_case.has_tag("fast"));
    assert!(!test_case.has_tag("slow"));
}

#[test]
fn test_test_stats_accumulation() {
    let mut stats = TestStats::new();
    
    // Initially empty
    assert_eq!(stats.total, 0);
    assert_eq!(stats.passed, 0);
    assert_eq!(stats.failed, 0);
    assert_eq!(stats.skipped, 0);
    assert_eq!(stats.ignored, 0);
    assert!(!stats.all_passed()); // No tests run yet
    
    // Add various results
    stats.add_result(&TestResult::Pass, false);
    stats.add_result(&TestResult::Pass, false);
    stats.add_result(&TestResult::Fail("error".to_string()), false);
    stats.add_result(&TestResult::Skip("not ready".to_string()), false);
    stats.add_result(&TestResult::Pass, true); // ignored test
    
    assert_eq!(stats.total, 5);
    assert_eq!(stats.passed, 2);
    assert_eq!(stats.failed, 1);
    assert_eq!(stats.skipped, 1);
    assert_eq!(stats.ignored, 1);
    assert!(!stats.all_passed()); // Has failures
    
    // Test success rate calculation (ignoring ignored tests)
    // 2 passed out of 4 non-ignored tests = 50%
    let success_rate = stats.success_rate();
    assert!((success_rate - 50.0).abs() < 0.001);
    
    // Test all passed scenario
    let mut all_pass_stats = TestStats::new();
    all_pass_stats.add_result(&TestResult::Pass, false);
    all_pass_stats.add_result(&TestResult::Pass, false);
    assert!(all_pass_stats.all_passed());
    assert!((all_pass_stats.success_rate() - 100.0).abs() < 0.001);
}

#[test]
fn test_test_config_filtering() {
    let config = TestConfig::new()
        .with_filter("unit".to_string())
        .with_tag("fast".to_string())
        .include_ignored();
    
    // Test case that matches filter and tag
    let test1 = TestCase::new("unit_test_1".to_string())
        .with_tag("fast".to_string());
    assert!(config.should_run_test(&test1));
    
    // Test case that doesn't match filter
    let test2 = TestCase::new("integration_test_1".to_string())
        .with_tag("fast".to_string());
    assert!(!config.should_run_test(&test2));
    
    // Test case that doesn't match tag
    let test3 = TestCase::new("unit_test_2".to_string())
        .with_tag("slow".to_string());
    assert!(!config.should_run_test(&test3));
    
    // Ignored test (should run because include_ignored is true)
    let test4 = TestCase::new("unit_test_3".to_string())
        .with_tag("fast".to_string())
        .ignore();
    assert!(config.should_run_test(&test4));
    
    // Test without include_ignored
    let config_no_ignored = TestConfig::new()
        .with_filter("unit".to_string())
        .with_tag("fast".to_string());
    assert!(!config_no_ignored.should_run_test(&test4));
}

#[test]
fn test_test_registry_operations() {
    let mut registry = TestRegistry::new();
    
    assert!(registry.is_empty());
    assert_eq!(registry.len(), 0);
    
    fn dummy_test1() -> TestResult { TestResult::Pass }
    fn dummy_test2() -> TestResult { TestResult::Pass }
    
    let test_case1 = TestCase::new("test_one".to_string())
        .with_tag("unit".to_string());
    let test_case2 = TestCase::new("test_two".to_string())
        .with_tag("integration".to_string());
    
    registry.register(test_case1.clone(), dummy_test1);
    registry.register(test_case2.clone(), dummy_test2);
    
    assert!(!registry.is_empty());
    assert_eq!(registry.len(), 2);
    
    // Test find_tests
    let found = registry.find_tests("one");
    assert_eq!(found.len(), 1);
    
    let found_all = registry.find_tests("test");
    assert_eq!(found_all.len(), 2);
    
    let found_none = registry.find_tests("nonexistent");
    assert_eq!(found_none.len(), 0);
    
    // Test get_tests_with_tags
    let mut unit_tags = OvieVec::new();
    unit_tags.push("unit".to_string());
    let unit_tests = registry.get_tests_with_tags(&unit_tags);
    assert_eq!(unit_tests.len(), 1);
    
    let mut integration_tags = OvieVec::new();
    integration_tags.push("integration".to_string());
    let integration_tests = registry.get_tests_with_tags(&integration_tags);
    assert_eq!(integration_tests.len(), 1);
    
    let mut nonexistent_tags = OvieVec::new();
    nonexistent_tags.push("nonexistent".to_string());
    let no_tests = registry.get_tests_with_tags(&nonexistent_tags);
    assert_eq!(no_tests.len(), 0);
    
    // Test clear
    registry.clear();
    assert!(registry.is_empty());
    assert_eq!(registry.len(), 0);
}

#[test]
fn test_assertion_functions() {
    // Basic assertions
    assert!(assert(true, "should pass").is_pass());
    assert!(assert(false, "should fail").is_fail());
    
    assert!(assert_true(true, "should pass").is_pass());
    assert!(assert_true(false, "should fail").is_fail());
    
    assert!(assert_false(false, "should pass").is_pass());
    assert!(assert_false(true, "should fail").is_fail());
    
    // Equality assertions
    assert!(assert_eq(&42, &42, "should pass").is_pass());
    assert!(assert_eq(&42, &43, "should fail").is_fail());
    
    assert!(assert_ne(&42, &43, "should pass").is_pass());
    assert!(assert_ne(&42, &42, "should fail").is_fail());
    
    // Option assertions
    let some_val = some(42);
    let none_val: OvieOption<i32> = none();
    
    assert!(assert_some(&some_val, "should pass").is_pass());
    assert!(assert_some(&none_val, "should fail").is_fail());
    
    assert!(assert_none(&none_val, "should pass").is_pass());
    assert!(assert_none(&some_val, "should fail").is_fail());
    
    assert!(assert_some_eq(&some_val, &42, "should pass").is_pass());
    assert!(assert_some_eq(&some_val, &43, "should fail").is_fail());
    
    // Result assertions
    let ok_val: OvieResult<i32, String> = ok(42);
    let err_val: OvieResult<i32, String> = err("error".to_string());
    
    assert!(assert_ok(&ok_val, "should pass").is_pass());
    assert!(assert_ok(&err_val, "should fail").is_fail());
    
    assert!(assert_err(&err_val, "should pass").is_pass());
    assert!(assert_err(&ok_val, "should fail").is_fail());
    
    assert!(assert_ok_eq(&ok_val, &42, "should pass").is_pass());
    assert!(assert_ok_eq(&ok_val, &43, "should fail").is_fail());
    
    assert!(assert_err_eq(&err_val, &"error".to_string(), "should pass").is_pass());
    assert!(assert_err_eq(&err_val, &"other".to_string(), "should fail").is_fail());
}

#[test]
fn test_collection_assertions() {
    let mut vec = OvieVec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    
    // Contains assertions
    assert!(assert_contains(&vec, &2, "should pass").is_pass());
    assert!(assert_contains(&vec, &5, "should fail").is_fail());
    
    assert!(assert_not_contains(&vec, &5, "should pass").is_pass());
    assert!(assert_not_contains(&vec, &2, "should fail").is_fail());
    
    // Empty/length assertions
    assert!(assert_not_empty(&vec, "should pass").is_pass());
    assert!(assert_length(&vec, 3, "should pass").is_pass());
    assert!(assert_length(&vec, 5, "should fail").is_fail());
    
    let empty_vec: OvieVec<i32> = OvieVec::new();
    assert!(assert_empty(&empty_vec, "should pass").is_pass());
    assert!(assert_empty(&vec, "should fail").is_fail());
}

#[test]
fn test_string_assertions() {
    let text = "Hello, world!";
    
    // Contains assertion
    assert!(assert_str_contains(text, "world", "should pass").is_pass());
    assert!(assert_str_contains(text, "foo", "should fail").is_fail());
    
    // Starts with assertion
    assert!(assert_str_starts_with(text, "Hello", "should pass").is_pass());
    assert!(assert_str_starts_with(text, "world", "should fail").is_fail());
    
    // Ends with assertion
    assert!(assert_str_ends_with(text, "world!", "should pass").is_pass());
    assert!(assert_str_ends_with(text, "Hello", "should fail").is_fail());
    
    // Pattern matching assertion
    assert!(assert_str_matches(text, "Hello*", "should pass").is_pass());
    assert!(assert_str_matches(text, "*world!", "should pass").is_pass());
    assert!(assert_str_matches(text, "Hello*world!", "should pass").is_pass());
    assert!(assert_str_matches(text, "foo*", "should fail").is_fail());
}

#[test]
fn test_numeric_assertions() {
    // Approximate equality
    assert!(assert_approx_eq(1.0, 1.0001, 0.001, "should pass").is_pass());
    assert!(assert_approx_eq(1.0, 1.1, 0.001, "should fail").is_fail());
    
    // Range assertion
    assert!(assert_in_range(&5, &1, &10, "should pass").is_pass());
    assert!(assert_in_range(&15, &1, &10, "should fail").is_fail());
    assert!(assert_in_range(&0, &1, &10, "should fail").is_fail());
}

#[test]
fn test_result_combination() {
    let mut results = OvieVec::new();
    results.push(TestResult::Pass);
    results.push(TestResult::Pass);
    results.push(TestResult::Pass);
    
    // All pass
    assert!(combine_results(&results).is_pass());
    
    // Add failures
    results.push(TestResult::Fail("error 1".to_string()));
    results.push(TestResult::Fail("error 2".to_string()));
    
    let combined = combine_results(&results);
    assert!(combined.is_fail());
    if let TestResult::Fail(msg) = combined {
        assert!(msg.contains("error 1"));
        assert!(msg.contains("error 2"));
    }
    
    // Skip takes precedence
    let mut skip_results = OvieVec::new();
    skip_results.push(TestResult::Pass);
    skip_results.push(TestResult::Skip("not implemented".to_string()));
    skip_results.push(TestResult::Fail("error".to_string()));
    
    let skip_combined = combine_results(&skip_results);
    assert!(skip_combined.is_skip());
}

#[test]
fn test_helper_functions() {
    // Basic helpers
    assert!(pass().is_pass());
    assert!(fail("test error").is_fail());
    assert!(skip("not ready").is_skip());
    
    // Conditional helpers
    assert!(pass_if(true, "should not fail").is_pass());
    assert!(pass_if(false, "should fail").is_fail());
    
    assert!(fail_if(false, "should not fail").is_pass());
    assert!(fail_if(true, "should fail").is_fail());
}

#[test]
fn test_property_based_testing_generators() {
    // Test integer range generator
    let int_gen = IntRangeGenerator::new(1, 10, 42);
    
    // Test deterministic generation
    let val1 = int_gen.generate(0);
    let val2 = int_gen.generate(0);
    assert_eq!(val1, val2);
    
    // Test range bounds
    for i in 0..100 {
        let val = int_gen.generate(i);
        assert!(val >= 1 && val <= 10);
    }
    
    // Test float range generator
    let float_gen = FloatRangeGenerator::new(0.0, 1.0, 42);
    
    let fval1 = float_gen.generate(0);
    let fval2 = float_gen.generate(0);
    assert_eq!(fval1, fval2);
    
    for i in 0..50 {
        let val = float_gen.generate(i);
        assert!(val >= 0.0 && val <= 1.0);
    }
    
    // Test boolean generator
    let bool_gen = BoolGenerator::new(42);
    
    let bval1 = bool_gen.generate(0);
    let bval2 = bool_gen.generate(0);
    assert_eq!(bval1, bval2);
    
    // Test string generator
    let str_gen = StringGenerator::new(5, 10, 42);
    
    let sval1 = str_gen.generate(0);
    let sval2 = str_gen.generate(0);
    assert_eq!(sval1, sval2);
    
    for i in 0..20 {
        let val = str_gen.generate(i);
        assert!(val.len() >= 5 && val.len() <= 10);
    }
    
    // Test vector generator
    let vec_gen = VecGenerator::new(2, 5, int_gen, 42);
    
    let vval1 = vec_gen.generate(0);
    let vval2 = vec_gen.generate(0);
    assert_eq!(vval1.len(), vval2.len());
    
    for i in 0..20 {
        let val = vec_gen.generate(i);
        assert!(val.len() >= 2 && val.len() <= 5);
        
        // Check element bounds
        for j in 0..val.len() {
            if let some(element) = val.get(j) {
                assert!(element >= 1 && element <= 10);
            }
        }
    }
}

#[test]
fn test_property_testing_execution() {
    // Test a property that should always pass
    let gen = IntRangeGenerator::new(1, 100, 42);
    let result = property("positive integers are positive", &gen, |&x| {
        assert(x > 0, "should be positive")
    });
    assert!(result.is_pass());
    
    // Test a property that should fail
    let result = property("all integers are even", &gen, |&x| {
        assert(x % 2 == 0, "should be even")
    });
    assert!(result.is_fail());
    
    if let TestResult::Fail(message) = result {
        assert!(message.contains("Counterexample"));
        assert!(message.contains("should be even"));
    }
}

#[test]
fn test_convenience_generators() {
    // Test convenience functions
    let int_gen = int_range(1, 10);
    let val = int_gen.generate(0);
    assert!(val >= 1 && val <= 10);
    
    let float_gen = float_range(0.0, 1.0);
    let val = float_gen.generate(0);
    assert!(val >= 0.0 && val <= 1.0);
    
    let bool_gen = bools();
    let _val = bool_gen.generate(0); // Just test it doesn't panic
    
    let string_gen = strings();
    let val = string_gen.generate(0);
    assert!(val.len() <= 20);
    
    let string_gen2 = strings_with_length(5, 5);
    let val = string_gen2.generate(0);
    assert_eq!(val.len(), 5);
    
    let vec_gen = vecs(int_range(1, 10));
    let val = vec_gen.generate(0);
    assert!(val.len() <= 10);
    
    let vec_gen2 = vecs_with_length(3, 3, int_range(1, 10));
    let val = vec_gen2.generate(0);
    assert_eq!(val.len(), 3);
}

#[test]
fn test_test_runner_basic_functionality() {
    let config = TestConfig::new();
    let runner = TestRunner::new(config);
    
    fn passing_test() -> TestResult {
        TestResult::Pass
    }
    
    fn failing_test() -> TestResult {
        TestResult::Fail("This test fails".to_string())
    }
    
    // Test single passing test
    let test_case = TestCase::new("passing_test".to_string());
    let execution = runner.run_single_test(&test_case, passing_test);
    
    assert!(execution.passed());
    assert_eq!(execution.test_case.name, "passing_test");
    
    // Test single failing test
    let test_case = TestCase::new("failing_test".to_string());
    let execution = runner.run_single_test(&test_case, failing_test);
    
    assert!(execution.failed());
    if let TestResult::Fail(message) = &execution.result {
        assert_eq!(message, "This test fails");
    }
}

#[test]
fn test_test_runner_panic_handling() {
    let config = TestConfig::new();
    let runner = TestRunner::new(config);
    
    fn panicking_test() -> TestResult {
        panic!("This test panics");
    }
    
    // Normal test that panics should fail
    let test_case = TestCase::new("panic_test".to_string());
    let execution = runner.run_single_test(&test_case, panicking_test);
    
    assert!(execution.failed());
    if let TestResult::Fail(message) = &execution.result {
        assert!(message.contains("panicked unexpectedly"));
    }
    
    // Test marked as should_panic should pass when it panics
    let should_panic_test = TestCase::new("should_panic_test".to_string()).should_panic();
    let execution = runner.run_single_test(&should_panic_test, panicking_test);
    
    assert!(execution.passed());
}

#[test]
fn test_test_suite_result_operations() {
    let config = TestConfig::new();
    let mut suite_result = TestSuiteResult::new(config);
    
    // Add various test executions
    let test1 = TestCase::new("test1".to_string());
    let execution1 = TestExecution::new(test1, TestResult::Pass, Duration::from_millis(50));
    
    let test2 = TestCase::new("test2".to_string());
    let execution2 = TestExecution::new(test2, TestResult::Fail("error".to_string()), Duration::from_millis(75));
    
    let test3 = TestCase::new("test3".to_string());
    let execution3 = TestExecution::new(test3, TestResult::Skip("not ready".to_string()), Duration::from_millis(25));
    
    suite_result.add_execution(execution1);
    suite_result.add_execution(execution2);
    suite_result.add_execution(execution3);
    
    assert_eq!(suite_result.stats.total, 3);
    assert_eq!(suite_result.stats.passed, 1);
    assert_eq!(suite_result.stats.failed, 1);
    assert_eq!(suite_result.stats.skipped, 1);
    assert!(!suite_result.all_passed());
    
    // Test failed executions
    let failed = suite_result.failed_executions();
    assert_eq!(failed.len(), 1);
    if let some(failed_exec) = failed.get(0) {
        assert_eq!(failed_exec.test_case.name, "test2");
    }
    
    // Test skipped executions
    let skipped = suite_result.skipped_executions();
    assert_eq!(skipped.len(), 1);
    if let some(skipped_exec) = skipped.get(0) {
        assert_eq!(skipped_exec.test_case.name, "test3");
    }
}

#[test]
fn test_report_formatters() {
    let config = TestConfig::new();
    let mut result = TestSuiteResult::new(config);
    
    let test1 = TestCase::new("test1".to_string());
    let execution1 = TestExecution::new(test1, TestResult::Pass, Duration::from_millis(50));
    
    let test2 = TestCase::new("test2".to_string());
    let execution2 = TestExecution::new(test2, TestResult::Fail("error message".to_string()), Duration::from_millis(75));
    
    result.add_execution(execution1);
    result.add_execution(execution2);
    
    // Test plain text format
    let plain_text = TestReportFormatter::format_plain_text(&result);
    assert!(plain_text.contains("Test Results Summary"));
    assert!(plain_text.contains("Total tests: 2"));
    assert!(plain_text.contains("Passed: 1"));
    assert!(plain_text.contains("Failed: 1"));
    assert!(plain_text.contains("Failed Tests:"));
    assert!(plain_text.contains("test2"));
    assert!(plain_text.contains("error message"));
    
    // Test JSON format
    let json = TestReportFormatter::format_json(&result);
    assert!(json.contains("\"summary\""));
    assert!(json.contains("\"total\": 2"));
    assert!(json.contains("\"passed\": 1"));
    assert!(json.contains("\"failed\": 1"));
    assert!(json.contains("\"tests\""));
    assert!(json.contains("\"name\": \"test1\""));
    assert!(json.contains("\"status\": \"pass\""));
    assert!(json.contains("\"name\": \"test2\""));
    assert!(json.contains("\"status\": \"fail\""));
    assert!(json.contains("\"error\": \"error message\""));
    
    // Test JUnit XML format
    let xml = TestReportFormatter::format_junit_xml(&result);
    assert!(xml.contains("<?xml version=\"1.0\""));
    assert!(xml.contains("<testsuite"));
    assert!(xml.contains("tests=\"2\""));
    assert!(xml.contains("failures=\"1\""));
    assert!(xml.contains("<testcase name=\"test1\""));
    assert!(xml.contains("<testcase name=\"test2\""));
    assert!(xml.contains("<failure"));
    assert!(xml.contains("error message"));
}

// Property-based tests for the testing framework itself
#[test]
fn test_framework_properties() {
    // Property: TestResult equality is reflexive
    let results = [
        TestResult::Pass,
        TestResult::Fail("error".to_string()),
        TestResult::Skip("reason".to_string()),
    ];
    
    for result in &results {
        assert_eq!(result, result);
    }
    
    // Property: TestStats accumulation is consistent
    let mut stats = TestStats::new();
    let test_results = [
        (TestResult::Pass, false),
        (TestResult::Fail("e1".to_string()), false),
        (TestResult::Skip("s1".to_string()), false),
        (TestResult::Pass, true), // ignored
    ];
    
    let mut expected_total = 0;
    let mut expected_passed = 0;
    let mut expected_failed = 0;
    let mut expected_skipped = 0;
    let mut expected_ignored = 0;
    
    for (result, ignored) in &test_results {
        stats.add_result(result, *ignored);
        expected_total += 1;
        
        if *ignored {
            expected_ignored += 1;
        } else {
            match result {
                TestResult::Pass => expected_passed += 1,
                TestResult::Fail(_) => expected_failed += 1,
                TestResult::Skip(_) => expected_skipped += 1,
            }
        }
        
        assert_eq!(stats.total, expected_total);
        assert_eq!(stats.passed, expected_passed);
        assert_eq!(stats.failed, expected_failed);
        assert_eq!(stats.skipped, expected_skipped);
        assert_eq!(stats.ignored, expected_ignored);
    }
    
    // Property: Success rate calculation is correct
    if stats.total > stats.ignored {
        let expected_rate = (stats.passed as f64 / (stats.total - stats.ignored) as f64) * 100.0;
        let actual_rate = stats.success_rate();
        assert!((actual_rate - expected_rate).abs() < 0.001);
    }
}

#[test]
fn test_deterministic_property_generation() {
    // Property: Same seed produces same sequence
    let gen1 = IntRangeGenerator::new(1, 100, 42);
    let gen2 = IntRangeGenerator::new(1, 100, 42);
    
    for i in 0..50 {
        assert_eq!(gen1.generate(i), gen2.generate(i));
    }
    
    // Property: Different seeds produce different sequences
    let gen3 = IntRangeGenerator::new(1, 100, 123);
    let mut different_found = false;
    
    for i in 0..50 {
        if gen1.generate(i) != gen3.generate(i) {
            different_found = true;
            break;
        }
    }
    
    assert!(different_found, "Different seeds should produce different sequences");
    
    // Property: Generated values are within bounds
    let gen = IntRangeGenerator::new(10, 20, 42);
    for i in 0..100 {
        let val = gen.generate(i);
        assert!(val >= 10 && val <= 20, "Generated value {} not in range [10, 20]", val);
    }
}

#[test]
fn test_test_case_helper_functions() {
    // Test all helper functions produce correct test cases
    let simple = test_case("simple");
    assert_eq!(simple.name, "simple");
    assert_eq!(simple.description, none());
    assert!(!simple.should_panic);
    assert!(!simple.ignore);
    
    let described = test_case_with_description("described", "A test with description");
    assert_eq!(described.name, "described");
    assert_eq!(described.description, some("A test with description".to_string()));
    
    let panic_test = test_case_should_panic("panic");
    assert_eq!(panic_test.name, "panic");
    assert!(panic_test.should_panic);
    
    let ignored = test_case_ignore("ignored", "Not ready");
    assert_eq!(ignored.name, "ignored");
    assert!(ignored.ignore);
    assert!(ignored.description.is_some());
    
    let timeout_test = test_case_with_timeout("timeout", 1000);
    assert_eq!(timeout_test.name, "timeout");
    assert_eq!(timeout_test.timeout_ms, some(1000));
    
    let tagged = test_case_with_tags("tagged", &["unit", "fast"]);
    assert_eq!(tagged.name, "tagged");
    assert!(tagged.has_tag("unit"));
    assert!(tagged.has_tag("fast"));
    assert!(!tagged.has_tag("slow"));
}