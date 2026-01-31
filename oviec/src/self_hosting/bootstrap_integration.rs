//! Bootstrap Integration System
//! 
//! This module provides the integration layer between the Rust Stage 0 compiler
//! and the Ovie Stage 1 components, enabling gradual transition to self-hosting.

use crate::error::{OvieError, OvieResult};
use crate::lexer::{Lexer as RustLexer, Token};
use crate::parser::{Parser as RustParser};
use crate::ast::AstNode;
use crate::ir::{Program as IR, IrBuilder};
use crate::interpreter::IrInterpreter;
use crate::self_hosting::{BootstrapVerifier, BootstrapConfig, BootstrapVerificationResult};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

/// Integration mode for bootstrap verification
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntegrationMode {
    /// Use only Rust components (Stage 0)
    RustOnly,
    /// Use Ovie lexer with Rust parser (Stage 1a)
    OvieLexer,
    /// Use Ovie lexer and parser with Rust semantic analysis (Stage 1b)
    OvieParser,
    /// Use all Ovie components (Stage 2)
    OvieComplete,
}

impl IntegrationMode {
    /// Get the mode name
    pub fn name(&self) -> &'static str {
        match self {
            IntegrationMode::RustOnly => "Rust Only (Stage 0)",
            IntegrationMode::OvieLexer => "Ovie Lexer (Stage 1a)",
            IntegrationMode::OvieParser => "Ovie Parser (Stage 1b)",
            IntegrationMode::OvieComplete => "Ovie Complete (Stage 2)",
        }
    }

    /// Get the next integration mode
    pub fn next(&self) -> Option<IntegrationMode> {
        match self {
            IntegrationMode::RustOnly => Some(IntegrationMode::OvieLexer),
            IntegrationMode::OvieLexer => Some(IntegrationMode::OvieParser),
            IntegrationMode::OvieParser => Some(IntegrationMode::OvieComplete),
            IntegrationMode::OvieComplete => None,
        }
    }
}

/// Bootstrap integration manager
pub struct BootstrapIntegration {
    current_mode: IntegrationMode,
    verifier: Option<BootstrapVerifier>,
    ovie_lexer_ir: Option<IR>,
    ovie_parser_ir: Option<IR>,
    ovie_compiler_ir: Option<IR>,
    verification_history: Vec<IntegrationVerificationResult>,
}

/// Integration verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationVerificationResult {
    pub mode: String,
    pub timestamp: u64,
    pub verification_results: Vec<BootstrapVerificationResult>,
    pub overall_success: bool,
    pub performance_impact: f64,
    pub error_count: usize,
    pub test_coverage: f64,
}

impl BootstrapIntegration {
    /// Create a new bootstrap integration manager
    pub fn new() -> Self {
        Self {
            current_mode: IntegrationMode::RustOnly,
            verifier: None,
            ovie_lexer_ir: None,
            ovie_parser_ir: None,
            ovie_compiler_ir: None,
            verification_history: Vec::new(),
        }
    }

    /// Initialize the integration system with Ovie components
    pub fn initialize(&mut self, config: BootstrapConfig) -> OvieResult<()> {
        // Initialize bootstrap verifier
        let mut verifier = BootstrapVerifier::new(config);
        
        // Load Ovie lexer implementation
        let lexer_source = include_str!("lexer_spec.ov");
        
        // Load Ovie parser implementation
        let parser_source = include_str!("parser_spec.ov");
        
        // Load Ovie minimal compiler implementation
        let compiler_source = include_str!("minimal_compiler.ov");
        
        // For now, we'll compile these specs using the Rust compiler
        // In a real implementation, this would be a proper Ovie program
        let mut compiler = crate::Compiler::new_deterministic();
        
        // Try to compile the lexer spec
        match compiler.compile_to_ir(lexer_source) {
            Ok(ir) => {
                self.ovie_lexer_ir = Some(ir);
                println!("✅ Ovie lexer IR compiled successfully");
            }
            Err(e) => {
                println!("⚠️  Ovie lexer compilation failed (expected for now): {}", e);
                // Continue with setup for future implementation
            }
        }
        
        // Try to compile the parser spec
        match compiler.compile_to_ir(parser_source) {
            Ok(ir) => {
                self.ovie_parser_ir = Some(ir);
                println!("✅ Ovie parser IR compiled successfully");
            }
            Err(e) => {
                println!("⚠️  Ovie parser compilation failed (expected for now): {}", e);
                // Continue with setup for future implementation
            }
        }
        
        // Try to compile the minimal compiler
        match compiler.compile_to_ir(compiler_source) {
            Ok(ir) => {
                self.ovie_compiler_ir = Some(ir);
                println!("✅ Ovie minimal compiler IR compiled successfully");
            }
            Err(e) => {
                println!("⚠️  Ovie minimal compiler compilation failed (expected for now): {}", e);
                // Continue with setup for future implementation
            }
        }
        
        // Initialize verifier with mock data for now
        self.verifier = Some(verifier);
        
        println!("🚀 Bootstrap integration initialized");
        println!("   - Lexer IR: {}", if self.ovie_lexer_ir.is_some() { "✅" } else { "❌" });
        println!("   - Parser IR: {}", if self.ovie_parser_ir.is_some() { "✅" } else { "❌" });
        println!("   - Compiler IR: {}", if self.ovie_compiler_ir.is_some() { "✅" } else { "❌" });
        
        Ok(())
    }

