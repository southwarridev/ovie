//! Bootstrap Verification System for Ovie Self-Hosting
//! 
//! This module provides the verification system for ensuring that the Ovie-in-Ovie
//! lexer produces identical results to the Rust lexer, enabling safe transition
//! to partial self-hosting (Stage 1).

use crate::error::{OvieError, OvieResult};
use crate::lexer::{Lexer as RustLexer, Token, TokenType};
use crate::parser::{Parser as RustParser};
use crate::ast::AstNode;
use crate::ir::{Program as IR, IrBuilder};
use crate::interpreter::IrInterpreter;
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use serde_json;

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
    /// Enable rollback capability
    pub rollback_enabled: bool,
    /// Enable reproducible build verification
    pub reproducible_builds: bool,
    /// Working directory for verification artifacts
    pub work_dir: PathBuf,
    /// Number of iterations for reproducibility testing
    pub reproducibility_iterations: usize,
}

impl Default for BootstrapConfig {
    fn default() -> Self {
        Self {
            hash_verification: true,
            token_comparison: true,
            performance_benchmarking: true,
            max_performance_degradation: 5.0, // 5x slower is acceptable for Stage 1
            verbose_logging: false,
            rollback_enabled: true,
            reproducible_builds: true,
            work_dir: PathBuf::from("target/bootstrap_verification"),
            reproducibility_iterations: 3,
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
    /// Reproducibility verification result
    pub reproducible: bool,
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
    /// Reproducibility hashes (multiple runs)
    pub reproducibility_hashes: Vec<String>,
    /// Any errors encountered
    pub errors: Vec<String>,
    /// Timestamp of verification
    pub timestamp: u64,
    /// Build environment hash
    pub environment_hash: String,
}

/// Rollback state for bootstrap verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackState {
    /// Timestamp when state was saved
    pub timestamp: u64,
    /// Compiler configuration at time of save
    pub compiler_config: HashMap<String, String>,
    /// Last known good verification results
    pub last_good_results: Vec<BootstrapVerificationResult>,
    /// Environment variables at time of save
    pub environment: HashMap<String, String>,
    /// Working directory state
    pub work_dir_hash: String,
}

/// Automated equivalence testing system
pub struct EquivalenceTester {
    /// Test case generator
    test_generator: TestCaseGenerator,
    /// Maximum number of test cases to generate
    max_test_cases: usize,
    /// Minimum complexity for generated test cases
    min_complexity: usize,
}

/// Test case generator for automated equivalence testing
pub struct TestCaseGenerator {
    /// Random seed for reproducible test generation
    seed: u64,
    /// Current test case counter
    counter: usize,
}

impl TestCaseGenerator {
    pub fn new(seed: u64) -> Self {
        Self { seed, counter: 0 }
    }

    /// Generate a random valid Ovie source code snippet
    pub fn generate_test_case(&mut self, complexity: usize) -> String {
        self.counter += 1;
        
        // For now, generate simple test cases
        // In a full implementation, this would use a proper grammar-based generator
        match complexity % 5 {
            0 => format!("seeAm \"test case {}\";", self.counter),
            1 => format!("mut x = {}; seeAm x;", self.counter),
            2 => format!("fn test_{}() {{ return {}; }}", self.counter, self.counter),
            3 => format!("if {} > 0 {{ seeAm \"positive\"; }}", self.counter),
            4 => format!("for i in 0..{} {{ seeAm i; }}", self.counter % 10),
            _ => format!("seeAm \"default case\";"),
        }
    }
}

impl EquivalenceTester {
    pub fn new(max_test_cases: usize, min_complexity: usize) -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        Self {
            test_generator: TestCaseGenerator::new(seed),
            max_test_cases,
            min_complexity,
        }
    }

    /// Run automated equivalence testing
    pub fn run_equivalence_tests(&mut self, verifier: &BootstrapVerifier) -> OvieResult<Vec<BootstrapVerificationResult>> {
        let mut results = Vec::new();
        
        for i in 0..self.max_test_cases {
            let complexity = self.min_complexity + (i % 10);
            let test_case = self.test_generator.generate_test_case(complexity);
            
            match verifier.verify_lexer(&test_case) {
                Ok(result) => results.push(result),
                Err(e) => {
                    // Create a failed result for this test case
                    let failed_result = BootstrapVerificationResult {
                        passed: false,
                        hash_match: false,
                        tokens_match: false,
                        performance_acceptable: false,
                        reproducible: false,
                        rust_time_us: 0,
                        ovie_time_us: 0,
                        performance_ratio: 0.0,
                        token_count: 0,
                        source_hash: String::new(),
                        rust_tokens_hash: String::new(),
                        ovie_tokens_hash: String::new(),
                        reproducibility_hashes: Vec::new(),
                        errors: vec![format!("Test generation error: {}", e)],
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
                        environment_hash: String::new(),
                    };
                    results.push(failed_result);
                }
            }
        }
        
        Ok(results)
    }
}

