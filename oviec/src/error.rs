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
    /// Fatal errors that prevent compilation and require immediate attention
    Fatal,
    /// Errors that prevent compilation but allow partial analysis
    Error,
    /// Warnings that should be addressed but don't prevent compilation
    Warning,
    /// Informational messages about code quality or style
    Info,
    /// Hints for improvement or optimization
    Hint,
}

impl ErrorSeverity {
    /// Get the severity as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorSeverity::Fatal => "fatal",
            ErrorSeverity::Error => "error",
            ErrorSeverity::Warning => "warning",
            ErrorSeverity::Info => "info",
            ErrorSeverity::Hint => "hint",
        }
    }

    /// Get the severity as an uppercase string for display
    pub fn as_display_str(&self) -> &'static str {
        match self {
            ErrorSeverity::Fatal => "FATAL",
            ErrorSeverity::Error => "ERROR",
            ErrorSeverity::Warning => "WARNING",
            ErrorSeverity::Info => "INFO",
            ErrorSeverity::Hint => "HINT",
        }
    }

    /// Get the numeric priority of this severity (higher = more severe)
    pub fn priority(&self) -> u8 {
        match self {
            ErrorSeverity::Fatal => 5,
            ErrorSeverity::Error => 4,
            ErrorSeverity::Warning => 3,
            ErrorSeverity::Info => 2,
            ErrorSeverity::Hint => 1,
        }
    }

    /// Check if this severity should stop compilation
    pub fn stops_compilation(&self) -> bool {
        matches!(self, ErrorSeverity::Fatal | ErrorSeverity::Error)
    }

    /// Check if this severity should be treated as an error when warnings-as-errors is enabled
    pub fn is_error_level(&self) -> bool {
        matches!(self, ErrorSeverity::Fatal | ErrorSeverity::Error)
    }

    /// Get the appropriate exit code for this severity
    pub fn exit_code(&self) -> i32 {
        match self {
            ErrorSeverity::Fatal => 2,  // Internal compiler error
            ErrorSeverity::Error => 1,  // Compilation error
            ErrorSeverity::Warning => 0, // Success with warnings
            ErrorSeverity::Info => 0,   // Success
            ErrorSeverity::Hint => 0,   // Success
        }
    }

    /// Get ANSI color code for terminal display
    pub fn color_code(&self) -> &'static str {
        match self {
            ErrorSeverity::Fatal => "\x1b[91m",    // Bright red
            ErrorSeverity::Error => "\x1b[31m",    // Red
            ErrorSeverity::Warning => "\x1b[33m",  // Yellow
            ErrorSeverity::Info => "\x1b[36m",     // Cyan
            ErrorSeverity::Hint => "\x1b[32m",     // Green
        }
    }

    /// Get the reset ANSI code
    pub fn reset_code() -> &'static str {
        "\x1b[0m"
    }
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

/// Error code taxonomy for structured error reporting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorCode {
    // Lexical Analysis Errors (E_LEX_xxx)
    /// Invalid character in source code
    ELex001,
    /// Unterminated string literal
    ELex002,
    /// Invalid number format
    ELex003,
    /// Invalid identifier
    ELex004,
    /// Unexpected end of file
    ELex005,

    // Syntax Parsing Errors (E_PARSE_xxx)
    /// Expected token not found
    EParse001,
    /// Unexpected token
    EParse002,
    /// Missing semicolon
    EParse003,
    /// Unmatched parentheses
    EParse004,
    /// Unmatched braces
    EParse005,
    /// Invalid expression
    EParse006,
    /// Invalid statement
    EParse007,

    // Semantic Analysis Errors (E_SEM_xxx)
    /// Undefined variable
    ESem001,
    /// Undefined function
    ESem002,
    /// Undefined type
    ESem003,
    /// Duplicate definition
    ESem004,
    /// Invalid scope access
    ESem005,
    /// Circular dependency
    ESem006,

    // Type System Errors (E_TYPE_xxx)
    /// Type mismatch
    EType001,
    /// Cannot infer type
    EType002,
    /// Invalid type conversion
    EType003,
    /// Undefined method
    EType004,
    /// Invalid trait implementation
    EType005,
    /// Lifetime error
    EType006,

    // Control Flow Errors (E_FLOW_xxx)
    /// Unreachable code
    EFlow001,
    /// Missing return statement
    EFlow002,
    /// Invalid break/continue
    EFlow003,
    /// Infinite loop detected
    EFlow004,

    // Memory Safety Errors (E_MEM_xxx)
    /// Use after free
    EMem001,
    /// Double free
    EMem002,
    /// Buffer overflow
    EMem003,
    /// Null pointer dereference
    EMem004,
    /// Memory leak detected
    EMem005,

    // I/O Errors (E_IO_xxx)
    /// File not found
    EIo001,
    /// Permission denied
    EIo002,
    /// Read error
    EIo003,
    /// Write error
    EIo004,
    /// Network error
    EIo005,

    // Internal Compiler Errors (E_ICE_xxx)
    /// Invariant violation
    EIce001,
    /// Unexpected compiler state
    EIce002,
    /// Code generation failure
    EIce003,
    /// Optimization failure
    EIce004,

    // Configuration Errors (E_CONFIG_xxx)
    /// Invalid configuration file
    EConfig001,
    /// Missing required setting
    EConfig002,
    /// Invalid setting value
    EConfig003,
    /// Environment setup error
    EConfig004,

    // Runtime Errors (E_RUNTIME_xxx)
    /// Division by zero
    ERuntime001,
    /// Array index out of bounds
    ERuntime002,
    /// Stack overflow
    ERuntime003,
    /// Assertion failure
    ERuntime004,
    /// Panic
    ERuntime005,
}

