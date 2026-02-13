//! Tests for the reasoning engine

use super::*;
use crate::reasoning::*;
use crate::diagnostic::{Diagnostic, DiagnosticCategory, SourceLocation};
use std::collections::HashMap;

#[test]
fn test_knowledge_base_initialization() {
    let kb = KnowledgeBase::new();
    assert!(!kb.entries.is_empty(), "Knowledge base should have default entries");
}

#[test]
fn test_knowledge_base_add_entry() {
    let mut kb = KnowledgeBase::new();
    let initial_count = kb.entries.len();
    
    kb.add_entry(KnowledgeEntry {
        id: "test_001".to_string(),
        category: KnowledgeCategory::SyntaxRule,
        content: "Test rule".to_string(),
        related_concepts: vec!["test".to_string()],
        confidence: 0.9,
        source: "Test".to_string(),
    });
    
    assert_eq!(kb.entries.len(), initial_count + 1);
    assert!(kb.get_entry("test_001").is_some());
}

#[test]
fn test_knowledge_base_search() {
    let kb = KnowledgeBase::new();
    
    // Search for something that should exist
    let results = kb.search("semicolon");
    assert!(!results.is_empty(), "Should find entries about semicolons");
    
    // Search for something that shouldn't exist
    let results = kb.search("nonexistent_concept_xyz");
    assert!(results.is_empty(), "Should not find nonexistent concepts");
}

#[test]
fn test_knowledge_base_category_lookup() {
    let kb = KnowledgeBase::new();
    
    let syntax_rules = kb.get_by_category(KnowledgeCategory::SyntaxRule);
    assert!(!syntax_rules.is_empty(), "Should have syntax rules");
    
    let type_rules = kb.get_by_category(KnowledgeCategory::TypeRule);
    assert!(!type_rules.is_empty(), "Should have type rules");
}

#[test]
fn test_knowledge_base_concept_lookup() {
    let kb = KnowledgeBase::new();
    
    let syntax_entries = kb.get_by_concept("syntax");
    assert!(!syntax_entries.is_empty(), "Should have entries related to syntax");
}

#[test]
fn test_inference_engine_creation() {
    let engine = InferenceEngine::new();
    assert!(!engine.knowledge_base().entries.is_empty());
    assert!(engine.get_reasoning_chains().is_empty());
}

#[test]
fn test_inference_engine_diagnostic_reasoning() {
    let mut engine = InferenceEngine::new();
    
    let diagnostic = Diagnostic {
        rule_id: "test_rule".to_string(),
        category: DiagnosticCategory::SyntaxError,
        severity: Severity::Error,
        message: "Missing semicolon at end of statement".to_string(),
        explanation: None,
        suggestion: Some("Add a semicolon".to_string()),
        location: SourceLocation {
            file: "test.ov".to_string(),
            line: 10,
            column: 15,
            span_length: 1,
            source_excerpt: Some("let x = 5".to_string()),
        },
        related_locations: vec![],
        metadata: HashMap::new(),
    };
    
    let result = engine.reason_about_diagnostic(&diagnostic);
    assert!(result.is_ok(), "Should successfully reason about diagnostic");
    
    let chain = result.unwrap();
    assert!(!chain.steps.is_empty(), "Should have reasoning steps");
    assert!(!chain.conclusion.is_empty(), "Should have a conclusion");
    assert!(chain.overall_confidence > 0.0, "Should have confidence > 0");
    assert!(chain.overall_confidence <= 1.0, "Should have confidence <= 1");
}

#[test]
fn test_inference_engine_finding_reasoning() {
    let mut engine = InferenceEngine::new();
    
    let finding = Finding {
        category: AnalysisCategory::Performance,
        severity: Severity::Warning,
        message: "Nested loop detected".to_string(),
        suggestion: Some("Consider using a hash table".to_string()),
        location: (20, 5),
        span_length: 10,
        rule_id: "perf_001".to_string(),
    };
    
    let result = engine.reason_about_finding(&finding);
    assert!(result.is_ok(), "Should successfully reason about finding");
    
    let chain = result.unwrap();
    assert!(!chain.steps.is_empty(), "Should have reasoning steps");
    assert!(!chain.conclusion.is_empty(), "Should have a conclusion");
}

#[test]
fn test_reasoning_engine_creation() {
    let engine = ReasoningEngine::new();
    assert!(engine.inference_engine().get_reasoning_chains().is_empty());
}

#[test]
fn test_reasoning_engine_explain_diagnostic() {
    let mut engine = ReasoningEngine::new();
    
    let diagnostic = Diagnostic {
        rule_id: "type_001".to_string(),
        category: DiagnosticCategory::TypeError,
        severity: Severity::Error,
        message: "Type mismatch: expected i32, found String".to_string(),
        explanation: None,
        suggestion: Some("Change the type annotation or convert the value".to_string()),
        location: SourceLocation {
            file: "test.ov".to_string(),
            line: 5,
            column: 10,
            span_length: 6,
            source_excerpt: Some("let x: i32 = \"hello\"".to_string()),
        },
        related_locations: vec![],
        metadata: HashMap::new(),
    };
    
    let result = engine.explain_diagnostic(&diagnostic);
    assert!(result.is_ok(), "Should successfully explain diagnostic");
    
    let (explanation, chain) = result.unwrap();
    assert!(!explanation.detailed_explanation.is_empty(), "Should have detailed explanation");
    assert!(!chain.steps.is_empty(), "Should have reasoning steps");
    assert!(explanation.confidence > 0.0, "Should have confidence");
}

