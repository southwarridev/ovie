//! Comprehensive Error Handling and Reporting for the Ovie Compiler
//! 
//! This module provides enterprise-grade error reporting with clear, actionable
//! error messages, specific suggestions for error resolution, error categorization
//! and codes, and IDE integration support.

use thiserror::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result type for Ovie operations
pub type OvieResult<T> = Result<T, OvieError>;

/// Error severity levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// Fatal errors that prevent compilation
    Error,
    /// Warnings that should be addressed
    Warning,
    /// Informational messages
    Info,
    /// Hints for improvement
    Hint,
}

/// Error categories for better organization
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorCategory {
    /// Syntax and lexical errors
    Syntax,
    /// Type system errors
    Type,
    /// Semantic analysis errors
    Semantic,
    /// Runtime errors
    Runtime,
    /// IO and file system errors
    Io,
    /// Code generation errors
    Codegen,
    /// Package management errors
    Package,
    /// Security and safety errors
    Security,
    /// Performance warnings
    Performance,
    /// Style and formatting issues
    Style,
}

/// Structured error suggestion
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorSuggestion {
    /// Human-readable suggestion message
    pub message: String,
    /// Optional code fix that can be applied
    pub code_fix: Option<CodeFix>,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f32,
}

/// Code fix that can be automatically applied
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeFix {
    /// Description of the fix
    pub description: String,
    /// Text replacements to apply
    pub replacements: Vec<TextReplacement>,
}

/// Text replacement for code fixes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextReplacement {
    /// Start position in the source
    pub start: SourcePosition,
    /// End position in the source
    pub end: SourcePosition,
    /// New text to insert
    pub new_text: String,
}

/// Enhanced source position with file information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourcePosition {
    /// File path (optional)
    pub file: Option<String>,
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)
    pub column: usize,
    /// Byte offset in the source
    pub offset: usize,
}

/// Comprehensive diagnostic information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Unique error code
    pub code: String,
    /// Error severity
    pub severity: ErrorSeverity,
    /// Error category
    pub category: ErrorCategory,
    /// Primary error message
    pub message: String,
    /// Source location where the error occurred
    pub location: SourcePosition,
    /// Optional secondary locations (for related errors)
    pub related_locations: Vec<(SourcePosition, String)>,
    /// Suggestions for fixing the error
    pub suggestions: Vec<ErrorSuggestion>,
    /// Additional context information
    pub context: HashMap<String, String>,
    /// Help URL for more information
    pub help_url: Option<String>,
}

/// Main error type for the Ovie compiler
#[derive(Error, Debug, Clone, PartialEq)]
pub enum OvieError {
    /// Comprehensive diagnostic error
    #[error("{}", .diagnostic.message)]
    Diagnostic {
        diagnostic: Diagnostic,
    },

    /// Legacy error types (for backward compatibility)
    #[error("Lexer error at line {line}, column {column}: {message}")]
    LexError {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("Parse error at line {line}, column {column}: {message}")]
    ParseError {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("Semantic error at line {line}, column {column}: {message}")]
    SemanticError {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("Runtime error: {message}")]
    RuntimeError {
        message: String,
    },

    #[error("IO error: {message}")]
    IoError {
        message: String,
    },

    #[error("IR error: {message}")]
    IrError {
        message: String,
    },

    #[error("Codegen error: {message}")]
    CodegenError {
        message: String,
    },

    #[error("Error: {message}")]
    Generic {
        message: String,
    },
}

impl OvieError {
    /// Create a comprehensive diagnostic error
    pub fn diagnostic(diagnostic: Diagnostic) -> Self {
        Self::Diagnostic { diagnostic }
    }

    /// Create a new lexer error with suggestions
    pub fn lex_error_with_suggestions(
        line: usize, 
        column: usize, 
        message: impl Into<String>,
        suggestions: Vec<ErrorSuggestion>
    ) -> Self {
        let diagnostic = Diagnostic {
            code: "E0001".to_string(),
            severity: ErrorSeverity::Error,
            category: ErrorCategory::Syntax,
            message: message.into(),
            location: SourcePosition {
                file: None,
                line,
                column,
                offset: 0,
            },
            related_locations: Vec::new(),
            suggestions,
            context: HashMap::new(),
            help_url: Some("https://ovie-lang.org/docs/errors/E0001".to_string()),
        };
        Self::Diagnostic { diagnostic }
    }

