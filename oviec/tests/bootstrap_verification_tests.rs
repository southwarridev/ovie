//! Bootstrap Verification Tests
//!
//! These tests verify the bootstrap verification infrastructure works correctly.
//! Currently uses Rust lexer for both sides as a proof-of-concept until the
//! Ovie-in-Ovie compiler is fully functional.

use oviec::self_hosting::bootstrap_verification::{
    BootstrapConfig, BootstrapVerifier, BootstrapVerificationResult,
};
use oviec::error::OvieResult;

#[test]
fn test_bootstrap_verifier_creation() {
    let config = BootstrapConfig::default();
    let verifier = BootstrapVerifier::new(config);
    
    // Verifier should be created successfully
    assert!(verifier.ovie_lexer_ir.is_none()); // Not loaded yet
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
fn test_hash_determinism() {
    let config = BootstrapConfig::default();
    let verifier = BootstrapVerifier::new(config);
    
    let tokens = vec![
        oviec::lexer::Token::new(
            oviec::lexer::TokenType::SeeAm,
            "seeAm".to_string(),
            oviec::error::SourceLocation::new(1, 1, 0),
        ),
    ];
    
    let hash1 = verifier.compute_token_hash(&tokens);
    let hash2 = verifier.compute_token_hash(&tokens);
    
    // Hashes should be identical for same input
    assert_eq!(hash1, hash2);
    assert!(!hash1.is_empty());
}

#[test]
fn test_simple_verification() -> OvieResult<()> {
    let config = BootstrapConfig {
        hash_verification: true,
        token_comparison: true,
        performance_benchmarking: false,
        reproducible_builds: false,
        ..Default::default()
    };
    
    let verifier = BootstrapVerifier::new(config);
    
    // Simple test case
    let source = "seeAm \"hello\";";
    
    // Run verification
    let result = verifier.verify_lexer(source)?;
    
    // Verification should pass
    assert!(result.passed);
    assert!(result.hash_match);
    assert!(result.tokens_match);
    assert!(result.token_count > 0);
    
    Ok(())
}

#[test]
fn test_token_mismatch_detection() {
    let config = BootstrapConfig::default();
    let verifier = BootstrapVerifier::new(config);
    
    let tokens1 = vec![
        oviec::lexer::Token::new(
            oviec::lexer::TokenType::SeeAm,
            "seeAm".to_string(),
            oviec::error::SourceLocation::new(1, 1, 0),
        ),
    ];
    
    let tokens2 = vec![
        oviec::lexer::Token::new(
            oviec::lexer::TokenType::Identifier,
            "seeAm".to_string(),
            oviec::error::SourceLocation::new(1, 1, 0),
        ),
    ];
    
    let mut errors: Vec<String> = Vec::new();
    let result = verifier.compare_tokens(&tokens1, &tokens2, &mut errors);
    
    // Should detect mismatch
    assert!(!result);
    assert!(!errors.is_empty());
    assert!(errors[0].contains("Token type mismatch"));
}

#[test]
fn test_verification_report_generation() -> OvieResult<()> {
    let config = BootstrapConfig::default();
    let verifier = BootstrapVerifier::new(config);
    
    // Create test results
    let results = vec![
        BootstrapVerificationResult {
            passed: true,
            hash_match: true,
            tokens_match: true,
            performance_acceptable: true,
            reproducible: true,
            rust_time_us: 100,
            ovie_time_us: 150,
            performance_ratio: 1.5,
            token_count: 10,
            source_hash: "abc123".to_string(),
            rust_tokens_hash: "def456".to_string(),
            ovie_tokens_hash: "def456".to_string(),
            reproducibility_hashes: vec!["def456".to_string()],
            errors: Vec::new(),
            timestamp: 1234567890,
            environment_hash: "env123".to_string(),
        },
        BootstrapVerificationResult {
            passed: false,
            hash_match: false,
            tokens_match: false,
            performance_acceptable: true,
            reproducible: true,
            rust_time_us: 100,
            ovie_time_us: 200,
            performance_ratio: 2.0,
            token_count: 5,
            source_hash: "xyz789".to_string(),
            rust_tokens_hash: "aaa111".to_string(),
            ovie_tokens_hash: "bbb222".to_string(),
            reproducibility_hashes: vec!["bbb222".to_string()],
            errors: vec!["Hash mismatch".to_string()],
            timestamp: 1234567891,
            environment_hash: "env123".to_string(),
        },
    ];
    
    let report = verifier.generate_verification_report(&results);
    
    // Report should contain key information
    assert!(report.contains("Bootstrap Verification Report"));
    assert!(report.contains("Total tests: 2"));
    assert!(report.contains("Passed: 1"));
    assert!(report.contains("Failed: 1"));
    assert!(report.contains("Success rate: 50.0%"));
    
    Ok(())
}

#[test]
fn test_comprehensive_verification() -> OvieResult<()> {
    let config = BootstrapConfig {
        performance_benchmarking: false,
        reproducible_builds: false,
        ..Default::default()
    };
    
    let verifier = BootstrapVerifier::new(config);
    
    let test_cases = vec![
        "seeAm \"test1\";",
        "let x = 42;",
        "fn main() { }",
    ];
    
    let results = verifier.run_comprehensive_verification(&test_cases)?;
    
    // Should have results for all test cases
    assert_eq!(results.len(), 3);
    
    // All should pass
    for result in &results {
        assert!(result.passed, "Test failed with errors: {:?}", result.errors);
    }
    
    Ok(())
}

#[test]
fn test_rollback_state_management() -> OvieResult<()> {
    let mut config = BootstrapConfig::default();
    config.rollback_enabled = true;
    config.work_dir = std::path::PathBuf::from("target/test_bootstrap_rollback");
    
    let mut verifier = BootstrapVerifier::new(config);
    
    // Save rollback state
    verifier.save_rollback_state()?;
    
    // Verify rollback state was saved
    assert!(verifier.rollback_state.is_some());
    
    // Verify rollback file exists
    let rollback_file = std::path::PathBuf::from("target/test_bootstrap_rollback/rollback_state.json");
    assert!(rollback_file.exists());
    
    // Clean up
    let _ = std::fs::remove_dir_all("target/test_bootstrap_rollback");
    
    Ok(())
}

#[test]
fn test_equivalence_tester() -> OvieResult<()> {
    let mut config = BootstrapConfig::default();
    config.performance_benchmarking = false;
    config.reproducible_builds = false;
    
    let mut verifier = BootstrapVerifier::new(config);
    verifier.initialize_equivalence_testing(5, 1);
    
    // Run automated equivalence testing
    let results = verifier.run_automated_equivalence_testing()?;
    
    // Should generate test results
    assert_eq!(results.len(), 5);
    
    Ok(())
}

#[test]
fn test_performance_benchmarking() -> OvieResult<()> {
    let config = BootstrapConfig {
        performance_benchmarking: true,
        reproducible_builds: false,
        ..Default::default()
    };
    
    let verifier = BootstrapVerifier::new(config);
    
    let source = "let x = 42; seeAm x;";
    let result = verifier.verify_lexer(source)?;
    
    // Should have performance data
    assert!(result.rust_time_us > 0);
    assert!(result.ovie_time_us > 0);
    assert!(result.performance_ratio > 0.0);
    assert!(result.performance_acceptable);
    
    Ok(())
}

#[test]
fn test_reproducibility_verification() -> OvieResult<()> {
    let config = BootstrapConfig {
        performance_benchmarking: false,
        reproducible_builds: true,
        reproducibility_iterations: 3,
        ..Default::default()
    };
    
    let verifier = BootstrapVerifier::new(config);
    
    let source = "seeAm \"reproducibility test\";";
    let result = verifier.verify_lexer(source)?;
    
    // Should have reproducibility data
    assert!(result.reproducible);
    assert_eq!(result.reproducibility_hashes.len(), 3);
    
    // All hashes should be identical
    let first_hash = &result.reproducibility_hashes[0];
    for hash in &result.reproducibility_hashes[1..] {
        assert_eq!(hash, first_hash);
    }
    
    Ok(())
}

#[test]
fn test_multiple_source_files() -> OvieResult<()> {
    let config = BootstrapConfig {
        performance_benchmarking: false,
        reproducible_builds: false,
        ..Default::default()
    };
    
    let verifier = BootstrapVerifier::new(config);
    
    let test_cases = vec![
        "// Comment test\nseeAm \"hello\";",
        "let x = 42;\nlet y = 100;\nseeAm x + y;",
        "fn add(a, b) { return a + b; }",
        "if true { seeAm \"yes\"; } else { seeAm \"no\"; }",
        "for i in 0..10 { seeAm i; }",
    ];
    
    let results = verifier.run_comprehensive_verification(&test_cases)?;
    
    assert_eq!(results.len(), 5);
    
    // All should pass
    let passed = results.iter().filter(|r| r.passed).count();
    assert_eq!(passed, 5, "Expected all tests to pass, but {} failed", 5 - passed);
    
    Ok(())
}
