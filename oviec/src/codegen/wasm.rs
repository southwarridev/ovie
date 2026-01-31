//! WebAssembly code generation backend with enhanced optimizations

use crate::ir::{Program, Function, BasicBlock, Instruction, Terminator, Opcode, Value, Constant};
use crate::error::{OvieError, OvieResult};
use super::CodegenBackend;
use wasm_encoder::*;
use std::collections::HashMap;

/// WebAssembly optimization configuration
#[derive(Debug, Clone)]
pub struct WasmOptimizationConfig {
    /// Enable size optimizations
    pub optimize_size: bool,
    /// Enable speed optimizations
    pub optimize_speed: bool,
    /// Enable dead code elimination
    pub dead_code_elimination: bool,
    /// Enable constant folding
    pub constant_folding: bool,
    /// Enable function inlining
    pub function_inlining: bool,
    /// Maximum function size for inlining
    pub inline_threshold: usize,
}

impl Default for WasmOptimizationConfig {
    fn default() -> Self {
        Self {
            optimize_size: true,
            optimize_speed: false,
            dead_code_elimination: true,
            constant_folding: true,
            function_inlining: false,
            inline_threshold: 50,
        }
    }
}

impl WasmOptimizationConfig {
    /// Create configuration optimized for size
    pub fn optimize_for_size() -> Self {
        Self {
            optimize_size: true,
            optimize_speed: false,
            dead_code_elimination: true,
            constant_folding: true,
            function_inlining: false,
            inline_threshold: 20,
        }
    }

    /// Create configuration optimized for speed
    pub fn optimize_for_speed() -> Self {
        Self {
            optimize_size: false,
            optimize_speed: true,
            dead_code_elimination: true,
            constant_folding: true,
            function_inlining: true,
            inline_threshold: 100,
        }
    }

    /// Create configuration with no optimizations
    pub fn no_optimizations() -> Self {
        Self {
            optimize_size: false,
            optimize_speed: false,
            dead_code_elimination: false,
            constant_folding: false,
            function_inlining: false,
            inline_threshold: 0,
        }
    }
}

/// WebAssembly target configuration
#[derive(Debug, Clone)]
pub struct WasmTargetConfig {
    /// Enable multi-value extension
    pub multi_value: bool,
    /// Enable bulk memory operations
    pub bulk_memory: bool,
    /// Enable sign extension operations
    pub sign_extension: bool,
    /// Enable SIMD operations
    pub simd: bool,
    /// Enable threads
    pub threads: bool,
    /// Enable tail calls
    pub tail_calls: bool,
    /// Memory configuration
    pub memory_config: WasmMemoryConfig,
}

/// WebAssembly memory configuration
#[derive(Debug, Clone)]
pub struct WasmMemoryConfig {
    /// Initial memory size in pages (64KB each)
    pub initial_pages: u32,
    /// Maximum memory size in pages (None = unlimited)
    pub maximum_pages: Option<u32>,
    /// Enable memory64
    pub memory64: bool,
    /// Enable shared memory
    pub shared: bool,
}

impl Default for WasmTargetConfig {
    fn default() -> Self {
        Self {
            multi_value: true,
            bulk_memory: true,
            sign_extension: true,
            simd: false,
            threads: false,
            tail_calls: false,
            memory_config: WasmMemoryConfig::default(),
        }
    }
}

impl Default for WasmMemoryConfig {
    fn default() -> Self {
        Self {
            initial_pages: 1,
            maximum_pages: Some(16),
            memory64: false,
            shared: false,
        }
    }
}

/// Enhanced WebAssembly code generation backend
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
    /// Optimization configuration
    optimization_config: WasmOptimizationConfig,
    /// Target configuration
    target_config: WasmTargetConfig,
    /// Local variable indices for current function
    local_indices: HashMap<String, u32>,
    /// Next available local index
    next_local_index: u32,
    /// Constant pool for optimization
    constant_pool: HashMap<i32, u32>,
}

impl WasmBackend {
    /// Create a new WASM backend with default configuration
    pub fn new() -> Self {
        Self {
            module: Module::new(),
            function_types: HashMap::new(),
            function_indices: HashMap::new(),
            next_type_index: 0,
            next_function_index: 0,
            deterministic_mode: false,
            optimization_config: WasmOptimizationConfig::default(),
            target_config: WasmTargetConfig::default(),
            local_indices: HashMap::new(),
            next_local_index: 0,
            constant_pool: HashMap::new(),
        }
    }