    /// Create a new parse error with suggestions
    pub fn parse_error_with_suggestions(
        line: usize, 
        column: usize, 
        message: impl Into<String>,
        suggestions: Vec<ErrorSuggestion>
    ) -> Self {
        let diagnostic = Diagnostic {
            code: "E0002".to_string(),
            severity: ErrorSeverity::Error,
            category: ErrorCategory::Syntax,
            message: message.into(),
            location: SourcePosition {
                file: None,
                line,
                column,
                offset: 0,
            },
            related_locations: Vec::new(),
            suggestions,
            context: HashMap::new(),
            help_url: Some("https://ovie-lang.org/docs/errors/E0002".to_string()),
        };
        Self::Diagnostic { diagnostic }
    }

    /// Create a type error with suggestions
    pub fn type_error(
        line: usize,
        column: usize,
        expected: &str,
        found: &str,
        suggestions: Vec<ErrorSuggestion>
    ) -> Self {
        let message = format!("Type mismatch: expected {}, found {}", expected, found);
        let mut context = HashMap::new();
        context.insert("expected_type".to_string(), expected.to_string());
        context.insert("found_type".to_string(), found.to_string());

        let diagnostic = Diagnostic {
            code: "E0003".to_string(),
            severity: ErrorSeverity::Error,
            category: ErrorCategory::Type,
            message,
            location: SourcePosition {
                file: None,
                line,
                column,
                offset: 0,
            },
            related_locations: Vec::new(),
            suggestions,
            context,
            help_url: Some("https://ovie-lang.org/docs/errors/E0003".to_string()),
        };
        Self::Diagnostic { diagnostic }
    }

    /// Create a security error with suggestions
    pub fn security_error(
        line: usize,
        column: usize,
        operation: &str,
        suggestions: Vec<ErrorSuggestion>
    ) -> Self {
        let message = format!("Unsafe operation detected: {}", operation);
        let mut context = HashMap::new();
        context.insert("unsafe_operation".to_string(), operation.to_string());

        let diagnostic = Diagnostic {
            code: "E0004".to_string(),
            severity: ErrorSeverity::Warning,
            category: ErrorCategory::Security,
            message,
            location: SourcePosition {
                file: None,
                line,
                column,
                offset: 0,
            },
            related_locations: Vec::new(),
            suggestions,
            context,
            help_url: Some("https://ovie-lang.org/docs/errors/E0004".to_string()),
        };
        Self::Diagnostic { diagnostic }
    }

    /// Legacy error constructors (for backward compatibility)
    pub fn lex_error(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::LexError {
            line,
            column,
            message: message.into(),
        }
    }

    pub fn parse_error(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::ParseError {
            line,
            column,
            message: message.into(),
        }
    }

    pub fn semantic_error(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::SemanticError {
            line,
            column,
            message: message.into(),
        }
    }

    pub fn runtime_error(message: impl Into<String>) -> Self {
        Self::RuntimeError {
            message: message.into(),
        }
    }

    pub fn io_error(message: impl Into<String>) -> Self {
        Self::IoError {
            message: message.into(),
        }
    }

    pub fn ir_error(message: impl Into<String>) -> Self {
        Self::IrError {
            message: message.into(),
        }
    }

    pub fn codegen_error(message: impl Into<String>) -> Self {
        Self::CodegenError {
            message: message.into(),
        }
    }

    pub fn generic(message: impl Into<String>) -> Self {
        Self::Generic {
            message: message.into(),
        }
    }

    /// Get the diagnostic information if available
    pub fn get_diagnostic(&self) -> Option<&Diagnostic> {
        match self {
            Self::Diagnostic { diagnostic } => Some(diagnostic),
            _ => None,
        }
    }

