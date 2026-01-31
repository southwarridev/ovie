//! Mid-level Intermediate Representation (MIR) for the Ovie compiler
//! 
//! MIR is the second IR stage after HIR, where control flow is made explicit
//! and the representation is suitable for optimization and code generation.

use crate::hir::{HirProgram, HirItem, HirFunction, HirStatement, HirStatementKind, HirExpression, HirExpressionKind, HirType, HirBinaryOp, HirUnaryOp, HirLiteral};
use crate::error::{OvieError, OvieResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unique identifier for MIR basic blocks
pub type BasicBlockId = u32;

/// Unique identifier for MIR locals (variables)
pub type LocalId = u32;

/// Unique identifier for MIR functions
pub type FunctionId = u32;

/// MIR Program - the complete program with explicit control flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirProgram {
    pub functions: HashMap<FunctionId, MirFunction>,
    pub globals: HashMap<String, MirGlobal>,
    pub type_definitions: HashMap<String, MirTypeDef>,
    pub metadata: MirMetadata,
    pub entry_point: Option<FunctionId>,
}

/// MIR Function with explicit control flow graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirFunction {
    pub id: FunctionId,
    pub name: String,
    pub signature: MirFunctionSignature,
    pub basic_blocks: HashMap<BasicBlockId, MirBasicBlock>,
    pub locals: Vec<MirLocal>,
    pub entry_block: BasicBlockId,
    pub is_main: bool,
}

/// Function signature in MIR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirFunctionSignature {
    pub parameters: Vec<MirType>,
    pub return_type: MirType,
}

/// MIR Basic Block - sequence of statements with single entry and exit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirBasicBlock {
    pub id: BasicBlockId,
    pub statements: Vec<MirStatement>,
    pub terminator: MirTerminator,
}

/// MIR Statement - operations within a basic block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirStatement {
    pub kind: MirStatementKind,
}

/// MIR Statement kinds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MirStatementKind {
    /// Assignment: place = rvalue
    Assign {
        place: MirPlace,
        rvalue: MirRvalue,
    },
    
    /// Storage allocation for a local
    StorageLive(LocalId),
    
    /// Storage deallocation for a local
    StorageDead(LocalId),
    
    /// No-op (for debugging or alignment)
    Nop,
}

/// MIR Terminator - control flow at end of basic block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MirTerminator {
    /// Return from function
    Return {
        value: Option<MirOperand>,
    },
    
    /// Unconditional jump to another block
    Goto {
        target: BasicBlockId,
    },
    
    /// Conditional branch based on discriminant
    SwitchInt {
        discriminant: MirOperand,
        targets: Vec<(u128, BasicBlockId)>,
        otherwise: BasicBlockId,
    },
    
    /// Function call
    Call {
        func: MirOperand,
        args: Vec<MirOperand>,
        destination: MirPlace,
        target: Option<BasicBlockId>,
        cleanup: Option<BasicBlockId>,
    },
    
    /// Unreachable code
    Unreachable,
    
    /// Drop (destructor call)
    Drop {
        place: MirPlace,
        target: BasicBlockId,
        unwind: Option<BasicBlockId>,
    },
}

/// MIR Place - memory location that can be assigned to
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirPlace {
    pub local: LocalId,
    pub projection: Vec<MirProjectionElem>,
}

/// MIR Projection element for complex places
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MirProjectionElem {
    /// Dereference: *place
    Deref,
    
    /// Field access: place.field
    Field(u32),
    
    /// Array/slice index: place[index]
    Index(LocalId),
    
    /// Subslice: place[from..to]
    Subslice { from: u32, to: u32 },
}

/// MIR Right-value - computations that produce values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MirRvalue {
    /// Use an operand (move or copy)
    Use(MirOperand),
    
    /// Repeat an operand N times: [operand; count]
    Repeat {
        operand: MirOperand,
        count: u64,
    },
    
    /// Reference to a place: &place or &mut place
    Ref {
        region: MirRegion,
        borrow_kind: MirBorrowKind,
        place: MirPlace,
    },
    
    /// Length of an array/slice: len(place)
    Len(MirPlace),
    
    /// Cast: operand as type
    Cast {
        kind: MirCastKind,
        operand: MirOperand,
        ty: MirType,
    },
    
    /// Binary operation: left op right
    BinaryOp {
        op: MirBinOp,
        left: MirOperand,
        right: MirOperand,
    },
    
    /// Unary operation: op operand
    UnaryOp {
        op: MirUnOp,
        operand: MirOperand,
    },
    
    /// Discriminant of an enum: discriminant(place)
    Discriminant(MirPlace),
    
    /// Aggregate construction: struct/enum/array/tuple
    Aggregate {
        kind: MirAggregateKind,
        operands: Vec<MirOperand>,
    },
}

/// MIR Operand - values used in computations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MirOperand {
    /// Copy a place
    Copy(MirPlace),
    
    /// Move from a place
    Move(MirPlace),
    
    /// Constant value
    Constant(MirConstant),
}

/// MIR Constant values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirConstant {
    pub literal: MirConstantValue,
    pub ty: MirType,
}

/// MIR Constant value literals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MirConstantValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Unit,
}

/// MIR Local variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirLocal {
    pub id: LocalId,
    pub ty: MirType,
    pub is_mutable: bool,
    pub name: Option<String>,
}

/// MIR Global variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirGlobal {
    pub name: String,
    pub ty: MirType,
    pub is_mutable: bool,
    pub initializer: Option<MirConstant>,
}

/// MIR Type system (simplified from HIR)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MirType {
    /// Primitive types
    String,
    Number,
    Boolean,
    Unit,
    
    /// Reference types
    Ref {
        region: MirRegion,
        ty: Box<MirType>,
        mutability: MirMutability,
    },
    
    /// User-defined types
    Adt {
        name: String,
        substs: Vec<MirType>,
    },
    
    /// Function pointer
    FnPtr {
        params: Vec<MirType>,
        return_type: Box<MirType>,
    },
    
    /// Array type
    Array {
        element_type: Box<MirType>,
        size: u64,
    },
    
    /// Slice type
    Slice(Box<MirType>),
    
    /// Tuple type
    Tuple(Vec<MirType>),
}