    /// Create a new WASM backend with specific optimization configuration
    pub fn new_with_optimization(optimization_config: WasmOptimizationConfig) -> Self {
        Self {
            module: Module::new(),
            function_types: HashMap::new(),
            function_indices: HashMap::new(),
            next_type_index: 0,
            next_function_index: 0,
            deterministic_mode: false,
            optimization_config,
            target_config: WasmTargetConfig::default(),
            local_indices: HashMap::new(),
            next_local_index: 0,
            constant_pool: HashMap::new(),
        }
    }

    /// Create a new WASM backend with specific target configuration
    pub fn new_with_target(target_config: WasmTargetConfig) -> Self {
        Self {
            module: Module::new(),
            function_types: HashMap::new(),
            function_indices: HashMap::new(),
            next_type_index: 0,
            next_function_index: 0,
            deterministic_mode: false,
            optimization_config: WasmOptimizationConfig::default(),
            target_config,
            local_indices: HashMap::new(),
            next_local_index: 0,
            constant_pool: HashMap::new(),
        }
    }

    /// Set deterministic mode for reproducible builds
    pub fn set_deterministic_mode(&mut self, enabled: bool) {
        self.deterministic_mode = enabled;
    }

    /// Set optimization configuration
    pub fn set_optimization_config(&mut self, config: WasmOptimizationConfig) {
        self.optimization_config = config;
    }

    /// Set target configuration
    pub fn set_target_config(&mut self, config: WasmTargetConfig) {
        self.target_config = config;
    }

    /// Apply constant folding optimization
    fn apply_constant_folding(&self, instruction: &Instruction) -> Option<i32> {
        if !self.optimization_config.constant_folding {
            return None;
        }

        match instruction.opcode {
            Opcode::Add => {
                if instruction.operands.len() >= 2 {
                    if let (Value::Constant(Constant::Number(a)), Value::Constant(Constant::Number(b))) = 
                        (&instruction.operands[0], &instruction.operands[1]) {
                        return Some(a + b);
                    }
                }
            }
            Opcode::Sub => {
                if instruction.operands.len() >= 2 {
                    if let (Value::Constant(Constant::Number(a)), Value::Constant(Constant::Number(b))) = 
                        (&instruction.operands[0], &instruction.operands[1]) {
                        return Some(a - b);
                    }
                }
            }
            Opcode::Mul => {
                if instruction.operands.len() >= 2 {
                    if let (Value::Constant(Constant::Number(a)), Value::Constant(Constant::Number(b))) = 
                        (&instruction.operands[0], &instruction.operands[1]) {
                        return Some(a * b);
                    }
                }
            }
            Opcode::Div => {
                if instruction.operands.len() >= 2 {
                    if let (Value::Constant(Constant::Number(a)), Value::Constant(Constant::Number(b))) = 
                        (&instruction.operands[0], &instruction.operands[1]) {
                        if *b != 0 {
                            return Some(a / b);
                        }
                    }
                }
            }
            _ => {}
        }

        None
    }

