//! Rule-based diagnostic engine for structured error and warning categorization
//!
//! This module provides a comprehensive diagnostic system that categorizes compiler
//! messages, provides structured output, and enables rule-based analysis.

use crate::{Finding, Severity, AnalysisCategory, AprokoResult, AprokoError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Unique identifier for diagnostic rules
pub type RuleId = String;

/// Diagnostic categories for structured error reporting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiagnosticCategory {
    /// Syntax errors and grammar violations
    SyntaxError,
    /// Type system violations
    TypeError,
    /// Ownership and borrowing violations
    OwnershipError,
    /// Memory safety violations
    MemoryError,
    /// Logic errors and potential bugs
    LogicError,
    /// Performance warnings and optimization opportunities
    PerformanceWarning,
    /// Security vulnerabilities
    SecurityWarning,
    /// Style and convention violations
    StyleWarning,
    /// Deprecation warnings
    DeprecationWarning,
    /// Informational messages
    Info,
}

/// Structured diagnostic message with categorization and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Unique rule identifier that generated this diagnostic
    pub rule_id: RuleId,
    /// Diagnostic category
    pub category: DiagnosticCategory,
    /// Severity level
    pub severity: Severity,
    /// Primary diagnostic message
    pub message: String,
    /// Optional detailed explanation
    pub explanation: Option<String>,
    /// Suggested fix or improvement
    pub suggestion: Option<String>,
    /// Source location information
    pub location: SourceLocation,
    /// Related locations (for multi-location diagnostics)
    pub related_locations: Vec<RelatedLocation>,
    /// Diagnostic-specific metadata
    pub metadata: HashMap<String, String>,
}

/// Source location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    /// File path (relative to project root)
    pub file: String,
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)
    pub column: usize,
    /// Length of the problematic span
    pub span_length: usize,
    /// Source text excerpt
    pub source_excerpt: Option<String>,
}

/// Related location for multi-location diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedLocation {
    /// Location information
    pub location: SourceLocation,
    /// Relationship to primary diagnostic
    pub relation: String,
    /// Optional message for this location
    pub message: Option<String>,
}

/// Rule definition for diagnostic generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticRule {
    /// Unique rule identifier
    pub id: RuleId,
    /// Human-readable rule name
    pub name: String,
    /// Detailed rule description
    pub description: String,
    /// Diagnostic category this rule produces
    pub category: DiagnosticCategory,
    /// Default severity level
    pub default_severity: Severity,
    /// Whether this rule is enabled by default
    pub enabled: bool,
    /// Rule-specific configuration options
    pub config_options: HashMap<String, RuleConfigOption>,
    /// Examples of code that triggers this rule
    pub examples: Vec<RuleExample>,
}

/// Configuration option for diagnostic rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConfigOption {
    /// Option name
    pub name: String,
    /// Option description
    pub description: String,
    /// Default value
    pub default_value: String,
    /// Possible values (for enum-like options)
    pub possible_values: Option<Vec<String>>,
}

/// Example code that demonstrates a rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleExample {
    /// Example description
    pub description: String,
    /// Code that triggers the rule
    pub code: String,
    /// Expected diagnostic message
    pub expected_message: String,
    /// Whether this is a positive (bad) or negative (good) example
    pub is_violation: bool,
}

/// Configuration for the diagnostic engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticConfig {
    /// Enabled diagnostic categories
    pub enabled_categories: Vec<DiagnosticCategory>,
    /// Minimum severity level to report
    pub min_severity: Severity,
    /// Rule-specific configurations
    pub rule_configs: HashMap<RuleId, RuleConfig>,
    /// Output format preferences
    pub output_format: OutputFormat,
    /// Maximum number of diagnostics to report
    pub max_diagnostics: Option<usize>,
}

/// Configuration for a specific rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConfig {
    /// Whether this rule is enabled
    pub enabled: bool,
    /// Override severity level
    pub severity_override: Option<Severity>,
    /// Rule-specific settings
    pub settings: HashMap<String, String>,
}

/// Output format for diagnostics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputFormat {
    /// Human-readable text format
    Text,
    /// JSON format for tool integration
    Json,
    /// Structured format with colors and formatting
    Rich,
    /// LSP-compatible format
    Lsp,
}

