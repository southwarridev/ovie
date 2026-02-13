//! Reasoning Engine for Aproko
//!
//! This module provides the reasoning infrastructure for explaining compiler decisions,
//! tracking reasoning chains, and generating intelligent explanations.

use crate::{Diagnostic, Finding, Explanation, ExplanationType, AprokoResult, AprokoError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Knowledge base entry representing a fact or rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEntry {
    /// Unique identifier for this knowledge entry
    pub id: String,
    /// Category of knowledge
    pub category: KnowledgeCategory,
    /// The fact or rule description
    pub content: String,
    /// Related concepts
    pub related_concepts: Vec<String>,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f32,
    /// Source of this knowledge (e.g., "language spec", "best practices")
    pub source: String,
}

/// Categories of knowledge in the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KnowledgeCategory {
    /// Language syntax rules
    SyntaxRule,
    /// Type system rules
    TypeRule,
    /// Semantic rules
    SemanticRule,
    /// Best practices
    BestPractice,
    /// Performance guidelines
    PerformanceGuideline,
    /// Security guidelines
    SecurityGuideline,
    /// Common patterns
    CommonPattern,
    /// Anti-patterns
    AntiPattern,
}

/// Represents a step in the reasoning process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningStep {
    /// Step number in the reasoning chain
    pub step_number: usize,
    /// Description of what was considered
    pub description: String,
    /// Knowledge entries used in this step
    pub knowledge_used: Vec<String>,
    /// Conclusion reached in this step
    pub conclusion: String,
    /// Confidence in this step (0.0 to 1.0)
    pub confidence: f32,
}

/// Complete reasoning chain for a decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningChain {
    /// Unique identifier for this reasoning chain
    pub id: String,
    /// What question or problem is being reasoned about
    pub question: String,
    /// Steps in the reasoning process
    pub steps: Vec<ReasoningStep>,
    /// Final conclusion
    pub conclusion: String,
    /// Overall confidence (0.0 to 1.0)
    pub overall_confidence: f32,
    /// Alternative conclusions considered
    pub alternatives: Vec<AlternativeConclusion>,
}

/// Alternative conclusion that was considered but not chosen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeConclusion {
    /// The alternative conclusion
    pub conclusion: String,
    /// Why it wasn't chosen
    pub reason_rejected: String,
    /// Confidence it would have had
    pub confidence: f32,
}

/// The knowledge base system
pub struct KnowledgeBase {
    /// All knowledge entries indexed by ID
    entries: HashMap<String, KnowledgeEntry>,
    /// Index by category for fast lookup
    category_index: HashMap<KnowledgeCategory, Vec<String>>,
    /// Index by concept for fast lookup
    concept_index: HashMap<String, Vec<String>>,
}

impl KnowledgeBase {
    /// Create a new knowledge base
    pub fn new() -> Self {
        let mut kb = Self {
            entries: HashMap::new(),
            category_index: HashMap::new(),
            concept_index: HashMap::new(),
        };
        kb.populate_default_knowledge();
        kb
    }

