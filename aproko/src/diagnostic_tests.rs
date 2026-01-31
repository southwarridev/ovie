//! Comprehensive tests for the diagnostic engine

#[cfg(test)]
mod tests {
    use super::super::diagnostic::*;
    use crate::{Finding, Severity, AnalysisCategory};

    #[test]
    fn test_diagnostic_engine_default_rules() {
        let engine = DiagnosticEngine::with_config(DiagnosticConfig::default());
        let rules = engine.get_rules();
        
        // Should have default rules registered
        assert!(rules.contains_key("E001")); // Syntax error
        assert!(rules.contains_key("E002")); // Type error
        assert!(rules.contains_key("E003")); // Ownership error
        assert!(rules.contains_key("W001")); // Performance warning
        assert!(rules.contains_key("S001")); // Security warning
    }

    #[test]
    fn test_diagnostic_generation_with_valid_rule() {
        let mut engine = DiagnosticEngine::with_config(DiagnosticConfig::default());
        let location = SourceLocation {
            file: "test.ov".to_string(),
            line: 10,
            column: 5,
            span_length: 3,
            source_excerpt: Some("let".to_string()),
        };

        let result = engine.generate_diagnostic(
            &"E001".to_string(),
            "Unexpected token 'let'".to_string(),
            location,
        );

        assert!(result.is_ok());
        let diagnostic = result.unwrap();
        assert_eq!(diagnostic.rule_id, "E001");
        assert_eq!(diagnostic.category, DiagnosticCategory::SyntaxError);
        assert_eq!(diagnostic.severity, Severity::Error);
        assert_eq!(diagnostic.message, "Unexpected token 'let'");
        assert_eq!(diagnostic.location.line, 10);
        assert_eq!(diagnostic.location.column, 5);
    }

