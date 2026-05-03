//! Real Bootstrap Verification System for Ovie Self-Hosting
//! 
//! This module provides REAL verification that compares the Rust lexer output
//! to the Ovie lexer output, enabling genuine self-hosting verification.
//! 
//! Unlike the previous fake implementation, this actually:
//! 1. Runs the Rust lexer on source code
//! 2. Executes the Ovie lexer (written in Ovie) on the same source code
//! 3. Compares the token streams for exact equivalence
//! 4. Provides detailed reporting on any differences

use crate::error::{OvieError, OvieResult};
use crate::lexer::{Lexer as RustLexer, Token, TokenType};
use crate::interpreter::Interpreter;
use crate::ast::AstNode;
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use serde::{Deserialize, Serialize};

/// Real bootstrap verification configuration
#[derive(Debug, Clone)]
pub struct RealBootstrapConfig {
    /// Enable detailed token comparison
    pub detailed_comparison: bool,
    /// Enable performance benchmarking
    pub performance_benchmarking: bool,
    /// Maximum allowed performance degradation (as multiplier)
    pub max_performance_degradation: f64,
    /// Enable verbose logging
    pub verbose_logging: bool,
    /// Working directory for verification artifacts
    pub work_dir: PathBuf,
    /// Path to the Ovie lexer source file
    pub ovie_lexer_path: PathBuf,
}

impl Default for RealBootstrapConfig {
    fn default() -> Self {
        // Get the workspace root directory
        let workspace_root = std::env::var("CARGO_MANIFEST_DIR")
            .map(|p| std::path::PathBuf::from(p).parent().unwrap().to_path_buf())
            .unwrap_or_else(|_| std::path::PathBuf::from("."));
        
        Self {
            detailed_comparison: true,
            performance_benchmarking: true,
            max_performance_degradation: 10.0, // 10x slower is acceptable for initial self-hosting
            verbose_logging: false,
            work_dir: workspace_root.join("target/real_bootstrap_verification"),
            // Use simplified lexer without type annotations until parser supports them
            ovie_lexer_path: workspace_root.join("std/lexer/mod_no_types.ov"),
        }
    }
}

/// Real bootstrap verification results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealBootstrapResult {
    /// Whether verification passed
    pub passed: bool,
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
    /// Timestamp of verification
    pub timestamp: u64,
    /// Test case identifier
    pub test_case_id: String,
}

/// Real bootstrap verification system
pub struct RealBootstrapVerifier {
    config: RealBootstrapConfig,
    ovie_lexer_source: Option<String>,
    interpreter: Interpreter,
}

impl RealBootstrapVerifier {
    /// Create a new real bootstrap verifier
    pub fn new(config: RealBootstrapConfig) -> Self {
        // Ensure work directory exists
        if let Err(e) = fs::create_dir_all(&config.work_dir) {
            eprintln!("Warning: Failed to create work directory: {}", e);
        }

        Self {
            config,
            ovie_lexer_source: None,
            interpreter: Interpreter::new(),
        }
    }
    
    /// Check if Ovie lexer source is loaded
    pub fn is_ovie_lexer_loaded(&self) -> bool {
        self.ovie_lexer_source.is_some()
    }

    /// Load the Ovie lexer source code
    pub fn load_ovie_lexer(&mut self) -> OvieResult<()> {
        if self.config.verbose_logging {
            println!("Loading Ovie lexer from: {:?}", self.config.ovie_lexer_path);
        }

        let source = fs::read_to_string(&self.config.ovie_lexer_path)
            .map_err(|e| OvieError::runtime_error(format!("Failed to read Ovie lexer source: {}", e)))?;

        self.ovie_lexer_source = Some(source);

        if self.config.verbose_logging {
            println!("Ovie lexer source loaded successfully ({} characters)", 
                    self.ovie_lexer_source.as_ref().unwrap().len());
        }

        Ok(())
    }