#[test]
fn test_reasoning_engine_explain_finding() {
    let mut engine = ReasoningEngine::new();
    
    let finding = Finding {
        category: AnalysisCategory::Security,
        severity: Severity::Critical,
        message: "Unchecked array access detected".to_string(),
        suggestion: Some("Use bounds checking with .get() method".to_string()),
        location: (15, 8),
        span_length: 8,
        rule_id: "sec_001".to_string(),
    };
    
    let result = engine.explain_finding(&finding);
    assert!(result.is_ok(), "Should successfully explain finding");
    
    let (explanation, chain) = result.unwrap();
    assert!(!explanation.detailed_explanation.is_empty(), "Should have detailed explanation");
    assert!(!chain.steps.is_empty(), "Should have reasoning steps");
}

#[test]
fn test_reasoning_chain_structure() {
    let mut engine = InferenceEngine::new();
    
    let diagnostic = Diagnostic {
        rule_id: "test".to_string(),
        category: DiagnosticCategory::SyntaxError,
        severity: Severity::Error,
        message: "Test error".to_string(),
        explanation: None,
        suggestion: None,
        location: SourceLocation {
            file: "test.ov".to_string(),
            line: 1,
            column: 1,
            span_length: 1,
            source_excerpt: None,
        },
        related_locations: vec![],
        metadata: HashMap::new(),
    };
    
    let chain = engine.reason_about_diagnostic(&diagnostic).unwrap();
    
    // Verify chain structure
    assert!(!chain.id.is_empty(), "Chain should have an ID");
    assert!(!chain.question.is_empty(), "Chain should have a question");
    assert!(!chain.conclusion.is_empty(), "Chain should have a conclusion");
    
    // Verify steps are numbered correctly
    for (i, step) in chain.steps.iter().enumerate() {
        assert_eq!(step.step_number, i + 1, "Steps should be numbered sequentially");
        assert!(!step.description.is_empty(), "Each step should have a description");
        assert!(!step.conclusion.is_empty(), "Each step should have a conclusion");
    }
}

#[test]
fn test_reasoning_config() {
    let config = ReasoningConfig::default();
    assert_eq!(config.max_reasoning_depth, 10);
    assert_eq!(config.min_confidence_threshold, 0.5);
    assert!(config.track_all_chains);
    
    let custom_config = ReasoningConfig {
        max_reasoning_depth: 5,
        min_confidence_threshold: 0.7,
        track_all_chains: false,
    };
    
    let mut engine = ReasoningEngine::with_config(custom_config.clone());
    assert_eq!(engine.config().max_reasoning_depth, 5);
    assert_eq!(engine.config().min_confidence_threshold, 0.7);
    assert!(!engine.config().track_all_chains);
}

#[test]
fn test_knowledge_categories() {
    let kb = KnowledgeBase::new();
    
    // Verify all categories have entries
    let categories = vec![
        KnowledgeCategory::SyntaxRule,
        KnowledgeCategory::TypeRule,
        KnowledgeCategory::BestPractice,
        KnowledgeCategory::PerformanceGuideline,
        KnowledgeCategory::SecurityGuideline,
        KnowledgeCategory::CommonPattern,
        KnowledgeCategory::AntiPattern,
    ];
    
    for category in categories {
        let entries = kb.get_by_category(category);
        assert!(!entries.is_empty(), "Category {:?} should have entries", category);
    }
}

#[test]
fn test_reasoning_step_confidence() {
    let mut engine = InferenceEngine::new();
    
    let diagnostic = Diagnostic {
        rule_id: "test".to_string(),
        category: DiagnosticCategory::SyntaxError,
        severity: Severity::Error,
        message: "semicolon".to_string(), // Should match knowledge base
        explanation: None,
        suggestion: None,
        location: SourceLocation {
            file: "test.ov".to_string(),
            line: 1,
            column: 1,
            span_length: 1,
            source_excerpt: None,
        },
        related_locations: vec![],
        metadata: HashMap::new(),
    };
    
    let chain = engine.reason_about_diagnostic(&diagnostic).unwrap();
    
    // All steps should have valid confidence values
    for step in &chain.steps {
        assert!(step.confidence >= 0.0, "Confidence should be >= 0");
        assert!(step.confidence <= 1.0, "Confidence should be <= 1");
    }
    
    // Overall confidence should be valid
    assert!(chain.overall_confidence >= 0.0, "Overall confidence should be >= 0");
    assert!(chain.overall_confidence <= 1.0, "Overall confidence should be <= 1");
}