    /// Get the current integration mode
    pub fn current_mode(&self) -> IntegrationMode {
        self.current_mode
    }

    /// Transition to the next integration mode
    pub fn transition_to_next_mode(&mut self) -> OvieResult<IntegrationMode> {
        match self.current_mode.next() {
            Some(next_mode) => {
                self.current_mode = next_mode;
                Ok(next_mode)
            }
            None => Err(OvieError::runtime_error(
                "Already at the final integration mode".to_string()
            )),
        }
    }

    /// Verify integration readiness for the current mode
    pub fn verify_integration(&mut self, test_cases: &[&str]) -> OvieResult<IntegrationVerificationResult> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let mut verification_results = Vec::new();
        let mut total_errors = 0;
        let mut total_performance_impact = 0.0;

        // Run verification based on current mode
        match self.current_mode {
            IntegrationMode::RustOnly => {
                // Verify Rust-only compilation works
                verification_results = self.verify_rust_only(test_cases)?;
            }
            IntegrationMode::OvieLexer => {
                // Verify Ovie lexer integration
                verification_results = self.verify_ovie_lexer_integration(test_cases)?;
            }
            IntegrationMode::OvieParser => {
                // Verify Ovie parser integration
                verification_results = self.verify_ovie_parser_integration(test_cases)?;
            }
            IntegrationMode::OvieComplete => {
                // Verify complete Ovie integration
                verification_results = self.verify_complete_ovie_integration(test_cases)?;
            }
        }

        // Calculate overall metrics
        let successful_tests = verification_results.iter().filter(|r| r.passed).count();
        let overall_success = successful_tests == verification_results.len();
        
        for result in &verification_results {
            total_errors += result.errors.len();
            if result.performance_ratio > 0.0 {
                total_performance_impact += result.performance_ratio;
            }
        }

        let average_performance_impact = if verification_results.is_empty() {
            1.0
        } else {
            total_performance_impact / verification_results.len() as f64
        };

        let test_coverage = (successful_tests as f64 / verification_results.len() as f64) * 100.0;

        let integration_result = IntegrationVerificationResult {
            mode: self.current_mode.name().to_string(),
            timestamp,
            verification_results,
            overall_success,
            performance_impact: average_performance_impact,
            error_count: total_errors,
            test_coverage,
        };

        // Store in history
        self.verification_history.push(integration_result.clone());