impl ErrorCode {
    /// Get the error code as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            // Lexical errors
            ErrorCode::ELex001 => "E_LEX_001",
            ErrorCode::ELex002 => "E_LEX_002",
            ErrorCode::ELex003 => "E_LEX_003",
            ErrorCode::ELex004 => "E_LEX_004",
            ErrorCode::ELex005 => "E_LEX_005",
            
            // Parse errors
            ErrorCode::EParse001 => "E_PARSE_001",
            ErrorCode::EParse002 => "E_PARSE_002",
            ErrorCode::EParse003 => "E_PARSE_003",
            ErrorCode::EParse004 => "E_PARSE_004",
            ErrorCode::EParse005 => "E_PARSE_005",
            ErrorCode::EParse006 => "E_PARSE_006",
            ErrorCode::EParse007 => "E_PARSE_007",
            
            // Semantic errors
            ErrorCode::ESem001 => "E_SEM_001",
            ErrorCode::ESem002 => "E_SEM_002",
            ErrorCode::ESem003 => "E_SEM_003",
            ErrorCode::ESem004 => "E_SEM_004",
            ErrorCode::ESem005 => "E_SEM_005",
            ErrorCode::ESem006 => "E_SEM_006",
            
            // Type errors
            ErrorCode::EType001 => "E_TYPE_001",
            ErrorCode::EType002 => "E_TYPE_002",
            ErrorCode::EType003 => "E_TYPE_003",
            ErrorCode::EType004 => "E_TYPE_004",
            ErrorCode::EType005 => "E_TYPE_005",
            ErrorCode::EType006 => "E_TYPE_006",
            
            // Control flow errors
            ErrorCode::EFlow001 => "E_FLOW_001",
            ErrorCode::EFlow002 => "E_FLOW_002",
            ErrorCode::EFlow003 => "E_FLOW_003",
            ErrorCode::EFlow004 => "E_FLOW_004",
            
            // Memory errors
            ErrorCode::EMem001 => "E_MEM_001",
            ErrorCode::EMem002 => "E_MEM_002",
            ErrorCode::EMem003 => "E_MEM_003",
            ErrorCode::EMem004 => "E_MEM_004",
            ErrorCode::EMem005 => "E_MEM_005",
            
            // I/O errors
            ErrorCode::EIo001 => "E_IO_001",
            ErrorCode::EIo002 => "E_IO_002",
            ErrorCode::EIo003 => "E_IO_003",
            ErrorCode::EIo004 => "E_IO_004",
            ErrorCode::EIo005 => "E_IO_005",
            
            // Internal compiler errors
            ErrorCode::EIce001 => "E_ICE_001",
            ErrorCode::EIce002 => "E_ICE_002",
            ErrorCode::EIce003 => "E_ICE_003",
            ErrorCode::EIce004 => "E_ICE_004",
            
            // Configuration errors
            ErrorCode::EConfig001 => "E_CONFIG_001",
            ErrorCode::EConfig002 => "E_CONFIG_002",
            ErrorCode::EConfig003 => "E_CONFIG_003",
            ErrorCode::EConfig004 => "E_CONFIG_004",
            