    /// Populate with default knowledge
    fn populate_default_knowledge(&mut self) {
        // Syntax rules
        self.add_entry(KnowledgeEntry {
            id: "syntax_001".to_string(),
            category: KnowledgeCategory::SyntaxRule,
            content: "All statements in Ovie must end with a semicolon".to_string(),
            related_concepts: vec!["syntax".to_string(), "statements".to_string()],
            confidence: 1.0,
            source: "Ovie Language Specification".to_string(),
        });

        self.add_entry(KnowledgeEntry {
            id: "syntax_002".to_string(),
            category: KnowledgeCategory::SyntaxRule,
            content: "Variable declarations use the 'let' keyword followed by an identifier".to_string(),
            related_concepts: vec!["syntax".to_string(), "variables".to_string()],
            confidence: 1.0,
            source: "Ovie Language Specification".to_string(),
        });

        // Type rules
        self.add_entry(KnowledgeEntry {
            id: "type_001".to_string(),
            category: KnowledgeCategory::TypeRule,
            content: "Ovie has a strong static type system that prevents implicit type conversions".to_string(),
            related_concepts: vec!["types".to_string(), "type_safety".to_string()],
            confidence: 1.0,
            source: "Ovie Type System Specification".to_string(),
        });

        self.add_entry(KnowledgeEntry {
            id: "type_002".to_string(),
            category: KnowledgeCategory::TypeRule,
            content: "Type annotations can be explicit or inferred from context".to_string(),
            related_concepts: vec!["types".to_string(), "type_inference".to_string()],
            confidence: 1.0,
            source: "Ovie Type System Specification".to_string(),
        });

        // Best practices
        self.add_entry(KnowledgeEntry {
            id: "best_001".to_string(),
            category: KnowledgeCategory::BestPractice,
            content: "Always handle Option and Result types explicitly rather than using unwrap()".to_string(),
            related_concepts: vec!["error_handling".to_string(), "safety".to_string()],
            confidence: 0.95,
            source: "Ovie Best Practices Guide".to_string(),
        });

        self.add_entry(KnowledgeEntry {
            id: "best_002".to_string(),
            category: KnowledgeCategory::BestPractice,
            content: "Use descriptive variable names that convey intent".to_string(),
            related_concepts: vec!["style".to_string(), "readability".to_string()],
            confidence: 0.9,
            source: "Ovie Style Guide".to_string(),
        });

        // Performance guidelines
        self.add_entry(KnowledgeEntry {
            id: "perf_001".to_string(),
            category: KnowledgeCategory::PerformanceGuideline,
            content: "Avoid nested loops when possible; consider using hash tables or sorting".to_string(),
            related_concepts: vec!["performance".to_string(), "algorithms".to_string()],
            confidence: 0.85,
            source: "Ovie Performance Guide".to_string(),
        });

        self.add_entry(KnowledgeEntry {
            id: "perf_002".to_string(),
            category: KnowledgeCategory::PerformanceGuideline,
            content: "Preallocate collections when the size is known to avoid reallocations".to_string(),
            related_concepts: vec!["performance".to_string(), "memory".to_string()],
            confidence: 0.9,
            source: "Ovie Performance Guide".to_string(),
        });

        // Security guidelines
        self.add_entry(KnowledgeEntry {
            id: "sec_001".to_string(),
            category: KnowledgeCategory::SecurityGuideline,
            content: "Always validate array indices before access to prevent out-of-bounds errors".to_string(),
            related_concepts: vec!["security".to_string(), "memory_safety".to_string()],
            confidence: 1.0,
            source: "Ovie Security Guide".to_string(),
        });

        self.add_entry(KnowledgeEntry {
            id: "sec_002".to_string(),
            category: KnowledgeCategory::SecurityGuideline,
            content: "Sanitize all user input before processing to prevent injection attacks".to_string(),
            related_concepts: vec!["security".to_string(), "input_validation".to_string()],
            confidence: 1.0,
            source: "Ovie Security Guide".to_string(),
        });

        // Common patterns
        self.add_entry(KnowledgeEntry {
            id: "pattern_001".to_string(),
            category: KnowledgeCategory::CommonPattern,
            content: "Use match expressions for exhaustive pattern matching on enums".to_string(),
            related_concepts: vec!["patterns".to_string(), "enums".to_string()],
            confidence: 0.95,
            source: "Ovie Patterns Guide".to_string(),
        });

        // Anti-patterns
        self.add_entry(KnowledgeEntry {
            id: "anti_001".to_string(),
            category: KnowledgeCategory::AntiPattern,
            content: "Avoid using unwrap() in production code; use proper error handling instead".to_string(),
            related_concepts: vec!["error_handling".to_string(), "safety".to_string()],
            confidence: 0.95,
            source: "Ovie Anti-Patterns Guide".to_string(),
        });
    }

    /// Add a knowledge entry
    pub fn add_entry(&mut self, entry: KnowledgeEntry) {
        let id = entry.id.clone();
        let category = entry.category;
        let concepts = entry.related_concepts.clone();

        // Add to main storage
        self.entries.insert(id.clone(), entry);

        // Update category index
        self.category_index
            .entry(category)
            .or_insert_with(Vec::new)
            .push(id.clone());

        // Update concept index
        for concept in concepts {
            self.concept_index
                .entry(concept)
                .or_insert_with(Vec::new)
                .push(id.clone());
        }
    }

    /// Get entry by ID
    pub fn get_entry(&self, id: &str) -> Option<&KnowledgeEntry> {
        self.entries.get(id)
    }

