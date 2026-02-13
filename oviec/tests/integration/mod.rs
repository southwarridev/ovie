//! Integration Tests for Ovie Compiler
//! 
//! This module contains integration tests that verify end-to-end functionality
//! and cross-component interactions.

pub mod end_to_end;
pub mod cross_platform;
pub mod cross_platform_validator;
pub mod performance;
pub mod invariant_pipeline;

// Re-export all integration test modules
pub use end_to_end::*;
pub use cross_platform::*;
pub use cross_platform_validator::*;
pub use performance::*;
pub use invariant_pipeline::*;