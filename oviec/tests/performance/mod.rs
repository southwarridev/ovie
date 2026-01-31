//! Performance Tests for Ovie Compiler
//! 
//! This module contains performance benchmarks and regression detection tests.

pub mod benchmarks;
pub mod regression;

// Re-export all performance test modules
pub use benchmarks::*;
pub use regression::*;