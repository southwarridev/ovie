//! Explanation and suggestion system for compiler decisions and diagnostics
//!
//! This module provides detailed explanations for compiler decisions, specific fix
//! suggestions, and educational content to help developers understand and resolve issues.

use crate::{Diagnostic, Finding, Severity, AnalysisCategory, AprokoResult, AprokoError};
use crate::diagnostic::{DiagnosticCategory, SourceLocation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Types of explanations the system can provide
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExplanationType {
    /// Explains why a compiler decision was made
    CompilerDecision,
    /// Explains what a diagnostic means
    DiagnosticExplanation,
    /// Provides step-by-step fix instructions
    FixSuggestion,
    /// Educational content about language features
    LanguageConcept,
    /// Best practice recommendations
    BestPractice,
    /// Performance optimization guidance
    OptimizationTip,
    /// Security consideration explanation
    SecurityGuidance,
}

/// Detailed explanation with context and suggestions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Explanation {
    /// Type of explanation
    pub explanation_type: ExplanationType,
    /// Brief summary of the explanation
    pub summary: String,
    /// Detailed explanation text
    pub detailed_explanation: String,
    /// Code examples demonstrating the concept
    pub code_examples: Vec<CodeExample>,
    /// Step-by-step fix suggestions
    pub fix_suggestions: Vec<FixSuggestion>,
    /// Related concepts or diagnostics
    pub related_topics: Vec<String>,
    /// External resources for further learning
    pub external_resources: Vec<ExternalResource>,
    /// Confidence level of the explanation (0.0 to 1.0)
    pub confidence: f32,
}

/// Code example with explanation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExample {
    /// Description of what this example demonstrates
    pub description: String,
    /// The example code
    pub code: String,
    /// Language of the code (usually "ovie")
    pub language: String,
    /// Whether this is a good or bad example
    pub is_good_example: bool,
    /// Additional notes about the example
    pub notes: Option<String>,
}

/// Specific fix suggestion with actionable steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixSuggestion {
    /// Brief description of the fix
    pub title: String,
    /// Detailed explanation of the fix
    pub description: String,
    /// Step-by-step instructions
    pub steps: Vec<FixStep>,
    /// Confidence that this fix will resolve the issue (0.0 to 1.0)
    pub confidence: f32,
    /// Estimated difficulty level
    pub difficulty: DifficultyLevel,
    /// Whether this fix should be applied automatically (always false for safety)
    pub auto_applicable: bool,
}

/// Individual step in a fix suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixStep {
    /// Step number
    pub step_number: usize,
    /// Description of what to do
    pub description: String,
    /// Code to add, remove, or modify
    pub code_change: Option<CodeChange>,
    /// Additional notes or warnings
    pub notes: Option<String>,
}

/// Represents a code change suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeChange {
    /// Type of change
    pub change_type: ChangeType,
    /// Original code (for replacements)
    pub original_code: Option<String>,
    /// New code to insert or replace with
    pub new_code: String,
    /// Location where the change should be applied
    pub location: Option<SourceLocation>,
}

/// Type of code change
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    /// Add new code
    Insert,
    /// Remove existing code
    Delete,
    /// Replace existing code
    Replace,
    /// Modify existing code
    Modify,
}

/// Difficulty level for fix suggestions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DifficultyLevel {
    /// Simple fix, usually one-line change
    Easy,
    /// Moderate fix, may require understanding of context
    Medium,
    /// Complex fix, requires significant refactoring
    Hard,
    /// Very complex, may require architectural changes
    Expert,
}

/// External resource for learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalResource {
    /// Title of the resource
    pub title: String,
    /// URL to the resource
    pub url: String,
    /// Type of resource
    pub resource_type: ResourceType,
    /// Brief description
    pub description: String,
}

/// Type of external resource
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    /// Official documentation
    Documentation,
    /// Tutorial or guide
    Tutorial,
    /// Blog post or article
    Article,
    /// Video content
    Video,
    /// Code example or repository
    CodeExample,
    /// Academic paper or specification
    Specification,
}

/// The main explanation engine
pub struct ExplanationEngine {
    /// Pre-built explanations for common issues
    explanation_database: HashMap<String, Explanation>,
    /// Configuration for explanation generation
    config: ExplanationConfig,
}

