//! WebAssembly code generation backend

use crate::ir::{Program, Function, BasicBlock, Instruction, Terminator, Opcode, Value, Constant};
use crate::error::{OvieError, OvieResult};
use super::CodegenBackend;
use wasm_encoder::*;
use std::collections::HashMap;

/// WebAssembly code generation backend
pub struct WasmBackend {
    /// Module being built
    module: Module,
    /// Function type indices
    function_types: HashMap<String, u32>,
    /// Function indices
    function_indices: HashMap<u32, u32>, // IR function ID -> WASM function index
    /// Next available type index
    next_type_index: u32,
    /// Next available function index
    next_function_index: u32,
    /// Deterministic mode for reproducible builds
    deterministic_mode: bool,
}

impl WasmBackend {
    /// Create a new WASM backend
    pub fn new() -> Self {
        Self {
            module: Module::new(),
            function_types: HashMap::new(),
            function_indices: HashMap::new(),
            next_type_index: 0,
            next_function_index: 0,
            deterministic_mode: false,
        }
    }

    /// Set deterministic mode for reproducible builds
    pub fn set_deterministic_mode(&mut self, enabled: bool) {
        self.deterministic_mode = enabled;
    }

    /// Generate WASM module from IR
    fn generate_module(&mut self, ir: &Program) -> OvieResult<Vec<u8>> {
        // Add type section
        self.add_types(ir)?;
        
        // Add import section (for print function)
        self.add_imports()?;
        
        // Add function section
        self.add_functions(ir)?;
        
        // Add export section
        self.add_exports(ir)?;
        
        // Add code section
        self.add_code(ir)?;
        
        // Clone the module to avoid move issues
        let module = std::mem::replace(&mut self.module, Module::new());
        Ok(module.finish())
    }

    /// Add type definitions
    fn add_types(&mut self, ir: &Program) -> OvieResult<()> {
        let mut types = TypeSection::new();
        
        // Add print function type: (i32) -> ()
        types.function([ValType::I32], []);
        self.function_types.insert("print".to_string(), self.next_type_index);
        self.next_type_index += 1;
        
        // Add main function type: () -> ()
        types.function([], []);
        self.function_types.insert("main".to_string(), self.next_type_index);
        self.next_type_index += 1;
        
        // Add types for other functions
        for (_, function) in &ir.functions {
            if function.name != "main" {
                // For now, assume all functions take no parameters and return nothing
                types.function([], []);
                self.function_types.insert(function.name.clone(), self.next_type_index);
                self.next_type_index += 1;
            }
        }
        
        self.module.section(&types);
        Ok(())
    }

    /// Add import section
    fn add_imports(&mut self) -> OvieResult<()> {
        let mut imports = ImportSection::new();
        
        // Import print function from host
        imports.import(
            "env",
            "print",
            EntityType::Function(self.function_types["print"])
        );
        
        self.module.section(&imports);
        Ok(())
    }

    /// Add function declarations
    fn add_functions(&mut self, ir: &Program) -> OvieResult<()> {
        let mut functions = FunctionSection::new();
        
        // Add all IR functions
        for (ir_id, function) in &ir.functions {
            let type_index = self.function_types.get(&function.name)
                .ok_or_else(|| OvieError::CodegenError { 
                    message: format!("No type found for function {}", function.name) 
                })?;
            
            functions.function(*type_index);
            self.function_indices.insert(*ir_id, self.next_function_index);
            self.next_function_index += 1;
        }
        
        self.module.section(&functions);
        Ok(())
    }

    /// Add exports
    fn add_exports(&mut self, ir: &Program) -> OvieResult<()> {
        let mut exports = ExportSection::new();
        
        // Export main function if it exists
        if let Some(entry_id) = ir.entry_point {
            if let Some(&wasm_index) = self.function_indices.get(&entry_id) {
                exports.export("main", ExportKind::Func, wasm_index + 1); // +1 for imported print
            }
        }
        
        self.module.section(&exports);
        Ok(())
    }

    /// Add code section with function bodies
    fn add_code(&mut self, ir: &Program) -> OvieResult<()> {
        let mut code = CodeSection::new();
        
        for (ir_id, function) in &ir.functions {
            let mut func_body = wasm_encoder::Function::new([]);
            
            // Generate code for the function
            self.generate_function_body(&mut func_body, function)?;
            
            code.function(&func_body);
        }
        
        self.module.section(&code);
        Ok(())
    }

    /// Generate WASM code for a function body
    fn generate_function_body(&mut self, func: &mut wasm_encoder::Function, ir_func: &Function) -> OvieResult<()> {
        // Get the entry block
        let entry_block = ir_func.basic_blocks.get(&ir_func.entry_block)
            .ok_or_else(|| OvieError::CodegenError { 
                message: "Entry block not found".to_string() 
            })?;
        
        // Generate code for the entry block
        self.generate_block_code(func, entry_block)?;
        
        Ok(())
    }

    /// Generate WASM code for a basic block
    fn generate_block_code(&mut self, func: &mut wasm_encoder::Function, block: &BasicBlock) -> OvieResult<()> {
        // Generate instructions
        for instruction in &block.instructions {
            self.generate_instruction_code(func, instruction)?;
        }
        
        // Generate terminator
        self.generate_terminator_code(func, &block.terminator)?;
        
        Ok(())
    }

