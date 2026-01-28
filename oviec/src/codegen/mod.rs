//! Code generation backends for the Ovie compiler

pub mod wasm;

#[cfg(feature = "llvm")]
pub mod llvm;

pub use wasm::WasmBackend;

#[cfg(feature = "llvm")]
pub use llvm::LlvmBackend;

/// Trait for code generation backends
pub trait CodegenBackend {
    type Output;
    type Error;

    /// Generate code from IR
    fn generate(&mut self, ir: &crate::ir::Program) -> Result<Self::Output, Self::Error>;
    
    /// Get the backend name
    fn name(&self) -> &'static str;
    
    /// Check if the backend supports the given target
    fn supports_target(&self, target: &str) -> bool;
}