//! Ovie Standard Library - Test Module Runtime Implementation
//! 
//! This module provides the runtime implementation of std::test types and functions
//! that are specified in std/testing/mod.ov. These implementations provide a
//! comprehensive testing framework for Ovie programs.

use crate::stdlib::core::{OvieResult, OvieOption, OvieVec, OvieHashMap, ok, err, some, none};
use std::fmt;
use std::time::{Instant, Duration};

/// Test result indicating success or failure
#[derive(Debug, Clone, PartialEq)]
pub enum TestResult {
    Pass,
    Fail(String),
    Skip(String),
}

impl TestResult {
    /// Check if the test passed
    pub fn is_pass(&self) -> bool {
        matches!(self, TestResult::Pass)
    }
    
    /// Check if the test failed
    pub fn is_fail(&self) -> bool {
        matches!(self, TestResult::Fail(_))
    }
    
    /// Check if the test was skipped
    pub fn is_skip(&self) -> bool {
        matches!(self, TestResult::Skip(_))
    }
    
    /// Get the failure message if the test failed
    pub fn failure_message(&self) -> OvieOption<String> {
        match self {
            TestResult::Fail(msg) => some(msg.clone()),
            _ => none(),
        }
    }
    
    /// Get the skip reason if the test was skipped
    pub fn skip_reason(&self) -> OvieOption<String> {
        match self {
            TestResult::Skip(reason) => some(reason.clone()),
            _ => none(),
        }
    }
}

impl fmt::Display for TestResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestResult::Pass => write!(f, "PASS"),
            TestResult::Fail(msg) => write!(f, "FAIL: {}", msg),
            TestResult::Skip(reason) => write!(f, "SKIP: {}", reason),
        }
    }
}

/// Test case metadata and execution information
#[derive(Debug, Clone)]
pub struct TestCase {
    pub name: String,
    pub description: OvieOption<String>,
    pub timeout_ms: OvieOption<u64>,
    pub should_panic: bool,
    pub ignore: bool,
    pub tags: OvieVec<String>,
}

impl TestCase {
    /// Create a new test case
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: none(),
            timeout_ms: none(),
            should_panic: false,
            ignore: false,
            tags: OvieVec::new(),
        }
    }
    
    /// Set the test description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = some(description);
        self
    }
    
    /// Set the test timeout in milliseconds
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = some(timeout_ms);
        self
    }
    
    /// Mark the test as expecting a panic
    pub fn should_panic(mut self) -> Self {
        self.should_panic = true;
        self
    }
    
    /// Mark the test as ignored
    pub fn ignore(mut self) -> Self {
        self.ignore = true;
        self
    }
    
    /// Add a tag to the test
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }
    
    /// Check if the test has a specific tag
    pub fn has_tag(&self, tag: &str) -> bool {
        for i in 0..self.tags.len() {
            if let OvieOption::Some(test_tag) = self.tags.get(i) {
                if test_tag == tag {
                    return true;
                }
            }
        }
        false
    }
}

/// Test execution statistics
#[derive(Debug, Clone)]
pub struct TestStats {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub ignored: usize,
    pub duration: Duration,
}

impl TestStats {
    /// Create new empty test statistics
    pub fn new() -> Self {
        Self {
            total: 0,
            passed: 0,
            failed: 0,
            skipped: 0,
            ignored: 0,
            duration: Duration::from_secs(0),
        }
    }
    
    /// Add a test result to the statistics
    pub fn add_result(&mut self, result: &TestResult, ignored: bool) {
        self.total += 1;
        
        if ignored {
            self.ignored += 1;
        } else {
            match result {
                TestResult::Pass => self.passed += 1,
                TestResult::Fail(_) => self.failed += 1,
                TestResult::Skip(_) => self.skipped += 1,
            }
        }
    }
    
    /// Check if all tests passed
    pub fn all_passed(&self) -> bool {
        self.failed == 0 && self.total > 0
    }
    
    /// Get success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.passed as f64 / (self.total - self.ignored) as f64) * 100.0
        }
    }
}

impl Default for TestStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Test runner configuration
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub filter: OvieOption<String>,
    pub include_ignored: bool,
    pub fail_fast: bool,
    pub quiet: bool,
    pub verbose: bool,
    pub show_output: bool,
    pub parallel: bool,
    pub test_threads: OvieOption<usize>,
    pub tags: OvieVec<String>,
}

impl TestConfig {
    /// Create default test configuration
    pub fn new() -> Self {
        Self {
            filter: none(),
            include_ignored: false,
            fail_fast: false,
            quiet: false,
            verbose: false,
            show_output: false,
            parallel: false,
            test_threads: none(),
            tags: OvieVec::new(),
        }
    }
    
    /// Set test name filter
    pub fn with_filter(mut self, filter: String) -> Self {
        self.filter = some(filter);
        self
    }
    
    /// Include ignored tests
    pub fn include_ignored(mut self) -> Self {
        self.include_ignored = true;
        self
    }
    
    /// Stop on first failure
    pub fn fail_fast(mut self) -> Self {
        self.fail_fast = true;
        self
    }
    
    /// Run in quiet mode
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }
    
    /// Run in verbose mode
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
    
    /// Show test output
    pub fn show_output(mut self) -> Self {
        self.show_output = true;
        self
    }
    
    /// Run tests in parallel
    pub fn parallel(mut self) -> Self {
        self.parallel = true;
        self
    }
    
    /// Set number of test threads
    pub fn with_test_threads(mut self, threads: usize) -> Self {
        self.test_threads = some(threads);
        self
    }
    
    /// Add a tag filter
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }
    
    /// Check if a test should be run based on configuration
    pub fn should_run_test(&self, test_case: &TestCase) -> bool {
        // Check if test is ignored and we're not including ignored tests
        if test_case.ignore && !self.include_ignored {
            return false;
        }
        
        // Check name filter
        if let OvieOption::Some(filter) = &self.filter {
            if !test_case.name.contains(filter) {
                return false;
            }
        }
        
        // Check tag filters
        if self.tags.len() > 0 {
            let mut has_matching_tag = false;
            for i in 0..self.tags.len() {
                if let OvieOption::Some(tag) = self.tags.get(i) {
                    if test_case.has_tag(&tag) {
                        has_matching_tag = true;
                        break;
                    }
                }
            }
            if !has_matching_tag {
                return false;
            }
        }
        
        true
    }
}

impl Default for TestConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Test execution context
#[derive(Debug)]
pub struct TestContext {
    pub test_case: TestCase,
    pub config: TestConfig,
    pub start_time: Instant,
    pub output: OvieVec<String>,
}

impl TestContext {
    /// Create a new test context
    pub fn new(test_case: TestCase, config: TestConfig) -> Self {
        Self {
            test_case,
            config,
            start_time: Instant::now(),
            output: OvieVec::new(),
        }
    }
    
    /// Add output to the test context
    pub fn add_output(&mut self, output: String) {
        self.output.push(output);
    }
    
    /// Get elapsed time since test started
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    /// Check if test has timed out
    pub fn has_timed_out(&self) -> bool {
        if let OvieOption::Some(timeout_ms) = self.test_case.timeout_ms {
            self.elapsed().as_millis() > timeout_ms as u128
        } else {
            false
        }
    }
}

/// Test function type
pub type TestFunction = fn() -> TestResult;

/// Test registry for storing and managing tests
#[derive(Debug)]
pub struct TestRegistry {
    tests: OvieVec<(TestCase, TestFunction)>,
}

impl TestRegistry {
    /// Create a new test registry
    pub fn new() -> Self {
        Self {
            tests: OvieVec::new(),
        }
    }
    
    /// Register a test function
    pub fn register(&mut self, test_case: TestCase, test_fn: TestFunction) {
        self.tests.push((test_case, test_fn));
    }
    
    /// Get all registered tests
    pub fn get_tests(&self) -> &OvieVec<(TestCase, TestFunction)> {
        &self.tests
    }
    
    /// Get the number of registered tests
    pub fn len(&self) -> usize {
        self.tests.len()
    }
    
    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.tests.is_empty()
    }
    
    /// Clear all registered tests
    pub fn clear(&mut self) {
        self.tests.clear();
    }
    
    /// Find tests matching a filter
    pub fn find_tests(&self, filter: &str) -> OvieVec<(TestCase, TestFunction)> {
        let mut matching_tests = OvieVec::new();
        
        for i in 0..self.tests.len() {
            if let OvieOption::Some((test_case, test_fn)) = self.tests.get(i) {
                if test_case.name.contains(filter) {
                    matching_tests.push((test_case.clone(), test_fn));
                }
            }
        }
        
        matching_tests
    }
    
    /// Get tests with specific tags
    pub fn get_tests_with_tags(&self, tags: &OvieVec<String>) -> OvieVec<(TestCase, TestFunction)> {
        let mut matching_tests = OvieVec::new();
        
        for i in 0..self.tests.len() {
            if let OvieOption::Some(test_tuple) = self.tests.get_ref(i) {
                let (test_case, test_fn) = test_tuple;
                let mut has_matching_tag = false;
                
                for j in 0..tags.len() {
                    if let OvieOption::Some(tag) = tags.get_ref(j) {
                        if test_case.has_tag(tag.as_str()) {
                            has_matching_tag = true;
                            break;
                        }
                    }
                }
                
                if has_matching_tag {
                    matching_tests.push((test_case.clone(), *test_fn));
                }
            }
        }
        
        matching_tests
    }
}

