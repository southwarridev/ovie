//! Tests for the self-hosting system

use super::*;
use crate::error::OvieResult;

/// Test cases for bootstrap verification
const BOOTSTRAP_TEST_CASES: &[&str] = &[
    // Simple hello world
    r#"seeAm "Hello, World!";"#,
    
    // Variable assignment
    r#"
    name = "Ovie";
    mut counter = 42;
    "#,
    
    // Function definition
    r#"
    fn greet(person) {
        seeAm "Hello, " + person + "!";
    }
    "#,
    
    // Control flow
    r#"
    if counter < 10 {
        seeAm "Small";
    } else {
        seeAm "Big";
    }
    "#,
    
    // Loops
    r#"
    for i in 0..10 {
        seeAm i;
    }
    
    while counter > 0 {
        counter = counter - 1;
    }
    "#,
    
    // Structs and enums
    r#"
    struct Person {
        name: String,
        age: Number,
    }
    
    enum Color {
        Red,
        Green,
        Blue,
    }
    "#,
    
    // Complex expressions
    r#"
    result = (a + b) * (c - d) / e % f;
    condition = x > y && z <= w || !flag;
    "#,
    
    // String literals with escapes
    r#"
    message = "Hello \"world\" with\nnewlines and\ttabs";
    "#,
    
    // Numbers
    r#"
    integer = 42;
    float = 3.14159;
    zero = 0;
    large = 1234567890;
    "#,
    
    // Comments and whitespace
    r#"
    // This is a comment
    value = 123; // End of line comment
    
    /* Multi-line comments would go here if supported */
    "#,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bootstrap_verification_basic() -> OvieResult<()> {
        let config = BootstrapConfig {
            hash_verification: true,
            token_comparison: true,
            performance_benchmarking: false, // Skip for unit tests
            max_performance_degradation: 10.0,
            verbose_logging: false,
        };
        
        let mut verifier = BootstrapVerifier::new(config);
        
        // For now, we can't actually load the Ovie lexer since it's not implemented
        // This test verifies the structure works
        
        // Test simple source
        let source = r#"seeAm "Hello, World!";"#;
        
        // This will fail because we haven't loaded the Ovie lexer, but that's expected
        let result = verifier.verify_lexer(source);
        
        // The verification should fail gracefully
        assert!(result.is_ok());
        let verification_result = result.unwrap();
        assert!(!verification_result.passed); // Should fail without Ovie lexer loaded
        
        Ok(())
    }

    #[test]
    fn test_self_hosting_manager_initialization() -> OvieResult<()> {
        let mut manager = SelfHostingManager::new();
        assert_eq!(manager.current_stage(), SelfHostingStage::Stage0);
        
        // Test bootstrap verification initialization
        let config = BootstrapConfig::default();
        
        // This will fail because the Ovie lexer source isn't a valid Ovie program yet
        // But it tests the initialization path
        let result = manager.initialize_bootstrap_verification(config);
        
        // Should fail gracefully
        assert!(result.is_err());
        
        Ok(())
    }

    #[test]
    fn test_comprehensive_verification_structure() -> OvieResult<()> {
        let config = BootstrapConfig {
            hash_verification: true,
            token_comparison: true,
            performance_benchmarking: false,
            max_performance_degradation: 5.0,
            verbose_logging: false,
        };
        
        let verifier = BootstrapVerifier::new(config);
        
        // Test that we can run verification on multiple test cases
        // This will fail because the Ovie lexer isn't loaded, but tests the structure
        let results = verifier.run_comprehensive_verification(&BOOTSTRAP_TEST_CASES[0..3]);
        
        assert!(results.is_ok());
        let verification_results = results.unwrap();
        assert_eq!(verification_results.len(), 3);
        
        // All should fail without Ovie lexer loaded
        for result in &verification_results {
            assert!(!result.passed);
        }
        
        Ok(())
    }

    #[test]
    fn test_verification_report_generation() {
        let config = BootstrapConfig::default();
        let verifier = BootstrapVerifier::new(config);
        
        // Create some mock results
        let results = vec![
            BootstrapVerificationResult {
                passed: true,
                hash_match: true,
                tokens_match: true,
                performance_acceptable: true,
                rust_time_us: 100,
                ovie_time_us: 200,
                performance_ratio: 2.0,
                token_count: 10,
                source_hash: "abc123".to_string(),
                rust_tokens_hash: "def456".to_string(),
                ovie_tokens_hash: "def456".to_string(),
                errors: vec![],
            },
            BootstrapVerificationResult {
                passed: false,
                hash_match: false,
                tokens_match: false,
                performance_acceptable: true,
                rust_time_us: 150,
                ovie_time_us: 300,
                performance_ratio: 2.0,
                token_count: 15,
                source_hash: "ghi789".to_string(),
                rust_tokens_hash: "jkl012".to_string(),
                ovie_tokens_hash: "mno345".to_string(),
                errors: vec!["Token mismatch".to_string()],
            },
        ];
        
        let report = verifier.generate_verification_report(&results);
        
        assert!(report.contains("Bootstrap Verification Report"));
        assert!(report.contains("Total tests: 2"));
        assert!(report.contains("Passed: 1"));
        assert!(report.contains("Failed: 1"));
        assert!(report.contains("Success rate: 50.0%"));
        assert!(report.contains("Performance Statistics"));
        assert!(report.contains("Average performance ratio: 2.00x"));
    }

    #[test]
    fn test_self_hosting_status_report() {
        let manager = SelfHostingManager::new();
        let report = manager.generate_status_report();
        
        assert!(report.contains("Ovie Self-Hosting Status Report"));
        assert!(report.contains("Stage 0 (Rust Bootstrap)"));
        assert!(report.contains("Rust lexer implementation complete"));
        assert!(report.contains("Next Steps for Stage 1 Transition"));
        assert!(report.contains("Bootstrap verifier not initialized"));
    }

    #[test]
    fn test_stage_progression_names() {
        assert_eq!(SelfHostingStage::Stage0.name(), "Stage 0 (Rust Bootstrap)");
        assert_eq!(SelfHostingStage::Stage1.name(), "Stage 1 (Partial Self-Hosting)");
        assert_eq!(SelfHostingStage::Stage2.name(), "Stage 2 (Full Self-Hosting)");
    }

    #[test]
    fn test_stage_descriptions() {
        assert!(SelfHostingStage::Stage0.description().contains("Rust implementation"));
        assert!(SelfHostingStage::Stage1.description().contains("Lexer and parser"));
        assert!(SelfHostingStage::Stage2.description().contains("Complete compiler"));
    }
}