/// MIR Type definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MirTypeDef {
    Struct {
        fields: Vec<MirFieldDef>,
    },
    Enum {
        variants: Vec<MirVariantDef>,
    },
}

/// MIR Field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirFieldDef {
    pub name: String,
    pub ty: MirType,
}

/// MIR Variant definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirVariantDef {
    pub name: String,
    pub fields: Vec<MirFieldDef>,
}

/// MIR Region (lifetime) - simplified for now
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MirRegion {
    Static,
    Local(u32),
}

/// MIR Mutability
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MirMutability {
    Mut,
    Not,
}

/// MIR Borrow kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MirBorrowKind {
    Shared,
    Mut,
    Unique,
}

/// MIR Cast kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MirCastKind {
    /// Numeric cast (int to float, etc.)
    NumericCast,
    
    /// Pointer cast
    PtrToPtr,
    
    /// Function pointer cast
    FnPtrToPtr,
}

/// MIR Binary operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MirBinOp {
    Add, Sub, Mul, Div, Rem,
    BitXor, BitAnd, BitOr,
    Shl, Shr,
    Eq, Lt, Le, Ne, Ge, Gt,
}

/// MIR Unary operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MirUnOp {
    Not,
    Neg,
}

/// MIR Aggregate kinds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MirAggregateKind {
    Array(MirType),
    Tuple,
    Adt {
        name: String,
        variant: Option<u32>,
    },
}

/// MIR Metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirMetadata {
    pub source_file: String,
    pub compiler_version: String,
    pub optimization_level: u8,
    pub target_triple: String,
}

/// Control Flow Graph Analysis results
#[derive(Debug, Clone)]
pub struct CfgAnalysis {
    pub function_analyses: HashMap<FunctionId, FunctionCfgAnalysis>,
}

/// Control Flow Graph Analysis for a single function
#[derive(Debug, Clone)]
pub struct FunctionCfgAnalysis {
    pub predecessors: HashMap<BasicBlockId, Vec<BasicBlockId>>,
    pub successors: HashMap<BasicBlockId, Vec<BasicBlockId>>,
    pub dominators: HashMap<BasicBlockId, BasicBlockId>,
    pub loops: Vec<LoopInfo>,
}

/// Information about a loop in the CFG
#[derive(Debug, Clone)]
pub struct LoopInfo {
    pub header: BasicBlockId,
    pub back_edge_source: BasicBlockId,
    pub body: Vec<BasicBlockId>,
}

impl CfgAnalysis {
    pub fn new() -> Self {
        Self {
            function_analyses: HashMap::new(),
        }
    }
}

impl FunctionCfgAnalysis {
    pub fn new() -> Self {
        Self {
            predecessors: HashMap::new(),
            successors: HashMap::new(),
            dominators: HashMap::new(),
            loops: Vec::new(),
        }
    }
}

/// MIR Builder - transforms HIR to MIR
pub struct MirBuilder {
    next_function_id: FunctionId,
    next_block_id: BasicBlockId,
    next_local_id: LocalId,
    current_function: Option<FunctionId>,
    current_block: Option<BasicBlockId>,
    local_map: HashMap<String, LocalId>,
}

impl MirBuilder {
    /// Create a new MIR builder
    pub fn new() -> Self {
        Self {
            next_function_id: 0,
            next_block_id: 0,
            next_local_id: 0,
            current_function: None,
            current_block: None,
            local_map: HashMap::new(),
        }
    }

    /// Transform HIR program to MIR
    pub fn transform_hir(&mut self, hir: &HirProgram) -> OvieResult<MirProgram> {
        let mut functions = HashMap::new();
        let mut globals = HashMap::new();
        let mut type_definitions = HashMap::new();
        let mut entry_point = None;

        // Transform all items
        for item in &hir.items {
            match item {
                HirItem::Function(hir_func) => {
                    let mir_func = self.transform_function(hir_func)?;
                    if mir_func.is_main {
                        entry_point = Some(mir_func.id);
                    }
                    functions.insert(mir_func.id, mir_func);
                }
                HirItem::Global(hir_global) => {
                    let mir_global = self.transform_global(hir_global)?;
                    globals.insert(mir_global.name.clone(), mir_global);
                }
                HirItem::Struct(hir_struct) => {
                    let mir_typedef = self.transform_struct(hir_struct)?;
                    type_definitions.insert(hir_struct.name.clone(), mir_typedef);
                }
                HirItem::Enum(hir_enum) => {
                    let mir_typedef = self.transform_enum(hir_enum)?;
                    type_definitions.insert(hir_enum.name.clone(), mir_typedef);
                }
            }
        }

        let metadata = MirMetadata {
            source_file: hir.metadata.source_file.clone(),
            compiler_version: hir.metadata.compiler_version.clone(),
            optimization_level: 0,
            target_triple: "unknown".to_string(),
        };

        Ok(MirProgram {
            functions,
            globals,
            type_definitions,
            metadata,
            entry_point,
        })
    }