impl Default for TestRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global test registry instance
static mut GLOBAL_TEST_REGISTRY: OvieOption<TestRegistry> = OvieOption::None;

/// Initialize the global test registry
pub fn init_test_registry() {
    unsafe {
        GLOBAL_TEST_REGISTRY = some(TestRegistry::new());
    }
}

/// Get a mutable reference to the global test registry
pub fn get_test_registry() -> &'static mut TestRegistry {
    unsafe {
        match &mut GLOBAL_TEST_REGISTRY {
            OvieOption::Some(registry) => registry,
            OvieOption::None => {
                init_test_registry();
                match &mut GLOBAL_TEST_REGISTRY {
                    OvieOption::Some(registry) => registry,
                    OvieOption::None => panic!("Failed to initialize test registry"),
                }
            }
        }
    }
}

/// Register a test function with the global registry
pub fn register_test(test_case: TestCase, test_fn: TestFunction) {
    let registry = get_test_registry();
    registry.register(test_case, test_fn);
}

/// Create a simple test case with just a name
pub fn test_case(name: &str) -> TestCase {
    TestCase::new(name.to_string())
}

/// Create a test case with description
pub fn test_case_with_description(name: &str, description: &str) -> TestCase {
    TestCase::new(name.to_string()).with_description(description.to_string())
}

/// Create a test case that should panic
pub fn test_case_should_panic(name: &str) -> TestCase {
    TestCase::new(name.to_string()).should_panic()
}

/// Create an ignored test case
pub fn test_case_ignore(name: &str, reason: &str) -> TestCase {
    TestCase::new(name.to_string())
        .ignore()
        .with_description(format!("Ignored: {}", reason))
}

/// Create a test case with timeout
pub fn test_case_with_timeout(name: &str, timeout_ms: u64) -> TestCase {
    TestCase::new(name.to_string()).with_timeout(timeout_ms)
}

/// Create a test case with tags
pub fn test_case_with_tags(name: &str, tags: &[&str]) -> TestCase {
    let mut test_case = TestCase::new(name.to_string());
    for &tag in tags {
        test_case = test_case.with_tag(tag.to_string());
    }
    test_case
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_result_creation() {
        let pass = TestResult::Pass;
        let fail = TestResult::Fail("error message".to_string());
        let skip = TestResult::Skip("not implemented".to_string());
        
        assert!(pass.is_pass());
        assert!(!pass.is_fail());
        assert!(!pass.is_skip());
        
        assert!(!fail.is_pass());
        assert!(fail.is_fail());
        assert!(!fail.is_skip());
        
        assert!(!skip.is_pass());
        assert!(!skip.is_fail());
        assert!(skip.is_skip());
    }
    
    #[test]
    fn test_test_result_messages() {
        let pass = TestResult::Pass;
        let fail = TestResult::Fail("error message".to_string());
        let skip = TestResult::Skip("not implemented".to_string());
        
        assert_eq!(pass.failure_message(), none());
        assert_eq!(fail.failure_message(), some("error message".to_string()));
        assert_eq!(skip.failure_message(), none());
        
        assert_eq!(pass.skip_reason(), none());
        assert_eq!(fail.skip_reason(), none());
        assert_eq!(skip.skip_reason(), some("not implemented".to_string()));
    }
    
    #[test]
    fn test_test_case_creation() {
        let test_case = TestCase::new("test_example".to_string());
        
        assert_eq!(test_case.name, "test_example");
        assert_eq!(test_case.description, none());
        assert_eq!(test_case.timeout_ms, none());
        assert!(!test_case.should_panic);
        assert!(!test_case.ignore);
        assert_eq!(test_case.tags.len(), 0);
    }
    
    #[test]
    fn test_test_case_builder() {
        let test_case = TestCase::new("test_example".to_string())
            .with_description("Test description".to_string())
            .with_timeout(5000)
            .should_panic()
            .ignore()
            .with_tag("unit".to_string())
            .with_tag("fast".to_string());
        
        assert_eq!(test_case.name, "test_example");
        assert_eq!(test_case.description, some("Test description".to_string()));
        assert_eq!(test_case.timeout_ms, some(5000));
        assert!(test_case.should_panic);
        assert!(test_case.ignore);
        assert_eq!(test_case.tags.len(), 2);
        assert!(test_case.has_tag("unit"));
        assert!(test_case.has_tag("fast"));
        assert!(!test_case.has_tag("slow"));
    }
    
    #[test]
    fn test_test_stats() {
        let mut stats = TestStats::new();
        
        assert_eq!(stats.total, 0);
        assert_eq!(stats.passed, 0);
        assert_eq!(stats.failed, 0);
        assert_eq!(stats.skipped, 0);
        assert_eq!(stats.ignored, 0);
        assert!(stats.all_passed() == false); // No tests run
        
        stats.add_result(&TestResult::Pass, false);
        stats.add_result(&TestResult::Fail("error".to_string()), false);
        stats.add_result(&TestResult::Skip("not implemented".to_string()), false);
        stats.add_result(&TestResult::Pass, true); // ignored
        
        assert_eq!(stats.total, 4);
        assert_eq!(stats.passed, 1);
        assert_eq!(stats.failed, 1);
        assert_eq!(stats.skipped, 1);
        assert_eq!(stats.ignored, 1);
        assert!(!stats.all_passed());
        
        // Success rate should be 1/3 = 33.33% (ignoring the ignored test)
        let success_rate = stats.success_rate();
        assert!((success_rate - 33.333333333333336).abs() < 0.001);
    }
    
    #[test]
    fn test_test_config() {
        let config = TestConfig::new()
            .with_filter("unit".to_string())
            .include_ignored()
            .fail_fast()
            .quiet()
            .verbose()
            .show_output()
            .parallel()
            .with_test_threads(4)
            .with_tag("fast".to_string());
        
        assert_eq!(config.filter, some("unit".to_string()));
        assert!(config.include_ignored);
        assert!(config.fail_fast);
        assert!(config.quiet);
        assert!(config.verbose);
        assert!(config.show_output);
        assert!(config.parallel);
        assert_eq!(config.test_threads, some(4));
        assert_eq!(config.tags.len(), 1);
    }
    
    #[test]
    fn test_test_config_should_run_test() {
        let config = TestConfig::new()
            .with_filter("unit".to_string())
            .with_tag("fast".to_string());
        
        let test1 = TestCase::new("unit_test_1".to_string())
            .with_tag("fast".to_string());
        let test2 = TestCase::new("integration_test_1".to_string())
            .with_tag("fast".to_string());
        let test3 = TestCase::new("unit_test_2".to_string())
            .with_tag("slow".to_string());
        let test4 = TestCase::new("unit_test_3".to_string())
            .ignore();
        
        assert!(config.should_run_test(&test1)); // matches filter and tag
        assert!(!config.should_run_test(&test2)); // doesn't match filter
        assert!(!config.should_run_test(&test3)); // doesn't match tag
        assert!(!config.should_run_test(&test4)); // ignored
        
        let config_with_ignored = TestConfig::new()
            .with_filter("unit".to_string())
            .include_ignored();
        
        assert!(config_with_ignored.should_run_test(&test4)); // ignored but included
    }
    
    #[test]
    fn test_test_registry() {
        let mut registry = TestRegistry::new();
        
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
        
        fn dummy_test() -> TestResult {
            TestResult::Pass
        }
        
        let test_case1 = TestCase::new("test1".to_string()).with_tag("unit".to_string());
        let test_case2 = TestCase::new("test2".to_string()).with_tag("integration".to_string());
        
        registry.register(test_case1.clone(), dummy_test);
        registry.register(test_case2.clone(), dummy_test);
        
        assert!(!registry.is_empty());
        assert_eq!(registry.len(), 2);
        
        let found_tests = registry.find_tests("test1");
        assert_eq!(found_tests.len(), 1);
        
        let mut tags = OvieVec::new();
        tags.push("unit".to_string());
        let tagged_tests = registry.get_tests_with_tags(&tags);
        assert_eq!(tagged_tests.len(), 1);
        
        registry.clear();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }
    
    #[test]
    fn test_helper_functions() {
        let test1 = test_case("simple_test");
        assert_eq!(test1.name, "simple_test");
        assert_eq!(test1.description, none());
        
        let test2 = test_case_with_description("described_test", "This is a test");
        assert_eq!(test2.name, "described_test");
        assert_eq!(test2.description, some("This is a test".to_string()));
        
        let test3 = test_case_should_panic("panic_test");
        assert_eq!(test3.name, "panic_test");
        assert!(test3.should_panic);
        
        let test4 = test_case_ignore("ignored_test", "Not implemented yet");
        assert_eq!(test4.name, "ignored_test");
        assert!(test4.ignore);
        
        let test5 = test_case_with_timeout("timeout_test", 1000);
        assert_eq!(test5.name, "timeout_test");
        assert_eq!(test5.timeout_ms, some(1000));
        
        let test6 = test_case_with_tags("tagged_test", &["unit", "fast"]);
        assert_eq!(test6.name, "tagged_test");
        assert!(test6.has_tag("unit"));
        assert!(test6.has_tag("fast"));
        assert!(!test6.has_tag("slow"));
    }
}

