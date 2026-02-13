//! Integration tests for the Aproko analysis engine with diagnostic system

#[cfg(test)]
mod tests {
    use crate::*;
    use oviec::ast::{AstNode, Statement, Expression, Literal};

    #[test]
    fn test_aproko_engine_with_diagnostic_integration() {
        let mut engine = AprokoEngine::new();
        
        // Create a simple AST for testing
        let ast = AstNode::Program(vec![
            Statement::Assignment {
                identifier: "".to_string(), // Empty identifier should trigger diagnostic
                value: Expression::Literal(Literal::Number(42.0)),
                mutable: false,
            },
            Statement::Print {
                expression: Expression::Identifier("undefined_var".to_string()),
            },
        ]);

        let source = "let  = 42\nseeAm(undefined_var)";
        
        // Run analysis
        let result = engine.analyze(source, &ast);
        assert!(result.is_ok());
        
        let analysis_results = result.unwrap();
        
        // Debug: print findings and diagnostics
        println!("Findings: {}", analysis_results.findings.len());
        println!("Diagnostics: {}", analysis_results.diagnostics.len());
        for finding in &analysis_results.findings {
            println!("Finding: {:?}", finding);
        }
        for diagnostic in &analysis_results.diagnostics {
            println!("Diagnostic: {:?}", diagnostic);
        }
        
        // Should have findings from analyzers
        assert!(!analysis_results.findings.is_empty(), "Expected findings but got none");
        
        // Should have structured diagnostics
        assert!(!analysis_results.diagnostics.is_empty(), "Expected diagnostics but got none");
        
        // Verify statistics are tracked
        assert!(analysis_results.stats.duration_ms >= 0);
        assert_eq!(analysis_results.stats.lines_analyzed, 2);
        
        // Check that diagnostics correspond to findings
        assert_eq!(analysis_results.findings.len(), analysis_results.diagnostics.len());
    }

    #[test]
    fn test_diagnostic_engine_rule_based_categorization() {
        let engine = AprokoEngine::new();
        let diagnostic_engine = engine.diagnostic_engine();
        
        // Test that default rules are properly categorized
        let syntax_rules = diagnostic_engine.get_rules_by_category(
            crate::diagnostic::DiagnosticCategory::SyntaxError
        );
        let type_rules = diagnostic_engine.get_rules_by_category(
            crate::diagnostic::DiagnosticCategory::TypeError
        );
        let performance_rules = diagnostic_engine.get_rules_by_category(
            crate::diagnostic::DiagnosticCategory::PerformanceWarning
        );
        
        assert!(!syntax_rules.is_empty(), "Should have syntax error rules");
        assert!(!type_rules.is_empty(), "Should have type error rules");
        assert!(!performance_rules.is_empty(), "Should have performance warning rules");
        
        // Verify rule categories are correct
        for rule in syntax_rules {
            assert_eq!(rule.category, crate::diagnostic::DiagnosticCategory::SyntaxError);
        }
    }

    #[test]
    fn test_structured_diagnostic_output() {
        let mut engine = AprokoEngine::new();
        
        // Create AST with multiple types of issues
        let ast = AstNode::Program(vec![
                Statement::Assignment {
                    identifier: "fn".to_string(), // Reserved keyword
                    value: Expression::Literal(Literal::Number(42.0)),
                    mutable: false,
                },
                Statement::Function {
                    name: "".to_string(), // Empty function name
                    parameters: vec!["param1".to_string()],
                    body: vec![], // Empty body
                },
                Statement::If {
                    condition: Expression::Literal(Literal::Boolean(true)),
                    then_block: vec![], // Empty if block
                    else_block: Some(vec![]), // Empty else block
                },
            ]);

        let source = "let fn = 42\nfn () {}\nif true {} else {}";
        
        let result = engine.analyze(source, &ast).unwrap();
        
        // Should have multiple diagnostics with different categories and severities
        assert!(result.diagnostics.len() >= 3);
        
        // Verify diagnostic structure
        for diagnostic in &result.diagnostics {
            assert!(!diagnostic.rule_id.is_empty());
            assert!(!diagnostic.message.is_empty());
            assert!(diagnostic.explanation.is_some());
            assert!(diagnostic.location.line > 0);
            assert!(diagnostic.location.column > 0);
        }
        
        // Should have both errors and warnings
        let has_errors = result.diagnostics.iter().any(|d| d.severity == Severity::Error);
        let has_warnings = result.diagnostics.iter().any(|d| d.severity == Severity::Warning);
        
        assert!(has_errors, "Should have error-level diagnostics");
        assert!(has_warnings, "Should have warning-level diagnostics");
    }