    /// Transform HIR function to MIR
    fn transform_function(&mut self, hir_func: &HirFunction) -> OvieResult<MirFunction> {
        let function_id = self.next_function_id;
        self.next_function_id += 1;
        self.current_function = Some(function_id);

        // Reset local state for new function
        self.next_local_id = 0;
        self.local_map.clear();

        // Create locals for parameters
        let mut locals = Vec::new();
        for param in &hir_func.parameters {
            let local_id = self.next_local_id;
            self.next_local_id += 1;
            
            locals.push(MirLocal {
                id: local_id,
                ty: self.transform_type(&param.param_type)?,
                is_mutable: false,
                name: Some(param.name.clone()),
            });
            
            self.local_map.insert(param.name.clone(), local_id);
        }

        // Transform function body with proper control flow graph construction
        let mut basic_blocks = HashMap::new();
        let entry_block = self.build_cfg(&hir_func.body, &mut basic_blocks, &mut locals)?;

        let signature = MirFunctionSignature {
            parameters: hir_func.parameters.iter()
                .map(|p| self.transform_type(&p.param_type))
                .collect::<Result<Vec<_>, _>>()?,
            return_type: self.transform_type(&hir_func.return_type)?,
        };

        Ok(MirFunction {
            id: function_id,
            name: hir_func.name.clone(),
            signature,
            basic_blocks,
            locals,
            entry_block,
            is_main: hir_func.is_main,
        })
    }

    /// Build Control Flow Graph from HIR block
    fn build_cfg(
        &mut self,
        hir_block: &crate::hir::HirBlock,
        basic_blocks: &mut HashMap<BasicBlockId, MirBasicBlock>,
        locals: &mut Vec<MirLocal>
    ) -> OvieResult<BasicBlockId> {
        let entry_block = self.next_block_id;
        self.next_block_id += 1;
        self.current_block = Some(entry_block);

        let mut current_statements = Vec::new();
        let mut current_block_id = entry_block;

        for hir_stmt in &hir_block.statements {
            match &hir_stmt.kind {
                HirStatementKind::If { condition, then_block, else_block } => {
                    // Finish current block with conditional branch
                    let condition_operand = self.transform_expression_to_operand(condition)?;
                    
                    // Create blocks for then and else branches
                    let then_block_id = self.next_block_id;
                    self.next_block_id += 1;
                    let else_block_id = self.next_block_id;
                    self.next_block_id += 1;
                    let merge_block_id = self.next_block_id;
                    self.next_block_id += 1;

                    // Create switch terminator for current block
                    let terminator = MirTerminator::SwitchInt {
                        discriminant: condition_operand,
                        targets: vec![(1, then_block_id)], // true -> then block
                        otherwise: else_block_id, // false -> else block
                    };

                    basic_blocks.insert(current_block_id, MirBasicBlock {
                        id: current_block_id,
                        statements: current_statements,
                        terminator,
                    });

                    // Build then block
                    self.build_cfg(then_block, basic_blocks, locals)?;
                    basic_blocks.insert(then_block_id, MirBasicBlock {
                        id: then_block_id,
                        statements: Vec::new(), // Simplified for now
                        terminator: MirTerminator::Goto { target: merge_block_id },
                    });

                    // Build else block
                    if let Some(else_hir_block) = else_block {
                        self.build_cfg(else_hir_block, basic_blocks, locals)?;
                    }
                    basic_blocks.insert(else_block_id, MirBasicBlock {
                        id: else_block_id,
                        statements: Vec::new(), // Simplified for now
                        terminator: MirTerminator::Goto { target: merge_block_id },
                    });

                    // Continue with merge block
                    current_block_id = merge_block_id;
                    current_statements = Vec::new();
                }
                HirStatementKind::While { condition, body } => {
                    // Create loop header, body, and exit blocks
                    let loop_header_id = self.next_block_id;
                    self.next_block_id += 1;
                    let loop_body_id = self.next_block_id;
                    self.next_block_id += 1;
                    let loop_exit_id = self.next_block_id;
                    self.next_block_id += 1;

                    // Finish current block with goto to loop header
                    basic_blocks.insert(current_block_id, MirBasicBlock {
                        id: current_block_id,
                        statements: current_statements,
                        terminator: MirTerminator::Goto { target: loop_header_id },
                    });

                    // Create loop header with condition check
                    let condition_operand = self.transform_expression_to_operand(condition)?;
                    basic_blocks.insert(loop_header_id, MirBasicBlock {
                        id: loop_header_id,
                        statements: Vec::new(),
                        terminator: MirTerminator::SwitchInt {
                            discriminant: condition_operand,
                            targets: vec![(1, loop_body_id)], // true -> body
                            otherwise: loop_exit_id, // false -> exit
                        },
                    });

                    // Build loop body
                    self.build_cfg(body, basic_blocks, locals)?;
                    basic_blocks.insert(loop_body_id, MirBasicBlock {
                        id: loop_body_id,
                        statements: Vec::new(), // Simplified for now
                        terminator: MirTerminator::Goto { target: loop_header_id },
                    });

                    // Continue with exit block
                    current_block_id = loop_exit_id;
                    current_statements = Vec::new();
                }
                HirStatementKind::Return(value) => {
                    // Return terminates the current block
                    let return_operand = if let Some(expr) = value {
                        Some(self.transform_expression_to_operand(expr)?)
                    } else {
                        None
                    };

                    basic_blocks.insert(current_block_id, MirBasicBlock {
                        id: current_block_id,
                        statements: current_statements,
                        terminator: MirTerminator::Return { value: return_operand },
                    });

                    // Create unreachable block for any remaining statements
                    current_block_id = self.next_block_id;
                    self.next_block_id += 1;
                    current_statements = Vec::new();
                }
                _ => {
                    // Regular statements
                    self.transform_statement(hir_stmt, &mut current_statements, locals)?;
                }
            }
        }

        // Finish the final block
        if !basic_blocks.contains_key(&current_block_id) {
            let terminator = MirTerminator::Return { value: None };
            basic_blocks.insert(current_block_id, MirBasicBlock {
                id: current_block_id,
                statements: current_statements,
                terminator,
            });
        }

        Ok(entry_block)
    }