            // Runtime errors
            ErrorCode::ERuntime001 => "E_RUNTIME_001",
            ErrorCode::ERuntime002 => "E_RUNTIME_002",
            ErrorCode::ERuntime003 => "E_RUNTIME_003",
            ErrorCode::ERuntime004 => "E_RUNTIME_004",
            ErrorCode::ERuntime005 => "E_RUNTIME_005",
        }
    }

    /// Get a human-readable description of the error
    pub fn description(&self) -> &'static str {
        match self {
            // Lexical errors
            ErrorCode::ELex001 => "Invalid character in source code",
            ErrorCode::ELex002 => "Unterminated string literal",
            ErrorCode::ELex003 => "Invalid number format",
            ErrorCode::ELex004 => "Invalid identifier",
            ErrorCode::ELex005 => "Unexpected end of file",
            
            // Parse errors
            ErrorCode::EParse001 => "Expected token not found",
            ErrorCode::EParse002 => "Unexpected token",
            ErrorCode::EParse003 => "Missing semicolon",
            ErrorCode::EParse004 => "Unmatched parentheses",
            ErrorCode::EParse005 => "Unmatched braces",
            ErrorCode::EParse006 => "Invalid expression",
            ErrorCode::EParse007 => "Invalid statement",
            
            // Semantic errors
            ErrorCode::ESem001 => "Undefined variable",
            ErrorCode::ESem002 => "Undefined function",
            ErrorCode::ESem003 => "Undefined type",
            ErrorCode::ESem004 => "Duplicate definition",
            ErrorCode::ESem005 => "Invalid scope access",
            ErrorCode::ESem006 => "Circular dependency",
            
            // Type errors
            ErrorCode::EType001 => "Type mismatch",
            ErrorCode::EType002 => "Cannot infer type",
            ErrorCode::EType003 => "Invalid type conversion",
            ErrorCode::EType004 => "Undefined method",
            ErrorCode::EType005 => "Invalid trait implementation",
            ErrorCode::EType006 => "Lifetime error",
            
            // Control flow errors
            ErrorCode::EFlow001 => "Unreachable code",
            ErrorCode::EFlow002 => "Missing return statement",
            ErrorCode::EFlow003 => "Invalid break/continue",
            ErrorCode::EFlow004 => "Infinite loop detected",
            
            // Memory errors
            ErrorCode::EMem001 => "Use after free",
            ErrorCode::EMem002 => "Double free",
            ErrorCode::EMem003 => "Buffer overflow",
            ErrorCode::EMem004 => "Null pointer dereference",
            ErrorCode::EMem005 => "Memory leak detected",
            
            // I/O errors
            ErrorCode::EIo001 => "File not found",
            ErrorCode::EIo002 => "Permission denied",
            ErrorCode::EIo003 => "Read error",
            ErrorCode::EIo004 => "Write error",
            ErrorCode::EIo005 => "Network error",
            
            // Internal compiler errors
            ErrorCode::EIce001 => "Invariant violation",
            ErrorCode::EIce002 => "Unexpected compiler state",
            ErrorCode::EIce003 => "Code generation failure",
            ErrorCode::EIce004 => "Optimization failure",
            
            // Configuration errors
            ErrorCode::EConfig001 => "Invalid configuration file",
            ErrorCode::EConfig002 => "Missing required setting",
            ErrorCode::EConfig003 => "Invalid setting value",
            ErrorCode::EConfig004 => "Environment setup error",
            
            // Runtime errors
            ErrorCode::ERuntime001 => "Division by zero",
            ErrorCode::ERuntime002 => "Array index out of bounds",
            ErrorCode::ERuntime003 => "Stack overflow",
            ErrorCode::ERuntime004 => "Assertion failure",
            ErrorCode::ERuntime005 => "Panic",
        }
    }

    /// Get the category this error belongs to
    pub fn category(&self) -> ErrorCategory {
        match self {
            ErrorCode::ELex001 | ErrorCode::ELex002 | ErrorCode::ELex003 | ErrorCode::ELex004 | ErrorCode::ELex005 => ErrorCategory::Syntax,
            ErrorCode::EParse001 | ErrorCode::EParse002 | ErrorCode::EParse003 | ErrorCode::EParse004 | ErrorCode::EParse005 | ErrorCode::EParse006 | ErrorCode::EParse007 => ErrorCategory::Syntax,
            ErrorCode::ESem001 | ErrorCode::ESem002 | ErrorCode::ESem003 | ErrorCode::ESem004 | ErrorCode::ESem005 | ErrorCode::ESem006 => ErrorCategory::Semantic,
            ErrorCode::EType001 | ErrorCode::EType002 | ErrorCode::EType003 | ErrorCode::EType004 | ErrorCode::EType005 | ErrorCode::EType006 => ErrorCategory::Type,
            ErrorCode::EFlow001 | ErrorCode::EFlow002 | ErrorCode::EFlow003 | ErrorCode::EFlow004 => ErrorCategory::Semantic,
            ErrorCode::EMem001 | ErrorCode::EMem002 | ErrorCode::EMem003 | ErrorCode::EMem004 | ErrorCode::EMem005 => ErrorCategory::Security,
            ErrorCode::EIo001 | ErrorCode::EIo002 | ErrorCode::EIo003 | ErrorCode::EIo004 | ErrorCode::EIo005 => ErrorCategory::Io,
            ErrorCode::EIce001 | ErrorCode::EIce002 | ErrorCode::EIce003 | ErrorCode::EIce004 => ErrorCategory::Codegen,
            ErrorCode::EConfig001 | ErrorCode::EConfig002 | ErrorCode::EConfig003 | ErrorCode::EConfig004 => ErrorCategory::Package,
            ErrorCode::ERuntime001 | ErrorCode::ERuntime002 | ErrorCode::ERuntime003 | ErrorCode::ERuntime004 | ErrorCode::ERuntime005 => ErrorCategory::Runtime,
        }
    }

    /// Get the default severity for this error code
    pub fn default_severity(&self) -> ErrorSeverity {
        match self {
            // Internal compiler errors are fatal
            ErrorCode::E_ICE_001 | ErrorCode::E_ICE_002 | ErrorCode::E_ICE_003 | ErrorCode::E_ICE_004 => ErrorSeverity::Fatal,
            
            // Runtime errors are typically fatal
            ErrorCode::E_RUNTIME_003 | ErrorCode::E_RUNTIME_005 => ErrorSeverity::Fatal,
            
            // Memory errors are fatal
            ErrorCode::E_MEM_001 | ErrorCode::E_MEM_002 | ErrorCode::E_MEM_003 | ErrorCode::E_MEM_004 | ErrorCode::E_MEM_005 => ErrorSeverity::Fatal,
            
            // Most other errors are regular errors
            _ => ErrorSeverity::Error,
        }
    }
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
    /// Category of suggestion
    pub category: SuggestionCategory,
    /// Priority level
    pub priority: SuggestionPriority,
}

