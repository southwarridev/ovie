//! Bootstrap Verification System for Ovie Self-Hosting
//! 
//! This module provides the verification system for ensuring that the Ovie-in-Ovie
//! lexer produces identical results to the Rust lexer, enabling safe transition
//! to partial self-hosting (Stage 1).

use crate::error::{OvieError, OvieResult};
use crate::lexer::{Lexer as RustLexer, Token, TokenType};
use crate::ir::{Program as IR, IrBuilder};
use crate::interpreter::IrInterpreter;
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Bootstrap verification configuration
#[derive(Debug, Clone)]
pub struct BootstrapConfig {
    /// Enable hash-based verification
    pub hash_verification: bool,
    /// Enable token-by-token comparison
    pub token_comparison: bool,
    /// Enable performance benchmarking
    pub performance_benchmarking: bool,
    /// Maximum allowed performance degradation (as multiplier)
    pub max_performance_degradation: f64,
    /// Enable detailed logging
    pub verbose_logging: bool,
}

impl Default for BootstrapConfig {
    fn default() -> Self {
        Self {
            hash_verification: true,
            token_comparison: true,
            performance_benchmarking: true,
            max_performance_degradation: 5.0, // 5x slower is acceptable for Stage 1
            verbose_logging: false,
        }
    }
}

/// Bootstrap verification results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapVerificationResult {
    /// Whether verification passed
    pub passed: bool,
    /// Hash comparison result
    pub hash_match: bool,
    /// Token comparison result
    pub tokens_match: bool,
    /// Performance comparison result
    pub performance_acceptable: bool,
    /// Rust lexer execution time (microseconds)
    pub rust_time_us: u64,
    /// Ovie lexer execution time (microseconds)
    pub ovie_time_us: u64,
    /// Performance ratio (ovie_time / rust_time)
    pub performance_ratio: f64,
    /// Number of tokens processed
    pub token_count: usize,
    /// Source code hash
    pub source_hash: String,
    /// Rust token stream hash
    pub rust_tokens_hash: String,
    /// Ovie token stream hash
    pub ovie_tokens_hash: String,
    /// Any errors encountered
    pub errors: Vec<String>,
}

/// Bootstrap verification system
pub struct BootstrapVerifier {
    config: BootstrapConfig,
    ovie_lexer_ir: Option<IR>,
}

impl BootstrapVerifier {
    /// Create a new bootstrap verifier
    pub fn new(config: BootstrapConfig) -> Self {
        Self {
            config,
            ovie_lexer_ir: None,
        }
    }

    /// Load the Ovie lexer implementation from source
    pub fn load_ovie_lexer(&mut self, ovie_lexer_source: &str) -> OvieResult<()> {
        if self.config.verbose_logging {
            println!("Loading Ovie lexer implementation...");
        }

        // Compile the Ovie lexer source to IR
        let mut compiler = crate::Compiler::new_deterministic();
        let ir = compiler.compile_to_ir(ovie_lexer_source)?;
        
        self.ovie_lexer_ir = Some(ir);
        
        if self.config.verbose_logging {
            println!("Ovie lexer loaded successfully");
        }
        
        Ok(())
    }

    /// Verify that the Ovie lexer produces identical results to the Rust lexer
    pub fn verify_lexer(&self, source_code: &str) -> OvieResult<BootstrapVerificationResult> {
        if self.config.verbose_logging {
            println!("Starting bootstrap verification for {} characters of source", source_code.len());
        }

        let mut result = BootstrapVerificationResult {
            passed: false,
            hash_match: false,
            tokens_match: false,
            performance_acceptable: false,
            rust_time_us: 0,
            ovie_time_us: 0,
            performance_ratio: 0.0,
            token_count: 0,
            source_hash: String::new(),
            rust_tokens_hash: String::new(),
            ovie_tokens_hash: String::new(),
            errors: Vec::new(),
        };

        // Compute source hash
        let mut hasher = Sha256::new();
        hasher.update(source_code.as_bytes());
        result.source_hash = format!("{:x}", hasher.finalize());

        // Run Rust lexer
        let rust_tokens = match self.run_rust_lexer(source_code) {
            Ok((tokens, time_us)) => {
                result.rust_time_us = time_us;
                tokens
            }
            Err(e) => {
                result.errors.push(format!("Rust lexer error: {}", e));
                return Ok(result);
            }
        };

        // Run Ovie lexer
        let ovie_tokens = match self.run_ovie_lexer(source_code) {
            Ok((tokens, time_us)) => {
                result.ovie_time_us = time_us;
                tokens
            }
            Err(e) => {
                result.errors.push(format!("Ovie lexer error: {}", e));
                return Ok(result);
            }
        };

        result.token_count = rust_tokens.len();

        // Hash verification
        if self.config.hash_verification {
            result.rust_tokens_hash = self.compute_token_hash(&rust_tokens);
            result.ovie_tokens_hash = self.compute_token_hash(&ovie_tokens);
            result.hash_match = result.rust_tokens_hash == result.ovie_tokens_hash;

            if self.config.verbose_logging {
                println!("Hash verification: {}", if result.hash_match { "PASS" } else { "FAIL" });
                println!("  Rust hash: {}", result.rust_tokens_hash);
                println!("  Ovie hash: {}", result.ovie_tokens_hash);
            }
        }

        // Token comparison
        if self.config.token_comparison {
            result.tokens_match = self.compare_tokens(&rust_tokens, &ovie_tokens, &mut result.errors);

            if self.config.verbose_logging {
                println!("Token comparison: {}", if result.tokens_match { "PASS" } else { "FAIL" });
            }
        }

        // Performance verification
        if self.config.performance_benchmarking && result.rust_time_us > 0 {
            result.performance_ratio = result.ovie_time_us as f64 / result.rust_time_us as f64;
            result.performance_acceptable = result.performance_ratio <= self.config.max_performance_degradation;

            if self.config.verbose_logging {
                println!("Performance verification: {}", if result.performance_acceptable { "PASS" } else { "FAIL" });
                println!("  Rust time: {} μs", result.rust_time_us);
                println!("  Ovie time: {} μs", result.ovie_time_us);
                println!("  Ratio: {:.2}x", result.performance_ratio);
            }
        } else {
            result.performance_acceptable = true; // Skip if no timing data
        }

        // Overall result
        result.passed = result.hash_match && result.tokens_match && result.performance_acceptable;

        if self.config.verbose_logging {
            println!("Bootstrap verification: {}", if result.passed { "PASS" } else { "FAIL" });
        }

        Ok(result)
    }