/// Configuration for the explanation engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationConfig {
    /// Maximum number of code examples per explanation
    pub max_code_examples: usize,
    /// Maximum number of fix suggestions per explanation
    pub max_fix_suggestions: usize,
    /// Minimum confidence threshold for suggestions
    pub min_confidence_threshold: f32,
    /// Whether to include external resources
    pub include_external_resources: bool,
    /// Preferred explanation verbosity
    pub verbosity_level: VerbosityLevel,
}

/// Verbosity level for explanations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerbosityLevel {
    /// Brief explanations
    Concise,
    /// Standard explanations
    Standard,
    /// Detailed explanations with examples
    Detailed,
    /// Comprehensive explanations with all available information
    Comprehensive,
}

impl ExplanationEngine {
    /// Create a new explanation engine with default configuration
    pub fn new() -> Self {
        let mut engine = Self {
            explanation_database: HashMap::new(),
            config: ExplanationConfig::default(),
        };
        engine.populate_default_explanations();
        engine
    }

    /// Create an explanation engine with custom configuration
    pub fn with_config(config: ExplanationConfig) -> Self {
        let mut engine = Self {
            explanation_database: HashMap::new(),
            config,
        };
        engine.populate_default_explanations();
        engine
    }

    /// Populate the database with default explanations
    fn populate_default_explanations(&mut self) {
        // Syntax error explanations
        self.add_explanation("E001", Explanation {
            explanation_type: ExplanationType::DiagnosticExplanation,
            summary: "Unexpected token encountered during parsing".to_string(),
            detailed_explanation: "The parser encountered a token that doesn't fit the expected grammar at this position. This usually indicates a syntax error such as missing punctuation, incorrect keyword usage, or malformed expressions.".to_string(),
            code_examples: vec![
                CodeExample {
                    description: "Missing semicolon".to_string(),
                    code: "let x = 5\nlet y = 10  // Error: missing semicolon after first statement".to_string(),
                    language: "ovie".to_string(),
                    is_good_example: false,
                    notes: Some("Each statement should end with a semicolon".to_string()),
                },
                CodeExample {
                    description: "Correct syntax with semicolons".to_string(),
                    code: "let x = 5;\nlet y = 10;  // Correct: semicolons properly placed".to_string(),
                    language: "ovie".to_string(),
                    is_good_example: true,
                    notes: None,
                },
            ],
            fix_suggestions: vec![
                FixSuggestion {
                    title: "Add missing semicolon".to_string(),
                    description: "Add a semicolon at the end of the statement".to_string(),
                    steps: vec![
                        FixStep {
                            step_number: 1,
                            description: "Locate the end of the statement".to_string(),
                            code_change: None,
                            notes: Some("Look for the position where the statement ends".to_string()),
                        },
                        FixStep {
                            step_number: 2,
                            description: "Add a semicolon".to_string(),
                            code_change: Some(CodeChange {
                                change_type: ChangeType::Insert,
                                original_code: None,
                                new_code: ";".to_string(),
                                location: None,
                            }),
                            notes: None,
                        },
                    ],
                    confidence: 0.9,
                    difficulty: DifficultyLevel::Easy,
                    auto_applicable: false,
                },
            ],
            related_topics: vec!["syntax".to_string(), "grammar".to_string(), "statements".to_string()],
            external_resources: vec![
                ExternalResource {
                    title: "Ovie Language Grammar Guide".to_string(),
                    url: "https://ovie-lang.org/docs/grammar".to_string(),
                    resource_type: ResourceType::Documentation,
                    description: "Complete grammar specification for the Ovie language".to_string(),
                },
            ],
            confidence: 0.95,
        });

        // Type error explanations
        self.add_explanation("E002", Explanation {
            explanation_type: ExplanationType::DiagnosticExplanation,
            summary: "Type mismatch between expected and actual types".to_string(),
            detailed_explanation: "The expression's type doesn't match what was expected in this context. Ovie has a strong type system that prevents implicit conversions between incompatible types to ensure memory safety and prevent runtime errors.".to_string(),
            code_examples: vec![
                CodeExample {
                    description: "String assigned to integer variable".to_string(),
                    code: "let x: i32 = \"hello\";  // Error: cannot assign string to integer".to_string(),
                    language: "ovie".to_string(),
                    is_good_example: false,
                    notes: Some("Types must match exactly".to_string()),
                },
                CodeExample {
                    description: "Correct type assignment".to_string(),
                    code: "let x: i32 = 42;        // Correct: integer assigned to integer variable\nlet y: String = \"hello\"; // Correct: string assigned to string variable".to_string(),
                    language: "ovie".to_string(),
                    is_good_example: true,
                    notes: None,
                },
            ],
            fix_suggestions: vec![
                FixSuggestion {
                    title: "Change variable type annotation".to_string(),
                    description: "Update the variable's type annotation to match the assigned value".to_string(),
                    steps: vec![
                        FixStep {
                            step_number: 1,
                            description: "Identify the actual type of the assigned value".to_string(),
                            code_change: None,
                            notes: Some("Look at the value being assigned to determine its type".to_string()),
                        },
                        FixStep {
                            step_number: 2,
                            description: "Update the type annotation".to_string(),
                            code_change: Some(CodeChange {
                                change_type: ChangeType::Replace,
                                original_code: Some("i32".to_string()),
                                new_code: "String".to_string(),
                                location: None,
                            }),
                            notes: None,
                        },
                    ],
                    confidence: 0.8,
                    difficulty: DifficultyLevel::Easy,
                    auto_applicable: false,
                },
                FixSuggestion {
                    title: "Convert the value to the expected type".to_string(),
                    description: "Use appropriate conversion methods to match the expected type".to_string(),
                    steps: vec![
                        FixStep {
                            step_number: 1,
                            description: "Use a conversion function".to_string(),
                            code_change: Some(CodeChange {
                                change_type: ChangeType::Replace,
                                original_code: Some("\"hello\"".to_string()),
                                new_code: "\"42\".parse::<i32>().unwrap()".to_string(),
                                location: None,
                            }),
                            notes: Some("Be careful with unwrap() in production code".to_string()),
                        },
                    ],
                    confidence: 0.7,
                    difficulty: DifficultyLevel::Medium,
                    auto_applicable: false,
                },
            ],
            related_topics: vec!["types".to_string(), "type_system".to_string(), "conversions".to_string()],
            external_resources: vec![
                ExternalResource {
                    title: "Ovie Type System Guide".to_string(),
                    url: "https://ovie-lang.org/docs/types".to_string(),
                    resource_type: ResourceType::Documentation,
                    description: "Comprehensive guide to Ovie's type system".to_string(),
                },
            ],
            confidence: 0.92,
        });

        // Performance warning explanations
        self.add_explanation("W001", Explanation {
            explanation_type: ExplanationType::OptimizationTip,
            summary: "Algorithm could be optimized for better performance".to_string(),
            detailed_explanation: "The current implementation has suboptimal time or space complexity. Consider using more efficient algorithms or data structures to improve performance, especially for large inputs.".to_string(),
            code_examples: vec![
                CodeExample {
                    description: "Inefficient nested loop".to_string(),
                    code: "for i in 0..n {\n    for j in 0..n {\n        // O(n²) complexity\n        process(i, j);\n    }\n}".to_string(),
                    language: "ovie".to_string(),
                    is_good_example: false,
                    notes: Some("This has quadratic time complexity".to_string()),
                },
                CodeExample {
                    description: "More efficient single loop".to_string(),
                    code: "for i in 0..n {\n    // O(n) complexity\n    process_efficiently(i);\n}".to_string(),
                    language: "ovie".to_string(),
                    is_good_example: true,
                    notes: Some("Linear time complexity is much better".to_string()),
                },
            ],
            fix_suggestions: vec![
                FixSuggestion {
                    title: "Optimize algorithm complexity".to_string(),
                    description: "Replace the nested loop with a more efficient algorithm".to_string(),
                    steps: vec![
                        FixStep {
                            step_number: 1,
                            description: "Analyze what the nested loop is trying to accomplish".to_string(),
                            code_change: None,
                            notes: Some("Understanding the goal helps find better algorithms".to_string()),
                        },
                        FixStep {
                            step_number: 2,
                            description: "Research more efficient algorithms for this problem".to_string(),
                            code_change: None,
                            notes: Some("Consider hash tables, sorting, or mathematical approaches".to_string()),
                        },
                        FixStep {
                            step_number: 3,
                            description: "Implement the optimized version".to_string(),
                            code_change: Some(CodeChange {
                                change_type: ChangeType::Replace,
                                original_code: Some("for i in 0..n {\n    for j in 0..n {\n        process(i, j);\n    }\n}".to_string()),
                                new_code: "// Use more efficient algorithm here\noptimized_process(data);".to_string(),
                                location: None,
                            }),
                            notes: None,
                        },
                    ],
                    confidence: 0.6,
                    difficulty: DifficultyLevel::Hard,
                    auto_applicable: false,
                },
            ],
            related_topics: vec!["performance".to_string(), "algorithms".to_string(), "complexity".to_string()],
            external_resources: vec![
                ExternalResource {
                    title: "Algorithm Complexity Guide".to_string(),
                    url: "https://ovie-lang.org/docs/performance".to_string(),
                    resource_type: ResourceType::Documentation,
                    description: "Guide to understanding and optimizing algorithm complexity".to_string(),
                },
            ],
            confidence: 0.85,
        });

        // Security warning explanations
        self.add_explanation("S001", Explanation {
            explanation_type: ExplanationType::SecurityGuidance,
            summary: "Potentially unsafe operation detected".to_string(),
            detailed_explanation: "This operation could lead to security vulnerabilities such as buffer overflows, out-of-bounds access, or other memory safety issues. Ovie's safety features help prevent these issues, but certain operations still require careful handling.".to_string(),
            code_examples: vec![
                CodeExample {
                    description: "Unchecked array access".to_string(),
                    code: "let arr = [1, 2, 3];\nlet val = arr[10];  // Potential out-of-bounds access".to_string(),
                    language: "ovie".to_string(),
                    is_good_example: false,
                    notes: Some("This could panic or access invalid memory".to_string()),
                },
                CodeExample {
                    description: "Safe array access with bounds checking".to_string(),
                    code: "let arr = [1, 2, 3];\nif let Some(val) = arr.get(10) {\n    // Safe access\n    seeAm(val);\n} else {\n    seeAm(\"Index out of bounds\");\n}".to_string(),
                    language: "ovie".to_string(),
                    is_good_example: true,
                    notes: Some("Always check bounds before accessing".to_string()),
                },
            ],
            fix_suggestions: vec![
                FixSuggestion {
                    title: "Add bounds checking".to_string(),
                    description: "Use safe array access methods that check bounds".to_string(),
                    steps: vec![
                        FixStep {
                            step_number: 1,
                            description: "Replace direct indexing with safe access".to_string(),
                            code_change: Some(CodeChange {
                                change_type: ChangeType::Replace,
                                original_code: Some("arr[index]".to_string()),
                                new_code: "arr.get(index)".to_string(),
                                location: None,
                            }),
                            notes: Some("This returns an Option that must be handled".to_string()),
                        },
                        FixStep {
                            step_number: 2,
                            description: "Handle the Option result".to_string(),
                            code_change: Some(CodeChange {
                                change_type: ChangeType::Replace,
                                original_code: Some("let val = arr.get(index);".to_string()),
                                new_code: "if let Some(val) = arr.get(index) {\n    // Use val\n} else {\n    // Handle out-of-bounds case\n}".to_string(),
                                location: None,
                            }),
                            notes: None,
                        },
                    ],
                    confidence: 0.9,
                    difficulty: DifficultyLevel::Medium,
                    auto_applicable: false,
                },
            ],
            related_topics: vec!["security".to_string(), "memory_safety".to_string(), "bounds_checking".to_string()],
            external_resources: vec![
                ExternalResource {
                    title: "Ovie Memory Safety Guide".to_string(),
                    url: "https://ovie-lang.org/docs/safety".to_string(),
                    resource_type: ResourceType::Documentation,
                    description: "Comprehensive guide to memory safety in Ovie".to_string(),
                },
            ],
            confidence: 0.88,
        });
    }