    /// Get all entries in a category
    pub fn get_by_category(&self, category: KnowledgeCategory) -> Vec<&KnowledgeEntry> {
        self.category_index
            .get(&category)
            .map(|ids| ids.iter().filter_map(|id| self.entries.get(id)).collect())
            .unwrap_or_default()
    }

    /// Get all entries related to a concept
    pub fn get_by_concept(&self, concept: &str) -> Vec<&KnowledgeEntry> {
        self.concept_index
            .get(concept)
            .map(|ids| ids.iter().filter_map(|id| self.entries.get(id)).collect())
            .unwrap_or_default()
    }

    /// Search for relevant knowledge
    pub fn search(&self, query: &str) -> Vec<&KnowledgeEntry> {
        let query_lower = query.to_lowercase();
        self.entries
            .values()
            .filter(|entry| {
                entry.content.to_lowercase().contains(&query_lower)
                    || entry.related_concepts.iter().any(|c| c.to_lowercase().contains(&query_lower))
            })
            .collect()
    }
}

impl Default for KnowledgeBase {
    fn default() -> Self {
        Self::new()
    }
}

/// The inference engine that performs reasoning
pub struct InferenceEngine {
    /// Knowledge base to reason over
    knowledge_base: KnowledgeBase,
    /// Reasoning chains generated
    reasoning_chains: Vec<ReasoningChain>,
}

impl InferenceEngine {
    /// Create a new inference engine
    pub fn new() -> Self {
        Self {
            knowledge_base: KnowledgeBase::new(),
            reasoning_chains: Vec::new(),
        }
    }

    /// Create with custom knowledge base
    pub fn with_knowledge_base(knowledge_base: KnowledgeBase) -> Self {
        Self {
            knowledge_base,
            reasoning_chains: Vec::new(),
        }
    }

    /// Reason about a diagnostic
    pub fn reason_about_diagnostic(&mut self, diagnostic: &Diagnostic) -> AprokoResult<ReasoningChain> {
        let question = format!("Why did this diagnostic occur: {}", diagnostic.message);
        
        let mut steps = Vec::new();
        let mut overall_confidence = 0.0;

        // Step 1: Identify the category
        steps.push(ReasoningStep {
            step_number: 1,
            description: format!("Identified diagnostic category: {:?}", diagnostic.category),
            knowledge_used: vec![],
            conclusion: format!("This is a {:?} diagnostic", diagnostic.category),
            confidence: 1.0,
        });

        // Step 2: Search for relevant knowledge
        let relevant_knowledge = self.knowledge_base.search(&diagnostic.message);
        let knowledge_ids: Vec<String> = relevant_knowledge.iter().map(|k| k.id.clone()).collect();
        
        steps.push(ReasoningStep {
            step_number: 2,
            description: format!("Found {} relevant knowledge entries", relevant_knowledge.len()),
            knowledge_used: knowledge_ids.clone(),
            conclusion: format!("Located {} applicable rules and guidelines", relevant_knowledge.len()),
            confidence: if relevant_knowledge.is_empty() { 0.5 } else { 0.9 },
        });

        // Step 3: Apply knowledge to understand the issue
        let mut conclusion_parts = Vec::new();
        for entry in &relevant_knowledge {
            conclusion_parts.push(entry.content.clone());
            overall_confidence += entry.confidence;
        }

        if !relevant_knowledge.is_empty() {
            overall_confidence /= relevant_knowledge.len() as f32;
        } else {
            overall_confidence = 0.5;
        }

        steps.push(ReasoningStep {
            step_number: 3,
            description: "Applied relevant knowledge to understand the issue".to_string(),
            knowledge_used: knowledge_ids,
            conclusion: if conclusion_parts.is_empty() {
                "No specific rules found; using general diagnostic analysis".to_string()
            } else {
                conclusion_parts.join(". ")
            },
            confidence: overall_confidence,
        });

        let final_conclusion = format!(
            "The diagnostic '{}' occurred because: {}",
            diagnostic.message,
            if conclusion_parts.is_empty() {
                "the code violates language rules or best practices"
            } else {
                &conclusion_parts[0]
            }
        );

        let chain = ReasoningChain {
            id: format!("chain_{}", self.reasoning_chains.len()),
            question,
            steps,
            conclusion: final_conclusion,
            overall_confidence,
            alternatives: vec![],
        };

        self.reasoning_chains.push(chain.clone());
        Ok(chain)
    }

