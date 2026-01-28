//! LLVM code generation backend

#[cfg(feature = "llvm")]
use crate::ir::{Program, Function, BasicBlock, Instruction, Terminator, Opcode, Value, Constant};
#[cfg(feature = "llvm")]
use crate::error::{OvieError, OvieResult};
#[cfg(feature = "llvm")]
use super::CodegenBackend;
#[cfg(feature = "llvm")]
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{FunctionValue, BasicValueEnum, IntValue};
use inkwell::types::{BasicTypeEnum, IntType};
use inkwell::{IntPredicate, OptimizationLevel};
use std::collections::HashMap;

/// LLVM code generation backend
pub struct LlvmBackend<'ctx> {
    /// LLVM context
    context: &'ctx Context,
    /// LLVM module
    module: Module<'ctx>,
    /// LLVM builder
    builder: Builder<'ctx>,
    /// Function mapping from IR to LLVM
    functions: HashMap<u32, FunctionValue<'ctx>>,
    /// Current function being built
    current_function: Option<FunctionValue<'ctx>>,
    /// Deterministic mode for reproducible builds
    deterministic_mode: bool,
}

impl<'ctx> LlvmBackend<'ctx> {
    /// Create a new LLVM backend
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        
        Self {
            context,
            module,
            builder,
            functions: HashMap::new(),
            current_function: None,
            deterministic_mode: false,
        }
    }

    /// Set deterministic mode for reproducible builds
    pub fn set_deterministic_mode(&mut self, enabled: bool) {
        self.deterministic_mode = enabled;
    }

    /// Generate LLVM IR from Ovie IR
    fn generate_llvm_ir(&mut self, ir: &Program) -> OvieResult<String> {
        // Declare external functions (like printf)
        self.declare_external_functions()?;
        
        // Generate function declarations
        self.generate_function_declarations(ir)?;
        
        // Generate function bodies
        self.generate_function_bodies(ir)?;
        
        // Return the LLVM IR as string
        Ok(self.module.print_to_string().to_string())
    }

    /// Declare external functions
    fn declare_external_functions(&mut self) -> OvieResult<()> {
        let i32_type = self.context.i32_type();
        let i8_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
        
        // Declare printf function: i32 printf(i8*, ...)
        let printf_type = i32_type.fn_type(&[i8_ptr_type.into()], true);
        self.module.add_function("printf", printf_type, None);
        
        Ok(())
    }

    /// Generate LLVM function declarations
    fn generate_function_declarations(&mut self, ir: &Program) -> OvieResult<()> {
        let void_type = self.context.void_type();
        
        for (ir_id, function) in &ir.functions {
            // For now, all functions are void -> void
            let fn_type = void_type.fn_type(&[], false);
            let llvm_function = self.module.add_function(&function.name, fn_type, None);
            
            self.functions.insert(*ir_id, llvm_function);
        }
        
        Ok(())
    }

    /// Generate LLVM function bodies
    fn generate_function_bodies(&mut self, ir: &Program) -> OvieResult<()> {
        for (ir_id, function) in &ir.functions {
            let llvm_function = self.functions[ir_id];
            self.current_function = Some(llvm_function);
            
            // Create entry basic block
            let entry_block = self.context.append_basic_block(llvm_function, "entry");
            self.builder.position_at_end(entry_block);
            
            // Generate code for the entry block
            let ir_entry_block = function.basic_blocks.get(&function.entry_block)
                .ok_or_else(|| OvieError::CodegenError { 
                    message: "Entry block not found".to_string() 
                })?;
            
            self.generate_block_code(ir_entry_block)?;
        }
        
        Ok(())
    }

    /// Generate LLVM code for a basic block
    fn generate_block_code(&mut self, block: &BasicBlock) -> OvieResult<()> {
        // Generate instructions
        for instruction in &block.instructions {
            self.generate_instruction_code(instruction)?;
        }
        
        // Generate terminator
        self.generate_terminator_code(&block.terminator)?;
        
        Ok(())
    }

    /// Generate LLVM code for an instruction
    fn generate_instruction_code(&mut self, instruction: &Instruction) -> OvieResult<()> {
        match instruction.opcode {
            Opcode::Print => {
                if let Some(operand) = instruction.operands.first() {
                    self.generate_print_call(operand)?;
                }
            }
            Opcode::Add => {
                if instruction.operands.len() >= 2 {
                    let left = self.generate_value_code(&instruction.operands[0])?;
                    let right = self.generate_value_code(&instruction.operands[1])?;
                    
                    if let (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) = (left, right) {
                        let _result = self.builder.build_int_add(l, r, "add_tmp");
                    }
                }
            }
            Opcode::Sub => {
                if instruction.operands.len() >= 2 {
                    let left = self.generate_value_code(&instruction.operands[0])?;
                    let right = self.generate_value_code(&instruction.operands[1])?;
                    
                    if let (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) = (left, right) {
                        let _result = self.builder.build_int_sub(l, r, "sub_tmp");
                    }
                }
            }
            Opcode::Mul => {
                if instruction.operands.len() >= 2 {
                    let left = self.generate_value_code(&instruction.operands[0])?;
                    let right = self.generate_value_code(&instruction.operands[1])?;
                    
                    if let (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) = (left, right) {
                        let _result = self.builder.build_int_mul(l, r, "mul_tmp");
                    }
                }
            }
            Opcode::Div => {
                if instruction.operands.len() >= 2 {
                    let left = self.generate_value_code(&instruction.operands[0])?;
                    let right = self.generate_value_code(&instruction.operands[1])?;
                    
                    if let (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) = (left, right) {
                        let _result = self.builder.build_int_signed_div(l, r, "div_tmp");
                    }
                }
            }
            _ => {
                // For now, ignore other opcodes
            }
        }
        
        Ok(())
    }

    /// Generate LLVM code for a print call
    fn generate_print_call(&mut self, value: &Value) -> OvieResult<()> {
        let printf_fn = self.module.get_function("printf")
            .ok_or_else(|| OvieError::CodegenError { 
                message: "printf function not found".to_string() 
            })?;
        
        match value {
            Value::Constant(Constant::String(s)) => {
                // Create a global string constant
                let format_str = format!("{}\n\0", s);
                let global_string = self.builder.build_global_string_ptr(&format_str, "str");
                
                // Call printf
                self.builder.build_call(printf_fn, &[global_string.as_pointer_value().into()], "printf_call");
            }
            Value::Constant(Constant::Number(n)) => {
                // Create format string for number
                let format_str = "%d\n\0";
                let global_string = self.builder.build_global_string_ptr(format_str, "num_fmt");
                let int_val = self.context.i32_type().const_int(*n as u64, false);
                
                // Call printf
                self.builder.build_call(printf_fn, &[
                    global_string.as_pointer_value().into(),
                    int_val.into()
                ], "printf_call");
            }
            _ => {
                // For other types, just print a placeholder
                let format_str = "value\n\0";
                let global_string = self.builder.build_global_string_ptr(format_str, "val_fmt");
                self.builder.build_call(printf_fn, &[global_string.as_pointer_value().into()], "printf_call");
            }
        }
        
        Ok(())
    }

    /// Generate LLVM code for a value
    fn generate_value_code(&mut self, value: &Value) -> OvieResult<BasicValueEnum<'ctx>> {
        match value {
            Value::Constant(constant) => {
                match constant {
                    Constant::Number(n) => {
                        let int_val = self.context.i32_type().const_int(*n as u64, false);
                        Ok(int_val.into())
                    }
                    Constant::Boolean(b) => {
                        let int_val = self.context.i32_type().const_int(if *b { 1 } else { 0 }, false);
                        Ok(int_val.into())
                    }
                    _ => {
                        // For other constants, return 0
                        let int_val = self.context.i32_type().const_int(0, false);
                        Ok(int_val.into())
                    }
                }
            }
            _ => {
                // For other values, return 0 for now
                let int_val = self.context.i32_type().const_int(0, false);
                Ok(int_val.into())
            }
        }
    }

    /// Generate LLVM code for a terminator
    fn generate_terminator_code(&mut self, terminator: &Terminator) -> OvieResult<()> {
        match terminator {
            Terminator::Return { value: _ } => {
                // For void functions, just return
                self.builder.build_return(None);
            }
            Terminator::Branch { target: _ } => {
                // For now, just return (simplified)
                self.builder.build_return(None);
            }
            Terminator::ConditionalBranch { condition: _, true_target: _, false_target: _ } => {
                // For now, just return (simplified)
                self.builder.build_return(None);
            }
            Terminator::Unreachable => {
                self.builder.build_unreachable();
            }
        }
        
        Ok(())
    }

    /// Get the LLVM module
    pub fn module(&self) -> &Module<'ctx> {
        &self.module
    }
}