    /// Convert legacy errors to diagnostics
    pub fn to_diagnostic(&self) -> Diagnostic {
        match self {
            Self::Diagnostic { diagnostic } => diagnostic.clone(),
            Self::LexError { line, column, message } => Diagnostic {
                code: "E0001".to_string(),
                severity: ErrorSeverity::Error,
                category: ErrorCategory::Syntax,
                message: message.clone(),
                location: SourcePosition {
                    file: None,
                    line: *line,
                    column: *column,
                    offset: 0,
                },
                related_locations: Vec::new(),
                suggestions: Vec::new(),
                context: HashMap::new(),
                help_url: Some("https://ovie-lang.org/docs/errors/E0001".to_string()),
            },
            Self::ParseError { line, column, message } => Diagnostic {
                code: "E0002".to_string(),
                severity: ErrorSeverity::Error,
                category: ErrorCategory::Syntax,
                message: message.clone(),
                location: SourcePosition {
                    file: None,
                    line: *line,
                    column: *column,
                    offset: 0,
                },
                related_locations: Vec::new(),
                suggestions: Vec::new(),
                context: HashMap::new(),
                help_url: Some("https://ovie-lang.org/docs/errors/E0002".to_string()),
            },
            Self::SemanticError { line, column, message } => Diagnostic {
                code: "E0005".to_string(),
                severity: ErrorSeverity::Error,
                category: ErrorCategory::Semantic,
                message: message.clone(),
                location: SourcePosition {
                    file: None,
                    line: *line,
                    column: *column,
                    offset: 0,
                },
                related_locations: Vec::new(),
                suggestions: Vec::new(),
                context: HashMap::new(),
                help_url: Some("https://ovie-lang.org/docs/errors/E0005".to_string()),
            },
            Self::RuntimeError { message } => Diagnostic {
                code: "E0006".to_string(),
                severity: ErrorSeverity::Error,
                category: ErrorCategory::Runtime,
                message: message.clone(),
                location: SourcePosition {
                    file: None,
                    line: 1,
                    column: 1,
                    offset: 0,
                },
                related_locations: Vec::new(),
                suggestions: Vec::new(),
                context: HashMap::new(),
                help_url: Some("https://ovie-lang.org/docs/errors/E0006".to_string()),
            },
            Self::IoError { message } => Diagnostic {
                code: "E0007".to_string(),
                severity: ErrorSeverity::Error,
                category: ErrorCategory::Io,
                message: message.clone(),
                location: SourcePosition {
                    file: None,
                    line: 1,
                    column: 1,
                    offset: 0,
                },
                related_locations: Vec::new(),
                suggestions: Vec::new(),
                context: HashMap::new(),
                help_url: Some("https://ovie-lang.org/docs/errors/E0007".to_string()),
            },
            Self::IrError { message } => Diagnostic {
                code: "E0008".to_string(),
                severity: ErrorSeverity::Error,
                category: ErrorCategory::Codegen,
                message: message.clone(),
                location: SourcePosition {
                    file: None,
                    line: 1,
                    column: 1,
                    offset: 0,
                },
                related_locations: Vec::new(),
                suggestions: Vec::new(),
                context: HashMap::new(),
                help_url: Some("https://ovie-lang.org/docs/errors/E0008".to_string()),
            },
            Self::CodegenError { message } => Diagnostic {
                code: "E0009".to_string(),
                severity: ErrorSeverity::Error,
                category: ErrorCategory::Codegen,
                message: message.clone(),
                location: SourcePosition {
                    file: None,
                    line: 1,
                    column: 1,
                    offset: 0,
                },
                related_locations: Vec::new(),
                suggestions: Vec::new(),
                context: HashMap::new(),
                help_url: Some("https://ovie-lang.org/docs/errors/E0009".to_string()),
            },
            Self::Generic { message } => Diagnostic {
                code: "E0010".to_string(),
                severity: ErrorSeverity::Error,
                category: ErrorCategory::Syntax,
                message: message.clone(),
                location: SourcePosition {
                    file: None,
                    line: 1,
                    column: 1,
                    offset: 0,
                },
                related_locations: Vec::new(),
                suggestions: Vec::new(),
                context: HashMap::new(),
                help_url: Some("https://ovie-lang.org/docs/errors/E0010".to_string()),
            },
        }
    }
}

/// Error reporter for IDE integration and batch reporting
#[derive(Debug, Clone)]
pub struct ErrorReporter {
    /// Collected diagnostics
    diagnostics: Vec<Diagnostic>,
    /// Maximum number of errors to collect
    max_errors: usize,
}