    /// Reason about a finding
    pub fn reason_about_finding(&mut self, finding: &Finding) -> AprokoResult<ReasoningChain> {
        let question = format!("Why was this finding reported: {}", finding.message);
        
        let mut steps = Vec::new();

        // Step 1: Identify category and severity
        steps.push(ReasoningStep {
            step_number: 1,
            description: format!("Identified finding: {:?} with severity {:?}", finding.category, finding.severity),
            knowledge_used: vec![],
            conclusion: format!("This is a {:?} issue at {:?} severity", finding.category, finding.severity),
            confidence: 1.0,
        });

        // Step 2: Search for relevant knowledge
        let category_knowledge = self.knowledge_base.get_by_category(match finding.category {
            crate::AnalysisCategory::Syntax => KnowledgeCategory::SyntaxRule,
            crate::AnalysisCategory::Performance => KnowledgeCategory::PerformanceGuideline,
            crate::AnalysisCategory::Security => KnowledgeCategory::SecurityGuideline,
            _ => KnowledgeCategory::BestPractice,
        });

        let knowledge_ids: Vec<String> = category_knowledge.iter().map(|k| k.id.clone()).collect();
        
        steps.push(ReasoningStep {
            step_number: 2,
            description: format!("Found {} relevant knowledge entries for this category", category_knowledge.len()),
            knowledge_used: knowledge_ids.clone(),
            conclusion: format!("Located {} applicable rules", category_knowledge.len()),
            confidence: 0.85,
        });

        // Step 3: Generate conclusion
        let conclusion = if let Some(suggestion) = &finding.suggestion {
            format!("The finding '{}' was reported because: {}. Suggested fix: {}", 
                finding.message, 
                if category_knowledge.is_empty() { "it violates code quality standards" } else { &category_knowledge[0].content },
                suggestion
            )
        } else {
            format!("The finding '{}' was reported because: {}", 
                finding.message,
                if category_knowledge.is_empty() { "it violates code quality standards" } else { &category_knowledge[0].content }
            )
        };

        steps.push(ReasoningStep {
            step_number: 3,
            description: "Generated explanation based on knowledge".to_string(),
            knowledge_used: knowledge_ids,
            conclusion: conclusion.clone(),
            confidence: 0.8,
        });

        let chain = ReasoningChain {
            id: format!("chain_{}", self.reasoning_chains.len()),
            question,
            steps,
            conclusion,
            overall_confidence: 0.8,
            alternatives: vec![],
        };

        self.reasoning_chains.push(chain.clone());
        Ok(chain)
    }

    /// Get all reasoning chains
    pub fn get_reasoning_chains(&self) -> &[ReasoningChain] {
        &self.reasoning_chains
    }

    /// Get the knowledge base
    pub fn knowledge_base(&self) -> &KnowledgeBase {
        &self.knowledge_base
    }

    /// Get mutable knowledge base
    pub fn knowledge_base_mut(&mut self) -> &mut KnowledgeBase {
        &mut self.knowledge_base
    }
}

impl Default for InferenceEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// The main reasoning engine that coordinates everything
pub struct ReasoningEngine {
    /// Inference engine for logical reasoning
    inference_engine: InferenceEngine,
    /// Configuration
    config: ReasoningConfig,
}

/// Configuration for the reasoning engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningConfig {
    /// Maximum reasoning depth
    pub max_reasoning_depth: usize,
    /// Minimum confidence threshold
    pub min_confidence_threshold: f32,
    /// Whether to track all reasoning chains
    pub track_all_chains: bool,
}

impl Default for ReasoningConfig {
    fn default() -> Self {
        Self {
            max_reasoning_depth: 10,
            min_confidence_threshold: 0.5,
            track_all_chains: true,
        }
    }
}

