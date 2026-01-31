//! Property-based tests for Aproko diagnostic completeness
//!
//! These tests validate that the Aproko system provides comprehensive and consistent
//! diagnostic coverage across all analysis categories and rule types.

use crate::*;
use crate::diagnostic::*;
use crate::explanation::*;
use oviec::ast::{AstNode, Statement, Expression, Literal};
use std::collections::{HashMap, HashSet};

/// Property 11: Aproko Diagnostic Completeness
/// 
/// This property validates that Aproko provides comprehensive diagnostic coverage
/// by testing various invariants about diagnostic generation, categorization,
/// and explanation completeness.

/// Test that all diagnostic rules have corresponding explanations
#[test]
fn property_all_rules_have_explanations() {
    let engine = AprokoEngine::new();
    let diagnostic_engine = engine.diagnostic_engine();
    let explanation_engine = engine.explanation_engine();
    
    let all_rules = diagnostic_engine.get_rules();
    let all_explanations = explanation_engine.get_all_explanations();
    
    // Property: Every diagnostic rule should have a corresponding explanation
    for (rule_id, _rule) in all_rules {
        assert!(
            all_explanations.contains_key(rule_id),
            "Rule {} has no corresponding explanation",
            rule_id
        );
    }
    
    println!("✓ Property verified: All {} diagnostic rules have explanations", all_rules.len());
}

/// Test that diagnostic categories are consistently mapped
#[test]
fn property_diagnostic_categories_consistent() {
    let engine = AprokoEngine::new();
    let diagnostic_engine = engine.diagnostic_engine();
    
    // Property: All diagnostic categories should be represented in the rule set
    let expected_categories = vec![
        DiagnosticCategory::SyntaxError,
        DiagnosticCategory::TypeError,
        DiagnosticCategory::OwnershipError,
        DiagnosticCategory::MemoryError,
        DiagnosticCategory::LogicError,
        DiagnosticCategory::PerformanceWarning,
        DiagnosticCategory::SecurityWarning,
        DiagnosticCategory::StyleWarning,
    ];
    
    let mut found_categories = HashSet::new();
    for (_rule_id, rule) in diagnostic_engine.get_rules() {
        found_categories.insert(rule.category);
    }
    
    for expected_category in expected_categories {
        assert!(
            found_categories.contains(&expected_category),
            "No rules found for category {:?}",
            expected_category
        );
    }
    
    println!("✓ Property verified: All diagnostic categories are represented");
}

/// Test that severity levels are properly distributed
#[test]
fn property_severity_levels_distributed() {
    let engine = AprokoEngine::new();
    let diagnostic_engine = engine.diagnostic_engine();
    
    let mut severity_counts = HashMap::new();
    for (_rule_id, rule) in diagnostic_engine.get_rules() {
        *severity_counts.entry(rule.default_severity).or_insert(0) += 1;
    }
    
    // Property: Should have rules at multiple severity levels
    assert!(
        severity_counts.len() >= 2,
        "Rules should span multiple severity levels, found: {:?}",
        severity_counts.keys().collect::<Vec<_>>()
    );
    
    // Property: Should have at least one error-level rule
    assert!(
        severity_counts.contains_key(&Severity::Error),
        "Should have at least one error-level rule"
    );
    
    println!("✓ Property verified: Severity levels are properly distributed: {:?}", severity_counts);
}

