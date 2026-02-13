use crate::ast::{AstNode, Statement, Expression};
use crate::error::OvieResult;
use std::collections::HashMap;

/// Represents a correction made by the normalizer
#[derive(Debug, Clone)]
pub struct Correction {
    pub original: String,
    pub corrected: String,
    pub reason: String,
    pub line: usize,
    pub column: usize,
}

/// The Normalizer performs safe auto-correction of typos and syntax normalization
pub struct Normalizer {
    corrections: Vec<Correction>,
    typo_corrections: HashMap<String, String>,
    enable_auto_correction: bool,
}

impl Normalizer {
    pub fn new() -> Self {
        let mut typo_corrections = HashMap::new();
        
        // Common typos for Ovie keywords
        typo_corrections.insert("seeam".to_string(), "seeAm".to_string());
        typo_corrections.insert("SEEAM".to_string(), "seeAm".to_string());
        typo_corrections.insert("see_am".to_string(), "seeAm".to_string());
        typo_corrections.insert("see am".to_string(), "seeAm".to_string());
        typo_corrections.insert("print".to_string(), "seeAm".to_string());
        typo_corrections.insert("println".to_string(), "seeAm".to_string());
        typo_corrections.insert("console.log".to_string(), "seeAm".to_string());
        
        // Function keyword typos
        typo_corrections.insert("function".to_string(), "fn".to_string());
        typo_corrections.insert("func".to_string(), "fn".to_string());
        typo_corrections.insert("def".to_string(), "fn".to_string());
        
        // Control flow typos
        typo_corrections.insert("elsif".to_string(), "else if".to_string());
        typo_corrections.insert("elif".to_string(), "else if".to_string());
        
        // Variable declaration typos
        typo_corrections.insert("var".to_string(), "mut".to_string());
        typo_corrections.insert("let".to_string(), "".to_string()); // Remove let, use immutable by default
        typo_corrections.insert("const".to_string(), "".to_string());
        
        Self {
            corrections: Vec::new(),
            typo_corrections,
            enable_auto_correction: true,
        }
    }

    /// Enable or disable auto-correction
    pub fn set_auto_correction(&mut self, enabled: bool) {
        self.enable_auto_correction = enabled;
    }

    /// Normalize an AST, performing safe corrections and logging changes
    pub fn normalize(&mut self, mut ast: AstNode) -> OvieResult<(AstNode, Vec<Correction>)> {
        if !self.enable_auto_correction {
            return Ok((ast, self.corrections.clone()));
        }

        // Clear previous corrections
        self.corrections.clear();

        // Normalize statements
        match &mut ast {
            AstNode::Program(statements) => {
                for statement in statements {
                    self.normalize_statement(statement)?;
                }
            }
        }

        // Normalize whitespace and formatting
        self.normalize_formatting(&mut ast)?;

        Ok((ast, self.corrections.clone()))
    }

    /// Normalize a single statement
    fn normalize_statement(&mut self, statement: &mut Statement) -> OvieResult<()> {
        match statement {
            Statement::Print { expression } => {
                self.normalize_expression(expression)?;
            }
            Statement::Assignment { identifier, value, .. } => {
                self.normalize_identifier(identifier)?;
                self.normalize_expression(value)?;
            }
            Statement::VariableDeclaration { identifier, value, .. } => {
                self.normalize_identifier(identifier)?;
                self.normalize_expression(value)?;
            }
            Statement::Function { name, parameters, body } => {
                self.normalize_identifier(name)?;
                for param in parameters {
                    self.normalize_identifier(param)?;
                }
                for stmt in body {
                    self.normalize_statement(stmt)?;
                }
            }
            Statement::FunctionDeclaration { name, parameters, body } => {
                self.normalize_identifier(name)?;
                for param in parameters {
                    self.normalize_identifier(param)?;
                }
                for stmt in body {
                    self.normalize_statement(stmt)?;
                }
            }
            Statement::If { condition, then_block, else_block } => {
                self.normalize_expression(condition)?;
                for stmt in then_block {
                    self.normalize_statement(stmt)?;
                }
                if let Some(else_stmts) = else_block {
                    for stmt in else_stmts {
                        self.normalize_statement(stmt)?;
                    }
                }
            }
            Statement::While { condition, body } => {
                self.normalize_expression(condition)?;
                for stmt in body {
                    self.normalize_statement(stmt)?;
                }
            }
            Statement::For { identifier, iterable, body } => {
                self.normalize_identifier(identifier)?;
                self.normalize_expression(iterable)?;
                for stmt in body {
                    self.normalize_statement(stmt)?;
                }
            }
            Statement::Return { value } => {
                if let Some(expr) = value {
                    self.normalize_expression(expr)?;
                }
            }
            Statement::Expression { expression } => {
                self.normalize_expression(expression)?;
            }
            Statement::Struct { name, .. } => {
                self.normalize_identifier(name)?;
            }
            Statement::Enum { name, .. } => {
                self.normalize_identifier(name)?;
            }
        }
        Ok(())
    }