impl ErrorReporter {
    /// Create a new error reporter
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
            max_errors: 100,
        }
    }

    /// Create a new error reporter with custom max errors
    pub fn with_max_errors(max_errors: usize) -> Self {
        Self {
            diagnostics: Vec::new(),
            max_errors,
        }
    }

    /// Add a diagnostic
    pub fn add_diagnostic(&mut self, diagnostic: Diagnostic) {
        if self.diagnostics.len() < self.max_errors {
            self.diagnostics.push(diagnostic);
        }
    }

    /// Add an error
    pub fn add_error(&mut self, error: OvieError) {
        self.add_diagnostic(error.to_diagnostic());
    }

    /// Get all diagnostics
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Get error count
    pub fn error_count(&self) -> usize {
        self.diagnostics.iter()
            .filter(|d| d.severity == ErrorSeverity::Error)
            .count()
    }

    /// Get warning count
    pub fn warning_count(&self) -> usize {
        self.diagnostics.iter()
            .filter(|d| d.severity == ErrorSeverity::Warning)
            .count()
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        self.error_count() > 0
    }

    /// Clear all diagnostics
    pub fn clear(&mut self) {
        self.diagnostics.clear();
    }

    /// Generate IDE-compatible diagnostic report (JSON format)
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.diagnostics)
    }

    /// Generate AI-friendly structured feedback (JSON format for LLM consumption)
    pub fn to_ai_friendly_json(&self) -> Result<String, serde_json::Error> {
        let ai_feedback: Vec<_> = self.diagnostics.iter().map(|diagnostic| {
            serde_json::json!({
                "error_code": diagnostic.code,
                "severity": format!("{:?}", diagnostic.severity).to_lowercase(),
                "category": format!("{:?}", diagnostic.category).to_lowercase(),
                "message": diagnostic.message,
                "location": {
                    "file": diagnostic.location.file,
                    "line": diagnostic.location.line,
                    "column": diagnostic.location.column
                },
                "suggestions": diagnostic.suggestions.iter().map(|s| {
                    serde_json::json!({
                        "message": s.message,
                        "confidence": s.confidence,
                        "has_code_fix": s.code_fix.is_some()
                    })
                }).collect::<Vec<_>>(),
                "context": diagnostic.context,
                "help_url": diagnostic.help_url,
                "training_data": {
                    "error_pattern": self.extract_error_pattern(diagnostic),
                    "fix_pattern": self.extract_fix_pattern(diagnostic),
                    "common_causes": self.get_common_causes(&diagnostic.code),
                    "learning_objective": self.get_learning_objective(&diagnostic.category)
                }
            })
        }).collect();

        serde_json::to_string_pretty(&serde_json::json!({
            "diagnostics": ai_feedback,
            "summary": {
                "total_errors": self.error_count(),
                "total_warnings": self.warning_count(),
                "categories": self.get_category_summary(),
                "complexity_score": self.calculate_complexity_score(),
                "learning_opportunities": self.identify_learning_opportunities()
            }
        }))
    }

    /// Extract error pattern for AI training
    fn extract_error_pattern(&self, diagnostic: &Diagnostic) -> serde_json::Value {
        serde_json::json!({
            "error_type": diagnostic.category,
            "common_triggers": match diagnostic.code.as_str() {
                "E0001" => vec!["missing_token", "invalid_character", "unexpected_symbol"],
                "E0002" => vec!["missing_semicolon", "unmatched_brackets", "invalid_syntax"],
                "E0003" => vec!["type_mismatch", "incompatible_types", "missing_conversion"],
                "E0004" => vec!["unsafe_operation", "security_violation", "unverified_access"],
                _ => vec!["unknown_pattern"]
            },
            "resolution_strategy": match diagnostic.category {
                ErrorCategory::Syntax => "syntax_correction",
                ErrorCategory::Type => "type_annotation_or_conversion",
                ErrorCategory::Security => "explicit_unsafe_or_safe_alternative",
                _ => "general_debugging"
            }
        })
    }

    /// Extract fix pattern for AI training
    fn extract_fix_pattern(&self, diagnostic: &Diagnostic) -> serde_json::Value {
        let fix_types: Vec<_> = diagnostic.suggestions.iter().map(|s| {
            if s.code_fix.is_some() {
                "automated_fix"
            } else if s.confidence > 0.8 {
                "high_confidence_suggestion"
            } else {
                "general_guidance"
            }
        }).collect();

        serde_json::json!({
            "fix_types": fix_types,
            "fix_complexity": if diagnostic.suggestions.is_empty() { "complex" } else { "simple" },
            "requires_context": !diagnostic.context.is_empty()
        })
    }

    /// Get common causes for error codes
    fn get_common_causes(&self, error_code: &str) -> Vec<&'static str> {
        match error_code {
            "E0001" => vec!["typo", "missing_character", "encoding_issue"],
            "E0002" => vec!["missing_punctuation", "syntax_error", "language_confusion"],
            "E0003" => vec!["wrong_type", "missing_cast", "api_misuse"],
            "E0004" => vec!["unsafe_code", "security_concern", "missing_permission"],
            _ => vec!["unknown_cause"]
        }
    }

    /// Get learning objective for error categories
    fn get_learning_objective(&self, category: &ErrorCategory) -> &'static str {
        match category {
            ErrorCategory::Syntax => "Learn proper Ovie syntax and grammar rules",
            ErrorCategory::Type => "Understand Ovie's type system and type safety",
            ErrorCategory::Semantic => "Master Ovie's semantic rules and program structure",
            ErrorCategory::Security => "Learn safe programming practices in Ovie",
            ErrorCategory::Runtime => "Debug runtime issues and understand execution model",
            _ => "General Ovie programming proficiency"
        }
    }

    /// Get category summary for AI analysis
    fn get_category_summary(&self) -> HashMap<String, usize> {
        let mut summary = HashMap::new();
        for diagnostic in &self.diagnostics {
            let category = format!("{:?}", diagnostic.category).to_lowercase();
            *summary.entry(category).or_insert(0) += 1;
        }
        summary
    }

    /// Calculate complexity score for AI assessment
    fn calculate_complexity_score(&self) -> f32 {
        if self.diagnostics.is_empty() {
            return 0.0;
        }

        let mut score = 0.0;
        for diagnostic in &self.diagnostics {
            score += match diagnostic.severity {
                ErrorSeverity::Error => 3.0,
                ErrorSeverity::Warning => 1.5,
                ErrorSeverity::Info => 0.5,
                ErrorSeverity::Hint => 0.2,
            };

            // Add complexity based on category
            score += match diagnostic.category {
                ErrorCategory::Type | ErrorCategory::Semantic => 2.0,
                ErrorCategory::Security => 1.5,
                ErrorCategory::Syntax => 1.0,
                _ => 0.5,
            };

            // Reduce score if good suggestions are available
            if !diagnostic.suggestions.is_empty() {
                score *= 0.8;
            }
        }

        (score / self.diagnostics.len() as f32).min(10.0)
    }

    /// Identify learning opportunities for AI training
    fn identify_learning_opportunities(&self) -> Vec<serde_json::Value> {
        let mut opportunities = Vec::new();

        // Group errors by category
        let mut category_counts = HashMap::new();
        for diagnostic in &self.diagnostics {
            *category_counts.entry(&diagnostic.category).or_insert(0) += 1;
        }

        for (category, count) in category_counts {
            if count > 1 {
                opportunities.push(serde_json::json!({
                    "type": "pattern_recognition",
                    "category": format!("{:?}", category).to_lowercase(),
                    "description": format!("Multiple {} errors suggest need for focused learning", 
                                         format!("{:?}", category).to_lowercase()),
                    "priority": if count > 3 { "high" } else { "medium" }
                }));
            }
        }

        // Identify missing knowledge areas
        if self.diagnostics.iter().any(|d| d.suggestions.is_empty()) {
            opportunities.push(serde_json::json!({
                "type": "knowledge_gap",
                "description": "Some errors lack clear solutions, indicating advanced topics",
                "priority": "high"
            }));
        }

        opportunities
    }

    /// Generate human-readable error report
    pub fn to_human_readable(&self) -> String {
        let mut output = String::new();
        
        for diagnostic in &self.diagnostics {
            output.push_str(&format!(
                "{}: {} [{}]\n",
                match diagnostic.severity {
                    ErrorSeverity::Error => "error",
                    ErrorSeverity::Warning => "warning",
                    ErrorSeverity::Info => "info",
                    ErrorSeverity::Hint => "hint",
                },
                diagnostic.message,
                diagnostic.code
            ));

            if let Some(ref file) = diagnostic.location.file {
                output.push_str(&format!("  --> {}:{}:{}\n", file, diagnostic.location.line, diagnostic.location.column));
            } else {
                output.push_str(&format!("  --> line {}, column {}\n", diagnostic.location.line, diagnostic.location.column));
            }

            for suggestion in &diagnostic.suggestions {
                output.push_str(&format!("  help: {}\n", suggestion.message));
            }

            if let Some(ref help_url) = diagnostic.help_url {
                output.push_str(&format!("  For more information, see: {}\n", help_url));
            }

            output.push('\n');
        }

        if self.has_errors() {
            output.push_str(&format!(
                "error: aborting due to {} previous error{}\n",
                self.error_count(),
                if self.error_count() == 1 { "" } else { "s" }
            ));
        }

        output
    }
}