/// Categories of error suggestions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SuggestionCategory {
    /// Syntax correction
    SyntaxFix,
    /// Type annotation or conversion
    TypeFix,
    /// Import or dependency fix
    ImportFix,
    /// Security improvement
    SecurityFix,
    /// Performance optimization
    PerformanceFix,
    /// Style improvement
    StyleFix,
    /// General guidance
    Guidance,
}

/// Priority levels for suggestions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SuggestionPriority {
    /// Critical - must be fixed
    Critical,
    /// High - should be fixed soon
    High,
    /// Medium - should be addressed
    Medium,
    /// Low - nice to have
    Low,
    /// Info - informational only
    Info,
}

impl ErrorSuggestion {
    /// Create a new error suggestion
    pub fn new(message: String, confidence: f32) -> Self {
        Self {
            message,
            confidence: confidence.clamp(0.0, 1.0),
            code_fix: None,
            category: SuggestionCategory::Guidance,
            priority: SuggestionPriority::Medium,
        }
    }

    /// Create a syntax fix suggestion
    pub fn syntax_fix(message: String, confidence: f32, code_fix: Option<CodeFix>) -> Self {
        Self {
            message,
            confidence: confidence.clamp(0.0, 1.0),
            code_fix,
            category: SuggestionCategory::SyntaxFix,
            priority: SuggestionPriority::High,
        }
    }

    /// Create a type fix suggestion
    pub fn type_fix(message: String, confidence: f32, code_fix: Option<CodeFix>) -> Self {
        Self {
            message,
            confidence: confidence.clamp(0.0, 1.0),
            code_fix,
            category: SuggestionCategory::TypeFix,
            priority: SuggestionPriority::High,
        }
    }

