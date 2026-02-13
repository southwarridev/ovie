//! Comprehensive Bootstrap Verification Tests
//! 
//! This test suite validates the bootstrap verification infrastructure
//! to ensure it's ready for actual bootstrap execution once the Ovie
//! compiler is functional.

use oviec::self_hosting::bootstrap_verification::{
    BootstrapConfig, BootstrapVerifier, BootstrapVerificationResult,
};

#[test]
fn test_bootstrap_verifier_creation() {
    let config = BootstrapConfig::default();
    let verifier = BootstrapVerifier::new(config);
    
    // Verifier should be created successfully
    assert!(verifier.ovie_lexer_ir.is_none()); // Not loaded yet
    assert!(verifier.rollback_state.is_none()); // Not saved yet
}

#[test]
fn test_bootstrap_config_defaults() {
    let config = BootstrapConfig::default();
    
    // Verify default configuration
    assert!(config.hash_verification);
    assert!(config.token_comparison);
    assert!(config.performance_benchmarking);
    assert!(config.rollback_enabled);
    assert!(config.reproducible_builds);
    assert_eq!(config.max_performance_degradation, 5.0);
    assert_eq!(config.reproducibility_iterations, 3);
}

#[test]
fn test_bootstrap_verifier_with_simple_source() {
    let config = BootstrapConfig::default();
    let verifier = BootstrapVerifier::new(config);
    
    // Test with simple source code
    let source = "seeAm \"hello\";";
    let result = verifier.verify_lexer(source).unwrap();
    
    // Should pass with Rust lexer on both sides (placeholder mode)
    assert!(result.passed, "Verification should pass");
    assert!(result.hash_match, "Hashes should match");
    assert!(result.tokens_match, "Tokens should match");
    assert!(result.performance_acceptable, "Performance should be acceptable");
    assert!(result.reproducible, "Should be reproducible");
    
    // Check that we got tokens
    assert!(result.token_count > 0, "Should have tokens");
    
    // Check that hashes are not empty
    assert!(!result.source_hash.is_empty(), "Source hash should not be empty");
    assert!(!result.rust_tokens_hash.is_empty(), "Rust tokens hash should not be empty");
    assert!(!result.ovie_tokens_hash.is_empty(), "Ovie tokens hash should not be empty");
}

#[test]
fn test_bootstrap_verifier_with_multiple_sources() {
    let config = BootstrapConfig::default();
    let verifier = BootstrapVerifier::new(config);
    
    let test_cases = vec![
        "seeAm 42;",
        "mut x = 10;",
        "fn main() { return 0; }",
        "if true { seeAm \"yes\"; }",
    ];
    
    let results = verifier.run_comprehensive_verification(&test_cases).unwrap();
    
    assert_eq!(results.len(), test_cases.len());
    
    // All should pass in placeholder mode
    for result in &results {
        assert!(result.passed, "All verifications should pass");
    }
}

#[test]
fn test_bootstrap_reproducibility() {
    let config = BootstrapConfig::default();
    let verifier = BootstrapVerifier::new(config);
    
    let test_cases = vec!["seeAm 42;", "mut x = 10;"];
    let reproducible = verifier.verify_bootstrap_reproducibility(&test_cases).unwrap();
    
    assert!(reproducible, "Bootstrap should be reproducible");
}

#[test]
fn test_bootstrap_report_generation() {
    let config = BootstrapConfig::default();
    let verifier = BootstrapVerifier::new(config);
    
    let source = "seeAm \"test\";";
    let result = verifier.verify_lexer(source).unwrap();
    
    let report = verifier.generate_verification_report(&vec![result]);
    
    // Verify report structure
    assert!(report.contains("Bootstrap Verification Report"));
    assert!(report.contains("Summary"));
    assert!(report.contains("Total tests:"));
    assert!(report.contains("Passed:"));
    assert!(report.contains("Verification Component Breakdown"));
    assert!(report.contains("Hash verification:"));
    assert!(report.contains("Token comparison:"));
    assert!(report.contains("Performance acceptable:"));
    assert!(report.contains("Reproducible builds:"));
}

