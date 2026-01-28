//! Intermediate Representation (IR) module for the Ovie compiler

use crate::error::{OvieError, OvieResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unique identifier for IR values
pub type ValueId = u32;

/// Unique identifier for basic blocks
pub type BlockId = u32;

/// Unique identifier for functions
pub type FunctionId = u32;

/// The complete IR program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub functions: HashMap<FunctionId, Function>,
    pub globals: HashMap<String, Global>,
    pub metadata: Metadata,
    pub entry_point: Option<FunctionId>,
}

/// Function in IR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub id: FunctionId,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: IrType,
    pub basic_blocks: HashMap<BlockId, BasicBlock>,
    pub entry_block: BlockId,
    pub local_variables: HashMap<String, ValueId>,
}

/// Function parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: IrType,
    pub value_id: ValueId,
}

/// Global variable or constant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Global {
    pub name: String,
    pub global_type: IrType,
    pub is_mutable: bool,
    pub initializer: Option<Constant>,
}

/// Basic block containing instructions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicBlock {
    pub id: BlockId,
    pub label: String,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
}

/// IR instruction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    pub id: ValueId,
    pub opcode: Opcode,
    pub operands: Vec<Value>,
    pub result_type: IrType,
}

/// IR opcodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Opcode {
    // Arithmetic
    Add, Sub, Mul, Div, Mod,
    
    // Comparison
    Eq, Ne, Lt, Le, Gt, Ge,
    
    // Logical
    And, Or, Not,
    
    // Memory
    Load, Store, Alloca,
    
    // Function calls
    Call,
    
    // Type conversions
    Cast,
    
    // String operations
    StringConcat,
    
    // Print operation (Ovie-specific)
    Print,
}

/// Block terminator (control flow)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Terminator {
    /// Return from function
    Return { value: Option<Value> },
    
    /// Unconditional branch
    Branch { target: BlockId },
    
    /// Conditional branch
    ConditionalBranch {
        condition: Value,
        true_target: BlockId,
        false_target: BlockId,
    },
    
    /// Unreachable code
    Unreachable,
}

/// IR value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    /// Reference to an instruction result
    Instruction(ValueId),
    
    /// Constant value
    Constant(Constant),
    
    /// Function parameter
    Parameter(ValueId),
    
    /// Global variable
    Global(String),
}

/// Constant values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Constant {
    String(String),
    Number(f64),
    Boolean(bool),
    Void,
}

/// IR types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IrType {
    String,
    Number,
    Boolean,
    Void,
    Pointer(Box<IrType>),
    Function {
        params: Vec<IrType>,
        return_type: Box<IrType>,
    },
}

/// Metadata for the IR program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub source_file: String,
    pub compiler_version: String,
    pub target_triple: String,
    pub optimization_level: u8,
    pub debug_info: bool,
}

/// IR builder for constructing IR from AST
pub struct IrBuilder {
    program: Program,
    current_function: Option<FunctionId>,
    current_block: Option<BlockId>,
    next_value_id: ValueId,
    next_block_id: BlockId,
    next_function_id: FunctionId,
    deterministic_mode: bool,
}

impl IrBuilder {
    /// Create a new IR builder
    pub fn new() -> Self {
        Self {
            program: Program {
                functions: HashMap::new(),
                globals: HashMap::new(),
                metadata: Metadata {
                    source_file: String::new(),
                    compiler_version: env!("CARGO_PKG_VERSION").to_string(),
                    target_triple: "unknown".to_string(),
                    optimization_level: 0,
                    debug_info: false,
                },
                entry_point: None,
            },
            current_function: None,
            current_block: None,
            next_value_id: 1,
            next_block_id: 1,
            next_function_id: 1,
            deterministic_mode: false,
        }
    }

    /// Set deterministic mode for reproducible builds
    pub fn set_deterministic_mode(&mut self, enabled: bool) {
        self.deterministic_mode = enabled;
    }

    /// Create a new function
    pub fn create_function(&mut self, name: &str, params: Vec<Parameter>, return_type: IrType) -> FunctionId {
        let function_id = self.next_function_id;
        self.next_function_id += 1;

        let entry_block_id = self.next_block_id;
        self.next_block_id += 1;

        let mut basic_blocks = HashMap::new();
        basic_blocks.insert(entry_block_id, BasicBlock {
            id: entry_block_id,
            label: "entry".to_string(),
            instructions: Vec::new(),
            terminator: Terminator::Return { value: None },
        });

        let function = Function {
            id: function_id,
            name: name.to_string(),
            parameters: params,
            return_type,
            basic_blocks,
            entry_block: entry_block_id,
            local_variables: HashMap::new(),
        };

        self.program.functions.insert(function_id, function);
        function_id
    }