    /// Check if instruction should be eliminated (dead code elimination)
    fn should_eliminate_instruction(&self, _instruction: &Instruction) -> bool {
        // Simple dead code elimination - for now, don't eliminate anything
        // In a full implementation, this would track variable usage
        !self.optimization_config.dead_code_elimination
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

    /// Generate WASM module from IR with enhanced optimizations
    fn generate_module(&mut self, ir: &Program) -> OvieResult<Vec<u8>> {
        // Add memory section if needed
        if self.target_config.memory_config.initial_pages > 0 {
            self.add_memory()?;
        }
        
        // Add type section
        self.add_types(ir)?;
        
        // Add import section (for print function)
        self.add_imports()?;
        
        // Add function section
        self.add_functions(ir)?;
        
        // Add export section
        self.add_exports(ir)?;
        
        // Add code section with optimizations
        self.add_code_with_optimizations(ir)?;
        
        // Clone the module to avoid move issues
        let module = std::mem::replace(&mut self.module, Module::new());
        Ok(module.finish())
    }

    /// Add memory section
    fn add_memory(&mut self) -> OvieResult<()> {
        let mut memories = MemorySection::new();
        
        let memory_type = MemoryType {
            minimum: self.target_config.memory_config.initial_pages,
            maximum: self.target_config.memory_config.maximum_pages,
            memory64: self.target_config.memory_config.memory64,
            shared: self.target_config.memory_config.shared,
        };
        
        memories.memory(memory_type);
        self.module.section(&memories);
        Ok(())
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

    /// Add code section with function bodies and optimizations
    fn add_code_with_optimizations(&mut self, ir: &Program) -> OvieResult<()> {
        let mut code = CodeSection::new();
        
        // Sort functions deterministically if in deterministic mode
        let mut functions: Vec<_> = ir.functions.iter().collect();
        if self.deterministic_mode {
            functions.sort_by_key(|(_, func)| &func.name);
        }
        
        for (ir_id, function) in functions {
            self.local_indices.clear();
            self.next_local_index = 0;
            
            // Determine local variables needed
            let locals = self.analyze_function_locals(function)?;
            let mut func_body = wasm_encoder::Function::new(locals);
            
            // Generate optimized code for the function
            self.generate_optimized_function_body(&mut func_body, function)?;
            
            code.function(&func_body);
        }
        
        self.module.section(&code);
        Ok(())
    }

    /// Analyze function to determine local variables needed
    fn analyze_function_locals(&mut self, ir_func: &Function) -> OvieResult<Vec<(u32, ValType)>> {
        let mut locals = Vec::new();
        
        // For now, assume we need some i32 locals for temporary values
        // In a full implementation, this would analyze the IR to determine exact needs
        locals.push((2, ValType::I32)); // 2 i32 locals for temporary calculations
        
        Ok(locals)
    }

    /// Generate optimized WASM code for a function body
    fn generate_optimized_function_body(&mut self, func: &mut wasm_encoder::Function, ir_func: &Function) -> OvieResult<()> {
        // Get the entry block
        let entry_block = ir_func.basic_blocks.get(&ir_func.entry_block)
            .ok_or_else(|| OvieError::CodegenError { 
                message: "Entry block not found".to_string() 
            })?;
        
        // Generate optimized code for the entry block
        self.generate_optimized_block_code(func, entry_block)?;
        
        Ok(())
    }

    /// Generate optimized WASM code for a basic block
    fn generate_optimized_block_code(&mut self, func: &mut wasm_encoder::Function, block: &BasicBlock) -> OvieResult<()> {
        // Generate instructions with optimizations
        for instruction in &block.instructions {
            // Skip eliminated instructions
            if self.should_eliminate_instruction(instruction) {
                continue;
            }
            
            // Apply constant folding
            if let Some(folded_value) = self.apply_constant_folding(instruction) {
                func.instruction(&wasm_encoder::Instruction::I32Const(folded_value));
                continue;
            }
            
            // Generate normal instruction
            self.generate_enhanced_instruction_code(func, instruction)?;
        }
        
        // Generate terminator
        self.generate_enhanced_terminator_code(func, &block.terminator)?;
        
        Ok(())
    }

    /// Generate enhanced WASM code for an instruction
    fn generate_enhanced_instruction_code(&mut self, func: &mut wasm_encoder::Function, instruction: &Instruction) -> OvieResult<()> {
        match instruction.opcode {
            Opcode::Print => {
                // For print, we need to get the value and call the imported print function
                if let Some(operand) = instruction.operands.first() {
                    self.generate_enhanced_value_code(func, operand)?;
                    func.instruction(&wasm_encoder::Instruction::Call(0)); // Call imported print function
                }
            }
            Opcode::Add => {
                // Generate code for operands with optimization
                if instruction.operands.len() >= 2 {
                    self.generate_enhanced_value_code(func, &instruction.operands[0])?;
                    self.generate_enhanced_value_code(func, &instruction.operands[1])?;
                    func.instruction(&wasm_encoder::Instruction::I32Add);
                }
            }
            Opcode::Sub => {
                if instruction.operands.len() >= 2 {
                    self.generate_enhanced_value_code(func, &instruction.operands[0])?;
                    self.generate_enhanced_value_code(func, &instruction.operands[1])?;
                    func.instruction(&wasm_encoder::Instruction::I32Sub);
                }
            }
            Opcode::Mul => {
                if instruction.operands.len() >= 2 {
                    self.generate_enhanced_value_code(func, &instruction.operands[0])?;
                    self.generate_enhanced_value_code(func, &instruction.operands[1])?;
                    func.instruction(&wasm_encoder::Instruction::I32Mul);
                }
            }
            Opcode::Div => {
                if instruction.operands.len() >= 2 {
                    self.generate_enhanced_value_code(func, &instruction.operands[0])?;
                    self.generate_enhanced_value_code(func, &instruction.operands[1])?;
                    func.instruction(&wasm_encoder::Instruction::I32DivS);
                }
            }
            Opcode::Load => {
                if let Some(operand) = instruction.operands.first() {
                    self.generate_enhanced_value_code(func, operand)?;
                    func.instruction(&wasm_encoder::Instruction::I32Load(MemArg { offset: 0, align: 2 }));
                }
            }
            Opcode::Store => {
                if instruction.operands.len() >= 2 {
                    self.generate_enhanced_value_code(func, &instruction.operands[0])?; // address
                    self.generate_enhanced_value_code(func, &instruction.operands[1])?; // value
                    func.instruction(&wasm_encoder::Instruction::I32Store(MemArg { offset: 0, align: 2 }));
                }
            }
            Opcode::Compare => {
                if instruction.operands.len() >= 2 {
                    self.generate_enhanced_value_code(func, &instruction.operands[0])?;
                    self.generate_enhanced_value_code(func, &instruction.operands[1])?;
                    func.instruction(&wasm_encoder::Instruction::I32Eq);
                }
            }
            _ => {
                // For now, ignore other opcodes
            }
        }
        
        Ok(())
    }

    /// Generate enhanced WASM code for a value with optimizations
    fn generate_enhanced_value_code(&mut self, func: &mut wasm_encoder::Function, value: &Value) -> OvieResult<()> {
        match value {
            Value::Constant(constant) => {
                match constant {
                    Constant::Number(n) => {
                        // Use constant pool for frequently used constants if optimizing for size
                        if self.optimization_config.optimize_size {
                            if let Some(&pool_index) = self.constant_pool.get(n) {
                                func.instruction(&wasm_encoder::Instruction::LocalGet(pool_index));
                                return Ok(());
                            }
                        }
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
                    Constant::Float(f) => {
                        func.instruction(&wasm_encoder::Instruction::F64Const(*f));
                    }
                }
            }
            Value::Variable(name) => {
                if let Some(&local_index) = self.local_indices.get(name) {
                    func.instruction(&wasm_encoder::Instruction::LocalGet(local_index));
                } else {
                    // Allocate new local variable
                    let local_index = self.next_local_index;
                    self.local_indices.insert(name.clone(), local_index);
                    self.next_local_index += 1;
                    func.instruction(&wasm_encoder::Instruction::LocalGet(local_index));
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
            Value::Function(_) => {
                // Function references not supported in value context
                func.instruction(&wasm_encoder::Instruction::I32Const(0));
            }
        }
        
        Ok(())
    }

    /// Generate enhanced WASM code for a terminator
    fn generate_enhanced_terminator_code(&mut self, func: &mut wasm_encoder::Function, terminator: &Terminator) -> OvieResult<()> {
        match terminator {
            Terminator::Return { value } => {
                if let Some(return_value) = value {
                    self.generate_enhanced_value_code(func, return_value)?;
                }
                func.instruction(&wasm_encoder::Instruction::Return);
            }
            Terminator::Branch { target: _ } => {
                // For now, just return (simplified)
                func.instruction(&wasm_encoder::Instruction::Return);
            }
            Terminator::ConditionalBranch { condition, true_target: _, false_target: _ } => {
                // Generate condition and use br_if for conditional branching
                self.generate_enhanced_value_code(func, condition)?;
                func.instruction(&wasm_encoder::Instruction::BrIf(0));
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
        matches!(target, 
            "wasm32-unknown-unknown" | 
            "wasm32-wasi" |
            "wasm64-unknown-unknown" |
            "wasm" | 
            "webassembly"
        )
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
        assert!(backend.supports_target("wasm32-wasi"));
        assert!(backend.supports_target("wasm64-unknown-unknown"));
        assert!(!backend.supports_target("x86_64-unknown-linux-gnu"));
    }

    #[test]
    fn test_optimization_configurations() {
        let size_config = WasmOptimizationConfig::optimize_for_size();
        assert!(size_config.optimize_size);
        assert!(!size_config.optimize_speed);
        assert!(size_config.dead_code_elimination);
        assert_eq!(size_config.inline_threshold, 20);

        let speed_config = WasmOptimizationConfig::optimize_for_speed();
        assert!(!speed_config.optimize_size);
        assert!(speed_config.optimize_speed);
        assert!(speed_config.function_inlining);
        assert_eq!(speed_config.inline_threshold, 100);

        let no_opt_config = WasmOptimizationConfig::no_optimizations();
        assert!(!no_opt_config.optimize_size);
        assert!(!no_opt_config.optimize_speed);
        assert!(!no_opt_config.dead_code_elimination);
        assert!(!no_opt_config.constant_folding);
    }

    #[test]
    fn test_target_configurations() {
        let default_config = WasmTargetConfig::default();
        assert!(default_config.multi_value);
        assert!(default_config.bulk_memory);
        assert!(default_config.sign_extension);
        assert!(!default_config.simd);
        assert!(!default_config.threads);

        let memory_config = &default_config.memory_config;
        assert_eq!(memory_config.initial_pages, 1);
        assert_eq!(memory_config.maximum_pages, Some(16));
        assert!(!memory_config.memory64);
        assert!(!memory_config.shared);
    }

    #[test]
    fn test_enhanced_wasm_backend_with_optimizations() {
        let opt_config = WasmOptimizationConfig::optimize_for_size();
        let backend = WasmBackend::new_with_optimization(opt_config);
        
        assert_eq!(backend.name(), "wasm");
        assert!(backend.optimization_config.optimize_size);
        assert!(backend.optimization_config.dead_code_elimination);
    }

    #[test]
    fn test_enhanced_wasm_backend_with_target() {
        let mut target_config = WasmTargetConfig::default();
        target_config.simd = true;
        target_config.threads = true;
        
        let backend = WasmBackend::new_with_target(target_config);
        
        assert_eq!(backend.name(), "wasm");
        assert!(backend.target_config.simd);
        assert!(backend.target_config.threads);
    }

    #[test]
    fn test_deterministic_mode() {
        let mut backend = WasmBackend::new();
        
        assert!(!backend.deterministic_mode);
        
        backend.set_deterministic_mode(true);
        assert!(backend.deterministic_mode);
    }

    #[test]
    fn test_compile_to_enhanced_wasm() {
        let source = r#"seeAm "Hello Enhanced WASM!";"#;
        let mut compiler = crate::Compiler::new();
        
        let result = compiler.compile_to_wasm(source);
        assert!(result.is_ok());
        
        let wasm_bytes = result.unwrap();
        assert!(!wasm_bytes.is_empty());
        
        // Verify WASM magic number
        assert_eq!(&wasm_bytes[0..4], &[0x00, 0x61, 0x73, 0x6d]);
        
        // Verify WASM version
        assert_eq!(&wasm_bytes[4..8], &[0x01, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_constant_folding() {
        let backend = WasmBackend::new_with_optimization(WasmOptimizationConfig::optimize_for_speed());
        
        // Create a simple add instruction with constants
        let instruction = Instruction {
            opcode: Opcode::Add,
            operands: vec![
                Value::Constant(Constant::Number(5)),
                Value::Constant(Constant::Number(3)),
            ],
            result: None,
        };
        
        let folded = backend.apply_constant_folding(&instruction);
        assert_eq!(folded, Some(8));
    }

    #[test]
    fn test_wasm_target_support() {
        let backend = WasmBackend::new();
        
        // Test supported targets
        assert!(backend.supports_target("wasm32-unknown-unknown"));
        assert!(backend.supports_target("wasm32-wasi"));
        assert!(backend.supports_target("wasm64-unknown-unknown"));
        assert!(backend.supports_target("wasm"));
        assert!(backend.supports_target("webassembly"));
        
        // Test unsupported targets
        assert!(!backend.supports_target("x86_64-unknown-linux-gnu"));
        assert!(!backend.supports_target("aarch64-apple-darwin"));
        assert!(!backend.supports_target("unknown-target"));
    }
}