#[test]
fn test_bootstrap_report_with_multiple_results() {
    let config = BootstrapConfig::default();
    let verifier = BootstrapVerifier::new(config);
    
    let test_cases = vec![
        "seeAm 1;",
        "seeAm 2;",
        "seeAm 3;",
    ];
    
    let results = verifier.run_comprehensive_verification(&test_cases).unwrap();
    let report = verifier.generate_verification_report(&results);
    
    // Verify report contains correct counts
    assert!(report.contains("Total tests: 3"));
    assert!(report.contains("Passed: 3"));
    assert!(report.contains("Success rate: 100.0%"));
}

#[test]
fn test_rollback_state_save() {
    let config = BootstrapConfig::default();
    let mut verifier = BootstrapVerifier::new(config);
    
    // Save rollback state
    verifier.save_rollback_state().unwrap();
    
    // Verify state was saved
    assert!(verifier.rollback_state.is_some());
    
    let state = verifier.rollback_state.as_ref().unwrap();
    assert!(state.timestamp > 0);
    assert!(!state.work_dir_hash.is_empty());
}

#[test]
fn test_rollback_state_save_and_restore() {
    let config = BootstrapConfig::default();
    let mut verifier = BootstrapVerifier::new(config);
    
    // Save state
    verifier.save_rollback_state().unwrap();
    assert!(verifier.rollback_state.is_some());
    
    let saved_timestamp = verifier.rollback_state.as_ref().unwrap().timestamp;
    
    // Restore state
    verifier.restore_rollback_state().unwrap();
    
    // Verify state was restored
    assert!(verifier.rollback_state.is_some());
    let restored_timestamp = verifier.rollback_state.as_ref().unwrap().timestamp;
    assert_eq!(saved_timestamp, restored_timestamp);
}

#[test]
fn test_automated_equivalence_testing() {
    let config = BootstrapConfig::default();
    let mut verifier = BootstrapVerifier::new(config);
    
    // Initialize equivalence testing with 10 test cases
    verifier.initialize_equivalence_testing(10, 1);
    
    // Run automated tests
    let results = verifier.run_automated_equivalence_testing().unwrap();
    
    // Should generate 10 test cases
    assert_eq!(results.len(), 10);
    
    // All should pass in placeholder mode
    for result in &results {
        assert!(result.passed, "Automated test should pass");
    }
}

#[test]
fn test_token_hash_determinism() {
    let config = BootstrapConfig::default();
    let verifier = BootstrapVerifier::new(config);
    
    let source = "seeAm \"hello\";";
    
    // Run verification multiple times
    let result1 = verifier.verify_lexer(source).unwrap();
    let result2 = verifier.verify_lexer(source).unwrap();
    let result3 = verifier.verify_lexer(source).unwrap();
    
    // Hashes should be identical
    assert_eq!(result1.rust_tokens_hash, result2.rust_tokens_hash);
    assert_eq!(result2.rust_tokens_hash, result3.rust_tokens_hash);
    assert_eq!(result1.ovie_tokens_hash, result2.ovie_tokens_hash);
    assert_eq!(result2.ovie_tokens_hash, result3.ovie_tokens_hash);
}

#[test]
fn test_performance_measurement() {
    let config = BootstrapConfig::default();
    let verifier = BootstrapVerifier::new(config);
    
    let source = "seeAm \"performance test\";";
    let result = verifier.verify_lexer(source).unwrap();
    
    // Performance metrics should be recorded
    assert!(result.rust_time_us > 0, "Rust time should be measured");
    assert!(result.ovie_time_us > 0, "Ovie time should be measured");
    assert!(result.performance_ratio > 0.0, "Performance ratio should be calculated");
    assert!(result.performance_acceptable, "Performance should be acceptable");
}

#[test]
fn test_reproducibility_hashes() {
    let config = BootstrapConfig::default();
    let verifier = BootstrapVerifier::new(config);
    
    let source = "seeAm \"reproducibility\";";
    let result = verifier.verify_lexer(source).unwrap();
    
    // Should have reproducibility hashes
    assert!(!result.reproducibility_hashes.is_empty(), "Should have reproducibility hashes");
    assert_eq!(result.reproducibility_hashes.len(), config.reproducibility_iterations);
    
    // All hashes should be identical
    let first_hash = &result.reproducibility_hashes[0];
    for hash in &result.reproducibility_hashes[1..] {
        assert_eq!(hash, first_hash, "All reproducibility hashes should match");
    }
}