    /// Generate WASM code for an instruction
    fn generate_instruction_code(&mut self, func: &mut wasm_encoder::Function, instruction: &Instruction) -> OvieResult<()> {
        match instruction.opcode {
            Opcode::Print => {
                // For print, we need to get the value and call the imported print function
                if let Some(operand) = instruction.operands.first() {
                    self.generate_value_code(func, operand)?;
                    func.instruction(&wasm_encoder::Instruction::Call(0)); // Call imported print function
                }
            }
            Opcode::Add => {
                // Generate code for operands
                if instruction.operands.len() >= 2 {
                    self.generate_value_code(func, &instruction.operands[0])?;
                    self.generate_value_code(func, &instruction.operands[1])?;
                    func.instruction(&wasm_encoder::Instruction::I32Add);
                }
            }
            Opcode::Sub => {
                if instruction.operands.len() >= 2 {
                    self.generate_value_code(func, &instruction.operands[0])?;
                    self.generate_value_code(func, &instruction.operands[1])?;
                    func.instruction(&wasm_encoder::Instruction::I32Sub);
                }
            }
            Opcode::Mul => {
                if instruction.operands.len() >= 2 {
                    self.generate_value_code(func, &instruction.operands[0])?;
                    self.generate_value_code(func, &instruction.operands[1])?;
                    func.instruction(&wasm_encoder::Instruction::I32Mul);
                }
            }
            Opcode::Div => {
                if instruction.operands.len() >= 2 {
                    self.generate_value_code(func, &instruction.operands[0])?;
                    self.generate_value_code(func, &instruction.operands[1])?;
                    func.instruction(&wasm_encoder::Instruction::I32DivS);
                }
            }
            _ => {
                // For now, ignore other opcodes
            }
        }
        
        Ok(())
    }

    /// Generate WASM code for a value
    fn generate_value_code(&mut self, func: &mut wasm_encoder::Function, value: &Value) -> OvieResult<()> {
        match value {
            Value::Constant(constant) => {
                match constant {
                    Constant::Number(n) => {
                        func.instruction(&wasm_encoder::Instruction::I32Const(*n as i32));
                    }
                    Constant::String(_s) => {
                        // For now, represent strings as integers (simplified)
                        func.instruction(&wasm_encoder::Instruction::I32Const(42)); // Placeholder
                    }
                    Constant::Boolean(b) => {
                        func.instruction(&wasm_encoder::Instruction::I32Const(if *b { 1 } else { 0 }));
                    }
                    Constant::Void => {
                        func.instruction(&wasm_encoder::Instruction::I32Const(0));
                    }
                }
            }
            Value::Instruction(_) => {
                // For now, just push a placeholder value
                func.instruction(&wasm_encoder::Instruction::I32Const(0));
            }
            Value::Global(_) => {
                // For now, just push a placeholder value
                func.instruction(&wasm_encoder::Instruction::I32Const(0));
            }
            Value::Parameter(_) => {
                // For now, just push a placeholder value
                func.instruction(&wasm_encoder::Instruction::I32Const(0));
            }
        }
        
        Ok(())
    }

    /// Generate WASM code for a terminator
    fn generate_terminator_code(&mut self, func: &mut wasm_encoder::Function, terminator: &Terminator) -> OvieResult<()> {
        match terminator {
            Terminator::Return { value: _ } => {
                // For void functions, just return
                func.instruction(&wasm_encoder::Instruction::Return);
            }
            Terminator::Branch { target: _ } => {
                // For now, just return (simplified)
                func.instruction(&wasm_encoder::Instruction::Return);
            }
            Terminator::ConditionalBranch { condition: _, true_target: _, false_target: _ } => {
                // For now, just return (simplified)
                func.instruction(&wasm_encoder::Instruction::Return);
            }
            Terminator::Unreachable => {
                func.instruction(&wasm_encoder::Instruction::Unreachable);
            }
        }
        
        Ok(())
    }
}

impl CodegenBackend for WasmBackend {
    type Output = Vec<u8>;
    type Error = OvieError;

    fn generate(&mut self, ir: &Program) -> Result<Self::Output, Self::Error> {
        self.generate_module(ir)
    }

    fn name(&self) -> &'static str {
        "wasm"
    }

    fn supports_target(&self, target: &str) -> bool {
        matches!(target, "wasm32-unknown-unknown" | "wasm" | "webassembly")
    }
}

impl Default for WasmBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{IrBuilder, IrType};

    #[test]
    fn test_wasm_backend_creation() {
        let backend = WasmBackend::new();
        assert_eq!(backend.name(), "wasm");
        assert!(backend.supports_target("wasm"));
        assert!(backend.supports_target("wasm32-unknown-unknown"));
        assert!(!backend.supports_target("x86_64-unknown-linux-gnu"));
    }

    #[test]
    fn test_compile_to_wasm() {
        let source = r#"seeAm "Hello WASM!";"#;
        let mut compiler = crate::Compiler::new();
        
        let result = compiler.compile_to_wasm(source);
        assert!(result.is_ok());
        
        let wasm_bytes = result.unwrap();
        assert!(!wasm_bytes.is_empty());
        
        // Verify WASM magic number
        assert_eq!(&wasm_bytes[0..4], &[0x00, 0x61, 0x73, 0x6d]);
    }
}