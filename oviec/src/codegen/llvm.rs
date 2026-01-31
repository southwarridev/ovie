//! LLVM code generation backend with enhanced native code generation

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
use inkwell::values::{FunctionValue, BasicValueEnum, IntValue, PointerValue};
use inkwell::types::{BasicTypeEnum, IntType, FunctionType};
use inkwell::{IntPredicate, OptimizationLevel, AddressSpace};
use inkwell::targets::{Target, TargetMachine, RelocMode, CodeModel, FileType};
use std::collections::HashMap;
use std::path::Path;

/// Target platform configuration
#[derive(Debug, Clone)]
pub struct TargetConfig {
    /// Target triple (e.g., "x86_64-unknown-linux-gnu")
    pub triple: String,
    /// CPU features to enable
    pub cpu_features: Vec<String>,
    /// Optimization level
    pub optimization_level: OptimizationLevel,
    /// Position independent code
    pub pic: bool,
    /// Code model
    pub code_model: CodeModel,
    /// Relocation mode
    pub reloc_mode: RelocMode,
}

impl Default for TargetConfig {
    fn default() -> Self {
        Self {
            triple: "x86_64-unknown-linux-gnu".to_string(),
            cpu_features: Vec::new(),
            optimization_level: OptimizationLevel::Default,
            pic: true,
            code_model: CodeModel::Default,
            reloc_mode: RelocMode::PIC,
        }
    }
}

impl TargetConfig {
    /// Create configuration for Windows x64
    pub fn windows_x64() -> Self {
        Self {
            triple: "x86_64-pc-windows-msvc".to_string(),
            cpu_features: vec!["sse2".to_string(), "sse3".to_string()],
            optimization_level: OptimizationLevel::Default,
            pic: false,
            code_model: CodeModel::Default,
            reloc_mode: RelocMode::Static,
        }
    }

    /// Create configuration for Linux x64
    pub fn linux_x64() -> Self {
        Self {
            triple: "x86_64-unknown-linux-gnu".to_string(),
            cpu_features: vec!["sse2".to_string(), "sse3".to_string(), "sse4.1".to_string()],
            optimization_level: OptimizationLevel::Default,
            pic: true,
            code_model: CodeModel::Default,
            reloc_mode: RelocMode::PIC,
        }
    }

    /// Create configuration for macOS x64
    pub fn macos_x64() -> Self {
        Self {
            triple: "x86_64-apple-darwin".to_string(),
            cpu_features: vec!["sse2".to_string(), "sse3".to_string(), "sse4.1".to_string()],
            optimization_level: OptimizationLevel::Default,
            pic: true,
            code_model: CodeModel::Default,
            reloc_mode: RelocMode::PIC,
        }
    }

    /// Create configuration for ARM64 Linux
    pub fn arm64_linux() -> Self {
        Self {
            triple: "aarch64-unknown-linux-gnu".to_string(),
            cpu_features: vec!["neon".to_string()],
            optimization_level: OptimizationLevel::Default,
            pic: true,
            code_model: CodeModel::Default,
            reloc_mode: RelocMode::PIC,
        }
    }

    /// Create configuration for ARM64 macOS
    pub fn arm64_macos() -> Self {
        Self {
            triple: "aarch64-apple-darwin".to_string(),
            cpu_features: vec!["neon".to_string()],
            optimization_level: OptimizationLevel::Default,
            pic: true,
            code_model: CodeModel::Default,
            reloc_mode: RelocMode::PIC,
        }
    }
}

/// ABI calling convention information
#[derive(Debug, Clone)]
pub struct AbiInfo {
    /// Calling convention for the target
    pub calling_convention: u32,
    /// Stack alignment requirements
    pub stack_alignment: u32,
    /// Register usage for parameters
    pub param_registers: Vec<String>,
    /// Return value handling
    pub return_handling: ReturnHandling,
}

#[derive(Debug, Clone)]
pub enum ReturnHandling {
    /// Return in register
    Register,
    /// Return via stack
    Stack,
    /// Return via hidden parameter
    HiddenParameter,
}

