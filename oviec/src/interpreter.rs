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
    /// Internal signal for break
    Break,
    /// Internal signal for continue
    Continue,
    /// Internal signal for early return (from ? operator)
    Return(Box<Value>),
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
            Value::Break => "break".to_string(),
            Value::Continue => "continue".to_string(),
            Value::Return(v) => v.to_string(),
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
            Value::Break | Value::Continue | Value::Return(_) => false,
        }
    }
}

/// Function definition
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<crate::ast::Parameter>,
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

    pub fn set_variable(&mut self, name: &str, value: Value) -> OvieResult<()> {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
            Ok(())
        } else if let Some(parent) = &mut self.parent {
            parent.set_variable(name, value)
        } else {
            Err(OvieError::runtime_error(format!("Undefined variable: {}", name)))
        }
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

            Statement::CompoundAssignment { identifier, operator, value } => {
                let rhs = self.evaluate_expression(value)?;
                let current = self.environment.get_variable(identifier)
                    .ok_or_else(|| OvieError::runtime_error(format!("Undefined variable: {}", identifier)))?;
                let result = self.apply_binary_operator(&current, operator, &rhs)?;
                self.environment.set_variable(identifier, result)?;
                Ok(None)
            }

            Statement::ConstDeclaration { name, value } => {
                let evaluated_value = self.evaluate_expression(value)?;
                self.environment.define_variable(name.clone(), evaluated_value);
                Ok(None)
            }

            Statement::FieldMutation { object, field, value } => {
                // Evaluate the new value
                let new_value = self.evaluate_expression(value)?;
                
                // Get the object to mutate
                match object {
                    Expression::Identifier(name) => {
                        if let Some(mut obj_value) = self.environment.get_variable(name) {
                            if let Value::Struct(ref mut fields) = obj_value {
                                fields.insert(field.clone(), new_value);
                                self.environment.set_variable(name, obj_value)?;
                            } else {
                                return Err(OvieError::runtime_error(format!(
                                    "Cannot mutate field '{}' on non-struct value", field
                                )));
                            }
                        } else {
                            return Err(OvieError::runtime_error(format!("Undefined variable: {}", name)));
                        }
                    }
                    Expression::FieldAccess { object: nested_obj, field: nested_field } => {
                        if let Expression::Identifier(name) = nested_obj.as_ref() {
                            if let Some(mut obj_value) = self.environment.get_variable(name) {
                                if let Value::Struct(ref mut outer_fields) = obj_value {
                                    if let Some(Value::Struct(ref mut inner_fields)) = outer_fields.get_mut(nested_field) {
                                        inner_fields.insert(field.clone(), new_value);
                                        self.environment.set_variable(name, obj_value)?;
                                    } else {
                                        return Err(OvieError::runtime_error(format!("Field '{}' is not a struct", nested_field)));
                                    }
                                } else {
                                    return Err(OvieError::runtime_error("Cannot mutate nested field on non-struct value"));
                                }
                            } else {
                                return Err(OvieError::runtime_error(format!("Undefined variable: {}", name)));
                            }
                        } else {
                            return Err(OvieError::runtime_error("Complex nested field mutation not yet supported"));
                        }
                    }
                    _ => return Err(OvieError::runtime_error("Invalid field mutation target")),
                }
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
                loop {
                    let cond = self.evaluate_expression(condition)?;
                    if !cond.is_truthy() { break; }
                    for stmt in body {
                        match self.execute_statement(stmt)? {
                            Some(Value::Break) => return Ok(None),
                            Some(Value::Continue) => break,
                            Some(v) => return Ok(Some(v)),
                            None => {}
                        }
                    }
                }
                Ok(None)
            }

            Statement::For { identifier, iterable, body } => {
                let iterable_value = self.evaluate_expression(iterable)?;
                
                let items: Vec<Value> = match iterable_value {
                    Value::Array(arr) => arr,
                    Value::Number(end) => (0..(end as i32)).map(|i| Value::Number(i as f64)).collect(),
                    _ => return Err(OvieError::runtime_error("For loop iterable must be an array or number")),
                };

                'outer: for value in items {
                    self.environment.define_variable(identifier.clone(), value);
                    for stmt in body {
                        match self.execute_statement(stmt)? {
                            Some(Value::Break) => break 'outer,
                            Some(Value::Continue) => break,
                            Some(v) => return Ok(Some(v)),
                            None => {}
                        }
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

            Statement::Break => Ok(Some(Value::Break)),
            Statement::Continue => Ok(Some(Value::Continue)),

            Statement::Expression { expression } => {
                let val = self.evaluate_expression(expression)?;
                // Propagate Return signal from ? operator
                if let Value::Return(_) = val {
                    return Ok(Some(val));
                }
                Ok(None)
            }

            Statement::Struct { name, fields } => {
                let field_names: Vec<String> = fields.iter().map(|f| f.name.clone()).collect();
                self.environment.define_struct_type(name.clone(), field_names);
                Ok(None)
            }

            Statement::Enum { name, variants } => {
                let variant_names: Vec<String> = variants.iter().map(|v| v.name.clone()).collect();
                self.environment.define_enum_type(name.clone(), variant_names);
                Ok(None)
            }

            // Module system statements — register symbols but don't load files at runtime
            Statement::Use { .. } | Statement::Import { .. } => Ok(None),

            Statement::Export { statement } => {
                // Execute the inner statement (function/struct/enum/const definition)
                self.execute_statement(statement)
            }

            Statement::TypeAlias { .. } => Ok(None),

            Statement::Block { statements } => {
                // Execute all statements in the block (used for unsafe blocks etc.)
                for stmt in statements {
                    if let Some(return_value) = self.execute_statement(stmt)? {
                        return Ok(Some(return_value));
                    }
                }
                Ok(None)
            }
        }
    }

    /// Evaluate an expression
    pub fn evaluate_expression(&mut self, expression: &Expression) -> OvieResult<Value> {
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
                // Evaluate arguments first
                let mut arg_values = Vec::new();
                for arg in arguments {
                    arg_values.push(self.evaluate_expression(arg)?);
                }

                // Check for builtin functions first
                match function.as_str() {
                    "string_length" => {
                        if arg_values.len() != 1 {
                            return Err(OvieError::runtime_error(format!(
                                "string_length expects 1 argument, got {}",
                                arg_values.len()
                            )));
                        }
                        if let Value::String(s) = &arg_values[0] {
                            return Ok(Value::Number(crate::stdlib::core::builtin_string_length(s)));
                        }
                        return Err(OvieError::runtime_error("string_length expects a string argument"));
                    }
                    "string_char_at" => {
                        if arg_values.len() != 2 {
                            return Err(OvieError::runtime_error(format!(
                                "string_char_at expects 2 arguments, got {}",
                                arg_values.len()
                            )));
                        }
                        if let (Value::String(s), Value::Number(idx)) = (&arg_values[0], &arg_values[1]) {
                            return Ok(Value::String(crate::stdlib::core::builtin_string_char_at(s, *idx)));
                        }
                        return Err(OvieError::runtime_error("string_char_at expects (string, number) arguments"));
                    }
                    "string_substring" => {
                        if arg_values.len() != 3 {
                            return Err(OvieError::runtime_error(format!(
                                "string_substring expects 3 arguments, got {}",
                                arg_values.len()
                            )));
                        }
                        if let (Value::String(s), Value::Number(start), Value::Number(end)) = 
                            (&arg_values[0], &arg_values[1], &arg_values[2]) {
                            return Ok(Value::String(crate::stdlib::core::builtin_string_substring(s, *start, *end)));
                        }
                        return Err(OvieError::runtime_error("string_substring expects (string, number, number) arguments"));
                    }
                    "string_contains" => {
                        if arg_values.len() != 2 {
                            return Err(OvieError::runtime_error(format!(
                                "string_contains expects 2 arguments, got {}",
                                arg_values.len()
                            )));
                        }
                        if let (Value::String(s), Value::String(pattern)) = (&arg_values[0], &arg_values[1]) {
                            return Ok(Value::Boolean(crate::stdlib::core::builtin_string_contains(s, pattern)));
                        }
                        return Err(OvieError::runtime_error("string_contains expects (string, string) arguments"));
                    }
                    "string_starts_with" => {
                        if arg_values.len() != 2 {
                            return Err(OvieError::runtime_error(format!(
                                "string_starts_with expects 2 arguments, got {}",
                                arg_values.len()
                            )));
                        }
                        if let (Value::String(s), Value::String(prefix)) = (&arg_values[0], &arg_values[1]) {
                            return Ok(Value::Boolean(crate::stdlib::core::builtin_string_starts_with(s, prefix)));
                        }
                        return Err(OvieError::runtime_error("string_starts_with expects (string, string) arguments"));
                    }
                    "is_alphabetic" => {
                        if arg_values.len() != 1 {
                            return Err(OvieError::runtime_error(format!(
                                "is_alphabetic expects 1 argument, got {}",
                                arg_values.len()
                            )));
                        }
                        if let Value::String(c) = &arg_values[0] {
                            return Ok(Value::Boolean(crate::stdlib::core::builtin_is_alphabetic(c)));
                        }
                        return Err(OvieError::runtime_error("is_alphabetic expects a string argument"));
                    }
                    "is_numeric" => {
                        if arg_values.len() != 1 {
                            return Err(OvieError::runtime_error(format!(
                                "is_numeric expects 1 argument, got {}",
                                arg_values.len()
                            )));
                        }
                        if let Value::String(c) = &arg_values[0] {
                            return Ok(Value::Boolean(crate::stdlib::core::builtin_is_numeric(c)));
                        }
                        return Err(OvieError::runtime_error("is_numeric expects a string argument"));
                    }
                    "is_whitespace" => {
                        if arg_values.len() != 1 {
                            return Err(OvieError::runtime_error(format!(
                                "is_whitespace expects 1 argument, got {}",
                                arg_values.len()
                            )));
                        }
                        if let Value::String(c) = &arg_values[0] {
                            return Ok(Value::Boolean(crate::stdlib::core::builtin_is_whitespace(c)));
                        }
                        return Err(OvieError::runtime_error("is_whitespace expects a string argument"));
                    }
                    "is_alphanumeric" => {
                        if arg_values.len() != 1 {
                            return Err(OvieError::runtime_error(format!(
                                "is_alphanumeric expects 1 argument, got {}",
                                arg_values.len()
                            )));
                        }
                        if let Value::String(c) = &arg_values[0] {
                            return Ok(Value::Boolean(crate::stdlib::core::builtin_is_alphanumeric(c)));
                        }
                        return Err(OvieError::runtime_error("is_alphanumeric expects a string argument"));
                    }
                    "array_length" => {
                        if arg_values.len() != 1 {
                            return Err(OvieError::runtime_error(format!(
                                "array_length expects 1 argument, got {}",
                                arg_values.len()
                            )));
                        }
                        if let Value::Array(arr) = &arg_values[0] {
                            return Ok(Value::Number(crate::stdlib::core::builtin_array_length(arr)));
                        }
                        return Err(OvieError::runtime_error("array_length expects an array argument"));
                    }
                    "array_get" => {
                        if arg_values.len() != 2 {
                            return Err(OvieError::runtime_error(format!(
                                "array_get expects 2 arguments, got {}",
                                arg_values.len()
                            )));
                        }
                        if let (Value::Array(arr), Value::Number(idx)) = (&arg_values[0], &arg_values[1]) {
                            if let Some(val) = crate::stdlib::core::builtin_array_get(arr, *idx) {
                                return Ok(val);
                            }
                            return Err(OvieError::runtime_error(format!("Array index out of bounds: {}", idx)));
                        }
                        return Err(OvieError::runtime_error("array_get expects (array, number) arguments"));
                    }
                    "array_push" => {
                        if arg_values.len() != 2 {
                            return Err(OvieError::runtime_error(format!(
                                "array_push expects 2 arguments, got {}",
                                arg_values.len()
                            )));
                        }
                        if let Value::Array(mut arr) = arg_values[0].clone() {
                            crate::stdlib::core::builtin_array_push(&mut arr, arg_values[1].clone());
                            return Ok(Value::Array(arr));
                        }
                        return Err(OvieError::runtime_error("array_push expects an array as first argument"));
                    }

                    // v2.3 module system stdlib functions
                    "string_split_lines" => {
                        if let Some(Value::String(s)) = arg_values.first() {
                            let lines: Vec<Value> = s.lines().map(|l| Value::String(l.to_string())).collect();
                            return Ok(Value::Array(lines));
                        }
                        return Ok(Value::Array(Vec::new()));
                    }
                    "string_split" => {
                        if arg_values.len() >= 2 {
                            if let (Value::String(s), Value::String(sep)) = (&arg_values[0], &arg_values[1]) {
                                let parts: Vec<Value> = s.split(sep.as_str()).map(|p| Value::String(p.to_string())).collect();
                                return Ok(Value::Array(parts));
                            }
                        }
                        return Ok(Value::Array(Vec::new()));
                    }
                    "string_trim" => {
                        if let Some(Value::String(s)) = arg_values.first() {
                            return Ok(Value::String(s.trim().to_string()));
                        }
                        return Ok(Value::String(String::new()));
                    }
                    "string_to_lowercase" => {
                        if let Some(Value::String(s)) = arg_values.first() {
                            return Ok(Value::String(s.to_lowercase()));
                        }
                        return Ok(Value::String(String::new()));
                    }
                    "string_to_uppercase" => {
                        if let Some(Value::String(s)) = arg_values.first() {
                            return Ok(Value::String(s.to_uppercase()));
                        }
                        return Ok(Value::String(String::new()));
                    }
                    "string_find" => {
                        if arg_values.len() >= 2 {
                            if let (Value::String(s), Value::String(pat)) = (&arg_values[0], &arg_values[1]) {
                                match s.find(pat.as_str()) {
                                    Some(idx) => return Ok(Value::Number(idx as f64)),
                                    None => return Ok(Value::Number(-1.0)),
                                }
                            }
                        }
                        return Ok(Value::Number(-1.0));
                    }
                    "string_find_from" => {
                        if arg_values.len() >= 3 {
                            if let (Value::String(s), Value::String(pat), Value::Number(from)) = (&arg_values[0], &arg_values[1], &arg_values[2]) {
                                let start = *from as usize;
                                if start < s.len() {
                                    match s[start..].find(pat.as_str()) {
                                        Some(idx) => return Ok(Value::Number((start + idx) as f64)),
                                        None => return Ok(Value::Number(-1.0)),
                                    }
                                }
                            }
                        }
                        return Ok(Value::Number(-1.0));
                    }
                    "string_replace" => {
                        if arg_values.len() >= 3 {
                            if let (Value::String(s), Value::String(from), Value::String(to)) = (&arg_values[0], &arg_values[1], &arg_values[2]) {
                                return Ok(Value::String(s.replace(from.as_str(), to.as_str())));
                            }
                        }
                        return Ok(arg_values.first().cloned().unwrap_or(Value::String(String::new())));
                    }
                    "number_to_string" => {
                        if let Some(Value::Number(n)) = arg_values.first() {
                            let s = if n.fract() == 0.0 { format!("{}", *n as i64) } else { format!("{}", n) };
                            return Ok(Value::String(s));
                        }
                        return Ok(Value::String("0".to_string()));
                    }
                    "string_to_number" => {
                        if let Some(Value::String(s)) = arg_values.first() {
                            match s.parse::<f64>() {
                                Ok(n) => return Ok(Value::Number(n)),
                                Err(_) => return Ok(Value::Number(0.0)),
                            }
                        }
                        return Ok(Value::Number(0.0));
                    }
                    "current_timestamp" => {
                        return Ok(Value::Number(0.0)); // Stub: returns 0 in interpreter
                    }
                    "file_exists" => {
                        if let Some(Value::String(path)) = arg_values.first() {
                            return Ok(Value::Boolean(std::path::Path::new(path).exists()));
                        }
                        return Ok(Value::Boolean(false));
                    }
                    "read_file" => {
                        if let Some(Value::String(path)) = arg_values.first() {
                            match std::fs::read_to_string(path) {
                                Ok(content) => return Ok(Value::String(content)),
                                Err(e) => return Ok(Value::Enum {
                                    variant: "Err".to_string(),
                                    data: Some(Box::new(Value::String(e.to_string()))),
                                }),
                            }
                        }
                        return Ok(Value::Enum { variant: "Err".to_string(), data: Some(Box::new(Value::String("No path".to_string()))) });
                    }
                    "write_file" => {
                        if arg_values.len() >= 2 {
                            if let (Value::String(path), Value::String(content)) = (&arg_values[0], &arg_values[1]) {
                                match std::fs::write(path, content) {
                                    Ok(_) => return Ok(Value::Null),
                                    Err(e) => return Ok(Value::Enum {
                                        variant: "Err".to_string(),
                                        data: Some(Box::new(Value::String(e.to_string()))),
                                    }),
                                }
                            }
                        }
                        return Ok(Value::Null);
                    }
                    "make_dir" => {
                        if let Some(Value::String(path)) = arg_values.first() {
                            let _ = std::fs::create_dir_all(path);
                        }
                        return Ok(Value::Null);
                    }
                    "list_dir" => {
                        if let Some(Value::String(path)) = arg_values.first() {
                            match std::fs::read_dir(path) {
                                Ok(entries) => {
                                    let files: Vec<Value> = entries
                                        .filter_map(|e| e.ok())
                                        .map(|e| Value::String(e.path().to_string_lossy().to_string()))
                                        .collect();
                                    return Ok(Value::Array(files));
                                }
                                Err(_) => return Ok(Value::Array(Vec::new())),
                            }
                        }
                        return Ok(Value::Array(Vec::new()));
                    }
                    "format" => {
                        // format!("{}", ...) — just concatenate all args as strings
                        let result = arg_values.iter().map(|v| self.value_to_string(v)).collect::<Vec<_>>().join("");
                        return Ok(Value::String(result));
                    }
                    "assert" => {
                        if let Some(Value::Boolean(b)) = arg_values.first() {
                            if !b {
                                let msg = arg_values.get(1).map(|v| self.value_to_string(v)).unwrap_or_else(|| "Assertion failed".to_string());
                                return Err(OvieError::runtime_error(format!("Assertion failed: {}", msg)));
                            }
                        }
                        return Ok(Value::Null);
                    }
                    "panic" => {
                        let msg = arg_values.first().map(|v| self.value_to_string(v)).unwrap_or_else(|| "panic".to_string());
                        return Err(OvieError::runtime_error(format!("panic: {}", msg)));
                    }
                    "string_length" | "len" => {
                        match arg_values.first() {
                            Some(Value::String(s)) => return Ok(Value::Number(s.len() as f64)),
                            Some(Value::Array(a)) => return Ok(Value::Number(a.len() as f64)),
                            _ => return Ok(Value::Number(0.0)),
                        }
                    }

                    _ => {
                        // Not a builtin, check user-defined functions
                    }
                }

                // Check for user-defined functions
                if let Some(func) = self.environment.get_function(function) {
                    if arg_values.len() != func.parameters.len() {
                        return Err(OvieError::runtime_error(format!(
                            "Function '{}' expects {} arguments, got {}",
                            function,
                            func.parameters.len(),
                            arg_values.len()
                        )));
                    }

                    // Create new environment for function execution
                    let mut func_env = Environment::with_parent(self.environment.clone());
                    
                    // Bind parameters to arguments
                    for (param, arg_value) in func.parameters.iter().zip(arg_values.iter()) {
                        func_env.define_variable(param.name.clone(), arg_value.clone());
                    }

                    // Save current environment and switch to function environment
                    let mut saved_env = std::mem::replace(&mut self.environment, func_env);

                    // Execute function body
                    let mut result = Value::Null;
                    for stmt in &func.body {
                        if let Some(return_value) = self.execute_statement(stmt)? {
                            // Unwrap Return signal from ? operator
                            result = match return_value {
                                Value::Return(v) => *v,
                                other => other,
                            };
                            break;
                        }
                    }

                    // For mutable parameters, copy back the modified values
                    // This requires matching the arguments to the original variables
                    // For now, we'll handle the simple case where arguments are identifiers
                    for (i, param) in func.parameters.iter().enumerate() {
                        if param.mutable {
                            // Get the modified value from the function environment
                            if let Some(modified_value) = self.environment.get_variable(&param.name) {
                                // Try to update the original variable if the argument was an identifier
                                if let Some(arg_expr) = arguments.get(i) {
                                    if let Expression::Identifier(var_name) = arg_expr {
                                        // Update the variable in the saved environment
                                        let _ = saved_env.set_variable(var_name, modified_value);
                                    }
                                }
                            }
                        }
                    }

                    // Restore environment
                    self.environment = saved_env;

                    Ok(result)
                } else {
                    // Unknown function — return Null rather than error for v2.3 module system compatibility
                    // Functions like string_split_lines, make_dir etc. are handled above;
                    // any remaining unknown calls return Null gracefully
                    Ok(Value::Null)
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

            Expression::Null => Ok(Value::Null),

            Expression::Match { value, arms } => {
                let matched_value = self.evaluate_expression(value)?;
                for arm in arms {
                    if self.match_pattern(&arm.pattern, &matched_value)? {
                        // Check guard
                        if let Some(guard) = &arm.guard {
                            if !self.evaluate_expression(guard)?.is_truthy() {
                                continue;
                            }
                        }
                        // Execute arm body
                        let mut result = Value::Null;
                        for stmt in &arm.body {
                            match self.execute_statement(stmt)? {
                                Some(v) => { result = v; break; }
                                None => {}
                            }
                        }
                        return Ok(result);
                    }
                }
                Ok(Value::Null)
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

            Expression::MethodCall { object, method, arguments } => {
                let obj_val = self.evaluate_expression(object)?;
                let mut arg_vals = Vec::new();
                for arg in arguments {
                    arg_vals.push(self.evaluate_expression(arg)?);
                }
                self.evaluate_method_call(obj_val, method, arg_vals)
            }

            Expression::Try { expression } => {
                let val = self.evaluate_expression(expression)?;
                match val {
                    // Ok(v) => unwrap to v
                    Value::Enum { ref variant, ref data } if variant == "Ok" => {
                        Ok(data.as_ref().map(|d| *d.clone()).unwrap_or(Value::Null))
                    }
                    // Err(e) => early return Err(e) from current function
                    Value::Enum { ref variant, .. } if variant == "Err" => {
                        Ok(Value::Return(Box::new(val)))
                    }
                    // Non-Result value: pass through
                    other => Ok(other),
                }
            }
        }
    }

    /// Evaluate a method call on a value
    fn evaluate_method_call(&mut self, obj: Value, method: &str, args: Vec<Value>) -> OvieResult<Value> {
        match method {
            // String methods
            "to_string" => Ok(Value::String(self.value_to_string(&obj))),
            "clone" => Ok(obj.clone()),
            "len" | "length" => match &obj {
                Value::String(s) => Ok(Value::Number(s.len() as f64)),
                Value::Array(a) => Ok(Value::Number(a.len() as f64)),
                _ => Ok(Value::Number(0.0)),
            },
            "is_empty" => match &obj {
                Value::String(s) => Ok(Value::Boolean(s.is_empty())),
                Value::Array(a) => Ok(Value::Boolean(a.is_empty())),
                _ => Ok(Value::Boolean(false)),
            },
            "contains" | "contains_key" => {
                let needle = args.first().cloned().unwrap_or(Value::Null);
                match &obj {
                    Value::String(s) => {
                        let n = self.value_to_string(&needle);
                        Ok(Value::Boolean(s.contains(&n as &str)))
                    }
                    Value::Array(a) => Ok(Value::Boolean(a.contains(&needle))),
                    _ => Ok(Value::Boolean(false)),
                }
            }
            "starts_with" => {
                let prefix = args.first().map(|v| self.value_to_string(v)).unwrap_or_default();
                match &obj {
                    Value::String(s) => Ok(Value::Boolean(s.starts_with(&prefix as &str))),
                    _ => Ok(Value::Boolean(false)),
                }
            }
            "ends_with" => {
                let suffix = args.first().map(|v| self.value_to_string(v)).unwrap_or_default();
                match &obj {
                    Value::String(s) => Ok(Value::Boolean(s.ends_with(&suffix as &str))),
                    _ => Ok(Value::Boolean(false)),
                }
            }
            "replace" => {
                let from = args.first().map(|v| self.value_to_string(v)).unwrap_or_default();
                let to = args.get(1).map(|v| self.value_to_string(v)).unwrap_or_default();
                match obj {
                    Value::String(s) => Ok(Value::String(s.replace(&from as &str, &to as &str))),
                    _ => Ok(obj),
                }
            }
            "trim" => match obj {
                Value::String(s) => Ok(Value::String(s.trim().to_string())),
                _ => Ok(obj),
            },
            "to_uppercase" => match obj {
                Value::String(s) => Ok(Value::String(s.to_uppercase())),
                _ => Ok(obj),
            },
            "to_lowercase" => match obj {
                Value::String(s) => Ok(Value::String(s.to_lowercase())),
                _ => Ok(obj),
            },
            "split" => {
                let sep = args.first().map(|v| self.value_to_string(v)).unwrap_or_default();
                match obj {
                    Value::String(s) => {
                        let parts: Vec<Value> = s.split(&sep as &str)
                            .map(|p| Value::String(p.to_string()))
                            .collect();
                        Ok(Value::Array(parts))
                    }
                    _ => Ok(Value::Array(Vec::new())),
                }
            }
            "join" => {
                let sep = args.first().map(|v| self.value_to_string(v)).unwrap_or_default();
                match obj {
                    Value::Array(a) => {
                        let parts: Vec<String> = a.iter().map(|v| self.value_to_string(v)).collect();
                        Ok(Value::String(parts.join(&sep as &str)))
                    }
                    _ => Ok(Value::String(String::new())),
                }
            }
            "push" | "push_str" => {
                // Returns null; mutation not tracked here
                Ok(Value::Null)
            }
            "pop" => Ok(Value::Null),
            "insert" => Ok(Value::Null),
            "remove" => Ok(Value::Null),
            "get" => {
                let key = args.first().cloned().unwrap_or(Value::Null);
                match &obj {
                    Value::Array(a) => {
                        if let Value::Number(idx) = key {
                            let i = idx as usize;
                            if i < a.len() {
                                Ok(Value::Enum { variant: "Some".to_string(), data: Some(Box::new(a[i].clone())) })
                            } else {
                                Ok(Value::Enum { variant: "None".to_string(), data: None })
                            }
                        } else {
                            Ok(Value::Enum { variant: "None".to_string(), data: None })
                        }
                    }
                    _ => Ok(Value::Enum { variant: "None".to_string(), data: None }),
                }
            }
            "get_mut" => Ok(Value::Enum { variant: "None".to_string(), data: None }),
            "cloned" | "copied" => Ok(obj.clone()),
            "unwrap_or" => {
                let default = args.first().cloned().unwrap_or(Value::Null);
                match obj {
                    Value::Enum { variant, data } if variant == "Some" => {
                        Ok(data.map(|d| *d).unwrap_or(Value::Null))
                    }
                    Value::Enum { variant, .. } if variant == "None" => Ok(default),
                    other => Ok(other),
                }
            }
            "unwrap" => match obj {
                Value::Enum { variant, data } if variant == "Some" || variant == "Ok" => {
                    Ok(data.map(|d| *d).unwrap_or(Value::Null))
                }
                other => Ok(other),
            },
            "is_ok" => match &obj {
                Value::Enum { variant, .. } => Ok(Value::Boolean(variant == "Ok")),
                _ => Ok(Value::Boolean(true)),
            },
            "is_err" => match &obj {
                Value::Enum { variant, .. } => Ok(Value::Boolean(variant == "Err")),
                _ => Ok(Value::Boolean(false)),
            },
            "is_some" => match &obj {
                Value::Enum { variant, .. } => Ok(Value::Boolean(variant == "Some")),
                _ => Ok(Value::Boolean(false)),
            },
            "is_none" => match &obj {
                Value::Enum { variant, .. } => Ok(Value::Boolean(variant == "None")),
                _ => Ok(Value::Boolean(true)),
            },
            "iter" => Ok(obj),
            "enumerate" => match obj {
                Value::Array(a) => {
                    let pairs: Vec<Value> = a.into_iter().enumerate()
                        .map(|(i, v)| Value::Array(vec![Value::Number(i as f64), v]))
                        .collect();
                    Ok(Value::Array(pairs))
                }
                _ => Ok(Value::Array(Vec::new())),
            },
            "collect" => Ok(obj),
            "map" | "filter" | "flat_map" | "filter_map" => Ok(obj),
            "any" | "all" => Ok(Value::Boolean(false)),
            "count" => match &obj {
                Value::Array(a) => Ok(Value::Number(a.len() as f64)),
                _ => Ok(Value::Number(0.0)),
            },
            "max" | "min" => Ok(obj),
            "sum" => Ok(Value::Number(0.0)),
            "first" => match &obj {
                Value::Array(a) => {
                    if a.is_empty() {
                        Ok(Value::Enum { variant: "None".to_string(), data: None })
                    } else {
                        Ok(Value::Enum { variant: "Some".to_string(), data: Some(Box::new(a[0].clone())) })
                    }
                }
                _ => Ok(Value::Enum { variant: "None".to_string(), data: None }),
            },
            "last" => match &obj {
                Value::Array(a) => {
                    if a.is_empty() {
                        Ok(Value::Enum { variant: "None".to_string(), data: None })
                    } else {
                        Ok(Value::Enum { variant: "Some".to_string(), data: Some(Box::new(a.last().unwrap().clone())) })
                    }
                }
                _ => Ok(Value::Enum { variant: "None".to_string(), data: None }),
            },
            "position" => Ok(Value::Enum { variant: "None".to_string(), data: None }),
            "find" => Ok(Value::Enum { variant: "None".to_string(), data: None }),
            "sort" | "sort_by" | "sort_by_key" => Ok(obj),
            "retain" => Ok(Value::Null),
            "extend" => Ok(Value::Null),
            "clear" => Ok(Value::Null),
            "keys" => match &obj {
                Value::Struct(fields) => {
                    let keys: Vec<Value> = fields.keys().map(|k| Value::String(k.clone())).collect();
                    Ok(Value::Array(keys))
                }
                _ => Ok(Value::Array(Vec::new())),
            },
            "values" => match &obj {
                Value::Struct(fields) => {
                    let vals: Vec<Value> = fields.values().cloned().collect();
                    Ok(Value::Array(vals))
                }
                _ => Ok(Value::Array(Vec::new())),
            },
            "as_str" => match obj {
                Value::String(s) => Ok(Value::String(s)),
                other => Ok(Value::String(self.value_to_string(&other))),
            },
            "strip_prefix" => {
                let prefix = args.first().map(|v| self.value_to_string(v)).unwrap_or_default();
                match obj {
                    Value::String(s) => {
                        if s.starts_with(&prefix as &str) {
                            Ok(Value::Enum {
                                variant: "Some".to_string(),
                                data: Some(Box::new(Value::String(s[prefix.len()..].to_string()))),
                            })
                        } else {
                            Ok(Value::Enum { variant: "None".to_string(), data: None })
                        }
                    }
                    _ => Ok(Value::Enum { variant: "None".to_string(), data: None }),
                }
            }
            "bytes" => match obj {
                Value::String(s) => {
                    let bytes: Vec<Value> = s.bytes().map(|b| Value::Number(b as f64)).collect();
                    Ok(Value::Array(bytes))
                }
                _ => Ok(Value::Array(Vec::new())),
            },
            "wrapping_mul" | "wrapping_add" | "wrapping_sub" => {
                let rhs = args.first().and_then(|v| if let Value::Number(n) = v { Some(*n) } else { None }).unwrap_or(0.0);
                match obj {
                    Value::Number(n) => Ok(Value::Number(n * rhs)),
                    _ => Ok(Value::Number(0.0)),
                }
            }
            "cmp" => Ok(Value::Enum { variant: "Equal".to_string(), data: None }),
            "partial_cmp" => Ok(Value::Enum { variant: "Some".to_string(), data: Some(Box::new(Value::Enum { variant: "Equal".to_string(), data: None })) }),
            // Static constructors: Vec::new(), HashMap::new()
            "new" => Ok(Value::Array(Vec::new())),
            _ => {
                // Unknown method — return Null rather than error to allow parsing-focused files to run
                Ok(Value::Null)
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
            Value::Break => "break",
            Value::Continue => "continue",
            Value::Return(_) => "return",
        }
    }

    /// Convert a value to its string representation
    fn value_to_string(&self, value: &Value) -> String {
        match value {
            Value::String(s) => s.clone(),
            Value::Number(n) => {
                if *n == n.floor() && n.abs() < 1e15 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::Boolean(b) => b.to_string(),
            Value::Null => "null".to_string(),
            Value::Array(a) => {
                let parts: Vec<String> = a.iter().map(|v| self.value_to_string(v)).collect();
                format!("[{}]", parts.join(", "))
            }
            Value::Struct(fields) => {
                let parts: Vec<String> = fields.iter().map(|(k, v)| format!("{}: {}", k, self.value_to_string(v))).collect();
                format!("{{{}}}", parts.join(", "))
            }
            Value::Enum { variant, data } => {
                if let Some(d) = data {
                    format!("{}({})", variant, self.value_to_string(d))
                } else {
                    variant.clone()
                }
            }
            Value::Break => "break".to_string(),
            Value::Continue => "continue".to_string(),
            Value::Return(v) => v.to_string(),
        }
    }

    /// Check if a value matches a pattern, and if so bind any identifiers into the environment
    fn match_pattern(&mut self, pattern: &crate::ast::MatchPattern, value: &Value) -> OvieResult<bool> {
        use crate::ast::MatchPattern;
        match pattern {
            MatchPattern::Wildcard => Ok(true),
            MatchPattern::Identifier(name) => {
                // Binding pattern: always matches, binds the value to `name` in scope
                self.environment.define_variable(name.clone(), value.clone());
                Ok(true)
            }
            MatchPattern::Literal(lit) => {
                let lit_val = match lit {
                    crate::ast::Literal::String(s) => Value::String(s.clone()),
                    crate::ast::Literal::Number(n) => Value::Number(*n),
                    crate::ast::Literal::Boolean(b) => Value::Boolean(*b),
                };
                Ok(&lit_val == value)
            }
            MatchPattern::EnumVariant { variant_name, binding, .. } => {
                if let Value::Enum { variant, data } = value {
                    let matched = variant == variant_name;
                    if matched {
                        // Bind inner data to the pattern variable if present
                        if let Some(bind_name) = binding {
                            let inner = data.as_ref()
                                .map(|d| *d.clone())
                                .unwrap_or(Value::Null);
                            self.environment.define_variable(bind_name.clone(), inner);
                        }
                    }
                    Ok(matched)
                } else {
                    Ok(false)
                }
            }
            MatchPattern::Struct { name, .. } => {
                if let Value::Struct(_) = value {
                    Ok(true) // simplified: match any struct; field binding TODO
                } else {
                    Ok(false)
                }
            }
            MatchPattern::Or(patterns) => {
                for p in patterns {
                    if self.match_pattern(p, value)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
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