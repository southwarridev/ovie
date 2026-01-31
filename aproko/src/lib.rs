//! Aproko - The Ovie Programming Language Assistant Engine
//!
//! Aproko provides real-time code analysis and assistance for Ovie programs,
//! offering guidance across six categories: syntax, logic, performance, 
//! security, correctness, and style.

pub mod analyzers;
pub mod diagnostic;
pub mod explanation;
#[cfg(test)]
mod integration_tests;
#[cfg(test)]
mod property_tests;

use analyzers::{SyntaxAnalyzer, LogicAnalyzer, PerformanceAnalyzer, SecurityAnalyzer, CorrectnessAnalyzer, StyleAnalyzer};
use diagnostic::{DiagnosticEngine, Diagnostic};
use explanation::ExplanationEngine;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur during analysis
#[derive(Error, Debug)]
pub enum AprokoError {
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Analysis error: {0}")]
    Analysis(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type AprokoResult<T> = Result<T, AprokoError>;

/// Analysis categories supported by Aproko
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnalysisCategory {
    /// Grammar compliance and syntax correctness
    Syntax,
    /// Control flow and logical correctness
    Logic,
    /// Algorithmic complexity and optimization opportunities
    Performance,
    /// Security vulnerabilities and unsafe operations
    Security,
    /// Ownership rules and memory safety
    Correctness,
    /// Code style and best practices
    Style,
}

/// Severity levels for analysis findings
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Severity {
    /// Informational suggestions
    Info,
    /// Style and best practice recommendations
    Warning,
    /// Logic errors and potential bugs
    Error,
    /// Critical security or safety issues
    Critical,
}

/// A specific analysis finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    /// Category of analysis that produced this finding
    pub category: AnalysisCategory,
    /// Severity level
    pub severity: Severity,
    /// Human-readable message
    pub message: String,
    /// Specific suggestion for improvement
    pub suggestion: Option<String>,
    /// Source location (line, column)
    pub location: (usize, usize),
    /// Length of the problematic code span
    pub span_length: usize,
    /// Rule or pattern that was violated
    pub rule_id: String,
}

/// Configuration for Aproko analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AprokoConfig {
    /// Enabled analysis categories
    pub enabled_categories: Vec<AnalysisCategory>,
    /// Minimum severity level to report
    pub min_severity: Severity,
    /// Category-specific configurations
    pub category_configs: HashMap<AnalysisCategory, CategoryConfig>,
    /// Custom rules and patterns
    pub custom_rules: Vec<CustomRule>,
}

/// Configuration for a specific analysis category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryConfig {
    /// Whether this category is enabled
    pub enabled: bool,
    /// Category-specific settings
    pub settings: HashMap<String, String>,
}

/// Custom analysis rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRule {
    /// Unique identifier for the rule
    pub id: String,
    /// Human-readable description
    pub description: String,
    /// Pattern to match (implementation-specific)
    pub pattern: String,
    /// Suggested fix or improvement
    pub suggestion: String,
    /// Severity level for violations
    pub severity: Severity,
}

impl Default for AprokoConfig {
    fn default() -> Self {
        let mut category_configs = HashMap::new();
        
        // Default configurations for each category
        for category in [
            AnalysisCategory::Syntax,
            AnalysisCategory::Logic,
            AnalysisCategory::Performance,
            AnalysisCategory::Security,
            AnalysisCategory::Correctness,
            AnalysisCategory::Style,
        ] {
            category_configs.insert(category, CategoryConfig {
                enabled: true,
                settings: HashMap::new(),
            });
        }

        Self {
            enabled_categories: vec![
                AnalysisCategory::Syntax,
                AnalysisCategory::Logic,
                AnalysisCategory::Performance,
                AnalysisCategory::Security,
                AnalysisCategory::Correctness,
                AnalysisCategory::Style,
            ],
            min_severity: Severity::Info,
            category_configs,
            custom_rules: Vec::new(),
        }
    }
}

/// Results of an analysis run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResults {
    /// All findings discovered during analysis
    pub findings: Vec<Finding>,
    /// Structured diagnostics from the diagnostic engine
    pub diagnostics: Vec<Diagnostic>,
    /// Analysis statistics
    pub stats: AnalysisStats,
    /// Configuration used for this analysis
    pub config: AprokoConfig,
}

/// Statistics about an analysis run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisStats {
    /// Total number of findings by severity
    pub findings_by_severity: HashMap<Severity, usize>,
    /// Total number of findings by category
    pub findings_by_category: HashMap<AnalysisCategory, usize>,
    /// Analysis duration in milliseconds
    pub duration_ms: u64,
    /// Number of lines analyzed
    pub lines_analyzed: usize,
}

/// The main Aproko analysis engine
pub struct AprokoEngine {
    config: AprokoConfig,
    analyzers: HashMap<AnalysisCategory, Box<dyn Analyzer>>,
    diagnostic_engine: DiagnosticEngine,
    explanation_engine: ExplanationEngine,
}

/// Trait for category-specific analyzers
pub trait Analyzer: Send + Sync {
    /// Analyze the given source code and AST
    fn analyze(&self, source: &str, ast: &oviec::ast::AstNode) -> AprokoResult<Vec<Finding>>;
    
    /// Get the category this analyzer handles
    fn category(&self) -> AnalysisCategory;
    
    /// Get analyzer-specific configuration
    fn configure(&mut self, config: &CategoryConfig) -> AprokoResult<()>;
}

impl AprokoEngine {
    /// Create a new Aproko engine with default configuration
    pub fn new() -> Self {
        Self::with_config(AprokoConfig::default())
    }