impl AbiInfo {
    /// Get ABI info for System V (Linux/macOS)
    pub fn system_v() -> Self {
        Self {
            calling_convention: 0, // Default calling convention
            stack_alignment: 16,
            param_registers: vec![
                "rdi".to_string(), "rsi".to_string(), "rdx".to_string(),
                "rcx".to_string(), "r8".to_string(), "r9".to_string(),
            ],
            return_handling: ReturnHandling::Register,
        }
    }

    /// Get ABI info for Windows x64
    pub fn windows_x64() -> Self {
        Self {
            calling_convention: 0, // Default calling convention
            stack_alignment: 16,
            param_registers: vec![
                "rcx".to_string(), "rdx".to_string(), "r8".to_string(), "r9".to_string(),
            ],
            return_handling: ReturnHandling::Register,
        }
    }

    /// Get ABI info for ARM64
    pub fn arm64() -> Self {
        Self {
            calling_convention: 0, // Default calling convention
            stack_alignment: 16,
            param_registers: vec![
                "x0".to_string(), "x1".to_string(), "x2".to_string(), "x3".to_string(),
                "x4".to_string(), "x5".to_string(), "x6".to_string(), "x7".to_string(),
            ],
            return_handling: ReturnHandling::Register,
        }
    }
}
/// Enhanced LLVM code generation backend with native code generation
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
    /// Target configuration
    target_config: TargetConfig,
    /// ABI information
    abi_info: AbiInfo,
    /// Variable mapping for current function
    variables: HashMap<String, PointerValue<'ctx>>,
    /// Target machine for native code generation
    target_machine: Option<TargetMachine>,
}

impl<'ctx> LlvmBackend<'ctx> {
    /// Create a new LLVM backend with default target
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        let target_config = TargetConfig::default();
        let abi_info = AbiInfo::system_v();
        