    #[test]
    fn test_diagnostic_generation_with_invalid_rule() {
        let mut engine = DiagnosticEngine::new();
        let location = SourceLocation {
            file: "test.ov".to_string(),
            line: 1,
            column: 1,
            span_length: 1,
            source_excerpt: None,
        };

        let result = engine.generate_diagnostic(
            &"INVALID".to_string(),
            "Test message".to_string(),
            location,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_diagnostic_statistics_tracking() {
        let mut engine = DiagnosticEngine::with_config(DiagnosticConfig::default());
        let location = SourceLocation {
            file: "test.ov".to_string(),
            line: 1,
            column: 1,
            span_length: 1,
            source_excerpt: None,
        };

        // Generate multiple diagnostics
        let _ = engine.generate_diagnostic(&"E001".to_string(), "Error 1".to_string(), location.clone());
        let _ = engine.generate_diagnostic(&"E002".to_string(), "Error 2".to_string(), location.clone());
        let _ = engine.generate_diagnostic(&"W001".to_string(), "Warning 1".to_string(), location);

        let stats = engine.get_stats();
        assert_eq!(stats.total_diagnostics, 3);
        assert_eq!(stats.by_severity.get(&Severity::Error), Some(&2));
        assert_eq!(stats.by_severity.get(&Severity::Warning), Some(&1));
        assert_eq!(stats.by_rule.get("E001"), Some(&1));
        assert_eq!(stats.by_rule.get("E002"), Some(&1));
        assert_eq!(stats.by_rule.get("W001"), Some(&1));
    }

    #[test]
    fn test_rule_configuration_override() {
        let mut config = DiagnosticConfig::default();
        
        // Disable a specific rule
        let mut rule_config = RuleConfig {
            enabled: false,
            severity_override: None,
            settings: std::collections::HashMap::new(),
        };
        config.rule_configs.insert("E001".to_string(), rule_config.clone());

        let mut engine = DiagnosticEngine::with_config(config);
        let location = SourceLocation {
            file: "test.ov".to_string(),
            line: 1,
            column: 1,
            span_length: 1,
            source_excerpt: None,
        };

        let result = engine.generate_diagnostic(
            &"E001".to_string(),
            "Test message".to_string(),
            location,
        );

        assert!(result.is_err()); // Should fail because rule is disabled

        // Test severity override
        rule_config.enabled = true;
        rule_config.severity_override = Some(Severity::Critical);
        engine.get_config().rule_configs.insert("E001".to_string(), rule_config);

        let location = SourceLocation {
            file: "test.ov".to_string(),
            line: 1,
            column: 1,
            span_length: 1,
            source_excerpt: None,
        };

        let result = engine.generate_diagnostic(
            &"E001".to_string(),
            "Test message".to_string(),
            location,
        );

        if let Ok(diagnostic) = result {
            assert_eq!(diagnostic.severity, Severity::Critical);
        }
    }

    #[test]
    fn test_minimum_severity_filtering() {
        let mut config = DiagnosticConfig::default();
        config.min_severity = Severity::Error; // Only show errors and above

        let mut engine = DiagnosticEngine::with_config(config);
        let location = SourceLocation {
            file: "test.ov".to_string(),
            line: 1,
            column: 1,
            span_length: 1,
            source_excerpt: None,
        };

        // Try to generate a warning (should be filtered out)
        let result = engine.generate_diagnostic(
            &"W001".to_string(),
            "Warning message".to_string(),
            location.clone(),
        );

        assert!(result.is_err()); // Should fail due to severity filtering

        // Generate an error (should succeed)
        let result = engine.generate_diagnostic(
            &"E001".to_string(),
            "Error message".to_string(),
            location,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_findings_to_diagnostics_conversion() {
        let mut engine = DiagnosticEngine::with_config(DiagnosticConfig::default());
        
        let findings = vec![
            Finding {
                category: AnalysisCategory::Syntax,
                severity: Severity::Error,
                message: "Syntax error".to_string(),
                suggestion: Some("Fix syntax".to_string()),
                location: (1, 1),
                span_length: 5,
                rule_id: "E001".to_string(),
            },
            Finding {
                category: AnalysisCategory::Performance,
                severity: Severity::Warning,
                message: "Performance issue".to_string(),
                suggestion: Some("Optimize code".to_string()),
                location: (2, 10),
                span_length: 3,
                rule_id: "W001".to_string(),
            },
        ];

        let diagnostics = engine.generate_diagnostics_from_findings(findings);
        assert_eq!(diagnostics.len(), 2);
        
        assert_eq!(diagnostics[0].rule_id, "E001");
        assert_eq!(diagnostics[0].message, "Syntax error");
        assert_eq!(diagnostics[0].location.line, 1);
        assert_eq!(diagnostics[0].location.column, 1);
        
        assert_eq!(diagnostics[1].rule_id, "W001");
        assert_eq!(diagnostics[1].message, "Performance issue");
        assert_eq!(diagnostics[1].location.line, 2);
        assert_eq!(diagnostics[1].location.column, 10);
    }

    #[test]
    fn test_diagnostic_categories_by_filter() {
        let engine = DiagnosticEngine::with_config(DiagnosticConfig::default());
        
        let syntax_rules = engine.get_rules_by_category(DiagnosticCategory::SyntaxError);
        let type_rules = engine.get_rules_by_category(DiagnosticCategory::TypeError);
        let performance_rules = engine.get_rules_by_category(DiagnosticCategory::PerformanceWarning);
        
        assert!(!syntax_rules.is_empty());
        assert!(!type_rules.is_empty());
        assert!(!performance_rules.is_empty());
        
        // Verify categories are correct
        for rule in syntax_rules {
            assert_eq!(rule.category, DiagnosticCategory::SyntaxError);
        }
        
        for rule in type_rules {
            assert_eq!(rule.category, DiagnosticCategory::TypeError);
        }
        
        for rule in performance_rules {
            assert_eq!(rule.category, DiagnosticCategory::PerformanceWarning);
        }
    }

    #[test]
    fn test_diagnostic_display_formatting() {
        let diagnostic = Diagnostic {
            rule_id: "E001".to_string(),
            category: DiagnosticCategory::SyntaxError,
            severity: Severity::Error,
            message: "Test error message".to_string(),
            explanation: Some("Detailed explanation".to_string()),
            suggestion: Some("Fix suggestion".to_string()),
            location: SourceLocation {
                file: "main.ov".to_string(),
                line: 42,
                column: 10,
                span_length: 5,
                source_excerpt: Some("error".to_string()),
            },
            related_locations: Vec::new(),
            metadata: std::collections::HashMap::new(),
        };

        let display_str = format!("{}", diagnostic);
        assert!(display_str.contains("Error"));
        assert!(display_str.contains("Test error message"));
        assert!(display_str.contains("main.ov"));
        assert!(display_str.contains("42"));
        assert!(display_str.contains("10"));
    }

    #[test]
    fn test_diagnostic_category_display() {
        assert_eq!(DiagnosticCategory::SyntaxError.to_string(), "syntax-error");
        assert_eq!(DiagnosticCategory::TypeError.to_string(), "type-error");
        assert_eq!(DiagnosticCategory::OwnershipError.to_string(), "ownership-error");
        assert_eq!(DiagnosticCategory::MemoryError.to_string(), "memory-error");
        assert_eq!(DiagnosticCategory::LogicError.to_string(), "logic-error");
        assert_eq!(DiagnosticCategory::PerformanceWarning.to_string(), "performance-warning");
        assert_eq!(DiagnosticCategory::SecurityWarning.to_string(), "security-warning");
        assert_eq!(DiagnosticCategory::StyleWarning.to_string(), "style-warning");
        assert_eq!(DiagnosticCategory::DeprecationWarning.to_string(), "deprecation-warning");
        assert_eq!(DiagnosticCategory::Info.to_string(), "info");
    }

    #[test]
    fn test_stats_reset() {
        let mut engine = DiagnosticEngine::with_config(DiagnosticConfig::default());
        let location = SourceLocation {
            file: "test.ov".to_string(),
            line: 1,
            column: 1,
            span_length: 1,
            source_excerpt: None,
        };

        // Generate some diagnostics
        let _ = engine.generate_diagnostic(&"E001".to_string(), "Error".to_string(), location);
        assert_eq!(engine.get_stats().total_diagnostics, 1);

        // Reset stats
        engine.reset_stats();
        assert_eq!(engine.get_stats().total_diagnostics, 0);
        assert!(engine.get_stats().by_severity.is_empty());
        assert!(engine.get_stats().by_category.is_empty());
        assert!(engine.get_stats().by_rule.is_empty());
    }
}