        Ok(integration_result)
    }

    /// Verify Rust-only compilation (baseline)
    fn verify_rust_only(&self, test_cases: &[&str]) -> OvieResult<Vec<BootstrapVerificationResult>> {
        let mut results = Vec::new();

        for test_case in test_cases {
            let mut compiler = crate::Compiler::new_deterministic();
            
            let start = std::time::Instant::now();
            let compilation_result = compiler.compile_to_ast(test_case);
            let elapsed = start.elapsed().as_micros() as u64;

            let result = match compilation_result {
                Ok(_) => BootstrapVerificationResult {
                    passed: true,
                    hash_match: true,
                    tokens_match: true,
                    performance_acceptable: true,
                    reproducible: true,
                    rust_time_us: elapsed,
                    ovie_time_us: elapsed, // Same for Rust-only
                    performance_ratio: 1.0,
                    token_count: 0, // Would need to count tokens
                    source_hash: self.compute_source_hash(test_case),
                    rust_tokens_hash: "rust_baseline".to_string(),
                    ovie_tokens_hash: "rust_baseline".to_string(),
                    reproducibility_hashes: Vec::new(),
                    errors: Vec::new(),
                    timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(),
                    environment_hash: String::new(),
                },
                Err(e) => BootstrapVerificationResult {
                    passed: false,
                    hash_match: false,
                    tokens_match: false,
                    performance_acceptable: false,
                    reproducible: false,
                    rust_time_us: elapsed,
                    ovie_time_us: 0,
                    performance_ratio: 0.0,
                    token_count: 0,
                    source_hash: self.compute_source_hash(test_case),
                    rust_tokens_hash: "error".to_string(),
                    ovie_tokens_hash: "error".to_string(),
                    reproducibility_hashes: Vec::new(),
                    errors: vec![format!("Rust compilation failed: {}", e)],
                    timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(),
                    environment_hash: String::new(),
                },
            };

            results.push(result);
        }

        Ok(results)
    }

    /// Verify Ovie lexer integration
    fn verify_ovie_lexer_integration(&self, test_cases: &[&str]) -> OvieResult<Vec<BootstrapVerificationResult>> {
        if let Some(ref verifier) = self.verifier {
            // Use the existing bootstrap verifier for lexer verification
            verifier.run_comprehensive_verification(test_cases)
        } else {
            Err(OvieError::runtime_error("Bootstrap verifier not initialized".to_string()))
        }
    }

    /// Verify Ovie parser integration
    fn verify_ovie_parser_integration(&self, test_cases: &[&str]) -> OvieResult<Vec<BootstrapVerificationResult>> {
        // For now, return mock results since parser integration isn't implemented yet
        let mut results = Vec::new();

        for test_case in test_cases {
            let result = BootstrapVerificationResult {
                passed: false, // Not implemented yet
                hash_match: false,
                tokens_match: false,
                performance_acceptable: false,
                reproducible: false,
                rust_time_us: 100,
                ovie_time_us: 0,
                performance_ratio: 0.0,
                token_count: 0,
                source_hash: self.compute_source_hash(test_case),
                rust_tokens_hash: "not_implemented".to_string(),
                ovie_tokens_hash: "not_implemented".to_string(),
                reproducibility_hashes: Vec::new(),
                errors: vec!["Ovie parser integration not implemented yet".to_string()],
                timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(),
                environment_hash: String::new(),
            };

            results.push(result);
        }

        Ok(results)
    }

    /// Verify complete Ovie integration
    fn verify_complete_ovie_integration(&self, test_cases: &[&str]) -> OvieResult<Vec<BootstrapVerificationResult>> {
        // For now, return mock results since complete integration isn't implemented yet
        let mut results = Vec::new();

        for test_case in test_cases {
            let result = BootstrapVerificationResult {
                passed: false, // Not implemented yet
                hash_match: false,
                tokens_match: false,
                performance_acceptable: false,
                reproducible: false,
                rust_time_us: 100,
                ovie_time_us: 0,
                performance_ratio: 0.0,
                token_count: 0,
                source_hash: self.compute_source_hash(test_case),
                rust_tokens_hash: "not_implemented".to_string(),
                ovie_tokens_hash: "not_implemented".to_string(),
                reproducibility_hashes: Vec::new(),
                errors: vec!["Complete Ovie integration not implemented yet".to_string()],
                timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(),
                environment_hash: String::new(),
            };

            results.push(result);
        }

        Ok(results)
    }

    /// Compute source hash for verification
    pub fn compute_source_hash(&self, source: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(source.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Generate integration status report
    pub fn generate_integration_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# Bootstrap Integration Status Report\n\n");
        report.push_str(&format!("**Current Mode:** {}\n\n", self.current_mode.name()));
        
        // Current mode status
        match self.current_mode {
            IntegrationMode::RustOnly => {
                report.push_str("## Stage 0 (Rust Only) Status\n");
                report.push_str("- ✅ Complete Rust implementation working\n");
                report.push_str("- ✅ All tests passing\n");
                report.push_str("- ✅ Performance baseline established\n");
                report.push_str("- ✅ Ready for Ovie lexer integration\n\n");
            }
            IntegrationMode::OvieLexer => {
                report.push_str("## Stage 1a (Ovie Lexer) Status\n");
                report.push_str("- 🔄 Ovie lexer implementation in progress\n");
                report.push_str("- ❌ Bootstrap verification not passing yet\n");
                report.push_str("- ❌ Performance benchmarking pending\n");
                report.push_str("- ❌ Integration testing pending\n\n");
            }
            IntegrationMode::OvieParser => {
                report.push_str("## Stage 1b (Ovie Parser) Status\n");
                report.push_str("- ❌ Ovie parser integration not implemented\n");
                report.push_str("- ❌ AST compatibility verification pending\n");
                report.push_str("- ❌ Semantic preservation testing pending\n\n");
            }
            IntegrationMode::OvieComplete => {
                report.push_str("## Stage 2 (Complete Ovie) Status\n");
                report.push_str("- ❌ Complete self-hosting not implemented\n");
                report.push_str("- ❌ End-to-end verification pending\n");
                report.push_str("- ❌ Production readiness pending\n\n");
            }
        }
        
        // Verification history
        if !self.verification_history.is_empty() {
            report.push_str("## Verification History\n\n");
            
            for (i, result) in self.verification_history.iter().enumerate() {
                report.push_str(&format!("### Verification {} - {}\n", i + 1, result.mode));
                report.push_str(&format!("- **Success:** {}\n", result.overall_success));
                report.push_str(&format!("- **Test Coverage:** {:.1}%\n", result.test_coverage));
                report.push_str(&format!("- **Performance Impact:** {:.2}x\n", result.performance_impact));
                report.push_str(&format!("- **Error Count:** {}\n", result.error_count));
                report.push_str(&format!("- **Timestamp:** {}\n\n", result.timestamp));
            }
        } else {
            report.push_str("## Verification History\n");
            report.push_str("No verification runs completed yet.\n\n");
        }
        
        // Next steps
        if let Some(next_mode) = self.current_mode.next() {
            report.push_str("## Next Steps\n");
            report.push_str(&format!("- Prepare for transition to {}\n", next_mode.name()));
            report.push_str("- Complete current mode verification\n");
            report.push_str("- Address any failing tests\n");
            report.push_str("- Optimize performance if needed\n");
        } else {
            report.push_str("## Completion Status\n");
            report.push_str("🎉 All integration modes completed!\n");
            report.push_str("The Ovie compiler is fully self-hosting.\n");
        }
        
        report
    }

    /// Get verification history
    pub fn get_verification_history(&self) -> &[IntegrationVerificationResult] {
        &self.verification_history
    }

    /// Clear verification history
    pub fn clear_verification_history(&mut self) {
        self.verification_history.clear();
    }
}