    /// Get the built program
    pub fn build(self) -> Program {
        self.program
    }

    /// Set the entry point for the program
    pub fn set_entry_point(&mut self, function_id: FunctionId) {
        self.program.entry_point = Some(function_id);
    }
}

impl Default for IrBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Program {
    /// Create a new empty program
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            globals: HashMap::new(),
            metadata: Metadata {
                source_file: String::new(),
                compiler_version: env!("CARGO_PKG_VERSION").to_string(),
                target_triple: "unknown".to_string(),
                optimization_level: 0,
                debug_info: false,
            },
            entry_point: None,
        }
    }

    /// Serialize the program to JSON
    pub fn to_json(&self) -> OvieResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| OvieError::IrError { message: format!("Serialization error: {}", e) })
    }

    /// Deserialize a program from JSON
    pub fn from_json(json: &str) -> OvieResult<Self> {
        serde_json::from_str(json)
            .map_err(|e| OvieError::IrError { message: format!("Deserialization error: {}", e) })
    }

    /// Validate the IR program for consistency
    pub fn validate(&self) -> OvieResult<()> {
        // Check that entry point exists
        if let Some(entry_id) = self.entry_point {
            if !self.functions.contains_key(&entry_id) {
                return Err(OvieError::IrError { 
                    message: "Entry point function not found".to_string() 
                });
            }
        }

        // Validate each function
        for (id, function) in &self.functions {
            if function.id != *id {
                return Err(OvieError::IrError { 
                    message: format!("Function ID mismatch: {} != {}", function.id, id) 
                });
            }

            // Check that entry block exists
            if !function.basic_blocks.contains_key(&function.entry_block) {
                return Err(OvieError::IrError { 
                    message: format!("Entry block {} not found in function {}", function.entry_block, function.name) 
                });
            }
        }

        Ok(())
    }
}
/// AST to IR transformation
use crate::ast::{AstNode, Statement, Expression, Literal};

impl IrBuilder {
    /// Transform an AST into IR
    pub fn transform_ast(&mut self, ast: &AstNode) -> OvieResult<()> {
        // Create main function
        let main_function = self.create_function("main", Vec::new(), IrType::Void);
        self.program.entry_point = Some(main_function);
        self.current_function = Some(main_function);

        // Transform statements into the main function
        for statement in &ast.statements {
            self.transform_statement(statement)?;
        }

        Ok(())
    }

    /// Transform a statement to IR (simplified version)
    fn transform_statement(&mut self, statement: &Statement) -> OvieResult<()> {
        match statement {
            Statement::Print { expression } => {
                // Transform the expression to get its value
                let value = self.transform_expression(expression)?;
                
                // Add print instruction to current function
                if let Some(function_id) = self.current_function {
                    if let Some(function) = self.program.functions.get_mut(&function_id) {
                        if let Some(block) = function.basic_blocks.get_mut(&function.entry_block) {
                            let instruction = Instruction {
                                id: self.next_value_id,
                                opcode: Opcode::Print,
                                operands: vec![value],
                                result_type: IrType::Void,
                            };
                            self.next_value_id += 1;
                            block.instructions.push(instruction);
                        }
                    }
                }
            }
            Statement::Assignment { identifier: _, value, mutable: _ } => {
                // Transform the value expression
                let _value = self.transform_expression(value)?;
                // In a full implementation, we'd handle variable storage
            }
            _ => {
                // For now, skip other statement types
            }
        }
        
        Ok(())
    }

    /// Transform an expression to IR (simplified version)
    fn transform_expression(&mut self, expression: &Expression) -> OvieResult<Value> {
        match expression {
            Expression::Literal(literal) => {
                let constant = match literal {
                    Literal::String(s) => Constant::String(s.clone()),
                    Literal::Number(n) => Constant::Number(*n),
                    Literal::Boolean(b) => Constant::Boolean(*b),
                };
                Ok(Value::Constant(constant))
            }
            Expression::Identifier(_name) => {
                // For now, return a placeholder
                Ok(Value::Constant(Constant::Void))
            }
            _ => {
                // For now, return a placeholder for other expression types
                Ok(Value::Constant(Constant::Void))
            }
        }
    }
}