impl<'ctx> CodegenBackend for LlvmBackend<'ctx> {
    type Output = String;
    type Error = OvieError;

    fn generate(&mut self, ir: &Program) -> Result<Self::Output, Self::Error> {
        self.generate_llvm_ir(ir)
    }

    fn name(&self) -> &'static str {
        "llvm"
    }

    fn supports_target(&self, target: &str) -> bool {
        matches!(target, 
            "x86_64-unknown-linux-gnu" | 
            "x86_64-pc-windows-msvc" | 
            "x86_64-apple-darwin" |
            "aarch64-unknown-linux-gnu" |
            "aarch64-apple-darwin" |
            "llvm"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{IrBuilder, IrType};

    #[test]
    fn test_llvm_backend_creation() {
        let context = Context::create();
        let backend = LlvmBackend::new(&context, "test_module");
        
        assert_eq!(backend.name(), "llvm");
        assert!(backend.supports_target("x86_64-unknown-linux-gnu"));
        assert!(backend.supports_target("llvm"));
        assert!(!backend.supports_target("wasm"));
    }

    #[test]
    fn test_simple_llvm_generation() {
        let context = Context::create();
        let mut backend = LlvmBackend::new(&context, "test_module");
        
        let mut ir_builder = IrBuilder::new();
        let main_func = ir_builder.create_function("main", Vec::new(), IrType::Void);
        ir_builder.set_entry_point(main_func);
        
        let ir = ir_builder.build();
        
        let result = backend.generate(&ir);
        assert!(result.is_ok());
        
        let llvm_ir = result.unwrap();
        assert!(!llvm_ir.is_empty());
        assert!(llvm_ir.contains("define void @main()"));
    }

    #[test]
    fn test_compile_to_llvm() {
        let source = r#"seeAm "Hello LLVM!";"#;
        let compiler = crate::Compiler::new();
        
        let result = compiler.compile_to_llvm(source);
        assert!(result.is_ok());
        
        let llvm_ir = result.unwrap();
        assert!(!llvm_ir.is_empty());
        assert!(llvm_ir.contains("@main"));
        assert!(llvm_ir.contains("printf"));
    }
}