/// Test that analysis produces consistent results for identical input
#[test]
fn property_analysis_deterministic() {
    let mut engine = AprokoEngine::new();
    
    // Create test AST
    let ast = AstNode {
        statements: vec![
            Statement::Assignment {
                identifier: "".to_string(), // Should trigger diagnostic
                value: Expression::Literal(Literal::Integer(42)),
                mutable: false,
            },
            Statement::Assignment {
                identifier: "valid_var".to_string(),
                value: Expression::Literal(Literal::String("test".to_string())),
                mutable: false,
            },
        ],
    };
    
    let source = "let  = 42\nlet valid_var = \"test\"";
    
    // Run analysis multiple times
    let result1 = engine.analyze(source, &ast).expect("First analysis should succeed");
    let result2 = engine.analyze(source, &ast).expect("Second analysis should succeed");
    let result3 = engine.analyze(source, &ast).expect("Third analysis should succeed");
    
    // Property: Results should be identical across runs
    assert_eq!(
        result1.findings.len(),
        result2.findings.len(),
        "Finding count should be consistent"
    );
    assert_eq!(
        result1.findings.len(),
        result3.findings.len(),
        "Finding count should be consistent"
    );
    
    assert_eq!(
        result1.diagnostics.len(),
        result2.diagnostics.len(),
        "Diagnostic count should be consistent"
    );
    assert_eq!(
        result1.diagnostics.len(),
        result3.diagnostics.len(),
        "Diagnostic count should be consistent"
    );
    
    // Property: Diagnostic messages should be identical
    for (i, (d1, d2)) in result1.diagnostics.iter().zip(result2.diagnostics.iter()).enumerate() {
        assert_eq!(
            d1.rule_id, d2.rule_id,
            "Diagnostic {} rule_id should be consistent",
            i
        );
        assert_eq!(
            d1.message, d2.message,
            "Diagnostic {} message should be consistent",
            i
        );
        assert_eq!(
            d1.severity, d2.severity,
            "Diagnostic {} severity should be consistent",
            i
        );
    }
    
    println!("✓ Property verified: Analysis is deterministic across {} runs", 3);
}

/// Test that all generated diagnostics can be explained
#[test]
fn property_all_diagnostics_explainable() {
    let mut engine = AprokoEngine::new();
    
    // Create test AST with various issues
    let ast = AstNode {
        statements: vec![
            Statement::Assignment {
                identifier: "".to_string(), // Empty identifier
                value: Expression::Literal(Literal::Integer(42)),
                mutable: false,
            },
            Statement::Assignment {
                identifier: "fn".to_string(), // Reserved keyword
                value: Expression::Literal(Literal::String("test".to_string())),
                mutable: false,
            },
            Statement::Function {
                name: "".to_string(), // Empty function name
                parameters: vec![],
                body: vec![], // Empty body
            },
        ],
    };
    
    let source = "let  = 42\nlet fn = \"test\"\nfn () {}";
    let result = engine.analyze(source, &ast).expect("Analysis should succeed");
    
    // Property: Every diagnostic should be explainable
    for diagnostic in &result.diagnostics {
        let explanation_result = engine.explain_diagnostic(diagnostic);
        assert!(
            explanation_result.is_ok(),
            "Diagnostic with rule {} should be explainable: {:?}",
            diagnostic.rule_id,
            explanation_result.err()
        );
        
        let explanation = explanation_result.unwrap();
        
        // Property: Explanations should have meaningful content
        assert!(
            !explanation.summary.is_empty(),
            "Explanation for rule {} should have non-empty summary",
            diagnostic.rule_id
        );
        assert!(
            !explanation.detailed_explanation.is_empty(),
            "Explanation for rule {} should have non-empty detailed explanation",
            diagnostic.rule_id
        );
        assert!(
            explanation.confidence > 0.0 && explanation.confidence <= 1.0,
            "Explanation confidence should be between 0 and 1, got {}",
            explanation.confidence
        );
    }
    
    println!("✓ Property verified: All {} diagnostics are explainable", result.diagnostics.len());
}

/// Test that fix suggestions are actionable and safe
#[test]
fn property_fix_suggestions_safe() {
    let engine = AprokoEngine::new();
    let explanation_engine = engine.explanation_engine();
    
    // Property: All fix suggestions should be marked as non-auto-applicable for safety
    for (_rule_id, explanation) in explanation_engine.get_all_explanations() {
        for fix_suggestion in &explanation.fix_suggestions {
            assert!(
                !fix_suggestion.auto_applicable,
                "Fix suggestion '{}' should not be auto-applicable for safety",
                fix_suggestion.title
            );
            
            // Property: Fix suggestions should have reasonable confidence
            assert!(
                fix_suggestion.confidence > 0.0 && fix_suggestion.confidence <= 1.0,
                "Fix suggestion confidence should be between 0 and 1, got {}",
                fix_suggestion.confidence
            );
            
            // Property: Fix suggestions should have steps
            assert!(
                !fix_suggestion.steps.is_empty(),
                "Fix suggestion '{}' should have at least one step",
                fix_suggestion.title
            );
            
            // Property: Steps should be properly numbered
            for (i, step) in fix_suggestion.steps.iter().enumerate() {
                assert_eq!(
                    step.step_number,
                    i + 1,
                    "Step should be numbered correctly"
                );
                assert!(
                    !step.description.is_empty(),
                    "Step {} should have non-empty description",
                    step.step_number
                );
            }
        }
    }
    
    println!("✓ Property verified: All fix suggestions are safe and actionable");
}