/// Assertion functions for testing
/// These provide a comprehensive set of assertion utilities for test cases

/// Assert that a condition is true
pub fn assert(condition: bool, message: &str) -> TestResult {
    if condition {
        TestResult::Pass
    } else {
        TestResult::Fail(format!("Assertion failed: {}", message))
    }
}

/// Assert that two values are equal
pub fn assert_eq<T>(left: &T, right: &T, message: &str) -> TestResult
where
    T: PartialEq + fmt::Debug,
{
    if left == right {
        TestResult::Pass
    } else {
        TestResult::Fail(format!(
            "Assertion failed: {} (left: {:?}, right: {:?})",
            message, left, right
        ))
    }
}

/// Assert that two values are not equal
pub fn assert_ne<T>(left: &T, right: &T, message: &str) -> TestResult
where
    T: PartialEq + fmt::Debug,
{
    if left != right {
        TestResult::Pass
    } else {
        TestResult::Fail(format!(
            "Assertion failed: {} (values are equal: {:?})",
            message, left
        ))
    }
}

/// Assert that a value is true
pub fn assert_true(value: bool, message: &str) -> TestResult {
    if value {
        TestResult::Pass
    } else {
        TestResult::Fail(format!("Expected true but got false: {}", message))
    }
}

/// Assert that a value is false
pub fn assert_false(value: bool, message: &str) -> TestResult {
    if !value {
        TestResult::Pass
    } else {
        TestResult::Fail(format!("Expected false but got true: {}", message))
    }
}

/// Assert that an Option is Some
pub fn assert_some<T>(option: &OvieOption<T>, message: &str) -> TestResult
where
    T: fmt::Debug,
{
    match option {
        OvieOption::Some(_) => TestResult::Pass,
        OvieOption::None => TestResult::Fail(format!("Expected Some but got None: {}", message)),
    }
}

/// Assert that an Option is None
pub fn assert_none<T>(option: &OvieOption<T>, message: &str) -> TestResult
where
    T: fmt::Debug,
{
    match option {
        OvieOption::None => TestResult::Pass,
        OvieOption::Some(value) => TestResult::Fail(format!(
            "Expected None but got Some({:?}): {}",
            value, message
        )),
    }
}

/// Assert that an Option is Some and contains the expected value
pub fn assert_some_eq<T>(option: &OvieOption<T>, expected: &T, message: &str) -> TestResult
where
    T: PartialEq + fmt::Debug,
{
    match option {
        OvieOption::Some(value) => {
            if value == expected {
                TestResult::Pass
            } else {
                TestResult::Fail(format!(
                    "Expected Some({:?}) but got Some({:?}): {}",
                    expected, value, message
                ))
            }
        }
        OvieOption::None => TestResult::Fail(format!(
            "Expected Some({:?}) but got None: {}",
            expected, message
        )),
    }
}

/// Assert that a Result is Ok
pub fn assert_ok<T, E>(result: &OvieResult<T, E>, message: &str) -> TestResult
where
    T: fmt::Debug,
    E: fmt::Debug,
{
    match result {
        OvieResult::Ok(_) => TestResult::Pass,
        OvieResult::Err(error) => TestResult::Fail(format!(
            "Expected Ok but got Err({:?}): {}",
            error, message
        )),
    }
}

/// Assert that a Result is Err
pub fn assert_err<T, E>(result: &OvieResult<T, E>, message: &str) -> TestResult
where
    T: fmt::Debug,
    E: fmt::Debug,
{
    match result {
        OvieResult::Err(_) => TestResult::Pass,
        OvieResult::Ok(value) => TestResult::Fail(format!(
            "Expected Err but got Ok({:?}): {}",
            value, message
        )),
    }
}

/// Assert that a Result is Ok and contains the expected value
pub fn assert_ok_eq<T, E>(result: &OvieResult<T, E>, expected: &T, message: &str) -> TestResult
where
    T: PartialEq + fmt::Debug,
    E: fmt::Debug,
{
    match result {
        OvieResult::Ok(value) => {
            if value == expected {
                TestResult::Pass
            } else {
                TestResult::Fail(format!(
                    "Expected Ok({:?}) but got Ok({:?}): {}",
                    expected, value, message
                ))
            }
        }
        OvieResult::Err(error) => TestResult::Fail(format!(
            "Expected Ok({:?}) but got Err({:?}): {}",
            expected, error, message
        )),
    }
}

/// Assert that a Result is Err and contains the expected error
pub fn assert_err_eq<T, E>(result: &OvieResult<T, E>, expected: &E, message: &str) -> TestResult
where
    T: fmt::Debug,
    E: PartialEq + fmt::Debug,
{
    match result {
        OvieResult::Err(error) => {
            if error == expected {
                TestResult::Pass
            } else {
                TestResult::Fail(format!(
                    "Expected Err({:?}) but got Err({:?}): {}",
                    expected, error, message
                ))
            }
        }
        OvieResult::Ok(value) => TestResult::Fail(format!(
            "Expected Err({:?}) but got Ok({:?}): {}",
            expected, value, message
        )),
    }
}

/// Assert that two floating point numbers are approximately equal
pub fn assert_approx_eq(left: f64, right: f64, epsilon: f64, message: &str) -> TestResult {
    let diff = (left - right).abs();
    if diff <= epsilon {
        TestResult::Pass
    } else {
        TestResult::Fail(format!(
            "Values not approximately equal: {} (left: {}, right: {}, diff: {}, epsilon: {})",
            message, left, right, diff, epsilon
        ))
    }
}

/// Assert that a value is within a range
pub fn assert_in_range<T>(value: &T, min: &T, max: &T, message: &str) -> TestResult
where
    T: PartialOrd + fmt::Debug,
{
    if value >= min && value <= max {
        TestResult::Pass
    } else {
        TestResult::Fail(format!(
            "Value not in range: {} (value: {:?}, range: {:?}..={:?})",
            message, value, min, max
        ))
    }
}

/// Assert that a collection contains a specific element
pub fn assert_contains<T>(collection: &OvieVec<T>, element: &T, message: &str) -> TestResult
where
    T: PartialEq + fmt::Debug + Clone,
{
    for i in 0..collection.len() {
        if let OvieOption::Some(item) = collection.get(i) {
            if item == *element {
                return TestResult::Pass;
            }
        }
    }
    TestResult::Fail(format!(
        "Collection does not contain element: {} (element: {:?})",
        message, element
    ))
}

/// Assert that a collection does not contain a specific element
pub fn assert_not_contains<T>(collection: &OvieVec<T>, element: &T, message: &str) -> TestResult
where
    T: PartialEq + fmt::Debug + Clone,
{
    for i in 0..collection.len() {
        if let OvieOption::Some(item) = collection.get(i) {
            if item == *element {
                return TestResult::Fail(format!(
                    "Collection contains element: {} (element: {:?})",
                    message, element
                ));
            }
        }
    }
    TestResult::Pass
}

/// Assert that a collection is empty
pub fn assert_empty<T>(collection: &OvieVec<T>, message: &str) -> TestResult {
    if collection.is_empty() {
        TestResult::Pass
    } else {
        TestResult::Fail(format!(
            "Collection is not empty: {} (length: {})",
            message,
            collection.len()
        ))
    }
}

/// Assert that a collection is not empty
pub fn assert_not_empty<T>(collection: &OvieVec<T>, message: &str) -> TestResult {
    if !collection.is_empty() {
        TestResult::Pass
    } else {
        TestResult::Fail(format!("Collection is empty: {}", message))
    }
}

/// Assert that a collection has a specific length
pub fn assert_length<T>(collection: &OvieVec<T>, expected_length: usize, message: &str) -> TestResult {
    let actual_length = collection.len();
    if actual_length == expected_length {
        TestResult::Pass
    } else {
        TestResult::Fail(format!(
            "Collection length mismatch: {} (expected: {}, actual: {})",
            message, expected_length, actual_length
        ))
    }
}

/// Assert that a string contains a substring
pub fn assert_str_contains(haystack: &str, needle: &str, message: &str) -> TestResult {
    if haystack.contains(needle) {
        TestResult::Pass
    } else {
        TestResult::Fail(format!(
            "String does not contain substring: {} (haystack: '{}', needle: '{}')",
            message, haystack, needle
        ))
    }
}

/// Assert that a string starts with a prefix
pub fn assert_str_starts_with(string: &str, prefix: &str, message: &str) -> TestResult {
    if string.starts_with(prefix) {
        TestResult::Pass
    } else {
        TestResult::Fail(format!(
            "String does not start with prefix: {} (string: '{}', prefix: '{}')",
            message, string, prefix
        ))
    }
}

/// Assert that a string ends with a suffix
pub fn assert_str_ends_with(string: &str, suffix: &str, message: &str) -> TestResult {
    if string.ends_with(suffix) {
        TestResult::Pass
    } else {
        TestResult::Fail(format!(
            "String does not end with suffix: {} (string: '{}', suffix: '{}')",
            message, string, suffix
        ))
    }
}

