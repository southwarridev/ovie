// Performance Regression Tests for Ovie Compiler
// 
// This module contains tests to detect performance regressions in compiler components.

use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Performance regression test result
#[derive(Debug, Clone)]
pub struct PerformanceRegressionResult {
    pub test_name: String,
    pub current_duration: Duration,
    pub baseline_duration: Option<Duration>,
    pub regression_percentage: Option<f64>,
    pub is_regression: bool,
}

/// Performance regression detector
pub struct PerformanceRegressionDetector {
    baselines: HashMap<String, Duration>,
    regression_threshold: f64,
}

impl PerformanceRegressionDetector {
    /// Create new performance regression detector
    pub fn new(regression_threshold: f64) -> Self {
        Self {
            baselines: HashMap::new(),
            regression_threshold,
        }
    }
    
    /// Add baseline measurement
    pub fn add_baseline(&mut self, test_name: String, duration: Duration) {
        self.baselines.insert(test_name, duration);
    }
    
    /// Check for regression
    pub fn check_regression(&self, test_name: &str, current_duration: Duration) -> PerformanceRegressionResult {
        let baseline = self.baselines.get(test_name).copied();
        
        let (regression_percentage, is_regression) = if let Some(baseline) = baseline {
            let change = (current_duration.as_secs_f64() - baseline.as_secs_f64()) / baseline.as_secs_f64() * 100.0;
            (Some(change), change > self.regression_threshold)
        } else {
            (None, false)
        };
        
        PerformanceRegressionResult {
            test_name: test_name.to_string(),
            current_duration,
            baseline_duration: baseline,
            regression_percentage,
            is_regression,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_regression_detection() {
        let mut detector = PerformanceRegressionDetector::new(5.0); // 5% threshold
        
        // Add baseline
        detector.add_baseline("test_compile".to_string(), Duration::from_millis(100));
        
        // Test no regression
        let result = detector.check_regression("test_compile", Duration::from_millis(102));
        assert!(!result.is_regression);
        
        // Test regression
        let result = detector.check_regression("test_compile", Duration::from_millis(110));
        assert!(result.is_regression);
        assert!(result.regression_percentage.unwrap() > 5.0);
    }
    
    #[test]
    fn test_no_baseline() {
        let detector = PerformanceRegressionDetector::new(5.0);
        
        let result = detector.check_regression("unknown_test", Duration::from_millis(100));
        assert!(!result.is_regression);
        assert!(result.baseline_duration.is_none());
        assert!(result.regression_percentage.is_none());
    }
}