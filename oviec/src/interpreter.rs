//! Interpreter for executing Ovie AST

use crate::ast::{AstNode, Statement, Expression, Literal, BinaryOperator, UnaryOperator};
use crate::error::{OvieError, OvieResult};
use std::collections::HashMap;

/// Runtime value types
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<Value>),
    Struct(HashMap<String, Value>),
    Enum { variant: String, data: Option<Box<Value>> },
    Null,
}

impl Value {
    /// Convert value to string for printing
    pub fn to_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::Boolean(b) => b.to_string(),
            Value::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", elements.join(", "))
            }
            Value::Struct(fields) => {
                let field_strs: Vec<String> = fields
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                format!("{{ {} }}", field_strs.join(", "))
            }
            Value::Enum { variant, data } => {
                if let Some(d) = data {
                    format!("{}({})", variant, d.to_string())
                } else {
                    variant.clone()
                }
            }
            Value::Null => "null".to_string(),
        }
    }

    /// Check if value is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(arr) => !arr.is_empty(),
            Value::Struct(_) => true,
            Value::Enum { .. } => true,
        }
    }
}

/// Function definition
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Vec<Statement>,
}

/// Environment for variable and function storage
#[derive(Debug, Clone)]
pub struct Environment {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Function>,
    struct_types: HashMap<String, Vec<String>>, // struct_name -> field_names
    enum_types: HashMap<String, Vec<String>>,   // enum_name -> variant_names
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            struct_types: HashMap::new(),
            enum_types: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Environment) -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            struct_types: HashMap::new(),
            enum_types: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn define_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.variables.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.get_variable(name)
        } else {
            None
        }
    }

    pub fn define_function(&mut self, function: Function) {
        self.functions.insert(function.name.clone(), function);
    }

    pub fn get_function(&self, name: &str) -> Option<Function> {
        if let Some(function) = self.functions.get(name) {
            Some(function.clone())
        } else if let Some(parent) = &self.parent {
            parent.get_function(name)
        } else {
            None
        }
    }

    pub fn define_struct_type(&mut self, name: String, fields: Vec<String>) {
        self.struct_types.insert(name, fields);
    }

    pub fn get_struct_type(&self, name: &str) -> Option<Vec<String>> {
        if let Some(fields) = self.struct_types.get(name) {
            Some(fields.clone())
        } else if let Some(parent) = &self.parent {
            parent.get_struct_type(name)
        } else {
            None
        }
    }

    pub fn define_enum_type(&mut self, name: String, variants: Vec<String>) {
        self.enum_types.insert(name, variants);
    }

    pub fn get_enum_type(&self, name: &str) -> Option<Vec<String>> {
        if let Some(variants) = self.enum_types.get(name) {
            Some(variants.clone())
        } else if let Some(parent) = &self.parent {
            parent.get_enum_type(name)
        } else {
            None
        }
    }
}

