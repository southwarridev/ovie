// Bootstrap Verification Tests
//
// This module contains tests for the bootstrap verification system.
// These tests verify that the Ovie-in-Ovie compiler produces identical
// results to the Rust compiler, enabling safe self-hosting.

// Include comprehensive test suite
mod comprehensive_tests;

// Include script integration tests
mod script_integration_tests;

#[cfg(test)]
mod bootstrap_tests {
    use crate::self_hosting::bootstrap_verification::{
        BootstrapConfig, BootstrapVerifier, BootstrapVerificationResult,
    };
    use crate::error::OvieResult;

    /// Test: Bootstrap verifier can be created with default config
    #[test]
    fn test_bootstrap_verifier_creation() {
        let config = BootstrapConfig::default();
        let verifier = BootstrapVerifier::new(config);
        
        // Verifier should be created successfully
        assert!(verifier.ovie_lexer_ir.is_none()); // Not loaded yet
    }

    /// Test: Bootstrap verifier can save and restore rollback state
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
        
        Ok(())
    }

    /// Test: Bootstrap verifier computes deterministic hashes
    #[test]
    fn test_hash_determinism() {
        let config = BootstrapConfig::default();
        let verifier = BootstrapVerifier::new(config);
        
        let tokens = vec![
            crate::lexer::Token::new(
                crate::lexer::TokenType::SeeAm,
                "seeAm".to_string(),
                crate::error::SourceLocation::new(1, 1, 0),
            ),
        ];
        
        let hash1 = verifier.compute_token_hash(&tokens);
        let hash2 = verifier.compute_token_hash(&tokens);
        
        // Hashes should be identical for same input
        assert_eq!(hash1, hash2);
        assert!(!hash1.is_empty());
    }

    /// Test: Bootstrap verifier can run verification on simple source
    #[test]
    fn test_simple_verification() -> OvieResult<()> {
        let config = BootstrapConfig {
            hash_verification: true,
            token_comparison: true,
            performance_benchmarking: false, // Skip for simple test
            reproducible_builds: false,      // Skip for simple test
            ..Default::default()
        };
        
        let verifier = BootstrapVerifier::new(config);
        
        // Simple test case
        let source = "seeAm \"hello\";";
        
        // Run verification (will use Rust lexer for both sides currently)
        let result = verifier.verify_lexer(source)?;
        
        // Verification should pass (both sides use Rust lexer)
        assert!(result.passed);
        assert!(result.hash_match);
        assert!(result.tokens_match);
        
        Ok(())
    }

    /// Test: Bootstrap verifier detects token mismatches
    #[test]
    fn test_token_mismatch_detection() {
        let config = BootstrapConfig::default();
        let verifier = BootstrapVerifier::new(config);
        
        let tokens1 = vec![
            crate::lexer::Token::new(
                crate::lexer::TokenType::SeeAm,
                "seeAm".to_string(),
                crate::error::SourceLocation::new(1, 1, 0),
            ),
        ];
        
        let tokens2 = vec![
            crate::lexer::Token::new(
                crate::lexer::TokenType::Identifier,
                "seeAm".to_string(),
                crate::error::SourceLocation::new(1, 1, 0),
            ),
        ];
        
        let mut errors = Vec::new();
        let result = verifier.compare_tokens(&tokens1, &tokens2, &mut errors);
        
        // Should detect mismatch
        assert!(!result);
        assert!(!errors.is_empty());
        assert!(errors[0].contains("Token type mismatch"));
    }

    /// Test: Bootstrap verifier generates comprehensive reports
    #[test]
    fn test_verification_report_generation() -> OvieResult<()> {
        let config = BootstrapConfig::default();
        let verifier = BootstrapVerifier::new(config);
        
        // Create some test results
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

    /// Test: Bootstrap verifier handles multiple test cases
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
        
        // All should pass (using Rust lexer for both sides)
        for result in &results {
            assert!(result.passed);
        }
        
        Ok(())
    }

    /// Test: Equivalence tester generates valid test cases
    #[test]
    fn test_equivalence_tester() {
        let mut config = BootstrapConfig::default();
        config.performance_benchmarking = false;
        config.reproducible_builds = false;
        
        let mut verifier = BootstrapVerifier::new(config);
        verifier.initialize_equivalence_testing(5, 1);
        
        // Run automated equivalence testing
        let results = verifier.run_automated_equivalence_testing();
        
        // Should generate test results
        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 5);
    }
}
