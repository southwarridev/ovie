//! Integration tests for Aproko analyzers

use crate::{AprokoEngine, AnalysisCategory, Severity, AprokoConfig, CategoryConfig, CustomRule};
use oviec::{lexer::Lexer, parser::Parser};
use std::collections::HashMap;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_syntax_analyzer_integration() {
        let source = r#"
            fn test() {
                seeAm "hello";
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let engine = AprokoEngine::new();
        let results = engine.analyze(source, &ast).unwrap();

        // Should have some findings (at least no critical errors)
        assert!(results.findings.iter().all(|f| f.severity != Severity::Critical));
    }

    #[test]
    fn test_logic_analyzer_integration() {
        let source = r#"
            fn test() {
                if true {
                    seeAm "always true";
                } else {
                    seeAm "never reached";
                }
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let engine = AprokoEngine::new();
        let results = engine.analyze(source, &ast).unwrap();

        // Should detect unreachable else block
        let logic_findings: Vec<_> = results.findings.iter()
            .filter(|f| f.category == AnalysisCategory::Logic)
            .collect();
        
        assert!(!logic_findings.is_empty());
        assert!(logic_findings.iter().any(|f| f.rule_id == "unreachable_else"));
    }

    #[test]
    fn test_security_analyzer_integration() {
        let source = r#"
            fn test() {
                password = "secret123";
                seeAm password;
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let engine = AprokoEngine::new();
        let results = engine.analyze(source, &ast).unwrap();

        // Should detect sensitive variable name and potential information disclosure
        let security_findings: Vec<_> = results.findings.iter()
            .filter(|f| f.category == AnalysisCategory::Security)
            .collect();
        
        assert!(!security_findings.is_empty());
        assert!(security_findings.iter().any(|f| f.rule_id.contains("sensitive")));
    }

    #[test]
    fn test_performance_analyzer_integration() {
        let source = r#"
            fn test() {
                for i in 1..10 {
                    for j in 1..10 {
                        seeAm i + j;
                    }
                }
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let engine = AprokoEngine::new();
        let results = engine.analyze(source, &ast).unwrap();

        // Should detect nested loops
        let performance_findings: Vec<_> = results.findings.iter()
            .filter(|f| f.category == AnalysisCategory::Performance)
            .collect();
        
        assert!(!performance_findings.is_empty());
        assert!(performance_findings.iter().any(|f| f.rule_id == "nested_loop"));
    }

    #[test]
    fn test_multiple_analyzers() {
        let source = r#"
            fn complex_function(a, b, c, d, e, f) {
                password = "hardcoded_secret";
                if true {
                    for i in 1..1000000 {
                        seeAm password;
                    }
                } else {
                    seeAm "unreachable";
                }
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let engine = AprokoEngine::new();
        let results = engine.analyze(source, &ast).unwrap();

        // Should have findings from multiple analyzers
        let categories: std::collections::HashSet<_> = results.findings.iter()
            .map(|f| f.category)
            .collect();
        
        assert!(categories.len() >= 2); // At least 2 different categories
        assert!(results.stats.findings_by_category.len() >= 2);
    }

    #[test]
    fn test_correctness_analyzer_integration() {
        let source = r#"
            fn test() {
                x = getValue();
                y = x;  // x is moved here
                seeAm x;  // use after move
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let engine = AprokoEngine::new();
        let results = engine.analyze(source, &ast).unwrap();

        // Should detect use after move
        let correctness_findings: Vec<_> = results.findings.iter()
            .filter(|f| f.category == AnalysisCategory::Correctness)
            .collect();
        
        assert!(!correctness_findings.is_empty());
        assert!(correctness_findings.iter().any(|f| f.rule_id.contains("move") || f.rule_id.contains("undeclared")));
    }

    #[test]
    fn test_style_analyzer_integration() {
        let source = r#"
            fn BadFunctionName(VeryLongParameterNameThatExceedsReasonableLength, another_param) {
                strUserName = "hardcoded_value";
                magic_number = 42;
                if (condition1 && condition2 && condition3 && condition4) {
                    if (nested_condition) {
                        if (deeply_nested) {
                            seeAm "too deep";
                        }
                    }
                }
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let engine = AprokoEngine::new();
        let results = engine.analyze(source, &ast).unwrap();

        // Should detect style issues
        let style_findings: Vec<_> = results.findings.iter()
            .filter(|f| f.category == AnalysisCategory::Style)
            .collect();
        
        assert!(!style_findings.is_empty());
        // Should detect naming issues, magic numbers, complex conditions, etc.
        assert!(style_findings.len() >= 3);
    }

    #[test]
    fn test_all_analyzers_working() {
        let source = r#"
            fn badFunction(a, b, c, d, e, f, g) {
                password = "secret123";
                for i in 1..1000000 {
                    for j in 1..100 {
                        if true {
                            seeAm password;
                        } else {
                            seeAm "unreachable";
                        }
                    }
                }
                undeclaredVar = getValue();
                return 42;
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let engine = AprokoEngine::new();
        let results = engine.analyze(source, &ast).unwrap();

        // Should have findings from all analyzer categories
        let categories: std::collections::HashSet<_> = results.findings.iter()
            .map(|f| f.category)
            .collect();
        
        // Should have findings from at least 4 different categories
        assert!(categories.len() >= 4);
        
        // Verify we have the expected categories
        assert!(categories.contains(&AnalysisCategory::Security)); // password variable
        assert!(categories.contains(&AnalysisCategory::Performance)); // nested loops
        assert!(categories.contains(&AnalysisCategory::Logic)); // unreachable else
        assert!(categories.contains(&AnalysisCategory::Style)); // bad naming, magic number
    }

    #[test]
    fn test_aproko_configuration_compliance() {
        let source = r#"
            fn test() {
                password = "secret";
                seeAm password;
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        // Test default configuration
        let engine = AprokoEngine::new();
        let results = engine.analyze(source, &ast).unwrap();
        
        // Should have all categories enabled by default
        assert_eq!(results.config.enabled_categories.len(), 6);
        assert_eq!(results.config.min_severity, Severity::Info);
        
        // Test custom configuration
        let mut custom_config = AprokoConfig::default();
        custom_config.min_severity = Severity::Warning;
        custom_config.enabled_categories = vec![AnalysisCategory::Security, AnalysisCategory::Syntax];
        
        let mut custom_engine = AprokoEngine::with_config(custom_config);
        let custom_results = custom_engine.analyze(source, &ast).unwrap();
        
        // Should respect custom configuration
        assert_eq!(custom_results.config.enabled_categories.len(), 2);
        assert_eq!(custom_results.config.min_severity, Severity::Warning);
        
        // Should only have findings from enabled categories
        let categories: std::collections::HashSet<_> = custom_results.findings.iter()
            .map(|f| f.category)
            .collect();
        
        for category in categories {
            assert!(custom_results.config.enabled_categories.contains(&category));
        }
        
        // Should filter by minimum severity
        for finding in &custom_results.findings {
            assert!(finding.severity >= Severity::Warning);
        }
    }

    #[test]
    fn test_aproko_toml_configuration() {
        use std::collections::HashMap;
        
        // Test TOML configuration parsing
        let mut config = AprokoConfig::default();
        
        // Test category configuration
        let mut category_config = CategoryConfig {
            enabled: false,
            settings: HashMap::new(),
        };
        category_config.settings.insert("check_grammar".to_string(), "false".to_string());
        
        config.category_configs.insert(AnalysisCategory::Syntax, category_config);
        
        let mut engine = AprokoEngine::with_config(config);
        
        // Configuration should be applied
        assert!(engine.config().category_configs.contains_key(&AnalysisCategory::Syntax));
        
        let syntax_config = &engine.config().category_configs[&AnalysisCategory::Syntax];
        assert!(!syntax_config.enabled);
        assert_eq!(syntax_config.settings.get("check_grammar"), Some(&"false".to_string()));
    }

    #[test]
    fn test_aproko_analyzer_configuration() {
        let source = r#"
            fn test() {
                x = 42;
                seeAm x;
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        // Test with performance analyzer configured for lower complexity threshold
        let mut config = AprokoConfig::default();
        let mut perf_config = CategoryConfig {
            enabled: true,
            settings: HashMap::new(),
        };
        perf_config.settings.insert("max_complexity".to_string(), "1".to_string());
        config.category_configs.insert(AnalysisCategory::Performance, perf_config);
        
        let engine = AprokoEngine::with_config(config);
        let results = engine.analyze(source, &ast).unwrap();
        
        // Configuration should be respected
        let perf_config = &results.config.category_configs[&AnalysisCategory::Performance];
        assert_eq!(perf_config.settings.get("max_complexity"), Some(&"1".to_string()));
    }

    #[test]
    fn test_aproko_custom_rules() {
        use crate::{CustomRule, Severity};
        
        let mut config = AprokoConfig::default();
        
        // Add custom rule
        let custom_rule = CustomRule {
            id: "test_rule".to_string(),
            description: "Test custom rule".to_string(),
            pattern: "test_pattern".to_string(),
            suggestion: "Test suggestion".to_string(),
            severity: Severity::Warning,
        };
        
        config.custom_rules.push(custom_rule);
        
        let engine = AprokoEngine::with_config(config);
        
        // Custom rule should be in configuration
        assert_eq!(engine.config().custom_rules.len(), 1);
        assert_eq!(engine.config().custom_rules[0].id, "test_rule");
        assert_eq!(engine.config().custom_rules[0].severity, Severity::Warning);
    }

    #[test]
    fn test_aproko_configuration_validation() {
        // Test that configuration changes are properly validated
        let mut engine = AprokoEngine::new();
        
        // Test updating configuration
        let mut new_config = AprokoConfig::default();
        new_config.min_severity = Severity::Error;
        new_config.enabled_categories = vec![AnalysisCategory::Security];
        
        let result = engine.set_config(new_config);
        assert!(result.is_ok());
        
        // Configuration should be updated
        assert_eq!(engine.config().min_severity, Severity::Error);
        assert_eq!(engine.config().enabled_categories.len(), 1);
        assert!(engine.config().enabled_categories.contains(&AnalysisCategory::Security));
    }

    #[test]
    fn test_empty_program() {
        let source = "";

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let engine = AprokoEngine::new();
        let results = engine.analyze(source, &ast).unwrap();

        // Should detect empty program
        let syntax_findings: Vec<_> = results.findings.iter()
            .filter(|f| f.category == AnalysisCategory::Syntax)
            .collect();
        
        assert!(syntax_findings.iter().any(|f| f.rule_id == "empty_program"));
    }
}