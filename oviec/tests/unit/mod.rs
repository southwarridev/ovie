//! Unit Tests for Ovie Compiler Components
//! 
//! This module contains unit tests for individual compiler components,
//! focusing on specific functionality and edge cases.

pub mod lexer;
pub mod parser;
pub mod type_checker;
pub mod codegen;
pub mod runtime;
pub mod ast_invariants;
pub mod hir_invariants;
pub mod mir_invariants;
pub mod backend_invariants;

// Re-export all unit test modules
pub use lexer::*;
pub use parser::*;
pub use type_checker::*;
pub use codegen::*;
pub use runtime::*;
pub use ast_invariants::*;
pub use hir_invariants::*;
pub use mir_invariants::*;
pub use backend_invariants::*;