#[test]
fn test_environment_hash_generation() {
    let config = BootstrapConfig::default();
    let verifier = BootstrapVerifier::new(config);
    
    let source = "seeAm \"env test\";";
    let result = verifier.verify_lexer(source).unwrap();
    
    // Environment hash should be generated
    assert!(!result.environment_hash.is_empty(), "Environment hash should not be empty");
    
    // Should be deterministic within same environment
    let result2 = verifier.verify_lexer(source).unwrap();
    assert_eq!(result.environment_hash, result2.environment_hash);
}

#[test]
fn test_verification_result_serialization() {
    let config = BootstrapConfig::default();
    let verifier = BootstrapVerifier::new(config);
    
    let source = "seeAm \"serialization test\";";
    let result = verifier.verify_lexer(source).unwrap();
    
    // Serialize to JSON
    let json = serde_json::to_string(&result).unwrap();
    assert!(!json.is_empty());
    
    // Deserialize back
    let deserialized: BootstrapVerificationResult = serde_json::from_str(&json).unwrap();
    
    // Verify key fields match
    assert_eq!(result.passed, deserialized.passed);
    assert_eq!(result.hash_match, deserialized.hash_match);
    assert_eq!(result.tokens_match, deserialized.tokens_match);
    assert_eq!(result.source_hash, deserialized.source_hash);
}

#[test]
fn test_comprehensive_verification_workflow() {
    // This test simulates the complete bootstrap verification workflow
    
    // Step 1: Create verifier with custom config
    let mut config = BootstrapConfig::default();
    config.verbose_logging = false; // Disable for test
    config.reproducibility_iterations = 2; // Reduce for speed
    
    let mut verifier = BootstrapVerifier::new(config);
    
    // Step 2: Save rollback state
    verifier.save_rollback_state().unwrap();
    
    // Step 3: Run verification on multiple test cases
    let test_cases = vec![
        "seeAm \"test 1\";",
        "mut x = 42;",
        "fn main() { return 0; }",
    ];
    
    let results = verifier.run_comprehensive_verification(&test_cases).unwrap();
    
    // Step 4: Verify all passed
    assert_eq!(results.len(), 3);
    for result in &results {
        assert!(result.passed);
    }
    
    // Step 5: Generate report
    let report = verifier.generate_verification_report(&results);
    assert!(report.contains("Total tests: 3"));
    assert!(report.contains("Passed: 3"));
    
    // Step 6: Verify reproducibility
    let reproducible = verifier.verify_bootstrap_reproducibility(&test_cases).unwrap();
    assert!(reproducible);
    
    // Step 7: Test rollback capability
    verifier.restore_rollback_state().unwrap();
}

#[test]
fn test_bootstrap_with_complex_source() {
    let config = BootstrapConfig::default();
    let verifier = BootstrapVerifier::new(config);
    
    // Test with more complex source code
    let source = r#"
        fn factorial(n) {
            if n <= 1 {
                return 1;
            }
            return n * factorial(n - 1);
        }
        
        mut result = factorial(5);
        seeAm result;
    "#;
    
    let result = verifier.verify_lexer(source).unwrap();
    
    assert!(result.passed);
    assert!(result.token_count > 10); // Should have many tokens
}

#[test]
fn test_bootstrap_infrastructure_ready() {
    // This test verifies that all bootstrap infrastructure is ready
    
    let config = BootstrapConfig::default();
    let mut verifier = BootstrapVerifier::new(config);
    
    // Test 1: Verifier creation
    assert!(verifier.ovie_lexer_ir.is_none()); // Not loaded yet (expected)
    
    // Test 2: Rollback capability
    verifier.save_rollback_state().unwrap();
    assert!(verifier.rollback_state.is_some());
    
    // Test 3: Verification execution
    let result = verifier.verify_lexer("seeAm 1;").unwrap();
    assert!(result.passed);
    
    // Test 4: Report generation
    let report = verifier.generate_verification_report(&vec![result]);
    assert!(!report.is_empty());
    
    // Test 5: Reproducibility
    let reproducible = verifier.verify_bootstrap_reproducibility(&vec!["seeAm 1;"]).unwrap();
    assert!(reproducible);
    
    // Test 6: Automated testing
    verifier.initialize_equivalence_testing(5, 1);
    let auto_results = verifier.run_automated_equivalence_testing().unwrap();
    assert_eq!(auto_results.len(), 5);
    
    println!("\n✓ All bootstrap infrastructure tests passed!");
    println!("✓ Bootstrap verification system is ready");
    println!("✓ Waiting for Ovie compiler to be functional");
}
