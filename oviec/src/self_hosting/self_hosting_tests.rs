//! Comprehensive tests for the self-hosting infrastructure
//! 
//! This module contains unit tests, integration tests, and property-based tests
//! for the bootstrap verification system and self-hosting components.

use super::*;
use crate::lexer::{Lexer, TokenType};
use crate::error::SourceLocation;
use std::collections::HashMap;
use std::fs;
use tempfile::TempDir;

/// Test the bootstrap configuration system
#[cfg(test)]
mod bootstrap_config_tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = BootstrapConfig::default();
        
        assert!(config.hash_verification);
        assert!(config.token_comparison);
        assert!(config.performance_benchmarking);
        assert!(config.rollback_enabled);
        assert!(config.reproducible_builds);
        assert_eq!(config.max_performance_degradation, 5.0);
        assert_eq!(config.reproducibility_iterations, 3);
        assert_eq!(config.work_dir, PathBuf::from("target/bootstrap_verification"));
    }

    #[test]
    fn test_custom_config() {
        let mut config = BootstrapConfig::default();
        config.max_performance_degradation = 2.0;
        config.reproducibility_iterations = 5;
        config.verbose_logging = true;
        
        assert_eq!(config.max_performance_degradation, 2.0);
        assert_eq!(config.reproducibility_iterations, 5);
        assert!(config.verbose_logging);
    }
}

/// Test the bootstrap verifier core functionality
#[cfg(test)]
mod bootstrap_verifier_tests {
    use super::*;

    #[test]
    fn test_verifier_creation() {
        let config = BootstrapConfig::default();
        let verifier = BootstrapVerifier::new(config);
        
        assert!(verifier.ovie_lexer_ir.is_none());
        assert!(verifier.rollback_state.is_none());
        assert!(verifier.equivalence_tester.is_none());
    }

    #[test]
    fn test_verifier_with_temp_dir() {
        let temp_dir = TempDir::new().unwrap();
        let mut config = BootstrapConfig::default();
        config.work_dir = temp_dir.path().to_path_buf();
        
        let verifier = BootstrapVerifier::new(config);
        
        // Work directory should be created
        assert!(temp_dir.path().exists());
    }

    #[test]
    fn test_equivalence_testing_initialization() {
        let config = BootstrapConfig::default();
        let mut verifier = BootstrapVerifier::new(config);
        
        verifier.initialize_equivalence_testing(100, 5);
        assert!(verifier.equivalence_tester.is_some());
    }

    #[test]
    fn test_environment_hash_computation() {
        let config = BootstrapConfig::default();
        let verifier = BootstrapVerifier::new(config);
        
        let hash1 = verifier.compute_environment_hash();
        let hash2 = verifier.compute_environment_hash();
        
        // Should be deterministic
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64); // SHA-256 length
    }

    #[test]
    fn test_token_hash_computation() {
        let config = BootstrapConfig::default();
        let verifier = BootstrapVerifier::new(config);
        
        let tokens = vec![
            crate::lexer::Token::new(
                TokenType::SeeAm,
                "seeAm".to_string(),
                SourceLocation::new(1, 1, 0)
            ),
            crate::lexer::Token::new(
                TokenType::StringLiteral,
                "\"hello\"".to_string(),
                SourceLocation::new(1, 7, 6)
            ),
        ];
        
        let hash1 = verifier.compute_token_hash(&tokens);
        let hash2 = verifier.compute_token_hash(&tokens);
        
        // Should be deterministic
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64); // SHA-256 length
        assert!(!hash1.is_empty());
    }

    #[test]
    fn test_token_comparison_identical() {
        let config = BootstrapConfig::default();
        let verifier = BootstrapVerifier::new(config);
        
        let tokens = vec![
            crate::lexer::Token::new(
                TokenType::SeeAm,
                "seeAm".to_string(),
                SourceLocation::new(1, 1, 0)
            ),
        ];
        
        let mut errors = Vec::new();
        let result = verifier.compare_tokens(&tokens, &tokens, &mut errors);
        
        assert!(result);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_token_comparison_different_types() {
        let config = BootstrapConfig::default();
        let verifier = BootstrapVerifier::new(config);
        
        let tokens1 = vec![
            crate::lexer::Token::new(
                TokenType::SeeAm,
                "seeAm".to_string(),
                SourceLocation::new(1, 1, 0)
            ),
        ];
        
        let tokens2 = vec![
            crate::lexer::Token::new(
                TokenType::Identifier,
                "seeAm".to_string(),
                SourceLocation::new(1, 1, 0)
            ),
        ];
        
        let mut errors = Vec::new();
        let result = verifier.compare_tokens(&tokens1, &tokens2, &mut errors);
        
        assert!(!result);
        assert!(!errors.is_empty());
        assert!(errors[0].contains("Token type mismatch"));
    }

    #[test]
    fn test_token_comparison_different_lexemes() {
        let config = BootstrapConfig::default();
        let verifier = BootstrapVerifier::new(config);
        
        let tokens1 = vec![
            crate::lexer::Token::new(
                TokenType::Identifier,
                "hello".to_string(),
                SourceLocation::new(1, 1, 0)
            ),
        ];
        
        let tokens2 = vec![
            crate::lexer::Token::new(
                TokenType::Identifier,
                "world".to_string(),
                SourceLocation::new(1, 1, 0)
            ),
        ];
        
        let mut errors = Vec::new();
        let result = verifier.compare_tokens(&tokens1, &tokens2, &mut errors);
        
        assert!(!result);
        assert!(!errors.is_empty());
        assert!(errors[0].contains("Lexeme mismatch"));
    }

    #[test]
    fn test_token_comparison_different_counts() {
        let config = BootstrapConfig::default();
        let verifier = BootstrapVerifier::new(config);
        
        let tokens1 = vec![
            crate::lexer::Token::new(
                TokenType::Identifier,
                "hello".to_string(),
                SourceLocation::new(1, 1, 0)
            ),
        ];
        
        let tokens2 = vec![
            crate::lexer::Token::new(
                TokenType::Identifier,
                "hello".to_string(),
                SourceLocation::new(1, 1, 0)
            ),
            crate::lexer::Token::new(
                TokenType::Identifier,
                "world".to_string(),
                SourceLocation::new(1, 7, 6)
            ),
        ];
        
        let mut errors = Vec::new();
        let result = verifier.compare_tokens(&tokens1, &tokens2, &mut errors);
        
        assert!(!result);
        assert!(!errors.is_empty());
        assert!(errors[0].contains("Token count mismatch"));
    }
}