    /// Add an explanation to the database
    pub fn add_explanation(&mut self, rule_id: &str, explanation: Explanation) {
        self.explanation_database.insert(rule_id.to_string(), explanation);
    }

    /// Get explanation for a specific rule or diagnostic
    pub fn get_explanation(&self, rule_id: &str) -> Option<&Explanation> {
        self.explanation_database.get(rule_id)
    }

    /// Generate explanation for a diagnostic
    pub fn explain_diagnostic(&self, diagnostic: &Diagnostic) -> AprokoResult<Explanation> {
        if let Some(explanation) = self.get_explanation(&diagnostic.rule_id) {
            Ok(self.customize_explanation_for_diagnostic(explanation, diagnostic))
        } else {
            self.generate_generic_explanation(diagnostic)
        }
    }

    /// Generate explanation for a finding
    pub fn explain_finding(&self, finding: &Finding) -> AprokoResult<Explanation> {
        if let Some(explanation) = self.get_explanation(&finding.rule_id) {
            Ok(self.customize_explanation_for_finding(explanation, finding))
        } else {
            self.generate_generic_explanation_for_finding(finding)
        }
    }

    /// Customize explanation based on specific diagnostic context
    fn customize_explanation_for_diagnostic(&self, base_explanation: &Explanation, diagnostic: &Diagnostic) -> Explanation {
        let mut customized = base_explanation.clone();
        
        // Add location-specific information
        customized.detailed_explanation = format!(
            "{}\n\nThis issue occurs at {}:{}:{}.",
            customized.detailed_explanation,
            diagnostic.location.file,
            diagnostic.location.line,
            diagnostic.location.column
        );

        // Adjust confidence based on context
        if diagnostic.location.source_excerpt.is_some() {
            customized.confidence = (customized.confidence * 1.1).min(1.0);
        }

        customized
    }