    /// Transform HIR statement to MIR
    fn transform_statement(
        &mut self, 
        hir_stmt: &HirStatement, 
        statements: &mut Vec<MirStatement>,
        locals: &mut Vec<MirLocal>
    ) -> OvieResult<()> {
        match &hir_stmt.kind {
            HirStatementKind::Local { name, var_type, is_mutable, initializer } => {
                // Create local variable
                let local_id = self.next_local_id;
                self.next_local_id += 1;
                
                locals.push(MirLocal {
                    id: local_id,
                    ty: self.transform_type(var_type)?,
                    is_mutable: *is_mutable,
                    name: Some(name.clone()),
                });
                
                self.local_map.insert(name.clone(), local_id);

                // Storage allocation
                statements.push(MirStatement {
                    kind: MirStatementKind::StorageLive(local_id),
                });

                // Initialize if there's an initializer
                if let Some(init_expr) = initializer {
                    let rvalue = self.transform_expression_to_rvalue(init_expr)?;
                    let place = MirPlace {
                        local: local_id,
                        projection: Vec::new(),
                    };
                    
                    statements.push(MirStatement {
                        kind: MirStatementKind::Assign { place, rvalue },
                    });
                }
            }
            HirStatementKind::Assign { target, value } => {
                let place = self.transform_place(target)?;
                let rvalue = self.transform_expression_to_rvalue(value)?;
                
                statements.push(MirStatement {
                    kind: MirStatementKind::Assign { place, rvalue },
                });
            }
            HirStatementKind::Expression(expr) => {
                // For expression statements, we might need to evaluate for side effects
                let _rvalue = self.transform_expression_to_rvalue(expr)?;
                // For now, just add a nop
                statements.push(MirStatement {
                    kind: MirStatementKind::Nop,
                });
            }
            HirStatementKind::Print(expr) => {
                // Transform print into a function call
                let operand = self.transform_expression_to_operand(expr)?;
                
                // Create a temporary local for the result
                let temp_local = self.next_local_id;
                self.next_local_id += 1;
                
                locals.push(MirLocal {
                    id: temp_local,
                    ty: MirType::Unit,
                    is_mutable: false,
                    name: None,
                });

                // Storage allocation for temporary
                statements.push(MirStatement {
                    kind: MirStatementKind::StorageLive(temp_local),
                });

                // Create function operand for print
                let print_func = MirOperand::Constant(MirConstant {
                    literal: MirConstantValue::String("print".to_string()),
                    ty: MirType::FnPtr {
                        params: vec![MirType::String],
                        return_type: Box::new(MirType::Unit),
                    },
                });

                // This would become a Call terminator in a real implementation
                // For now, just assign the result
                statements.push(MirStatement {
                    kind: MirStatementKind::Assign {
                        place: MirPlace {
                            local: temp_local,
                            projection: Vec::new(),
                        },
                        rvalue: MirRvalue::Use(operand),
                    },
                });
            }
            HirStatementKind::Return(value) => {
                // Return statements become terminators, not regular statements
                // For now, just add a nop
                statements.push(MirStatement {
                    kind: MirStatementKind::Nop,
                });
            }
            _ => {
                // Other statement types would be handled here
                statements.push(MirStatement {
                    kind: MirStatementKind::Nop,
                });
            }
        }
        
        Ok(())
    }

    /// Transform HIR expression to MIR rvalue
    fn transform_expression_to_rvalue(&mut self, expr: &HirExpression) -> OvieResult<MirRvalue> {
        match &expr.kind {
            HirExpressionKind::Literal(lit) => {
                let constant = self.transform_literal(lit, &expr.expr_type)?;
                Ok(MirRvalue::Use(MirOperand::Constant(constant)))
            }
            HirExpressionKind::Variable(name) => {
                if let Some(&local_id) = self.local_map.get(name) {
                    let place = MirPlace {
                        local: local_id,
                        projection: Vec::new(),
                    };
                    Ok(MirRvalue::Use(MirOperand::Copy(place)))
                } else {
                    Err(OvieError::SemanticError {
                        message: format!("Variable '{}' not found in MIR transformation", name),
                    })
                }
            }
            HirExpressionKind::Binary { left, op, right } => {
                let left_operand = self.transform_expression_to_operand(left)?;
                let right_operand = self.transform_expression_to_operand(right)?;
                let mir_op = self.transform_binary_op(op);
                
                Ok(MirRvalue::BinaryOp {
                    op: mir_op,
                    left: left_operand,
                    right: right_operand,
                })
            }
            HirExpressionKind::Unary { op, operand } => {
                let mir_operand = self.transform_expression_to_operand(operand)?;
                let mir_op = self.transform_unary_op(op);
                
                Ok(MirRvalue::UnaryOp {
                    op: mir_op,
                    operand: mir_operand,
                })
            }
            HirExpressionKind::FieldAccess { object, field } => {
                let object_place = self.transform_expression_to_place(object)?;
                // For now, assume field access is by index (would need field resolution)
                let field_place = MirPlace {
                    local: object_place.local,
                    projection: {
                        let mut proj = object_place.projection;
                        proj.push(MirProjectionElem::Field(0)); // Simplified field index
                        proj
                    },
                };
                Ok(MirRvalue::Use(MirOperand::Copy(field_place)))
            }
            HirExpressionKind::StructInit { struct_name, fields } => {
                let mut operands = Vec::new();
                for field_init in fields {
                    operands.push(self.transform_expression_to_operand(&field_init.value)?);
                }
                
                Ok(MirRvalue::Aggregate {
                    kind: MirAggregateKind::Adt {
                        name: struct_name.clone(),
                        variant: None,
                    },
                    operands,
                })
            }
            HirExpressionKind::Range { start, end } => {
                let start_operand = self.transform_expression_to_operand(start)?;
                let end_operand = self.transform_expression_to_operand(end)?;
                
                Ok(MirRvalue::Aggregate {
                    kind: MirAggregateKind::Adt {
                        name: "Range".to_string(),
                        variant: None,
                    },
                    operands: vec![start_operand, end_operand],
                })
            }
            HirExpressionKind::Call { function, arguments } => {
                // Function calls need special handling - they become terminators
                // For now, return a placeholder
                Ok(MirRvalue::Use(MirOperand::Constant(MirConstant {
                    literal: MirConstantValue::Unit,
                    ty: MirType::Unit,
                })))
            }
        }
    }

