//! High-level Intermediate Representation (HIR) for the Ovie compiler
//! 
//! HIR is the first IR stage after AST, where names are resolved and types are known.
//! This stage performs semantic analysis and type checking.

use crate::ast::{AstNode, Statement, Expression, Literal, BinaryOperator, UnaryOperator};
use crate::error::{OvieError, OvieResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unique identifier for HIR nodes
pub type NodeId = u32;

/// Symbol table entry
pub type Symbol = String;

/// Source location information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceSpan {
    pub start: usize,
    pub end: usize,
    pub line: u32,
    pub column: u32,
}

impl Default for SourceSpan {
    fn default() -> Self {
        Self {
            start: 0,
            end: 0,
            line: 1,
            column: 1,
        }
    }
}

/// HIR Program - the complete program after semantic analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirProgram {
    pub items: Vec<HirItem>,
    pub symbol_table: SymbolTable,
    pub type_table: TypeTable,
    pub metadata: HirMetadata,
}

/// Top-level items in HIR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HirItem {
    Function(HirFunction),
    Struct(HirStruct),
    Enum(HirEnum),
    Global(HirGlobal),
}

/// HIR Function with resolved names and types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirFunction {
    pub id: NodeId,
    pub name: Symbol,
    pub parameters: Vec<HirParameter>,
    pub return_type: HirType,
    pub body: HirBlock,
    pub span: SourceSpan,
    pub is_main: bool,
}

/// Function parameter with type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirParameter {
    pub name: Symbol,
    pub param_type: HirType,
    pub span: SourceSpan,
}

/// HIR Struct definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirStruct {
    pub id: NodeId,
    pub name: Symbol,
    pub fields: Vec<HirField>,
    pub span: SourceSpan,
}

/// Struct field with type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirField {
    pub name: Symbol,
    pub field_type: HirType,
    pub span: SourceSpan,
}

/// HIR Enum definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirEnum {
    pub id: NodeId,
    pub name: Symbol,
    pub variants: Vec<HirVariant>,
    pub span: SourceSpan,
}

/// Enum variant with optional data type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirVariant {
    pub name: Symbol,
    pub data_type: Option<HirType>,
    pub span: SourceSpan,
}

/// Global variable or constant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirGlobal {
    pub id: NodeId,
    pub name: Symbol,
    pub global_type: HirType,
    pub is_mutable: bool,
    pub initializer: Option<HirExpression>,
    pub span: SourceSpan,
}

/// HIR Block (sequence of statements)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirBlock {
    pub statements: Vec<HirStatement>,
    pub span: SourceSpan,
}

/// HIR Statement with type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirStatement {
    pub id: NodeId,
    pub kind: HirStatementKind,
    pub span: SourceSpan,
}

/// HIR Statement kinds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HirStatementKind {
    /// Local variable declaration
    Local {
        name: Symbol,
        var_type: HirType,
        is_mutable: bool,
        initializer: Option<HirExpression>,
    },
    
    /// Assignment to existing variable
    Assign {
        target: HirPlace,
        value: HirExpression,
    },
    
    /// Expression statement
    Expression(HirExpression),
    
    /// Print statement (Ovie-specific)
    Print(HirExpression),
    
    /// Return statement
    Return(Option<HirExpression>),
    
    /// If statement
    If {
        condition: HirExpression,
        then_block: HirBlock,
        else_block: Option<HirBlock>,
    },
    
    /// While loop
    While {
        condition: HirExpression,
        body: HirBlock,
    },
    
    /// For loop
    For {
        variable: Symbol,
        iterable: HirExpression,
        body: HirBlock,
    },
}

/// HIR Expression with type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirExpression {
    pub id: NodeId,
    pub kind: HirExpressionKind,
    pub expr_type: HirType,
    pub span: SourceSpan,
}

/// HIR Expression kinds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HirExpressionKind {
    /// Literal value
    Literal(HirLiteral),
    
    /// Variable reference
    Variable(Symbol),
    
    /// Binary operation
    Binary {
        left: Box<HirExpression>,
        op: HirBinaryOp,
        right: Box<HirExpression>,
    },
    
    /// Unary operation
    Unary {
        op: HirUnaryOp,
        operand: Box<HirExpression>,
    },
    
    /// Function call
    Call {
        function: Symbol,
        arguments: Vec<HirExpression>,
    },
    
    /// Field access
    FieldAccess {
        object: Box<HirExpression>,
        field: Symbol,
    },
    
    /// Struct instantiation
    StructInit {
        struct_name: Symbol,
        fields: Vec<HirFieldInit>,
    },
    
    /// Range expression
    Range {
        start: Box<HirExpression>,
        end: Box<HirExpression>,
    },
}

/// HIR Place (assignable location)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirPlace {
    pub kind: HirPlaceKind,
    pub place_type: HirType,
    pub span: SourceSpan,
}

/// HIR Place kinds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HirPlaceKind {
    /// Local variable
    Local(Symbol),
    
    /// Field access
    Field {
        object: Box<HirPlace>,
        field: Symbol,
    },
}

/// HIR Literal values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HirLiteral {
    String(String),
    Number(f64),
    Boolean(bool),
    Unit,
}

/// HIR Binary operators (same as AST but with type information)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HirBinaryOp {
    Add, Sub, Mul, Div, Mod,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or,
}

/// HIR Unary operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HirUnaryOp {
    Not, Neg,
}

/// Field initializer in struct instantiation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirFieldInit {
    pub name: Symbol,
    pub value: HirExpression,
    pub span: SourceSpan,
}

/// HIR Type system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HirType {
    /// Primitive types
    String,
    Number,
    Boolean,
    Unit,
    
    /// User-defined struct
    Struct(Symbol),
    
    /// User-defined enum
    Enum(Symbol),
    
    /// Function type
    Function {
        params: Vec<HirType>,
        return_type: Box<HirType>,
    },
    
    /// Range type
    Range(Box<HirType>),
    
    /// Error type (for error recovery)
    Error,
    
    /// Inferred type (during type checking)
    Infer(u32),
}

/// Symbol table for name resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
    current_scope: usize,
}

/// Scope in symbol table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scope {
    symbols: HashMap<Symbol, SymbolInfo>,
    parent: Option<usize>,
}

/// Symbol information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolInfo {
    pub symbol_type: HirType,
    pub is_mutable: bool,
    pub is_function: bool,
    pub span: SourceSpan,
}

/// Type table for type definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeTable {
    types: HashMap<Symbol, TypeInfo>,
}

/// Type definition information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeInfo {
    Struct {
        fields: HashMap<Symbol, HirType>,
    },
    Enum {
        variants: HashMap<Symbol, Option<HirType>>,
    },
}

