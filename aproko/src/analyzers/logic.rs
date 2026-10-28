//! Logic analysis for control flow and logical correctness

use crate::{Analyzer, AnalysisCategory, AprokoResult, Finding, Severity, CategoryConfig};
use oviec::ast::{AstNode, Statement, Expression, Literal};
use std::collections::HashSet;
use regex::Regex;

/// Analyzer for logic and control flow validation
pub struct LogicAnalyzer {
    check_unreachable: bool,
    validate_control_flow: bool,
}

impl LogicAnalyzer {
    pub fn new() -> Self {
        Self {
            check_unreachable: true,
            validate_control_flow: true,
        }
    }

    /// Check for unreachable code patterns
    fn check_unreachable_code(&self, ast: &AstNode) -> Vec<Finding> {
        let mut findings = Vec::new();
        
        if !self.check_unreachable {
            return findings;
        }

        match ast {
            AstNode::Program(statements) => {
                for (i, statement) in statements.iter().enumerate() {
                    findings.extend(self.check_statement_reachability(statement, i + 1));
                }
            }
        }

        findings
    }

    /// Check reachability of individual statements
    fn check_statement_reachability(&self, statement: &Statement, line: usize) -> Vec<Finding> {
        let mut findings = Vec::new();

        match statement {
            Statement::Function { body, .. } => {
                findings.extend(self.check_function_reachability(body, line));
            }
            Statement::If { condition, then_block, else_block } => {
                // Check for always true/false conditions
                if let Some(always_result) = self.evaluate_condition_statically(condition) {
                    if always_result {
                        if let Some(else_stmts) = else_block {
                            if !else_stmts.is_empty() {
                                findings.push(Finding {
                                    category: AnalysisCategory::Logic,
                                    severity: Severity::Warning,
                                    message: "Else block is unreachable due to always-true condition".to_string(),
                                    suggestion: Some("Remove the else block or fix the condition".to_string()),
                                    location: (line, 1),
                                    span_length: 0,
                                    rule_id: "unreachable_else".to_string(),
                                });
                            }
                        }
                    } else {
                        if !then_block.is_empty() {
                            findings.push(Finding {
                                category: AnalysisCategory::Logic,
                                severity: Severity::Warning,
                                message: "If block is unreachable due to always-false condition".to_string(),
                                suggestion: Some("Remove the if block or fix the condition".to_string()),
                                location: (line, 1),
                                span_length: 0,
                                rule_id: "unreachable_if".to_string(),
                            });
                        }
                    }
                }

                // Recursively check blocks
                for (i, stmt) in then_block.iter().enumerate() {
                    findings.extend(self.check_statement_reachability(stmt, line + i + 1));
                }
                
                if let Some(else_stmts) = else_block {
                    for (i, stmt) in else_stmts.iter().enumerate() {
                        findings.extend(self.check_statement_reachability(stmt, line + i + 1));
                    }
                }
            }
            Statement::While { condition, body } => {
                // Check for infinite loops
                if let Some(always_result) = self.evaluate_condition_statically(condition) {
                    if always_result {
                        findings.push(Finding {
                            category: AnalysisCategory::Logic,
                            severity: Severity::Warning,
                            message: "Potential infinite loop detected".to_string(),
                            suggestion: Some("Ensure the loop condition can become false".to_string()),
                            location: (line, 1),
                            span_length: 0,
                            rule_id: "infinite_loop".to_string(),
                        });
                    } else if !body.is_empty() {
                        findings.push(Finding {
                            category: AnalysisCategory::Logic,
                            severity: Severity::Warning,
                            message: "While loop body is unreachable due to always-false condition".to_string(),
                            suggestion: Some("Fix the condition or remove the loop".to_string()),
                            location: (line, 1),
                            span_length: 0,
                            rule_id: "unreachable_while_body".to_string(),
                        });
                    }
                }

                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_reachability(stmt, line + i + 1));
                }
            }
            Statement::For { body, .. } => {
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_reachability(stmt, line + i + 1));
                }
            }
            _ => {}
        }

        findings
    }

    /// Check function for unreachable code after return statements
    fn check_function_reachability(&self, body: &[Statement], start_line: usize) -> Vec<Finding> {
        let mut findings = Vec::new();
        let mut found_return = false;

        for (i, statement) in body.iter().enumerate() {
            let line = start_line + i;
            
            if found_return {
                findings.push(Finding {
                    category: AnalysisCategory::Logic,
                    severity: Severity::Warning,
                    message: "Unreachable code after return statement".to_string(),
                    suggestion: Some("Remove unreachable code or restructure function".to_string()),
                    location: (line, 1),
                    span_length: 0,
                    rule_id: "unreachable_after_return".to_string(),
                });
                break;
            }

            if matches!(statement, Statement::Return { .. }) {
                found_return = true;
            }

            findings.extend(self.check_statement_reachability(statement, line));
        }

        findings
    }

    /// Validate control flow patterns
    fn validate_control_flow(&self, ast: &AstNode) -> Vec<Finding> {
        let mut findings = Vec::new();
        
        if !self.validate_control_flow {
            return findings;
        }

        // Check for variable usage before declaration
        let mut declared_vars = HashSet::new();
        
        match ast {
            AstNode::Program(statements) => {
                for (i, statement) in statements.iter().enumerate() {
                    findings.extend(self.check_variable_usage(statement, &mut declared_vars, i + 1));
                }
            }
        }

        findings
    }

    /// Check variable usage patterns
    fn check_variable_usage(&self, statement: &Statement, declared_vars: &mut HashSet<String>, line: usize) -> Vec<Finding> {
        let mut findings = Vec::new();

        match statement {
            Statement::Assignment { identifier, value, .. } => {
                // Check if value uses undeclared variables
                findings.extend(self.check_expression_variables(value, declared_vars, line));
                
                // Declare this variable
                declared_vars.insert(identifier.clone());
            }
            Statement::Print { expression } => {
                findings.extend(self.check_expression_variables(expression, declared_vars, line));
            }
            Statement::Function { parameters, body, .. } => {
                // Function parameters are declared in function scope
                let mut function_vars = declared_vars.clone();
                for param in parameters {
                    function_vars.insert(param.name.clone());
                }
                
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_variable_usage(stmt, &mut function_vars, line + i + 1));
                }
            }
            Statement::If { condition, then_block, else_block } => {
                findings.extend(self.check_expression_variables(condition, declared_vars, line));
                
                // Check blocks with current variable scope
                let mut then_vars = declared_vars.clone();
                for (i, stmt) in then_block.iter().enumerate() {
                    findings.extend(self.check_variable_usage(stmt, &mut then_vars, line + i + 1));
                }
                
                if let Some(else_stmts) = else_block {
                    let mut else_vars = declared_vars.clone();
                    for (i, stmt) in else_stmts.iter().enumerate() {
                        findings.extend(self.check_variable_usage(stmt, &mut else_vars, line + i + 1));
                    }
                }
            }
            Statement::While { condition, body } => {
                findings.extend(self.check_expression_variables(condition, declared_vars, line));
                
                let mut loop_vars = declared_vars.clone();
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_variable_usage(stmt, &mut loop_vars, line + i + 1));
                }
            }
            Statement::For { identifier, iterable, body } => {
                findings.extend(self.check_expression_variables(iterable, declared_vars, line));
                
                // Iterator variable is declared in loop scope
                let mut loop_vars = declared_vars.clone();
                loop_vars.insert(identifier.clone());
                
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_variable_usage(stmt, &mut loop_vars, line + i + 1));
                }
            }
            Statement::Return { value } => {
                if let Some(expr) = value {
                    findings.extend(self.check_expression_variables(expr, declared_vars, line));
                }
            }
            Statement::Expression { expression } => {
                findings.extend(self.check_expression_variables(expression, declared_vars, line));
            }
            _ => {}
        }

        findings
    }

    /// Check if expression uses undeclared variables
    fn check_expression_variables(&self, expression: &Expression, declared_vars: &HashSet<String>, line: usize) -> Vec<Finding> {
        let mut findings = Vec::new();

        match expression {
            Expression::Identifier(name) => {
                if !declared_vars.contains(name) && !self.is_builtin_identifier(name) {
                    findings.push(Finding {
                        category: AnalysisCategory::Logic,
                        severity: Severity::Error,
                        message: format!("Variable '{}' used before declaration", name),
                        suggestion: Some("Declare the variable before using it".to_string()),
                        location: (line, 1),
                        span_length: name.len(),
                        rule_id: "undeclared_variable".to_string(),
                    });
                }
            }
            Expression::Binary { left, right, .. } => {
                findings.extend(self.check_expression_variables(left, declared_vars, line));
                findings.extend(self.check_expression_variables(right, declared_vars, line));
            }
            Expression::Unary { operand, .. } => {
                findings.extend(self.check_expression_variables(operand, declared_vars, line));
            }
            Expression::Call { arguments, .. } => {
                for arg in arguments {
                    findings.extend(self.check_expression_variables(arg, declared_vars, line));
                }
            }
            Expression::FieldAccess { object, .. } => {
                findings.extend(self.check_expression_variables(object, declared_vars, line));
            }
            Expression::StructInstantiation { fields, .. } => {
                for field in fields {
                    findings.extend(self.check_expression_variables(&field.value, declared_vars, line));
                }
            }
            Expression::Range { start, end } => {
                findings.extend(self.check_expression_variables(start, declared_vars, line));
                findings.extend(self.check_expression_variables(end, declared_vars, line));
            }
            Expression::EnumVariantConstruction { data, .. } => {
                if let Some(data_expr) = data {
                    findings.extend(self.check_expression_variables(data_expr, declared_vars, line));
                }
            }
            Expression::Index { object, index } => {
                findings.extend(self.check_expression_variables(object, declared_vars, line));
                findings.extend(self.check_expression_variables(index, declared_vars, line));
            }
            Expression::ArrayLiteral { elements } => {
                for element in elements {
                    findings.extend(self.check_expression_variables(element, declared_vars, line));
                }
            }
            Expression::Match { value, arms } => {
                findings.extend(self.check_expression_variables(value, declared_vars, line));
                for arm in arms {
                    if let Some(guard) = &arm.guard {
                        findings.extend(self.check_expression_variables(guard, declared_vars, line));
                    }
                    // Match arms can introduce new bindings, but we'll keep it simple for now
                    // Create a mutable clone for the match arm scope
                    let mut arm_vars = declared_vars.clone();
                    for stmt in &arm.body {
                        findings.extend(self.check_variable_usage(stmt, &mut arm_vars, line));
                    }
                }
            }
            Expression::MethodCall { object, arguments, .. } => {
                findings.extend(self.check_expression_variables(object, declared_vars, line));
                for arg in arguments {
                    findings.extend(self.check_expression_variables(arg, declared_vars, line));
                }
            }
            Expression::Try { expression } => {
                findings.extend(self.check_expression_variables(expression, declared_vars, line));
            }
            Expression::Null => {
                // Null literal doesn't reference variables
            }
            Expression::Literal(_) => {
                // Literals don't reference variables
            }
        }

        findings
    }

    /// Check if identifier is a builtin (like function names)
    fn is_builtin_identifier(&self, name: &str) -> bool {
        // Built-in functions and constants
        matches!(name, "seeAm" | "true" | "false")
    }


    /// Check for common best-practice patterns and recommend improvements
    fn check_best_practices(&self, source: &str) -> Vec<Finding> {
        let mut findings = Vec::new();
        for (i, line) in source.lines().enumerate() {
            let ln = i + 1;
            let trimmed = line.trim();

            // Recommend oya over bare assignment for fixed values
            // e.g. `name = "Ovie"` at top level without mut or oya
            if !trimmed.starts_with("//") && !trimmed.starts_with("fn ")
                && !trimmed.starts_with("mut ") && !trimmed.starts_with("oya ")
                && !trimmed.starts_with("const ") && !trimmed.starts_with("let ")
                && !trimmed.starts_with("return ") && !trimmed.starts_with("seeAm ")
                && !trimmed.starts_with("print(") && !trimmed.starts_with("if ")
                && !trimmed.starts_with("while ") && !trimmed.starts_with("for ")
                && !trimmed.starts_with("struct ") && !trimmed.starts_with("enum ")
                && !trimmed.starts_with("use ") && !trimmed.starts_with("import ")
                && !trimmed.starts_with("export ") && !trimmed.starts_with("}")
                && !trimmed.starts_with("{") && !trimmed.is_empty()
                && (trimmed.contains(" = \"") || trimmed.contains(" = true") || trimmed.contains(" = false"))
                && !trimmed.contains("==") && !trimmed.contains("+=")
                && !trimmed.contains("-=") && !trimmed.contains("*=")
            {
                findings.push(Finding {
                    category: AnalysisCategory::Logic,
                    severity: Severity::Warning,
                    message: format!("Line {ln}: bare assignment looks like a constant — consider using `oya`"),
                    suggestion: Some(format!("Change to: oya {}", trimmed)),
                    location: (ln, 1),
                    span_length: trimmed.len(),
                    rule_id: "prefer_oya_for_constants".to_string(),
                });
            }

            // Detect `while true` without a break — infinite loop warning
            if trimmed == "while true {" || trimmed == "while true{" {
                findings.push(Finding {
                    category: AnalysisCategory::Logic,
                    severity: Severity::Warning,
                    message: format!("Line {ln}: `while true` detected — make sure there is a `break` or return inside"),
                    suggestion: Some("Add a `break` condition or use a `mut running = true` pattern".to_string()),
                    location: (ln, 1),
                    span_length: trimmed.len(),
                    rule_id: "while_true_loop".to_string(),
                });
            }

            // Detect division without zero check
            if trimmed.contains("/ ") && !trimmed.starts_with("//") {
                if source[..source.lines().take(ln.saturating_sub(5)).map(|l| l.len()+1).sum::<usize>()].contains("== 0") {
                    // zero check present nearby, skip
                } else if trimmed.contains("/ 0") {
                    findings.push(Finding {
                        category: AnalysisCategory::Logic,
                        severity: Severity::Error,
                        message: format!("Line {ln}: division by literal zero"),
                        suggestion: Some("Remove or replace with a safe value".to_string()),
                        location: (ln, 1),
                        span_length: trimmed.len(),
                        rule_id: "division_by_zero".to_string(),
                    });
                }
            }

            // Recommend extracting magic numbers into oya constants
            let magic_re = Regex::new(r"\b(\d{3,})\b").unwrap();
            for cap in magic_re.captures_iter(trimmed) {
                let n = &cap[1];
                if !trimmed.starts_with("oya ") && !trimmed.starts_with("const ") {
                    findings.push(Finding {
                        category: AnalysisCategory::Logic,
                        severity: Severity::Info,
                        message: format!("Line {ln}: magic number {n} — consider naming it with `oya LIMIT = {n}`"),
                        suggestion: Some(format!("oya YOUR_CONSTANT = {n}")),
                        location: (ln, 1),
                        span_length: n.len(),
                        rule_id: "magic_number".to_string(),
                    });
                }
            }
        }
        findings
    }

    /// Try to statically evaluate a condition to detect always true/false
    fn evaluate_condition_statically(&self, condition: &Expression) -> Option<bool> {
        match condition {
            Expression::Literal(Literal::Boolean(b)) => Some(*b),
            Expression::Literal(Literal::Number(n)) => Some(*n != 0.0),
            Expression::Literal(Literal::String(s)) => Some(!s.is_empty()),
            _ => None, // Can't statically evaluate complex expressions
        }
    }
}

impl Analyzer for LogicAnalyzer {
    fn analyze(&self, _source: &str, ast: &oviec::ast::AstNode) -> AprokoResult<Vec<Finding>> {
        let mut findings = Vec::new();
        
        // Check for unreachable code
        findings.extend(self.check_unreachable_code(ast));
        
        // Validate control flow
        findings.extend(self.validate_control_flow(ast));

        // Best-practice recommendations
        findings.extend(self.check_best_practices(_source));
        
        Ok(findings)
    }

    fn category(&self) -> AnalysisCategory {
        AnalysisCategory::Logic
    }

    fn configure(&mut self, config: &CategoryConfig) -> AprokoResult<()> {
        if let Some(check_unreachable) = config.settings.get("check_unreachable") {
            self.check_unreachable = check_unreachable == "true";
        }
        
        if let Some(validate_control_flow) = config.settings.get("validate_control_flow") {
            self.validate_control_flow = validate_control_flow == "true";
        }
        
        Ok(())
    }
}

impl Default for LogicAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}