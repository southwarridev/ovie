//! Correctness analysis for ownership rules and memory safety

use crate::{Analyzer, AnalysisCategory, AprokoResult, Finding, Severity, CategoryConfig};
use oviec::ast::{AstNode, Statement, Expression, Literal};
use std::collections::{HashMap, HashSet};

/// Analyzer for ownership correctness and memory safety
pub struct CorrectnessAnalyzer {
    enforce_ownership: bool,
    check_state_transitions: bool,
}

/// Represents the ownership state of a variable
#[derive(Debug, Clone, PartialEq)]
enum OwnershipState {
    /// Variable is owned and can be used
    Owned,
    /// Variable has been moved and cannot be used
    Moved,
    /// Variable is borrowed immutably
    BorrowedImmutable,
    /// Variable is borrowed mutably
    BorrowedMutable,
    /// Variable is uninitialized
    Uninitialized,
}

/// Tracks variable ownership and borrowing
#[derive(Debug, Clone)]
struct OwnershipTracker {
    variables: HashMap<String, OwnershipState>,
    borrows: HashMap<String, Vec<String>>, // variable -> list of borrowers
}

impl OwnershipTracker {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
            borrows: HashMap::new(),
        }
    }

    fn declare_variable(&mut self, name: String, mutable: bool) {
        let state = if mutable {
            OwnershipState::Owned
        } else {
            OwnershipState::Owned
        };
        self.variables.insert(name, state);
    }

    fn move_variable(&mut self, name: &str) -> bool {
        if let Some(state) = self.variables.get_mut(name) {
            match state {
                OwnershipState::Owned => {
                    *state = OwnershipState::Moved;
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    }

    fn borrow_variable(&mut self, name: &str, mutable: bool) -> bool {
        if let Some(state) = self.variables.get(name) {
            match state {
                OwnershipState::Owned => {
                    if mutable {
                        // Can only have one mutable borrow
                        if self.borrows.get(name).map_or(0, |b| b.len()) == 0 {
                            self.borrows.entry(name.to_string()).or_default().push("mutable".to_string());
                            true
                        } else {
                            false
                        }
                    } else {
                        // Can have multiple immutable borrows
                        self.borrows.entry(name.to_string()).or_default().push("immutable".to_string());
                        true
                    }
                }
                OwnershipState::BorrowedImmutable if !mutable => {
                    // Can add more immutable borrows
                    self.borrows.entry(name.to_string()).or_default().push("immutable".to_string());
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    }

    fn is_usable(&self, name: &str) -> bool {
        if let Some(state) = self.variables.get(name) {
            !matches!(state, OwnershipState::Moved | OwnershipState::Uninitialized)
        } else {
            false
        }
    }

    fn get_state(&self, name: &str) -> Option<&OwnershipState> {
        self.variables.get(name)
    }
}

impl CorrectnessAnalyzer {
    pub fn new() -> Self {
        Self {
            enforce_ownership: true,
            check_state_transitions: true,
        }
    }

    /// Enforce ownership rules throughout the program
    fn enforce_ownership_rules(&self, ast: &AstNode) -> Vec<Finding> {
        let mut findings = Vec::new();
        
        if !self.enforce_ownership {
            return findings;
        }

        let mut tracker = OwnershipTracker::new();
        
        match ast {
            AstNode::Program(statements) => {
                for (i, statement) in statements.iter().enumerate() {
                    findings.extend(self.check_statement_ownership(statement, &mut tracker, i + 1));
                }
            }
        }

        findings
    }

    /// Check ownership rules for individual statements
    fn check_statement_ownership(&self, statement: &Statement, tracker: &mut OwnershipTracker, line: usize) -> Vec<Finding> {
        let mut findings = Vec::new();

        match statement {
            Statement::Assignment { identifier, value, mutable } => {
                // Check if the value expression uses any moved variables
                findings.extend(self.check_expression_ownership(value, tracker, line));
                
                // Declare or reassign the variable
                if tracker.variables.contains_key(identifier) {
                    // Reassignment - check if variable is mutable
                    if let Some(state) = tracker.get_state(identifier) {
                        match state {
                            OwnershipState::Moved => {
                                findings.push(Finding {
                                    category: AnalysisCategory::Correctness,
                                    severity: Severity::Error,
                                    message: format!("Cannot assign to moved variable '{}'", identifier),
                                    suggestion: Some("Variable has been moved and cannot be used".to_string()),
                                    location: (line, 1),
                                    span_length: identifier.len(),
                                    rule_id: "use_after_move".to_string(),
                                });
                            }
                            OwnershipState::BorrowedMutable | OwnershipState::BorrowedImmutable => {
                                findings.push(Finding {
                                    category: AnalysisCategory::Correctness,
                                    severity: Severity::Error,
                                    message: format!("Cannot assign to borrowed variable '{}'", identifier),
                                    suggestion: Some("Wait for borrows to end before reassigning".to_string()),
                                    location: (line, 1),
                                    span_length: identifier.len(),
                                    rule_id: "assign_to_borrowed".to_string(),
                                });
                            }
                            _ => {}
                        }
                    }
                } else {
                    // New variable declaration
                    tracker.declare_variable(identifier.clone(), *mutable);
                }

                // Check if the assignment moves the value
                if self.is_move_expression(value) {
                    if let Expression::Identifier(moved_var) = value {
                        if !tracker.move_variable(moved_var) {
                            findings.push(Finding {
                                category: AnalysisCategory::Correctness,
                                severity: Severity::Error,
                                message: format!("Cannot move from variable '{}'", moved_var),
                                suggestion: Some("Variable is not in a movable state".to_string()),
                                location: (line, 1),
                                span_length: moved_var.len(),
                                rule_id: "invalid_move".to_string(),
                            });
                        }
                    }
                }
            }
            Statement::Function { parameters, body, .. } => {
                // Create new scope for function
                let mut function_tracker = tracker.clone();
                
                // Parameters are owned in function scope
                for param in parameters {
                    function_tracker.declare_variable(param.clone(), true);
                }
                
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_ownership(stmt, &mut function_tracker, line + i + 1));
                }
            }
            Statement::If { condition, then_block, else_block } => {
                findings.extend(self.check_expression_ownership(condition, tracker, line));
                
                // Each branch gets its own tracker state
                let mut then_tracker = tracker.clone();
                for (i, stmt) in then_block.iter().enumerate() {
                    findings.extend(self.check_statement_ownership(stmt, &mut then_tracker, line + i + 1));
                }
                
                if let Some(else_stmts) = else_block {
                    let mut else_tracker = tracker.clone();
                    for (i, stmt) in else_stmts.iter().enumerate() {
                        findings.extend(self.check_statement_ownership(stmt, &mut else_tracker, line + i + 1));
                    }
                }
            }
            Statement::While { condition, body } => {
                findings.extend(self.check_expression_ownership(condition, tracker, line));
                
                // Loop body can modify tracker state
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_ownership(stmt, tracker, line + i + 1));
                }
            }
            Statement::For { identifier, iterable, body } => {
                findings.extend(self.check_expression_ownership(iterable, tracker, line));
                
                // Iterator variable is owned in loop scope
                let mut loop_tracker = tracker.clone();
                loop_tracker.declare_variable(identifier.clone(), false);
                
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_ownership(stmt, &mut loop_tracker, line + i + 1));
                }
            }
            Statement::Return { value } => {
                if let Some(expr) = value {
                    findings.extend(self.check_expression_ownership(expr, tracker, line));
                }
            }
            Statement::Print { expression } => {
                findings.extend(self.check_expression_ownership(expression, tracker, line));
            }
            Statement::Expression { expression } => {
                findings.extend(self.check_expression_ownership(expression, tracker, line));
            }
            _ => {}
        }

        findings
    }

    /// Check ownership rules for expressions
    fn check_expression_ownership(&self, expression: &Expression, tracker: &mut OwnershipTracker, line: usize) -> Vec<Finding> {
        let mut findings = Vec::new();

        match expression {
            Expression::Identifier(name) => {
                if !tracker.is_usable(name) {
                    if let Some(state) = tracker.get_state(name) {
                        match state {
                            OwnershipState::Moved => {
                                findings.push(Finding {
                                    category: AnalysisCategory::Correctness,
                                    severity: Severity::Error,
                                    message: format!("Use of moved variable '{}'", name),
                                    suggestion: Some("Variable has been moved and cannot be used".to_string()),
                                    location: (line, 1),
                                    span_length: name.len(),
                                    rule_id: "use_after_move".to_string(),
                                });
                            }
                            OwnershipState::Uninitialized => {
                                findings.push(Finding {
                                    category: AnalysisCategory::Correctness,
                                    severity: Severity::Error,
                                    message: format!("Use of uninitialized variable '{}'", name),
                                    suggestion: Some("Initialize the variable before using it".to_string()),
                                    location: (line, 1),
                                    span_length: name.len(),
                                    rule_id: "use_uninitialized".to_string(),
                                });
                            }
                            _ => {}
                        }
                    } else {
                        findings.push(Finding {
                            category: AnalysisCategory::Correctness,
                            severity: Severity::Error,
                            message: format!("Use of undeclared variable '{}'", name),
                            suggestion: Some("Declare the variable before using it".to_string()),
                            location: (line, 1),
                            span_length: name.len(),
                            rule_id: "use_undeclared".to_string(),
                        });
                    }
                }
            }
            Expression::Binary { left, right, .. } => {
                findings.extend(self.check_expression_ownership(left, tracker, line));
                findings.extend(self.check_expression_ownership(right, tracker, line));
            }
            Expression::Unary { operand, .. } => {
                findings.extend(self.check_expression_ownership(operand, tracker, line));
            }
            Expression::Call { arguments, .. } => {
                for arg in arguments {
                    findings.extend(self.check_expression_ownership(arg, tracker, line));
                    
                    // Function calls might move arguments
                    if self.is_move_expression(arg) {
                        if let Expression::Identifier(moved_var) = arg {
                            if !tracker.move_variable(moved_var) {
                                findings.push(Finding {
                                    category: AnalysisCategory::Correctness,
                                    severity: Severity::Error,
                                    message: format!("Cannot move variable '{}' in function call", moved_var),
                                    suggestion: Some("Variable is not in a movable state".to_string()),
                                    location: (line, 1),
                                    span_length: moved_var.len(),
                                    rule_id: "invalid_move_call".to_string(),
                                });
                            }
                        }
                    }
                }
            }
            Expression::FieldAccess { object, .. } => {
                findings.extend(self.check_expression_ownership(object, tracker, line));
            }
            Expression::StructInstantiation { fields, .. } => {
                for field in fields {
                    findings.extend(self.check_expression_ownership(&field.value, tracker, line));
                }
            }
            Expression::Range { start, end } => {
                findings.extend(self.check_expression_ownership(start, tracker, line));
                findings.extend(self.check_expression_ownership(end, tracker, line));
            }
            Expression::EnumVariantConstruction { data, .. } => {
                if let Some(data_expr) = data {
                    findings.extend(self.check_expression_ownership(data_expr, tracker, line));
                }
            }
            Expression::Index { object, index } => {
                findings.extend(self.check_expression_ownership(object, tracker, line));
                findings.extend(self.check_expression_ownership(index, tracker, line));
            }
            Expression::ArrayLiteral { elements } => {
                for element in elements {
                    findings.extend(self.check_expression_ownership(element, tracker, line));
                }
            }
            Expression::Literal(_) => {
                // Literals don't have ownership issues
            }
        }

        findings
    }

    /// Check if an expression represents a move operation
    fn is_move_expression(&self, expression: &Expression) -> bool {
        match expression {
            Expression::Identifier(_) => true, // Simple identifier access is a move
            Expression::Call { .. } => true,   // Function calls can move arguments
            _ => false,
        }
    }

    /// Check state transitions for validity
    fn check_state_transitions(&self, ast: &AstNode) -> Vec<Finding> {
        let mut findings = Vec::new();
        
        if !self.check_state_transitions {
            return findings;
        }

        // Track state changes throughout the program
        let mut state_tracker = HashMap::new();
        
        match ast {
            AstNode::Program(statements) => {
                for (i, statement) in statements.iter().enumerate() {
                    findings.extend(self.check_statement_state_transitions(statement, &mut state_tracker, i + 1));
                }
            }
        }

        findings
    }

    /// Check state transitions in statements
    fn check_statement_state_transitions(&self, statement: &Statement, state_tracker: &mut HashMap<String, String>, line: usize) -> Vec<Finding> {
        let mut findings = Vec::new();

        match statement {
            Statement::Assignment { identifier, value, .. } => {
                // Track state changes
                let new_state = self.infer_state_from_expression(value);
                
                if let Some(old_state) = state_tracker.get(identifier) {
                    if !self.is_valid_state_transition(old_state, &new_state) {
                        findings.push(Finding {
                            category: AnalysisCategory::Correctness,
                            severity: Severity::Warning,
                            message: format!("Invalid state transition for '{}': {} -> {}", identifier, old_state, new_state),
                            suggestion: Some("Ensure state transitions follow valid patterns".to_string()),
                            location: (line, 1),
                            span_length: identifier.len(),
                            rule_id: "invalid_state_transition".to_string(),
                        });
                    }
                }
                
                state_tracker.insert(identifier.clone(), new_state);
            }
            Statement::Function { body, .. } => {
                // Functions have their own state scope
                let mut function_state = state_tracker.clone();
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_state_transitions(stmt, &mut function_state, line + i + 1));
                }
            }
            Statement::If { then_block, else_block, .. } => {
                // Check both branches
                let mut then_state = state_tracker.clone();
                for (i, stmt) in then_block.iter().enumerate() {
                    findings.extend(self.check_statement_state_transitions(stmt, &mut then_state, line + i + 1));
                }
                
                if let Some(else_stmts) = else_block {
                    let mut else_state = state_tracker.clone();
                    for (i, stmt) in else_stmts.iter().enumerate() {
                        findings.extend(self.check_statement_state_transitions(stmt, &mut else_state, line + i + 1));
                    }
                }
            }
            Statement::While { body, .. } | Statement::For { body, .. } => {
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_state_transitions(stmt, state_tracker, line + i + 1));
                }
            }
            _ => {}
        }

        findings
    }

    /// Infer state from expression value
    fn infer_state_from_expression(&self, expression: &Expression) -> String {
        match expression {
            Expression::Literal(Literal::Boolean(true)) => "active".to_string(),
            Expression::Literal(Literal::Boolean(false)) => "inactive".to_string(),
            Expression::Literal(Literal::Number(n)) if *n == 0.0 => "empty".to_string(),
            Expression::Literal(Literal::Number(_)) => "filled".to_string(),
            Expression::Literal(Literal::String(s)) if s.is_empty() => "empty".to_string(),
            Expression::Literal(Literal::String(_)) => "filled".to_string(),
            Expression::Call { .. } => "initialized".to_string(),
            _ => "unknown".to_string(),
        }
    }

    /// Check if a state transition is valid
    fn is_valid_state_transition(&self, from: &str, to: &str) -> bool {
        match (from, to) {
            // Valid transitions
            ("empty", "filled") => true,
            ("inactive", "active") => true,
            ("unknown", _) => true,
            (_, "unknown") => true,
            (same_from, same_to) if same_from == same_to => true,
            // Invalid transitions
            ("filled", "empty") => false, // Can't unfill without explicit action
            ("active", "inactive") => false, // Can't deactivate without explicit action
            _ => true, // Allow other transitions by default
        }
    }

    /// Detect memory safety violations
    fn detect_memory_safety_violations(&self, ast: &AstNode) -> Vec<Finding> {
        let mut findings = Vec::new();
        
        // Track potential memory issues
        let mut allocated_resources = HashSet::new();
        let mut freed_resources = HashSet::new();
        
        match ast {
            AstNode::Program(statements) => {
                for (i, statement) in statements.iter().enumerate() {
                    findings.extend(self.check_memory_safety_statement(
                        statement, 
                        &mut allocated_resources, 
                        &mut freed_resources, 
                        i + 1
                    ));
                }
            }
        }

        // Check for resource leaks
        for resource in &allocated_resources {
            if !freed_resources.contains(resource) {
                findings.push(Finding {
                    category: AnalysisCategory::Correctness,
                    severity: Severity::Warning,
                    message: format!("Potential resource leak: '{}'", resource),
                    suggestion: Some("Ensure all allocated resources are properly freed".to_string()),
                    location: (1, 1),
                    span_length: 0,
                    rule_id: "resource_leak".to_string(),
                });
            }
        }

        findings
    }

    /// Check memory safety for individual statements
    fn check_memory_safety_statement(
        &self, 
        statement: &Statement, 
        allocated: &mut HashSet<String>, 
        freed: &mut HashSet<String>, 
        line: usize
    ) -> Vec<Finding> {
        let mut findings = Vec::new();

        match statement {
            Statement::Assignment { identifier, value, .. } => {
                // Check for potential allocation patterns
                if let Expression::Call { function, .. } = value {
                    if function.contains("alloc") || function.contains("new") || function.contains("create") {
                        allocated.insert(identifier.clone());
                    } else if function.contains("free") || function.contains("delete") || function.contains("drop") {
                        if !allocated.contains(identifier) {
                            findings.push(Finding {
                                category: AnalysisCategory::Correctness,
                                severity: Severity::Error,
                                message: format!("Attempting to free unallocated resource '{}'", identifier),
                                suggestion: Some("Only free resources that have been allocated".to_string()),
                                location: (line, 1),
                                span_length: identifier.len(),
                                rule_id: "free_unallocated".to_string(),
                            });
                        } else if freed.contains(identifier) {
                            findings.push(Finding {
                                category: AnalysisCategory::Correctness,
                                severity: Severity::Error,
                                message: format!("Double free of resource '{}'", identifier),
                                suggestion: Some("Resources should only be freed once".to_string()),
                                location: (line, 1),
                                span_length: identifier.len(),
                                rule_id: "double_free".to_string(),
                            });
                        } else {
                            freed.insert(identifier.clone());
                        }
                    }
                }
            }
            Statement::Function { body, .. } => {
                // Functions have their own memory scope
                let mut func_allocated = allocated.clone();
                let mut func_freed = freed.clone();
                
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_memory_safety_statement(
                        stmt, 
                        &mut func_allocated, 
                        &mut func_freed, 
                        line + i + 1
                    ));
                }
            }
            _ => {}
        }

        findings
    }
}

impl Analyzer for CorrectnessAnalyzer {
    fn analyze(&self, _source: &str, ast: &oviec::ast::AstNode) -> AprokoResult<Vec<Finding>> {
        let mut findings = Vec::new();
        
        // Enforce ownership rules
        findings.extend(self.enforce_ownership_rules(ast));
        
        // Check state transitions
        findings.extend(self.check_state_transitions(ast));
        
        // Detect memory safety violations
        findings.extend(self.detect_memory_safety_violations(ast));
        
        Ok(findings)
    }

    fn category(&self) -> AnalysisCategory {
        AnalysisCategory::Correctness
    }

    fn configure(&mut self, config: &CategoryConfig) -> AprokoResult<()> {
        if let Some(enforce_ownership) = config.settings.get("enforce_ownership") {
            self.enforce_ownership = enforce_ownership == "true";
        }
        
        if let Some(check_state_transitions) = config.settings.get("check_state_transitions") {
            self.check_state_transitions = check_state_transitions == "true";
        }
        
        Ok(())
    }
}

impl Default for CorrectnessAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}