/// HIR metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirMetadata {
    pub source_file: String,
    pub compiler_version: String,
    pub has_main_function: bool,
    pub error_count: usize,
    pub warning_count: usize,
}

/// HIR Builder - transforms AST to HIR
pub struct HirBuilder {
    next_node_id: NodeId,
    symbol_table: SymbolTable,
    type_table: TypeTable,
    errors: Vec<OvieError>,
    warnings: Vec<OvieError>,
}

impl HirBuilder {
    /// Create a new HIR builder
    pub fn new() -> Self {
        let mut symbol_table = SymbolTable::new();
        let mut type_table = TypeTable::new();
        
        // Pre-populate with built-in types and functions
        Self::populate_builtins(&mut symbol_table, &mut type_table);
        
        Self {
            next_node_id: 1,
            symbol_table,
            type_table,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Populate symbol table with built-in types and functions
    fn populate_builtins(symbol_table: &mut SymbolTable, type_table: &mut TypeTable) {
        // Built-in types are already handled in resolve_type()
        
        // Built-in functions
        let print_type = HirType::Function {
            params: vec![HirType::String],
            return_type: Box::new(HirType::Unit),
        };
        
        let _ = symbol_table.insert("print".to_string(), SymbolInfo {
            symbol_type: print_type,
            is_mutable: false,
            is_function: true,
            span: SourceSpan::default(),
        });
        
        // String conversion functions
        let to_string_type = HirType::Function {
            params: vec![HirType::Number],
            return_type: Box::new(HirType::String),
        };
        
        let _ = symbol_table.insert("to_string".to_string(), SymbolInfo {
            symbol_type: to_string_type,
            is_mutable: false,
            is_function: true,
            span: SourceSpan::default(),
        });
    }

    /// Generate next node ID
    fn next_id(&mut self) -> NodeId {
        let id = self.next_node_id;
        self.next_node_id += 1;
        id
    }

    /// Transform AST to HIR with comprehensive error handling
    pub fn transform_ast(&mut self, ast: &AstNode) -> OvieResult<HirProgram> {
        let mut items = Vec::new();
        let mut has_main = false;

        // First pass: collect type definitions and validate them
        for statement in &ast.statements {
            match statement {
                Statement::Struct { name, fields } => {
                    if let Err(e) = self.validate_struct_definition(name, fields) {
                        self.errors.push(e);
                        continue;
                    }
                    let hir_struct = self.transform_struct(name, fields)?;
                    self.register_struct_type(name, fields)?;
                    items.push(HirItem::Struct(hir_struct));
                }
                Statement::Enum { name, variants } => {
                    if let Err(e) = self.validate_enum_definition(name, variants) {
                        self.errors.push(e);
                        continue;
                    }
                    let hir_enum = self.transform_enum(name, variants)?;
                    self.register_enum_type(name, variants)?;
                    items.push(HirItem::Enum(hir_enum));
                }
                _ => {}
            }
        }

        // Second pass: collect function signatures and validate them
        for statement in &ast.statements {
            if let Statement::Function { name, parameters, body: _ } = statement {
                if let Err(e) = self.validate_function_signature(name, parameters) {
                    self.errors.push(e);
                    continue;
                }
                self.register_function(name, parameters)?;
            }
        }

        // Third pass: transform functions and other items with full context
        for statement in &ast.statements {
            match statement {
                Statement::Function { name, parameters, body } => {
                    match self.transform_function(name, parameters, body) {
                        Ok(hir_function) => {
                            if name == "main" {
                                has_main = true;
                            }
                            items.push(HirItem::Function(hir_function));
                        }
                        Err(e) => {
                            self.errors.push(e);
                        }
                    }
                }
                Statement::Assignment { identifier, value, mutable } => {
                    // Global variable
                    match self.transform_global(identifier, value, *mutable) {
                        Ok(hir_global) => {
                            items.push(HirItem::Global(hir_global));
                        }
                        Err(e) => {
                            self.errors.push(e);
                        }
                    }
                }
                Statement::Struct { .. } | Statement::Enum { .. } => {
                    // Already handled in first pass
                }
                _ => {
                    // Other statements at top level - create implicit main
                }
            }
        }

        // If no main function found, create an implicit one from top-level statements
        if !has_main {
            let main_statements: Vec<_> = ast.statements.iter()
                .filter(|stmt| !matches!(stmt, 
                    Statement::Function { .. } | 
                    Statement::Struct { .. } | 
                    Statement::Enum { .. } |
                    Statement::Assignment { .. }
                ))
                .collect();

            if !main_statements.is_empty() {
                match self.create_implicit_main(&main_statements) {
                    Ok(main_function) => {
                        items.push(HirItem::Function(main_function));
                        has_main = true;
                    }
                    Err(e) => {
                        self.errors.push(e);
                    }
                }
            }
        }

        // Perform final validation and type inference
        self.perform_type_inference(&mut items)?;
        self.validate_program_semantics(&items)?;

        let metadata = HirMetadata {
            source_file: "main.ov".to_string(),
            compiler_version: env!("CARGO_PKG_VERSION").to_string(),
            has_main_function: has_main,
            error_count: self.errors.len(),
            warning_count: self.warnings.len(),
        };

        // Return errors if any occurred
        if !self.errors.is_empty() {
            return Err(self.errors[0].clone());
        }

        Ok(HirProgram {
            items,
            symbol_table: self.symbol_table.clone(),
            type_table: self.type_table.clone(),
            metadata,
        })
    }

    /// Transform a struct definition
    fn transform_struct(&mut self, name: &str, fields: &[crate::ast::StructField]) -> OvieResult<HirStruct> {
        let mut hir_fields = Vec::new();
        
        for field in fields {
            let field_type = self.resolve_type(&field.type_annotation)?;
            hir_fields.push(HirField {
                name: field.name.clone(),
                field_type,
                span: SourceSpan::default(),
            });
        }

        Ok(HirStruct {
            id: self.next_id(),
            name: name.to_string(),
            fields: hir_fields,
            span: SourceSpan::default(),
        })
    }

    /// Transform an enum definition
    fn transform_enum(&mut self, name: &str, variants: &[crate::ast::EnumVariant]) -> OvieResult<HirEnum> {
        let mut hir_variants = Vec::new();
        
        for variant in variants {
            let data_type = if let Some(ref type_name) = variant.data_type {
                Some(self.resolve_type(type_name)?)
            } else {
                None
            };
            
            hir_variants.push(HirVariant {
                name: variant.name.clone(),
                data_type,
                span: SourceSpan::default(),
            });
        }

        Ok(HirEnum {
            id: self.next_id(),
            name: name.to_string(),
            variants: hir_variants,
            span: SourceSpan::default(),
        })
    }

    /// Transform a function definition
    fn transform_function(&mut self, name: &str, parameters: &[String], body: &[Statement]) -> OvieResult<HirFunction> {
        self.symbol_table.enter_scope();

        // Add parameters to scope
        let mut hir_params = Vec::new();
        for param in parameters {
            let param_type = HirType::Infer(self.next_id()); // Type inference
            self.symbol_table.insert(param.clone(), SymbolInfo {
                symbol_type: param_type.clone(),
                is_mutable: false,
                is_function: false,
                span: SourceSpan::default(),
            })?;
            
            hir_params.push(HirParameter {
                name: param.clone(),
                param_type,
                span: SourceSpan::default(),
            });
        }

        // Transform function body
        let hir_body = self.transform_block(body)?;
        
        self.symbol_table.exit_scope();

        Ok(HirFunction {
            id: self.next_id(),
            name: name.to_string(),
            parameters: hir_params,
            return_type: HirType::Infer(self.next_id()), // Type inference
            body: hir_body,
            span: SourceSpan::default(),
            is_main: name == "main",
        })
    }

    /// Transform a global variable
    fn transform_global(&mut self, name: &str, value: &Expression, is_mutable: bool) -> OvieResult<HirGlobal> {
        let initializer = self.transform_expression(value)?;
        let global_type = initializer.expr_type.clone();

        Ok(HirGlobal {
            id: self.next_id(),
            name: name.to_string(),
            global_type,
            is_mutable,
            initializer: Some(initializer),
            span: SourceSpan::default(),
        })
    }

    /// Create implicit main function from top-level statements
    fn create_implicit_main(&mut self, statements: &[&Statement]) -> OvieResult<HirFunction> {
        let mut hir_statements = Vec::new();
        
        for statement in statements {
            let hir_stmt = self.transform_statement(statement)?;
            hir_statements.push(hir_stmt);
        }

        Ok(HirFunction {
            id: self.next_id(),
            name: "main".to_string(),
            parameters: Vec::new(),
            return_type: HirType::Unit,
            body: HirBlock {
                statements: hir_statements,
                span: SourceSpan::default(),
            },
            span: SourceSpan::default(),
            is_main: true,
        })
    }

    /// Transform a block of statements
    fn transform_block(&mut self, statements: &[Statement]) -> OvieResult<HirBlock> {
        let mut hir_statements = Vec::new();
        
        for statement in statements {
            let hir_stmt = self.transform_statement(statement)?;
            hir_statements.push(hir_stmt);
        }

        Ok(HirBlock {
            statements: hir_statements,
            span: SourceSpan::default(),
        })
    }

    /// Transform a statement
    fn transform_statement(&mut self, statement: &Statement) -> OvieResult<HirStatement> {
        let kind = match statement {
            Statement::Assignment { identifier, value, mutable } => {
                let hir_value = self.transform_expression(value)?;
                let var_type = hir_value.expr_type.clone();
                
                // Add to symbol table
                self.symbol_table.insert(identifier.clone(), SymbolInfo {
                    symbol_type: var_type.clone(),
                    is_mutable: *mutable,
                    is_function: false,
                    span: SourceSpan::default(),
                })?;

                HirStatementKind::Local {
                    name: identifier.clone(),
                    var_type,
                    is_mutable: *mutable,
                    initializer: Some(hir_value),
                }
            }
            Statement::Print { expression } => {
                let hir_expr = self.transform_expression(expression)?;
                HirStatementKind::Print(hir_expr)
            }
            Statement::Return { value } => {
                let hir_value = if let Some(expr) = value {
                    Some(self.transform_expression(expr)?)
                } else {
                    None
                };
                HirStatementKind::Return(hir_value)
            }
            Statement::Expression { expression } => {
                let hir_expr = self.transform_expression(expression)?;
                HirStatementKind::Expression(hir_expr)
            }
            Statement::If { condition, then_block, else_block } => {
                let hir_condition = self.transform_expression(condition)?;
                let hir_then = self.transform_block(then_block)?;
                let hir_else = if let Some(else_stmts) = else_block {
                    Some(self.transform_block(else_stmts)?)
                } else {
                    None
                };
                
                HirStatementKind::If {
                    condition: hir_condition,
                    then_block: hir_then,
                    else_block: hir_else,
                }
            }
            Statement::While { condition, body } => {
                let hir_condition = self.transform_expression(condition)?;
                let hir_body = self.transform_block(body)?;
                
                HirStatementKind::While {
                    condition: hir_condition,
                    body: hir_body,
                }
            }
            Statement::For { identifier, iterable, body } => {
                let hir_iterable = self.transform_expression(iterable)?;
                let hir_body = self.transform_block(body)?;
                
                HirStatementKind::For {
                    variable: identifier.clone(),
                    iterable: hir_iterable,
                    body: hir_body,
                }
            }
            _ => {
                return Err(OvieError::SemanticError {
                    message: "Unsupported statement type in HIR transformation".to_string(),
                });
            }
        };

        Ok(HirStatement {
            id: self.next_id(),
            kind,
            span: SourceSpan::default(),
        })
    }

    /// Transform an expression
    fn transform_expression(&mut self, expression: &Expression) -> OvieResult<HirExpression> {
        let (kind, expr_type) = match expression {
            Expression::Literal(literal) => {
                let (hir_literal, hir_type) = match literal {
                    Literal::String(s) => (HirLiteral::String(s.clone()), HirType::String),
                    Literal::Number(n) => (HirLiteral::Number(*n), HirType::Number),
                    Literal::Boolean(b) => (HirLiteral::Boolean(*b), HirType::Boolean),
                };
                (HirExpressionKind::Literal(hir_literal), hir_type)
            }
            Expression::Identifier(name) => {
                let symbol_info = self.symbol_table.lookup(name)?;
                (HirExpressionKind::Variable(name.clone()), symbol_info.symbol_type)
            }
            Expression::Binary { left, operator, right } => {
                let hir_left = self.transform_expression(left)?;
                let hir_right = self.transform_expression(right)?;
                let hir_op = self.transform_binary_op(operator);
                
                // Type checking for binary operations
                let result_type = self.check_binary_op_type(&hir_left.expr_type, &hir_op, &hir_right.expr_type)?;
                
                (HirExpressionKind::Binary {
                    left: Box::new(hir_left),
                    op: hir_op,
                    right: Box::new(hir_right),
                }, result_type)
            }
            Expression::Unary { operator, operand } => {
                let hir_operand = self.transform_expression(operand)?;
                let hir_op = self.transform_unary_op(operator);
                
                // Type checking for unary operations
                let result_type = self.check_unary_op_type(&hir_op, &hir_operand.expr_type)?;
                
                (HirExpressionKind::Unary {
                    op: hir_op,
                    operand: Box::new(hir_operand),
                }, result_type)
            }
            Expression::Call { function, arguments } => {
                let mut hir_args = Vec::new();
                for arg in arguments {
                    hir_args.push(self.transform_expression(arg)?);
                }
                
                // Look up function type
                let func_info = self.symbol_table.lookup(function)?;
                let return_type = if let HirType::Function { return_type, .. } = &func_info.symbol_type {
                    (**return_type).clone()
                } else {
                    HirType::Infer(self.next_id())
                };
                
                (HirExpressionKind::Call {
                    function: function.clone(),
                    arguments: hir_args,
                }, return_type)
            }
            Expression::FieldAccess { object, field } => {
                let hir_object = self.transform_expression(object)?;
                let field_type = self.get_field_type(&hir_object.expr_type, field)?;
                
                (HirExpressionKind::FieldAccess {
                    object: Box::new(hir_object),
                    field: field.clone(),
                }, field_type)
            }
            Expression::StructInstantiation { struct_name, fields } => {
                let mut hir_fields = Vec::new();
                for field_init in fields {
                    let hir_value = self.transform_expression(&field_init.value)?;
                    hir_fields.push(HirFieldInit {
                        name: field_init.name.clone(),
                        value: hir_value,
                        span: SourceSpan::default(),
                    });
                }
                
                (HirExpressionKind::StructInit {
                    struct_name: struct_name.clone(),
                    fields: hir_fields,
                }, HirType::Struct(struct_name.clone()))
            }
            Expression::Range { start, end } => {
                let hir_start = self.transform_expression(start)?;
                let hir_end = self.transform_expression(end)?;
                
                // Both start and end should be the same type
                let range_type = HirType::Range(Box::new(hir_start.expr_type.clone()));
                
                (HirExpressionKind::Range {
                    start: Box::new(hir_start),
                    end: Box::new(hir_end),
                }, range_type)
            }
        };

        Ok(HirExpression {
            id: self.next_id(),
            kind,
            expr_type,
            span: SourceSpan::default(),
        })
    }

    /// Transform binary operator
    fn transform_binary_op(&self, op: &BinaryOperator) -> HirBinaryOp {
        match op {
            BinaryOperator::Add => HirBinaryOp::Add,
            BinaryOperator::Subtract => HirBinaryOp::Sub,
            BinaryOperator::Multiply => HirBinaryOp::Mul,
            BinaryOperator::Divide => HirBinaryOp::Div,
            BinaryOperator::Modulo => HirBinaryOp::Mod,
            BinaryOperator::Equal => HirBinaryOp::Eq,
            BinaryOperator::NotEqual => HirBinaryOp::Ne,
            BinaryOperator::Less => HirBinaryOp::Lt,
            BinaryOperator::LessEqual => HirBinaryOp::Le,
            BinaryOperator::Greater => HirBinaryOp::Gt,
            BinaryOperator::GreaterEqual => HirBinaryOp::Ge,
            BinaryOperator::And => HirBinaryOp::And,
            BinaryOperator::Or => HirBinaryOp::Or,
        }
    }

    /// Transform unary operator
    fn transform_unary_op(&self, op: &UnaryOperator) -> HirUnaryOp {
        match op {
            UnaryOperator::Not => HirUnaryOp::Not,
            UnaryOperator::Negate => HirUnaryOp::Neg,
        }
    }

    /// Check binary operation type compatibility
    fn check_binary_op_type(&self, left: &HirType, op: &HirBinaryOp, right: &HirType) -> OvieResult<HirType> {
        match (left, op, right) {
            // Arithmetic operations
            (HirType::Number, HirBinaryOp::Add | HirBinaryOp::Sub | HirBinaryOp::Mul | HirBinaryOp::Div | HirBinaryOp::Mod, HirType::Number) => {
                Ok(HirType::Number)
            }
            // String concatenation
            (HirType::String, HirBinaryOp::Add, HirType::String) => {
                Ok(HirType::String)
            }
            // Comparison operations
            (HirType::Number, HirBinaryOp::Lt | HirBinaryOp::Le | HirBinaryOp::Gt | HirBinaryOp::Ge, HirType::Number) => {
                Ok(HirType::Boolean)
            }
            // Equality operations
            (left_ty, HirBinaryOp::Eq | HirBinaryOp::Ne, right_ty) if left_ty == right_ty => {
                Ok(HirType::Boolean)
            }
            // Logical operations
            (HirType::Boolean, HirBinaryOp::And | HirBinaryOp::Or, HirType::Boolean) => {
                Ok(HirType::Boolean)
            }
            _ => {
                Err(OvieError::TypeError {
                    message: format!("Type mismatch in binary operation: {:?} {:?} {:?}", left, op, right),
                })
            }
        }
    }

    /// Check unary operation type compatibility
    fn check_unary_op_type(&self, op: &HirUnaryOp, operand: &HirType) -> OvieResult<HirType> {
        match (op, operand) {
            (HirUnaryOp::Not, HirType::Boolean) => Ok(HirType::Boolean),
            (HirUnaryOp::Neg, HirType::Number) => Ok(HirType::Number),
            _ => {
                Err(OvieError::TypeError {
                    message: format!("Type mismatch in unary operation: {:?} {:?}", op, operand),
                })
            }
        }
    }

    /// Get field type from struct type
    fn get_field_type(&self, struct_type: &HirType, field_name: &str) -> OvieResult<HirType> {
        if let HirType::Struct(struct_name) = struct_type {
            if let Some(TypeInfo::Struct { fields }) = self.type_table.types.get(struct_name) {
                if let Some(field_type) = fields.get(field_name) {
                    Ok(field_type.clone())
                } else {
                    Err(OvieError::TypeError {
                        message: format!("Field '{}' not found in struct '{}'", field_name, struct_name),
                    })
                }
            } else {
                Err(OvieError::TypeError {
                    message: format!("Struct '{}' not found", struct_name),
                })
            }
        } else {
            Err(OvieError::TypeError {
                message: format!("Cannot access field '{}' on non-struct type {:?}", field_name, struct_type),
            })
        }
    }

    /// Resolve type name to HIR type
    fn resolve_type(&self, type_name: &str) -> OvieResult<HirType> {
        match type_name {
            "String" => Ok(HirType::String),
            "Number" => Ok(HirType::Number),
            "Boolean" => Ok(HirType::Boolean),
            "Unit" => Ok(HirType::Unit),
            _ => {
                // Check if it's a user-defined type
                if self.type_table.types.contains_key(type_name) {
                    Ok(HirType::Struct(type_name.to_string()))
                } else {
                    Err(OvieError::TypeError {
                        message: format!("Unknown type: {}", type_name),
                    })
                }
            }
        }
    }

    /// Register struct type in type table
    fn register_struct_type(&mut self, name: &str, fields: &[crate::ast::StructField]) -> OvieResult<()> {
        let mut field_types = HashMap::new();
        for field in fields {
            let field_type = self.resolve_type(&field.type_annotation)?;
            field_types.insert(field.name.clone(), field_type);
        }
        
        self.type_table.types.insert(name.to_string(), TypeInfo::Struct {
            fields: field_types,
        });
        
        Ok(())
    }

    /// Register enum type in type table
    fn register_enum_type(&mut self, name: &str, variants: &[crate::ast::EnumVariant]) -> OvieResult<()> {
        let mut variant_types = HashMap::new();
        for variant in variants {
            let data_type = if let Some(ref type_name) = variant.data_type {
                Some(self.resolve_type(type_name)?)
            } else {
                None
            };
            variant_types.insert(variant.name.clone(), data_type);
        }
        
        self.type_table.types.insert(name.to_string(), TypeInfo::Enum {
            variants: variant_types,
        });
        
        Ok(())
    }

    /// Register function in symbol table
    fn register_function(&mut self, name: &str, parameters: &[String]) -> OvieResult<()> {
        let param_types = vec![HirType::Infer(self.next_id()); parameters.len()];
        let return_type = HirType::Infer(self.next_id());
        
        let func_type = HirType::Function {
            params: param_types,
            return_type: Box::new(return_type),
        };
        
        self.symbol_table.insert(name.to_string(), SymbolInfo {
            symbol_type: func_type,
            is_mutable: false,
            is_function: true,
            span: SourceSpan::default(),
        })?;
        
        Ok(())
    }

    /// Validate struct definition
    fn validate_struct_definition(&self, name: &str, fields: &[crate::ast::StructField]) -> OvieResult<()> {
        // Check for duplicate field names
        let mut field_names = std::collections::HashSet::new();
        for field in fields {
            if !field_names.insert(&field.name) {
                return Err(OvieError::SemanticError {
                    message: format!("Duplicate field '{}' in struct '{}'", field.name, name),
                });
            }
        }

        // Validate field types exist
        for field in fields {
            self.validate_type_name(&field.type_annotation)?;
        }

        // Check for reserved names
        if matches!(name, "String" | "Number" | "Boolean" | "Unit") {
            return Err(OvieError::SemanticError {
                message: format!("Cannot use reserved type name '{}' for struct", name),
            });
        }

        Ok(())
    }

    /// Validate enum definition
    fn validate_enum_definition(&self, name: &str, variants: &[crate::ast::EnumVariant]) -> OvieResult<()> {
        // Check for duplicate variant names
        let mut variant_names = std::collections::HashSet::new();
        for variant in variants {
            if !variant_names.insert(&variant.name) {
                return Err(OvieError::SemanticError {
                    message: format!("Duplicate variant '{}' in enum '{}'", variant.name, name),
                });
            }
        }

        // Validate variant data types exist
        for variant in variants {
            if let Some(ref type_name) = variant.data_type {
                self.validate_type_name(type_name)?;
            }
        }

        // Check for reserved names
        if matches!(name, "String" | "Number" | "Boolean" | "Unit") {
            return Err(OvieError::SemanticError {
                message: format!("Cannot use reserved type name '{}' for enum", name),
            });
        }

        // Ensure at least one variant
        if variants.is_empty() {
            return Err(OvieError::SemanticError {
                message: format!("Enum '{}' must have at least one variant", name),
            });
        }

        Ok(())
    }

    /// Validate function signature
    fn validate_function_signature(&self, name: &str, parameters: &[String]) -> OvieResult<()> {
        // Check for duplicate parameter names
        let mut param_names = std::collections::HashSet::new();
        for param in parameters {
            if !param_names.insert(param) {
                return Err(OvieError::SemanticError {
                    message: format!("Duplicate parameter '{}' in function '{}'", param, name),
                });
            }
        }

        // Check for reserved function names
        if matches!(name, "print" | "to_string") {
            return Err(OvieError::SemanticError {
                message: format!("Cannot redefine built-in function '{}'", name),
            });
        }

        Ok(())
    }

    /// Validate type name exists
    fn validate_type_name(&self, type_name: &str) -> OvieResult<()> {
        match type_name {
            "String" | "Number" | "Boolean" | "Unit" => Ok(()),
            _ => {
                if self.type_table.types.contains_key(type_name) {
                    Ok(())
                } else {
                    Err(OvieError::TypeError {
                        message: format!("Unknown type: {}", type_name),
                    })
                }
            }
        }
    }

    /// Perform type inference on HIR items
    fn perform_type_inference(&mut self, items: &mut [HirItem]) -> OvieResult<()> {
        // Simple type inference - replace Infer types with concrete types where possible
        for item in items {
            match item {
                HirItem::Function(func) => {
                    self.infer_function_types(func)?;
                }
                HirItem::Global(global) => {
                    if let Some(ref init) = global.initializer {
                        global.global_type = init.expr_type.clone();
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Infer types for a function
    fn infer_function_types(&mut self, func: &mut HirFunction) -> OvieResult<()> {
        // Infer return type from return statements
        let mut return_type = HirType::Unit;
        self.infer_block_return_type(&func.body, &mut return_type)?;
        
        if matches!(return_type, HirType::Infer(_)) {
            return_type = HirType::Unit;
        }
        
        func.return_type = return_type;
        Ok(())
    }

    /// Infer return type from block
    fn infer_block_return_type(&self, block: &HirBlock, return_type: &mut HirType) -> OvieResult<()> {
        for stmt in &block.statements {
            if let HirStatementKind::Return(Some(ref expr)) = stmt.kind {
                if matches!(*return_type, HirType::Unit | HirType::Infer(_)) {
                    *return_type = expr.expr_type.clone();
                } else if *return_type != expr.expr_type {
                    return Err(OvieError::TypeError {
                        message: "Inconsistent return types in function".to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    /// Validate program semantics
    fn validate_program_semantics(&self, items: &[HirItem]) -> OvieResult<()> {
        // Check for main function if needed
        let has_main = items.iter().any(|item| {
            matches!(item, HirItem::Function(func) if func.is_main)
        });

        // Validate function calls reference existing functions
        for item in items {
            if let HirItem::Function(func) = item {
                self.validate_function_calls(&func.body)?;
            }
        }

        // Validate struct field accesses
        for item in items {
            if let HirItem::Function(func) = item {
                self.validate_field_accesses(&func.body)?;
            }
        }

        Ok(())
    }

    /// Validate function calls in a block
    fn validate_function_calls(&self, block: &HirBlock) -> OvieResult<()> {
        for stmt in &block.statements {
            match &stmt.kind {
                HirStatementKind::Expression(expr) => {
                    self.validate_expression_calls(expr)?;
                }
                HirStatementKind::Print(expr) => {
                    self.validate_expression_calls(expr)?;
                }
                HirStatementKind::Return(Some(expr)) => {
                    self.validate_expression_calls(expr)?;
                }
                HirStatementKind::Local { initializer: Some(expr), .. } => {
                    self.validate_expression_calls(expr)?;
                }
                HirStatementKind::Assign { value: expr, .. } => {
                    self.validate_expression_calls(expr)?;
                }
                HirStatementKind::If { condition, then_block, else_block } => {
                    self.validate_expression_calls(condition)?;
                    self.validate_function_calls(then_block)?;
                    if let Some(else_blk) = else_block {
                        self.validate_function_calls(else_blk)?;
                    }
                }
                HirStatementKind::While { condition, body } => {
                    self.validate_expression_calls(condition)?;
                    self.validate_function_calls(body)?;
                }
                HirStatementKind::For { iterable, body, .. } => {
                    self.validate_expression_calls(iterable)?;
                    self.validate_function_calls(body)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Validate function calls in an expression
    fn validate_expression_calls(&self, expr: &HirExpression) -> OvieResult<()> {
        match &expr.kind {
            HirExpressionKind::Call { function, arguments } => {
                // Check if function exists
                if let Err(_) = self.symbol_table.lookup(function) {
                    return Err(OvieError::SemanticError {
                        message: format!("Function '{}' not found", function),
                    });
                }
                
                // Validate arguments
                for arg in arguments {
                    self.validate_expression_calls(arg)?;
                }
            }
            HirExpressionKind::Binary { left, right, .. } => {
                self.validate_expression_calls(left)?;
                self.validate_expression_calls(right)?;
            }
            HirExpressionKind::Unary { operand, .. } => {
                self.validate_expression_calls(operand)?;
            }
            HirExpressionKind::FieldAccess { object, .. } => {
                self.validate_expression_calls(object)?;
            }
            HirExpressionKind::StructInit { fields, .. } => {
                for field in fields {
                    self.validate_expression_calls(&field.value)?;
                }
            }
            HirExpressionKind::Range { start, end } => {
                self.validate_expression_calls(start)?;
                self.validate_expression_calls(end)?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Validate field accesses in a block
    fn validate_field_accesses(&self, block: &HirBlock) -> OvieResult<()> {
        for stmt in &block.statements {
            match &stmt.kind {
                HirStatementKind::Expression(expr) => {
                    self.validate_expression_field_accesses(expr)?;
                }
                HirStatementKind::Print(expr) => {
                    self.validate_expression_field_accesses(expr)?;
                }
                HirStatementKind::Return(Some(expr)) => {
                    self.validate_expression_field_accesses(expr)?;
                }
                HirStatementKind::Local { initializer: Some(expr), .. } => {
                    self.validate_expression_field_accesses(expr)?;
                }
                HirStatementKind::Assign { value: expr, .. } => {
                    self.validate_expression_field_accesses(expr)?;
                }
                HirStatementKind::If { condition, then_block, else_block } => {
                    self.validate_expression_field_accesses(condition)?;
                    self.validate_field_accesses(then_block)?;
                    if let Some(else_blk) = else_block {
                        self.validate_field_accesses(else_blk)?;
                    }
                }
                HirStatementKind::While { condition, body } => {
                    self.validate_expression_field_accesses(condition)?;
                    self.validate_field_accesses(body)?;
                }
                HirStatementKind::For { iterable, body, .. } => {
                    self.validate_expression_field_accesses(iterable)?;
                    self.validate_field_accesses(body)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Validate field accesses in an expression
    fn validate_expression_field_accesses(&self, expr: &HirExpression) -> OvieResult<()> {
        match &expr.kind {
            HirExpressionKind::FieldAccess { object, field } => {
                self.validate_expression_field_accesses(object)?;
                // Field access validation is already done in get_field_type during transformation
            }
            HirExpressionKind::Binary { left, right, .. } => {
                self.validate_expression_field_accesses(left)?;
                self.validate_expression_field_accesses(right)?;
            }
            HirExpressionKind::Unary { operand, .. } => {
                self.validate_expression_field_accesses(operand)?;
            }
            HirExpressionKind::Call { arguments, .. } => {
                for arg in arguments {
                    self.validate_expression_field_accesses(arg)?;
                }
            }
            HirExpressionKind::StructInit { fields, .. } => {
                for field in fields {
                    self.validate_expression_field_accesses(&field.value)?;
                }
            }
            HirExpressionKind::Range { start, end } => {
                self.validate_expression_field_accesses(start)?;
                self.validate_expression_field_accesses(end)?;
            }
            _ => {}
        }
        Ok(())
    }
}

impl Default for HirBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolTable {
    /// Create a new symbol table
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new()],
            current_scope: 0,
        }
    }

    /// Enter a new scope
    pub fn enter_scope(&mut self) {
        let new_scope = Scope {
            symbols: HashMap::new(),
            parent: Some(self.current_scope),
        };
        self.scopes.push(new_scope);
        self.current_scope = self.scopes.len() - 1;
    }

    /// Exit current scope
    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.scopes[self.current_scope].parent {
            self.current_scope = parent;
        }
    }

    /// Insert symbol into current scope
    pub fn insert(&mut self, name: Symbol, info: SymbolInfo) -> OvieResult<()> {
        if self.scopes[self.current_scope].symbols.contains_key(&name) {
            return Err(OvieError::SemanticError {
                message: format!("Symbol '{}' already defined in current scope", name),
            });
        }
        
        self.scopes[self.current_scope].symbols.insert(name, info);
        Ok(())
    }

    /// Look up symbol in symbol table
    pub fn lookup(&self, name: &str) -> OvieResult<SymbolInfo> {
        let mut current = self.current_scope;
        
        loop {
            if let Some(info) = self.scopes[current].symbols.get(name) {
                return Ok(info.clone());
            }
            
            if let Some(parent) = self.scopes[current].parent {
                current = parent;
            } else {
                break;
            }
        }
        
        Err(OvieError::SemanticError {
            message: format!("Symbol '{}' not found", name),
        })
    }
}

impl Scope {
    fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            parent: None,
        }
    }
}

impl TypeTable {
    /// Create a new type table
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
        }
    }
}

impl HirProgram {
    /// Serialize HIR program to JSON
    pub fn to_json(&self) -> OvieResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| OvieError::IrError { message: format!("HIR serialization error: {}", e) })
    }

    /// Deserialize HIR program from JSON
    pub fn from_json(json: &str) -> OvieResult<Self> {
        serde_json::from_str(json)
            .map_err(|e| OvieError::IrError { message: format!("HIR deserialization error: {}", e) })
    }

    /// Validate HIR program
    pub fn validate(&self) -> OvieResult<()> {
        // Check that we have a main function if needed
        let has_main = self.items.iter().any(|item| {
            matches!(item, HirItem::Function(func) if func.is_main)
        });

        if !has_main && self.metadata.has_main_function {
            return Err(OvieError::SemanticError {
                message: "Main function not found".to_string(),
            });
        }

        // Additional validation can be added here
        Ok(())
    }

    /// Validate HIR invariants according to Stage 2.1 compiler invariants
    /// 
    /// HIR Invariants (from docs/compiler_invariants.md):
    /// - All identifiers are resolved to symbols
    /// - No unresolved names exist
    /// - Every expression has a known type
    /// - Type inference is complete
    /// - Symbol table is fully populated
    /// - Semantic errors have been caught
    /// - Function signatures are resolved
    /// - Struct/enum definitions are complete
    pub fn validate_invariants(&self) -> Result<(), crate::ast::InvariantError> {
        // Check that all items have valid invariants
        for item in &self.items {
            self.validate_item_invariants(item)?;
        }

        // Check symbol table is populated
        if self.symbol_table.scopes.is_empty() {
            return Err(crate::ast::InvariantError {
                message: "HIR must have populated symbol table".to_string(),
                location: Some("symbol_table".to_string()),
            });
        }

        // Check that no error types remain (type inference should be complete)
        self.validate_no_error_types()?;

        Ok(())
    }

    fn validate_item_invariants(&self, item: &HirItem) -> Result<(), crate::ast::InvariantError> {
        match item {
            HirItem::Function(func) => {
                // Function must have resolved return type
                if matches!(func.return_type, HirType::Error | HirType::Infer(_)) {
                    return Err(crate::ast::InvariantError {
                        message: format!("Function '{}' has unresolved return type", func.name),
                        location: Some(format!("function:{}", func.name)),
                    });
                }

                // All parameters must have resolved types
                for param in &func.parameters {
                    if matches!(param.param_type, HirType::Error | HirType::Infer(_)) {
                        return Err(crate::ast::InvariantError {
                            message: format!("Parameter '{}' has unresolved type", param.name),
                            location: Some(format!("function:{}:param:{}", func.name, param.name)),
                        });
                    }
                }

                // Validate function body
                self.validate_block_invariants(&func.body)?;
            }
            HirItem::Struct(struct_def) => {
                // All fields must have resolved types
                for field in &struct_def.fields {
                    if matches!(field.field_type, HirType::Error | HirType::Infer(_)) {
                        return Err(crate::ast::InvariantError {
                            message: format!("Struct field '{}' has unresolved type", field.name),
                            location: Some(format!("struct:{}:field:{}", struct_def.name, field.name)),
                        });
                    }
                }
            }
            HirItem::Enum(enum_def) => {
                // All variants must have resolved types (if they have data)
                for variant in &enum_def.variants {
                    if let Some(ref data_type) = variant.data_type {
                        if matches!(data_type, HirType::Error | HirType::Infer(_)) {
                            return Err(crate::ast::InvariantError {
                                message: format!("Enum variant '{}' has unresolved data type", variant.name),
                                location: Some(format!("enum:{}:variant:{}", enum_def.name, variant.name)),
                            });
                        }
                    }
                }
            }
            HirItem::Global(global) => {
                // Global must have resolved type
                if matches!(global.global_type, HirType::Error | HirType::Infer(_)) {
                    return Err(crate::ast::InvariantError {
                        message: format!("Global '{}' has unresolved type", global.name),
                        location: Some(format!("global:{}", global.name)),
                    });
                }

                // If there's an initializer, validate it
                if let Some(ref init) = global.initializer {
                    self.validate_expression_invariants(init)?;
                }
            }
        }
        Ok(())
    }

    fn validate_block_invariants(&self, block: &HirBlock) -> Result<(), crate::ast::InvariantError> {
        for stmt in &block.statements {
            self.validate_statement_invariants(stmt)?;
        }
        Ok(())
    }

    fn validate_statement_invariants(&self, stmt: &HirStatement) -> Result<(), crate::ast::InvariantError> {
        match &stmt.kind {
            HirStatementKind::Local { var_type, initializer, .. } => {
                if matches!(var_type, HirType::Error | HirType::Infer(_)) {
                    return Err(crate::ast::InvariantError {
                        message: "Local variable has unresolved type".to_string(),
                        location: Some(format!("statement:{}", stmt.id)),
                    });
                }
                if let Some(init) = initializer {
                    self.validate_expression_invariants(init)?;
                }
            }
            HirStatementKind::Assign { target, value } => {
                self.validate_place_invariants(target)?;
                self.validate_expression_invariants(value)?;
            }
            HirStatementKind::Expression(expr) => {
                self.validate_expression_invariants(expr)?;
            }
            HirStatementKind::Print(expr) => {
                self.validate_expression_invariants(expr)?;
            }
            HirStatementKind::Return(expr_opt) => {
                if let Some(expr) = expr_opt {
                    self.validate_expression_invariants(expr)?;
                }
            }
            HirStatementKind::If { condition, then_block, else_block } => {
                self.validate_expression_invariants(condition)?;
                self.validate_block_invariants(then_block)?;
                if let Some(else_blk) = else_block {
                    self.validate_block_invariants(else_blk)?;
                }
            }
            HirStatementKind::While { condition, body } => {
                self.validate_expression_invariants(condition)?;
                self.validate_block_invariants(body)?;
            }
            HirStatementKind::For { iterable, body, .. } => {
                self.validate_expression_invariants(iterable)?;
                self.validate_block_invariants(body)?;
            }
        }
        Ok(())
    }

    fn validate_expression_invariants(&self, expr: &HirExpression) -> Result<(), crate::ast::InvariantError> {
        // Every expression must have a known type (no Error or Infer types)
        if matches!(expr.expr_type, HirType::Error | HirType::Infer(_)) {
            return Err(crate::ast::InvariantError {
                message: "Expression has unresolved type".to_string(),
                location: Some(format!("expression:{}", expr.id)),
            });
        }

        match &expr.kind {
            HirExpressionKind::Literal(_) => {
                // Literals are always valid
            }
            HirExpressionKind::Variable(symbol) => {
                // Variable must be resolved (this is checked by having a Symbol type)
                // The fact that we have a Symbol means it was resolved
            }
            HirExpressionKind::Binary { left, right, .. } => {
                self.validate_expression_invariants(left)?;
                self.validate_expression_invariants(right)?;
            }
            HirExpressionKind::Unary { operand, .. } => {
                self.validate_expression_invariants(operand)?;
            }
            HirExpressionKind::Call { arguments, .. } => {
                for arg in arguments {
                    self.validate_expression_invariants(arg)?;
                }
            }
            HirExpressionKind::FieldAccess { object, .. } => {
                self.validate_expression_invariants(object)?;
            }
            HirExpressionKind::StructInit { fields, .. } => {
                for field in fields {
                    self.validate_expression_invariants(&field.value)?;
                }
            }
            HirExpressionKind::Range { start, end } => {
                self.validate_expression_invariants(start)?;
                self.validate_expression_invariants(end)?;
            }
        }
        Ok(())
    }

    fn validate_place_invariants(&self, place: &HirPlace) -> Result<(), crate::ast::InvariantError> {
        if matches!(place.place_type, HirType::Error | HirType::Infer(_)) {
            return Err(crate::ast::InvariantError {
                message: "Place has unresolved type".to_string(),
                location: Some("place".to_string()),
            });
        }

        match &place.kind {
            HirPlaceKind::Local(_) => {
                // Local places are valid if they have resolved types
            }
            HirPlaceKind::Field { object, .. } => {
                self.validate_place_invariants(object)?;
            }
        }
        Ok(())
    }

    fn validate_no_error_types(&self) -> Result<(), crate::ast::InvariantError> {
        // This is a comprehensive check that no Error or Infer types remain
        // The individual checks above should catch most cases, but this is a final safety net
        
        for item in &self.items {
            match item {
                HirItem::Function(func) => {
                    if self.type_contains_error(&func.return_type) {
                        return Err(crate::ast::InvariantError {
                            message: format!("Function '{}' contains unresolved types", func.name),
                            location: Some(format!("function:{}", func.name)),
                        });
                    }
                }
                HirItem::Struct(struct_def) => {
                    for field in &struct_def.fields {
                        if self.type_contains_error(&field.field_type) {
                            return Err(crate::ast::InvariantError {
                                message: format!("Struct '{}' contains unresolved types", struct_def.name),
                                location: Some(format!("struct:{}", struct_def.name)),
                            });
                        }
                    }
                }
                HirItem::Enum(enum_def) => {
                    for variant in &enum_def.variants {
                        if let Some(ref data_type) = variant.data_type {
                            if self.type_contains_error(data_type) {
                                return Err(crate::ast::InvariantError {
                                    message: format!("Enum '{}' contains unresolved types", enum_def.name),
                                    location: Some(format!("enum:{}", enum_def.name)),
                                });
                            }
                        }
                    }
                }
                HirItem::Global(global) => {
                    if self.type_contains_error(&global.global_type) {
                        return Err(crate::ast::InvariantError {
                            message: format!("Global '{}' contains unresolved types", global.name),
                            location: Some(format!("global:{}", global.name)),
                        });
                    }
                }
            }
        }
        Ok(())
    }

    fn type_contains_error(&self, hir_type: &HirType) -> bool {
        match hir_type {
            HirType::Error | HirType::Infer(_) => true,
            HirType::Function { params, return_type } => {
                params.iter().any(|p| self.type_contains_error(p)) || 
                self.type_contains_error(return_type)
            }
            HirType::Range(inner) => self.type_contains_error(inner),
            _ => false,
        }
    }

    /// Generate human-readable HIR report
    pub fn generate_hir_report(&self) -> OvieResult<String> {
        let mut report = String::new();
        
        report.push_str("=== HIR Program Analysis Report ===\n\n");
        
        // Program overview
        report.push_str(&format!("Source File: {}\n", self.metadata.source_file));
        report.push_str(&format!("Compiler Version: {}\n", self.metadata.compiler_version));
        report.push_str(&format!("Has Main Function: {}\n", self.metadata.has_main_function));
        report.push_str(&format!("Errors: {}\n", self.metadata.error_count));
        report.push_str(&format!("Warnings: {}\n\n", self.metadata.warning_count));
        
        // Items summary
        let mut function_count = 0;
        let mut struct_count = 0;
        let mut enum_count = 0;
        let mut global_count = 0;
        
        for item in &self.items {
            match item {
                HirItem::Function(_) => function_count += 1,
                HirItem::Struct(_) => struct_count += 1,
                HirItem::Enum(_) => enum_count += 1,
                HirItem::Global(_) => global_count += 1,
            }
        }
        
        report.push_str(&format!("Functions: {}\n", function_count));
        report.push_str(&format!("Structs: {}\n", struct_count));
        report.push_str(&format!("Enums: {}\n", enum_count));
        report.push_str(&format!("Globals: {}\n\n", global_count));
        
        // Symbol table summary
        report.push_str("=== Symbol Table ===\n");
        report.push_str(&format!("Scopes: {}\n\n", self.symbol_table.scopes.len()));
        
        // Type table summary
        report.push_str("=== Type Definitions ===\n");
        for (type_name, type_info) in &self.type_table.types {
            match type_info {
                crate::hir::TypeInfo::Struct { fields } => {
                    report.push_str(&format!("Struct {}: {} fields\n", type_name, fields.len()));
                }
                crate::hir::TypeInfo::Enum { variants } => {
                    report.push_str(&format!("Enum {}: {} variants\n", type_name, variants.len()));
                }
            }
        }
        report.push_str("\n");
        
        // Item details
        report.push_str("=== Item Details ===\n\n");
        for item in &self.items {
            match item {
                HirItem::Function(func) => {
                    report.push_str(&format!("Function: {} (ID: {})\n", func.name, func.id));
                    report.push_str(&format!("  Is Main: {}\n", func.is_main));
                    report.push_str(&format!("  Parameters: {}\n", func.parameters.len()));
                    report.push_str(&format!("  Return Type: {:?}\n", func.return_type));
                    report.push_str(&format!("  Statements: {}\n\n", func.body.statements.len()));
                }
                HirItem::Struct(s) => {
                    report.push_str(&format!("Struct: {} (ID: {})\n", s.name, s.id));
                    report.push_str(&format!("  Fields: {}\n", s.fields.len()));
                    for field in &s.fields {
                        report.push_str(&format!("    {}: {:?}\n", field.name, field.field_type));
                    }
                    report.push_str("\n");
                }
                HirItem::Enum(e) => {
                    report.push_str(&format!("Enum: {} (ID: {})\n", e.name, e.id));
                    report.push_str(&format!("  Variants: {}\n", e.variants.len()));
                    for variant in &e.variants {
                        let data_info = variant.data_type.as_ref()
                            .map(|t| format!("{:?}", t))
                            .unwrap_or_else(|| "()".to_string());
                        report.push_str(&format!("    {}: {}\n", variant.name, data_info));
                    }
                    report.push_str("\n");
                }
                HirItem::Global(g) => {
                    report.push_str(&format!("Global: {} (ID: {})\n", g.name, g.id));
                    report.push_str(&format!("  Type: {:?}\n", g.global_type));
                    report.push_str(&format!("  Mutable: {}\n", g.is_mutable));
                    report.push_str(&format!("  Has Initializer: {}\n\n", g.initializer.is_some()));
                }
            }
        }
        
        Ok(report)
    }
}