impl Default for ErrorReporter {
    fn default() -> Self {
        Self::new()
    }
}

/// Source location information (legacy)
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl SourceLocation {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset }
    }
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self::new(1, 1, 0)
    }
}

impl Default for SourcePosition {
    fn default() -> Self {
        Self {
            file: None,
            line: 1,
            column: 1,
            offset: 0,
        }
    }
}

impl SourcePosition {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self {
            file: None,
            line,
            column,
            offset,
        }
    }

    pub fn with_file(mut self, file: String) -> Self {
        self.file = Some(file);
        self
    }
}

impl ErrorSuggestion {
    /// Create a simple text suggestion
    pub fn simple(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code_fix: None,
            confidence: 0.8,
        }
    }

    /// Create a suggestion with a code fix
    pub fn with_fix(message: impl Into<String>, code_fix: CodeFix) -> Self {
        Self {
            message: message.into(),
            code_fix: Some(code_fix),
            confidence: 0.9,
        }
    }
}

impl CodeFix {
    /// Create a simple text replacement fix
    pub fn replace_text(
        description: impl Into<String>,
        start: SourcePosition,
        end: SourcePosition,
        new_text: impl Into<String>
    ) -> Self {
        Self {
            description: description.into(),
            replacements: vec![TextReplacement {
                start,
                end,
                new_text: new_text.into(),
            }],
        }
    }
}