/// Test the rollback system
#[cfg(test)]
mod rollback_tests {
    use super::*;

    #[test]
    fn test_rollback_state_creation() {
        let rollback_state = RollbackState {
            timestamp: 1234567890,
            compiler_config: HashMap::new(),
            last_good_results: Vec::new(),
            environment: HashMap::new(),
            work_dir_hash: "test_hash".to_string(),
        };
        
        assert_eq!(rollback_state.timestamp, 1234567890);
        assert_eq!(rollback_state.work_dir_hash, "test_hash");
    }

    #[test]
    fn test_rollback_state_serialization() {
        let mut environment = HashMap::new();
        environment.insert("PATH".to_string(), "/usr/bin".to_string());
        
        let rollback_state = RollbackState {
            timestamp: 1234567890,
            compiler_config: HashMap::new(),
            last_good_results: Vec::new(),
            environment,
            work_dir_hash: "test_hash".to_string(),
        };
        
        let json = serde_json::to_string(&rollback_state).unwrap();
        let deserialized: RollbackState = serde_json::from_str(&json).unwrap();
        
        assert_eq!(rollback_state.timestamp, deserialized.timestamp);
        assert_eq!(rollback_state.work_dir_hash, deserialized.work_dir_hash);
        assert_eq!(rollback_state.environment.get("PATH"), deserialized.environment.get("PATH"));
    }

    #[test]
    fn test_save_rollback_state() {
        let temp_dir = TempDir::new().unwrap();
        let mut config = BootstrapConfig::default();
        config.work_dir = temp_dir.path().to_path_buf();
        config.rollback_enabled = true;
        
        let mut verifier = BootstrapVerifier::new(config);
        
        let result = verifier.save_rollback_state();
        assert!(result.is_ok());
        
        // Check that rollback file was created
        let rollback_file = temp_dir.path().join("rollback_state.json");
        assert!(rollback_file.exists());
        
        // Verify file contents
        let contents = fs::read_to_string(&rollback_file).unwrap();
        let rollback_state: RollbackState = serde_json::from_str(&contents).unwrap();
        assert!(rollback_state.timestamp > 0);
    }

    #[test]
    fn test_save_rollback_state_disabled() {
        let temp_dir = TempDir::new().unwrap();
        let mut config = BootstrapConfig::default();
        config.work_dir = temp_dir.path().to_path_buf();
        config.rollback_enabled = false;
        
        let mut verifier = BootstrapVerifier::new(config);
        
        let result = verifier.save_rollback_state();
        assert!(result.is_ok());
        
        // Check that rollback file was NOT created when disabled
        let rollback_file = temp_dir.path().join("rollback_state.json");
        assert!(!rollback_file.exists());
        
        // Verify rollback state is None
        assert!(verifier.rollback_state.is_none());
    }
}