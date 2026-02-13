//! Syntax analysis for grammar compliance and syntax correctness

use crate::{Analyzer, AnalysisCategory, AprokoResult, Finding, Severity, CategoryConfig};
use oviec::ast::{AstNode, Statement, Expression};

/// Analyzer for syntax and grammar compliance
pub struct SyntaxAnalyzer {
    check_grammar: bool,
    validate_keywords: bool,
}

impl SyntaxAnalyzer {
    pub fn new() -> Self {
        Self {
            check_grammar: true,
            validate_keywords: true,
        }
    }

    /// Check for grammar compliance issues
    fn check_grammar_compliance(&self, ast: &AstNode) -> Vec<Finding> {
        let mut findings = Vec::new();
        
        if !self.check_grammar {
            return findings;
        }

        match ast {
            AstNode::Program(statements) => {
                // Check for empty programs
                if statements.is_empty() {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Warning,
                        message: "Empty program detected".to_string(),
                        suggestion: Some("Consider adding at least one statement".to_string()),
                        location: (1, 1),
                        span_length: 0,
                        rule_id: "empty_program".to_string(),
                    });
                }

                // Check each statement for syntax issues
                for (i, statement) in statements.iter().enumerate() {
                    findings.extend(self.check_statement_syntax(statement, i + 1));
                }
            }
        }

        findings
    }

    /// Check syntax of individual statements
    fn check_statement_syntax(&self, statement: &Statement, line: usize) -> Vec<Finding> {
        let mut findings = Vec::new();

        match statement {
            Statement::Print { expression } => {
                findings.extend(self.check_expression_syntax(expression, line));
            }
            Statement::Assignment { identifier, value, mutable } => {
                // Check identifier naming
                if identifier.is_empty() {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Error,
                        message: "Empty identifier in assignment".to_string(),
                        suggestion: Some("Provide a valid identifier name".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "empty_identifier".to_string(),
                    });
                }

                // Check for reserved keywords used as identifiers
                if self.is_reserved_keyword(identifier) {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Error,
                        message: format!("Reserved keyword '{}' used as identifier", identifier),
                        suggestion: Some("Use a different identifier name".to_string()),
                        location: (line, 1),
                        span_length: identifier.len(),
                        rule_id: "reserved_keyword_identifier".to_string(),
                    });
                }

                // Check if mutable assignment uses proper syntax
                if *mutable && !identifier.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Warning,
                        message: "Mutable identifier contains special characters".to_string(),
                        suggestion: Some("Use only alphanumeric characters and underscores".to_string()),
                        location: (line, 1),
                        span_length: identifier.len(),
                        rule_id: "invalid_mutable_identifier".to_string(),
                    });
                }

                findings.extend(self.check_expression_syntax(value, line));
            }
            Statement::VariableDeclaration { identifier, value, mutable } => {
                // Check identifier naming
                if identifier.is_empty() {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Error,
                        message: "Empty identifier in variable declaration".to_string(),
                        suggestion: Some("Provide a valid identifier name".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "empty_identifier".to_string(),
                    });
                }

                // Check for reserved keywords used as identifiers
                if self.is_reserved_keyword(identifier) {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Error,
                        message: format!("Reserved keyword '{}' used as identifier", identifier),
                        suggestion: Some("Use a different identifier name".to_string()),
                        location: (line, 1),
                        span_length: identifier.len(),
                        rule_id: "reserved_keyword_identifier".to_string(),
                    });
                }

                // Check if mutable variable uses proper syntax
                if *mutable && !identifier.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Warning,
                        message: "Mutable variable identifier contains special characters".to_string(),
                        suggestion: Some("Use only alphanumeric characters and underscores".to_string()),
                        location: (line, 1),
                        span_length: identifier.len(),
                        rule_id: "invalid_mutable_identifier".to_string(),
                    });
                }

                findings.extend(self.check_expression_syntax(value, line));
            }
            Statement::Function { name, parameters, body } => {
                // Check function name
                if name.is_empty() {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Error,
                        message: "Empty function name".to_string(),
                        suggestion: Some("Provide a valid function name".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "empty_function_name".to_string(),
                    });
                }

                // Check parameter names
                for param in parameters {
                    if param.is_empty() {
                        findings.push(Finding {
                            category: AnalysisCategory::Syntax,
                            severity: Severity::Error,
                            message: "Empty parameter name".to_string(),
                            suggestion: Some("Provide valid parameter names".to_string()),
                            location: (line, 1),
                            span_length: 0,
                            rule_id: "empty_parameter_name".to_string(),
                        });
                    }
                }

                // Check function body
                if body.is_empty() {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Warning,
                        message: "Empty function body".to_string(),
                        suggestion: Some("Consider adding function implementation or return statement".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "empty_function_body".to_string(),
                    });
                }

                // Recursively check body statements
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_syntax(stmt, line + i + 1));
                }
            }
            Statement::FunctionDeclaration { name, parameters, body } => {
                // Check function name
                if name.is_empty() {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Error,
                        message: "Empty function name in declaration".to_string(),
                        suggestion: Some("Provide a valid function name".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "empty_function_name".to_string(),
                    });
                }

                // Check parameter names
                for param in parameters {
                    if param.is_empty() {
                        findings.push(Finding {
                            category: AnalysisCategory::Syntax,
                            severity: Severity::Error,
                            message: "Empty parameter name in function declaration".to_string(),
                            suggestion: Some("Provide valid parameter names".to_string()),
                            location: (line, 1),
                            span_length: 0,
                            rule_id: "empty_parameter_name".to_string(),
                        });
                    }
                }

                // Check function body
                if body.is_empty() {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Warning,
                        message: "Empty function body in declaration".to_string(),
                        suggestion: Some("Consider adding function implementation or return statement".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "empty_function_body".to_string(),
                    });
                }

                // Recursively check body statements
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_syntax(stmt, line + i + 1));
                }
            }
            Statement::If { condition, then_block, else_block } => {
                findings.extend(self.check_expression_syntax(condition, line));
                
                // Check then block
                if then_block.is_empty() {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Warning,
                        message: "Empty if block".to_string(),
                        suggestion: Some("Add statements to the if block".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "empty_if_block".to_string(),
                    });
                }

                // Check else block if present
                if let Some(else_stmts) = else_block {
                    if else_stmts.is_empty() {
                        findings.push(Finding {
                            category: AnalysisCategory::Syntax,
                            severity: Severity::Warning,
                            message: "Empty else block".to_string(),
                            suggestion: Some("Add statements to the else block or remove it".to_string()),
                            location: (line, 1),
                            span_length: 0,
                            rule_id: "empty_else_block".to_string(),
                        });
                    }
                }
            }
            Statement::While { condition, body } => {
                findings.extend(self.check_expression_syntax(condition, line));
                
                if body.is_empty() {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Warning,
                        message: "Empty while loop body".to_string(),
                        suggestion: Some("Add statements to the while loop body".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "empty_while_body".to_string(),
                    });
                }
            }
            Statement::For { identifier, iterable, body } => {
                if identifier.is_empty() {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Error,
                        message: "Empty iterator variable in for loop".to_string(),
                        suggestion: Some("Provide a valid iterator variable name".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "empty_for_iterator".to_string(),
                    });
                }

                findings.extend(self.check_expression_syntax(iterable, line));
                
                if body.is_empty() {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Warning,
                        message: "Empty for loop body".to_string(),
                        suggestion: Some("Add statements to the for loop body".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "empty_for_body".to_string(),
                    });
                }
            }
            Statement::Return { value } => {
                if let Some(expr) = value {
                    findings.extend(self.check_expression_syntax(expr, line));
                }
            }
            Statement::Expression { expression } => {
                findings.extend(self.check_expression_syntax(expression, line));
            }
            Statement::Struct { name, .. } => {
                if name.is_empty() {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Error,
                        message: "Empty struct name".to_string(),
                        suggestion: Some("Provide a valid struct name".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "empty_struct_name".to_string(),
                    });
                }
            }
            Statement::Enum { name, .. } => {
                if name.is_empty() {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Error,
                        message: "Empty enum name".to_string(),
                        suggestion: Some("Provide a valid enum name".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "empty_enum_name".to_string(),
                    });
                }
            }
        }

        findings
    }

    /// Check syntax of expressions
    fn check_expression_syntax(&self, expression: &Expression, line: usize) -> Vec<Finding> {
        let mut findings = Vec::new();

        match expression {
            Expression::Identifier(name) => {
                if name.is_empty() {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Error,
                        message: "Empty identifier in expression".to_string(),
                        suggestion: Some("Provide a valid identifier".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "empty_expression_identifier".to_string(),
                    });
                }
            }
            Expression::Binary { left, right, .. } => {
                findings.extend(self.check_expression_syntax(left, line));
                findings.extend(self.check_expression_syntax(right, line));
            }
            Expression::Unary { operand, .. } => {
                findings.extend(self.check_expression_syntax(operand, line));
            }
            Expression::Call { function, arguments } => {
                if function.is_empty() {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Error,
                        message: "Empty function name in call".to_string(),
                        suggestion: Some("Provide a valid function name".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "empty_function_call".to_string(),
                    });
                }

                for arg in arguments {
                    findings.extend(self.check_expression_syntax(arg, line));
                }
            }
            Expression::FieldAccess { object, field } => {
                findings.extend(self.check_expression_syntax(object, line));
                if field.is_empty() {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Error,
                        message: "Empty field name in field access".to_string(),
                        suggestion: Some("Provide a valid field name".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "empty_field_access".to_string(),
                    });
                }
            }
            Expression::StructInstantiation { struct_name, fields } => {
                if struct_name.is_empty() {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Error,
                        message: "Empty struct name in instantiation".to_string(),
                        suggestion: Some("Provide a valid struct name".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "empty_struct_instantiation".to_string(),
                    });
                }

                for field in fields {
                    findings.extend(self.check_expression_syntax(&field.value, line));
                }
            }
            Expression::Range { start, end } => {
                findings.extend(self.check_expression_syntax(start, line));
                findings.extend(self.check_expression_syntax(end, line));
            }
            Expression::EnumVariantConstruction { enum_name, variant_name, data } => {
                if enum_name.is_empty() {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Error,
                        message: "Empty enum name in variant construction".to_string(),
                        suggestion: Some("Provide a valid enum name".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "empty_enum_variant_construction".to_string(),
                    });
                }
                if variant_name.is_empty() {
                    findings.push(Finding {
                        category: AnalysisCategory::Syntax,
                        severity: Severity::Error,
                        message: "Empty variant name in enum construction".to_string(),
                        suggestion: Some("Provide a valid variant name".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "empty_enum_variant_name".to_string(),
                    });
                }
                if let Some(data_expr) = data {
                    findings.extend(self.check_expression_syntax(data_expr, line));
                }
            }
            Expression::Index { object, index } => {
                findings.extend(self.check_expression_syntax(object, line));
                findings.extend(self.check_expression_syntax(index, line));
            }
            Expression::ArrayLiteral { elements } => {
                for element in elements {
                    findings.extend(self.check_expression_syntax(element, line));
                }
            }
            Expression::Literal(_) => {
                // Literals are generally fine syntactically
            }
        }

        findings
    }

    /// Check if a string is a reserved keyword
    fn is_reserved_keyword(&self, identifier: &str) -> bool {
        matches!(identifier, 
            "fn" | "mut" | "if" | "else" | "for" | "while" | 
            "struct" | "enum" | "unsafe" | "return" | 
            "true" | "false" | "seeAm"
        )
    }
}

impl Analyzer for SyntaxAnalyzer {
    fn analyze(&self, _source: &str, ast: &oviec::ast::AstNode) -> AprokoResult<Vec<Finding>> {
        let mut findings = Vec::new();
        
        // Check grammar compliance
        findings.extend(self.check_grammar_compliance(ast));
        
        Ok(findings)
    }

    fn category(&self) -> AnalysisCategory {
        AnalysisCategory::Syntax
    }

    fn configure(&mut self, config: &CategoryConfig) -> AprokoResult<()> {
        if let Some(check_grammar) = config.settings.get("check_grammar") {
            self.check_grammar = check_grammar == "true";
        }
        
        if let Some(validate_keywords) = config.settings.get("validate_keywords") {
            self.validate_keywords = validate_keywords == "true";
        }
        
        Ok(())
    }
}

impl Default for SyntaxAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}