    /// Normalize an expression
    fn normalize_expression(&mut self, expression: &mut Expression) -> OvieResult<()> {
        match expression {
            Expression::Identifier(name) => {
                self.normalize_identifier(name)?;
            }
            Expression::Binary { left, right, .. } => {
                self.normalize_expression(left)?;
                self.normalize_expression(right)?;
            }
            Expression::Unary { operand, .. } => {
                self.normalize_expression(operand)?;
            }
            Expression::Call { function, arguments } => {
                self.normalize_identifier(function)?;
                for arg in arguments {
                    self.normalize_expression(arg)?;
                }
            }
            Expression::FieldAccess { object, field } => {
                self.normalize_expression(object)?;
                self.normalize_identifier(field)?;
            }
            Expression::StructInstantiation { struct_name, fields } => {
                self.normalize_identifier(struct_name)?;
                for field in fields {
                    // Note: field.name is not mutable in the current AST design
                    // This would require AST changes to make fields mutable
                    self.normalize_expression(&mut field.value)?;
                }
            }
            Expression::Range { start, end } => {
                self.normalize_expression(start)?;
                self.normalize_expression(end)?;
            }
            Expression::EnumVariantConstruction { enum_name, variant_name, data } => {
                self.normalize_identifier(enum_name)?;
                self.normalize_identifier(variant_name)?;
                if let Some(data_expr) = data {
                    self.normalize_expression(data_expr)?;
                }
            }
            Expression::Index { object, index } => {
                self.normalize_expression(object)?;
                self.normalize_expression(index)?;
            }
            Expression::ArrayLiteral { elements } => {
                for element in elements {
                    self.normalize_expression(element)?;
                }
            }
            Expression::Literal(_) => {
                // Literals don't need normalization
            }
        }
        Ok(())
    }

    /// Normalize an identifier (check for typos)
    fn normalize_identifier(&mut self, identifier: &mut String) -> OvieResult<()> {
        let original = identifier.clone();
        
        // Check for common typos
        if let Some(correction) = self.typo_corrections.get(&original.to_lowercase()) {
            if self.is_safe_correction(&original, correction) {
                *identifier = correction.clone();
                self.log_correction(Correction {
                    original: original.clone(),
                    corrected: correction.clone(),
                    reason: format!("Corrected common typo '{}' to '{}'", original, correction),
                    line: 1, // TODO: Get actual line number from AST
                    column: 1, // TODO: Get actual column number from AST
                });
            }
        }

        // Normalize naming conventions
        if original.contains("_") && !original.starts_with("_") {
            let camel_case = self.to_camel_case(&original);
            if camel_case != original && self.is_safe_correction(&original, &camel_case) {
                *identifier = camel_case.clone();
                self.log_correction(Correction {
                    original: original.clone(),
                    corrected: camel_case,
                    reason: "Normalized to camelCase naming convention".to_string(),
                    line: 1,
                    column: 1,
                });
            }
        }

        Ok(())
    }

    /// Normalize formatting and whitespace
    fn normalize_formatting(&mut self, _ast: &mut AstNode) -> OvieResult<()> {
        // TODO: Implement whitespace normalization
        // This would involve pretty-printing the AST with consistent formatting
        Ok(())
    }

    /// Convert snake_case to camelCase
    fn to_camel_case(&self, input: &str) -> String {
        let parts: Vec<&str> = input.split('_').collect();
        if parts.len() <= 1 {
            return input.to_string();
        }

        let mut result = parts[0].to_string();
        for part in &parts[1..] {
            if !part.is_empty() {
                let mut chars = part.chars();
                if let Some(first) = chars.next() {
                    result.push(first.to_uppercase().next().unwrap_or(first));
                    result.extend(chars);
                }
            }
        }
        result
    }

    /// Check if a correction is safe (doesn't change semantic meaning)
    pub fn is_safe_correction(&self, original: &str, corrected: &str) -> bool {
        // Don't correct if the strings are the same
        if original == corrected {
            return false;
        }

        // Don't correct if the corrected version is empty and original isn't
        if corrected.is_empty() && !original.is_empty() {
            return false;
        }

        // Don't correct if it would change a likely intentional name
        if original.len() > 10 {
            return false;
        }

        // Don't correct if the original contains numbers (likely intentional)
        if original.chars().any(|c| c.is_ascii_digit()) {
            return false;
        }

        true
    }

    /// Log a correction that was made
    fn log_correction(&mut self, correction: Correction) {
        self.corrections.push(correction);
    }

    /// Normalize source code before lexing (fix typos at source level)
    pub fn normalize_source(&mut self, source: &str) -> (String, Vec<Correction>) {
        let mut normalized = source.to_string();
        let mut corrections = Vec::new();

        // Sort typos by length (descending) to match longer patterns first
        let mut sorted_typos: Vec<_> = self.typo_corrections.iter().collect();
        sorted_typos.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

        // Apply source-level corrections
        for (typo, correction) in sorted_typos {
            if normalized.contains(typo) {
                let corrected_source = normalized.replace(typo, correction);
                if corrected_source != normalized {
                    corrections.push(Correction {
                        original: typo.clone(),
                        corrected: correction.clone(),
                        reason: format!("Corrected '{}' to '{}'", typo, correction),
                        line: 1, // TODO: Calculate actual line
                        column: 1, // TODO: Calculate actual column
                    });
                    normalized = corrected_source;
                }
            }
        }

        (normalized, corrections)
    }

    /// Get all corrections made during normalization
    pub fn get_corrections(&self) -> &[Correction] {
        &self.corrections
    }
}

impl Default for Normalizer {
    fn default() -> Self {
        Self::new()
    }
}