    /// Create a security fix suggestion
    pub fn security_fix(message: String, confidence: f32) -> Self {
        Self {
            message,
            confidence: confidence.clamp(0.0, 1.0),
            code_fix: None,
            category: SuggestionCategory::SecurityFix,
            priority: SuggestionPriority::Critical,
        }
    }

    /// Create a guidance suggestion
    pub fn guidance(message: String) -> Self {
        Self {
            message,
            confidence: 0.7,
            code_fix: None,
            category: SuggestionCategory::Guidance,
            priority: SuggestionPriority::Medium,
        }
    }

    /// Check if this suggestion has an automatic fix
    pub fn has_auto_fix(&self) -> bool {
        self.code_fix.is_some()
    }

    /// Get the priority as a numeric value (higher = more important)
    pub fn priority_value(&self) -> u8 {
        match self.priority {
            SuggestionPriority::Critical => 5,
            SuggestionPriority::High => 4,
            SuggestionPriority::Medium => 3,
            SuggestionPriority::Low => 2,
            SuggestionPriority::Info => 1,
        }
    }
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

/// Explanation system for detailed error context
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorExplanation {
    /// Brief explanation of what went wrong
    pub what_happened: String,
    /// Why this error occurred
    pub why_it_happened: String,
    /// How to fix it
    pub how_to_fix: String,
    /// Related concepts to learn
    pub related_concepts: Vec<String>,
    /// Example of correct code
    pub example_fix: Option<String>,
    /// Common mistakes that lead to this error
    pub common_mistakes: Vec<String>,
    /// Learning resources
    pub learning_resources: Vec<LearningResource>,
}

/// Learning resource for error explanations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearningResource {
    /// Title of the resource
    pub title: String,
    /// URL to the resource
    pub url: String,
    /// Type of resource
    pub resource_type: ResourceType,
    /// Difficulty level
    pub difficulty: DifficultyLevel,
}

/// Types of learning resources
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResourceType {
    /// Official documentation
    Documentation,
    /// Tutorial or guide
    Tutorial,
    /// Example code
    Example,
    /// Video explanation
    Video,
    /// Interactive exercise
    Interactive,
    /// Community discussion
    Community,
}

/// Difficulty levels for learning resources
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DifficultyLevel {
    /// Beginner level
    Beginner,
    /// Intermediate level
    Intermediate,
    /// Advanced level
    Advanced,
    /// Expert level
    Expert,
}