/// The main diagnostic engine
#[derive(Clone)]
pub struct DiagnosticEngine {
    /// Engine configuration
    config: DiagnosticConfig,
    /// Registered diagnostic rules
    rules: HashMap<RuleId, DiagnosticRule>,
    /// Diagnostic statistics
    stats: DiagnosticStats,
}

/// Statistics about diagnostic generation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiagnosticStats {
    /// Total diagnostics generated
    pub total_diagnostics: usize,
    /// Diagnostics by category
    pub by_category: HashMap<DiagnosticCategory, usize>,
    /// Diagnostics by severity
    pub by_severity: HashMap<Severity, usize>,
    /// Diagnostics by rule
    pub by_rule: HashMap<RuleId, usize>,
    /// Analysis duration
    pub analysis_duration_ms: u64,
}

impl DiagnosticEngine {
    /// Create a new diagnostic engine with default configuration
    pub fn new() -> Self {
        Self {
            config: DiagnosticConfig::default(),
            rules: HashMap::new(),
            stats: DiagnosticStats::default(),
        }
    }

    /// Create a diagnostic engine with custom configuration
    pub fn with_config(config: DiagnosticConfig) -> Self {
        let mut engine = Self {
            config,
            rules: HashMap::new(),
            stats: DiagnosticStats::default(),
        };
        engine.register_default_rules();
        engine
    }

    /// Register default diagnostic rules
    fn register_default_rules(&mut self) {
        // Syntax error rules
        self.register_rule(DiagnosticRule {
            id: "E001".to_string(),
            name: "Unexpected Token".to_string(),
            description: "An unexpected token was encountered during parsing".to_string(),
            category: DiagnosticCategory::SyntaxError,
            default_severity: Severity::Error,
            enabled: true,
            config_options: HashMap::new(),
            examples: vec![
                RuleExample {
                    description: "Missing semicolon".to_string(),
                    code: "let x = 5\nlet y = 10".to_string(),
                    expected_message: "Expected ';' after expression".to_string(),
                    is_violation: true,
                }
            ],
        });

        // Type error rules
        self.register_rule(DiagnosticRule {
            id: "E002".to_string(),
            name: "Type Mismatch".to_string(),
            description: "Expression type does not match expected type".to_string(),
            category: DiagnosticCategory::TypeError,
            default_severity: Severity::Error,
            enabled: true,
            config_options: HashMap::new(),
            examples: vec![
                RuleExample {
                    description: "String assigned to number variable".to_string(),
                    code: "let x: i32 = \"hello\"".to_string(),
                    expected_message: "Cannot assign string to integer variable".to_string(),
                    is_violation: true,
                }
            ],
        });

        // Ownership error rules
        self.register_rule(DiagnosticRule {
            id: "E003".to_string(),
            name: "Ownership Violation".to_string(),
            description: "Value used after move or borrow checker violation".to_string(),
            category: DiagnosticCategory::OwnershipError,
            default_severity: Severity::Error,
            enabled: true,
            config_options: HashMap::new(),
            examples: vec![
                RuleExample {
                    description: "Use after move".to_string(),
                    code: "let x = vec![1, 2, 3];\nlet y = x;\nseeAm(x);".to_string(),
                    expected_message: "Value used after move".to_string(),
                    is_violation: true,
                }
            ],
        });

        // Performance warning rules
        self.register_rule(DiagnosticRule {
            id: "W001".to_string(),
            name: "Inefficient Algorithm".to_string(),
            description: "Algorithm could be optimized for better performance".to_string(),
            category: DiagnosticCategory::PerformanceWarning,
            default_severity: Severity::Warning,
            enabled: true,
            config_options: HashMap::new(),
            examples: vec![
                RuleExample {
                    description: "Nested loop with O(n²) complexity".to_string(),
                    code: "for i in 0..n { for j in 0..n { /* work */ } }".to_string(),
                    expected_message: "Consider optimizing nested loop structure".to_string(),
                    is_violation: true,
                }
            ],
        });

        // Security warning rules
        self.register_rule(DiagnosticRule {
            id: "S001".to_string(),
            name: "Unsafe Operation".to_string(),
            description: "Potentially unsafe operation detected".to_string(),
            category: DiagnosticCategory::SecurityWarning,
            default_severity: Severity::Warning,
            enabled: true,
            config_options: HashMap::new(),
            examples: vec![
                RuleExample {
                    description: "Unchecked array access".to_string(),
                    code: "let arr = [1, 2, 3];\nlet val = arr[10];".to_string(),
                    expected_message: "Array access may be out of bounds".to_string(),
                    is_violation: true,
                }
            ],
        });
    }