impl From<std::io::Error> for OvieError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError {
            message: error.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_creation() {
        let suggestion = ErrorSuggestion::simple("Try adding a semicolon");
        let error = OvieError::parse_error_with_suggestions(
            1, 10, "Expected semicolon", vec![suggestion]
        );
        
        let diagnostic = error.get_diagnostic().unwrap();
        assert_eq!(diagnostic.code, "E0002");
        assert_eq!(diagnostic.severity, ErrorSeverity::Error);
        assert_eq!(diagnostic.category, ErrorCategory::Syntax);
        assert_eq!(diagnostic.suggestions.len(), 1);
    }

    #[test]
    fn test_error_reporter() {
        let mut reporter = ErrorReporter::new();
        
        let error1 = OvieError::parse_error(1, 1, "Missing semicolon");
        let error2 = OvieError::type_error(2, 5, "string", "number", vec![]);
        
        reporter.add_error(error1);
        reporter.add_error(error2);
        
        assert_eq!(reporter.error_count(), 2);
        assert_eq!(reporter.warning_count(), 0);
        assert!(reporter.has_errors());
    }

    #[test]
    fn test_human_readable_output() {
        let mut reporter = ErrorReporter::new();
        let error = OvieError::parse_error(1, 10, "Expected semicolon");
        reporter.add_error(error);
        
        let output = reporter.to_human_readable();
        assert!(output.contains("error: Expected semicolon [E0002]"));
        assert!(output.contains("line 1, column 10"));
    }

    #[test]
    fn test_json_output() {
        let mut reporter = ErrorReporter::new();
        let error = OvieError::parse_error(1, 10, "Expected semicolon");
        reporter.add_error(error);
        
        let json = reporter.to_json().unwrap();
        assert!(json.contains("E0002"));
        assert!(json.contains("Expected semicolon"));
    }

    #[test]
    fn test_ai_friendly_output() {
        let mut reporter = ErrorReporter::new();
        let error = OvieError::parse_error_with_suggestions(
            1, 10, "Expected semicolon", 
            vec![ErrorSuggestion::simple("Add a semicolon at the end of the statement")]
        );
        reporter.add_error(error);
        
        let ai_json = reporter.to_ai_friendly_json().unwrap();
        assert!(ai_json.contains("training_data"));
        assert!(ai_json.contains("learning_opportunities"));
        assert!(ai_json.contains("complexity_score"));
    }
}