/// Assert that a string matches a pattern (simple wildcard matching)
pub fn assert_str_matches(string: &str, pattern: &str, message: &str) -> TestResult {
    if simple_pattern_match(string, pattern) {
        TestResult::Pass
    } else {
        TestResult::Fail(format!(
            "String does not match pattern: {} (string: '{}', pattern: '{}')",
            message, string, pattern
        ))
    }
}

/// Simple wildcard pattern matching (* matches any sequence of characters)
fn simple_pattern_match(string: &str, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    
    if !pattern.contains('*') {
        return string == pattern;
    }
    
    let parts: Vec<&str> = pattern.split('*').collect();
    if parts.is_empty() {
        return true;
    }
    
    let mut pos = 0;
    for (i, part) in parts.iter().enumerate() {
        if part.is_empty() {
            continue;
        }
        
        if i == 0 {
            // First part must match the beginning
            if !string[pos..].starts_with(part) {
                return false;
            }
            pos += part.len();
        } else if i == parts.len() - 1 {
            // Last part must match the end
            return string[pos..].ends_with(part);
        } else {
            // Middle parts must be found somewhere
            if let Some(found_pos) = string[pos..].find(part) {
                pos += found_pos + part.len();
            } else {
                return false;
            }
        }
    }
    
    true
}

/// Combine multiple test results into one
pub fn combine_results(results: &OvieVec<TestResult>) -> TestResult {
    let mut failures = OvieVec::new();
    
    for i in 0..results.len() {
        if let OvieOption::Some(result) = results.get(i) {
            match result {
                TestResult::Fail(msg) => failures.push(msg),
                TestResult::Skip(reason) => {
                    return TestResult::Skip(reason);
                }
                TestResult::Pass => {}
            }
        }
    }
    
    if failures.is_empty() {
        TestResult::Pass
    } else {
        let mut combined_message = String::new();
        for i in 0..failures.len() {
            if let OvieOption::Some(msg) = failures.get(i) {
                if i > 0 {
                    combined_message.push_str("; ");
                }
                combined_message.push_str(&msg);
            }
        }
        TestResult::Fail(combined_message)
    }
}

/// Create a test result that always passes
pub fn pass() -> TestResult {
    TestResult::Pass
}

/// Create a test result that always fails with a message
pub fn fail(message: &str) -> TestResult {
    TestResult::Fail(message.to_string())
}

/// Create a test result that skips with a reason
pub fn skip(reason: &str) -> TestResult {
    TestResult::Skip(reason.to_string())
}

/// Conditional test result - pass if condition is true, fail otherwise
pub fn pass_if(condition: bool, failure_message: &str) -> TestResult {
    if condition {
        TestResult::Pass
    } else {
        TestResult::Fail(failure_message.to_string())
    }
}

/// Conditional test result - fail if condition is true, pass otherwise
pub fn fail_if(condition: bool, failure_message: &str) -> TestResult {
    if condition {
        TestResult::Fail(failure_message.to_string())
    } else {
        TestResult::Pass
    }
}

#[cfg(test)]
mod assertion_tests {
    use super::*;

    #[test]
    fn test_basic_assertions() {
        assert!(assert(true, "should pass").is_pass());
        assert!(assert(false, "should fail").is_fail());
        
        assert!(assert_true(true, "should pass").is_pass());
        assert!(assert_true(false, "should fail").is_fail());
        
        assert!(assert_false(false, "should pass").is_pass());
        assert!(assert_false(true, "should fail").is_fail());
    }
    
    #[test]
    fn test_equality_assertions() {
        assert!(assert_eq(&42, &42, "should pass").is_pass());
        assert!(assert_eq(&42, &43, "should fail").is_fail());
        
        assert!(assert_ne(&42, &43, "should pass").is_pass());
        assert!(assert_ne(&42, &42, "should fail").is_fail());
    }
    
    #[test]
    fn test_option_assertions() {
        let some_value = some(42);
        let none_value: OvieOption<i32> = none();
        
        assert!(assert_some(&some_value, "should pass").is_pass());
        assert!(assert_some(&none_value, "should fail").is_fail());
        
        assert!(assert_none(&none_value, "should pass").is_pass());
        assert!(assert_none(&some_value, "should fail").is_fail());
        
        assert!(assert_some_eq(&some_value, &42, "should pass").is_pass());
        assert!(assert_some_eq(&some_value, &43, "should fail").is_fail());
        assert!(assert_some_eq(&none_value, &42, "should fail").is_fail());
    }
    
    #[test]
    fn test_result_assertions() {
        let ok_value: OvieResult<i32, String> = ok(42);
        let err_value: OvieResult<i32, String> = err("error".to_string());
        
        assert!(assert_ok(&ok_value, "should pass").is_pass());
        assert!(assert_ok(&err_value, "should fail").is_fail());
        
        assert!(assert_err(&err_value, "should pass").is_pass());
        assert!(assert_err(&ok_value, "should fail").is_fail());
        
        assert!(assert_ok_eq(&ok_value, &42, "should pass").is_pass());
        assert!(assert_ok_eq(&ok_value, &43, "should fail").is_fail());
        assert!(assert_ok_eq(&err_value, &42, "should fail").is_fail());
        
        assert!(assert_err_eq(&err_value, &"error".to_string(), "should pass").is_pass());
        assert!(assert_err_eq(&err_value, &"other".to_string(), "should fail").is_fail());
        assert!(assert_err_eq(&ok_value, &"error".to_string(), "should fail").is_fail());
    }
    
    #[test]
    fn test_approximate_equality() {
        assert!(assert_approx_eq(1.0, 1.0001, 0.001, "should pass").is_pass());
        assert!(assert_approx_eq(1.0, 1.1, 0.001, "should fail").is_fail());
    }
    
    #[test]
    fn test_range_assertions() {
        assert!(assert_in_range(&5, &1, &10, "should pass").is_pass());
        assert!(assert_in_range(&15, &1, &10, "should fail").is_fail());
        assert!(assert_in_range(&0, &1, &10, "should fail").is_fail());
    }
    
    #[test]
    fn test_collection_assertions() {
        let mut vec = OvieVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        
        assert!(assert_contains(&vec, &2, "should pass").is_pass());
        assert!(assert_contains(&vec, &5, "should fail").is_fail());
        
        assert!(assert_not_contains(&vec, &5, "should pass").is_pass());
        assert!(assert_not_contains(&vec, &2, "should fail").is_fail());
        
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
        
        assert!(assert_str_contains(text, "world", "should pass").is_pass());
        assert!(assert_str_contains(text, "foo", "should fail").is_fail());
        
        assert!(assert_str_starts_with(text, "Hello", "should pass").is_pass());
        assert!(assert_str_starts_with(text, "world", "should fail").is_fail());
        
        assert!(assert_str_ends_with(text, "world!", "should pass").is_pass());
        assert!(assert_str_ends_with(text, "Hello", "should fail").is_fail());
        
        assert!(assert_str_matches(text, "Hello*", "should pass").is_pass());
        assert!(assert_str_matches(text, "*world!", "should pass").is_pass());
        assert!(assert_str_matches(text, "Hello*world!", "should pass").is_pass());
        assert!(assert_str_matches(text, "foo*", "should fail").is_fail());
    }
    
    #[test]
    fn test_pattern_matching() {
        assert!(simple_pattern_match("hello", "hello"));
        assert!(simple_pattern_match("hello", "*"));
        assert!(simple_pattern_match("hello", "h*"));
        assert!(simple_pattern_match("hello", "*o"));
        assert!(simple_pattern_match("hello", "h*o"));
        assert!(simple_pattern_match("hello world", "hello*world"));
        assert!(simple_pattern_match("hello beautiful world", "hello*world"));
        
        assert!(!simple_pattern_match("hello", "world"));
        assert!(!simple_pattern_match("hello", "h*x"));
        assert!(!simple_pattern_match("hello", "x*o"));
    }
    
    #[test]
    fn test_combine_results() {
        let mut results = OvieVec::new();
        results.push(TestResult::Pass);
        results.push(TestResult::Pass);
        results.push(TestResult::Pass);
        
        assert!(combine_results(&results).is_pass());
        
        results.push(TestResult::Fail("error 1".to_string()));
        results.push(TestResult::Fail("error 2".to_string()));
        
        let combined = combine_results(&results);
        assert!(combined.is_fail());
        if let TestResult::Fail(msg) = combined {
            assert!(msg.contains("error 1"));
            assert!(msg.contains("error 2"));
        }
        
        let mut skip_results = OvieVec::new();
        skip_results.push(TestResult::Pass);
        skip_results.push(TestResult::Skip("not implemented".to_string()));
        
        let skip_combined = combine_results(&skip_results);
        assert!(skip_combined.is_skip());
    }
    
    #[test]
    fn test_helper_functions() {
        assert!(pass().is_pass());
        assert!(fail("test error").is_fail());
        assert!(skip("not ready").is_skip());
        
        assert!(pass_if(true, "should not fail").is_pass());
        assert!(pass_if(false, "should fail").is_fail());
        
        assert!(fail_if(false, "should not fail").is_pass());
        assert!(fail_if(true, "should fail").is_fail());
    }
}
/// Property-based testing support
/// This provides a simple property-based testing framework using deterministic generators

