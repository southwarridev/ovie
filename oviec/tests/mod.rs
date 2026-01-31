//! Comprehensive Testing Framework for Ovie Compiler
//! 
//! This module provides the complete testing infrastructure for the Ovie Stage 2 compiler,
//! including unit tests, property-based tests, integration tests, and conformance tests.
//! 
//! ## Test Suite Organization
//! 
//! - `unit/`: Component-specific unit tests
//! - `property/`: Property-based tests for universal correctness properties
//! - `integration/`: End-to-end and cross-component integration tests
//! - `conformance/`: Language specification compliance tests
//! - `performance/`: Performance benchmarking and regression tests
//! - `regression/`: Regression test suite for compiler behavior

pub mod unit;
pub mod property;
pub mod integration;
pub mod conformance;
pub mod performance;
pub mod regression;
pub mod runner;
pub mod utils;

use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};

/// Test suite configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuiteConfig {
    /// Enable property-based testing
    pub enable_property_tests: bool,
    /// Number of iterations for property tests
    pub property_test_iterations: usize,
    /// Enable cross-platform testing
    pub enable_cross_platform: bool,
    /// Target platforms for cross-platform testing
    pub target_platforms: Vec<String>,
    /// Enable performance benchmarking
    pub enable_performance_tests: bool,
    /// Performance regression threshold (percentage)
    pub performance_regression_threshold: f64,
    /// Enable regression testing
    pub enable_regression_tests: bool,
    /// Test timeout in seconds
    pub test_timeout_seconds: u64,
    /// Enable deterministic test execution
    pub deterministic_execution: bool,
    /// Random seed for deterministic tests
    pub random_seed: Option<u64>,
}

impl Default for TestSuiteConfig {
    fn default() -> Self {
        Self {
            enable_property_tests: true,
            property_test_iterations: 1000, // Increased from standard 100 for compiler complexity
            enable_cross_platform: true,
            target_platforms: vec![
                "x86_64-pc-windows-gnu".to_string(),
                "x86_64-unknown-linux-gnu".to_string(),
                "wasm32-unknown-unknown".to_string(),
            ],
            enable_performance_tests: true,
            performance_regression_threshold: 5.0, // 5% regression threshold
            enable_regression_tests: true,
            test_timeout_seconds: 300, // 5 minutes per test
            deterministic_execution: true,
            random_seed: Some(42), // Fixed seed for reproducible tests
        }
    }
}

/// Test result for individual tests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    /// Test name
    pub name: String,
    /// Test category
    pub category: TestCategory,
    /// Test status
    pub status: TestStatus,
    /// Execution duration
    pub duration: Duration,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Test category classification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestCategory {
    Unit,
    Property,
    Integration,
    Conformance,
    Performance,
    Regression,
}

/// Test execution status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestStatus {
    Passed,
    Failed,
    Skipped,
    Timeout,
}

/// Comprehensive test suite results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuiteResults {
    /// Individual test results
    pub test_results: Vec<TestResult>,
    /// Overall execution time
    pub total_duration: Duration,
    /// Summary statistics
    pub summary: TestSummary,
    /// Cross-platform consistency results
    pub cross_platform_results: Option<CrossPlatformResults>,
    /// Performance benchmark results
    pub performance_results: Option<PerformanceResults>,
    /// Regression detection results
    pub regression_results: Option<RegressionResults>,
}

/// Test execution summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSummary {
    /// Total number of tests
    pub total_tests: usize,
    /// Number of passed tests
    pub passed: usize,
    /// Number of failed tests
    pub failed: usize,
    /// Number of skipped tests
    pub skipped: usize,
    /// Number of timed out tests
    pub timeout: usize,
    /// Success rate as percentage
    pub success_rate: f64,
}

/// Cross-platform test consistency results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossPlatformResults {
    /// Platform-specific results
    pub platform_results: HashMap<String, Vec<TestResult>>,
    /// Consistency analysis
    pub consistency_analysis: ConsistencyAnalysis,
}