/// Property-based tests for bootstrap verification
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    /// **Property 10: Bootstrap Verification**
    /// **Validates: Requirements 5.4**
    /// 
    /// This property ensures that the bootstrap verification system correctly
    /// identifies when Rust and Ovie lexers produce identical results.
    mod bootstrap_verification_properties {
        use super::*;

        // Generator for valid Ovie source code
        fn valid_ovie_source() -> impl Strategy<Value = String> {
            prop_oneof![
                // Simple print statements
                prop::string::string_regex(r#"seeAm "[^"]*";"#).unwrap(),
                
                // Variable assignments
                prop::string::string_regex(r"[a-zA-Z_][a-zA-Z0-9_]* = [0-9]+;").unwrap(),
                
                // Function calls
                prop::string::string_regex(r"[a-zA-Z_][a-zA-Z0-9_]*\(\);").unwrap(),
                
                // Simple expressions
                prop::string::string_regex(r"[0-9]+ \+ [0-9]+;").unwrap(),
            ]
        }

        proptest! {
            #[test]
            fn prop_bootstrap_verification_deterministic(source in valid_ovie_source()) {
                let config = BootstrapConfig {
                    hash_verification: true,
                    token_comparison: true,
                    performance_benchmarking: false,
                    max_performance_degradation: 10.0,
                    verbose_logging: false,
                };
                
                let verifier = BootstrapVerifier::new(config);
                
                // Run verification twice on the same source
                let result1 = verifier.verify_lexer(&source);
                let result2 = verifier.verify_lexer(&source);
                
                // Both should succeed (or fail) consistently
                prop_assert_eq!(result1.is_ok(), result2.is_ok());
                
                if let (Ok(r1), Ok(r2)) = (result1, result2) {
                    // Results should be identical for the same input
                    prop_assert_eq!(r1.source_hash, r2.source_hash);
                    prop_assert_eq!(r1.rust_tokens_hash, r2.rust_tokens_hash);
                    // Note: ovie_tokens_hash might differ if Ovie lexer isn't loaded
                }
            }

            #[test]
            fn prop_verification_result_consistency(source in valid_ovie_source()) {
                let config = BootstrapConfig::default();
                let verifier = BootstrapVerifier::new(config);
                
                if let Ok(result) = verifier.verify_lexer(&source) {
                    // If hash matches, tokens should match
                    if result.hash_match {
                        prop_assert!(result.tokens_match);
                    }
                    
                    // If both hash and tokens match, and performance is acceptable,
                    // overall result should pass
                    if result.hash_match && result.tokens_match && result.performance_acceptable {
                        prop_assert!(result.passed);
                    }
                    
                    // Source hash should be consistent
                    prop_assert!(!result.source_hash.is_empty());
                    prop_assert_eq!(result.source_hash.len(), 64); // SHA-256 hex length
                }
            }

            #[test]
            fn prop_comprehensive_verification_scales(
                test_cases in prop::collection::vec(valid_ovie_source(), 1..10)
            ) {
                let config = BootstrapConfig {
                    hash_verification: true,
                    token_comparison: true,
                    performance_benchmarking: false,
                    max_performance_degradation: 5.0,
                    verbose_logging: false,
                };
                
                let verifier = BootstrapVerifier::new(config);
                
                let test_case_refs: Vec<&str> = test_cases.iter().map(|s| s.as_str()).collect();
                let results = verifier.run_comprehensive_verification(&test_case_refs);
                
                prop_assert!(results.is_ok());
                
                if let Ok(verification_results) = results {
                    // Should have one result per test case
                    prop_assert_eq!(verification_results.len(), test_cases.len());
                    
                    // Each result should have a valid source hash
                    for result in &verification_results {
                        prop_assert!(!result.source_hash.is_empty());
                        prop_assert_eq!(result.source_hash.len(), 64);
                    }
                }
            }

            #[test]
            fn prop_verification_report_completeness(
                results in prop::collection::vec(
                    (any::<bool>(), any::<bool>(), any::<bool>(), 1u64..1000u64, 1u64..5000u64),
                    1..20
                )
            ) {
                let config = BootstrapConfig::default();
                let verifier = BootstrapVerifier::new(config);
                
                // Convert tuples to BootstrapVerificationResult
                let verification_results: Vec<BootstrapVerificationResult> = results
                    .into_iter()
                    .enumerate()
                    .map(|(i, (hash_match, tokens_match, perf_acceptable, rust_time, ovie_time))| {
                        BootstrapVerificationResult {
                            passed: hash_match && tokens_match && perf_acceptable,
                            hash_match,
                            tokens_match,
                            performance_acceptable: perf_acceptable,
                            rust_time_us: rust_time,
                            ovie_time_us: ovie_time,
                            performance_ratio: ovie_time as f64 / rust_time as f64,
                            token_count: i * 10,
                            source_hash: format!("source_{:x}", i),
                            rust_tokens_hash: format!("rust_{:x}", i),
                            ovie_tokens_hash: if hash_match { 
                                format!("rust_{:x}", i) 
                            } else { 
                                format!("ovie_{:x}", i) 
                            },
                            errors: if tokens_match { vec![] } else { vec!["Mismatch".to_string()] },
                        }
                    })
                    .collect();
                
                let report = verifier.generate_verification_report(&verification_results);
                
                // Report should contain key sections
                prop_assert!(report.contains("Bootstrap Verification Report"));
                prop_assert!(report.contains("Summary"));
                prop_assert!(report.contains("Total tests:"));
                prop_assert!(report.contains("Success rate:"));
                
                // If there are performance results, should include performance stats
                if verification_results.iter().any(|r| r.performance_ratio > 0.0) {
                    prop_assert!(report.contains("Performance Statistics"));
                }
                
                // If there are failures, should include failed tests section
                if verification_results.iter().any(|r| !r.passed) {
                    prop_assert!(report.contains("Failed Tests"));
                }
            }
        }
    }
}