impl ErrorExplanation {
    /// Create a comprehensive explanation for an error code
    pub fn for_error_code(error_code: &ErrorCode) -> Self {
        match error_code {
            ErrorCode::E_LEX_001 => Self {
                what_happened: "The lexer encountered an invalid character that is not part of the Ovie language syntax.".to_string(),
                why_it_happened: "This usually happens when you have a typo, use a character that's not allowed in Ovie, or have encoding issues.".to_string(),
                how_to_fix: "Check for typos, ensure you're using valid Ovie syntax, and verify your file encoding is UTF-8.".to_string(),
                related_concepts: vec!["lexical analysis".to_string(), "character encoding".to_string(), "syntax rules".to_string()],
                example_fix: Some("// Instead of: let x = 42@;\n// Use: let x = 42;".to_string()),
                common_mistakes: vec![
                    "Using special characters from other languages".to_string(),
                    "Copy-pasting code with invisible characters".to_string(),
                    "File encoding issues".to_string(),
                ],
                learning_resources: vec![
                    LearningResource {
                        title: "Ovie Lexical Rules".to_string(),
                        url: "https://ovie-lang.org/docs/lexical-rules".to_string(),
                        resource_type: ResourceType::Documentation,
                        difficulty: DifficultyLevel::Beginner,
                    }
                ],
            },
            ErrorCode::E_PARSE_001 => Self {
                what_happened: "The parser expected a specific token but found something else.".to_string(),
                why_it_happened: "This occurs when the syntax doesn't match what the parser expects at this point in the code.".to_string(),
                how_to_fix: "Check the syntax around the error location and ensure it follows Ovie's grammar rules.".to_string(),
                related_concepts: vec!["parsing".to_string(), "grammar rules".to_string(), "syntax".to_string()],
                example_fix: Some("// Instead of: if x == 42 { print(\"yes\") }\n// Use: if x == 42 { print(\"yes\"); }".to_string()),
                common_mistakes: vec![
                    "Missing semicolons".to_string(),
                    "Incorrect bracket matching".to_string(),
                    "Wrong keyword usage".to_string(),
                ],
                learning_resources: vec![
                    LearningResource {
                        title: "Ovie Syntax Guide".to_string(),
                        url: "https://ovie-lang.org/docs/syntax".to_string(),
                        resource_type: ResourceType::Documentation,
                        difficulty: DifficultyLevel::Beginner,
                    }
                ],
            },
            ErrorCode::E_TYPE_001 => Self {
                what_happened: "There's a mismatch between the expected type and the actual type of a value.".to_string(),
                why_it_happened: "Ovie has a strong type system that prevents mixing incompatible types without explicit conversion.".to_string(),
                how_to_fix: "Either change the value to match the expected type, or add an explicit type conversion.".to_string(),
                related_concepts: vec!["type system".to_string(), "type safety".to_string(), "type conversion".to_string()],
                example_fix: Some("// Instead of: let x: i32 = \"42\";\n// Use: let x: i32 = 42; or let x: i32 = \"42\".parse().unwrap();".to_string()),
                common_mistakes: vec![
                    "Mixing strings and numbers".to_string(),
                    "Forgetting type annotations".to_string(),
                    "Incorrect function return types".to_string(),
                ],
                learning_resources: vec![
                    LearningResource {
                        title: "Understanding Ovie's Type System".to_string(),
                        url: "https://ovie-lang.org/docs/types".to_string(),
                        resource_type: ResourceType::Documentation,
                        difficulty: DifficultyLevel::Intermediate,
                    }
                ],
            },
            _ => Self {
                what_happened: "An error occurred during compilation.".to_string(),
                why_it_happened: "The specific cause depends on the error code and context.".to_string(),
                how_to_fix: "Check the error message and location for specific guidance.".to_string(),
                related_concepts: vec!["debugging".to_string(), "error handling".to_string()],
                example_fix: None,
                common_mistakes: vec!["Various syntax and semantic issues".to_string()],
                learning_resources: vec![
                    LearningResource {
                        title: "Ovie Error Guide".to_string(),
                        url: "https://ovie-lang.org/docs/errors".to_string(),
                        resource_type: ResourceType::Documentation,
                        difficulty: DifficultyLevel::Beginner,
                    }
                ],
            },
        }
    }
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

impl SourcePosition {
    /// Create a new source position
    pub fn new(file: Option<String>, line: usize, column: usize, offset: usize) -> Self {
        Self { file, line, column, offset }
    }

    /// Create a source position without file information
    pub fn at_line_column(line: usize, column: usize) -> Self {
        Self {
            file: None,
            line,
            column,
            offset: 0,
        }
    }

    /// Create a source position with file information
    pub fn in_file(file: String, line: usize, column: usize, offset: usize) -> Self {
        Self {
            file: Some(file),
            line,
            column,
            offset,
        }
    }

    /// Get a display string for this position
    pub fn display(&self) -> String {
        match &self.file {
            Some(file) => format!("{}:{}:{}", file, self.line, self.column),
            None => format!("{}:{}", self.line, self.column),
        }
    }

    /// Check if this position is valid
    pub fn is_valid(&self) -> bool {
        self.line > 0 && self.column > 0
    }