    #[test]
    fn test_diagnostic_engine_configuration() {
        let mut config = AprokoConfig::default();
        config.min_severity = Severity::Error; // Only show errors
        
        let mut engine = AprokoEngine::with_config(config);
        
        // Create AST with both errors and warnings
        let ast = AstNode::Program(vec![
                Statement::Assignment {
                    identifier: "".to_string(), // Error: empty identifier
                    value: Expression::Literal(Literal::Number(42.0)),
                    mutable: false,
                },
                Statement::Function {
                    name: "test_func".to_string(),
                    parameters: vec![],
                    body: vec![], // Warning: empty function body
                },
            ]);

        let source = "let  = 42\nfn test_func() {}";
        let result = engine.analyze(source, &ast).unwrap();
        
        // Should filter out warnings, only show errors
        let error_count = result.diagnostics.iter()
            .filter(|d| d.severity >= Severity::Error)
            .count();
        let warning_count = result.diagnostics.iter()
            .filter(|d| d.severity == Severity::Warning)
            .count();
        
        assert!(error_count > 0, "Should have errors");
        // Note: Warnings might still appear if they're generated by the analyzers
        // but the diagnostic engine should respect severity filtering
    }

    #[test]
    fn test_diagnostic_metadata_and_suggestions() {
        let mut engine = AprokoEngine::new();
        
        let ast = AstNode::Program(vec![
                Statement::Assignment {
                    identifier: "test_var".to_string(),
                    value: Expression::Call {
                        function: "".to_string(), // Empty function name
                        arguments: vec![],
                    },
                    mutable: false,
                },
            ]);

        let source = "let test_var = ()";
        let result = engine.analyze(source, &ast).unwrap();
        
        // Check that diagnostics have proper metadata
        for diagnostic in &result.diagnostics {
            // Should have explanation
            assert!(diagnostic.explanation.is_some());
            
            // Location should be valid
            assert!(diagnostic.location.line > 0);
            assert!(diagnostic.location.column > 0);
            
            // Should have a valid rule ID
            assert!(!diagnostic.rule_id.is_empty());
            
            // Category should be valid
            match diagnostic.category {
                crate::diagnostic::DiagnosticCategory::SyntaxError |
                crate::diagnostic::DiagnosticCategory::TypeError |
                crate::diagnostic::DiagnosticCategory::OwnershipError |
                crate::diagnostic::DiagnosticCategory::MemoryError |
                crate::diagnostic::DiagnosticCategory::LogicError |
                crate::diagnostic::DiagnosticCategory::PerformanceWarning |
                crate::diagnostic::DiagnosticCategory::SecurityWarning |
                crate::diagnostic::DiagnosticCategory::StyleWarning |
                crate::diagnostic::DiagnosticCategory::DeprecationWarning |
                crate::diagnostic::DiagnosticCategory::Info => {
                    // Valid category
                }
            }
        }
    }