/// Test that diagnostic statistics are accurate
#[test]
fn property_statistics_accurate() {
    let mut engine = AprokoEngine::new();
    
    // Create test AST with known number of issues
    let ast = AstNode {
        statements: vec![
            Statement::Assignment {
                identifier: "".to_string(), // Error
                value: Expression::Literal(Literal::Integer(42)),
                mutable: false,
            },
            Statement::Function {
                name: "test_func".to_string(),
                parameters: vec![],
                body: vec![], // Warning
            },
            Statement::Assignment {
                identifier: "valid_var".to_string(), // No issue
                value: Expression::Literal(Literal::String("test".to_string())),
                mutable: false,
            },
        ],
    };
    
    let source = "let  = 42\nfn test_func() {}\nlet valid_var = \"test\"";
    let result = engine.analyze(source, &ast).expect("Analysis should succeed");
    
    // Property: Statistics should match actual findings
    let total_by_severity: usize = result.stats.findings_by_severity.values().sum();
    let total_by_category: usize = result.stats.findings_by_category.values().sum();
    
    assert_eq!(
        result.findings.len(),
        total_by_severity,
        "Total findings should match sum of findings by severity"
    );
    assert_eq!(
        result.findings.len(),
        total_by_category,
        "Total findings should match sum of findings by category"
    );
    
    // Property: Lines analyzed should match source
    assert_eq!(
        result.stats.lines_analyzed,
        source.lines().count(),
        "Lines analyzed should match source line count"
    );
    
    // Property: Duration should be reasonable
    assert!(
        result.stats.duration_ms > 0,
        "Analysis duration should be positive"
    );
    
    println!("✓ Property verified: Statistics are accurate");
}

/// Test that analysis handles edge cases gracefully
#[test]
fn property_handles_edge_cases() {
    let mut engine = AprokoEngine::new();
    
    // Test empty program
    let empty_ast = AstNode { statements: vec![] };
    let empty_result = engine.analyze("", &empty_ast);
    assert!(empty_result.is_ok(), "Should handle empty program gracefully");
    
    // Test program with only whitespace
    let whitespace_result = engine.analyze("   \n  \n  ", &empty_ast);
    assert!(whitespace_result.is_ok(), "Should handle whitespace-only program gracefully");
    
    // Test very long identifier
    let long_identifier = "a".repeat(1000);
    let long_ast = AstNode {
        statements: vec![
            Statement::Assignment {
                identifier: long_identifier,
                value: Expression::Literal(Literal::Integer(42)),
                mutable: false,
            },
        ],
    };
    let long_result = engine.analyze("let a... = 42", &long_ast);
    assert!(long_result.is_ok(), "Should handle long identifiers gracefully");
    
    // Test deeply nested structure (simulate with many statements)
    let many_statements: Vec<Statement> = (0..100)
        .map(|i| Statement::Assignment {
            identifier: format!("var_{}", i),
            value: Expression::Literal(Literal::Integer(i as i64)),
            mutable: false,
        })
        .collect();
    
    let large_ast = AstNode { statements: many_statements };
    let large_result = engine.analyze("// Large program", &large_ast);
    assert!(large_result.is_ok(), "Should handle large programs gracefully");
    
    println!("✓ Property verified: Analysis handles edge cases gracefully");
}

/// Test that diagnostic rules cover all analysis categories
#[test]
fn property_rules_cover_all_categories() {
    let engine = AprokoEngine::new();
    let diagnostic_engine = engine.diagnostic_engine();
    
    // Map analysis categories to diagnostic categories
    let category_mapping = vec![
        (AnalysisCategory::Syntax, DiagnosticCategory::SyntaxError),
        (AnalysisCategory::Logic, DiagnosticCategory::LogicError),
        (AnalysisCategory::Performance, DiagnosticCategory::PerformanceWarning),
        (AnalysisCategory::Security, DiagnosticCategory::SecurityWarning),
        (AnalysisCategory::Correctness, DiagnosticCategory::OwnershipError),
        (AnalysisCategory::Style, DiagnosticCategory::StyleWarning),
    ];
    
    let mut covered_diagnostic_categories = HashSet::new();
    for (_rule_id, rule) in diagnostic_engine.get_rules() {
        covered_diagnostic_categories.insert(rule.category);
    }
    
    // Property: Each analysis category should have corresponding diagnostic rules
    for (_analysis_category, diagnostic_category) in category_mapping {
        assert!(
            covered_diagnostic_categories.contains(&diagnostic_category),
            "No diagnostic rules found for category {:?}",
            diagnostic_category
        );
    }
    
    println!("✓ Property verified: Diagnostic rules cover all analysis categories");
}