    /// Create a range from this position to another
    pub fn to(&self, end: &SourcePosition) -> SourceRange {
        SourceRange {
            start: self.clone(),
            end: end.clone(),
        }
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

impl std::fmt::Display for SourcePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

/// Source range for multi-character spans
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceRange {
    /// Start position
    pub start: SourcePosition,
    /// End position
    pub end: SourcePosition,
}

impl SourceRange {
    /// Create a new source range
    pub fn new(start: SourcePosition, end: SourcePosition) -> Self {
        Self { start, end }
    }

    /// Create a single-character range
    pub fn single_char(position: SourcePosition) -> Self {
        let mut end = position.clone();
        end.column += 1;
        end.offset += 1;
        Self {
            start: position,
            end,
        }
    }

    /// Get the length of this range in characters
    pub fn length(&self) -> usize {
        if self.start.line == self.end.line {
            self.end.column.saturating_sub(self.start.column)
        } else {
            // Multi-line range - approximate
            self.end.offset.saturating_sub(self.start.offset)
        }
    }

    /// Check if this range contains a position
    pub fn contains(&self, position: &SourcePosition) -> bool {
        if self.start.line == self.end.line {
            // Single line range
            position.line == self.start.line
                && position.column >= self.start.column
                && position.column < self.end.column
        } else {
            // Multi-line range
            (position.line > self.start.line && position.line < self.end.line)
                || (position.line == self.start.line && position.column >= self.start.column)
                || (position.line == self.end.line && position.column < self.end.column)
        }
    }

    /// Get a display string for this range
    pub fn display(&self) -> String {
        if self.start.file == self.end.file {
            match &self.start.file {
                Some(file) => {
                    if self.start.line == self.end.line {
                        format!("{}:{}:{}-{}", file, self.start.line, self.start.column, self.end.column)
                    } else {
                        format!("{}:{}:{}-{}:{}", file, self.start.line, self.start.column, self.end.line, self.end.column)
                    }
                }
                None => {
                    if self.start.line == self.end.line {
                        format!("{}:{}-{}", self.start.line, self.start.column, self.end.column)
                    } else {
                        format!("{}:{}-{}:{}", self.start.line, self.start.column, self.end.line, self.end.column)
                    }
                }
            }
        } else {
            format!("{} to {}", self.start.display(), self.end.display())
        }
    }
}

impl std::fmt::Display for SourceRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
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

    #[error("Invariant violation in {stage}: {message}")]
    InvariantViolation {
        stage: String,
        message: String,
    },

    #[error("Hardware error: {0}")]
    HardwareError(String),

    #[error("Compilation error: {message}")]
    CompileError {
        message: String,
    },

    #[error("JSON serialization/deserialization error: {0}")]
    SerdeJson(String),
}

impl OvieError {
    /// Create a comprehensive diagnostic error
    pub fn diagnostic(diagnostic: Diagnostic) -> Self {
        Self::Diagnostic { diagnostic }
    }

    /// Create an invariant violation error
    pub fn invariant_violation(stage: impl Into<String>, message: impl Into<String>) -> Self {
        Self::InvariantViolation {
            stage: stage.into(),
            message: message.into(),
        }
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

    pub fn compile_error(message: impl Into<String>) -> Self {
        Self::CompileError {
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
            Self::HardwareError(message) => Diagnostic {
                code: "E0011".to_string(),
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
                help_url: Some("https://ovie-lang.org/docs/errors/E0011".to_string()),
            },
            Self::CompileError { message } => Diagnostic {
                code: "E0012".to_string(),
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
                help_url: Some("https://ovie-lang.org/docs/errors/E0012".to_string()),
            },
            Self::SerdeJson(error) => Diagnostic {
                code: "E0013".to_string(),
                severity: ErrorSeverity::Error,
                category: ErrorCategory::Io,
                message: format!("JSON serialization/deserialization error: {}", error),
                location: SourcePosition {
                    file: None,
                    line: 1,
                    column: 1,
                    offset: 0,
                },
                related_locations: Vec::new(),
                suggestions: Vec::new(),
                context: HashMap::new(),
                help_url: Some("https://ovie-lang.org/docs/errors/E0013".to_string()),
            },
            Self::InvariantViolation { stage, message } => Diagnostic {
                code: "E0014".to_string(),
                severity: ErrorSeverity::Error,
                category: ErrorCategory::Semantic,
                message: format!("Invariant violation in {}: {}", stage, message),
                location: SourcePosition {
                    file: None,
                    line: 1,
                    column: 1,
                    offset: 0,
                },
                related_locations: Vec::new(),
                suggestions: Vec::new(),
                context: HashMap::new(),
                help_url: Some("https://ovie-lang.org/docs/errors/E0014".to_string()),
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
                ErrorSeverity::Fatal => 5.0,
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
                    ErrorSeverity::Fatal => "fatal",
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

impl SourcePosition {
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
            category: SuggestionCategory::Guidance,
            priority: SuggestionPriority::Medium,
        }
    }

    /// Create a suggestion with a code fix
    pub fn with_fix(message: impl Into<String>, code_fix: CodeFix) -> Self {
        Self {
            message: message.into(),
            code_fix: Some(code_fix),
            confidence: 0.9,
            category: SuggestionCategory::SyntaxFix,
            priority: SuggestionPriority::High,
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

impl From<serde_json::Error> for OvieError {
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeJson(error.to_string())
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