    /// Register a new diagnostic rule
    pub fn register_rule(&mut self, rule: DiagnosticRule) {
        self.rules.insert(rule.id.clone(), rule);
    }

    /// Generate a diagnostic using a specific rule
    pub fn generate_diagnostic(
        &mut self,
        rule_id: &RuleId,
        message: String,
        location: SourceLocation,
    ) -> AprokoResult<Diagnostic> {
        let rule = self.rules.get(rule_id)
            .ok_or_else(|| AprokoError::Analysis(format!("Unknown rule: {}", rule_id)))?;

        // Check if rule is enabled
        let rule_config = self.config.rule_configs.get(rule_id);
        if let Some(config) = rule_config {
            if !config.enabled {
                return Err(AprokoError::Analysis(format!("Rule {} is disabled", rule_id)));
            }
        } else if !rule.enabled {
            return Err(AprokoError::Analysis(format!("Rule {} is disabled by default", rule_id)));
        }

        // Determine severity (with possible override)
        let severity = rule_config
            .and_then(|c| c.severity_override)
            .unwrap_or(rule.default_severity);

        // Check minimum severity threshold
        if severity < self.config.min_severity {
            return Err(AprokoError::Analysis("Diagnostic below minimum severity threshold".to_string()));
        }

        let diagnostic = Diagnostic {
            rule_id: rule_id.clone(),
            category: rule.category,
            severity,
            message,
            explanation: Some(rule.description.clone()),
            suggestion: None,
            location,
            related_locations: Vec::new(),
            metadata: HashMap::new(),
        };

        // Update statistics
        self.stats.total_diagnostics += 1;
        *self.stats.by_category.entry(rule.category).or_insert(0) += 1;
        *self.stats.by_severity.entry(severity).or_insert(0) += 1;
        *self.stats.by_rule.entry(rule_id.clone()).or_insert(0) += 1;

        Ok(diagnostic)
    }

    /// Generate multiple diagnostics from findings
    pub fn generate_diagnostics_from_findings(&mut self, findings: Vec<Finding>) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        for finding in findings {
            let location = SourceLocation {
                file: "<input>".to_string(),
                line: finding.location.0,
                column: finding.location.1,
                span_length: finding.span_length,
                source_excerpt: None,
            };

            // Try to generate diagnostic using the finding's rule_id
            let diagnostic_result = self.generate_diagnostic(&finding.rule_id, finding.message.clone(), location.clone());
            
            if let Ok(diagnostic) = diagnostic_result {
                diagnostics.push(diagnostic);
            } else {
                // If the rule_id doesn't exist, create a diagnostic directly
                // Map the finding category to a diagnostic category
                let category = match finding.category {
                    AnalysisCategory::Syntax => DiagnosticCategory::SyntaxError,
                    AnalysisCategory::Logic => DiagnosticCategory::LogicError,
                    AnalysisCategory::Performance => DiagnosticCategory::PerformanceWarning,
                    AnalysisCategory::Security => DiagnosticCategory::SecurityWarning,
                    AnalysisCategory::Correctness => DiagnosticCategory::OwnershipError,
                    AnalysisCategory::Style => DiagnosticCategory::StyleWarning,
                };

                let diagnostic = Diagnostic {
                    rule_id: finding.rule_id.clone(),
                    category,
                    severity: finding.severity,
                    message: finding.message,
                    explanation: finding.suggestion.clone(),
                    suggestion: finding.suggestion,
                    location,
                    related_locations: Vec::new(),
                    metadata: HashMap::new(),
                };

                // Update stats
                *self.stats.by_severity.entry(diagnostic.severity).or_insert(0) += 1;
                *self.stats.by_category.entry(diagnostic.category).or_insert(0) += 1;
                *self.stats.by_rule.entry(diagnostic.rule_id.clone()).or_insert(0) += 1;
                self.stats.total_diagnostics += 1;

                diagnostics.push(diagnostic);
            }
        }