    /// Transform HIR expression to MIR operand
    fn transform_expression_to_operand(&mut self, expr: &HirExpression) -> OvieResult<MirOperand> {
        match &expr.kind {
            HirExpressionKind::Literal(lit) => {
                let constant = self.transform_literal(lit, &expr.expr_type)?;
                Ok(MirOperand::Constant(constant))
            }
            HirExpressionKind::Variable(name) => {
                if let Some(&local_id) = self.local_map.get(name) {
                    let place = MirPlace {
                        local: local_id,
                        projection: Vec::new(),
                    };
                    Ok(MirOperand::Copy(place))
                } else {
                    Err(OvieError::SemanticError {
                        message: format!("Variable '{}' not found in MIR transformation", name),
                    })
                }
            }
            _ => {
                // For complex expressions, create a temporary and assign the rvalue to it
                let temp_local = self.next_local_id;
                self.next_local_id += 1;
                
                let place = MirPlace {
                    local: temp_local,
                    projection: Vec::new(),
                };
                
                Ok(MirOperand::Copy(place))
            }
        }
    }

    /// Transform HIR expression to MIR place (for assignment targets)
    fn transform_expression_to_place(&mut self, expr: &HirExpression) -> OvieResult<MirPlace> {
        match &expr.kind {
            HirExpressionKind::Variable(name) => {
                if let Some(&local_id) = self.local_map.get(name) {
                    Ok(MirPlace {
                        local: local_id,
                        projection: Vec::new(),
                    })
                } else {
                    Err(OvieError::SemanticError {
                        message: format!("Variable '{}' not found in MIR transformation", name),
                    })
                }
            }
            HirExpressionKind::FieldAccess { object, field: _ } => {
                let object_place = self.transform_expression_to_place(object)?;
                Ok(MirPlace {
                    local: object_place.local,
                    projection: {
                        let mut proj = object_place.projection;
                        proj.push(MirProjectionElem::Field(0)); // Simplified field index
                        proj
                    },
                })
            }
            _ => {
                Err(OvieError::SemanticError {
                    message: "Invalid assignment target".to_string(),
                })
            }
        }
    }

    /// Transform HIR literal to MIR constant
    fn transform_literal(&self, lit: &HirLiteral, ty: &HirType) -> OvieResult<MirConstant> {
        let (literal, mir_type) = match lit {
            HirLiteral::String(s) => (MirConstantValue::String(s.clone()), MirType::String),
            HirLiteral::Number(n) => (MirConstantValue::Number(*n), MirType::Number),
            HirLiteral::Boolean(b) => (MirConstantValue::Boolean(*b), MirType::Boolean),
            HirLiteral::Unit => (MirConstantValue::Unit, MirType::Unit),
        };
        
        Ok(MirConstant {
            literal,
            ty: mir_type,
        })
    }

    /// Transform HIR type to MIR type
    fn transform_type(&self, hir_type: &HirType) -> OvieResult<MirType> {
        match hir_type {
            HirType::String => Ok(MirType::String),
            HirType::Number => Ok(MirType::Number),
            HirType::Boolean => Ok(MirType::Boolean),
            HirType::Unit => Ok(MirType::Unit),
            HirType::Struct(name) => Ok(MirType::Adt {
                name: name.clone(),
                substs: Vec::new(),
            }),
            HirType::Enum(name) => Ok(MirType::Adt {
                name: name.clone(),
                substs: Vec::new(),
            }),
            HirType::Function { params, return_type } => {
                let param_types = params.iter()
                    .map(|p| self.transform_type(p))
                    .collect::<Result<Vec<_>, _>>()?;
                let ret_type = self.transform_type(return_type)?;
                
                Ok(MirType::FnPtr {
                    params: param_types,
                    return_type: Box::new(ret_type),
                })
            }
            HirType::Range(inner) => {
                // For now, treat ranges as a special struct
                Ok(MirType::Adt {
                    name: "Range".to_string(),
                    substs: vec![self.transform_type(inner)?],
                })
            }
            HirType::Error => Ok(MirType::Unit), // Error recovery
            HirType::Infer(_) => Ok(MirType::Unit), // Should be resolved by now
        }
    }

    /// Transform HIR binary operator to MIR
    fn transform_binary_op(&self, op: &HirBinaryOp) -> MirBinOp {
        match op {
            HirBinaryOp::Add => MirBinOp::Add,
            HirBinaryOp::Sub => MirBinOp::Sub,
            HirBinaryOp::Mul => MirBinOp::Mul,
            HirBinaryOp::Div => MirBinOp::Div,
            HirBinaryOp::Mod => MirBinOp::Rem,
            HirBinaryOp::Eq => MirBinOp::Eq,
            HirBinaryOp::Ne => MirBinOp::Ne,
            HirBinaryOp::Lt => MirBinOp::Lt,
            HirBinaryOp::Le => MirBinOp::Le,
            HirBinaryOp::Gt => MirBinOp::Gt,
            HirBinaryOp::Ge => MirBinOp::Ge,
            HirBinaryOp::And => MirBinOp::BitAnd, // Logical AND as bitwise for now
            HirBinaryOp::Or => MirBinOp::BitOr,   // Logical OR as bitwise for now
        }
    }

    /// Transform HIR unary operator to MIR
    fn transform_unary_op(&self, op: &HirUnaryOp) -> MirUnOp {
        match op {
            HirUnaryOp::Not => MirUnOp::Not,
            HirUnaryOp::Neg => MirUnOp::Neg,
        }
    }