impl ReasoningEngine {
    /// Create a new reasoning engine
    pub fn new() -> Self {
        Self {
            inference_engine: InferenceEngine::new(),
            config: ReasoningConfig::default(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: ReasoningConfig) -> Self {
        Self {
            inference_engine: InferenceEngine::new(),
            config,
        }
    }

    /// Explain a diagnostic with reasoning
    pub fn explain_diagnostic(&mut self, diagnostic: &Diagnostic) -> AprokoResult<(Explanation, ReasoningChain)> {
        // Generate reasoning chain
        let chain = self.inference_engine.reason_about_diagnostic(diagnostic)?;

        // Generate explanation based on reasoning
        let explanation = Explanation {
            explanation_type: ExplanationType::DiagnosticExplanation,
            summary: format!("Diagnostic: {}", diagnostic.message),
            detailed_explanation: chain.conclusion.clone(),
            code_examples: vec![],
            fix_suggestions: vec![],
            related_topics: vec![],
            external_resources: vec![],
            confidence: chain.overall_confidence,
        };

        Ok((explanation, chain))
    }

    /// Explain a finding with reasoning
    pub fn explain_finding(&mut self, finding: &Finding) -> AprokoResult<(Explanation, ReasoningChain)> {
        // Generate reasoning chain
        let chain = self.inference_engine.reason_about_finding(finding)?;

        // Generate explanation based on reasoning
        let explanation = Explanation {
            explanation_type: ExplanationType::DiagnosticExplanation,
            summary: format!("Finding: {}", finding.message),
            detailed_explanation: chain.conclusion.clone(),
            code_examples: vec![],
            fix_suggestions: vec![],
            related_topics: vec![],
            external_resources: vec![],
            confidence: chain.overall_confidence,
        };

        Ok((explanation, chain))
    }

    /// Get the inference engine
    pub fn inference_engine(&self) -> &InferenceEngine {
        &self.inference_engine
    }

    /// Get mutable inference engine
    pub fn inference_engine_mut(&mut self) -> &mut InferenceEngine {
        &mut self.inference_engine
    }

    /// Get configuration
    pub fn config(&self) -> &ReasoningConfig {
        &self.config
    }

    /// Set configuration
    pub fn set_config(&mut self, config: ReasoningConfig) {
        self.config = config;
    }
}

impl Default for ReasoningEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostic::{DiagnosticCategory, SourceLocation};

    #[test]
    fn test_knowledge_base_creation() {
        let kb = KnowledgeBase::new();
        assert!(!kb.entries.is_empty());
    }

    #[test]
    fn test_knowledge_base_search() {
        let kb = KnowledgeBase::new();
        let results = kb.search("semicolon");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_knowledge_base_category_lookup() {
        let kb = KnowledgeBase::new();
        let syntax_rules = kb.get_by_category(KnowledgeCategory::SyntaxRule);
        assert!(!syntax_rules.is_empty());
    }

    #[test]
    fn test_inference_engine_diagnostic_reasoning() {
        let mut engine = InferenceEngine::new();
        let diagnostic = Diagnostic {
            rule_id: "test_001".to_string(),
            category: DiagnosticCategory::SyntaxError,
            severity: crate::Severity::Error,
            message: "Missing semicolon".to_string(),
            explanation: None,
            suggestion: None,
            location: SourceLocation {
                file: "test.ov".to_string(),
                line: 1,
                column: 10,
                span_length: 1,
                source_excerpt: None,
            },
            related_locations: vec![],
            metadata: HashMap::new(),
        };

        let result = engine.reason_about_diagnostic(&diagnostic);
        assert!(result.is_ok());
        
        let chain = result.unwrap();
        assert!(!chain.steps.is_empty());
        assert!(!chain.conclusion.is_empty());
    }

    #[test]
    fn test_reasoning_engine_explain_diagnostic() {
        let mut engine = ReasoningEngine::new();
        let diagnostic = Diagnostic {
            rule_id: "test_002".to_string(),
            category: DiagnosticCategory::TypeError,
            severity: crate::Severity::Error,
            message: "Type mismatch".to_string(),
            explanation: None,
            suggestion: None,
            location: SourceLocation {
                file: "test.ov".to_string(),
                line: 5,
                column: 15,
                span_length: 3,
                source_excerpt: None,
            },
            related_locations: vec![],
            metadata: HashMap::new(),
        };

        let result = engine.explain_diagnostic(&diagnostic);
        assert!(result.is_ok());
        
        let (explanation, chain) = result.unwrap();
        assert!(!explanation.detailed_explanation.is_empty());
        assert!(!chain.steps.is_empty());
    }
}