/// Platform consistency analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyAnalysis {
    /// Tests that are consistent across all platforms
    pub consistent_tests: Vec<String>,
    /// Tests with platform-specific differences
    pub inconsistent_tests: Vec<InconsistentTest>,
    /// Overall consistency percentage
    pub consistency_percentage: f64,
}

/// Information about inconsistent test behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InconsistentTest {
    /// Test name
    pub test_name: String,
    /// Platform-specific results
    pub platform_differences: HashMap<String, String>,
    /// Severity of inconsistency
    pub severity: InconsistencySeverity,
}

/// Severity of cross-platform inconsistency
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InconsistencySeverity {
    /// Minor differences that don't affect correctness
    Minor,
    /// Significant differences that may indicate issues
    Major,
    /// Critical differences that indicate bugs
    Critical,
}

/// Performance benchmark results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceResults {
    /// Benchmark measurements
    pub benchmarks: Vec<BenchmarkResult>,
    /// Regression analysis
    pub regression_analysis: PerformanceRegressionAnalysis,
}

/// Individual benchmark result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Benchmark name
    pub name: String,
    /// Execution time
    pub execution_time: Duration,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Throughput (operations per second)
    pub throughput: Option<f64>,
    /// Baseline comparison
    pub baseline_comparison: Option<BaselineComparison>,
}

/// Comparison with performance baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineComparison {
    /// Baseline execution time
    pub baseline_time: Duration,
    /// Performance change percentage (positive = improvement)
    pub performance_change: f64,
    /// Whether this represents a regression
    pub is_regression: bool,
}

/// Performance regression analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRegressionAnalysis {
    /// Detected regressions
    pub regressions: Vec<PerformanceRegression>,
    /// Overall performance trend
    pub overall_trend: PerformanceTrend,
}

/// Individual performance regression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRegression {
    /// Benchmark name
    pub benchmark_name: String,
    /// Regression percentage
    pub regression_percentage: f64,
    /// Severity assessment
    pub severity: RegressionSeverity,
}

/// Performance trend analysis
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerformanceTrend {
    Improving,
    Stable,
    Degrading,
}

/// Regression severity classification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegressionSeverity {
    Minor,    // < 5% regression
    Major,    // 5-15% regression
    Critical, // > 15% regression
}

/// Regression test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionResults {
    /// Detected regressions
    pub detected_regressions: Vec<CompilerRegression>,
    /// Regression analysis summary
    pub analysis_summary: RegressionAnalysisSummary,
}

/// Individual compiler regression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerRegression {
    /// Test case that detected the regression
    pub test_case: String,
    /// Component affected
    pub component: String,
    /// Regression description
    pub description: String,
    /// Severity level
    pub severity: RegressionSeverity,
    /// Suggested fix or investigation
    pub suggested_action: String,
}

/// Summary of regression analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionAnalysisSummary {
    /// Total regressions detected
    pub total_regressions: usize,
    /// Regressions by severity
    pub regressions_by_severity: HashMap<RegressionSeverity, usize>,
    /// Components with regressions
    pub affected_components: Vec<String>,
    /// Overall regression risk assessment
    pub risk_assessment: RiskAssessment,
}

/// Risk assessment for detected regressions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskAssessment {
    Low,      // No critical regressions, few minor ones
    Medium,   // Some major regressions or many minor ones
    High,     // Critical regressions detected
}

impl TestSuiteResults {
    /// Create new empty test suite results
    pub fn new() -> Self {
        Self {
            test_results: Vec::new(),
            total_duration: Duration::from_secs(0),
            summary: TestSummary {
                total_tests: 0,
                passed: 0,
                failed: 0,
                skipped: 0,
                timeout: 0,
                success_rate: 0.0,
            },
            cross_platform_results: None,
            performance_results: None,
            regression_results: None,
        }
    }

    /// Add a test result
    pub fn add_test_result(&mut self, result: TestResult) {
        self.test_results.push(result);
        self.update_summary();
    }