/// Trait for generating test data
pub trait Generator<T> {
    /// Generate a value for the given test case number
    fn generate(&self, case: usize) -> T;
    
    /// Get the number of test cases to generate
    fn case_count(&self) -> usize {
        100 // Default to 100 test cases
    }
}

/// Property test configuration
#[derive(Debug, Clone)]
pub struct PropertyConfig {
    pub max_cases: usize,
    pub max_shrink_attempts: usize,
    pub seed: u64,
}

impl PropertyConfig {
    /// Create default property configuration
    pub fn new() -> Self {
        Self {
            max_cases: 100,
            max_shrink_attempts: 10,
            seed: 42, // Deterministic seed for reproducible tests
        }
    }
    
    /// Set maximum number of test cases
    pub fn with_max_cases(mut self, max_cases: usize) -> Self {
        self.max_cases = max_cases;
        self
    }
    
    /// Set maximum shrinking attempts
    pub fn with_max_shrink_attempts(mut self, max_shrink_attempts: usize) -> Self {
        self.max_shrink_attempts = max_shrink_attempts;
        self
    }
    
    /// Set deterministic seed
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }
}

impl Default for PropertyConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Property test result with counterexample information
#[derive(Debug, Clone)]
pub struct PropertyResult<T> {
    pub result: TestResult,
    pub cases_tested: usize,
    pub counterexample: OvieOption<T>,
    pub shrunk_counterexample: OvieOption<T>,
}

impl<T> PropertyResult<T> {
    /// Create a passing property result
    pub fn pass(cases_tested: usize) -> Self {
        Self {
            result: TestResult::Pass,
            cases_tested,
            counterexample: none(),
            shrunk_counterexample: none(),
        }
    }
    
    /// Create a failing property result
    pub fn fail(cases_tested: usize, counterexample: T, message: String) -> Self {
        Self {
            result: TestResult::Fail(message),
            cases_tested,
            counterexample: some(counterexample),
            shrunk_counterexample: none(),
        }
    }
    
    /// Add shrunk counterexample
    pub fn with_shrunk_counterexample(mut self, shrunk: T) -> Self {
        self.shrunk_counterexample = some(shrunk);
        self
    }
}

/// Run a property-based test
pub fn check_property<T, F>(
    generator: &dyn Generator<T>,
    property: F,
    config: PropertyConfig,
) -> PropertyResult<T>
where
    T: Clone + fmt::Debug,
    F: Fn(&T) -> TestResult,
{
    let max_cases = std::cmp::min(config.max_cases, generator.case_count());
    
    for case in 0..max_cases {
        let test_value = generator.generate(case);
        let result = property(&test_value);
        
        match result {
            TestResult::Pass => continue,
            TestResult::Fail(message) => {
                return PropertyResult::fail(case + 1, test_value, message);
            }
            TestResult::Skip(reason) => {
                return PropertyResult {
                    result: TestResult::Skip(reason),
                    cases_tested: case + 1,
                    counterexample: none(),
                    shrunk_counterexample: none(),
                };
            }
        }
    }
    
    PropertyResult::pass(max_cases)
}

/// Simple deterministic random number generator for reproducible tests
#[derive(Debug, Clone)]
pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    /// Create a new RNG with the given seed
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    
    /// Generate the next random number
    pub fn next(&mut self) -> u64 {
        // Simple linear congruential generator
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        self.state
    }
    
    /// Generate a random number in the range [0, max)
    pub fn next_range(&mut self, max: u64) -> u64 {
        if max == 0 {
            0
        } else {
            self.next() % max
        }
    }
    
    /// Generate a random boolean
    pub fn next_bool(&mut self) -> bool {
        self.next() % 2 == 0
    }
    
    /// Generate a random f64 in the range [0.0, 1.0)
    pub fn next_f64(&mut self) -> f64 {
        (self.next() as f64) / (u64::MAX as f64)
    }
}

/// Generator for integers in a range
#[derive(Debug, Clone)]
pub struct IntRangeGenerator {
    min: i64,
    max: i64,
    rng: SimpleRng,
}

impl IntRangeGenerator {
    /// Create a new integer range generator
    pub fn new(min: i64, max: i64, seed: u64) -> Self {
        Self {
            min,
            max,
            rng: SimpleRng::new(seed),
        }
    }
}

impl Generator<i64> for IntRangeGenerator {
    fn generate(&self, case: usize) -> i64 {
        let mut rng = self.rng.clone();
        // Advance RNG state based on case number for deterministic generation
        for _ in 0..case {
            rng.next();
        }
        
        if self.min == self.max {
            self.min
        } else {
            let range = (self.max - self.min) as u64;
            self.min + (rng.next_range(range + 1) as i64)
        }
    }
}

/// Generator for floating point numbers in a range
#[derive(Debug, Clone)]
pub struct FloatRangeGenerator {
    min: f64,
    max: f64,
    rng: SimpleRng,
}

impl FloatRangeGenerator {
    /// Create a new float range generator
    pub fn new(min: f64, max: f64, seed: u64) -> Self {
        Self {
            min,
            max,
            rng: SimpleRng::new(seed),
        }
    }
}

impl Generator<f64> for FloatRangeGenerator {
    fn generate(&self, case: usize) -> f64 {
        let mut rng = self.rng.clone();
        // Advance RNG state based on case number for deterministic generation
        for _ in 0..case {
            rng.next();
        }
        
        let t = rng.next_f64();
        self.min + t * (self.max - self.min)
    }
}

/// Generator for boolean values
#[derive(Debug, Clone)]
pub struct BoolGenerator {
    rng: SimpleRng,
}

impl BoolGenerator {
    /// Create a new boolean generator
    pub fn new(seed: u64) -> Self {
        Self {
            rng: SimpleRng::new(seed),
        }
    }
}

impl Generator<bool> for BoolGenerator {
    fn generate(&self, case: usize) -> bool {
        let mut rng = self.rng.clone();
        // Advance RNG state based on case number for deterministic generation
        for _ in 0..case {
            rng.next();
        }
        
        rng.next_bool()
    }
}

/// Generator for strings of varying lengths
#[derive(Debug, Clone)]
pub struct StringGenerator {
    min_length: usize,
    max_length: usize,
    charset: String,
    rng: SimpleRng,
}

impl StringGenerator {
    /// Create a new string generator with ASCII letters
    pub fn new(min_length: usize, max_length: usize, seed: u64) -> Self {
        Self {
            min_length,
            max_length,
            charset: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string(),
            rng: SimpleRng::new(seed),
        }
    }
    
    /// Create a string generator with custom character set
    pub fn with_charset(min_length: usize, max_length: usize, charset: String, seed: u64) -> Self {
        Self {
            min_length,
            max_length,
            charset,
            rng: SimpleRng::new(seed),
        }
    }
}

impl Generator<String> for StringGenerator {
    fn generate(&self, case: usize) -> String {
        let mut rng = self.rng.clone();
        // Advance RNG state based on case number for deterministic generation
        for _ in 0..case {
            rng.next();
        }
        
        let length = if self.min_length == self.max_length {
            self.min_length
        } else {
            let range = self.max_length - self.min_length;
            self.min_length + (rng.next_range(range as u64 + 1) as usize)
        };
        
        let charset_chars: Vec<char> = self.charset.chars().collect();
        let mut result = String::new();
        
        for _ in 0..length {
            let char_index = rng.next_range(charset_chars.len() as u64) as usize;
            result.push(charset_chars[char_index]);
        }
        
        result
    }
}