/// Test that explanation confidence correlates with rule maturity
#[test]
fn property_explanation_confidence_reasonable() {
    let engine = AprokoEngine::new();
    let explanation_engine = engine.explanation_engine();
    
    let mut confidence_values = vec![];
    
    for (_rule_id, explanation) in explanation_engine.get_all_explanations() {
        // Property: Confidence should be in valid range
        assert!(
            explanation.confidence >= 0.0 && explanation.confidence <= 1.0,
            "Explanation confidence should be between 0 and 1, got {}",
            explanation.confidence
        );
        
        confidence_values.push(explanation.confidence);
        
        // Property: Explanations with code examples should have higher confidence
        if !explanation.code_examples.is_empty() {
            assert!(
                explanation.confidence >= 0.5,
                "Explanations with code examples should have confidence >= 0.5, got {}",
                explanation.confidence
            );
        }
        
        // Property: Explanations with fix suggestions should have reasonable confidence
        if !explanation.fix_suggestions.is_empty() {
            assert!(
                explanation.confidence >= 0.4,
                "Explanations with fix suggestions should have confidence >= 0.4, got {}",
                explanation.confidence
            );
        }
    }
    
    // Property: Should have a reasonable distribution of confidence values
    let avg_confidence: f32 = confidence_values.iter().sum::<f32>() / confidence_values.len() as f32;
    assert!(
        avg_confidence >= 0.5,
        "Average explanation confidence should be >= 0.5, got {}",
        avg_confidence
    );
    
    println!("✓ Property verified: Explanation confidence values are reasonable (avg: {:.2})", avg_confidence);
}

/// Test that the system maintains consistency under configuration changes
#[test]
fn property_consistent_under_config_changes() {
    let mut engine = AprokoEngine::new();
    
    let ast = AstNode {
        statements: vec![
            Statement::Assignment {
                identifier: "".to_string(),
                value: Expression::Literal(Literal::Integer(42)),
                mutable: false,
            },
        ],
    };
    
    let source = "let  = 42";
    
    // Test with default configuration
    let result1 = engine.analyze(source, &ast).expect("Analysis should succeed");
    
    // Test with modified configuration (higher minimum severity)
    let mut config = AprokoConfig::default();
    config.min_severity = Severity::Error;
    engine.set_config(config).expect("Config update should succeed");
    
    let result2 = engine.analyze(source, &ast).expect("Analysis should succeed");
    
    // Property: Higher severity threshold should result in same or fewer findings
    assert!(
        result2.findings.len() <= result1.findings.len(),
        "Higher severity threshold should not increase finding count"
    );
    
    // Property: All remaining findings should meet the severity threshold
    for finding in &result2.findings {
        assert!(
            finding.severity >= Severity::Error,
            "All findings should meet the minimum severity threshold"
        );
    }
    
    println!("✓ Property verified: System maintains consistency under configuration changes");
}

/// Integration test that runs all property tests
#[test]
fn run_all_aproko_property_tests() {
    println!("Running comprehensive Aproko property tests...\n");
    
    property_all_rules_have_explanations();
    property_diagnostic_categories_consistent();
    property_severity_levels_distributed();
    property_analysis_deterministic();
    property_all_diagnostics_explainable();
    property_fix_suggestions_safe();
    property_statistics_accurate();
    property_handles_edge_cases();
    property_rules_cover_all_categories();
    property_explanation_confidence_reasonable();
    property_consistent_under_config_changes();
    
    println!("\n🎉 All Aproko property tests passed!");
    println!("\nProperty 11: Aproko Diagnostic Completeness - VERIFIED");
    println!("✓ Comprehensive diagnostic coverage");
    println!("✓ Consistent rule-to-explanation mapping");
    println!("✓ Deterministic analysis behavior");
    println!("✓ Safe and actionable fix suggestions");
    println!("✓ Accurate statistical reporting");
    println!("✓ Graceful edge case handling");
    println!("✓ Configuration consistency");
}