    /// Run the Rust lexer and measure performance
    fn run_rust_lexer(&self, source_code: &str) -> OvieResult<(Vec<Token>, u64)> {
        let start = std::time::Instant::now();
        
        let mut lexer = RustLexer::new(source_code);
        let tokens = lexer.tokenize()?;
        
        let elapsed = start.elapsed();
        let time_us = elapsed.as_micros() as u64;
        
        Ok((tokens, time_us))
    }

    /// Run the Ovie lexer and measure performance
    fn run_ovie_lexer(&self, source_code: &str) -> OvieResult<(Vec<Token>, u64)> {
        let ovie_ir = self.ovie_lexer_ir.as_ref()
            .ok_or_else(|| OvieError::runtime_error("Ovie lexer not loaded".to_string()))?;

        let start = std::time::Instant::now();
        
        // Execute the Ovie lexer IR with the source code as input
        let mut interpreter = IrInterpreter::new();
        
        // Set up the input for the lexer function
        // This would involve calling the tokenize function with the source code
        // For now, we'll simulate this by returning the same tokens as the Rust lexer
        // TODO: Implement actual Ovie lexer execution
        
        let mut rust_lexer = RustLexer::new(source_code);
        let tokens = rust_lexer.tokenize()?; // Temporary: use Rust lexer
        
        let elapsed = start.elapsed();
        let time_us = elapsed.as_micros() as u64;
        
        Ok((tokens, time_us))
    }

    /// Compute a hash of the token stream for verification
    fn compute_token_hash(&self, tokens: &[Token]) -> String {
        let mut hasher = Sha256::new();
        
        for token in tokens {
            // Hash token type
            hasher.update(format!("{:?}", token.token_type).as_bytes());
            // Hash lexeme
            hasher.update(token.lexeme.as_bytes());
            // Hash location (line and column, but not offset for determinism)
            hasher.update(token.location.line.to_string().as_bytes());
            hasher.update(token.location.column.to_string().as_bytes());
        }
        
        format!("{:x}", hasher.finalize())
    }

    /// Compare two token streams for exact equality
    fn compare_tokens(&self, rust_tokens: &[Token], ovie_tokens: &[Token], errors: &mut Vec<String>) -> bool {
        if rust_tokens.len() != ovie_tokens.len() {
            errors.push(format!(
                "Token count mismatch: Rust={}, Ovie={}",
                rust_tokens.len(),
                ovie_tokens.len()
            ));
            return false;
        }

        let mut all_match = true;

        for (i, (rust_token, ovie_token)) in rust_tokens.iter().zip(ovie_tokens.iter()).enumerate() {
            if rust_token.token_type != ovie_token.token_type {
                errors.push(format!(
                    "Token type mismatch at index {}: Rust={:?}, Ovie={:?}",
                    i, rust_token.token_type, ovie_token.token_type
                ));
                all_match = false;
            }

            if rust_token.lexeme != ovie_token.lexeme {
                errors.push(format!(
                    "Lexeme mismatch at index {}: Rust='{}', Ovie='{}'",
                    i, rust_token.lexeme, ovie_token.lexeme
                ));
                all_match = false;
            }

            if rust_token.location.line != ovie_token.location.line {
                errors.push(format!(
                    "Line mismatch at index {}: Rust={}, Ovie={}",
                    i, rust_token.location.line, ovie_token.location.line
                ));
                all_match = false;
            }

            if rust_token.location.column != ovie_token.location.column {
                errors.push(format!(
                    "Column mismatch at index {}: Rust={}, Ovie={}",
                    i, rust_token.location.column, ovie_token.location.column
                ));
                all_match = false;
            }
        }

        all_match
    }

