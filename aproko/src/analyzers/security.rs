//! Security analysis for vulnerabilities and unsafe operations

use crate::{Analyzer, AnalysisCategory, AprokoResult, Finding, Severity, CategoryConfig};
use oviec::ast::{AstNode, Statement, Expression, Literal};

/// Analyzer for security vulnerabilities and unsafe operations
pub struct SecurityAnalyzer {
    check_unsafe: bool,
    validate_memory_safety: bool,
}

impl SecurityAnalyzer {
    pub fn new() -> Self {
        Self {
            check_unsafe: true,
            validate_memory_safety: true,
        }
    }

    /// Check for unsafe operations
    fn check_unsafe_operations(&self, ast: &AstNode) -> Vec<Finding> {
        let mut findings = Vec::new();
        
        if !self.check_unsafe {
            return findings;
        }

        match ast {
            AstNode::Program(statements) => {
                for (i, statement) in statements.iter().enumerate() {
                    findings.extend(self.check_statement_safety(statement, i + 1));
                }
            }
        }

        findings
    }

    /// Check safety of individual statements
    fn check_statement_safety(&self, statement: &Statement, line: usize) -> Vec<Finding> {
        let mut findings = Vec::new();

        match statement {
            Statement::Assignment { identifier, value, .. } => {
                // Check for potentially unsafe assignments
                findings.extend(self.check_expression_safety(value, line));
                
                // Check for sensitive variable names
                if self.is_sensitive_variable_name(identifier) {
                    findings.push(Finding {
                        category: AnalysisCategory::Security,
                        severity: Severity::Warning,
                        message: format!("Potentially sensitive variable name: '{}'", identifier),
                        suggestion: Some("Consider using a more generic name for sensitive data".to_string()),
                        location: (line, 1),
                        span_length: identifier.len(),
                        rule_id: "sensitive_variable_name".to_string(),
                    });
                }
            }
            Statement::Print { expression } => {
                // Check for potential information disclosure
                if self.might_disclose_sensitive_info(expression) {
                    findings.push(Finding {
                        category: AnalysisCategory::Security,
                        severity: Severity::Warning,
                        message: "Potential information disclosure in print statement".to_string(),
                        suggestion: Some("Ensure no sensitive information is being printed".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "information_disclosure".to_string(),
                    });
                }
                
                findings.extend(self.check_expression_safety(expression, line));
            }
            Statement::Function { name, parameters, body } => {
                // Check for unsafe function patterns
                if name.contains("unsafe") {
                    findings.push(Finding {
                        category: AnalysisCategory::Security,
                        severity: Severity::Critical,
                        message: "Function name suggests unsafe operations".to_string(),
                        suggestion: Some("Ensure proper safety checks and documentation".to_string()),
                        location: (line, 1),
                        span_length: name.len(),
                        rule_id: "unsafe_function_name".to_string(),
                    });
                }

                // Check for functions with too many parameters (potential for confusion)
                if parameters.len() > 5 {
                    findings.push(Finding {
                        category: AnalysisCategory::Security,
                        severity: Severity::Info,
                        message: "Function has many parameters, which may lead to confusion".to_string(),
                        suggestion: Some("Consider using a struct to group related parameters".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "many_parameters".to_string(),
                    });
                }

                // Recursively check body
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_safety(stmt, line + i + 1));
                }
            }
            Statement::If { condition, then_block, else_block } => {
                findings.extend(self.check_expression_safety(condition, line));
                
                for (i, stmt) in then_block.iter().enumerate() {
                    findings.extend(self.check_statement_safety(stmt, line + i + 1));
                }
                
                if let Some(else_stmts) = else_block {
                    for (i, stmt) in else_stmts.iter().enumerate() {
                        findings.extend(self.check_statement_safety(stmt, line + i + 1));
                    }
                }
            }
            Statement::While { condition, body } => {
                findings.extend(self.check_expression_safety(condition, line));
                
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_safety(stmt, line + i + 1));
                }
            }
            Statement::For { identifier: _, iterable, body } => {
                findings.extend(self.check_expression_safety(iterable, line));
                
                // Check for potential iterator issues
                if self.is_potentially_unsafe_iterator(iterable) {
                    findings.push(Finding {
                        category: AnalysisCategory::Security,
                        severity: Severity::Warning,
                        message: "Potentially unsafe iterator in for loop".to_string(),
                        suggestion: Some("Validate iterator bounds and safety".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "unsafe_iterator".to_string(),
                    });
                }
                
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_safety(stmt, line + i + 1));
                }
            }
            Statement::Return { value } => {
                if let Some(expr) = value {
                    findings.extend(self.check_expression_safety(expr, line));
                }
            }
            Statement::Expression { expression } => {
                findings.extend(self.check_expression_safety(expression, line));
            }
            _ => {}
        }

        findings
    }

    /// Check safety of expressions
    fn check_expression_safety(&self, expression: &Expression, line: usize) -> Vec<Finding> {
        let mut findings = Vec::new();

        match expression {
            Expression::Identifier(name) => {
                if self.is_sensitive_variable_name(name) {
                    findings.push(Finding {
                        category: AnalysisCategory::Security,
                        severity: Severity::Info,
                        message: format!("Reference to potentially sensitive variable: '{}'", name),
                        suggestion: Some("Ensure proper handling of sensitive data".to_string()),
                        location: (line, 1),
                        span_length: name.len(),
                        rule_id: "sensitive_variable_reference".to_string(),
                    });
                }
            }
            Expression::Literal(Literal::String(s)) => {
                // Check for hardcoded sensitive information
                if self.contains_sensitive_pattern(s) {
                    findings.push(Finding {
                        category: AnalysisCategory::Security,
                        severity: Severity::Critical,
                        message: "Potential hardcoded sensitive information in string literal".to_string(),
                        suggestion: Some("Move sensitive data to configuration or environment variables".to_string()),
                        location: (line, 1),
                        span_length: s.len() + 2, // +2 for quotes
                        rule_id: "hardcoded_sensitive_data".to_string(),
                    });
                }
            }
            Expression::Binary { left, right, .. } => {
                findings.extend(self.check_expression_safety(left, line));
                findings.extend(self.check_expression_safety(right, line));
            }
            Expression::Unary { operand, .. } => {
                findings.extend(self.check_expression_safety(operand, line));
            }
            Expression::Call { function, arguments } => {
                // Check for potentially unsafe function calls
                if self.is_potentially_unsafe_function(function) {
                    findings.push(Finding {
                        category: AnalysisCategory::Security,
                        severity: Severity::Warning,
                        message: format!("Call to potentially unsafe function: '{}'", function),
                        suggestion: Some("Ensure proper input validation and error handling".to_string()),
                        location: (line, 1),
                        span_length: function.len(),
                        rule_id: "unsafe_function_call".to_string(),
                    });
                }

                for arg in arguments {
                    findings.extend(self.check_expression_safety(arg, line));
                }
            }
            Expression::FieldAccess { object, field } => {
                findings.extend(self.check_expression_safety(object, line));
                
                if self.is_sensitive_field_name(field) {
                    findings.push(Finding {
                        category: AnalysisCategory::Security,
                        severity: Severity::Info,
                        message: format!("Access to potentially sensitive field: '{}'", field),
                        suggestion: Some("Ensure proper access controls for sensitive fields".to_string()),
                        location: (line, 1),
                        span_length: field.len(),
                        rule_id: "sensitive_field_access".to_string(),
                    });
                }
            }
            Expression::StructInstantiation { struct_name, fields } => {
                if self.is_sensitive_struct_name(struct_name) {
                    findings.push(Finding {
                        category: AnalysisCategory::Security,
                        severity: Severity::Info,
                        message: format!("Instantiation of potentially sensitive struct: '{}'", struct_name),
                        suggestion: Some("Ensure proper initialization of sensitive data structures".to_string()),
                        location: (line, 1),
                        span_length: struct_name.len(),
                        rule_id: "sensitive_struct_instantiation".to_string(),
                    });
                }

                for field in fields {
                    findings.extend(self.check_expression_safety(&field.value, line));
                }
            }
            Expression::Range { start, end } => {
                findings.extend(self.check_expression_safety(start, line));
                findings.extend(self.check_expression_safety(end, line));
                
                // Check for potential integer overflow in ranges
                if self.might_cause_overflow(start, end) {
                    findings.push(Finding {
                        category: AnalysisCategory::Security,
                        severity: Severity::Warning,
                        message: "Range expression might cause integer overflow".to_string(),
                        suggestion: Some("Validate range bounds to prevent overflow".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "range_overflow".to_string(),
                    });
                }
            }
            Expression::EnumVariantConstruction { data, .. } => {
                if let Some(data_expr) = data {
                    findings.extend(self.check_expression_safety(data_expr, line));
                }
            }
            Expression::Index { object, index } => {
                findings.extend(self.check_expression_safety(object, line));
                findings.extend(self.check_expression_safety(index, line));
            }
            Expression::ArrayLiteral { elements } => {
                for element in elements {
                    findings.extend(self.check_expression_safety(element, line));
                }
            }
            Expression::Literal(_) => {
                // Other literals are generally safe
            }
        }

        findings
    }

    /// Check if variable name suggests sensitive data
    fn is_sensitive_variable_name(&self, name: &str) -> bool {
        let name_lower = name.to_lowercase();
        name_lower.contains("password") ||
        name_lower.contains("secret") ||
        name_lower.contains("key") ||
        name_lower.contains("token") ||
        name_lower.contains("auth") ||
        name_lower.contains("credential") ||
        name_lower.contains("private")
    }

    /// Check if field name suggests sensitive data
    fn is_sensitive_field_name(&self, name: &str) -> bool {
        self.is_sensitive_variable_name(name)
    }

    /// Check if struct name suggests sensitive data
    fn is_sensitive_struct_name(&self, name: &str) -> bool {
        let name_lower = name.to_lowercase();
        name_lower.contains("credential") ||
        name_lower.contains("auth") ||
        name_lower.contains("secret") ||
        name_lower.contains("private")
    }

    /// Check if string contains sensitive patterns
    fn contains_sensitive_pattern(&self, s: &str) -> bool {
        let s_lower = s.to_lowercase();
        
        // Check for common sensitive patterns
        s_lower.contains("password=") ||
        s_lower.contains("secret=") ||
        s_lower.contains("key=") ||
        s_lower.contains("token=") ||
        s_lower.contains("api_key") ||
        s_lower.contains("private_key") ||
        // Check for potential SQL injection patterns
        s_lower.contains("select ") ||
        s_lower.contains("insert ") ||
        s_lower.contains("update ") ||
        s_lower.contains("delete ") ||
        s_lower.contains("drop ") ||
        // Check for potential command injection
        s_lower.contains("system(") ||
        s_lower.contains("exec(") ||
        s_lower.contains("eval(")
    }

    /// Check if function name suggests unsafe operations
    fn is_potentially_unsafe_function(&self, name: &str) -> bool {
        let name_lower = name.to_lowercase();
        name_lower.contains("unsafe") ||
        name_lower.contains("raw") ||
        name_lower.contains("unchecked") ||
        name_lower.contains("system") ||
        name_lower.contains("exec") ||
        name_lower.contains("eval")
    }

    /// Check if iterator might be unsafe
    fn is_potentially_unsafe_iterator(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Call { function, .. } => {
                self.is_potentially_unsafe_function(function)
            }
            Expression::Range { .. } => {
                // Ranges can potentially overflow
                true
            }
            _ => false,
        }
    }

    /// Check if expression might disclose sensitive information
    fn might_disclose_sensitive_info(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Identifier(name) => self.is_sensitive_variable_name(name),
            Expression::Literal(Literal::String(s)) => self.contains_sensitive_pattern(s),
            Expression::FieldAccess { field, .. } => self.is_sensitive_field_name(field),
            Expression::Binary { left, right, .. } => {
                self.might_disclose_sensitive_info(left) || self.might_disclose_sensitive_info(right)
            }
            Expression::Unary { operand, .. } => self.might_disclose_sensitive_info(operand),
            _ => false,
        }
    }

    /// Check if range might cause overflow
    fn might_cause_overflow(&self, start: &Expression, end: &Expression) -> bool {
        match (start, end) {
            (Expression::Literal(Literal::Number(s)), Expression::Literal(Literal::Number(e))) => {
                // Check for very large ranges
                (e - s).abs() > 1_000_000.0
            }
            _ => false, // Can't statically determine for complex expressions
        }
    }

    /// Validate memory safety patterns
    fn validate_memory_safety(&self, ast: &AstNode) -> Vec<Finding> {
        let mut findings = Vec::new();
        
        if !self.validate_memory_safety {
            return findings;
        }

        // For now, this is a placeholder for memory safety analysis
        // In a real implementation, this would check for:
        // - Use after free
        // - Double free
        // - Buffer overflows
        // - Null pointer dereferences
        // - etc.

        match ast {
            AstNode::Program(statements) => {
                for (i, statement) in statements.iter().enumerate() {
                    findings.extend(self.check_memory_safety_statement(statement, i + 1));
                }
            }
        }

        findings
    }

    /// Check memory safety of statements
    fn check_memory_safety_statement(&self, statement: &Statement, line: usize) -> Vec<Finding> {
        let mut findings = Vec::new();

        // This is a simplified memory safety check
        // In practice, this would be much more sophisticated
        match statement {
            Statement::Assignment { identifier, .. } => {
                if identifier.contains("ptr") || identifier.contains("ref") {
                    findings.push(Finding {
                        category: AnalysisCategory::Security,
                        severity: Severity::Info,
                        message: "Variable name suggests pointer/reference usage".to_string(),
                        suggestion: Some("Ensure proper memory management for pointer-like variables".to_string()),
                        location: (line, 1),
                        span_length: identifier.len(),
                        rule_id: "pointer_variable".to_string(),
                    });
                }
            }
            Statement::Function { body, .. } => {
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_memory_safety_statement(stmt, line + i + 1));
                }
            }
            _ => {}
        }

        findings
    }
}

impl Analyzer for SecurityAnalyzer {
    fn analyze(&self, _source: &str, ast: &oviec::ast::AstNode) -> AprokoResult<Vec<Finding>> {
        let mut findings = Vec::new();
        
        // Check for unsafe operations
        findings.extend(self.check_unsafe_operations(ast));
        
        // Validate memory safety
        findings.extend(self.validate_memory_safety(ast));
        
        Ok(findings)
    }

    fn category(&self) -> AnalysisCategory {
        AnalysisCategory::Security
    }

    fn configure(&mut self, config: &CategoryConfig) -> AprokoResult<()> {
        if let Some(check_unsafe) = config.settings.get("check_unsafe") {
            self.check_unsafe = check_unsafe == "true";
        }
        
        if let Some(validate_memory_safety) = config.settings.get("validate_memory_safety") {
            self.validate_memory_safety = validate_memory_safety == "true";
        }
        
        Ok(())
    }
}

impl Default for SecurityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}