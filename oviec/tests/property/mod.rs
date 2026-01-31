//! Property-Based Tests for Ovie Compiler
//! 
//! This module contains property-based tests that verify universal correctness
//! properties across randomized inputs, ensuring the compiler behaves correctly
//! for all valid programs.

pub mod compiler;
pub mod stdlib;
pub mod bootstrap;
pub mod security;

// Re-export all property test modules
pub use compiler::*;
pub use stdlib::*;
pub use bootstrap::*;
pub use security::*;