    /// Verify that the Ovie lexer produces identical results to the Rust lexer
    pub fn verify_lexer(&mut self, source_code: &str, test_case_id: &str) -> OvieResult<RealBootstrapResult> {
        if self.config.verbose_logging {
            println!("Starting real bootstrap verification for test case: {}", test_case_id);
            println!("Source code length: {} characters", source_code.len());
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let mut result = RealBootstrapResult {
            passed: false,
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
            timestamp,
            test_case_id: test_case_id.to_string(),
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

        result.token_count = rust_tokens.len();

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

        // Hash verification
        result.rust_tokens_hash = self.compute_token_hash(&rust_tokens);
        result.ovie_tokens_hash = self.compute_token_hash(&ovie_tokens);

        // Token comparison
        result.tokens_match = self.compare_tokens(&rust_tokens, &ovie_tokens, &mut result.errors);

        if self.config.verbose_logging {
            println!("Token comparison: {}", if result.tokens_match { "PASS" } else { "FAIL" });
            println!("  Rust tokens: {} (hash: {})", rust_tokens.len(), &result.rust_tokens_hash[..8]);
            println!("  Ovie tokens: {} (hash: {})", ovie_tokens.len(), &result.ovie_tokens_hash[..8]);
        }

        // Performance verification
        if self.config.performance_benchmarking && result.rust_time_us > 0 {
            result.performance_ratio = result.ovie_time_us as f64 / result.rust_time_us as f64;
            result.performance_acceptable = result.performance_ratio <= self.config.max_performance_degradation;

            if self.config.verbose_logging {
                println!("Performance comparison: {}", if result.performance_acceptable { "PASS" } else { "FAIL" });
                println!("  Rust time: {} μs", result.rust_time_us);
                println!("  Ovie time: {} μs", result.ovie_time_us);
                println!("  Ratio: {:.2}x", result.performance_ratio);
            }
        } else {
            result.performance_acceptable = true; // Skip if no timing data
        }

        // Overall result
        result.passed = result.tokens_match && result.performance_acceptable;

        if self.config.verbose_logging {
            println!("Real bootstrap verification: {}", if result.passed { "PASS" } else { "FAIL" });
        }

        Ok(result)
    }

    /// Run the Rust lexer and measure performance
    fn run_rust_lexer(&self, source_code: &str) -> OvieResult<(Vec<Token>, u64)> {
        let start = Instant::now();
        
        let mut lexer = RustLexer::new(source_code);
        let tokens = lexer.tokenize()?;
        
        let elapsed = start.elapsed();
        let time_us = elapsed.as_micros() as u64;
        
        Ok((tokens, time_us))
    }

    /// Run the Ovie lexer and measure performance
    fn run_ovie_lexer(&mut self, source_code: &str) -> OvieResult<(Vec<Token>, u64)> {
        let ovie_source = self.ovie_lexer_source.as_ref()
            .ok_or_else(|| OvieError::runtime_error("Ovie lexer source not loaded".to_string()))?;

        let start = Instant::now();
        
        // Compile the Ovie lexer source
        let mut compiler = crate::Compiler::new();
        let ast = compiler.compile_to_ast(ovie_source)?;
        
        // Execute the Ovie lexer via interpreter
        self.interpreter.interpret(&ast)?;
        
        // Call the ovie_tokenize function with the source code
        let tokenize_call = crate::ast::Expression::Call {
            function: "ovie_tokenize".to_string(),
            arguments: vec![crate::ast::Expression::Literal(crate::ast::Literal::String(source_code.to_string()))],
        };
        
        let result = self.interpreter.evaluate_expression(&tokenize_call)?;
        
        // Convert the result to Rust tokens
        let tokens = self.convert_ovie_result_to_tokens(result)?;
        
        let elapsed = start.elapsed();
        let time_us = elapsed.as_micros() as u64;
        
        Ok((tokens, time_us))
    }

    /// Convert Ovie execution result to Rust tokens
    fn convert_ovie_result_to_tokens(&self, result: crate::interpreter::Value) -> OvieResult<Vec<Token>> {
        use crate::interpreter::Value;
        
        // Result should be an array of Token structs
        match result {
            Value::Array(token_values) => {
                let mut tokens = Vec::new();
                
                for token_value in token_values {
                    match token_value {
                        Value::Struct(fields) => {
                            // Extract token fields
                            let token_type_value = fields.get("token_type")
                                .ok_or_else(|| OvieError::runtime_error("Token missing token_type field".to_string()))?;
                            
                            let lexeme = fields.get("lexeme")
                                .and_then(|v| match v {
                                    Value::String(s) => Some(s.clone()),
                                    _ => None,
                                })
                                .ok_or_else(|| OvieError::runtime_error("Token missing or invalid lexeme field".to_string()))?;
                            
                            let line = fields.get("line")
                                .and_then(|v| match v {
                                    Value::Number(n) => Some(*n as usize),
                                    _ => None,
                                })
                                .ok_or_else(|| OvieError::runtime_error("Token missing or invalid line field".to_string()))?;
                            
                            let column = fields.get("column")
                                .and_then(|v| match v {
                                    Value::Number(n) => Some(*n as usize),
                                    _ => None,
                                })
                                .ok_or_else(|| OvieError::runtime_error("Token missing or invalid column field".to_string()))?;
                            
                            // Convert Ovie TokenType enum to Rust TokenType
                            let token_type = self.convert_token_type(token_type_value)?;
                            
                            // Create Rust Token
                            let location = crate::error::SourceLocation::new(line, column, 0);
                            tokens.push(Token::new(token_type, lexeme, location));
                        }
                        _ => {
                            return Err(OvieError::runtime_error("Expected Token struct in array".to_string()));
                        }
                    }
                }
                
                Ok(tokens)
            }
            _ => {
                Err(OvieError::runtime_error("Expected array of tokens from ovie_tokenize".to_string()))
            }
        }
    }
    
    /// Convert Ovie TokenType enum to Rust TokenType
    fn convert_token_type(&self, token_type_value: &crate::interpreter::Value) -> OvieResult<TokenType> {
        use crate::interpreter::Value;
        
        match token_type_value {
            Value::Enum { variant, data } => {
                match variant.as_str() {
                    "Fn" => Ok(TokenType::Fn),
                    "Mut" => Ok(TokenType::Mut),
                    "Let" => Ok(TokenType::Let),
                    "If" => Ok(TokenType::If),
                    "Else" => Ok(TokenType::Else),
                    "For" => Ok(TokenType::For),
                    "While" => Ok(TokenType::While),
                    "Struct" => Ok(TokenType::Struct),
                    "Enum" => Ok(TokenType::Enum),
                    "Unsafe" => Ok(TokenType::Unsafe),
                    "Return" => Ok(TokenType::Return),
                    "True" => Ok(TokenType::True),
                    "False" => Ok(TokenType::False),
                    "SeeAm" => Ok(TokenType::SeeAm),
                    "In" => Ok(TokenType::In),
                    "Identifier" => Ok(TokenType::Identifier),
                    "StringLiteral" => Ok(TokenType::StringLiteral),
                    "IntegerLiteral" => Ok(TokenType::IntegerLiteral),
                    "FloatLiteral" => Ok(TokenType::FloatLiteral),
                    "Plus" => Ok(TokenType::Plus),
                    "Minus" => Ok(TokenType::Minus),
                    "Star" => Ok(TokenType::Star),
                    "Slash" => Ok(TokenType::Slash),
                    "Percent" => Ok(TokenType::Percent),
                    "EqualEqual" => Ok(TokenType::EqualEqual),
                    "NotEqual" => Ok(TokenType::NotEqual),
                    "Less" => Ok(TokenType::Less),
                    "LessEqual" => Ok(TokenType::LessEqual),
                    "Greater" => Ok(TokenType::Greater),
                    "GreaterEqual" => Ok(TokenType::GreaterEqual),
                    "AndAnd" => Ok(TokenType::AndAnd),
                    "OrOr" => Ok(TokenType::OrOr),
                    "Bang" => Ok(TokenType::Bang),
                    "Equal" => Ok(TokenType::Equal),
                    "LeftParen" => Ok(TokenType::LeftParen),
                    "RightParen" => Ok(TokenType::RightParen),
                    "LeftBrace" => Ok(TokenType::LeftBrace),
                    "RightBrace" => Ok(TokenType::RightBrace),
                    "LeftBracket" => Ok(TokenType::LeftBracket),
                    "RightBracket" => Ok(TokenType::RightBracket),
                    "Comma" => Ok(TokenType::Comma),
                    "Semicolon" => Ok(TokenType::Semicolon),
                    "Colon" => Ok(TokenType::Colon),
                    "Dot" => Ok(TokenType::Dot),
                    "DotDot" => Ok(TokenType::DotDot),
                    "Eof" => Ok(TokenType::Eof),
                    "Error" => Ok(TokenType::Error),
                    _ => Err(OvieError::runtime_error(format!("Unknown token type variant: {}", variant))),
                }
            }
            _ => Err(OvieError::runtime_error("Expected TokenType enum value".to_string())),
        }
    }

    /// Compute a hash of the token stream for verification
    pub fn compute_token_hash(&self, tokens: &[Token]) -> String {
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
    pub fn compare_tokens(&self, rust_tokens: &[Token], ovie_tokens: &[Token], errors: &mut Vec<String>) -> bool {
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

            // Stop after first few errors to avoid spam
            if errors.len() > 10 {
                errors.push("... (truncated additional errors)".to_string());
                break;
            }
        }

        all_match
    }

    /// Run comprehensive verification on multiple test cases
    pub fn run_comprehensive_verification(&mut self, test_cases: &[(&str, &str)]) -> OvieResult<Vec<RealBootstrapResult>> {
        let mut results = Vec::new();

        for (i, (test_case, test_id)) in test_cases.iter().enumerate() {
            if self.config.verbose_logging {
                println!("Running verification test case {} of {}: {}", i + 1, test_cases.len(), test_id);
            }

            let result = self.verify_lexer(test_case, test_id)?;
            results.push(result);
        }

        Ok(results)
    }

    /// Generate a comprehensive verification report
    pub fn generate_verification_report(&self, results: &[RealBootstrapResult]) -> String {
        let mut report = String::new();
        
        report.push_str("# Real Bootstrap Verification Report\n\n");
        
        let total_tests = results.len();
        let passed_tests = results.iter().filter(|r| r.passed).count();
        let failed_tests = total_tests - passed_tests;
        
        report.push_str(&format!("## Summary\n"));
        report.push_str(&format!("- Total tests: {}\n", total_tests));
        report.push_str(&format!("- Passed: {}\n", passed_tests));
        report.push_str(&format!("- Failed: {}\n", failed_tests));
        report.push_str(&format!("- Success rate: {:.1}%\n\n", (passed_tests as f64 / total_tests as f64) * 100.0));
        
        // Component breakdown
        let token_matches = results.iter().filter(|r| r.tokens_match).count();
        let performance_acceptable = results.iter().filter(|r| r.performance_acceptable).count();
        
        report.push_str("## Verification Component Breakdown\n");
        report.push_str(&format!("- Token comparison: {}/{} ({:.1}%)\n", token_matches, total_tests, (token_matches as f64 / total_tests as f64) * 100.0));
        report.push_str(&format!("- Performance acceptable: {}/{} ({:.1}%)\n\n", performance_acceptable, total_tests, (performance_acceptable as f64 / total_tests as f64) * 100.0));
        
        if failed_tests > 0 {
            report.push_str("## Failed Tests\n\n");
            for result in results.iter().filter(|r| !r.passed) {
                report.push_str(&format!("### Test: {}\n", result.test_case_id));
                report.push_str(&format!("- Token match: {}\n", result.tokens_match));
                report.push_str(&format!("- Performance acceptable: {}\n", result.performance_acceptable));
                report.push_str(&format!("- Token count: {}\n", result.token_count));
                report.push_str(&format!("- Performance ratio: {:.2}x\n", result.performance_ratio));
                
                if !result.errors.is_empty() {
                    report.push_str("- Errors:\n");
                    for error in &result.errors {
                        report.push_str(&format!("  - {}\n", error));
                    }
                }
                report.push_str("\n");
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
            }
        }
        
        report.push_str("\n## Conclusion\n\n");
        if passed_tests == total_tests {
            report.push_str("🎉 **REAL SELF-HOSTING ACHIEVED!** 🎉\n\n");
            report.push_str("The Ovie lexer written in Ovie produces identical output to the Rust lexer.\n");
            report.push_str("This is genuine self-hosting - Ovie code is being used to process Ovie code.\n");
        } else {
            report.push_str("❌ **Self-hosting not yet achieved**\n\n");
            report.push_str("The Ovie lexer does not yet produce identical output to the Rust lexer.\n");
            report.push_str("Further development is needed to achieve real self-hosting.\n");
        }
        
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_bootstrap_verifier_creation() {
        let config = RealBootstrapConfig::default();
        let verifier = RealBootstrapVerifier::new(config);
        assert!(verifier.ovie_lexer_source.is_none());
    }

    #[test]
    fn test_real_bootstrap_config_defaults() {
        let config = RealBootstrapConfig::default();
        assert!(config.detailed_comparison);
        assert!(config.performance_benchmarking);
        assert_eq!(config.max_performance_degradation, 10.0);
        assert_eq!(config.ovie_lexer_path, PathBuf::from("std/lexer/mod.ov"));
    }

    #[test]
    fn test_token_hash_computation() {
        let config = RealBootstrapConfig::default();
        let verifier = RealBootstrapVerifier::new(config);
        
        let tokens = vec![
            Token::new(
                TokenType::SeeAm,
                "seeAm".to_string(),
                crate::error::SourceLocation::new(1, 1, 0)
            ),
        ];
        
        let hash1 = verifier.compute_token_hash(&tokens);
        let hash2 = verifier.compute_token_hash(&tokens);
        
        // Hash should be deterministic
        assert_eq!(hash1, hash2);
        assert!(!hash1.is_empty());
    }
}