/// Generator for vectors of values
#[derive(Debug, Clone)]
pub struct VecGenerator<T, G> {
    min_length: usize,
    max_length: usize,
    element_generator: G,
    rng: SimpleRng,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, G> VecGenerator<T, G>
where
    G: Generator<T>,
{
    /// Create a new vector generator
    pub fn new(min_length: usize, max_length: usize, element_generator: G, seed: u64) -> Self {
        Self {
            min_length,
            max_length,
            element_generator,
            rng: SimpleRng::new(seed),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, G> Generator<OvieVec<T>> for VecGenerator<T, G>
where
    T: Clone,
    G: Generator<T>,
{
    fn generate(&self, case: usize) -> OvieVec<T> {
        let mut rng = self.rng.clone();
        // Advance RNG state based on case number for deterministic generation
        for _ in 0..case {
            rng.next();
        }
        
        let length = if self.min_length == self.max_length {
            self.min_length
        } else {
            let range = self.max_length - self.min_length;
            self.min_length + (rng.next_range(range as u64 + 1) as usize)
        };
        
        let mut result = OvieVec::new();
        for i in 0..length {
            let element = self.element_generator.generate(case * 1000 + i);
            result.push(element);
        }
        
        result
    }
}

/// Convenience functions for creating common generators

/// Create an integer generator for the range [min, max]
pub fn int_range(min: i64, max: i64) -> IntRangeGenerator {
    IntRangeGenerator::new(min, max, 42)
}

/// Create a float generator for the range [min, max]
pub fn float_range(min: f64, max: f64) -> FloatRangeGenerator {
    FloatRangeGenerator::new(min, max, 42)
}

/// Create a boolean generator
pub fn bools() -> BoolGenerator {
    BoolGenerator::new(42)
}

/// Create a string generator with default settings
pub fn strings() -> StringGenerator {
    StringGenerator::new(0, 20, 42)
}

/// Create a string generator with specific length range
pub fn strings_with_length(min_length: usize, max_length: usize) -> StringGenerator {
    StringGenerator::new(min_length, max_length, 42)
}

/// Create a vector generator
pub fn vecs<T, G>(element_generator: G) -> VecGenerator<T, G>
where
    G: Generator<T>,
{
    VecGenerator::new(0, 10, element_generator, 42)
}

/// Create a vector generator with specific length range
pub fn vecs_with_length<T, G>(min_length: usize, max_length: usize, element_generator: G) -> VecGenerator<T, G>
where
    G: Generator<T>,
{
    VecGenerator::new(min_length, max_length, element_generator, 42)
}

/// Property test macro-like function
pub fn property<T, F>(
    name: &str,
    generator: &dyn Generator<T>,
    property_fn: F,
) -> TestResult
where
    T: Clone + fmt::Debug,
    F: Fn(&T) -> TestResult,
{
    let config = PropertyConfig::new();
    let result = check_property(generator, property_fn, config);
    
    match result.result {
        TestResult::Pass => TestResult::Pass,
        TestResult::Fail(message) => {
            let detailed_message = if let OvieOption::Some(counterexample) = result.counterexample {
                format!(
                    "Property '{}' failed after {} cases. Counterexample: {:?}. Error: {}",
                    name, result.cases_tested, counterexample, message
                )
            } else {
                format!(
                    "Property '{}' failed after {} cases. Error: {}",
                    name, result.cases_tested, message
                )
            };
            TestResult::Fail(detailed_message)
        }
        TestResult::Skip(reason) => TestResult::Skip(reason),
    }
}

/// Property test with custom configuration
pub fn property_with_config<T, F>(
    name: &str,
    generator: &dyn Generator<T>,
    property_fn: F,
    config: PropertyConfig,
) -> TestResult
where
    T: Clone + fmt::Debug,
    F: Fn(&T) -> TestResult,
{
    let result = check_property(generator, property_fn, config);
    
    match result.result {
        TestResult::Pass => TestResult::Pass,
        TestResult::Fail(message) => {
            let detailed_message = if let OvieOption::Some(counterexample) = result.counterexample {
                format!(
                    "Property '{}' failed after {} cases. Counterexample: {:?}. Error: {}",
                    name, result.cases_tested, counterexample, message
                )
            } else {
                format!(
                    "Property '{}' failed after {} cases. Error: {}",
                    name, result.cases_tested, message
                )
            };
            TestResult::Fail(detailed_message)
        }
        TestResult::Skip(reason) => TestResult::Skip(reason),
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;

    #[test]
    fn test_simple_rng_deterministic() {
        let mut rng1 = SimpleRng::new(42);
        let mut rng2 = SimpleRng::new(42);
        
        // Same seed should produce same sequence
        for _ in 0..10 {
            assert_eq!(rng1.next(), rng2.next());
        }
        
        // Different seeds should produce different sequences
        let mut rng3 = SimpleRng::new(123);
        assert_ne!(rng1.next(), rng3.next());
    }
    
    #[test]
    fn test_int_range_generator() {
        let gen = IntRangeGenerator::new(1, 10, 42);
        
        // Test deterministic generation
        let val1 = gen.generate(0);
        let val2 = gen.generate(0);
        assert_eq!(val1, val2);
        
        // Test range bounds
        for i in 0..100 {
            let val = gen.generate(i);
            assert!(val >= 1 && val <= 10);
        }
        
        // Test single value range
        let single_gen = IntRangeGenerator::new(5, 5, 42);
        assert_eq!(single_gen.generate(0), 5);
        assert_eq!(single_gen.generate(99), 5);
    }
    
    #[test]
    fn test_float_range_generator() {
        let gen = FloatRangeGenerator::new(0.0, 1.0, 42);
        
        // Test deterministic generation
        let val1 = gen.generate(0);
        let val2 = gen.generate(0);
        assert_eq!(val1, val2);
        
        // Test range bounds
        for i in 0..100 {
            let val = gen.generate(i);
            assert!(val >= 0.0 && val <= 1.0);
        }
    }
    
    #[test]
    fn test_bool_generator() {
        let gen = BoolGenerator::new(42);
        
        // Test deterministic generation
        let val1 = gen.generate(0);
        let val2 = gen.generate(0);
        assert_eq!(val1, val2);
        
        // Test that we get both true and false values
        let mut has_true = false;
        let mut has_false = false;
        
        for i in 0..100 {
            let val = gen.generate(i);
            if val {
                has_true = true;
            } else {
                has_false = true;
            }
        }
        
        assert!(has_true && has_false);
    }
    
    #[test]
    fn test_string_generator() {
        let gen = StringGenerator::new(5, 10, 42);
        
        // Test deterministic generation
        let val1 = gen.generate(0);
        let val2 = gen.generate(0);
        assert_eq!(val1, val2);
        
        // Test length bounds
        for i in 0..50 {
            let val = gen.generate(i);
            assert!(val.len() >= 5 && val.len() <= 10);
        }
        
        // Test custom charset
        let custom_gen = StringGenerator::with_charset(3, 3, "abc".to_string(), 42);
        let custom_val = custom_gen.generate(0);
        assert_eq!(custom_val.len(), 3);
        for ch in custom_val.chars() {
            assert!(ch == 'a' || ch == 'b' || ch == 'c');
        }
    }
    
    #[test]
    fn test_vec_generator() {
        let int_gen = IntRangeGenerator::new(1, 100, 42);
        let vec_gen = VecGenerator::new(2, 5, int_gen, 42);
        
        // Test deterministic generation
        let val1 = vec_gen.generate(0);
        let val2 = vec_gen.generate(0);
        assert_eq!(val1.len(), val2.len());
        for i in 0..val1.len() {
            assert_eq!(val1.get(i), val2.get(i));
        }
        
        // Test length bounds
        for i in 0..50 {
            let val = vec_gen.generate(i);
            assert!(val.len() >= 2 && val.len() <= 5);
            
            // Test element bounds
            for j in 0..val.len() {
                if let OvieOption::Some(element) = val.get(j) {
                    assert!(element >= 1 && element <= 100);
                }
            }
        }
    }
    
    #[test]
    fn test_property_testing() {
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
    fn test_property_with_config() {
        let gen = IntRangeGenerator::new(1, 10, 42);
        let config = PropertyConfig::new().with_max_cases(50);
        
        let result = property_with_config("small test", &gen, |&x| {
            assert(x <= 10, "should be <= 10")
        }, config);
        
        assert!(result.is_pass());
    }
    
    #[test]
    fn test_convenience_generators() {
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
}
/// Test runner and reporting functionality
/// This provides comprehensive test execution and result reporting

/// Test execution result for a single test
#[derive(Debug, Clone)]
pub struct TestExecution {
    pub test_case: TestCase,
    pub result: TestResult,
    pub duration: Duration,
    pub output: OvieVec<String>,
}

impl TestExecution {
    /// Create a new test execution result
    pub fn new(test_case: TestCase, result: TestResult, duration: Duration) -> Self {
        Self {
            test_case,
            result,
            duration,
            output: OvieVec::new(),
        }
    }
    
    /// Add output to the execution result
    pub fn with_output(mut self, output: OvieVec<String>) -> Self {
        self.output = output;
        self
    }
    
    /// Check if the test passed
    pub fn passed(&self) -> bool {
        self.result.is_pass()
    }
    
    /// Check if the test failed
    pub fn failed(&self) -> bool {
        self.result.is_fail()
    }
    
    /// Check if the test was skipped
    pub fn skipped(&self) -> bool {
        self.result.is_skip()
    }
}

/// Test suite execution results
#[derive(Debug, Clone)]
pub struct TestSuiteResult {
    pub executions: OvieVec<TestExecution>,
    pub stats: TestStats,
    pub total_duration: Duration,
    pub config: TestConfig,
}

impl TestSuiteResult {
    /// Create a new test suite result
    pub fn new(config: TestConfig) -> Self {
        Self {
            executions: OvieVec::new(),
            stats: TestStats::new(),
            total_duration: Duration::from_secs(0),
            config,
        }
    }
    
    /// Add a test execution result
    pub fn add_execution(&mut self, execution: TestExecution) {
        let ignored = execution.test_case.ignore && !self.config.include_ignored;
        self.stats.add_result(&execution.result, ignored);
        self.total_duration += execution.duration;
        self.executions.push(execution);
    }
    
    /// Get failed test executions
    pub fn failed_executions(&self) -> OvieVec<TestExecution> {
        let mut failed = OvieVec::new();
        for i in 0..self.executions.len() {
            if let OvieOption::Some(execution) = self.executions.get(i) {
                if execution.failed() {
                    failed.push(execution);
                }
            }
        }
        failed
    }
    
    /// Get skipped test executions
    pub fn skipped_executions(&self) -> OvieVec<TestExecution> {
        let mut skipped = OvieVec::new();
        for i in 0..self.executions.len() {
            if let OvieOption::Some(execution) = self.executions.get(i) {
                if execution.skipped() {
                    skipped.push(execution);
                }
            }
        }
        skipped
    }
    
    /// Check if all tests passed
    pub fn all_passed(&self) -> bool {
        self.stats.all_passed()
    }
}

/// Test runner for executing test suites
#[derive(Debug)]
pub struct TestRunner {
    config: TestConfig,
}

impl TestRunner {
    /// Create a new test runner with configuration
    pub fn new(config: TestConfig) -> Self {
        Self { config }
    }
    
    /// Create a test runner with default configuration
    pub fn default() -> Self {
        Self {
            config: TestConfig::new(),
        }
    }
    
    /// Run all tests in the global registry
    pub fn run_all_tests(&self) -> TestSuiteResult {
        let registry = get_test_registry();
        let tests = registry.get_tests();
        self.run_tests(tests)
    }
    
    /// Run a specific set of tests
    pub fn run_tests(&self, tests: &OvieVec<(TestCase, TestFunction)>) -> TestSuiteResult {
        let start_time = Instant::now();
        let mut result = TestSuiteResult::new(self.config.clone());
        
        if !self.config.quiet {
            self.print_test_start(tests.len());
        }
        
        for i in 0..tests.len() {
            if let OvieOption::Some((test_case, test_fn)) = tests.get(i) {
                if !self.config.should_run_test(&test_case) {
                    continue;
                }
                
                let execution = self.run_single_test(&test_case, test_fn);
                
                if !self.config.quiet {
                    self.print_test_result(&execution);
                }
                
                let should_stop = execution.failed() && self.config.fail_fast;
                result.add_execution(execution);
                
                if should_stop {
                    break;
                }
            }
        }
        
        result.total_duration = start_time.elapsed();
        
        if !self.config.quiet {
            self.print_test_summary(&result);
        }
        
        result
    }
    
    /// Run a single test
    fn run_single_test(&self, test_case: &TestCase, test_fn: TestFunction) -> TestExecution {
        let start_time = Instant::now();
        let mut context = TestContext::new(test_case.clone(), self.config.clone());
        
        // Check if test should be skipped due to ignore flag
        if test_case.ignore && !self.config.include_ignored {
            let duration = start_time.elapsed();
            let skip_reason = test_case.description.clone()
                .unwrap_or_else(|| "Test ignored".to_string());
            return TestExecution::new(
                test_case.clone(),
                TestResult::Skip(skip_reason),
                duration,
            );
        }
        
        // Execute the test function
        let result = if test_case.should_panic {
            // For should_panic tests, we expect a panic
            let panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                test_fn()
            }));
            
            match panic_result {
                Ok(test_result) => {
                    // Test didn't panic but should have
                    match test_result {
                        TestResult::Pass => TestResult::Fail("Test was expected to panic but didn't".to_string()),
                        other => other, // Preserve other results (Fail, Skip)
                    }
                }
                Err(_) => {
                    // Test panicked as expected
                    TestResult::Pass
                }
            }
        } else {
            // Normal test execution
            let panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                test_fn()
            }));
            
            match panic_result {
                Ok(test_result) => test_result,
                Err(_) => TestResult::Fail("Test panicked unexpectedly".to_string()),
            }
        };
        
        let duration = start_time.elapsed();
        
        // Check for timeout
        let final_result = if let OvieOption::Some(timeout_ms) = test_case.timeout_ms {
            if duration.as_millis() > timeout_ms as u128 {
                TestResult::Fail(format!("Test timed out after {}ms", timeout_ms))
            } else {
                result
            }
        } else {
            result
        };
        
        TestExecution::new(test_case.clone(), final_result, duration)
            .with_output(context.output)
    }
    
    /// Print test start message
    fn print_test_start(&self, test_count: usize) {
        println!("Running {} tests", test_count);
        println!();
    }
    
    /// Print individual test result
    fn print_test_result(&self, execution: &TestExecution) {
        if self.config.verbose {
            match &execution.result {
                TestResult::Pass => {
                    println!("test {} ... ok ({:.2?})", execution.test_case.name, execution.duration);
                }
                TestResult::Fail(message) => {
                    println!("test {} ... FAILED ({:.2?})", execution.test_case.name, execution.duration);
                    if self.config.show_output {
                        println!("  {}", message);
                    }
                }
                TestResult::Skip(reason) => {
                    println!("test {} ... ignored ({:.2?})", execution.test_case.name, execution.duration);
                    if self.config.show_output {
                        println!("  {}", reason);
                    }
                }
            }
        } else {
            // Compact output
            match &execution.result {
                TestResult::Pass => print!("."),
                TestResult::Fail(_) => print!("F"),
                TestResult::Skip(_) => print!("i"),
            }
        }
    }
    
    /// Print test summary
    fn print_test_summary(&self, result: &TestSuiteResult) {
        if !self.config.verbose {
            println!(); // New line after compact output
        }
        
        println!();
        
        // Print failed tests details
        let failed = result.failed_executions();
        if failed.len() > 0 {
            println!("failures:");
            println!();
            
            for i in 0..failed.len() {
                if let OvieOption::Some(execution) = failed.get(i) {
                    println!("---- {} ----", execution.test_case.name);
                    if let TestResult::Fail(message) = &execution.result {
                        println!("{}", message);
                    }
                    println!();
                }
            }
        }
        
        // Print summary statistics
        println!("test result: {}. {} passed; {} failed; {} ignored; {} measured; 0 filtered out; finished in {:.2?}",
            if result.all_passed() { "ok" } else { "FAILED" },
            result.stats.passed,
            result.stats.failed,
            result.stats.ignored,
            0, // measured (for benchmark tests, not implemented)
            result.total_duration
        );
        
        if result.stats.total > 0 {
            println!("Success rate: {:.1}%", result.stats.success_rate());
        }
    }
}

/// Test report formatter for different output formats
#[derive(Debug)]
pub struct TestReportFormatter;

impl TestReportFormatter {
    /// Format test results as plain text
    pub fn format_plain_text(result: &TestSuiteResult) -> String {
        let mut output = String::new();
        
        output.push_str(&format!("Test Results Summary\n"));
        output.push_str(&format!("===================\n\n"));
        
        output.push_str(&format!("Total tests: {}\n", result.stats.total));
        output.push_str(&format!("Passed: {}\n", result.stats.passed));
        output.push_str(&format!("Failed: {}\n", result.stats.failed));
        output.push_str(&format!("Skipped: {}\n", result.stats.skipped));
        output.push_str(&format!("Ignored: {}\n", result.stats.ignored));
        output.push_str(&format!("Duration: {:.2?}\n", result.total_duration));
        output.push_str(&format!("Success rate: {:.1}%\n\n", result.stats.success_rate()));
        
        // Failed tests details
        let failed = result.failed_executions();
        if failed.len() > 0 {
            output.push_str("Failed Tests:\n");
            output.push_str("-------------\n");
            
            for i in 0..failed.len() {
                if let OvieOption::Some(execution) = failed.get(i) {
                    output.push_str(&format!("• {} ({:.2?})\n", execution.test_case.name, execution.duration));
                    if let TestResult::Fail(message) = &execution.result {
                        output.push_str(&format!("  Error: {}\n", message));
                    }
                    output.push_str("\n");
                }
            }
        }
        
        output
    }
    