    /// Update summary statistics
    fn update_summary(&mut self) {
        let total = self.test_results.len();
        let passed = self.test_results.iter().filter(|r| r.status == TestStatus::Passed).count();
        let failed = self.test_results.iter().filter(|r| r.status == TestStatus::Failed).count();
        let skipped = self.test_results.iter().filter(|r| r.status == TestStatus::Skipped).count();
        let timeout = self.test_results.iter().filter(|r| r.status == TestStatus::Timeout).count();

        self.summary = TestSummary {
            total_tests: total,
            passed,
            failed,
            skipped,
            timeout,
            success_rate: if total > 0 { (passed as f64 / total as f64) * 100.0 } else { 0.0 },
        };
    }

    /// Generate a human-readable report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("=== Ovie Compiler Test Suite Results ===\n\n");
        
        // Summary
        report.push_str(&format!("Total Tests: {}\n", self.summary.total_tests));
        report.push_str(&format!("Passed: {} ({:.1}%)\n", self.summary.passed, 
            if self.summary.total_tests > 0 { 
                (self.summary.passed as f64 / self.summary.total_tests as f64) * 100.0 
            } else { 0.0 }));
        report.push_str(&format!("Failed: {}\n", self.summary.failed));
        report.push_str(&format!("Skipped: {}\n", self.summary.skipped));
        report.push_str(&format!("Timeout: {}\n", self.summary.timeout));
        report.push_str(&format!("Success Rate: {:.1}%\n", self.summary.success_rate));
        report.push_str(&format!("Total Duration: {:.2}s\n\n", self.total_duration.as_secs_f64()));

        // Failed tests details
        if self.summary.failed > 0 {
            report.push_str("=== Failed Tests ===\n");
            for result in &self.test_results {
                if result.status == TestStatus::Failed {
                    report.push_str(&format!("- {} ({}): {}\n", 
                        result.name, 
                        format!("{:?}", result.category),
                        result.error_message.as_deref().unwrap_or("No error message")));
                }
            }
            report.push('\n');
        }

        // Cross-platform results
        if let Some(ref cross_platform) = self.cross_platform_results {
            report.push_str("=== Cross-Platform Consistency ===\n");
            report.push_str(&format!("Consistency: {:.1}%\n", cross_platform.consistency_analysis.consistency_percentage));
            if !cross_platform.consistency_analysis.inconsistent_tests.is_empty() {
                report.push_str("Inconsistent Tests:\n");
                for inconsistent in &cross_platform.consistency_analysis.inconsistent_tests {
                    report.push_str(&format!("- {} ({:?})\n", inconsistent.test_name, inconsistent.severity));
                }
            }
            report.push('\n');
        }

        // Performance results
        if let Some(ref performance) = self.performance_results {
            report.push_str("=== Performance Analysis ===\n");
            if !performance.regression_analysis.regressions.is_empty() {
                report.push_str("Performance Regressions:\n");
                for regression in &performance.regression_analysis.regressions {
                    report.push_str(&format!("- {} ({:.1}% regression, {:?})\n", 
                        regression.benchmark_name, 
                        regression.regression_percentage,
                        regression.severity));
                }
            } else {
                report.push_str("No performance regressions detected.\n");
            }
            report.push('\n');
        }

        // Regression results
        if let Some(ref regression) = self.regression_results {
            report.push_str("=== Regression Analysis ===\n");
            report.push_str(&format!("Total Regressions: {}\n", regression.analysis_summary.total_regressions));
            report.push_str(&format!("Risk Assessment: {:?}\n", regression.analysis_summary.risk_assessment));
            if !regression.detected_regressions.is_empty() {
                report.push_str("Detected Regressions:\n");
                for reg in &regression.detected_regressions {
                    report.push_str(&format!("- {} in {}: {} ({:?})\n", 
                        reg.test_case, reg.component, reg.description, reg.severity));
                }
            }
            report.push('\n');
        }

        report
    }
}

impl Default for TestSuiteResults {
    fn default() -> Self {
        Self::new()
    }
}