/// Interpreter for Ovie programs
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    /// Interpret an AST
    pub fn interpret(&mut self, ast: &AstNode) -> OvieResult<()> {
        match ast {
            AstNode::Program(statements) => {
                for statement in statements {
                    self.execute_statement(statement)?;
                }
                Ok(())
            }
        }
    }

    /// Execute a statement
    fn execute_statement(&mut self, statement: &Statement) -> OvieResult<Option<Value>> {
        match statement {
            Statement::Print { expression } => {
                let value = self.evaluate_expression(expression)?;
                println!("{}", value.to_string());
                Ok(None)
            }

            Statement::Assignment { identifier, value, .. } => {
                let evaluated_value = self.evaluate_expression(value)?;
                self.environment.define_variable(identifier.clone(), evaluated_value);
                Ok(None)
            }

            Statement::VariableDeclaration { identifier, value, .. } => {
                let evaluated_value = self.evaluate_expression(value)?;
                self.environment.define_variable(identifier.clone(), evaluated_value);
                Ok(None)
            }

            Statement::Function { name, parameters, body } => {
                let function = Function {
                    name: name.clone(),
                    parameters: parameters.clone(),
                    body: body.clone(),
                };
                self.environment.define_function(function);
                Ok(None)
            }

            Statement::FunctionDeclaration { name, parameters, body } => {
                let function = Function {
                    name: name.clone(),
                    parameters: parameters.clone(),
                    body: body.clone(),
                };
                self.environment.define_function(function);
                Ok(None)
            }

            Statement::If { condition, then_block, else_block } => {
                let condition_value = self.evaluate_expression(condition)?;
                
                if condition_value.is_truthy() {
                    for stmt in then_block {
                        if let Some(return_value) = self.execute_statement(stmt)? {
                            return Ok(Some(return_value));
                        }
                    }
                } else if let Some(else_stmts) = else_block {
                    for stmt in else_stmts {
                        if let Some(return_value) = self.execute_statement(stmt)? {
                            return Ok(Some(return_value));
                        }
                    }
                }
                Ok(None)
            }

            Statement::While { condition, body } => {
                while self.evaluate_expression(condition)?.is_truthy() {
                    for stmt in body {
                        if let Some(return_value) = self.execute_statement(stmt)? {
                            return Ok(Some(return_value));
                        }
                    }
                }
                Ok(None)
            }

            Statement::For { identifier, iterable, body } => {
                let iterable_value = self.evaluate_expression(iterable)?;
                
                match iterable_value {
                    Value::Array(arr) => {
                        for value in arr {
                            self.environment.define_variable(
                                identifier.clone(),
                                value
                            );
                            
                            for stmt in body {
                                if let Some(return_value) = self.execute_statement(stmt)? {
                                    return Ok(Some(return_value));
                                }
                            }
                        }
                    }
                    Value::Number(end) => {
                        // Legacy support for simple numeric ranges
                        for i in 0..(end as i32) {
                            self.environment.define_variable(
                                identifier.clone(),
                                Value::Number(i as f64)
                            );
                            
                            for stmt in body {
                                if let Some(return_value) = self.execute_statement(stmt)? {
                                    return Ok(Some(return_value));
                                }
                            }
                        }
                    }
                    _ => {
                        return Err(OvieError::runtime_error(
                            "For loop iterable must be an array or number"
                        ));
                    }
                }
                Ok(None)
            }

            Statement::Return { value } => {
                let return_value = if let Some(expr) = value {
                    self.evaluate_expression(expr)?
                } else {
                    Value::Null
                };
                Ok(Some(return_value))
            }

            Statement::Expression { expression } => {
                self.evaluate_expression(expression)?;
                Ok(None)
            }

            Statement::Struct { name, fields } => {
                // Register struct type with field names
                let field_names: Vec<String> = fields.iter().map(|f| f.name.clone()).collect();
                self.environment.define_struct_type(name.clone(), field_names);
                Ok(None)
            }

            Statement::Enum { name, variants } => {
                // Register enum type with variant names
                let variant_names: Vec<String> = variants.iter().map(|v| v.name.clone()).collect();
                self.environment.define_enum_type(name.clone(), variant_names);
                Ok(None)
            }
        }
    }

    /// Evaluate an expression
    fn evaluate_expression(&mut self, expression: &Expression) -> OvieResult<Value> {
        match expression {
            Expression::Literal(literal) => {
                match literal {
                    Literal::String(s) => Ok(Value::String(s.clone())),
                    Literal::Number(n) => Ok(Value::Number(*n)),
                    Literal::Boolean(b) => Ok(Value::Boolean(*b)),
                }
            }

            Expression::Identifier(name) => {
                if let Some(value) = self.environment.get_variable(name) {
                    Ok(value)
                } else {
                    Err(OvieError::runtime_error(format!("Undefined variable: {}", name)))
                }
            }

            Expression::Binary { left, operator, right } => {
                let left_value = self.evaluate_expression(left)?;
                let right_value = self.evaluate_expression(right)?;
                
                self.apply_binary_operator(&left_value, operator, &right_value)
            }

            Expression::Unary { operator, operand } => {
                let operand_value = self.evaluate_expression(operand)?;
                self.apply_unary_operator(operator, &operand_value)
            }

            Expression::Call { function, arguments } => {
                if let Some(func) = self.environment.get_function(function) {
                    if arguments.len() != func.parameters.len() {
                        return Err(OvieError::runtime_error(format!(
                            "Function '{}' expects {} arguments, got {}",
                            function,
                            func.parameters.len(),
                            arguments.len()
                        )));
                    }

                    // Evaluate arguments
                    let mut arg_values = Vec::new();
                    for arg in arguments {
                        arg_values.push(self.evaluate_expression(arg)?);
                    }

                    // Create new environment for function execution
                    let mut func_env = Environment::with_parent(self.environment.clone());
                    
                    // Bind parameters to arguments
                    for (param, arg_value) in func.parameters.iter().zip(arg_values.iter()) {
                        func_env.define_variable(param.clone(), arg_value.clone());
                    }

                    // Save current environment and switch to function environment
                    let saved_env = std::mem::replace(&mut self.environment, func_env);

                    // Execute function body
                    let mut result = Value::Null;
                    for stmt in &func.body {
                        if let Some(return_value) = self.execute_statement(stmt)? {
                            result = return_value;
                            break;
                        }
                    }

                    // Restore environment
                    self.environment = saved_env;

                    Ok(result)
                } else {
                    Err(OvieError::runtime_error(format!("Undefined function: {}", function)))
                }
            }

            Expression::FieldAccess { object, field } => {
                let object_value = self.evaluate_expression(object)?;
                
                match object_value {
                    Value::Struct(fields) => {
                        // Try exact match first
                        if let Some(value) = fields.get(field) {
                            return Ok(value.clone());
                        }
                        
                        // Convert camelCase to snake_case for lookup
                        let field_snake = self.camel_to_snake(field);
                        if let Some(value) = fields.get(&field_snake) {
                            return Ok(value.clone());
                        }
                        
                        // Convert snake_case to camelCase for lookup
                        let field_camel = self.snake_to_camel(field);
                        if let Some(value) = fields.get(&field_camel) {
                            return Ok(value.clone());
                        }
                        
                        // Debug: show available fields
                        let available: Vec<String> = fields.keys().cloned().collect();
                        Err(OvieError::runtime_error(format!(
                            "Field '{}' not found in struct. Available fields: {:?}",
                            field, available
                        )))
                    }
                    _ => Err(OvieError::runtime_error(format!(
                        "Cannot access field '{}' on non-struct value",
                        field
                    ))),
                }
            }

            Expression::StructInstantiation { struct_name, fields } => {
                // Verify struct type exists
                if self.environment.get_struct_type(struct_name).is_none() {
                    return Err(OvieError::runtime_error(format!(
                        "Undefined struct type: {}",
                        struct_name
                    )));
                }

                // Evaluate field values
                let mut field_values = HashMap::new();
                for field_init in fields {
                    let value = self.evaluate_expression(&field_init.value)?;
                    field_values.insert(field_init.name.clone(), value);
                }

                Ok(Value::Struct(field_values))
            }

            Expression::Range { start, end } => {
                let start_val = self.evaluate_expression(start)?;
                let end_val = self.evaluate_expression(end)?;
                
                match (start_val, end_val) {
                    (Value::Number(s), Value::Number(e)) => {
                        let start_int = s as i32;
                        let end_int = e as i32;
                        let range_values: Vec<Value> = (start_int..end_int)
                            .map(|i| Value::Number(i as f64))
                            .collect();
                        Ok(Value::Array(range_values))
                    }
                    _ => Err(OvieError::runtime_error("Range expressions require numeric values"))
                }
            }

            Expression::EnumVariantConstruction { enum_name, variant_name, data } => {
                // Verify enum type exists
                if self.environment.get_enum_type(enum_name).is_none() {
                    return Err(OvieError::runtime_error(format!(
                        "Undefined enum type: {}",
                        enum_name
                    )));
                }

                // Evaluate data if present
                let variant_data = if let Some(data_expr) = data {
                    Some(Box::new(self.evaluate_expression(data_expr)?))
                } else {
                    None
                };

                Ok(Value::Enum {
                    variant: variant_name.clone(),
                    data: variant_data,
                })
            }

            Expression::ArrayLiteral { elements } => {
                let mut array_values = Vec::new();
                for element in elements {
                    array_values.push(self.evaluate_expression(element)?);
                }
                Ok(Value::Array(array_values))
            }

            Expression::Index { object, index } => {
                let object_value = self.evaluate_expression(object)?;
                let index_value = self.evaluate_expression(index)?;
                
                match (object_value, index_value) {
                    (Value::Array(arr), Value::Number(idx)) => {
                        let index = idx as usize;
                        if index < arr.len() {
                            Ok(arr[index].clone())
                        } else {
                            Err(OvieError::runtime_error(format!(
                                "Array index out of bounds: {} (length: {})",
                                index, arr.len()
                            )))
                        }
                    }
                    (Value::String(s), Value::Number(idx)) => {
                        let index = idx as usize;
                        let chars: Vec<char> = s.chars().collect();
                        if index < chars.len() {
                            Ok(Value::String(chars[index].to_string()))
                        } else {
                            Err(OvieError::runtime_error(format!(
                                "String index out of bounds: {} (length: {})",
                                index, chars.len()
                            )))
                        }
                    }
                    (obj, idx) => {
                        Err(OvieError::runtime_error(format!(
                            "Cannot index {} with {}",
                            self.value_type_name(&obj),
                            self.value_type_name(&idx)
                        )))
                    }
                }
            }
        }
    }

    /// Apply binary operator
    fn apply_binary_operator(
        &self,
        left: &Value,
        operator: &BinaryOperator,
        right: &Value,
    ) -> OvieResult<Value> {
        match (left, operator, right) {
            // Arithmetic operations
            (Value::Number(a), BinaryOperator::Add, Value::Number(b)) => {
                Ok(Value::Number(a + b))
            }
            (Value::Number(a), BinaryOperator::Subtract, Value::Number(b)) => {
                Ok(Value::Number(a - b))
            }
            (Value::Number(a), BinaryOperator::Multiply, Value::Number(b)) => {
                Ok(Value::Number(a * b))
            }
            (Value::Number(a), BinaryOperator::Divide, Value::Number(b)) => {
                if *b == 0.0 {
                    Err(OvieError::runtime_error("Division by zero"))
                } else {
                    Ok(Value::Number(a / b))
                }
            }
            (Value::Number(a), BinaryOperator::Modulo, Value::Number(b)) => {
                if *b == 0.0 {
                    Err(OvieError::runtime_error("Modulo by zero"))
                } else {
                    Ok(Value::Number(a % b))
                }
            }

            // String concatenation
            (Value::String(a), BinaryOperator::Add, Value::String(b)) => {
                Ok(Value::String(format!("{}{}", a, b)))
            }
            (Value::String(a), BinaryOperator::Add, b) => {
                Ok(Value::String(format!("{}{}", a, b.to_string())))
            }
            (a, BinaryOperator::Add, Value::String(b)) => {
                Ok(Value::String(format!("{}{}", a.to_string(), b)))
            }

            // Array concatenation
            (Value::Array(a), BinaryOperator::Add, Value::Array(b)) => {
                let mut result = a.clone();
                result.extend(b.clone());
                Ok(Value::Array(result))
            }

            // Comparison operations
            (Value::Number(a), BinaryOperator::Equal, Value::Number(b)) => {
                Ok(Value::Boolean(a == b))
            }
            (Value::String(a), BinaryOperator::Equal, Value::String(b)) => {
                Ok(Value::Boolean(a == b))
            }
            (Value::Boolean(a), BinaryOperator::Equal, Value::Boolean(b)) => {
                Ok(Value::Boolean(a == b))
            }
            (a, BinaryOperator::NotEqual, b) => {
                let equal_result = self.apply_binary_operator(a, &BinaryOperator::Equal, b)?;
                match equal_result {
                    Value::Boolean(b) => Ok(Value::Boolean(!b)),
                    _ => unreachable!(),
                }
            }

            (Value::Number(a), BinaryOperator::Less, Value::Number(b)) => {
                Ok(Value::Boolean(a < b))
            }
            (Value::Number(a), BinaryOperator::LessEqual, Value::Number(b)) => {
                Ok(Value::Boolean(a <= b))
            }
            (Value::Number(a), BinaryOperator::Greater, Value::Number(b)) => {
                Ok(Value::Boolean(a > b))
            }
            (Value::Number(a), BinaryOperator::GreaterEqual, Value::Number(b)) => {
                Ok(Value::Boolean(a >= b))
            }

            // Logical operations
            (a, BinaryOperator::And, b) => {
                if a.is_truthy() {
                    Ok(b.clone())
                } else {
                    Ok(a.clone())
                }
            }
            (a, BinaryOperator::Or, b) => {
                if a.is_truthy() {
                    Ok(a.clone())
                } else {
                    Ok(b.clone())
                }
            }

            _ => Err(OvieError::runtime_error(format!(
                "Invalid binary operation: {} {} {}",
                self.value_type_name(left),
                operator,
                self.value_type_name(right)
            ))),
        }
    }

    /// Apply unary operator
    fn apply_unary_operator(&self, operator: &UnaryOperator, operand: &Value) -> OvieResult<Value> {
        match (operator, operand) {
            (UnaryOperator::Negate, Value::Number(n)) => Ok(Value::Number(-n)),
            (UnaryOperator::Not, operand) => Ok(Value::Boolean(!operand.is_truthy())),
            _ => Err(OvieError::runtime_error(format!(
                "Invalid unary operation: {} {}",
                operator,
                self.value_type_name(operand)
            ))),
        }
    }

    /// Get type name for error messages
    fn value_type_name(&self, value: &Value) -> &'static str {
        match value {
            Value::String(_) => "string",
            Value::Number(_) => "number",
            Value::Boolean(_) => "boolean",
            Value::Array(_) => "array",
            Value::Struct(_) => "struct",
            Value::Enum { .. } => "enum",
            Value::Null => "null",
        }
    }

    /// Convert camelCase to snake_case
    fn camel_to_snake(&self, input: &str) -> String {
        let mut result = String::new();
        for (i, ch) in input.chars().enumerate() {
            if ch.is_uppercase() && i > 0 {
                result.push('_');
                result.push(ch.to_lowercase().next().unwrap());
            } else {
                result.push(ch);
            }
        }
        result
    }

    /// Convert snake_case to camelCase
    fn snake_to_camel(&self, input: &str) -> String {
        let parts: Vec<&str> = input.split('_').collect();
        if parts.len() <= 1 {
            return input.to_string();
        }

        let mut result = parts[0].to_string();
        for part in &parts[1..] {
            if !part.is_empty() {
                let mut chars = part.chars();
                if let Some(first) = chars.next() {
                    result.push(first.to_uppercase().next().unwrap_or(first));
                    result.extend(chars);
                }
            }
        }
        result
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn interpret_source(source: &str) -> OvieResult<()> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&ast)
    }

    #[test]
    fn test_simple_print() {
        // This test will print to stdout, so we can't easily assert the output
        // But we can verify it doesn't error
        interpret_source(r#"seeAm "Hello, World!";"#).unwrap();
    }

    #[test]
    fn test_arithmetic() {
        interpret_source("result = 10 + 5; seeAm result;").unwrap();
    }

    #[test]
    fn test_function_call() {
        interpret_source(r#"
            fn add(a, b) {
                return a + b;
            }
            result = add(10, 5);
            seeAm result;
        "#).unwrap();
    }

    #[test]
    fn test_ir_interpreter_simple_print() {
        let source = r#"seeAm "Hello from IR!";"#;
        let mut compiler = crate::Compiler::new();
        
        // This should not panic and should print to stdout
        compiler.compile_and_run_ir(source).unwrap();
    }

    #[test]
    fn test_ir_generation() {
        let source = r#"seeAm "Hello, World!";"#;
        let mut compiler = crate::Compiler::new();
        
        let ir = compiler.compile_to_ir(source).unwrap();
        
        // Verify IR structure
        assert!(ir.entry_point.is_some());
        assert!(!ir.functions.is_empty());
        
        let entry_function = ir.functions.get(&ir.entry_point.unwrap()).unwrap();
        assert_eq!(entry_function.name, "main");
        assert!(!entry_function.basic_blocks.is_empty());
        
        let entry_block = entry_function.basic_blocks.get(&entry_function.entry_block).unwrap();
        assert!(!entry_block.instructions.is_empty());
        
        // Should have a print instruction
        let print_instruction = &entry_block.instructions[0];
        assert!(matches!(print_instruction.opcode, crate::ir::Opcode::Print));
    }
}
/// IR Interpreter for executing IR programs
use crate::ir::{Program, Function as IrFunction, BasicBlock, Instruction, Terminator, Opcode, Value as IrValue, Constant};

/// IR Interpreter state
pub struct IrInterpreter {
    /// Global variables
    globals: HashMap<String, Value>,
    /// Function call stack
    call_stack: Vec<CallFrame>,
    /// Current execution state
    current_function: Option<u32>,
    current_block: Option<u32>,
    instruction_pointer: usize,
}

/// Call frame for function calls
#[derive(Debug, Clone)]
struct CallFrame {
    function_id: u32,
    locals: HashMap<u32, Value>, // ValueId -> Value
    return_address: Option<(u32, u32, usize)>, // (function_id, block_id, instruction_index)
}

impl IrInterpreter {
    /// Create a new IR interpreter
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
            call_stack: Vec::new(),
            current_function: None,
            current_block: None,
            instruction_pointer: 0,
        }
    }

    /// Execute an IR program
    pub fn execute(&mut self, program: &Program) -> OvieResult<()> {
        // Find entry point
        let entry_function_id = program.entry_point.ok_or_else(|| {
            OvieError::RuntimeError { message: "No entry point found".to_string() }
        })?;

        let entry_function = program.functions.get(&entry_function_id).ok_or_else(|| {
            OvieError::RuntimeError { message: "Entry function not found".to_string() }
        })?;

        // Initialize call frame for main function
        let call_frame = CallFrame {
            function_id: entry_function_id,
            locals: HashMap::new(),
            return_address: None,
        };
        
        self.call_stack.push(call_frame);
        self.current_function = Some(entry_function_id);
        self.current_block = Some(entry_function.entry_block);
        self.instruction_pointer = 0;

        // Execute until completion
        while !self.call_stack.is_empty() {
            self.execute_step(program)?;
        }

        Ok(())
    }

    /// Execute a single step
    fn execute_step(&mut self, program: &Program) -> OvieResult<()> {
        let function_id = self.current_function.ok_or_else(|| {
            OvieError::RuntimeError { message: "No current function".to_string() }
        })?;

        let block_id = self.current_block.ok_or_else(|| {
            OvieError::RuntimeError { message: "No current block".to_string() }
        })?;

        let function = program.functions.get(&function_id).ok_or_else(|| {
            OvieError::RuntimeError { message: "Function not found".to_string() }
        })?;

        let block = function.basic_blocks.get(&block_id).ok_or_else(|| {
            OvieError::RuntimeError { message: "Block not found".to_string() }
        })?;

        // Execute instruction if within bounds
        if self.instruction_pointer < block.instructions.len() {
            let instruction = &block.instructions[self.instruction_pointer];
            self.execute_instruction(instruction)?;
            self.instruction_pointer += 1;
        } else {
            // Execute terminator
            self.execute_terminator(&block.terminator, program)?;
        }

        Ok(())
    }

    /// Execute an instruction
    fn execute_instruction(&mut self, instruction: &Instruction) -> OvieResult<()> {
        let result = match instruction.opcode {
            Opcode::Print => {
                if let Some(operand) = instruction.operands.first() {
                    let value = self.evaluate_ir_value(operand)?;
                    println!("{}", value.to_string());
                }
                Value::Null
            }
            Opcode::Add => {
                let left = self.evaluate_ir_value(&instruction.operands[0])?;
                let right = self.evaluate_ir_value(&instruction.operands[1])?;
                self.add_values(left, right)?
            }
            Opcode::Sub => {
                let left = self.evaluate_ir_value(&instruction.operands[0])?;
                let right = self.evaluate_ir_value(&instruction.operands[1])?;
                self.subtract_values(left, right)?
            }
            Opcode::Mul => {
                let left = self.evaluate_ir_value(&instruction.operands[0])?;
                let right = self.evaluate_ir_value(&instruction.operands[1])?;
                self.multiply_values(left, right)?
            }
            Opcode::Div => {
                let left = self.evaluate_ir_value(&instruction.operands[0])?;
                let right = self.evaluate_ir_value(&instruction.operands[1])?;
                self.divide_values(left, right)?
            }
            _ => Value::Null, // Placeholder for other opcodes
        };

        // Store result in current call frame
        if let Some(call_frame) = self.call_stack.last_mut() {
            call_frame.locals.insert(instruction.id, result);
        }

        Ok(())
    }

    /// Execute a terminator
    fn execute_terminator(&mut self, terminator: &Terminator, _program: &Program) -> OvieResult<()> {
        match terminator {
            Terminator::Return { value: _ } => {
                // Pop call frame
                self.call_stack.pop();
                
                if let Some(call_frame) = self.call_stack.last() {
                    // Restore execution context
                    self.current_function = Some(call_frame.function_id);
                    if let Some((_, block_id, ip)) = call_frame.return_address {
                        self.current_block = Some(block_id);
                        self.instruction_pointer = ip;
                    }
                } else {
                    // Program finished
                    self.current_function = None;
                    self.current_block = None;
                }
            }
            Terminator::Branch { target } => {
                self.current_block = Some(*target);
                self.instruction_pointer = 0;
            }
            Terminator::ConditionalBranch { condition, true_target, false_target } => {
                let condition_value = self.evaluate_ir_value(condition)?;
                let target = if condition_value.is_truthy() {
                    *true_target
                } else {
                    *false_target
                };
                self.current_block = Some(target);
                self.instruction_pointer = 0;
            }
            Terminator::Unreachable => {
                return Err(OvieError::RuntimeError { 
                    message: "Reached unreachable code".to_string() 
                });
            }
        }

        Ok(())
    }

    /// Evaluate an IR value
    fn evaluate_ir_value(&self, ir_value: &IrValue) -> OvieResult<Value> {
        match ir_value {
            IrValue::Constant(constant) => {
                Ok(match constant {
                    Constant::String(s) => Value::String(s.clone()),
                    Constant::Number(n) => Value::Number(*n),
                    Constant::Boolean(b) => Value::Boolean(*b),
                    Constant::Void => Value::Null,
                })
            }
            IrValue::Instruction(value_id) => {
                if let Some(call_frame) = self.call_stack.last() {
                    call_frame.locals.get(value_id).cloned().ok_or_else(|| {
                        OvieError::RuntimeError { 
                            message: format!("Value {} not found", value_id) 
                        }
                    })
                } else {
                    Err(OvieError::RuntimeError { 
                        message: "No call frame".to_string() 
                    })
                }
            }
            IrValue::Global(name) => {
                self.globals.get(name).cloned().ok_or_else(|| {
                    OvieError::RuntimeError { 
                        message: format!("Global variable '{}' not found", name) 
                    }
                })
            }
            IrValue::Parameter(_) => {
                // For now, return null for parameters
                Ok(Value::Null)
            }
        }
    }

    /// Add two values
    fn add_values(&self, left: Value, right: Value) -> OvieResult<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            _ => Err(OvieError::RuntimeError { 
                message: "Cannot add these types".to_string() 
            }),
        }
    }

    /// Subtract two values
    fn subtract_values(&self, left: Value, right: Value) -> OvieResult<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            _ => Err(OvieError::RuntimeError { 
                message: "Cannot subtract these types".to_string() 
            }),
        }
    }

    /// Multiply two values
    fn multiply_values(&self, left: Value, right: Value) -> OvieResult<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            _ => Err(OvieError::RuntimeError { 
                message: "Cannot multiply these types".to_string() 
            }),
        }
    }

    /// Divide two values
    fn divide_values(&self, left: Value, right: Value) -> OvieResult<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if b == 0.0 {
                    Err(OvieError::RuntimeError { 
                        message: "Division by zero".to_string() 
                    })
                } else {
                    Ok(Value::Number(a / b))
                }
            }
            _ => Err(OvieError::RuntimeError { 
                message: "Cannot divide these types".to_string() 
            }),
        }
    }
}

impl Default for IrInterpreter {
    fn default() -> Self {
        Self::new()
    }
}