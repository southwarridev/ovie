// Test modules for the Ovie compiler

pub mod property_tests;
pub mod grammar_validation_tests;
pub mod hir_tests;
pub mod mir_tests;
pub mod stdlib_integration_tests;
pub mod offline_first_tests;

// Re-export test functions for easy access
pub use property_tests::*;
pub use grammar_validation_tests::*;
pub use hir_tests::*;
pub use mir_tests::*;
pub use stdlib_integration_tests::*;
pub use offline_first_tests::*;