    /// Transform HIR place (for assignment targets)
    fn transform_place(&self, _hir_place: &crate::hir::HirPlace) -> OvieResult<MirPlace> {
        // Simplified implementation
        Ok(MirPlace {
            local: 0,
            projection: Vec::new(),
        })
    }

    /// Transform HIR global to MIR
    fn transform_global(&self, hir_global: &crate::hir::HirGlobal) -> OvieResult<MirGlobal> {
        let initializer = if let Some(ref init_expr) = hir_global.initializer {
            // For globals, initializer must be a constant
            if let HirExpressionKind::Literal(lit) = &init_expr.kind {
                Some(self.transform_literal(lit, &init_expr.expr_type)?)
            } else {
                None // Non-constant initializers not supported yet
            }
        } else {
            None
        };

        Ok(MirGlobal {
            name: hir_global.name.clone(),
            ty: self.transform_type(&hir_global.global_type)?,
            is_mutable: hir_global.is_mutable,
            initializer,
        })
    }

    /// Transform HIR struct to MIR type definition
    fn transform_struct(&self, hir_struct: &crate::hir::HirStruct) -> OvieResult<MirTypeDef> {
        let mut fields = Vec::new();
        
        for field in &hir_struct.fields {
            fields.push(MirFieldDef {
                name: field.name.clone(),
                ty: self.transform_type(&field.field_type)?,
            });
        }

        Ok(MirTypeDef::Struct { fields })
    }

    /// Transform HIR enum to MIR type definition
    fn transform_enum(&self, hir_enum: &crate::hir::HirEnum) -> OvieResult<MirTypeDef> {
        let mut variants = Vec::new();
        
        for variant in &hir_enum.variants {
            let fields = if let Some(ref data_type) = variant.data_type {
                vec![MirFieldDef {
                    name: "data".to_string(),
                    ty: self.transform_type(data_type)?,
                }]
            } else {
                Vec::new()
            };
            
            variants.push(MirVariantDef {
                name: variant.name.clone(),
                fields,
            });
        }

        Ok(MirTypeDef::Enum { variants })
    }

    /// Check if statements contain a terminator
    fn has_terminator(&self, _statements: &[MirStatement]) -> bool {
        // Simplified check - in a real implementation, we'd check the last statement
        false
    }
}

