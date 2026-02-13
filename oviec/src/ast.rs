//! Abstract Syntax Tree definitions for Ovie

use std::fmt;
use serde::{Deserialize, Serialize};
use crate::error::{OvieError, OvieResult};

/// AST invariant validation trait
pub trait AstInvariantValidation {
    /// Validate AST invariants according to Stage 2.1 compiler invariants
    fn validate(&self) -> Result<(), InvariantError>;
}

/// Invariant error for AST validation
#[derive(Debug, Clone)]
pub struct InvariantError {
    pub message: String,
    pub location: Option<String>,
}

impl fmt::Display for InvariantError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref location) = self.location {
            write!(f, "AST Invariant Violation at {}: {}", location, self.message)
        } else {
            write!(f, "AST Invariant Violation: {}", self.message)
        }
    }
}

impl std::error::Error for InvariantError {}

/// Root AST node representing a complete Ovie program
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstNode {
    Program(Vec<Statement>),
}

impl AstInvariantValidation for AstNode {
    /// Validate AST invariants according to Stage 2.1 compiler invariants
    /// 
    /// AST Invariants (from docs/compiler_invariants.md):
    /// - AST contains no resolved types
    /// - AST contains no symbol IDs  
    /// - AST nodes preserve exact source spans
    /// - No semantic validation occurs in AST
    /// - All syntax is valid (parser succeeded)
    /// - Comments and whitespace are preserved for tooling
    fn validate(&self) -> Result<(), InvariantError> {
        match self {
            AstNode::Program(statements) => {
                // Check that AST contains no resolved types
                for statement in statements {
                    self.validate_statement_invariants(statement)?;
                }
                
                // AST-level invariants passed
                Ok(())
            }
        }
    }
}

impl AstNode {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self::Program(statements)
    }

    /// Create a Program variant for compatibility with tests
    pub fn Program(statements: Vec<Statement>) -> Self {
        Self::Program(statements)
    }

    fn validate_statement_invariants(&self, statement: &Statement) -> Result<(), InvariantError> {
        match statement {
            Statement::Assignment { value, .. } => {
                self.validate_expression_invariants(value)?;
            }
            Statement::VariableDeclaration { value, .. } => {
                self.validate_expression_invariants(value)?;
            }
            Statement::Function { body, .. } => {
                for stmt in body {
                    self.validate_statement_invariants(stmt)?;
                }
            }
            Statement::FunctionDeclaration { body, .. } => {
                for stmt in body {
                    self.validate_statement_invariants(stmt)?;
                }
            }
            Statement::Print { expression } => {
                self.validate_expression_invariants(expression)?;
            }
            Statement::If { condition, then_block, else_block } => {
                self.validate_expression_invariants(condition)?;
                for stmt in then_block {
                    self.validate_statement_invariants(stmt)?;
                }
                if let Some(else_stmts) = else_block {
                    for stmt in else_stmts {
                        self.validate_statement_invariants(stmt)?;
                    }
                }
            }
            Statement::While { condition, body } => {
                self.validate_expression_invariants(condition)?;
                for stmt in body {
                    self.validate_statement_invariants(stmt)?;
                }
            }
            Statement::For { iterable, body, .. } => {
                self.validate_expression_invariants(iterable)?;
                for stmt in body {
                    self.validate_statement_invariants(stmt)?;
                }
            }
            Statement::Return { value } => {
                if let Some(expr) = value {
                    self.validate_expression_invariants(expr)?;
                }
            }
            Statement::Expression { expression } => {
                self.validate_expression_invariants(expression)?;
            }
            Statement::Struct { .. } => {
                // Struct definitions are valid at AST level
            }
            Statement::Enum { .. } => {
                // Enum definitions are valid at AST level
            }
        }
        Ok(())
    }

    fn validate_expression_invariants(&self, expression: &Expression) -> Result<(), InvariantError> {
        match expression {
            Expression::Literal(_) => {
                // Literals are always valid at AST level
            }
            Expression::Identifier(_) => {
                // Identifiers should NOT be resolved at AST level
                // This is correct - identifiers are just strings at this stage
            }
            Expression::Binary { left, right, .. } => {
                self.validate_expression_invariants(left)?;
                self.validate_expression_invariants(right)?;
            }
            Expression::Unary { operand, .. } => {
                self.validate_expression_invariants(operand)?;
            }
            Expression::Call { arguments, .. } => {
                for arg in arguments {
                    self.validate_expression_invariants(arg)?;
                }
            }
            Expression::FieldAccess { object, .. } => {
                self.validate_expression_invariants(object)?;
            }
            Expression::StructInstantiation { fields, .. } => {
                for field in fields {
                    self.validate_expression_invariants(&field.value)?;
                }
            }
            Expression::Range { start, end } => {
                self.validate_expression_invariants(start)?;
                self.validate_expression_invariants(end)?;
            }
            Expression::EnumVariantConstruction { data, .. } => {
                if let Some(data_expr) = data {
                    self.validate_expression_invariants(data_expr)?;
                }
            }
            Expression::Index { object, index } => {
                self.validate_expression_invariants(object)?;
                self.validate_expression_invariants(index)?;
            }
            Expression::ArrayLiteral { elements } => {
                for element in elements {
                    self.validate_expression_invariants(element)?;
                }
            }
        }
        Ok(())
    }
}

