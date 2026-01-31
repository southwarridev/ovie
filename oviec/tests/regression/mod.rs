//! Regression Tests for Ovie Compiler
//! 
//! This module contains tests to detect behavioral regressions in compiler components.

pub mod compiler_behavior;
pub mod performance_regression;
pub mod regression_detector;

// Re-export all regression test modules
pub use compiler_behavior::*;
pub use performance_regression::*;
pub use regression_detector::*;