    /// Customize explanation based on specific finding context
    fn customize_explanation_for_finding(&self, base_explanation: &Explanation, finding: &Finding) -> Explanation {
        let mut customized = base_explanation.clone();
        
        // Add location-specific information
        customized.detailed_explanation = format!(
            "{}\n\nThis issue occurs at line {}, column {}.",
            customized.detailed_explanation,
            finding.location.0,
            finding.location.1
        );

        // Add suggestion if available
        if let Some(suggestion) = &finding.suggestion {
            customized.fix_suggestions.insert(0, FixSuggestion {
                title: "Quick fix".to_string(),
                description: suggestion.clone(),
                steps: vec![
                    FixStep {
                        step_number: 1,
                        description: suggestion.clone(),
                        code_change: None,
                        notes: None,
                    }
                ],
                confidence: 0.7,
                difficulty: DifficultyLevel::Easy,
                auto_applicable: false,
            });
        }

        customized
    }

    /// Generate a generic explanation for unknown diagnostics
    fn generate_generic_explanation(&self, diagnostic: &Diagnostic) -> AprokoResult<Explanation> {
        Ok(Explanation {
            explanation_type: ExplanationType::DiagnosticExplanation,
            summary: format!("Diagnostic: {}", diagnostic.message),
            detailed_explanation: format!(
                "A {} level {} diagnostic was generated: {}\n\nLocation: {}:{}:{}",
                diagnostic.severity,
                diagnostic.category,
                diagnostic.message,
                diagnostic.location.file,
                diagnostic.location.line,
                diagnostic.location.column
            ),
            code_examples: vec![],
            fix_suggestions: vec![],
            related_topics: vec![format!("{:?}", diagnostic.category).to_lowercase()],
            external_resources: vec![],
            confidence: 0.5,
        })
    }