    /// Format test results as JSON
    pub fn format_json(result: &TestSuiteResult) -> String {
        let mut output = String::new();
        
        output.push_str("{\n");
        output.push_str(&format!("  \"summary\": {{\n"));
        output.push_str(&format!("    \"total\": {},\n", result.stats.total));
        output.push_str(&format!("    \"passed\": {},\n", result.stats.passed));
        output.push_str(&format!("    \"failed\": {},\n", result.stats.failed));
        output.push_str(&format!("    \"skipped\": {},\n", result.stats.skipped));
        output.push_str(&format!("    \"ignored\": {},\n", result.stats.ignored));
        output.push_str(&format!("    \"duration_ms\": {},\n", result.total_duration.as_millis()));
        output.push_str(&format!("    \"success_rate\": {:.1}\n", result.stats.success_rate()));
        output.push_str("  },\n");
        
        output.push_str("  \"tests\": [\n");
        for i in 0..result.executions.len() {
            if let OvieOption::Some(execution) = result.executions.get(i) {
                if i > 0 {
                    output.push_str(",\n");
                }
                
                output.push_str("    {\n");
                output.push_str(&format!("      \"name\": \"{}\",\n", execution.test_case.name));
                output.push_str(&format!("      \"status\": \"{}\",\n", match &execution.result {
                    TestResult::Pass => "pass",
                    TestResult::Fail(_) => "fail",
                    TestResult::Skip(_) => "skip",
                }));
                output.push_str(&format!("      \"duration_ms\": {}", execution.duration.as_millis()));
                
                match &execution.result {
                    TestResult::Fail(message) => {
                        output.push_str(",\n");
                        output.push_str(&format!("      \"error\": \"{}\"", message.replace("\"", "\\\"")));
                    }
                    TestResult::Skip(reason) => {
                        output.push_str(",\n");
                        output.push_str(&format!("      \"skip_reason\": \"{}\"", reason.replace("\"", "\\\"")));
                    }
                    _ => {}
                }
                
                output.push_str("\n    }");
            }
        }
        output.push_str("\n  ]\n");
        output.push_str("}\n");
        
        output
    }
    