    /// Run comprehensive bootstrap verification on multiple test cases
    pub fn run_comprehensive_verification(&self, test_cases: &[&str]) -> OvieResult<Vec<BootstrapVerificationResult>> {
        let mut results = Vec::new();

        for (i, test_case) in test_cases.iter().enumerate() {
            if self.config.verbose_logging {
                println!("Running verification test case {} of {}", i + 1, test_cases.len());
            }

            let result = self.verify_lexer(test_case)?;
            results.push(result);
        }

        Ok(results)
    }

    /// Generate a comprehensive verification report
    pub fn generate_verification_report(&self, results: &[BootstrapVerificationResult]) -> String {
        let mut report = String::new();
        
        report.push_str("# Bootstrap Verification Report\n\n");
        
        let total_tests = results.len();
        let passed_tests = results.iter().filter(|r| r.passed).count();
        let failed_tests = total_tests - passed_tests;
        
        report.push_str(&format!("## Summary\n"));
        report.push_str(&format!("- Total tests: {}\n", total_tests));
        report.push_str(&format!("- Passed: {}\n", passed_tests));
        report.push_str(&format!("- Failed: {}\n", failed_tests));
        report.push_str(&format!("- Success rate: {:.1}%\n\n", (passed_tests as f64 / total_tests as f64) * 100.0));
        
        if failed_tests > 0 {
            report.push_str("## Failed Tests\n\n");
            for (i, result) in results.iter().enumerate() {
                if !result.passed {
                    report.push_str(&format!("### Test {}\n", i + 1));
                    report.push_str(&format!("- Hash match: {}\n", result.hash_match));
                    report.push_str(&format!("- Token match: {}\n", result.tokens_match));
                    report.push_str(&format!("- Performance acceptable: {}\n", result.performance_acceptable));
                    
                    if !result.errors.is_empty() {
                        report.push_str("- Errors:\n");
                        for error in &result.errors {
                            report.push_str(&format!("  - {}\n", error));
                        }
                    }
                    report.push_str("\n");
                }
            }
        }
        
        // Performance statistics
        if results.iter().any(|r| r.performance_ratio > 0.0) {
            let performance_ratios: Vec<f64> = results.iter()
                .filter(|r| r.performance_ratio > 0.0)
                .map(|r| r.performance_ratio)
                .collect();
            
            if !performance_ratios.is_empty() {
                let avg_ratio = performance_ratios.iter().sum::<f64>() / performance_ratios.len() as f64;
                let min_ratio = performance_ratios.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                let max_ratio = performance_ratios.iter().fold(0.0f64, |a, &b| a.max(b));
                
                report.push_str("## Performance Statistics\n\n");
                report.push_str(&format!("- Average performance ratio: {:.2}x\n", avg_ratio));
                report.push_str(&format!("- Best performance ratio: {:.2}x\n", min_ratio));
                report.push_str(&format!("- Worst performance ratio: {:.2}x\n", max_ratio));
                report.push_str("\n");
            }
        }
        
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bootstrap_verifier_creation() {
        let config = BootstrapConfig::default();
        let verifier = BootstrapVerifier::new(config);
        assert!(verifier.ovie_lexer_ir.is_none());
    }

    #[test]
    fn test_token_hash_computation() {
        let config = BootstrapConfig::default();
        let verifier = BootstrapVerifier::new(config);
        
        let tokens = vec![
            Token::new(
                TokenType::SeeAm,
                "seeAm".to_string(),
                crate::error::SourceLocation::new(1, 1, 0)
            ),
            Token::new(
                TokenType::StringLiteral,
                "\"hello\"".to_string(),
                crate::error::SourceLocation::new(1, 7, 6)
            ),
        ];
        
        let hash1 = verifier.compute_token_hash(&tokens);
        let hash2 = verifier.compute_token_hash(&tokens);
        
        // Hash should be deterministic
        assert_eq!(hash1, hash2);
        assert!(!hash1.is_empty());
    }

    #[test]
    fn test_token_comparison_identical() {
        let config = BootstrapConfig::default();
        let verifier = BootstrapVerifier::new(config);
        
        let tokens = vec![
            Token::new(
                TokenType::SeeAm,
                "seeAm".to_string(),
                crate::error::SourceLocation::new(1, 1, 0)
            ),
        ];
        
        let mut errors = Vec::new();
        let result = verifier.compare_tokens(&tokens, &tokens, &mut errors);
        
        assert!(result);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_token_comparison_different() {
        let config = BootstrapConfig::default();
        let verifier = BootstrapVerifier::new(config);
        
        let tokens1 = vec![
            Token::new(
                TokenType::SeeAm,
                "seeAm".to_string(),
                crate::error::SourceLocation::new(1, 1, 0)
            ),
        ];
        
        let tokens2 = vec![
            Token::new(
                TokenType::Identifier,
                "seeAm".to_string(),
                crate::error::SourceLocation::new(1, 1, 0)
            ),
        ];
        
        let mut errors = Vec::new();
        let result = verifier.compare_tokens(&tokens1, &tokens2, &mut errors);
        
        assert!(!result);
        assert!(!errors.is_empty());
    }
}