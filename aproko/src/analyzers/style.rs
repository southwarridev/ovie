//! Style analysis for code style and best practices

use crate::{Analyzer, AnalysisCategory, AprokoResult, Finding, Severity, CategoryConfig};
use oviec::ast::{AstNode, Statement, Expression, Literal};

/// Analyzer for code style and best practices
pub struct StyleAnalyzer {
    enforce_naming: bool,
    check_formatting: bool,
    max_line_length: usize,
}

impl StyleAnalyzer {
    pub fn new() -> Self {
        Self {
            enforce_naming: true,
            check_formatting: true,
            max_line_length: 100,
        }
    }

    /// Enforce naming conventions
    fn enforce_naming_conventions(&self, ast: &AstNode) -> Vec<Finding> {
        let mut findings = Vec::new();
        
        if !self.enforce_naming {
            return findings;
        }

        for (i, statement) in ast.statements.iter().enumerate() {
            findings.extend(self.check_statement_naming(statement, i + 1));
        }

        findings
    }

    /// Check naming conventions for statements
    fn check_statement_naming(&self, statement: &Statement, line: usize) -> Vec<Finding> {
        let mut findings = Vec::new();

        match statement {
            Statement::Assignment { identifier, .. } => {
                findings.extend(self.check_variable_naming(identifier, line));
            }
            Statement::Function { name, parameters, body } => {
                findings.extend(self.check_function_naming(name, line));
                
                for param in parameters {
                    findings.extend(self.check_parameter_naming(param, line));
                }
                
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_naming(stmt, line + i + 1));
                }
            }
            Statement::Struct { name, .. } => {
                findings.extend(self.check_type_naming(name, line));
            }
            Statement::Enum { name, .. } => {
                findings.extend(self.check_type_naming(name, line));
            }
            Statement::For { identifier, body, .. } => {
                findings.extend(self.check_variable_naming(identifier, line));
                
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_naming(stmt, line + i + 1));
                }
            }
            Statement::If { then_block, else_block, .. } => {
                for (i, stmt) in then_block.iter().enumerate() {
                    findings.extend(self.check_statement_naming(stmt, line + i + 1));
                }
                
                if let Some(else_stmts) = else_block {
                    for (i, stmt) in else_stmts.iter().enumerate() {
                        findings.extend(self.check_statement_naming(stmt, line + i + 1));
                    }
                }
            }
            Statement::While { body, .. } => {
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_naming(stmt, line + i + 1));
                }
            }
            _ => {}
        }

        findings
    }

    /// Check variable naming conventions
    fn check_variable_naming(&self, name: &str, line: usize) -> Vec<Finding> {
        let mut findings = Vec::new();

        // Check for camelCase convention
        if !self.is_camel_case(name) && !self.is_snake_case(name) {
            findings.push(Finding {
                category: AnalysisCategory::Style,
                severity: Severity::Warning,
                message: format!("Variable '{}' should use camelCase or snake_case", name),
                suggestion: Some("Use camelCase (myVariable) or snake_case (my_variable)".to_string()),
                location: (line, 1),
                span_length: name.len(),
                rule_id: "variable_naming_convention".to_string(),
            });
        }

        // Check for meaningful names
        if name.len() < 2 && !matches!(name, "i" | "j" | "k" | "x" | "y" | "z") {
            findings.push(Finding {
                category: AnalysisCategory::Style,
                severity: Severity::Info,
                message: format!("Variable '{}' has a very short name", name),
                suggestion: Some("Consider using a more descriptive name".to_string()),
                location: (line, 1),
                span_length: name.len(),
                rule_id: "short_variable_name".to_string(),
            });
        }

        // Check for overly long names
        if name.len() > 30 {
            findings.push(Finding {
                category: AnalysisCategory::Style,
                severity: Severity::Info,
                message: format!("Variable '{}' has a very long name", name),
                suggestion: Some("Consider using a shorter, more concise name".to_string()),
                location: (line, 1),
                span_length: name.len(),
                rule_id: "long_variable_name".to_string(),
            });
        }

        // Check for Hungarian notation (discouraged)
        if self.uses_hungarian_notation(name) {
            findings.push(Finding {
                category: AnalysisCategory::Style,
                severity: Severity::Warning,
                message: format!("Variable '{}' appears to use Hungarian notation", name),
                suggestion: Some("Avoid Hungarian notation; use descriptive names instead".to_string()),
                location: (line, 1),
                span_length: name.len(),
                rule_id: "hungarian_notation".to_string(),
            });
        }

        findings
    }

    /// Check function naming conventions
    fn check_function_naming(&self, name: &str, line: usize) -> Vec<Finding> {
        let mut findings = Vec::new();

        // Functions should use camelCase or snake_case
        if !self.is_camel_case(name) && !self.is_snake_case(name) {
            findings.push(Finding {
                category: AnalysisCategory::Style,
                severity: Severity::Warning,
                message: format!("Function '{}' should use camelCase or snake_case", name),
                suggestion: Some("Use camelCase (myFunction) or snake_case (my_function)".to_string()),
                location: (line, 1),
                span_length: name.len(),
                rule_id: "function_naming_convention".to_string(),
            });
        }

        // Check for verb-based function names
        if !self.is_verb_based_name(name) {
            findings.push(Finding {
                category: AnalysisCategory::Style,
                severity: Severity::Info,
                message: format!("Function '{}' should start with a verb", name),
                suggestion: Some("Function names should describe what they do (e.g., calculateSum, getUserData)".to_string()),
                location: (line, 1),
                span_length: name.len(),
                rule_id: "function_verb_naming".to_string(),
            });
        }

        findings
    }

    /// Check parameter naming conventions
    fn check_parameter_naming(&self, name: &str, line: usize) -> Vec<Finding> {
        // Parameters follow same rules as variables
        self.check_variable_naming(name, line)
    }

    /// Check type naming conventions (structs, enums)
    fn check_type_naming(&self, name: &str, line: usize) -> Vec<Finding> {
        let mut findings = Vec::new();

        // Types should use PascalCase
        if !self.is_pascal_case(name) {
            findings.push(Finding {
                category: AnalysisCategory::Style,
                severity: Severity::Warning,
                message: format!("Type '{}' should use PascalCase", name),
                suggestion: Some("Use PascalCase for type names (e.g., MyStruct, UserData)".to_string()),
                location: (line, 1),
                span_length: name.len(),
                rule_id: "type_naming_convention".to_string(),
            });
        }

        // Check for noun-based type names
        if !self.is_noun_based_name(name) {
            findings.push(Finding {
                category: AnalysisCategory::Style,
                severity: Severity::Info,
                message: format!("Type '{}' should be a noun or noun phrase", name),
                suggestion: Some("Type names should describe what they represent (e.g., User, DatabaseConnection)".to_string()),
                location: (line, 1),
                span_length: name.len(),
                rule_id: "type_noun_naming".to_string(),
            });
        }

        findings
    }

    /// Check code formatting and structure
    fn check_formatting(&self, source: &str) -> Vec<Finding> {
        let mut findings = Vec::new();
        
        if !self.check_formatting {
            return findings;
        }

        let lines: Vec<&str> = source.lines().collect();
        
        for (i, line) in lines.iter().enumerate() {
            let line_num = i + 1;
            
            // Check line length
            if line.len() > self.max_line_length {
                findings.push(Finding {
                    category: AnalysisCategory::Style,
                    severity: Severity::Warning,
                    message: format!("Line {} exceeds maximum length of {} characters", line_num, self.max_line_length),
                    suggestion: Some("Break long lines into multiple lines".to_string()),
                    location: (line_num, 1),
                    span_length: line.len(),
                    rule_id: "line_too_long".to_string(),
                });
            }

            // Check for trailing whitespace
            if line.ends_with(' ') || line.ends_with('\t') {
                findings.push(Finding {
                    category: AnalysisCategory::Style,
                    severity: Severity::Info,
                    message: format!("Line {} has trailing whitespace", line_num),
                    suggestion: Some("Remove trailing whitespace".to_string()),
                    location: (line_num, line.len()),
                    span_length: 1,
                    rule_id: "trailing_whitespace".to_string(),
                });
            }

            // Check for mixed tabs and spaces
            if line.contains('\t') && line.contains("    ") {
                findings.push(Finding {
                    category: AnalysisCategory::Style,
                    severity: Severity::Warning,
                    message: format!("Line {} mixes tabs and spaces for indentation", line_num),
                    suggestion: Some("Use either tabs or spaces consistently for indentation".to_string()),
                    location: (line_num, 1),
                    span_length: 0,
                    rule_id: "mixed_indentation".to_string(),
                });
            }

            // Check for multiple consecutive blank lines
            if i > 0 && line.trim().is_empty() && lines.get(i - 1).map_or(false, |prev| prev.trim().is_empty()) {
                findings.push(Finding {
                    category: AnalysisCategory::Style,
                    severity: Severity::Info,
                    message: format!("Multiple consecutive blank lines at line {}", line_num),
                    suggestion: Some("Use single blank lines to separate code sections".to_string()),
                    location: (line_num, 1),
                    span_length: 0,
                    rule_id: "multiple_blank_lines".to_string(),
                });
            }
        }

        findings
    }

    /// Check for code complexity and best practices
    fn check_best_practices(&self, ast: &AstNode) -> Vec<Finding> {
        let mut findings = Vec::new();

        for (i, statement) in ast.statements.iter().enumerate() {
            findings.extend(self.check_statement_best_practices(statement, i + 1));
        }

        findings
    }

    /// Check best practices for statements
    fn check_statement_best_practices(&self, statement: &Statement, line: usize) -> Vec<Finding> {
        let mut findings = Vec::new();

        match statement {
            Statement::Function { parameters, body, .. } => {
                // Check function length
                if body.len() > 20 {
                    findings.push(Finding {
                        category: AnalysisCategory::Style,
                        severity: Severity::Warning,
                        message: "Function is very long".to_string(),
                        suggestion: Some("Consider breaking this function into smaller functions".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "long_function".to_string(),
                    });
                }

                // Check parameter count
                if parameters.len() > 5 {
                    findings.push(Finding {
                        category: AnalysisCategory::Style,
                        severity: Severity::Warning,
                        message: "Function has too many parameters".to_string(),
                        suggestion: Some("Consider using a struct to group related parameters".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "too_many_parameters".to_string(),
                    });
                }

                // Recursively check body
                for (i, stmt) in body.iter().enumerate() {
                    findings.extend(self.check_statement_best_practices(stmt, line + i + 1));
                }
            }
            Statement::If { condition, then_block, else_block } => {
                // Check for complex conditions
                if self.is_complex_condition(condition) {
                    findings.push(Finding {
                        category: AnalysisCategory::Style,
                        severity: Severity::Info,
                        message: "Complex condition in if statement".to_string(),
                        suggestion: Some("Consider extracting condition to a well-named variable".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "complex_condition".to_string(),
                    });
                }

                // Check for deeply nested blocks
                let nesting_depth = self.calculate_nesting_depth(then_block);
                if nesting_depth > 3 {
                    findings.push(Finding {
                        category: AnalysisCategory::Style,
                        severity: Severity::Warning,
                        message: "Deeply nested code block".to_string(),
                        suggestion: Some("Consider using early returns or extracting functions to reduce nesting".to_string()),
                        location: (line, 1),
                        span_length: 0,
                        rule_id: "deep_nesting".to_string(),
                    });
                }

                // Recursively check blocks
                for (i, stmt) in then_block.iter().enumerate() {
                    findings.extend(self.check_statement_best_practices(stmt, line + i + 1));
                }
                
                if let Some(else_stmts) = else_block {
                    for (i, stmt) in else_stmts.iter().enumerate() {
                        findings.extend(self.check_statement_best_practices(stmt, line + i + 1));
                    }
                }
            }
            Statement::Assignment { value, .. } => {
                // Check for magic numbers
                if let Expression::Literal(Literal::Number(n)) = value {
                    if self.is_magic_number(*n) {
                        findings.push(Finding {
                            category: AnalysisCategory::Style,
                            severity: Severity::Info,
                            message: format!("Magic number {} should be a named constant", n),
                            suggestion: Some("Define this number as a named constant".to_string()),
                            location: (line, 1),
                            span_length: 0,
                            rule_id: "magic_number".to_string(),
                        });
                    }
                }
            }
            _ => {}
        }

        findings
    }

    /// Check if name follows camelCase convention
    fn is_camel_case(&self, name: &str) -> bool {
        if name.is_empty() {
            return false;
        }
        
        let first_char = name.chars().next().unwrap();
        if !first_char.is_ascii_lowercase() {
            return false;
        }
        
        !name.contains('_') && name.chars().all(|c| c.is_alphanumeric())
    }

    /// Check if name follows snake_case convention
    fn is_snake_case(&self, name: &str) -> bool {
        if name.is_empty() {
            return false;
        }
        
        name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') &&
        !name.starts_with('_') && !name.ends_with('_') && !name.contains("__")
    }

    /// Check if name follows PascalCase convention
    fn is_pascal_case(&self, name: &str) -> bool {
        if name.is_empty() {
            return false;
        }
        
        let first_char = name.chars().next().unwrap();
        if !first_char.is_ascii_uppercase() {
            return false;
        }
        
        !name.contains('_') && name.chars().all(|c| c.is_alphanumeric())
    }

    /// Check if name uses Hungarian notation
    fn uses_hungarian_notation(&self, name: &str) -> bool {
        if name.len() < 3 {
            return false;
        }
        
        let prefixes = ["str", "int", "bool", "arr", "obj", "ptr", "ref"];
        prefixes.iter().any(|prefix| name.to_lowercase().starts_with(prefix))
    }

    /// Check if name is verb-based (for functions)
    fn is_verb_based_name(&self, name: &str) -> bool {
        let verbs = [
            "get", "set", "is", "has", "can", "should", "will", "create", "make", "build",
            "add", "remove", "delete", "update", "modify", "change", "convert", "transform",
            "calculate", "compute", "process", "handle", "manage", "control", "execute",
            "run", "start", "stop", "pause", "resume", "load", "save", "read", "write",
            "parse", "format", "validate", "check", "test", "find", "search", "filter",
            "sort", "compare", "merge", "split", "join", "combine", "extract", "generate"
        ];
        
        let name_lower = name.to_lowercase();
        verbs.iter().any(|verb| name_lower.starts_with(verb))
    }

    /// Check if name is noun-based (for types)
    fn is_noun_based_name(&self, name: &str) -> bool {
        // This is a simplified check - in practice, this would be more sophisticated
        let name_lower = name.to_lowercase();
        
        // Check if it ends with common noun suffixes
        let noun_suffixes = ["er", "or", "tion", "sion", "ment", "ness", "ity", "ty", "data", "info", "config", "manager", "handler", "service", "client", "server"];
        
        noun_suffixes.iter().any(|suffix| name_lower.ends_with(suffix)) ||
        // Or if it doesn't start with a verb
        !self.is_verb_based_name(name)
    }

    /// Check if condition is complex
    fn is_complex_condition(&self, condition: &Expression) -> bool {
        self.count_logical_operators(condition) > 2
    }

    /// Count logical operators in expression
    fn count_logical_operators(&self, expression: &Expression) -> usize {
        match expression {
            Expression::Binary { left, right, .. } => {
                1 + self.count_logical_operators(left) + self.count_logical_operators(right)
            }
            Expression::Unary { operand, .. } => {
                1 + self.count_logical_operators(operand)
            }
            _ => 0,
        }
    }

    /// Calculate nesting depth of statements
    fn calculate_nesting_depth(&self, statements: &[Statement]) -> usize {
        let mut max_depth = 0;
        
        for statement in statements {
            let depth = match statement {
                Statement::If { then_block, else_block, .. } => {
                    let then_depth = self.calculate_nesting_depth(then_block);
                    let else_depth = else_block.as_ref().map_or(0, |stmts| self.calculate_nesting_depth(stmts));
                    1 + then_depth.max(else_depth)
                }
                Statement::While { body, .. } | Statement::For { body, .. } => {
                    1 + self.calculate_nesting_depth(body)
                }
                Statement::Function { body, .. } => {
                    self.calculate_nesting_depth(body)
                }
                _ => 0,
            };
            
            max_depth = max_depth.max(depth);
        }
        
        max_depth
    }

    /// Check if number is a magic number
    fn is_magic_number(&self, n: f64) -> bool {
        // Common non-magic numbers
        let common_numbers = [0.0, 1.0, -1.0, 2.0, 10.0, 100.0, 1000.0];
        
        !common_numbers.contains(&n) && n.fract() == 0.0 && n.abs() > 1.0 && n.abs() < 1000.0
    }
}

impl Analyzer for StyleAnalyzer {
    fn analyze(&self, source: &str, ast: &oviec::ast::AstNode) -> AprokoResult<Vec<Finding>> {
        let mut findings = Vec::new();
        
        // Check naming conventions
        findings.extend(self.enforce_naming_conventions(ast));
        
        // Check formatting
        findings.extend(self.check_formatting(source));
        
        // Check best practices
        findings.extend(self.check_best_practices(ast));
        
        Ok(findings)
    }

    fn category(&self) -> AnalysisCategory {
        AnalysisCategory::Style
    }

    fn configure(&mut self, config: &CategoryConfig) -> AprokoResult<()> {
        if let Some(enforce_naming) = config.settings.get("enforce_naming") {
            self.enforce_naming = enforce_naming == "true";
        }
        
        if let Some(check_formatting) = config.settings.get("check_formatting") {
            self.check_formatting = check_formatting == "true";
        }
        
        if let Some(max_line_length) = config.settings.get("max_line_length") {
            if let Ok(length) = max_line_length.parse::<usize>() {
                self.max_line_length = length;
            }
        }
        
        Ok(())
    }
}

impl Default for StyleAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}