        diagnostics
    }

    /// Get all registered rules
    pub fn get_rules(&self) -> &HashMap<RuleId, DiagnosticRule> {
        &self.rules
    }

    /// Get rules by category
    pub fn get_rules_by_category(&self, category: DiagnosticCategory) -> Vec<&DiagnosticRule> {
        self.rules.values()
            .filter(|rule| rule.category == category)
            .collect()
    }

    /// Get diagnostic statistics
    pub fn get_stats(&self) -> &DiagnosticStats {
        &self.stats
    }

    /// Reset diagnostic statistics
    pub fn reset_stats(&mut self) {
        self.stats = DiagnosticStats::default();
    }

    /// Update engine configuration
    pub fn set_config(&mut self, config: DiagnosticConfig) {
        self.config = config;
    }

    /// Get current configuration
    pub fn get_config(&self) -> &DiagnosticConfig {
        &self.config
    }
}

impl Default for DiagnosticConfig {
    fn default() -> Self {
        Self {
            enabled_categories: vec![
                DiagnosticCategory::SyntaxError,
                DiagnosticCategory::TypeError,
                DiagnosticCategory::OwnershipError,
                DiagnosticCategory::MemoryError,
                DiagnosticCategory::LogicError,
                DiagnosticCategory::PerformanceWarning,
                DiagnosticCategory::SecurityWarning,
                DiagnosticCategory::StyleWarning,
                DiagnosticCategory::Info,
            ],
            min_severity: Severity::Info,
            rule_configs: HashMap::new(),
            output_format: OutputFormat::Text,
            max_diagnostics: Some(100),
        }
    }
}

impl Default for DiagnosticEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {} ({}:{}:{})", 
            self.severity, 
            self.message, 
            self.location.file,
            self.location.line, 
            self.location.column
        )
    }
}

impl fmt::Display for DiagnosticCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiagnosticCategory::SyntaxError => write!(f, "syntax-error"),
            DiagnosticCategory::TypeError => write!(f, "type-error"),
            DiagnosticCategory::OwnershipError => write!(f, "ownership-error"),
            DiagnosticCategory::MemoryError => write!(f, "memory-error"),
            DiagnosticCategory::LogicError => write!(f, "logic-error"),
            DiagnosticCategory::PerformanceWarning => write!(f, "performance-warning"),
            DiagnosticCategory::SecurityWarning => write!(f, "security-warning"),
            DiagnosticCategory::StyleWarning => write!(f, "style-warning"),
            DiagnosticCategory::DeprecationWarning => write!(f, "deprecation-warning"),
            DiagnosticCategory::Info => write!(f, "info"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_engine_creation() {
        let engine = DiagnosticEngine::new();
        assert_eq!(engine.get_stats().total_diagnostics, 0);
    }

    #[test]
    fn test_rule_registration() {
        let mut engine = DiagnosticEngine::new();
        let rule = DiagnosticRule {
            id: "TEST001".to_string(),
            name: "Test Rule".to_string(),
            description: "A test rule".to_string(),
            category: DiagnosticCategory::Info,
            default_severity: Severity::Info,
            enabled: true,
            config_options: HashMap::new(),
            examples: Vec::new(),
        };

        engine.register_rule(rule);
        assert!(engine.get_rules().contains_key("TEST001"));
    }

    #[test]
    fn test_diagnostic_generation() {
        let mut engine = DiagnosticEngine::with_config(DiagnosticConfig::default());
        let location = SourceLocation {
            file: "test.ov".to_string(),
            line: 1,
            column: 1,
            span_length: 5,
            source_excerpt: None,
        };

        let result = engine.generate_diagnostic(
            &"E001".to_string(),
            "Test message".to_string(),
            location,
        );

        assert!(result.is_ok());
        let diagnostic = result.unwrap();
        assert_eq!(diagnostic.rule_id, "E001");
        assert_eq!(diagnostic.message, "Test message");
    }

    #[test]
    fn test_diagnostic_categories() {
        assert_eq!(DiagnosticCategory::SyntaxError.to_string(), "syntax-error");
        assert_eq!(DiagnosticCategory::TypeError.to_string(), "type-error");
        assert_eq!(DiagnosticCategory::Info.to_string(), "info");
    }
}

// Include comprehensive tests
#[cfg(test)]
#[path = "diagnostic_tests.rs"]
mod diagnostic_tests;