    /// Format test results as XML (JUnit format)
    pub fn format_junit_xml(result: &TestSuiteResult) -> String {
        let mut output = String::new();
        
        output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        output.push_str(&format!(
            "<testsuite name=\"ovie-tests\" tests=\"{}\" failures=\"{}\" skipped=\"{}\" time=\"{:.3}\">\n",
            result.stats.total,
            result.stats.failed,
            result.stats.skipped + result.stats.ignored,
            result.total_duration.as_secs_f64()
        ));
        
        for i in 0..result.executions.len() {
            if let OvieOption::Some(execution) = result.executions.get(i) {
                output.push_str(&format!(
                    "  <testcase name=\"{}\" time=\"{:.3}\"",
                    execution.test_case.name,
                    execution.duration.as_secs_f64()
                ));
                
                match &execution.result {
                    TestResult::Pass => {
                        output.push_str(" />\n");
                    }
                    TestResult::Fail(message) => {
                        output.push_str(">\n");
                        output.push_str(&format!("    <failure message=\"{}\">{}</failure>\n", 
                            message.replace("\"", "&quot;").replace("<", "&lt;").replace(">", "&gt;"),
                            message.replace("<", "&lt;").replace(">", "&gt;").replace("&", "&amp;")
                        ));
                        output.push_str("  </testcase>\n");
                    }
                    TestResult::Skip(reason) => {
                        output.push_str(">\n");
                        output.push_str(&format!("    <skipped message=\"{}\" />\n", 
                            reason.replace("\"", "&quot;").replace("<", "&lt;").replace(">", "&gt;")
                        ));
                        output.push_str("  </testcase>\n");
                    }
                }
            }
        }
        
        output.push_str("</testsuite>\n");
        output
    }
}

/// Convenience function to run all tests with default configuration
pub fn run_tests() -> TestSuiteResult {
    let runner = TestRunner::default();
    runner.run_all_tests()
}

/// Convenience function to run tests with custom configuration
pub fn run_tests_with_config(config: TestConfig) -> TestSuiteResult {
    let runner = TestRunner::new(config);
    runner.run_all_tests()
}

/// Convenience function to run tests and exit with appropriate code
pub fn run_tests_and_exit() -> ! {
    let result = run_tests();
    let exit_code = if result.all_passed() { 0 } else { 1 };
    std::process::exit(exit_code);
}

/// Convenience function to run tests with config and exit with appropriate code
pub fn run_tests_with_config_and_exit(config: TestConfig) -> ! {
    let result = run_tests_with_config(config);
    let exit_code = if result.all_passed() { 0 } else { 1 };
    std::process::exit(exit_code);
}

#[cfg(test)]
mod runner_tests {
    use super::*;

    fn dummy_passing_test() -> TestResult {
        TestResult::Pass
    }
    
    fn dummy_failing_test() -> TestResult {
        TestResult::Fail("This test always fails".to_string())
    }
    
    fn dummy_skipping_test() -> TestResult {
        TestResult::Skip("This test is not implemented".to_string())
    }
    
    #[test]
    fn test_test_execution_creation() {
        let test_case = TestCase::new("test_example".to_string());
        let result = TestResult::Pass;
        let duration = Duration::from_millis(100);
        
        let execution = TestExecution::new(test_case.clone(), result.clone(), duration);
        
        assert_eq!(execution.test_case.name, "test_example");
        assert!(execution.result.is_pass());
        assert_eq!(execution.duration, duration);
        assert!(execution.passed());
        assert!(!execution.failed());
        assert!(!execution.skipped());
    }
    
    #[test]
    fn test_test_suite_result() {
        let config = TestConfig::new();
        let mut suite_result = TestSuiteResult::new(config);
        
        let test_case1 = TestCase::new("test1".to_string());
        let execution1 = TestExecution::new(test_case1, TestResult::Pass, Duration::from_millis(50));
        
        let test_case2 = TestCase::new("test2".to_string());
        let execution2 = TestExecution::new(test_case2, TestResult::Fail("error".to_string()), Duration::from_millis(75));
        
        suite_result.add_execution(execution1);
        suite_result.add_execution(execution2);
        
        assert_eq!(suite_result.stats.total, 2);
        assert_eq!(suite_result.stats.passed, 1);
        assert_eq!(suite_result.stats.failed, 1);
        assert!(!suite_result.all_passed());
        
        let failed = suite_result.failed_executions();
        assert_eq!(failed.len(), 1);
    }
    
    #[test]
    fn test_test_runner_single_test() {
        let config = TestConfig::new();
        let runner = TestRunner::new(config);
        
        let test_case = TestCase::new("passing_test".to_string());
        let execution = runner.run_single_test(&test_case, dummy_passing_test);
        
        assert!(execution.passed());
        assert_eq!(execution.test_case.name, "passing_test");
    }
    
    #[test]
    fn test_test_runner_should_panic() {
        let config = TestConfig::new();
        let runner = TestRunner::new(config);
        
        // Test that should panic and does panic
        let test_case = TestCase::new("panic_test".to_string()).should_panic();
        
        fn panicking_test() -> TestResult {
            panic!("This test panics");
        }
        
        let execution = runner.run_single_test(&test_case, panicking_test);
        assert!(execution.passed()); // Should pass because it panicked as expected
        
        // Test that should panic but doesn't
        let execution2 = runner.run_single_test(&test_case, dummy_passing_test);
        assert!(execution2.failed()); // Should fail because it didn't panic
    }
    
    #[test]
    fn test_test_runner_timeout() {
        let config = TestConfig::new();
        let runner = TestRunner::new(config);
        
        let test_case = TestCase::new("timeout_test".to_string()).with_timeout(1); // 1ms timeout
        
        fn slow_test() -> TestResult {
            std::thread::sleep(Duration::from_millis(10)); // Sleep longer than timeout
            TestResult::Pass
        }
        
        let execution = runner.run_single_test(&test_case, slow_test);
        assert!(execution.failed());
        
        if let TestResult::Fail(message) = &execution.result {
            assert!(message.contains("timed out"));
        }
    }
    
    #[test]
    fn test_test_runner_ignored_tests() {
        let config = TestConfig::new(); // Don't include ignored tests
        let runner = TestRunner::new(config);
        
        let test_case = TestCase::new("ignored_test".to_string()).ignore();
        let execution = runner.run_single_test(&test_case, dummy_passing_test);
        
        assert!(execution.skipped());
        
        // Test with include_ignored = true
        let config2 = TestConfig::new().include_ignored();
        let runner2 = TestRunner::new(config2);
        let execution2 = runner2.run_single_test(&test_case, dummy_passing_test);
        
        assert!(execution2.passed()); // Should run and pass
    }
    
    #[test]
    fn test_report_formatter_plain_text() {
        let config = TestConfig::new();
        let mut result = TestSuiteResult::new(config);
        
        let test_case1 = TestCase::new("test1".to_string());
        let execution1 = TestExecution::new(test_case1, TestResult::Pass, Duration::from_millis(50));
        
        let test_case2 = TestCase::new("test2".to_string());
        let execution2 = TestExecution::new(test_case2, TestResult::Fail("error".to_string()), Duration::from_millis(75));
        
        result.add_execution(execution1);
        result.add_execution(execution2);
        
        let report = TestReportFormatter::format_plain_text(&result);
        
        assert!(report.contains("Test Results Summary"));
        assert!(report.contains("Total tests: 2"));
        assert!(report.contains("Passed: 1"));
        assert!(report.contains("Failed: 1"));
        assert!(report.contains("Failed Tests:"));
        assert!(report.contains("test2"));
    }
    
    #[test]
    fn test_report_formatter_json() {
        let config = TestConfig::new();
        let mut result = TestSuiteResult::new(config);
        
        let test_case = TestCase::new("test1".to_string());
        let execution = TestExecution::new(test_case, TestResult::Pass, Duration::from_millis(50));
        result.add_execution(execution);
        
        let report = TestReportFormatter::format_json(&result);
        
        assert!(report.contains("\"summary\""));
        assert!(report.contains("\"total\": 1"));
        assert!(report.contains("\"passed\": 1"));
        assert!(report.contains("\"tests\""));
        assert!(report.contains("\"name\": \"test1\""));
        assert!(report.contains("\"status\": \"pass\""));
    }
    
    #[test]
    fn test_report_formatter_junit_xml() {
        let config = TestConfig::new();
        let mut result = TestSuiteResult::new(config);
        
        let test_case = TestCase::new("test1".to_string());
        let execution = TestExecution::new(test_case, TestResult::Fail("error message".to_string()), Duration::from_millis(50));
        result.add_execution(execution);
        
        let report = TestReportFormatter::format_junit_xml(&result);
        
        assert!(report.contains("<?xml version=\"1.0\""));
        assert!(report.contains("<testsuite"));
        assert!(report.contains("tests=\"1\""));
        assert!(report.contains("failures=\"1\""));
        assert!(report.contains("<testcase name=\"test1\""));
        assert!(report.contains("<failure"));
        assert!(report.contains("error message"));
    }
}
