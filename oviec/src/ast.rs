//! Abstract Syntax Tree definitions for Ovie

use std::fmt;
use serde::{Deserialize, Serialize};

/// Root AST node representing a complete Ovie program
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AstNode {
    pub statements: Vec<Statement>,
}

impl AstNode {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
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

    /// Function definition: fn identifier(params) { body }
    Function {
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