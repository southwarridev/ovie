//! Conformance Tests for Ovie Compiler
//! 
//! This module contains tests that verify compliance with formal specifications
//! and language standards.

pub mod language_spec;
pub mod stdlib_spec;
pub mod abi_spec;

// Re-export all conformance test modules
pub use language_spec::*;
pub use stdlib_spec::*;
pub use abi_spec::*;