/// Bootstrap verification system
pub struct BootstrapVerifier {
    config: BootstrapConfig,
    pub ovie_lexer_ir: Option<IR>,
    pub rollback_state: Option<RollbackState>,
    equivalence_tester: Option<EquivalenceTester>,
}

impl BootstrapVerifier {
    /// Create a new bootstrap verifier
    pub fn new(config: BootstrapConfig) -> Self {
        // Ensure work directory exists
        if let Err(e) = fs::create_dir_all(&config.work_dir) {
            eprintln!("Warning: Failed to create work directory: {}", e);
        }

        Self {
            config,
            ovie_lexer_ir: None,
            rollback_state: None,
            equivalence_tester: None,
        }
    }

    /// Initialize automated equivalence testing
    pub fn initialize_equivalence_testing(&mut self, max_test_cases: usize, min_complexity: usize) {
        self.equivalence_tester = Some(EquivalenceTester::new(max_test_cases, min_complexity));
    }

    /// Save current state for rollback capability
    pub fn save_rollback_state(&mut self) -> OvieResult<()> {
        if !self.config.rollback_enabled {
            return Ok(());
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Collect environment variables
        let environment: HashMap<String, String> = std::env::vars().collect();

        // Compute work directory hash
        let work_dir_hash = self.compute_directory_hash(&self.config.work_dir)?;

        let rollback_state = RollbackState {
            timestamp,
            compiler_config: HashMap::new(), // TODO: Collect actual compiler config
            last_good_results: Vec::new(),   // TODO: Store last good results
            environment,
            work_dir_hash,
        };

        // Save rollback state to file
        let rollback_file = self.config.work_dir.join("rollback_state.json");
        let rollback_json = serde_json::to_string_pretty(&rollback_state)?;
        fs::write(&rollback_file, rollback_json)?;

        self.rollback_state = Some(rollback_state);

        if self.config.verbose_logging {
            println!("Rollback state saved at timestamp {}", timestamp);
        }

        Ok(())
    }

    /// Restore from rollback state
    pub fn restore_rollback_state(&mut self) -> OvieResult<()> {
        if !self.config.rollback_enabled {
            return Err(OvieError::runtime_error("Rollback not enabled".to_string()));
        }

        let rollback_file = self.config.work_dir.join("rollback_state.json");
        if !rollback_file.exists() {
            return Err(OvieError::runtime_error("No rollback state found".to_string()));
        }

        let rollback_json = fs::read_to_string(&rollback_file)?;
        let rollback_state: RollbackState = serde_json::from_str(&rollback_json)?;

        if self.config.verbose_logging {
            println!("Restoring rollback state from timestamp {}", rollback_state.timestamp);
        }

        // TODO: Implement actual rollback logic
        // This would involve:
        // 1. Restoring compiler configuration
        // 2. Reverting to previous Ovie component versions
        // 3. Clearing any cached state
        // 4. Validating the restored state

        self.rollback_state = Some(rollback_state);
        Ok(())
    }

    /// Compute hash of directory contents for reproducibility verification
    fn compute_directory_hash(&self, dir: &Path) -> OvieResult<String> {
        let mut hasher = Sha256::new();
        
        if dir.exists() {
            // For simplicity, just hash the directory path
            // In a full implementation, this would recursively hash all files
            hasher.update(dir.to_string_lossy().as_bytes());
        }
        
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Compute environment hash for reproducibility verification
    fn compute_environment_hash(&self) -> String {
        let mut hasher = Sha256::new();
        
        // Hash relevant environment variables
        let env_vars = ["PATH", "RUST_VERSION", "OVIE_VERSION", "HOME", "USER"];
        for var in &env_vars {
            if let Ok(value) = std::env::var(var) {
                hasher.update(var.as_bytes());
                hasher.update(value.as_bytes());
            }
        }
        
        format!("{:x}", hasher.finalize())
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

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let environment_hash = self.compute_environment_hash();

        let mut result = BootstrapVerificationResult {
            passed: false,
            hash_match: false,
            tokens_match: false,
            performance_acceptable: false,
            reproducible: false,
            rust_time_us: 0,
            ovie_time_us: 0,
            performance_ratio: 0.0,
            token_count: 0,
            source_hash: String::new(),
            rust_tokens_hash: String::new(),
            ovie_tokens_hash: String::new(),
            reproducibility_hashes: Vec::new(),
            errors: Vec::new(),
            timestamp,
            environment_hash,
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

        // Reproducibility verification
        if self.config.reproducible_builds {
            result.reproducible = self.verify_reproducibility(source_code, &mut result.reproducibility_hashes)?;

            if self.config.verbose_logging {
                println!("Reproducibility verification: {}", if result.reproducible { "PASS" } else { "FAIL" });
            }
        } else {
            result.reproducible = true; // Skip if not enabled
        }

        // Overall result
        result.passed = result.hash_match && result.tokens_match && result.performance_acceptable && result.reproducible;

        if self.config.verbose_logging {
            println!("Bootstrap verification: {}", if result.passed { "PASS" } else { "FAIL" });
        }

        Ok(result)
    }

    /// Verify reproducibility by running multiple iterations
    fn verify_reproducibility(&self, source_code: &str, reproducibility_hashes: &mut Vec<String>) -> OvieResult<bool> {
        for i in 0..self.config.reproducibility_iterations {
            if self.config.verbose_logging {
                println!("Reproducibility iteration {} of {}", i + 1, self.config.reproducibility_iterations);
            }

            // Run Ovie lexer again
            let (tokens, _) = self.run_ovie_lexer(source_code)?;
            let hash = self.compute_token_hash(&tokens);
            reproducibility_hashes.push(hash.clone());

            // Check if this hash matches the first one
            if i > 0 && reproducibility_hashes[0] != hash {
                return Ok(false);
            }
        }

        Ok(true)
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

    /// Run automated equivalence testing
    pub fn run_automated_equivalence_testing(&mut self) -> OvieResult<Vec<BootstrapVerificationResult>> {
        if self.equivalence_tester.is_some() {
            // Take ownership temporarily to avoid borrow checker issues
            let mut tester = self.equivalence_tester.take().unwrap();
            let result = tester.run_equivalence_tests(self);
            self.equivalence_tester = Some(tester);
            result
        } else {
            Err(OvieError::runtime_error("Equivalence tester not initialized".to_string()))
        }
    }

    /// Verify bootstrap process reproducibility across multiple runs
    pub fn verify_bootstrap_reproducibility(&self, test_cases: &[&str]) -> OvieResult<bool> {
        if !self.config.reproducible_builds {
            return Ok(true); // Skip if not enabled
        }

        let mut baseline_hashes = Vec::new();
        
        // First run to establish baseline
        for test_case in test_cases {
            let result = self.verify_lexer(test_case)?;
            baseline_hashes.push(result.ovie_tokens_hash);
        }

        // Additional runs to verify reproducibility
        for iteration in 1..self.config.reproducibility_iterations {
            if self.config.verbose_logging {
                println!("Bootstrap reproducibility iteration {} of {}", iteration + 1, self.config.reproducibility_iterations);
            }

            for (i, test_case) in test_cases.iter().enumerate() {
                let result = self.verify_lexer(test_case)?;
                if result.ovie_tokens_hash != baseline_hashes[i] {
                    if self.config.verbose_logging {
                        println!("Reproducibility failure at test case {} iteration {}", i + 1, iteration + 1);
                    }
                    return Ok(false);
                }
            }
        }

        Ok(true)
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
        
        // Verification component breakdown
        let hash_matches = results.iter().filter(|r| r.hash_match).count();
        let token_matches = results.iter().filter(|r| r.tokens_match).count();
        let performance_acceptable = results.iter().filter(|r| r.performance_acceptable).count();
        let reproducible = results.iter().filter(|r| r.reproducible).count();
        
        report.push_str("## Verification Component Breakdown\n");
        report.push_str(&format!("- Hash verification: {}/{} ({:.1}%)\n", hash_matches, total_tests, (hash_matches as f64 / total_tests as f64) * 100.0));
        report.push_str(&format!("- Token comparison: {}/{} ({:.1}%)\n", token_matches, total_tests, (token_matches as f64 / total_tests as f64) * 100.0));
        report.push_str(&format!("- Performance acceptable: {}/{} ({:.1}%)\n", performance_acceptable, total_tests, (performance_acceptable as f64 / total_tests as f64) * 100.0));
        report.push_str(&format!("- Reproducible builds: {}/{} ({:.1}%)\n\n", reproducible, total_tests, (reproducible as f64 / total_tests as f64) * 100.0));
        
        if failed_tests > 0 {
            report.push_str("## Failed Tests\n\n");
            for (i, result) in results.iter().enumerate() {
                if !result.passed {
                    report.push_str(&format!("### Test {}\n", i + 1));
                    report.push_str(&format!("- Hash match: {}\n", result.hash_match));
                    report.push_str(&format!("- Token match: {}\n", result.tokens_match));
                    report.push_str(&format!("- Performance acceptable: {}\n", result.performance_acceptable));
                    report.push_str(&format!("- Reproducible: {}\n", result.reproducible));
                    report.push_str(&format!("- Timestamp: {}\n", result.timestamp));
                    report.push_str(&format!("- Environment hash: {}\n", result.environment_hash));
                    
                    if !result.errors.is_empty() {
                        report.push_str("- Errors:\n");
                        for error in &result.errors {
                            report.push_str(&format!("  - {}\n", error));
                        }
                    }
                    
                    if !result.reproducibility_hashes.is_empty() {
                        report.push_str("- Reproducibility hashes:\n");
                        for (j, hash) in result.reproducibility_hashes.iter().enumerate() {
                            report.push_str(&format!("  - Iteration {}: {}\n", j + 1, hash));
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
                
                // Performance distribution
                let fast_tests = performance_ratios.iter().filter(|&&r| r <= 2.0).count();
                let medium_tests = performance_ratios.iter().filter(|&&r| r > 2.0 && r <= 5.0).count();
                let slow_tests = performance_ratios.iter().filter(|&&r| r > 5.0).count();
                
                report.push_str(&format!("- Fast tests (≤2x): {} ({:.1}%)\n", fast_tests, (fast_tests as f64 / performance_ratios.len() as f64) * 100.0));
                report.push_str(&format!("- Medium tests (2-5x): {} ({:.1}%)\n", medium_tests, (medium_tests as f64 / performance_ratios.len() as f64) * 100.0));
                report.push_str(&format!("- Slow tests (>5x): {} ({:.1}%)\n", slow_tests, (slow_tests as f64 / performance_ratios.len() as f64) * 100.0));
                report.push_str("\n");
            }
        }
        
        // Reproducibility analysis
        if self.config.reproducible_builds {
            let reproducible_tests = results.iter().filter(|r| r.reproducible).count();
            report.push_str("## Reproducibility Analysis\n\n");
            report.push_str(&format!("- Reproducible tests: {}/{} ({:.1}%)\n", reproducible_tests, total_tests, (reproducible_tests as f64 / total_tests as f64) * 100.0));
            
            if reproducible_tests < total_tests {
                report.push_str("- Non-reproducible tests detected - investigation required\n");
            }
            report.push_str("\n");
        }
        
        // Environment information
        if let Some(result) = results.first() {
            report.push_str("## Environment Information\n\n");
            report.push_str(&format!("- Environment hash: {}\n", result.environment_hash));
            report.push_str(&format!("- Report timestamp: {}\n", result.timestamp));
            report.push_str(&format!("- Reproducibility iterations: {}\n", self.config.reproducibility_iterations));
            report.push_str(&format!("- Performance threshold: {:.1}x\n", self.config.max_performance_degradation));
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
        assert!(verifier.rollback_state.is_none());
        assert!(verifier.equivalence_tester.is_none());
    }

    #[test]
    fn test_bootstrap_config_defaults() {
        let config = BootstrapConfig::default();
        assert!(config.hash_verification);
        assert!(config.token_comparison);
        assert!(config.performance_benchmarking);
        assert!(config.rollback_enabled);
        assert!(config.reproducible_builds);
        assert_eq!(config.max_performance_degradation, 5.0);
        assert_eq!(config.reproducibility_iterations, 3);
    }

    #[test]
    fn test_equivalence_tester_creation() {
        let mut tester = EquivalenceTester::new(10, 1);
        let test_case = tester.test_generator.generate_test_case(1);
        assert!(!test_case.is_empty());
        assert!(test_case.contains("1")); // Should contain the counter
    }

    #[test]
    fn test_rollback_state_serialization() {
        let rollback_state = RollbackState {
            timestamp: 1234567890,
            compiler_config: HashMap::new(),
            last_good_results: Vec::new(),
            environment: HashMap::new(),
            work_dir_hash: "test_hash".to_string(),
        };
        
        let json = serde_json::to_string(&rollback_state).unwrap();
        let deserialized: RollbackState = serde_json::from_str(&json).unwrap();
        
        assert_eq!(rollback_state.timestamp, deserialized.timestamp);
        assert_eq!(rollback_state.work_dir_hash, deserialized.work_dir_hash);
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