impl Default for MirBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl MirProgram {
    /// Serialize MIR program to JSON
    pub fn to_json(&self) -> OvieResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| OvieError::IrError { message: format!("MIR serialization error: {}", e) })
    }

    /// Deserialize MIR program from JSON
    pub fn from_json(json: &str) -> OvieResult<Self> {
        serde_json::from_str(json)
            .map_err(|e| OvieError::IrError { message: format!("MIR deserialization error: {}", e) })
    }

    /// Analyze control flow graph and compute basic block properties
    pub fn analyze_cfg(&self) -> OvieResult<CfgAnalysis> {
        let mut analysis = CfgAnalysis::new();
        
        for (func_id, function) in &self.functions {
            let func_analysis = self.analyze_function_cfg(function)?;
            analysis.function_analyses.insert(*func_id, func_analysis);
        }
        
        Ok(analysis)
    }

    /// Generate human-readable IR report
    pub fn generate_ir_report(&self) -> OvieResult<String> {
        let mut report = String::new();
        
        report.push_str("=== MIR Program Analysis Report ===\n\n");
        
        // Program overview
        report.push_str(&format!("Source File: {}\n", self.metadata.source_file));
        report.push_str(&format!("Compiler Version: {}\n", self.metadata.compiler_version));
        report.push_str(&format!("Target Triple: {}\n", self.metadata.target_triple));
        report.push_str(&format!("Optimization Level: {}\n\n", self.metadata.optimization_level));
        
        // Functions summary
        report.push_str(&format!("Functions: {}\n", self.functions.len()));
        report.push_str(&format!("Globals: {}\n", self.globals.len()));
        report.push_str(&format!("Type Definitions: {}\n\n", self.type_definitions.len()));
        
        // Entry point
        if let Some(entry_id) = self.entry_point {
            if let Some(entry_func) = self.functions.get(&entry_id) {
                report.push_str(&format!("Entry Point: {} (ID: {})\n\n", entry_func.name, entry_id));
            }
        }
        
        // Function details
        report.push_str("=== Function Details ===\n\n");
        for (func_id, function) in &self.functions {
            report.push_str(&self.generate_function_report(function)?);
            report.push_str("\n");
        }
        
        // Control flow analysis
        if let Ok(cfg_analysis) = self.analyze_cfg() {
            report.push_str("=== Control Flow Analysis ===\n\n");
            for (func_id, func_analysis) in &cfg_analysis.function_analyses {
                if let Some(function) = self.functions.get(func_id) {
                    report.push_str(&format!("Function: {}\n", function.name));
                    report.push_str(&format!("  Basic Blocks: {}\n", function.basic_blocks.len()));
                    report.push_str(&format!("  Locals: {}\n", function.locals.len()));
                    report.push_str(&format!("  Loops: {}\n", func_analysis.loops.len()));
                    
                    // Predecessor/successor info
                    for (block_id, successors) in &func_analysis.successors {
                        if !successors.is_empty() {
                            report.push_str(&format!("  Block {} -> {:?}\n", block_id, successors));
                        }
                    }
                    report.push_str("\n");
                }
            }
        }
        
        Ok(report)
    }

    /// Generate detailed function report
    fn generate_function_report(&self, function: &MirFunction) -> OvieResult<String> {
        let mut report = String::new();
        
        report.push_str(&format!("Function: {} (ID: {})\n", function.name, function.id));
        report.push_str(&format!("  Is Main: {}\n", function.is_main));
        report.push_str(&format!("  Parameters: {:?}\n", function.signature.parameters));
        report.push_str(&format!("  Return Type: {:?}\n", function.signature.return_type));
        report.push_str(&format!("  Entry Block: {}\n", function.entry_block));
        report.push_str(&format!("  Basic Blocks: {}\n", function.basic_blocks.len()));
        report.push_str(&format!("  Locals: {}\n", function.locals.len()));
        
        // Local variables details
        if !function.locals.is_empty() {
            report.push_str("  Local Variables:\n");
            for local in &function.locals {
                let name = local.name.as_ref().map(|s| s.as_str()).unwrap_or("<anonymous>");
                let mutability = if local.is_mutable { "mut" } else { "immut" };
                report.push_str(&format!("    {}: {:?} ({})\n", name, local.ty, mutability));
            }
        }
        
        // Basic blocks details
        report.push_str("  Basic Blocks:\n");
        for (block_id, block) in &function.basic_blocks {
            report.push_str(&format!("    Block {}:\n", block_id));
            report.push_str(&format!("      Statements: {}\n", block.statements.len()));
            report.push_str(&format!("      Terminator: {:?}\n", block.terminator));
        }
        
        Ok(report)
    }

    /// Export MIR in GraphViz DOT format for visualization
    pub fn to_dot(&self) -> OvieResult<String> {
        let mut dot = String::new();
        
        dot.push_str("digraph MIR {\n");
        dot.push_str("  rankdir=TB;\n");
        dot.push_str("  node [shape=box];\n\n");
        
        for (func_id, function) in &self.functions {
            dot.push_str(&format!("  subgraph cluster_{} {{\n", func_id));
            dot.push_str(&format!("    label=\"Function: {}\";\n", function.name));
            dot.push_str("    style=filled;\n");
            dot.push_str("    color=lightgrey;\n\n");
            
            // Add basic blocks as nodes
            for (block_id, block) in &function.basic_blocks {
                let label = format!("Block {}\\n{} statements", block_id, block.statements.len());
                dot.push_str(&format!("    \"{}_{}\", [label=\"{}\"];\n", func_id, block_id, label));
            }
            
            // Add edges based on terminators
            for (block_id, block) in &function.basic_blocks {
                let successors = self.get_block_successors(&block.terminator);
                for successor in successors {
                    dot.push_str(&format!("    \"{}_{}\", -> \"{}_{}\",;\n", 
                                        func_id, block_id, func_id, successor));
                }
            }
            
            dot.push_str("  }\n\n");
        }
        
        dot.push_str("}\n");
        Ok(dot)
    }

    /// Analyze control flow graph for a single function
    fn analyze_function_cfg(&self, function: &MirFunction) -> OvieResult<FunctionCfgAnalysis> {
        let mut analysis = FunctionCfgAnalysis::new();
        
        // Build predecessor and successor maps
        for (block_id, block) in &function.basic_blocks {
            let successors = self.get_block_successors(&block.terminator);
            analysis.successors.insert(*block_id, successors.clone());
            
            // Add to predecessors
            for successor in successors {
                analysis.predecessors.entry(successor)
                    .or_insert_with(Vec::new)
                    .push(*block_id);
            }
        }

        // Compute dominance information
        analysis.dominators = self.compute_dominators(function, &analysis)?;
        
        // Identify loops
        analysis.loops = self.identify_loops(function, &analysis)?;
        
        Ok(analysis)
    }

    /// Get successor blocks from a terminator
    fn get_block_successors(&self, terminator: &MirTerminator) -> Vec<BasicBlockId> {
        match terminator {
            MirTerminator::Return { .. } | MirTerminator::Unreachable => Vec::new(),
            MirTerminator::Goto { target } => vec![*target],
            MirTerminator::SwitchInt { targets, otherwise, .. } => {
                let mut successors: Vec<_> = targets.iter().map(|(_, target)| *target).collect();
                successors.push(*otherwise);
                successors
            }
            MirTerminator::Call { target, cleanup, .. } => {
                let mut successors = Vec::new();
                if let Some(t) = target {
                    successors.push(*t);
                }
                if let Some(c) = cleanup {
                    successors.push(*c);
                }
                successors
            }
            MirTerminator::Drop { target, unwind, .. } => {
                let mut successors = vec![*target];
                if let Some(u) = unwind {
                    successors.push(*u);
                }
                successors
            }
        }
    }

    /// Compute dominator tree for a function
    fn compute_dominators(
        &self, 
        function: &MirFunction, 
        cfg_analysis: &FunctionCfgAnalysis
    ) -> OvieResult<HashMap<BasicBlockId, BasicBlockId>> {
        let mut dominators = HashMap::new();
        let entry = function.entry_block;
        
        // Entry block dominates itself
        dominators.insert(entry, entry);
        
        // Simple dominator computation (could be optimized)
        let mut changed = true;
        while changed {
            changed = false;
            
            for &block_id in function.basic_blocks.keys() {
                if block_id == entry {
                    continue;
                }
                
                if let Some(predecessors) = cfg_analysis.predecessors.get(&block_id) {
                    if !predecessors.is_empty() {
                        // Find common dominator of all predecessors
                        let mut new_dom = predecessors[0];
                        for &pred in &predecessors[1..] {
                            new_dom = self.find_common_dominator(new_dom, pred, &dominators);
                        }
                        
                        if dominators.get(&block_id) != Some(&new_dom) {
                            dominators.insert(block_id, new_dom);
                            changed = true;
                        }
                    }
                }
            }
        }
        
        Ok(dominators)
    }

    /// Find common dominator of two blocks
    fn find_common_dominator(
        &self,
        block1: BasicBlockId,
        block2: BasicBlockId,
        dominators: &HashMap<BasicBlockId, BasicBlockId>
    ) -> BasicBlockId {
        // Simplified implementation - in practice would use more efficient algorithm
        if let (Some(&dom1), Some(&dom2)) = (dominators.get(&block1), dominators.get(&block2)) {
            if dom1 == dom2 {
                dom1
            } else {
                // Find common ancestor in dominator tree
                dom1 // Simplified
            }
        } else {
            block1 // Fallback
        }
    }

    /// Identify loops in the control flow graph
    fn identify_loops(
        &self,
        function: &MirFunction,
        cfg_analysis: &FunctionCfgAnalysis
    ) -> OvieResult<Vec<LoopInfo>> {
        let mut loops = Vec::new();
        
        // Find back edges (edges to dominators)
        for (block_id, block) in &function.basic_blocks {
            let successors = self.get_block_successors(&block.terminator);
            
            for successor in successors {
                if let Some(dominators) = &cfg_analysis.dominators {
                    if let Some(&dom) = dominators.get(block_id) {
                        if successor == dom || self.dominates(successor, *block_id, dominators) {
                            // This is a back edge - indicates a loop
                            loops.push(LoopInfo {
                                header: successor,
                                back_edge_source: *block_id,
                                body: Vec::new(), // Would compute loop body
                            });
                        }
                    }
                }
            }
        }
        
        Ok(loops)
    }

    /// Check if block1 dominates block2
    fn dominates(
        &self,
        block1: BasicBlockId,
        block2: BasicBlockId,
        dominators: &HashMap<BasicBlockId, BasicBlockId>
    ) -> bool {
        let mut current = block2;
        while let Some(&dom) = dominators.get(&current) {
            if dom == block1 {
                return true;
            }
            if dom == current {
                break; // Reached root
            }
            current = dom;
        }
        false
    }

    /// Validate MIR program
    pub fn validate(&self) -> OvieResult<()> {
        // Check that entry point exists
        if let Some(entry_id) = self.entry_point {
            if !self.functions.contains_key(&entry_id) {
                return Err(OvieError::IrError {
                    message: "Entry point function not found".to_string(),
                });
            }
        }

        // Validate each function's control flow graph
        for (id, function) in &self.functions {
            if function.id != *id {
                return Err(OvieError::IrError {
                    message: format!("Function ID mismatch: {} != {}", function.id, id),
                });
            }

            // Check that entry block exists
            if !function.basic_blocks.contains_key(&function.entry_block) {
                return Err(OvieError::IrError {
                    message: format!("Entry block {} not found in function {}", function.entry_block, function.name),
                });
            }

            // Validate basic blocks
            for (block_id, block) in &function.basic_blocks {
                if block.id != *block_id {
                    return Err(OvieError::IrError {
                        message: format!("Block ID mismatch: {} != {}", block.id, block_id),
                    });
                }

                // Check that all referenced locals exist
                for statement in &block.statements {
                    self.validate_statement(statement, function)?;
                }

                self.validate_terminator(&block.terminator, function)?;
            }
        }

        Ok(())
    }

    /// Validate a MIR statement
    fn validate_statement(&self, statement: &MirStatement, function: &MirFunction) -> OvieResult<()> {
        match &statement.kind {
            MirStatementKind::Assign { place, rvalue: _ } => {
                self.validate_place(place, function)?;
            }
            MirStatementKind::StorageLive(local_id) | MirStatementKind::StorageDead(local_id) => {
                if !function.locals.iter().any(|local| local.id == *local_id) {
                    return Err(OvieError::IrError {
                        message: format!("Local {} not found in function {}", local_id, function.name),
                    });
                }
            }
            MirStatementKind::Nop => {
                // No validation needed for nop
            }
        }
        Ok(())
    }

    /// Validate a MIR terminator
    fn validate_terminator(&self, terminator: &MirTerminator, function: &MirFunction) -> OvieResult<()> {
        match terminator {
            MirTerminator::Goto { target } => {
                if !function.basic_blocks.contains_key(target) {
                    return Err(OvieError::IrError {
                        message: format!("Target block {} not found in function {}", target, function.name),
                    });
                }
            }
            MirTerminator::SwitchInt { targets, otherwise, .. } => {
                for (_, target) in targets {
                    if !function.basic_blocks.contains_key(target) {
                        return Err(OvieError::IrError {
                            message: format!("Target block {} not found in function {}", target, function.name),
                        });
                    }
                }
                if !function.basic_blocks.contains_key(otherwise) {
                    return Err(OvieError::IrError {
                        message: format!("Otherwise block {} not found in function {}", otherwise, function.name),
                    });
                }
            }
            MirTerminator::Call { destination, target, cleanup, .. } => {
                self.validate_place(destination, function)?;
                if let Some(target_block) = target {
                    if !function.basic_blocks.contains_key(target_block) {
                        return Err(OvieError::IrError {
                            message: format!("Target block {} not found in function {}", target_block, function.name),
                        });
                    }
                }
                if let Some(cleanup_block) = cleanup {
                    if !function.basic_blocks.contains_key(cleanup_block) {
                        return Err(OvieError::IrError {
                            message: format!("Cleanup block {} not found in function {}", cleanup_block, function.name),
                        });
                    }
                }
            }
            MirTerminator::Drop { target, unwind, .. } => {
                if !function.basic_blocks.contains_key(target) {
                    return Err(OvieError::IrError {
                        message: format!("Target block {} not found in function {}", target, function.name),
                    });
                }
                if let Some(unwind_block) = unwind {
                    if !function.basic_blocks.contains_key(unwind_block) {
                        return Err(OvieError::IrError {
                            message: format!("Unwind block {} not found in function {}", unwind_block, function.name),
                        });
                    }
                }
            }
            MirTerminator::Return { .. } | MirTerminator::Unreachable => {
                // No validation needed
            }
        }
        Ok(())
    }

    /// Validate a MIR place
    fn validate_place(&self, place: &MirPlace, function: &MirFunction) -> OvieResult<()> {
        if !function.locals.iter().any(|local| local.id == place.local) {
            return Err(OvieError::IrError {
                message: format!("Local {} not found in function {}", place.local, function.name),
            });
        }
        Ok(())
    }
}