    /// Generate a generic explanation for unknown findings
    fn generate_generic_explanation_for_finding(&self, finding: &Finding) -> AprokoResult<Explanation> {
        let mut fix_suggestions = vec![];
        
        if let Some(suggestion) = &finding.suggestion {
            fix_suggestions.push(FixSuggestion {
                title: "Suggested fix".to_string(),
                description: suggestion.clone(),
                steps: vec![
                    FixStep {
                        step_number: 1,
                        description: suggestion.clone(),
                        code_change: None,
                        notes: None,
                    }
                ],
                confidence: 0.6,
                difficulty: DifficultyLevel::Easy,
                auto_applicable: false,
            });
        }

        Ok(Explanation {
            explanation_type: ExplanationType::DiagnosticExplanation,
            summary: format!("Finding: {}", finding.message),
            detailed_explanation: format!(
                "A {} level {} finding was detected: {}\n\nLocation: line {}, column {}",
                finding.severity,
                finding.category,
                finding.message,
                finding.location.0,
                finding.location.1
            ),
            code_examples: vec![],
            fix_suggestions,
            related_topics: vec![format!("{:?}", finding.category).to_lowercase()],
            external_resources: vec![],
            confidence: 0.5,
        })
    }

    /// Get all available explanations
    pub fn get_all_explanations(&self) -> &HashMap<String, Explanation> {
        &self.explanation_database
    }

