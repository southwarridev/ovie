//! Backend invariant tests for the Ovie compiler
//! 
//! These tests verify that the IR is properly prepared for backend code generation
//! and that all backend invariants are maintained.

use crate::ir::{Program, Function, BasicBlock, Instruction, Terminator, Opcode, Value, Constant, IrType, Parameter, Global, Metadata, BackendInvariantValidation};
use crate::error::OvieError;
use std::collections::HashMap;

#[test]
fn test_backend_invariants_valid_program() {
    // Create a valid program that should pass all backend invariants
    let mut program = create_valid_program();
    
    // Should pass all invariant checks
    assert!(program.validate_backend_invariants().is_ok());
}

#[test]
fn test_optimized_mir_validation_passes_for_optimized_code() {
    // Create a program with optimized IR (no unreachable blocks, constants folded)
    let program = create_optimized_program();
    
    // Should pass optimized MIR validation
    assert!(program.validate_optimized_mir().is_ok());
}

#[test]
fn test_optimized_mir_validation_fails_for_unreachable_blocks() {
    // Create a program with unreachable basic blocks
    let program = create_program_with_unreachable_blocks();
    
    // Should fail optimized MIR validation
    let result = program.validate_optimized_mir();
    assert!(result.is_err());
    
    if let Err(OvieError::InvariantViolation { stage, message }) = result {
        assert_eq!(stage, "Backend");
        assert!(message.contains("Unreachable basic block"));
    } else {
        panic!("Expected InvariantViolation error");
    }
}

#[test]
fn test_optimized_mir_validation_fails_for_unfolded_constants() {
    // Create a program with constant expressions that should have been folded
    let program = create_program_with_unfolded_constants();
    
    // Should fail optimized MIR validation
    let result = program.validate_optimized_mir();
    assert!(result.is_err());
    
    if let Err(OvieError::InvariantViolation { stage, message }) = result {
        assert_eq!(stage, "Backend");
        assert!(message.contains("Constant folding not applied"));
    } else {
        panic!("Expected InvariantViolation error");
    }
}

#[test]
fn test_complete_abi_validation_passes_for_complete_abi() {
    // Create a program with complete ABI information
    let program = create_program_with_complete_abi();
    
    // Should pass ABI validation
    assert!(program.validate_complete_abi().is_ok());
}

#[test]
fn test_complete_abi_validation_fails_for_empty_function_name() {
    // Create a program with empty function name
    let program = create_program_with_empty_function_name();
    
    // Should fail ABI validation
    let result = program.validate_complete_abi();
    assert!(result.is_err());
    
    if let Err(OvieError::InvariantViolation { stage, message }) = result {
        assert_eq!(stage, "Backend");
        assert!(message.contains("has empty name"));
    } else {
        panic!("Expected InvariantViolation error");
    }
}

#[test]
fn test_resolved_symbols_validation_passes_for_resolved_symbols() {
    // Create a program with all symbols resolved
    let program = create_program_with_resolved_symbols();
    
    // Should pass symbol resolution validation
    assert!(program.validate_resolved_symbols().is_ok());
}

#[test]
fn test_resolved_symbols_validation_fails_for_unresolved_global() {
    // Create a program with unresolved global symbol
    let program = create_program_with_unresolved_global();
    
    // Should fail symbol resolution validation
    let result = program.validate_resolved_symbols();
    assert!(result.is_err());
    
    if let Err(OvieError::InvariantViolation { stage, message }) = result {
        assert_eq!(stage, "Backend");
        assert!(message.contains("Unresolved global symbol"));
    } else {
        panic!("Expected InvariantViolation error");
    }
}

#[test]
fn test_resolved_symbols_validation_fails_for_unresolved_instruction() {
    // Create a program with unresolved instruction reference
    let program = create_program_with_unresolved_instruction();
    
    // Should fail symbol resolution validation
    let result = program.validate_resolved_symbols();
    assert!(result.is_err());
    
    if let Err(OvieError::InvariantViolation { stage, message }) = result {
        assert_eq!(stage, "Backend");
        assert!(message.contains("Unresolved instruction reference"));
    } else {
        panic!("Expected InvariantViolation error");
    }
}