        Self {
            context,
            module,
            builder,
            functions: HashMap::new(),
            current_function: None,
            deterministic_mode: false,
            target_config,
            abi_info,
            variables: HashMap::new(),
            target_machine: None,
        }
    }

    /// Create a new LLVM backend with specific target configuration
    pub fn new_with_target(context: &'ctx Context, module_name: &str, target_config: TargetConfig) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        
        // Select ABI based on target
        let abi_info = if target_config.triple.contains("windows") {
            AbiInfo::windows_x64()
        } else if target_config.triple.contains("aarch64") {
            AbiInfo::arm64()
        } else {
            AbiInfo::system_v()
        };
        
        Self {
            context,
            module,
            builder,
            functions: HashMap::new(),
            current_function: None,
            deterministic_mode: false,
            target_config,
            abi_info,
            variables: HashMap::new(),
            target_machine: None,
        }
    }

    /// Set deterministic mode for reproducible builds
    pub fn set_deterministic_mode(&mut self, enabled: bool) {
        self.deterministic_mode = enabled;
    }

    /// Initialize target machine for native code generation
    pub fn initialize_target_machine(&mut self) -> OvieResult<()> {
        Target::initialize_all(&inkwell::targets::InitializationConfig::default());
        
        let target = Target::from_triple(&self.target_config.triple)
            .map_err(|e| OvieError::CodegenError { 
                message: format!("Failed to create target: {}", e) 
            })?;
        
        let cpu = "generic";
        let features = self.target_config.cpu_features.join(",");
        
        let target_machine = target.create_target_machine(
            &self.target_config.triple,
            cpu,
            &features,
            self.target_config.optimization_level,
            self.target_config.reloc_mode,
            self.target_config.code_model,
        ).ok_or_else(|| OvieError::CodegenError { 
            message: "Failed to create target machine".to_string() 
        })?;
        
        // Set target data layout
        let data_layout = target_machine.get_target_data().get_data_layout();
        self.module.set_data_layout(&data_layout);
        self.module.set_triple(&self.target_config.triple);
        
        self.target_machine = Some(target_machine);
        Ok(())
    }

    /// Generate native object file
    pub fn generate_object_file(&self, output_path: &Path) -> OvieResult<()> {
        let target_machine = self.target_machine.as_ref()
            .ok_or_else(|| OvieError::CodegenError { 
                message: "Target machine not initialized".to_string() 
            })?;
        
        target_machine.write_to_file(&self.module, FileType::Object, output_path)
            .map_err(|e| OvieError::CodegenError { 
                message: format!("Failed to write object file: {}", e) 
            })?;
        
        Ok(())
    }

    /// Generate native assembly file
    pub fn generate_assembly_file(&self, output_path: &Path) -> OvieResult<()> {
        let target_machine = self.target_machine.as_ref()
            .ok_or_else(|| OvieError::CodegenError { 
                message: "Target machine not initialized".to_string() 
            })?;
        
        target_machine.write_to_file(&self.module, FileType::Assembly, output_path)
            .map_err(|e| OvieError::CodegenError { 
                message: format!("Failed to write assembly file: {}", e) 
            })?;
        
        Ok(())
    }

    /// Apply platform-specific optimizations
    pub fn apply_platform_optimizations(&mut self) -> OvieResult<()> {
        // Add platform-specific function attributes
        for (_, function) in &self.functions {
            // Set calling convention based on ABI
            function.set_call_conventions(self.abi_info.calling_convention);
            
            // Add target-specific attributes
            if self.target_config.triple.contains("x86_64") {
                // Enable SSE2 for x86_64
                function.add_attribute(inkwell::attributes::AttributeLoc::Function, 
                    self.context.create_string_attribute("target-features", "+sse2"));
            } else if self.target_config.triple.contains("aarch64") {
                // Enable NEON for ARM64
                function.add_attribute(inkwell::attributes::AttributeLoc::Function, 
                    self.context.create_string_attribute("target-features", "+neon"));
            }
            
            // Add stack alignment attribute
            if self.abi_info.stack_alignment > 0 {
                function.add_attribute(inkwell::attributes::AttributeLoc::Function,
                    self.context.create_string_attribute("stackrealign", ""));
            }
        }
        
        Ok(())
    }

    /// Generate LLVM IR from Ovie IR with enhanced native code generation
    fn generate_llvm_ir(&mut self, ir: &Program) -> OvieResult<String> {
        // Initialize target machine
        self.initialize_target_machine()?;
        
        // Declare external functions (like printf)
        self.declare_external_functions()?;
        
        // Generate function declarations
        self.generate_function_declarations(ir)?;
        
        // Generate function bodies
        self.generate_function_bodies(ir)?;
        
        // Apply platform-specific optimizations
        self.apply_platform_optimizations()?;
        
        // Run optimization passes if not in deterministic mode
        if !self.deterministic_mode {
            self.run_optimization_passes()?;
        }
        
        // Return the LLVM IR as string
        Ok(self.module.print_to_string().to_string())
    }

    /// Run LLVM optimization passes
    fn run_optimization_passes(&mut self) -> OvieResult<()> {
        use inkwell::passes::{PassManager, PassManagerBuilder};
        
        let pass_manager_builder = PassManagerBuilder::create();
        pass_manager_builder.set_optimization_level(self.target_config.optimization_level);
        
        let function_pass_manager = PassManager::create(&self.module);
        pass_manager_builder.populate_function_pass_manager(&function_pass_manager);
        
        // Initialize and run function passes
        function_pass_manager.initialize();
        for (_, function) in &self.functions {
            function_pass_manager.run_on(function);
        }
        function_pass_manager.finalize();
        
        // Run module passes
        let module_pass_manager = PassManager::create(());
        pass_manager_builder.populate_module_pass_manager(&module_pass_manager);
        module_pass_manager.run_on(&self.module);
        
        Ok(())
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

    /// Generate LLVM function declarations with proper ABI
    fn generate_function_declarations(&mut self, ir: &Program) -> OvieResult<()> {
        for (ir_id, function) in &ir.functions {
            // Create function type based on ABI
            let fn_type = self.create_function_type(&function.name, &[], "void")?;
            let llvm_function = self.module.add_function(&function.name, fn_type, None);
            
            // Set calling convention
            llvm_function.set_call_conventions(self.abi_info.calling_convention);
            
            // Add function attributes for better optimization
            llvm_function.add_attribute(inkwell::attributes::AttributeLoc::Function,
                self.context.create_enum_attribute(inkwell::attributes::Attribute::get_named_enum_kind_id("nounwind"), 0));
            
            self.functions.insert(*ir_id, llvm_function);
        }
        
        Ok(())
    }

    /// Create LLVM function type with proper ABI handling
    fn create_function_type(&self, _name: &str, _params: &[&str], return_type: &str) -> OvieResult<FunctionType<'ctx>> {
        let return_llvm_type = match return_type {
            "void" => None,
            "i32" => Some(self.context.i32_type().into()),
            "i64" => Some(self.context.i64_type().into()),
            "f32" => Some(self.context.f32_type().into()),
            "f64" => Some(self.context.f64_type().into()),
            _ => None,
        };
        
        // For now, all functions are void -> void
        let fn_type = match return_llvm_type {
            Some(ret_type) => ret_type.fn_type(&[], false),
            None => self.context.void_type().fn_type(&[], false),
        };
        
        Ok(fn_type)
    }

    /// Generate LLVM function bodies with enhanced instruction support
    fn generate_function_bodies(&mut self, ir: &Program) -> OvieResult<()> {
        for (ir_id, function) in &ir.functions {
            let llvm_function = self.functions[ir_id];
            self.current_function = Some(llvm_function);
            self.variables.clear(); // Clear variables for new function
            
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

    /// Generate LLVM code for an instruction with enhanced support
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
                    
                    match (left, right) {
                        (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                            let _result = self.builder.build_int_add(l, r, "add_tmp");
                        }
                        (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                            let _result = self.builder.build_float_add(l, r, "fadd_tmp");
                        }
                        _ => return Err(OvieError::CodegenError { 
                            message: "Type mismatch in add operation".to_string() 
                        }),
                    }
                }
            }
            Opcode::Sub => {
                if instruction.operands.len() >= 2 {
                    let left = self.generate_value_code(&instruction.operands[0])?;
                    let right = self.generate_value_code(&instruction.operands[1])?;
                    
                    match (left, right) {
                        (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                            let _result = self.builder.build_int_sub(l, r, "sub_tmp");
                        }
                        (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                            let _result = self.builder.build_float_sub(l, r, "fsub_tmp");
                        }
                        _ => return Err(OvieError::CodegenError { 
                            message: "Type mismatch in sub operation".to_string() 
                        }),
                    }
                }
            }
            Opcode::Mul => {
                if instruction.operands.len() >= 2 {
                    let left = self.generate_value_code(&instruction.operands[0])?;
                    let right = self.generate_value_code(&instruction.operands[1])?;
                    
                    match (left, right) {
                        (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                            let _result = self.builder.build_int_mul(l, r, "mul_tmp");
                        }
                        (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                            let _result = self.builder.build_float_mul(l, r, "fmul_tmp");
                        }
                        _ => return Err(OvieError::CodegenError { 
                            message: "Type mismatch in mul operation".to_string() 
                        }),
                    }
                }
            }
            Opcode::Div => {
                if instruction.operands.len() >= 2 {
                    let left = self.generate_value_code(&instruction.operands[0])?;
                    let right = self.generate_value_code(&instruction.operands[1])?;
                    
                    match (left, right) {
                        (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                            let _result = self.builder.build_int_signed_div(l, r, "div_tmp");
                        }
                        (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                            let _result = self.builder.build_float_div(l, r, "fdiv_tmp");
                        }
                        _ => return Err(OvieError::CodegenError { 
                            message: "Type mismatch in div operation".to_string() 
                        }),
                    }
                }
            }
            Opcode::Load => {
                if let Some(operand) = instruction.operands.first() {
                    let _loaded_value = self.generate_load_instruction(operand)?;
                }
            }
            Opcode::Store => {
                if instruction.operands.len() >= 2 {
                    self.generate_store_instruction(&instruction.operands[0], &instruction.operands[1])?;
                }
            }
            Opcode::Alloca => {
                if let Some(operand) = instruction.operands.first() {
                    self.generate_alloca_instruction(operand)?;
                }
            }
            Opcode::Call => {
                if !instruction.operands.is_empty() {
                    self.generate_call_instruction(&instruction.operands)?;
                }
            }
            Opcode::Compare => {
                if instruction.operands.len() >= 2 {
                    let _result = self.generate_compare_instruction(&instruction.operands[0], &instruction.operands[1])?;
                }
            }
            _ => {
                // For now, ignore other opcodes
            }
        }
        
        Ok(())
    }

    /// Generate load instruction
    fn generate_load_instruction(&mut self, operand: &Value) -> OvieResult<BasicValueEnum<'ctx>> {
        match operand {
            Value::Variable(name) => {
                if let Some(ptr) = self.variables.get(name) {
                    let loaded = self.builder.build_load(self.context.i32_type(), *ptr, &format!("{}_load", name));
                    Ok(loaded)
                } else {
                    Err(OvieError::CodegenError { 
                        message: format!("Variable {} not found", name) 
                    })
                }
            }
            _ => Err(OvieError::CodegenError { 
                message: "Invalid operand for load instruction".to_string() 
            }),
        }
    }

    /// Generate store instruction
    fn generate_store_instruction(&mut self, value: &Value, target: &Value) -> OvieResult<()> {
        let val = self.generate_value_code(value)?;
        
        match target {
            Value::Variable(name) => {
                if let Some(ptr) = self.variables.get(name) {
                    self.builder.build_store(*ptr, val);
                    Ok(())
                } else {
                    Err(OvieError::CodegenError { 
                        message: format!("Variable {} not found", name) 
                    })
                }
            }
            _ => Err(OvieError::CodegenError { 
                message: "Invalid target for store instruction".to_string() 
            }),
        }
    }

    /// Generate alloca instruction
    fn generate_alloca_instruction(&mut self, operand: &Value) -> OvieResult<()> {
        match operand {
            Value::Variable(name) => {
                let alloca = self.builder.build_alloca(self.context.i32_type(), name);
                self.variables.insert(name.clone(), alloca);
                Ok(())
            }
            _ => Err(OvieError::CodegenError { 
                message: "Invalid operand for alloca instruction".to_string() 
            }),
        }
    }

    /// Generate call instruction
    fn generate_call_instruction(&mut self, operands: &[Value]) -> OvieResult<BasicValueEnum<'ctx>> {
        if let Value::Function(func_name) = &operands[0] {
            if let Some(function) = self.module.get_function(func_name) {
                let args: Result<Vec<_>, _> = operands[1..].iter()
                    .map(|arg| self.generate_value_code(arg))
                    .collect();
                
                let args = args?;
                let args_refs: Vec<_> = args.iter().map(|v| (*v).into()).collect();
                
                let call_result = self.builder.build_call(function, &args_refs, "call_tmp");
                
                if let Some(result) = call_result.try_as_basic_value().left() {
                    Ok(result)
                } else {
                    // Void function, return a dummy value
                    Ok(self.context.i32_type().const_int(0, false).into())
                }
            } else {
                Err(OvieError::CodegenError { 
                    message: format!("Function {} not found", func_name) 
                })
            }
        } else {
            Err(OvieError::CodegenError { 
                message: "Invalid function operand for call instruction".to_string() 
            })
        }
    }

    /// Generate compare instruction
    fn generate_compare_instruction(&mut self, left: &Value, right: &Value) -> OvieResult<BasicValueEnum<'ctx>> {
        let left_val = self.generate_value_code(left)?;
        let right_val = self.generate_value_code(right)?;
        
        match (left_val, right_val) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = self.builder.build_int_compare(IntPredicate::EQ, l, r, "cmp_tmp");
                Ok(result.into())
            }
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                let result = self.builder.build_float_compare(inkwell::FloatPredicate::OEQ, l, r, "fcmp_tmp");
                Ok(result.into())
            }
            _ => Err(OvieError::CodegenError { 
                message: "Type mismatch in compare operation".to_string() 
            }),
        }
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

    /// Generate LLVM code for a value with enhanced type support
    fn generate_value_code(&mut self, value: &Value) -> OvieResult<BasicValueEnum<'ctx>> {
        match value {
            Value::Constant(constant) => {
                match constant {
                    Constant::Number(n) => {
                        let int_val = self.context.i32_type().const_int(*n as u64, false);
                        Ok(int_val.into())
                    }
                    Constant::Float(f) => {
                        let float_val = self.context.f64_type().const_float(*f);
                        Ok(float_val.into())
                    }
                    Constant::Boolean(b) => {
                        let int_val = self.context.i1_type().const_int(if *b { 1 } else { 0 }, false);
                        Ok(int_val.into())
                    }
                    Constant::String(s) => {
                        // Create a global string constant
                        let global_string = self.builder.build_global_string_ptr(s, "str_const");
                        Ok(global_string.as_pointer_value().into())
                    }
                    _ => {
                        // For other constants, return 0
                        let int_val = self.context.i32_type().const_int(0, false);
                        Ok(int_val.into())
                    }
                }
            }
            Value::Variable(name) => {
                if let Some(ptr) = self.variables.get(name) {
                    let loaded = self.builder.build_load(self.context.i32_type(), *ptr, &format!("{}_val", name));
                    Ok(loaded)
                } else {
                    // Variable not found, return 0 for now
                    let int_val = self.context.i32_type().const_int(0, false);
                    Ok(int_val.into())
                }
            }
            Value::Function(_) => {
                // Function references not supported in value context
                let int_val = self.context.i32_type().const_int(0, false);
                Ok(int_val.into())
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
            "x86_64-pc-windows-gnu" |
            "x86_64-apple-darwin" |
            "aarch64-unknown-linux-gnu" |
            "aarch64-apple-darwin" |
            "i686-unknown-linux-gnu" |
            "i686-pc-windows-msvc" |
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
        assert!(backend.supports_target("x86_64-pc-windows-gnu"));
        assert!(backend.supports_target("aarch64-apple-darwin"));
        assert!(backend.supports_target("llvm"));
        assert!(!backend.supports_target("wasm"));
    }

    #[test]
    fn test_target_configurations() {
        let windows_config = TargetConfig::windows_x64();
        assert_eq!(windows_config.triple, "x86_64-pc-windows-msvc");
        assert!(!windows_config.pic);
        assert_eq!(windows_config.reloc_mode, RelocMode::Static);

        let linux_config = TargetConfig::linux_x64();
        assert_eq!(linux_config.triple, "x86_64-unknown-linux-gnu");
        assert!(linux_config.pic);
        assert_eq!(linux_config.reloc_mode, RelocMode::PIC);

        let arm64_config = TargetConfig::arm64_linux();
        assert_eq!(arm64_config.triple, "aarch64-unknown-linux-gnu");
        assert!(arm64_config.cpu_features.contains(&"neon".to_string()));
    }

    #[test]
    fn test_abi_configurations() {
        let system_v = AbiInfo::system_v();
        assert_eq!(system_v.stack_alignment, 16);
        assert!(system_v.param_registers.contains(&"rdi".to_string()));

        let windows_abi = AbiInfo::windows_x64();
        assert_eq!(windows_abi.stack_alignment, 16);
        assert!(windows_abi.param_registers.contains(&"rcx".to_string()));

        let arm64_abi = AbiInfo::arm64();
        assert_eq!(arm64_abi.stack_alignment, 16);
        assert!(arm64_abi.param_registers.contains(&"x0".to_string()));
    }

    #[test]
    fn test_enhanced_llvm_generation() {
        let context = Context::create();
        let target_config = TargetConfig::linux_x64();
        let mut backend = LlvmBackend::new_with_target(&context, "test_module", target_config);
        
        let mut ir_builder = IrBuilder::new();
        let main_func = ir_builder.create_function("main", Vec::new(), IrType::Void);
        ir_builder.set_entry_point(main_func);
        
        let ir = ir_builder.build();
        
        let result = backend.generate(&ir);
        assert!(result.is_ok());
        
        let llvm_ir = result.unwrap();
        assert!(!llvm_ir.is_empty());
        assert!(llvm_ir.contains("define void @main()"));
        assert!(llvm_ir.contains("target triple"));
    }

    #[test]
    fn test_platform_specific_features() {
        let context = Context::create();
        
        // Test Windows configuration
        let windows_config = TargetConfig::windows_x64();
        let windows_backend = LlvmBackend::new_with_target(&context, "windows_module", windows_config);
        assert_eq!(windows_backend.target_config.triple, "x86_64-pc-windows-msvc");
        
        // Test ARM64 configuration
        let arm64_config = TargetConfig::arm64_macos();
        let arm64_backend = LlvmBackend::new_with_target(&context, "arm64_module", arm64_config);
        assert_eq!(arm64_backend.target_config.triple, "aarch64-apple-darwin");
        assert!(arm64_backend.target_config.cpu_features.contains(&"neon".to_string()));
    }

    #[test]
    fn test_deterministic_mode() {
        let context = Context::create();
        let mut backend = LlvmBackend::new(&context, "test_module");
        
        assert!(!backend.deterministic_mode);
        
        backend.set_deterministic_mode(true);
        assert!(backend.deterministic_mode);
    }
}