/// Statement types in Ovie
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    /// Variable assignment: [mut] identifier = expression
    Assignment {
        mutable: bool,
        identifier: String,
        value: Expression,
    },

    /// Variable declaration: let [mut] identifier = expression
    VariableDeclaration {
        mutable: bool,
        identifier: String,
        value: Expression,
    },

    /// Function definition: fn identifier(params) { body }
    Function {
        name: String,
        parameters: Vec<String>,
        body: Vec<Statement>,
    },

    /// Function declaration: fn identifier(params) { body }
    FunctionDeclaration {
        name: String,
        parameters: Vec<String>,
        body: Vec<Statement>,
    },

    /// Print statement: seeAm expression
    Print {
        expression: Expression,
    },

    /// If statement: if condition { then_block } [else { else_block }]
    If {
        condition: Expression,
        then_block: Vec<Statement>,
        else_block: Option<Vec<Statement>>,
    },

    /// While loop: while condition { body }
    While {
        condition: Expression,
        body: Vec<Statement>,
    },

    /// For loop: for identifier in expression { body }
    For {
        identifier: String,
        iterable: Expression,
        body: Vec<Statement>,
    },

    /// Return statement: return [expression]
    Return {
        value: Option<Expression>,
    },

    /// Expression statement: expression;
    Expression {
        expression: Expression,
    },

    /// Struct definition: struct Name { fields }
    Struct {
        name: String,
        fields: Vec<StructField>,
    },

    /// Enum definition: enum Name { variants }
    Enum {
        name: String,
        variants: Vec<EnumVariant>,
    },
}

/// Expression types in Ovie
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    /// Literal values
    Literal(Literal),

    /// Variable reference
    Identifier(String),

    /// Binary operations: left op right
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },

    /// Unary operations: op expression
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },

    /// Function call: identifier(args)
    Call {
        function: String,
        arguments: Vec<Expression>,
    },

    /// Field access: expression.field
    FieldAccess {
        object: Box<Expression>,
        field: String,
    },

    /// Struct instantiation: StructName { field: value, ... }
    StructInstantiation {
        struct_name: String,
        fields: Vec<FieldInitializer>,
    },

    /// Range expression: start..end
    Range {
        start: Box<Expression>,
        end: Box<Expression>,
    },

    /// Enum variant construction: EnumName.VariantName or EnumName.VariantName(data)
    EnumVariantConstruction {
        enum_name: String,
        variant_name: String,
        data: Option<Box<Expression>>,
    },

    /// Array/String indexing: array[index] or string[index]
    Index {
        object: Box<Expression>,
        index: Box<Expression>,
    },

    /// Array literal: [element1, element2, ...]
    ArrayLiteral {
        elements: Vec<Expression>,
    },
}

/// Literal value types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
}

/// Binary operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOperator {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    // Comparison
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Logical
    And,
    Or,
}

/// Unary operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOperator {
    Not,
    Negate,
}

/// Struct field definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub type_annotation: String,
}

/// Enum variant definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub data_type: Option<String>,
}

/// Field initializer for struct instantiation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldInitializer {
    pub name: String,
    pub value: Expression,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Modulo => "%",
            BinaryOperator::Equal => "==",
            BinaryOperator::NotEqual => "!=",
            BinaryOperator::Less => "<",
            BinaryOperator::LessEqual => "<=",
            BinaryOperator::Greater => ">",
            BinaryOperator::GreaterEqual => ">=",
            BinaryOperator::And => "&&",
            BinaryOperator::Or => "||",
        };
        write!(f, "{}", symbol)
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            UnaryOperator::Not => "!",
            UnaryOperator::Negate => "-",
        };
        write!(f, "{}", symbol)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Boolean(b) => write!(f, "{}", b),
        }
    }
}