    /// Create a new Aproko engine with the given configuration
    pub fn with_config(config: AprokoConfig) -> Self {
        let mut engine = Self {
            config,
            analyzers: HashMap::new(),
            diagnostic_engine: DiagnosticEngine::new(),
            explanation_engine: ExplanationEngine::new(),
        };
        
        // Register default analyzers
        engine.register_default_analyzers();
        engine
    }

    /// Register all default analyzers
    fn register_default_analyzers(&mut self) {
        self.register_analyzer(Box::new(SyntaxAnalyzer::new()));
        self.register_analyzer(Box::new(LogicAnalyzer::new()));
        self.register_analyzer(Box::new(PerformanceAnalyzer::new()));
        self.register_analyzer(Box::new(SecurityAnalyzer::new()));
        self.register_analyzer(Box::new(CorrectnessAnalyzer::new()));
        self.register_analyzer(Box::new(StyleAnalyzer::new()));
    }

    /// Register a custom analyzer
    pub fn register_analyzer(&mut self, analyzer: Box<dyn Analyzer>) {
        let category = analyzer.category();
        self.analyzers.insert(category, analyzer);
    }

    /// Load configuration from a TOML file
    pub fn load_config_from_file(&mut self, path: &std::path::Path) -> AprokoResult<()> {
        let content = std::fs::read_to_string(path)?;
        let config: AprokoConfig = toml::from_str(&content)
            .map_err(|e| AprokoError::Config(format!("Failed to parse config: {}", e)))?;
        
        self.config = config;
        self.configure_analyzers()?;
        Ok(())
    }

    /// Configure all registered analyzers with current config
    fn configure_analyzers(&mut self) -> AprokoResult<()> {
        for (category, analyzer) in &mut self.analyzers {
            if let Some(category_config) = self.config.category_configs.get(category) {
                analyzer.configure(category_config)?;
            }
        }
        Ok(())
    }

    /// Analyze the given source code and AST
    pub fn analyze(&self, source: &str, ast: &oviec::ast::AstNode) -> AprokoResult<AnalysisResults> {
        let start_time = std::time::Instant::now();
        let mut all_findings = Vec::new();
        let mut findings_by_severity = HashMap::new();
        let mut findings_by_category = HashMap::new();

        // Run analysis for each enabled category
        for category in &self.config.enabled_categories {
            if let Some(analyzer) = self.analyzers.get(category) {
                if let Some(category_config) = self.config.category_configs.get(category) {
                    if category_config.enabled {
                        let findings = analyzer.analyze(source, ast)?;
                        
                        // Filter by minimum severity
                        let filtered_findings: Vec<_> = findings
                            .into_iter()
                            .filter(|f| f.severity >= self.config.min_severity)
                            .collect();

                        // Update statistics
                        for finding in &filtered_findings {
                            *findings_by_severity.entry(finding.severity).or_insert(0) += 1;
                            *findings_by_category.entry(finding.category).or_insert(0) += 1;
                        }

                        all_findings.extend(filtered_findings);
                    }
                }
            }
        }

        // Generate structured diagnostics from findings
        let mut diagnostic_engine = self.diagnostic_engine.clone();
        let diagnostics = diagnostic_engine.generate_diagnostics_from_findings(all_findings.clone());

        let duration = start_time.elapsed();
        let lines_analyzed = source.lines().count();

        Ok(AnalysisResults {
            findings: all_findings,
            diagnostics,
            stats: AnalysisStats {
                findings_by_severity,
                findings_by_category,
                duration_ms: duration.as_millis() as u64,
                lines_analyzed,
            },
            config: self.config.clone(),
        })
    }

    /// Get the current configuration
    pub fn config(&self) -> &AprokoConfig {
        &self.config
    }

    /// Update the configuration
    pub fn set_config(&mut self, config: AprokoConfig) -> AprokoResult<()> {
        self.config = config;
        self.configure_analyzers()
    }

    /// Get the diagnostic engine
    pub fn diagnostic_engine(&self) -> &DiagnosticEngine {
        &self.diagnostic_engine
    }

    /// Get a mutable reference to the diagnostic engine
    pub fn diagnostic_engine_mut(&mut self) -> &mut DiagnosticEngine {
        &mut self.diagnostic_engine
    }

    /// Get the explanation engine
    pub fn explanation_engine(&self) -> &ExplanationEngine {
        &self.explanation_engine
    }

    /// Get a mutable reference to the explanation engine
    pub fn explanation_engine_mut(&mut self) -> &mut ExplanationEngine {
        &mut self.explanation_engine
    }

    /// Get detailed explanation for a diagnostic
    pub fn explain_diagnostic(&self, diagnostic: &Diagnostic) -> AprokoResult<explanation::Explanation> {
        self.explanation_engine.explain_diagnostic(diagnostic)
    }

    /// Get detailed explanation for a finding
    pub fn explain_finding(&self, finding: &Finding) -> AprokoResult<explanation::Explanation> {
        self.explanation_engine.explain_finding(finding)
    }
}

impl Default for AprokoEngine {
    fn default() -> Self {
        Self::new()
    }
}

// Re-export commonly used types
pub use oviec;
pub use diagnostic::{DiagnosticEngine, Diagnostic, DiagnosticCategory, DiagnosticRule, SourceLocation};
pub use explanation::{ExplanationEngine, Explanation, ExplanationType, FixSuggestion, CodeExample};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AprokoConfig::default();
        assert_eq!(config.enabled_categories.len(), 6);
        assert_eq!(config.min_severity, Severity::Info);
        assert!(config.category_configs.contains_key(&AnalysisCategory::Syntax));
    }

    #[test]
    fn test_engine_creation() {
        let engine = AprokoEngine::new();
        assert_eq!(engine.config().enabled_categories.len(), 6);
    }

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Info < Severity::Warning);
        assert!(Severity::Warning < Severity::Error);
        assert!(Severity::Error < Severity::Critical);
    }
}