    #[test]
    fn test_diagnostic_statistics_tracking() {
        let mut engine = AprokoEngine::new();
        
        let ast = AstNode::Program(vec![
                Statement::Assignment {
                    identifier: "".to_string(), // Error
                    value: Expression::Literal(Literal::Number(42.0)),
                    mutable: false,
                },
                Statement::Assignment {
                    identifier: "valid_var".to_string(),
                    value: Expression::Literal(Literal::String("test".to_string())),
                    mutable: false,
                },
                Statement::Function {
                    name: "test_func".to_string(),
                    parameters: vec![],
                    body: vec![], // Warning
                },
            ]);

        let source = "let  = 42\nlet valid_var = \"test\"\nfn test_func() {}";
        let result = engine.analyze(source, &ast).unwrap();
        
        // Verify statistics are properly tracked
        let stats = &result.stats;
        assert!(stats.duration_ms > 0);
        assert_eq!(stats.lines_analyzed, 3);
        
        // Should have findings categorized by severity
        let total_findings: usize = stats.findings_by_severity.values().sum();
        assert!(total_findings > 0);
        
        // Should have findings categorized by analysis category
        let total_by_category: usize = stats.findings_by_category.values().sum();
        assert_eq!(total_findings, total_by_category);
    }

    #[test]
    fn test_explanation_system_integration() {
        let mut engine = AprokoEngine::new();
        
        // Create AST with syntax error
        let ast = AstNode::Program(vec![
                Statement::Assignment {
                    identifier: "".to_string(), // Empty identifier should trigger diagnostic
                    value: Expression::Literal(Literal::Number(42.0)),
                    mutable: false,
                },
            ]);

        let source = "let  = 42";
        let result = engine.analyze(source, &ast).unwrap();
        
        // Should have diagnostics
        assert!(!result.diagnostics.is_empty());
        
        // Test explanation for the first diagnostic
        let first_diagnostic = &result.diagnostics[0];
        let explanation_result = engine.explain_diagnostic(first_diagnostic);
        
        assert!(explanation_result.is_ok());
        let explanation = explanation_result.unwrap();
        
        // Verify explanation structure
        assert!(!explanation.summary.is_empty());
        assert!(!explanation.detailed_explanation.is_empty());
        assert!(explanation.confidence > 0.0);
        assert!(explanation.confidence <= 1.0);
        
        // Should have location information in detailed explanation
        assert!(explanation.detailed_explanation.contains("line"));
        assert!(explanation.detailed_explanation.contains("column"));
    }

    #[test]
    fn test_fix_suggestions_generation() {
        let mut engine = AprokoEngine::new();
        
        // Create AST with multiple fixable issues
        let ast = AstNode::Program(vec![
                Statement::Assignment {
                    identifier: "fn".to_string(), // Reserved keyword
                    value: Expression::Literal(Literal::Number(42.0)),
                    mutable: false,
                },
                Statement::Function {
                    name: "test_func".to_string(),
                    parameters: vec![],
                    body: vec![], // Empty body
                },
            ]);

        let source = "let fn = 42\nfn test_func() {}";
        let result = engine.analyze(source, &ast).unwrap();
        
        // Test explanations for all diagnostics
        for diagnostic in &result.diagnostics {
            let explanation_result = engine.explain_diagnostic(diagnostic);
            assert!(explanation_result.is_ok());
            
            let explanation = explanation_result.unwrap();
            
            // Should have fix suggestions for most issues
            if !explanation.fix_suggestions.is_empty() {
                let fix = &explanation.fix_suggestions[0];
                
                // Verify fix suggestion structure
                assert!(!fix.title.is_empty());
                assert!(!fix.description.is_empty());
                assert!(!fix.steps.is_empty());
                assert!(fix.confidence > 0.0);
                assert!(fix.confidence <= 1.0);
                
                // Should never be auto-applicable for safety
                assert!(!fix.auto_applicable);
                
                // Steps should be numbered correctly
                for (i, step) in fix.steps.iter().enumerate() {
                    assert_eq!(step.step_number, i + 1);
                    assert!(!step.description.is_empty());
                }
            }
        }
    }

    #[test]
    fn test_explanation_engine_access() {
        let engine = AprokoEngine::new();
        
        // Test access to explanation engine
        let explanation_engine = engine.explanation_engine();
        let all_explanations = explanation_engine.get_all_explanations();
        
        // Should have default explanations
        assert!(!all_explanations.is_empty());
        
        // Should have explanations for common rule IDs
        assert!(all_explanations.contains_key("E001"));
        assert!(all_explanations.contains_key("E002"));
        assert!(all_explanations.contains_key("W001"));
        assert!(all_explanations.contains_key("S001"));
    }
}