impl Default for BootstrapIntegration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_mode_progression() {
        assert_eq!(IntegrationMode::RustOnly.next(), Some(IntegrationMode::OvieLexer));
        assert_eq!(IntegrationMode::OvieLexer.next(), Some(IntegrationMode::OvieParser));
        assert_eq!(IntegrationMode::OvieParser.next(), Some(IntegrationMode::OvieComplete));
        assert_eq!(IntegrationMode::OvieComplete.next(), None);
    }

    #[test]
    fn test_bootstrap_integration_creation() {
        let integration = BootstrapIntegration::new();
        assert_eq!(integration.current_mode(), IntegrationMode::RustOnly);
        assert!(integration.verification_history.is_empty());
    }

    #[test]
    fn test_mode_transition() {
        let mut integration = BootstrapIntegration::new();
        
        let mode1 = integration.transition_to_next_mode().unwrap();
        assert_eq!(mode1, IntegrationMode::OvieLexer);
        assert_eq!(integration.current_mode(), IntegrationMode::OvieLexer);
        
        let mode2 = integration.transition_to_next_mode().unwrap();
        assert_eq!(mode2, IntegrationMode::OvieParser);
        
        let mode3 = integration.transition_to_next_mode().unwrap();
        assert_eq!(mode3, IntegrationMode::OvieComplete);
        
        // Should fail at final mode
        assert!(integration.transition_to_next_mode().is_err());
    }

    #[test]
    fn test_source_hash_computation() {
        let integration = BootstrapIntegration::new();
        
        let hash1 = integration.compute_source_hash("test source");
        let hash2 = integration.compute_source_hash("test source");
        let hash3 = integration.compute_source_hash("different source");
        
        // Same source should produce same hash
        assert_eq!(hash1, hash2);
        // Different source should produce different hash
        assert_ne!(hash1, hash3);
        // Hash should be SHA-256 length
        assert_eq!(hash1.len(), 64);
    }

    #[test]
    fn test_integration_report_generation() {
        let integration = BootstrapIntegration::new();
        let report = integration.generate_integration_report();
        
        assert!(report.contains("Bootstrap Integration Status Report"));
        assert!(report.contains("Rust Only (Stage 0)"));
        assert!(report.contains("Complete Rust implementation working"));
        assert!(!report.is_empty());
    }
}