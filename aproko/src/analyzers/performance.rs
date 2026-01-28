//! Performance analysis for algorithmic complexity and optimization opportunities

use crate::{Analyzer, AnalysisCategory, AprokoResult, Finding, Severity, CategoryConfig};
use oviec::ast::{AstNode, Statement, Expression};

/// Analyzer for performance and algorithmic complexity
pub struct PerformanceAnalyzer {
    check_complexity: bool,
    max_complexity: usize,
}

impl PerformanceAnalyzer {
    pub fn new() -> Self {
        Self {
            check_complexity: true,
            max_complexity: 10,
        }
    }

    /// Check for algorithmic complexity issues
    fn check_complexity(&self, ast: &AstNode) -> Vec<Finding> {
        let mut findings = Vec::new();
        
        if !self.check_complexity {
            return findings;
        }

        for (i, statement) in ast.statements.iter().enumerate() {
            findings.extend(self.check_statement_complexity(statement, i + 1));
        }

        findings
    }

    /// Check complexity of individual statements
    fn check_statement_complexity(&self, statement: &Statement, line: usize) -> Vec<Finding> {
        let mut findings = Vec::new();

        match statement {
            Statement::Function { body, .. } => {
                let complexity = self.calculate_cyclomatic_complexity(body);
                if complexity > self.max_complexity {
                    findings.push(Finding {
                        category: AnalysisCategory::Performance,
                        severity: Severity::Warning,
                        message: format!("High cyclomatic complexity: {} (max: {})", complexity, self.max_complexity),
                        suggestion: Some("Consider breaking this function into smaller functions".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "high_complexity".to_string(),
                    });
                }

                // Check for nested loops
                findings.extend(self.check_nested_loops(body, line, 0));
                
                // Recursively check body statements
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_complexity(stmt, line + i + 1));
                }
            }
            Statement::While { condition, body } => {
                // Check for potential infinite loops with complex conditions
                if self.is_complex_condition(condition) {
                    findings.push(Finding {
                        category: AnalysisCategory::Performance,
                        severity: Severity::Info,
                        message: "Complex condition in while loop may impact performance".to_string(),
                        suggestion: Some("Consider simplifying the condition or caching the result".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "complex_loop_condition".to_string(),
                    });
                }

                findings.extend(self.check_nested_loops(body, line, 1));
                
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_complexity(stmt, line + i + 1));
                }
            }
            Statement::For { body, .. } => {
                findings.extend(self.check_nested_loops(body, line, 1));
                
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_complexity(stmt, line + i + 1));
                }
            }
            Statement::If { condition, then_block, else_block } => {
                if self.is_complex_condition(condition) {
                    findings.push(Finding {
                        category: AnalysisCategory::Performance,
                        severity: Severity::Info,
                        message: "Complex condition in if statement".to_string(),
                        suggestion: Some("Consider simplifying the condition or using early returns".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "complex_if_condition".to_string(),
                    });
                }

                for (i, stmt) in then_block.iter().enumerate() {
                    findings.extend(self.check_statement_complexity(stmt, line + i + 1));
                }
                
                if let Some(else_stmts) = else_block {
                    for (i, stmt) in else_stmts.iter().enumerate() {
                        findings.extend(self.check_statement_complexity(stmt, line + i + 1));
                    }
                }
            }
            _ => {}
        }

        findings
    }

    /// Calculate cyclomatic complexity of a block of statements
    fn calculate_cyclomatic_complexity(&self, statements: &[Statement]) -> usize {
        let mut complexity = 1; // Base complexity

        for statement in statements {
            complexity += self.statement_complexity_contribution(statement);
        }

        complexity
    }

    /// Calculate complexity contribution of a single statement
    fn statement_complexity_contribution(&self, statement: &Statement) -> usize {
        match statement {
            Statement::If { then_block, else_block, .. } => {
                let mut contribution = 1; // +1 for the if
                contribution += self.calculate_cyclomatic_complexity(then_block);
                if let Some(else_stmts) = else_block {
                    contribution += self.calculate_cyclomatic_complexity(else_stmts);
                }
                contribution
            }
            Statement::While { body, .. } => {
                1 + self.calculate_cyclomatic_complexity(body) // +1 for the while
            }
            Statement::For { body, .. } => {
                1 + self.calculate_cyclomatic_complexity(body) // +1 for the for
            }
            Statement::Function { body, .. } => {
                self.calculate_cyclomatic_complexity(body)
            }
            _ => 0, // Other statements don't add complexity
        }
    }

    /// Check for nested loops that could cause performance issues
    fn check_nested_loops(&self, statements: &[Statement], start_line: usize, nesting_level: usize) -> Vec<Finding> {
        let mut findings = Vec::new();

        for (i, statement) in statements.iter().enumerate() {
            let line = start_line + i;
            
            match statement {
                Statement::While { body, .. } | Statement::For { body, .. } => {
                    let new_nesting = nesting_level + 1;
                    
                    if new_nesting >= 3 {
                        findings.push(Finding {
                            category: AnalysisCategory::Performance,
                            severity: Severity::Warning,
                            message: format!("Deeply nested loop (level {})", new_nesting),
                            suggestion: Some("Consider extracting inner loops into separate functions".to_string()),
                            location: (line, 1),
                            span_length: 0,
                            rule_id: "deep_nesting".to_string(),
                        });
                    } else if new_nesting == 2 {
                        findings.push(Finding {
                            category: AnalysisCategory::Performance,
                            severity: Severity::Info,
                            message: "Nested loop detected - consider algorithmic optimization".to_string(),
                            suggestion: Some("Review if the nested loop can be optimized or avoided".to_string()),
                            location: (line, 1),
                            span_length: 0,
                            rule_id: "nested_loop".to_string(),
                        });
                    }

                    findings.extend(self.check_nested_loops(body, line + 1, new_nesting));
                }
                Statement::If { then_block, else_block, .. } => {
                    findings.extend(self.check_nested_loops(then_block, line + 1, nesting_level));
                    if let Some(else_stmts) = else_block {
                        findings.extend(self.check_nested_loops(else_stmts, line + 1, nesting_level));
                    }
                }
                Statement::Function { body, .. } => {
                    // Functions reset nesting level
                    findings.extend(self.check_nested_loops(body, line + 1, 0));
                }
                _ => {}
            }
        }

        findings
    }

    /// Check if a condition is complex and might impact performance
    fn is_complex_condition(&self, condition: &Expression) -> bool {
        self.count_expression_operations(condition) > 3
    }

    /// Count the number of operations in an expression
    fn count_expression_operations(&self, expression: &Expression) -> usize {
        match expression {
            Expression::Binary { left, right, .. } => {
                1 + self.count_expression_operations(left) + self.count_expression_operations(right)
            }
            Expression::Unary { operand, .. } => {
                1 + self.count_expression_operations(operand)
            }
            Expression::Call { arguments, .. } => {
                1 + arguments.iter().map(|arg| self.count_expression_operations(arg)).sum::<usize>()
            }
            Expression::FieldAccess { object, .. } => {
                1 + self.count_expression_operations(object)
            }
            Expression::StructInstantiation { fields, .. } => {
                fields.iter().map(|field| self.count_expression_operations(&field.value)).sum()
            }
            Expression::Range { start, end } => {
                self.count_expression_operations(start) + self.count_expression_operations(end)
            }
            Expression::Identifier(_) | Expression::Literal(_) => 0,
        }
    }

    /// Check for optimization opportunities
    fn check_optimization_opportunities(&self, ast: &AstNode) -> Vec<Finding> {
        let mut findings = Vec::new();

        for (i, statement) in ast.statements.iter().enumerate() {
            findings.extend(self.check_statement_optimizations(statement, i + 1));
        }

        findings
    }

    /// Check optimization opportunities in statements
    fn check_statement_optimizations(&self, statement: &Statement, line: usize) -> Vec<Finding> {
        let mut findings = Vec::new();

        match statement {
            Statement::Assignment { value, .. } => {
                // Check for expensive operations in assignments
                if self.is_expensive_expression(value) {
                    findings.push(Finding {
                        category: AnalysisCategory::Performance,
                        severity: Severity::Info,
                        message: "Potentially expensive operation in assignment".to_string(),
                        suggestion: Some("Consider caching the result if used multiple times".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "expensive_assignment".to_string(),
                    });
                }
            }
            Statement::While { condition, body } => {
                // Check for invariant computations in loop conditions
                if self.has_loop_invariant_computation(condition) {
                    findings.push(Finding {
                        category: AnalysisCategory::Performance,
                        severity: Severity::Info,
                        message: "Loop condition contains computation that could be moved outside".to_string(),
                        suggestion: Some("Consider computing invariant values before the loop".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "loop_invariant".to_string(),
                    });
                }

                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_optimizations(stmt, line + i + 1));
                }
            }
            Statement::For { body, .. } => {
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_optimizations(stmt, line + i + 1));
                }
            }
            Statement::Function { body, .. } => {
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_optimizations(stmt, line + i + 1));
                }
            }
            Statement::If { then_block, else_block, .. } => {
                for (i, stmt) in then_block.iter().enumerate() {
                    findings.extend(self.check_statement_optimizations(stmt, line + i + 1));
                }
                
                if let Some(else_stmts) = else_block {
                    for (i, stmt) in else_stmts.iter().enumerate() {
                        findings.extend(self.check_statement_optimizations(stmt, line + i + 1));
                    }
                }
            }
            _ => {}
        }

        findings
    }

    /// Check if an expression is potentially expensive
    fn is_expensive_expression(&self, expression: &Expression) -> bool {
        match expression {
            Expression::Call { .. } => true, // Function calls can be expensive
            Expression::Binary { left, right, .. } => {
                self.is_expensive_expression(left) || self.is_expensive_expression(right)
            }
            Expression::Unary { operand, .. } => self.is_expensive_expression(operand),
            _ => false,
        }
    }

    /// Check if expression contains loop-invariant computations
    fn has_loop_invariant_computation(&self, expression: &Expression) -> bool {
        match expression {
            Expression::Call { .. } => true, // Function calls might be invariant
            Expression::Binary { left, right, .. } => {
                self.has_loop_invariant_computation(left) || self.has_loop_invariant_computation(right)
            }
            Expression::Unary { operand, .. } => self.has_loop_invariant_computation(operand),
            _ => false,
        }
    }
}

impl Analyzer for PerformanceAnalyzer {
    fn analyze(&self, _source: &str, ast: &oviec::ast::AstNode) -> AprokoResult<Vec<Finding>> {
        let mut findings = Vec::new();
        
        // Check complexity
        findings.extend(self.check_complexity(ast));
        
        // Check optimization opportunities
        findings.extend(self.check_optimization_opportunities(ast));
        
        Ok(findings)
    }

    fn category(&self) -> AnalysisCategory {
        AnalysisCategory::Performance
    }

    fn configure(&mut self, config: &CategoryConfig) -> AprokoResult<()> {
        if let Some(check_complexity) = config.settings.get("check_complexity") {
            self.check_complexity = check_complexity == "true";
        }
        
        if let Some(max_complexity) = config.settings.get("max_complexity") {
            if let Ok(complexity) = max_complexity.parse::<usize>() {
                self.max_complexity = complexity;
            }
        }
        
        Ok(())
    }
}

impl Default for PerformanceAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}