#[test]
fn test_resolved_symbols_validation_fails_for_unresolved_function_call() {
    // Create a program with unresolved function call
    let program = create_program_with_unresolved_function_call();
    
    // Should fail symbol resolution validation
    let result = program.validate_resolved_symbols();
    assert!(result.is_err());
    
    if let Err(OvieError::InvariantViolation { stage, message }) = result {
        assert_eq!(stage, "Backend");
        assert!(message.contains("Unresolved function call"));
    } else {
        panic!("Expected InvariantViolation error");
    }
}

// Helper functions to create test programs

fn create_valid_program() -> Program {
    let mut functions = HashMap::new();
    let mut basic_blocks = HashMap::new();
    
    // Create a simple basic block with return
    basic_blocks.insert(1, BasicBlock {
        id: 1,
        label: "entry".to_string(),
        instructions: vec![],
        terminator: Terminator::Return { value: None },
    });
    
    // Create main function
    let main_function = Function {
        id: 1,
        name: "main".to_string(),
        parameters: vec![],
        return_type: IrType::Void,
        basic_blocks,
        entry_block: 1,
        local_variables: HashMap::new(),
    };
    
    functions.insert(1, main_function);
    
    Program {
        functions,
        globals: HashMap::new(),
        metadata: Metadata {
            source_file: "test.ov".to_string(),
            compiler_version: "2.2.0".to_string(),
            target_triple: "wasm32-unknown-unknown".to_string(),
            optimization_level: 2,
            debug_info: false,
        },
        entry_point: Some(1),
    }
}

fn create_optimized_program() -> Program {
    // Create a program that appears to be optimized (no unreachable blocks, constants folded)
    create_valid_program()
}

fn create_program_with_unreachable_blocks() -> Program {
    let mut program = create_valid_program();
    
    // Add an unreachable block
    if let Some(function) = program.functions.get_mut(&1) {
        function.basic_blocks.insert(2, BasicBlock {
            id: 2,
            label: "unreachable".to_string(),
            instructions: vec![],
            terminator: Terminator::Return { value: None },
        });
    }
    
    program
}

fn create_program_with_unfolded_constants() -> Program {
    let mut program = create_valid_program();
    
    // Add an instruction that adds two constants (should have been folded)
    if let Some(function) = program.functions.get_mut(&1) {
        if let Some(block) = function.basic_blocks.get_mut(&1) {
            block.instructions.push(Instruction {
                id: 1,
                opcode: Opcode::Add,
                operands: vec![
                    Value::Constant(Constant::Number(1.0)),
                    Value::Constant(Constant::Number(2.0)),
                ],
                result_type: IrType::Number,
            });
        }
    }
    
    program
}

fn create_program_with_complete_abi() -> Program {
    create_valid_program()
}

fn create_program_with_empty_function_name() -> Program {
    let mut program = create_valid_program();
    
    // Set function name to empty
    if let Some(function) = program.functions.get_mut(&1) {
        function.name = String::new();
    }
    
    program
}

fn create_program_with_resolved_symbols() -> Program {
    create_valid_program()
}

fn create_program_with_unresolved_global() -> Program {
    let mut program = create_valid_program();
    
    // Add an instruction that references a non-existent global
    if let Some(function) = program.functions.get_mut(&1) {
        if let Some(block) = function.basic_blocks.get_mut(&1) {
            block.instructions.push(Instruction {
                id: 1,
                opcode: Opcode::Load,
                operands: vec![Value::Global("nonexistent_global".to_string())],
                result_type: IrType::Number,
            });
        }
    }
    
    program
}

fn create_program_with_unresolved_instruction() -> Program {
    let mut program = create_valid_program();
    
    // Add an instruction that references a non-existent instruction
    if let Some(function) = program.functions.get_mut(&1) {
        if let Some(block) = function.basic_blocks.get_mut(&1) {
            block.instructions.push(Instruction {
                id: 1,
                opcode: Opcode::Add,
                operands: vec![
                    Value::Instruction(999), // Non-existent instruction
                    Value::Constant(Constant::Number(1.0)),
                ],
                result_type: IrType::Number,
            });
        }
    }
    
    program
}

fn create_program_with_unresolved_function_call() -> Program {
    let mut program = create_valid_program();
    
    // Add a call instruction to a non-existent function
    if let Some(function) = program.functions.get_mut(&1) {
        if let Some(block) = function.basic_blocks.get_mut(&1) {
            block.instructions.push(Instruction {
                id: 1,
                opcode: Opcode::Call,
                operands: vec![Value::Global("nonexistent_function".to_string())],
                result_type: IrType::Void,
            });
        }
    }
    
    program
}