    /// Get explanations by type
    pub fn get_explanations_by_type(&self, explanation_type: ExplanationType) -> Vec<(&String, &Explanation)> {
        self.explanation_database
            .iter()
            .filter(|(_, explanation)| explanation.explanation_type == explanation_type)
            .collect()
    }

    /// Update configuration
    pub fn set_config(&mut self, config: ExplanationConfig) {
        self.config = config;
    }

    /// Get current configuration
    pub fn get_config(&self) -> &ExplanationConfig {
        &self.config
    }
}

impl Default for ExplanationConfig {
    fn default() -> Self {
        Self {
            max_code_examples: 3,
            max_fix_suggestions: 2,
            min_confidence_threshold: 0.5,
            include_external_resources: true,
            verbosity_level: VerbosityLevel::Standard,
        }
    }
}

impl Default for ExplanationEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ExplanationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExplanationType::CompilerDecision => write!(f, "compiler-decision"),
            ExplanationType::DiagnosticExplanation => write!(f, "diagnostic-explanation"),
            ExplanationType::FixSuggestion => write!(f, "fix-suggestion"),
            ExplanationType::LanguageConcept => write!(f, "language-concept"),
            ExplanationType::BestPractice => write!(f, "best-practice"),
            ExplanationType::OptimizationTip => write!(f, "optimization-tip"),
            ExplanationType::SecurityGuidance => write!(f, "security-guidance"),
        }
    }
}

impl fmt::Display for DifficultyLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DifficultyLevel::Easy => write!(f, "easy"),
            DifficultyLevel::Medium => write!(f, "medium"),
            DifficultyLevel::Hard => write!(f, "hard"),
            DifficultyLevel::Expert => write!(f, "expert"),
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Info => write!(f, "info"),
            Severity::Warning => write!(f, "warning"),
            Severity::Error => write!(f, "error"),
            Severity::Critical => write!(f, "critical"),
        }
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

impl fmt::Display for AnalysisCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnalysisCategory::Syntax => write!(f, "syntax"),
            AnalysisCategory::Logic => write!(f, "logic"),
            AnalysisCategory::Performance => write!(f, "performance"),
            AnalysisCategory::Security => write!(f, "security"),
            AnalysisCategory::Correctness => write!(f, "correctness"),
            AnalysisCategory::Style => write!(f, "style"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explanation_engine_creation() {
        let engine = ExplanationEngine::new();
        assert!(!engine.get_all_explanations().is_empty());
    }

    #[test]
    fn test_explanation_retrieval() {
        let engine = ExplanationEngine::new();
        let explanation = engine.get_explanation("E001");
        assert!(explanation.is_some());
        
        if let Some(exp) = explanation {
            assert_eq!(exp.explanation_type, ExplanationType::DiagnosticExplanation);
            assert!(!exp.summary.is_empty());
            assert!(!exp.detailed_explanation.is_empty());
        }
    }

    #[test]
    fn test_explanations_by_type() {
        let engine = ExplanationEngine::new();
        let diagnostic_explanations = engine.get_explanations_by_type(ExplanationType::DiagnosticExplanation);
        let optimization_tips = engine.get_explanations_by_type(ExplanationType::OptimizationTip);
        
        assert!(!diagnostic_explanations.is_empty());
        assert!(!optimization_tips.is_empty());
    }

    #[test]
    fn test_explanation_customization() {
        let engine = ExplanationEngine::new();
        let diagnostic = Diagnostic {
            rule_id: "E001".to_string(),
            category: DiagnosticCategory::SyntaxError,
            severity: Severity::Error,
            message: "Test error".to_string(),
            explanation: None,
            suggestion: None,
            location: SourceLocation {
                file: "test.ov".to_string(),
                line: 10,
                column: 5,
                span_length: 3,
                source_excerpt: Some("let".to_string()),
            },
            related_locations: vec![],
            metadata: HashMap::new(),
        };

        let result = engine.explain_diagnostic(&diagnostic);
        assert!(result.is_ok());
        
        let explanation = result.unwrap();
        assert!(explanation.detailed_explanation.contains("test.ov:10:5"));
    }

    #[test]
    fn test_display_formatting() {
        assert_eq!(ExplanationType::DiagnosticExplanation.to_string(), "diagnostic-explanation");
        assert_eq!(DifficultyLevel::Medium.to_string(), "medium");
        assert_eq!(Severity::Error.to_string(), "error");
    }
}