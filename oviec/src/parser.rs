//! Parser for the Ovie programming language

use crate::ast::{
    AstNode, Statement, Expression, Literal, BinaryOperator, UnaryOperator,
    StructField, EnumVariant, FieldInitializer, Parameter,
    MatchArm, MatchPattern, UseItems, UseItem,
};
use crate::error::OvieError;
use crate::lexer::{Token, TokenType};

/// Result type for parsing operations
pub type ParseResult<T> = Result<T, OvieError>;

/// Parser for Ovie source code
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Create a new parser with the given tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Parse the tokens into an AST
    pub fn parse(&mut self) -> ParseResult<AstNode> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if self.check(&TokenType::Eof) {
                break;
            }
            statements.push(self.statement()?);
        }

        Ok(AstNode::new(statements))
    }

    /// Parse a statement
    fn statement(&mut self) -> ParseResult<Statement> {
        match &self.peek().token_type {
            TokenType::Fn => self.function_statement(),
            TokenType::SeeAm => self.print_statement(),
            TokenType::If => self.if_statement(),
            TokenType::While => self.while_statement(),
            TokenType::For => self.for_statement(),
            TokenType::Return => self.return_statement(),
            TokenType::Struct => self.struct_statement(),
            TokenType::Enum => self.enum_statement(),
            TokenType::Let => self.let_statement(),
            TokenType::Const => self.const_statement(),
            TokenType::Use => self.use_statement(),
            TokenType::Import => self.import_statement(),
            TokenType::Export => self.export_statement(),
            TokenType::Pub => self.pub_statement(),
            TokenType::Type => self.type_alias_statement(),
            TokenType::Unsafe => {
                // unsafe { ... } — parse as a transparent block (no safety enforcement in v2.3 interpreter)
                self.advance(); // consume 'unsafe'
                let body = self.block_statement()?;
                Ok(Statement::Block { statements: body })
            }
            TokenType::Match => {
                // match as a statement — parse as expression statement
                let expr = self.match_expression()?;
                self.consume_optional_semicolon();
                Ok(Statement::Expression { expression: expr })
            }
            TokenType::Break => {
                self.advance();
                self.consume_optional_semicolon();
                Ok(Statement::Break)
            }
            TokenType::Continue => {
                self.advance();
                self.consume_optional_semicolon();
                Ok(Statement::Continue)
            }
            TokenType::Mut => self.assignment_statement(true),
            TokenType::Identifier => {
                // Check for 'static' keyword (static mut IDENT: Type = expr)
                if self.peek().lexeme == "static" {
                    self.advance(); // consume 'static'
                    let mutable = self.match_token(&TokenType::Mut);
                    let identifier = self.consume_identifier("Expected variable name after 'static'")?;
                    if self.match_token(&TokenType::Colon) {
                        self.skip_type_annotation()?;
                    }
                    self.consume(&TokenType::Equal, "Expected '=' in static declaration")?;
                    let value = self.expression()?;
                    self.consume_optional_semicolon();
                    return Ok(Statement::VariableDeclaration { mutable, identifier, value });
                }
                // Look ahead to see if this is an assignment, compound assignment, or field mutation
                let next = self.tokens.get(self.current + 1).map(|t| &t.token_type);
                match next {
                    Some(TokenType::Equal) => self.assignment_statement(false),
                    Some(TokenType::PlusEqual) | Some(TokenType::MinusEqual)
                    | Some(TokenType::StarEqual) | Some(TokenType::SlashEqual) => {
                        self.compound_assignment_statement()
                    }
                    Some(TokenType::Dot) => {
                        // Check if this is field mutation: identifier.field = value
                        let mut lookahead = self.current + 2;
                        let mut found_mutation = false;
                        while lookahead < self.tokens.len() {
                            match &self.tokens[lookahead].token_type {
                                TokenType::Identifier => {
                                    if lookahead + 1 < self.tokens.len() {
                                        match &self.tokens[lookahead + 1].token_type {
                                            TokenType::Equal => {
                                                found_mutation = true;
                                                break;
                                            }
                                            TokenType::Dot => {
                                                lookahead += 2;
                                            }
                                            _ => break,
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                _ => break,
                            }
                        }
                        if found_mutation {
                            return self.field_mutation_statement();
                        }
                        self.expression_statement()
                    }
                    _ => self.expression_statement(),
                }
            }
            _ => self.expression_statement(),
        }
    }

    /// Parse a function definition
    fn function_statement(&mut self) -> ParseResult<Statement> {
        self.consume(&TokenType::Fn, "Expected 'fn'")?;
        
        let name = self.consume_identifier("Expected function name")?;
        
        self.consume(&TokenType::LeftParen, "Expected '(' after function name")?;
        
        let mut parameters = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                // Check for 'mut' keyword before parameter name
                let mutable = self.match_token(&TokenType::Mut);
                let param_name = self.consume_identifier("Expected parameter name")?;
                // Skip optional type annotation: param: Type
                if self.match_token(&TokenType::Colon) {
                    self.skip_type_annotation()?;
                }
                parameters.push(Parameter {
                    name: param_name,
                    mutable,
                });
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }
        
        self.consume(&TokenType::RightParen, "Expected ')' after parameters")?;
        
        // Skip optional return type annotation: -> Type
        if self.match_token(&TokenType::Arrow) {
            self.skip_type_annotation()?;
        }
        
        let body = self.block_statement()?;
        
        Ok(Statement::Function {
            name,
            parameters,
            body,
        })
    }

    /// Parse a print statement
    fn print_statement(&mut self) -> ParseResult<Statement> {
        self.consume(&TokenType::SeeAm, "Expected 'seeAm'")?;
        let expression = self.expression()?;
        // Ovie doesn't require semicolons after statements
        // Optional semicolon for compatibility
        if self.check(&TokenType::Semicolon) {
            self.advance();
        }
        
        Ok(Statement::Print { expression })
    }

   /// Parse an if statement
fn if_statement(&mut self) -> ParseResult<Statement> {
    self.consume(&TokenType::If, "Expected 'if'")?;
    
    // Handle `if let Pattern = expr` — treat as `if expr` (binding ignored for now)
    let condition = if self.check(&TokenType::Let) {
        self.advance(); // consume 'let'
        // Skip the pattern (could be Some(x), Some(&x), Ok(v), Err(e), etc.)
        // Consume tokens until we hit '=' at depth 0 (not inside parens/brackets)
        let mut depth = 0;
        while !self.is_at_end() {
            if self.check(&TokenType::LeftParen) || self.check(&TokenType::LeftBracket) {
                depth += 1;
                self.advance();
            } else if self.check(&TokenType::RightParen) || self.check(&TokenType::RightBracket) {
                if depth == 0 { break; }
                depth -= 1;
                self.advance();
            } else if self.check(&TokenType::Equal) && depth == 0 {
                break;
            } else if self.check(&TokenType::Ampersand) {
                // Handle & and &mut in patterns like Some(&id)
                self.advance();
                if self.check(&TokenType::Mut) {
                    self.advance();
                }
            } else {
                self.advance();
            }
        }
        self.consume(&TokenType::Equal, "Expected '=' in if let")?;
        self.expression()?
    } else {
        self.expression()?
    };
    
    let then_block = self.block_statement()?;
    
    let else_block = if self.match_token(&TokenType::Else) {
        // Check if this is 'else if' (chained if statement)
        if self.check(&TokenType::If) {
            // Parse the if statement as a single-statement else block
            // This allows 'else if' to work naturally through recursion
            Some(vec![self.if_statement()?])
        } else {
            // Regular else block with braces
            Some(self.block_statement()?)
        }
    } else {
        None
    };
    
    Ok(Statement::If {
        condition,
        then_block,
        else_block,
    })
}


    /// Parse a while statement
    fn while_statement(&mut self) -> ParseResult<Statement> {
        self.consume(&TokenType::While, "Expected 'while'")?;
        let condition = self.expression()?;
        let body = self.block_statement()?;
        
        Ok(Statement::While { condition, body })
    }

    /// Parse a for statement
    fn for_statement(&mut self) -> ParseResult<Statement> {
        self.consume(&TokenType::For, "Expected 'for'")?;
        
        // Handle tuple destructuring: for (a, b) in expr
        let identifier = if self.check(&TokenType::LeftParen) {
            self.advance(); // consume '('
            // Collect tuple elements
            let mut parts = Vec::new();
            while !self.check(&TokenType::RightParen) && !self.is_at_end() {
                if self.check(&TokenType::Identifier) {
                    parts.push(self.advance().lexeme.clone());
                } else {
                    // Accept keywords too
                    parts.push(self.advance().lexeme.clone());
                }
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
            self.consume(&TokenType::RightParen, "Expected ')' after tuple pattern")?;
            // Join with comma for now (interpreter will need to handle this)
            parts.join(",")
        } else if self.check(&TokenType::Identifier) {
            self.advance().lexeme.clone()
        } else {
            // Accept keywords as loop variable names (e.g., `import`, `use`, `type`)
            let tok = self.advance();
            tok.lexeme.clone()
        };
        
        self.consume(&TokenType::In, "Expected 'in' after loop variable")?;
        let iterable = self.expression()?;
        let body = self.block_statement()?;
        
        Ok(Statement::For {
            identifier,
            iterable,
            body,
        })
    }

    /// Parse a return statement
    fn return_statement(&mut self) -> ParseResult<Statement> {
        self.consume(&TokenType::Return, "Expected 'return'")?;
        
        let value = if self.check(&TokenType::Semicolon) || self.check(&TokenType::RightBrace) {
            None
        } else {
            Some(self.expression()?)
        };
        
        self.consume_optional_semicolon();
        
        Ok(Statement::Return { value })
    }

    /// Parse a struct definition
    fn struct_statement(&mut self) -> ParseResult<Statement> {
        self.consume(&TokenType::Struct, "Expected 'struct'")?;
        let name = self.consume_identifier("Expected struct name")?;
        
        self.consume(&TokenType::LeftBrace, "Expected '{' after struct name")?;
        
        let mut fields = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let field_name = self.consume_identifier("Expected field name")?;
            self.consume(&TokenType::Colon, "Expected ':' after field name")?;
            // Collect the full type annotation including generics like Vec<T>, Option<Vec<String>>
            let type_annotation = self.consume_type_annotation()?;
            
            fields.push(StructField {
                name: field_name,
                type_annotation,
            });
            
            if !self.match_token(&TokenType::Comma) {
                break;
            }
        }
        
        self.consume(&TokenType::RightBrace, "Expected '}' after struct fields")?;
        
        Ok(Statement::Struct { name, fields })
    }

    /// Consume a full type annotation as a string, including generics, references, and slices
    fn consume_type_annotation(&mut self) -> ParseResult<String> {
        let mut type_str = String::new();

        // Handle reference types: &T or &mut T
        if self.check(&TokenType::Ampersand) {
            self.advance();
            type_str.push('&');
            if self.check(&TokenType::Mut) {
                self.advance();
                type_str.push_str("mut ");
            }
            let inner = self.consume_type_annotation()?;
            type_str.push_str(&inner);
            return Ok(type_str);
        }

        // Handle tuple types: (T, U)
        if self.check(&TokenType::LeftParen) {
            self.advance();
            type_str.push('(');
            while !self.check(&TokenType::RightParen) && !self.is_at_end() {
                type_str.push_str(&self.consume_type_annotation()?);
                if self.check(&TokenType::Comma) {
                    self.advance();
                    type_str.push_str(", ");
                } else {
                    break;
                }
            }
            if self.check(&TokenType::RightParen) { self.advance(); }
            type_str.push(')');
            return Ok(type_str);
        }

        // Handle slice/array types: [T]
        if self.check(&TokenType::LeftBracket) {
            self.advance();
            type_str.push('[');
            type_str.push_str(&self.consume_type_annotation()?);
            if self.check(&TokenType::Semicolon) {
                self.advance();
                type_str.push(';');
                if self.check(&TokenType::IntegerLiteral) {
                    type_str.push_str(&self.advance().lexeme.clone());
                }
            }
            if self.check(&TokenType::RightBracket) { self.advance(); }
            type_str.push(']');
            return Ok(type_str);
        }

        // Consume base type name (may be path like std::core::Vec)
        if self.check(&TokenType::Identifier) {
            type_str.push_str(&self.advance().lexeme.clone());
            while self.check(&TokenType::ColonColon) {
                self.advance();
                type_str.push_str("::");
                if self.check(&TokenType::Identifier) {
                    type_str.push_str(&self.advance().lexeme.clone());
                }
            }
        }
        // Handle generic params <T, U, ...> with nesting
        if self.check(&TokenType::Less) {
            self.advance();
            type_str.push('<');
            let mut depth = 1;
            while depth > 0 && !self.is_at_end() {
                match self.peek().token_type {
                    TokenType::Less => {
                        depth += 1;
                        type_str.push('<');
                        self.advance();
                    }
                    TokenType::Greater => {
                        depth -= 1;
                        type_str.push('>');
                        self.advance();
                    }
                    TokenType::Comma => {
                        type_str.push_str(", ");
                        self.advance();
                    }
                    TokenType::Identifier => {
                        type_str.push_str(&self.advance().lexeme.clone());
                    }
                    TokenType::ColonColon => {
                        type_str.push_str("::");
                        self.advance();
                    }
                    TokenType::Ampersand => {
                        type_str.push('&');
                        self.advance();
                    }
                    _ => { self.advance(); }
                }
            }
        }
        if type_str.is_empty() {
            // Return unit type rather than error — allows bare () return types
            return Ok("()".to_string());
        }
        Ok(type_str)
    }

    /// Parse an enum definition
    fn enum_statement(&mut self) -> ParseResult<Statement> {
        self.consume(&TokenType::Enum, "Expected 'enum'")?;
        let name = self.consume_identifier("Expected enum name")?;
        
        self.consume(&TokenType::LeftBrace, "Expected '{' after enum name")?;
        
        let mut variants = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let variant_name = self.consume_identifier("Expected variant name")?;
            
            let data_type = if self.match_token(&TokenType::LeftParen) {
                let type_name = self.consume_type_annotation()?;
                self.consume(&TokenType::RightParen, "Expected ')' after variant data type")?;
                Some(type_name)
            } else {
                None
            };
            
            variants.push(EnumVariant {
                name: variant_name,
                data_type,
            });
            
            if !self.match_token(&TokenType::Comma) {
                break;
            }
        }
        
        self.consume(&TokenType::RightBrace, "Expected '}' after enum variants")?;
        
        Ok(Statement::Enum { name, variants })
    }

    /// Parse an assignment statement
    fn assignment_statement(&mut self, mutable: bool) -> ParseResult<Statement> {
        if mutable {
            self.consume(&TokenType::Mut, "Expected 'mut'")?;
        }
        
        let identifier = self.consume_identifier("Expected variable name")?;
        
        // Skip optional type annotation: mut x: Type = expr
        if self.match_token(&TokenType::Colon) {
            self.skip_type_annotation()?;
        }
        
        self.consume(&TokenType::Equal, "Expected '=' in assignment")?;
        let value = self.expression()?;
        self.consume_optional_semicolon();
        
        Ok(Statement::Assignment {
            mutable,
            identifier,
            value,
        })
    }

    /// Parse a let statement: let [mut] identifier [: Type] = expression
    fn let_statement(&mut self) -> ParseResult<Statement> {
        self.consume(&TokenType::Let, "Expected 'let'")?;
        let mutable = self.match_token(&TokenType::Mut);
        
        // Handle tuple destructuring: let (a, b) = expr
        let identifier = if self.check(&TokenType::LeftParen) {
            self.advance(); // consume '('
            // Collect tuple elements
            let mut parts = Vec::new();
            while !self.check(&TokenType::RightParen) && !self.is_at_end() {
                parts.push(self.consume_identifier("Expected identifier in tuple pattern")?);
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
            self.consume(&TokenType::RightParen, "Expected ')' after tuple pattern")?;
            // Join with comma for now (interpreter will need to handle this)
            parts.join(",")
        } else {
            self.consume_identifier("Expected variable name")?
        };
        
        // Skip optional type annotation
        if self.match_token(&TokenType::Colon) {
            self.skip_type_annotation()?;
        }
        self.consume(&TokenType::Equal, "Expected '=' in let binding")?;
        let value = self.expression()?;
        self.consume_optional_semicolon();
        Ok(Statement::VariableDeclaration { mutable, identifier, value })
    }

    /// Parse a const declaration: const NAME [: Type] = expression
    fn const_statement(&mut self) -> ParseResult<Statement> {
        self.consume(&TokenType::Const, "Expected 'const'")?;
        let name = self.consume_identifier("Expected constant name")?;
        if self.match_token(&TokenType::Colon) {
            self.skip_type_annotation()?;
        }
        self.consume(&TokenType::Equal, "Expected '=' in const declaration")?;
        let value = self.expression()?;
        self.consume_optional_semicolon();
        Ok(Statement::ConstDeclaration { name, value })
    }

    /// Parse a compound assignment: identifier op= expression
    fn compound_assignment_statement(&mut self) -> ParseResult<Statement> {
        let identifier = self.consume_identifier("Expected variable name")?;
        let operator = match &self.peek().token_type {
            TokenType::PlusEqual => { self.advance(); BinaryOperator::Add }
            TokenType::MinusEqual => { self.advance(); BinaryOperator::Subtract }
            TokenType::StarEqual => { self.advance(); BinaryOperator::Multiply }
            TokenType::SlashEqual => { self.advance(); BinaryOperator::Divide }
            _ => return Err(self.error("Expected compound assignment operator")),
        };
        let value = self.expression()?;
        self.consume_optional_semicolon();
        Ok(Statement::CompoundAssignment { identifier, operator, value })
    }

    /// Parse a use statement: use path::to::module or use path::{a, b} or use path as alias
    fn use_statement(&mut self) -> ParseResult<Statement> {
        self.consume(&TokenType::Use, "Expected 'use'")?;
        let path = self.parse_module_path()?;
        
        let items = if self.match_token(&TokenType::As) {
            let alias = self.consume_identifier("Expected alias name")?;
            UseItems::Alias(alias)
        } else if self.check(&TokenType::ColonColon) {
            self.advance(); // consume ::
            if self.check(&TokenType::Star) {
                self.advance();
                UseItems::All
            } else if self.check(&TokenType::LeftBrace) {
                self.advance(); // consume {
                let mut items = Vec::new();
                while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
                    let name = self.consume_identifier("Expected import name")?;
                    let alias = if self.match_token(&TokenType::As) {
                        Some(self.consume_identifier("Expected alias")?)
                    } else {
                        None
                    };
                    items.push(UseItem { name, alias });
                    if !self.match_token(&TokenType::Comma) {
                        break;
                    }
                }
                self.consume(&TokenType::RightBrace, "Expected '}'")?;
                UseItems::Named(items)
            } else {
                UseItems::Module
            }
        } else {
            UseItems::Module
        };
        
        self.consume_optional_semicolon();
        Ok(Statement::Use { path, items })
    }

    /// Parse an import statement: import "path"
    fn import_statement(&mut self) -> ParseResult<Statement> {
        self.consume(&TokenType::Import, "Expected 'import'")?;
        let path = if self.check(&TokenType::StringLiteral) {
            let tok = self.advance();
            let raw = tok.lexeme.clone();
            self.parse_string_literal(&raw)?
        } else {
            self.consume_identifier("Expected import path")?
        };
        self.consume_optional_semicolon();
        Ok(Statement::Import { path })
    }

    /// Parse an export statement: export fn/struct/enum/const/use ...
    fn export_statement(&mut self) -> ParseResult<Statement> {
        self.consume(&TokenType::Export, "Expected 'export'")?;
        // Handle `export use path::{...}` re-export syntax
        if self.check(&TokenType::Use) {
            let inner = self.use_statement()?;
            return Ok(Statement::Export { statement: Box::new(inner) });
        }
        let inner = self.statement()?;
        Ok(Statement::Export { statement: Box::new(inner) })
    }

    /// Parse a pub statement (treat as export)
    fn pub_statement(&mut self) -> ParseResult<Statement> {
        self.consume(&TokenType::Pub, "Expected 'pub'")?;
        let inner = self.statement()?;
        Ok(Statement::Export { statement: Box::new(inner) })
    }

    /// Parse a type alias: type Name<T> = AliasedType<T>;
    fn type_alias_statement(&mut self) -> ParseResult<Statement> {
        self.consume(&TokenType::Type, "Expected 'type'")?;
        let name = self.consume_identifier("Expected type alias name")?;
        // Skip optional generic params like <T>
        if self.check(&TokenType::Less) {
            self.advance();
            let mut depth = 1;
            while !self.is_at_end() && depth > 0 {
                if self.check(&TokenType::Less) { depth += 1; }
                else if self.check(&TokenType::Greater) { depth -= 1; }
                self.advance();
            }
        }
        self.consume(&TokenType::Equal, "Expected '=' in type alias")?;
        let aliased_type = self.consume_type_annotation()?;
        self.consume_optional_semicolon();
        Ok(Statement::TypeAlias { name, aliased_type })
    }

    /// Parse a module path like std::core or ./loader
    fn parse_module_path(&mut self) -> ParseResult<Vec<String>> {
        let mut path = Vec::new();
        // Handle relative paths starting with ./ or ../
        if self.check(&TokenType::Dot) {
            self.advance();
            path.push(".".to_string());
            if self.check(&TokenType::Slash) {
                self.advance();
            }
        }
        if self.check(&TokenType::Identifier) {
            path.push(self.advance().lexeme.clone());
            while self.check(&TokenType::ColonColon) {
                // Peek ahead — only consume :: if followed by identifier (not { or *)
                let next_is_ident = self.tokens.get(self.current + 1)
                    .map(|t| matches!(t.token_type, TokenType::Identifier))
                    .unwrap_or(false);
                if !next_is_ident {
                    break; // leave :: for use_statement to handle
                }
                self.advance(); // consume ::
                if self.check(&TokenType::Identifier) {
                    path.push(self.advance().lexeme.clone());
                } else {
                    break;
                }
            }
        }
        Ok(path)
    }

    /// Skip a type annotation (identifier, possibly with generics, references, or slices)
    /// Handles: T, &T, &mut T, Vec<T>, Option<T>, Result<T,E>, (T, U), [T]
    fn skip_type_annotation(&mut self) -> ParseResult<()> {
        // Handle reference types: &T or &mut T
        if self.check(&TokenType::Ampersand) {
            self.advance(); // consume '&'
            // Optional 'mut'
            if self.check(&TokenType::Mut) {
                self.advance();
            }
            // Recurse to handle the inner type
            return self.skip_type_annotation();
        }

        // Handle tuple types: (T, U, ...)
        if self.check(&TokenType::LeftParen) {
            self.advance(); // consume '('
            while !self.check(&TokenType::RightParen) && !self.is_at_end() {
                self.skip_type_annotation()?;
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
            if self.check(&TokenType::RightParen) { self.advance(); }
            return Ok(());
        }

        // Handle slice/array types: [T] or [T; N]
        if self.check(&TokenType::LeftBracket) {
            self.advance(); // consume '['
            self.skip_type_annotation()?;
            // Optional '; N' for fixed-size arrays
            if self.check(&TokenType::Semicolon) {
                self.advance();
                if self.check(&TokenType::IntegerLiteral) { self.advance(); }
            }
            if self.check(&TokenType::RightBracket) { self.advance(); }
            return Ok(());
        }

        // consume the type name (could be path like std::core::Vec)
        if self.check(&TokenType::Identifier) {
            self.advance();
            while self.check(&TokenType::ColonColon) {
                self.advance();
                if self.check(&TokenType::Identifier) { self.advance(); }
            }
        }
        // skip generic params <T, U>
        if self.check(&TokenType::Less) {
            self.advance();
            let mut depth = 1;
            while depth > 0 && !self.is_at_end() {
                match &self.peek().token_type {
                    TokenType::Less => { depth += 1; self.advance(); }
                    TokenType::Greater => { depth -= 1; self.advance(); }
                    _ => { self.advance(); }
                }
            }
        }
        Ok(())
    }

    /// Consume an optional semicolon
    fn consume_optional_semicolon(&mut self) {
        if self.check(&TokenType::Semicolon) {
            self.advance();
        }
    }

    /// Parse a field mutation statement: object.field = value
    fn field_mutation_statement(&mut self) -> ParseResult<Statement> {
        // Parse the object (can be a simple identifier or nested field access)
        let mut object = Expression::Identifier(self.consume_identifier("Expected object name")?);
        
        // Parse field access chain
        let mut field_name = String::new();
        while self.match_token(&TokenType::Dot) {
            field_name = self.consume_identifier("Expected field name after '.'")?;
            
            // Check if there's another dot (nested field access)
            if self.check(&TokenType::Dot) {
                // Build up the object expression
                object = Expression::FieldAccess {
                    object: Box::new(object),
                    field: field_name.clone(),
                };
            } else {
                // This is the final field to mutate
                break;
            }
        }
        
        self.consume(&TokenType::Equal, "Expected '=' in field mutation")?;
        let value = self.expression()?;
        self.consume_optional_semicolon();
        
        Ok(Statement::FieldMutation {
            object,
            field: field_name,
            value,
        })
    }

    /// Parse an expression statement
    fn expression_statement(&mut self) -> ParseResult<Statement> {
        let expression = self.expression()?;
        self.consume_optional_semicolon();
        
        Ok(Statement::Expression { expression })
    }

    /// Parse a block statement
    fn block_statement(&mut self) -> ParseResult<Vec<Statement>> {
        self.consume(&TokenType::LeftBrace, "Expected '{'")?;
        
        let mut statements = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.statement()?);
        }
        
        self.consume(&TokenType::RightBrace, "Expected '}'")?;
        
        Ok(statements)
    }

    /// Parse an expression
    fn expression(&mut self) -> ParseResult<Expression> {
        self.logical_or()
    }

    /// Parse logical OR expression
    fn logical_or(&mut self) -> ParseResult<Expression> {
        let mut expr = self.logical_and()?;

        while self.match_token(&TokenType::OrOr) {
            let right = self.logical_and()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOperator::Or,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parse logical AND expression
    fn logical_and(&mut self) -> ParseResult<Expression> {
        let mut expr = self.equality()?;

        while self.match_token(&TokenType::AndAnd) {
            let right = self.equality()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOperator::And,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parse equality expression
    fn equality(&mut self) -> ParseResult<Expression> {
        let mut expr = self.comparison()?;

        while let Some(operator) = self.match_equality_operator() {
            let right = self.comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parse comparison expression
    fn comparison(&mut self) -> ParseResult<Expression> {
        let mut expr = self.range()?;

        while let Some(operator) = self.match_comparison_operator() {
            let right = self.range()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parse range expression
    fn range(&mut self) -> ParseResult<Expression> {
        let mut expr = self.term()?;

        if self.match_token(&TokenType::DotDot) {
            let end = self.term()?;
            expr = Expression::Range {
                start: Box::new(expr),
                end: Box::new(end),
            };
        }

        Ok(expr)
    }

    /// Parse term expression (addition/subtraction)
    fn term(&mut self) -> ParseResult<Expression> {
        let mut expr = self.factor()?;

        while let Some(operator) = self.match_term_operator() {
            let right = self.factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parse factor expression (multiplication/division)
    fn factor(&mut self) -> ParseResult<Expression> {
        let mut expr = self.unary()?;

        while let Some(operator) = self.match_factor_operator() {
            let right = self.unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parse unary expression
    fn unary(&mut self) -> ParseResult<Expression> {
        // Handle &mut expr and &expr — consume the reference syntax transparently.
        // Ovie passes values by reference at the call site; the & / &mut are hints
        // that the runtime handles automatically, so we just return the inner expr.
        if self.check(&TokenType::Ampersand) {
            self.advance(); // consume '&'
            // Optionally consume 'mut' after '&'
            if self.check(&TokenType::Mut) {
                self.advance(); // consume 'mut'
            }
            return self.unary();
        }

        if let Some(operator) = self.match_unary_operator() {
            let operand = self.unary()?;
            return Ok(Expression::Unary {
                operator,
                operand: Box::new(operand),
            });
        }

        self.primary()
    }

    /// Parse primary expression
    fn primary(&mut self) -> ParseResult<Expression> {
        let mut expr = self.primary_base()?;
        
        // Handle range expressions
        if self.match_token(&TokenType::DotDot) {
            let end = self.primary_base()?;
            expr = Expression::Range {
                start: Box::new(expr),
                end: Box::new(end),
            };
        }
        
        // Handle 'as' type cast: expr as Type
        if self.check(&TokenType::As) {
            self.advance(); // consume 'as'
            // Skip the type annotation (we don't enforce types at runtime)
            self.skip_type_annotation()?;
            // Return the expression unchanged (cast is a no-op in interpreter)
        }
        
        // Handle field access, array indexing, enum variant construction, and ? operator
        loop {
            if self.match_token(&TokenType::Question) {
                // ? operator: error propagation
                expr = Expression::Try {
                    expression: Box::new(expr),
                };
            } else if self.match_token(&TokenType::Dot) {
                // Check for tuple field access (.0, .1, .2, etc.) or regular field access
                let field = if self.check(&TokenType::IntegerLiteral) {
                    // Tuple field access: expr.0, expr.1, etc.
                    let tok = self.advance();
                    tok.lexeme.clone()
                } else {
                    // Regular field access: expr.field_name
                    self.consume_identifier("Expected field name or tuple index after '.'")?
                };
                
                // Check if this is an enum variant construction with data
                if self.match_token(&TokenType::LeftParen) {
                    // This is EnumName.VariantName(data) or expr.method(args)
                    if let Expression::Identifier(enum_name) = expr {
                        // Could be enum variant or method call on identifier
                        // If field starts with uppercase, treat as enum variant
                        if field.chars().next().map_or(false, |c| c.is_uppercase()) {
                            let data = self.expression()?;
                            self.consume(&TokenType::RightParen, "Expected ')' after enum variant data")?;
                            expr = Expression::EnumVariantConstruction {
                                enum_name,
                                variant_name: field,
                                data: Some(Box::new(data)),
                            };
                        } else {
                            // Method call on identifier
                            let mut arguments = Vec::new();
                            if !self.check(&TokenType::RightParen) {
                                loop {
                                    arguments.push(self.expression()?);
                                    if !self.match_token(&TokenType::Comma) {
                                        break;
                                    }
                                }
                            }
                            self.consume(&TokenType::RightParen, "Expected ')' after method arguments")?;
                            expr = Expression::MethodCall {
                                object: Box::new(Expression::Identifier(enum_name)),
                                method: field,
                                arguments,
                            };
                        }
                    } else {
                        // Method call on non-identifier expression (e.g. "str".to_string())
                        let mut arguments = Vec::new();
                        if !self.check(&TokenType::RightParen) {
                            loop {
                                arguments.push(self.expression()?);
                                if !self.match_token(&TokenType::Comma) {
                                    break;
                                }
                            }
                        }
                        self.consume(&TokenType::RightParen, "Expected ')' after method arguments")?;
                        expr = Expression::MethodCall {
                            object: Box::new(expr),
                            method: field,
                            arguments,
                        };
                    }
                } else {
                    // Check if this might be an enum variant without data
                    // We need to distinguish between field access and enum variant
                    // For now, we'll treat EnumName.VariantName as enum variant if EnumName is capitalized
                    if let Expression::Identifier(ref name) = expr {
                        if name.chars().next().map_or(false, |c| c.is_uppercase()) {
                            // This looks like an enum variant construction without data
                            expr = Expression::EnumVariantConstruction {
                                enum_name: name.clone(),
                                variant_name: field,
                                data: None,
                            };
                            continue;
                        }
                    }
                    
                    // Regular field access
                    expr = Expression::FieldAccess {
                        object: Box::new(expr),
                        field,
                    };
                }
            } else if self.match_token(&TokenType::LeftBracket) {
                // Array/String indexing: expr[index]
                let index = self.expression()?;
                self.consume(&TokenType::RightBracket, "Expected ']' after index")?;
                expr = Expression::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else {
                break;
            }
        }
        
        Ok(expr)
    }

    /// Parse base primary expression (without range or field access)
    fn primary_base(&mut self) -> ParseResult<Expression> {
        match &self.peek().token_type {
            TokenType::Match => {
                return self.match_expression();
            }
            // Closure: |params| expr or |params| { body }
            // Parse as a null literal (closures are not yet executable in v2.2 interpreter)
            // but we need to consume the tokens so the parser doesn't fail
            TokenType::Pipe => {
                self.advance(); // consume first '|'
                // Consume parameters until closing '|'
                while !self.check(&TokenType::Pipe) && !self.is_at_end() {
                    self.advance();
                }
                if self.check(&TokenType::Pipe) { self.advance(); } // consume closing '|'
                // Consume the body
                if self.check(&TokenType::LeftBrace) {
                    self.block_statement()?;
                } else {
                    self.expression()?;
                }
                return Ok(Expression::Null);
            }
            TokenType::True => {
                self.advance();
                Ok(Expression::Literal(Literal::Boolean(true)))
            }
            TokenType::False => {
                self.advance();
                Ok(Expression::Literal(Literal::Boolean(false)))
            }
            TokenType::IntegerLiteral => {
                let token = self.advance();
                let value = token.lexeme.parse::<f64>()
                    .map_err(|_| self.error("Invalid number literal"))?;
                Ok(Expression::Literal(Literal::Number(value)))
            }
            TokenType::FloatLiteral => {
                let token = self.advance();
                let value = token.lexeme.parse::<f64>()
                    .map_err(|_| self.error("Invalid number literal"))?;
                Ok(Expression::Literal(Literal::Number(value)))
            }
            TokenType::StringLiteral => {
                let token = self.advance();
                // Remove quotes and handle escape sequences - clone to avoid borrow issues
                let lexeme = token.lexeme.clone();
                let value = self.parse_string_literal(&lexeme)?;
                Ok(Expression::Literal(Literal::String(value)))
            }
            TokenType::CharLiteral => {
                // Treat char literals as single-character strings
                let token = self.advance();
                let lexeme = token.lexeme.clone();
                // Remove surrounding single quotes: 'x' -> x
                let inner = if lexeme.len() >= 2 {
                    &lexeme[1..lexeme.len()-1]
                } else {
                    &lexeme
                };
                // Handle escape sequences
                let value = match inner {
                    "\\n" => "\n".to_string(),
                    "\\t" => "\t".to_string(),
                    "\\r" => "\r".to_string(),
                    "\\\\" => "\\".to_string(),
                    "\\'" => "'".to_string(),
                    "\\0" => "\0".to_string(),
                    other => other.to_string(),
                };
                Ok(Expression::Literal(Literal::String(value)))
            }
            TokenType::Identifier => {
                let name = self.advance().lexeme.clone();
                
                // Check for function call
                if self.check(&TokenType::LeftParen) {
                    self.advance(); // consume '('
                    
                    let mut arguments = Vec::new();
                    if !self.check(&TokenType::RightParen) {
                        loop {
                            arguments.push(self.expression()?);
                            if !self.match_token(&TokenType::Comma) {
                                break;
                            }
                        }
                    }
                    
                    self.consume(&TokenType::RightParen, "Expected ')' after arguments")?;
                    
                    Ok(Expression::Call {
                        function: name,
                        arguments,
                    })
                } else if self.check(&TokenType::LeftBrace) && self.looks_like_struct_instantiation() {
                    // Struct instantiation - only if it looks like field initialization
                    // This prevents treating "if a == b {" as struct instantiation
                    self.advance(); // consume '{'
                    
                    let mut fields = Vec::new();
                    while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
                        let field_name = self.consume_identifier("Expected field name")?;
                        self.consume(&TokenType::Colon, "Expected ':' after field name")?;
                        let value = self.expression()?;
                        
                        fields.push(FieldInitializer {
                            name: field_name,
                            value,
                        });
                        
                        if !self.match_token(&TokenType::Comma) {
                            break;
                        }
                    }
                    
                    self.consume(&TokenType::RightBrace, "Expected '}' after struct fields")?;
                    
                    Ok(Expression::StructInstantiation {
                        struct_name: name,
                        fields,
                    })
                } else if self.check(&TokenType::Bang) {
                    // Macro-style call: name!(args) or name![args]
                    self.advance(); // consume '!'
                    if self.check(&TokenType::LeftParen) {
                        self.advance(); // consume '('
                        let mut arguments = Vec::new();
                        if !self.check(&TokenType::RightParen) {
                            loop {
                                arguments.push(self.expression()?);
                                if !self.match_token(&TokenType::Comma) {
                                    break;
                                }
                            }
                        }
                        self.consume(&TokenType::RightParen, "Expected ')' after macro arguments")?;
                        Ok(Expression::Call { function: name, arguments })
                    } else if self.check(&TokenType::LeftBracket) {
                        self.advance(); // consume '['
                        let mut elements = Vec::new();
                        if !self.check(&TokenType::RightBracket) {
                            loop {
                                elements.push(self.expression()?);
                                if !self.match_token(&TokenType::Comma) {
                                    break;
                                }
                            }
                        }
                        self.consume(&TokenType::RightBracket, "Expected ']' after macro arguments")?;
                        Ok(Expression::ArrayLiteral { elements })
                    } else {
                        // Unknown macro form — return identifier
                        Ok(Expression::Identifier(name))
                    }
                } else if self.check(&TokenType::ColonColon) {
                    // Handle Type::method() or Enum::Variant() syntax
                    self.advance(); // consume '::'
                    let member = self.consume_identifier("Expected member name after '::'")?;
                    
                    if self.check(&TokenType::LeftParen) {
                        self.advance(); // consume '('
                        let mut arguments = Vec::new();
                        if !self.check(&TokenType::RightParen) {
                            loop {
                                arguments.push(self.expression()?);
                                if !self.match_token(&TokenType::Comma) {
                                    break;
                                }
                            }
                        }
                        self.consume(&TokenType::RightParen, "Expected ')' after arguments")?;
                        
                        // Determine if this is enum variant construction or a static method call
                        // If member starts with uppercase, treat as enum variant
                        if member.chars().next().map_or(false, |c| c.is_uppercase()) {
                            let data = if arguments.is_empty() {
                                None
                            } else if arguments.len() == 1 {
                                Some(Box::new(arguments.into_iter().next().unwrap()))
                            } else {
                                // Multiple args — wrap in a tuple-like call
                                Some(Box::new(Expression::Call {
                                    function: "__tuple".to_string(),
                                    arguments,
                                }))
                            };
                            Ok(Expression::EnumVariantConstruction {
                                enum_name: name,
                                variant_name: member,
                                data,
                            })
                        } else {
                            // Static method call: Type::method(args)
                            Ok(Expression::MethodCall {
                                object: Box::new(Expression::Identifier(name)),
                                method: member,
                                arguments,
                            })
                        }
                    } else if self.check(&TokenType::LeftBrace) && self.looks_like_struct_instantiation() {
                        // Type::StructName { fields } — treat as struct instantiation
                        self.advance(); // consume '{'
                        let mut fields = Vec::new();
                        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
                            let field_name = self.consume_identifier("Expected field name")?;
                            self.consume(&TokenType::Colon, "Expected ':' after field name")?;
                            let value = self.expression()?;
                            fields.push(FieldInitializer { name: field_name, value });
                            if !self.match_token(&TokenType::Comma) { break; }
                        }
                        self.consume(&TokenType::RightBrace, "Expected '}' after struct fields")?;
                        Ok(Expression::StructInstantiation {
                            struct_name: format!("{}::{}", name, member),
                            fields,
                        })
                    } else {
                        // Enum variant without data: Enum::Variant
                        Ok(Expression::EnumVariantConstruction {
                            enum_name: name,
                            variant_name: member,
                            data: None,
                        })
                    }
                } else {
                    // Simple identifier
                    Ok(Expression::Identifier(name))
                }
            }
            TokenType::LeftParen => {
                self.advance(); // consume '('
                // Handle unit value: ()
                if self.check(&TokenType::RightParen) {
                    self.advance(); // consume ')'
                    return Ok(Expression::Literal(Literal::String("()".to_string())));
                }
                let expr = self.expression()?;
                self.consume(&TokenType::RightParen, "Expected ')' after expression")?;
                Ok(expr)
            }
            TokenType::LeftBracket => {
                // Array literal: [element1, element2, ...]
                self.advance(); // consume '['
                
                let mut elements = Vec::new();
                if !self.check(&TokenType::RightBracket) {
                    loop {
                        elements.push(self.expression()?);
                        if !self.match_token(&TokenType::Comma) {
                            break;
                        }
                    }
                }
                
                self.consume(&TokenType::RightBracket, "Expected ']' after array elements")?;
                
                Ok(Expression::ArrayLiteral { elements })
            }
            _ => Err(self.error("Expected expression")),
        }
    }

    /// Parse a match expression
    fn match_expression(&mut self) -> ParseResult<Expression> {
        self.consume(&TokenType::Match, "Expected 'match'")?;
        let value = self.expression()?;
        self.consume(&TokenType::LeftBrace, "Expected '{' after match value")?;
        
        let mut arms = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let pattern = self.parse_match_pattern()?;
            
            // Handle OR patterns: Pattern1 | Pattern2 => body
            // Skip additional patterns after | — use the first pattern
            while self.check(&TokenType::Pipe) {
                self.advance(); // consume '|'
                // Parse and discard the alternative pattern
                let _ = self.parse_match_pattern();
            }
            
            // Optional guard: if condition
            let guard = if self.check(&TokenType::If) {
                self.advance();
                Some(self.expression()?)
            } else {
                None
            };
            
            self.consume(&TokenType::FatArrow, "Expected '=>' after match pattern")?;
            
            // Body can be a block, a return statement, continue, break, or a single expression
            let body = if self.check(&TokenType::LeftBrace) {
                self.block_statement()?
            } else if self.check(&TokenType::Return) {
                let stmt = self.return_statement()?;
                self.consume_optional_semicolon();
                vec![stmt]
            } else if self.check(&TokenType::Continue) {
                self.advance();
                self.consume_optional_semicolon();
                vec![Statement::Continue]
            } else if self.check(&TokenType::Break) {
                self.advance();
                self.consume_optional_semicolon();
                vec![Statement::Break]
            } else {
                let expr = self.expression()?;
                self.consume_optional_semicolon();
                vec![Statement::Expression { expression: expr }]
            };
            
            arms.push(MatchArm { pattern, body, guard });
            
            // Optional comma between arms
            self.match_token(&TokenType::Comma);
        }
        
        self.consume(&TokenType::RightBrace, "Expected '}' after match arms")?;
        
        Ok(Expression::Match {
            value: Box::new(value),
            arms,
        })
    }

    /// Parse a match pattern
    fn parse_match_pattern(&mut self) -> ParseResult<MatchPattern> {
        // Wildcard: _ (identifier with name "_")
        if self.check(&TokenType::Identifier) && self.peek().lexeme == "_" {
            self.advance();
            return Ok(MatchPattern::Wildcard);
        }
        
        // Literal patterns
        if self.check(&TokenType::IntegerLiteral) || self.check(&TokenType::FloatLiteral) {
            let tok = self.advance();
            let n = tok.lexeme.parse::<f64>().unwrap_or(0.0);
            return Ok(MatchPattern::Literal(Literal::Number(n)));
        }
        if self.check(&TokenType::StringLiteral) {
            let tok = self.advance();
            let raw = tok.lexeme.clone();
            let s = self.parse_string_literal(&raw)?;
            return Ok(MatchPattern::Literal(Literal::String(s)));
        }
        if self.check(&TokenType::True) {
            self.advance();
            return Ok(MatchPattern::Literal(Literal::Boolean(true)));
        }
        if self.check(&TokenType::False) {
            self.advance();
            return Ok(MatchPattern::Literal(Literal::Boolean(false)));
        }
        
        // Identifier or enum variant
        if self.check(&TokenType::Identifier) {
            let name = self.advance().lexeme.clone();
            
            // Check for enum variant: EnumName.Variant or EnumName::Variant
            if self.check(&TokenType::Dot) || self.check(&TokenType::ColonColon) {
                self.advance();
                let variant = self.consume_identifier("Expected variant name")?;
                let binding = if self.check(&TokenType::LeftParen) {
                    self.advance();
                    let b = if self.check(&TokenType::Identifier) {
                        Some(self.advance().lexeme.clone())
                    } else {
                        None
                    };
                    self.consume(&TokenType::RightParen, "Expected ')'")?;
                    b
                } else {
                    None
                };
                return Ok(MatchPattern::EnumVariant {
                    enum_name: name,
                    variant_name: variant,
                    binding,
                });
            }
            
            // Check for Variant(binding) pattern: Ok(x), Err(e), Some(v), None, etc.
            if self.check(&TokenType::LeftParen) {
                self.advance(); // consume '('
                // Could be Ok(binding), Err(binding), Some(binding), or a tuple pattern
                let binding = if self.check(&TokenType::Identifier) {
                    let b = self.advance().lexeme.clone();
                    // Skip any nested patterns (e.g. Ok(ModuleError::NotFound(_)))
                    // by consuming until the matching ')'
                    let mut depth = 0;
                    while !self.is_at_end() {
                        if self.check(&TokenType::LeftParen) {
                            depth += 1;
                            self.advance();
                        } else if self.check(&TokenType::RightParen) {
                            if depth == 0 {
                                break;
                            }
                            depth -= 1;
                            self.advance();
                        } else if self.check(&TokenType::Comma) && depth == 0 {
                            break;
                        } else {
                            self.advance();
                        }
                    }
                    Some(b)
                } else if self.check(&TokenType::Identifier) && self.peek().lexeme == "_" {
                    self.advance();
                    None
                } else {
                    // Skip until matching ')'
                    let mut depth = 0;
                    while !self.is_at_end() {
                        if self.check(&TokenType::LeftParen) {
                            depth += 1;
                            self.advance();
                        } else if self.check(&TokenType::RightParen) {
                            if depth == 0 { break; }
                            depth -= 1;
                            self.advance();
                        } else {
                            self.advance();
                        }
                    }
                    None
                };
                self.consume(&TokenType::RightParen, "Expected ')' after pattern binding")?;
                // Treat as enum variant with the name as both enum and variant
                return Ok(MatchPattern::EnumVariant {
                    enum_name: name.clone(),
                    variant_name: name,
                    binding,
                });
            }
            
            return Ok(MatchPattern::Identifier(name));
        }
        
        Err(self.error("Expected match pattern"))
    }

    /// Parse string literal, handling escape sequences
    fn parse_string_literal(&self, lexeme: &str) -> ParseResult<String> {
        // Remove surrounding quotes
        let content = &lexeme[1..lexeme.len()-1];
        
        // Handle escape sequences
        let mut result = String::new();
        let mut chars = content.chars();
        
        while let Some(ch) = chars.next() {
            if ch == '\\' {
                match chars.next() {
                    Some('n') => result.push('\n'),
                    Some('r') => result.push('\r'),
                    Some('t') => result.push('\t'),
                    Some('\\') => result.push('\\'),
                    Some('"') => result.push('"'),
                    Some('0') => result.push('\0'),
                    Some(other) => {
                        result.push('\\');
                        result.push(other);
                    }
                    None => result.push('\\'),
                }
            } else {
                result.push(ch);
            }
        }
        
        Ok(result)
    }

    /// Helper methods for operator matching
    fn match_equality_operator(&mut self) -> Option<BinaryOperator> {
        match &self.peek().token_type {
            TokenType::EqualEqual => {
                self.advance();
                Some(BinaryOperator::Equal)
            }
            TokenType::NotEqual => {
                self.advance();
                Some(BinaryOperator::NotEqual)
            }
            _ => None,
        }
    }

    fn match_comparison_operator(&mut self) -> Option<BinaryOperator> {
        match &self.peek().token_type {
            TokenType::Greater => {
                self.advance();
                Some(BinaryOperator::Greater)
            }
            TokenType::GreaterEqual => {
                self.advance();
                Some(BinaryOperator::GreaterEqual)
            }
            TokenType::Less => {
                self.advance();
                Some(BinaryOperator::Less)
            }
            TokenType::LessEqual => {
                self.advance();
                Some(BinaryOperator::LessEqual)
            }
            _ => None,
        }
    }

    fn match_term_operator(&mut self) -> Option<BinaryOperator> {
        match &self.peek().token_type {
            TokenType::Plus => {
                self.advance();
                Some(BinaryOperator::Add)
            }
            TokenType::Minus => {
                self.advance();
                Some(BinaryOperator::Subtract)
            }
            _ => None,
        }
    }

    fn match_factor_operator(&mut self) -> Option<BinaryOperator> {
        match &self.peek().token_type {
            TokenType::Star => {
                self.advance();
                Some(BinaryOperator::Multiply)
            }
            TokenType::Slash => {
                self.advance();
                Some(BinaryOperator::Divide)
            }
            TokenType::Percent => {
                self.advance();
                Some(BinaryOperator::Modulo)
            }
            _ => None,
        }
    }

    fn match_unary_operator(&mut self) -> Option<UnaryOperator> {
        match &self.peek().token_type {
            TokenType::Bang => {
                self.advance();
                Some(UnaryOperator::Not)
            }
            TokenType::Minus => {
                self.advance();
                Some(UnaryOperator::Negate)
            }
            _ => None,
        }
    }

    /// Utility methods
    fn looks_like_struct_instantiation(&self) -> bool {
        // Look ahead to see if this looks like struct instantiation
        // Pattern: { identifier : ...
        // We need to check if after the '{' there's an identifier followed by ':'
        if self.current + 1 >= self.tokens.len() {
            return false;
        }
        
        // Check if next token after '{' is an identifier
        if self.tokens[self.current + 1].token_type != TokenType::Identifier {
            return false;
        }
        
        // Check if token after identifier is ':'
        if self.current + 2 >= self.tokens.len() {
            return false;
        }
        
        self.tokens[self.current + 2].token_type == TokenType::Colon
    }
    
    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().token_type == token_type
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> ParseResult<&Token> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(self.error(message))
        }
    }

    fn consume_identifier(&mut self, message: &str) -> ParseResult<String> {
        if self.check(&TokenType::Identifier) {
            Ok(self.advance().lexeme.clone())
        } else {
            Err(self.error(message))
        }
    }

    fn error(&self, message: &str) -> OvieError {
        let token = self.peek();
        OvieError::parse_error(
            token.location.line,
            token.location.column,
            format!("{} (found '{}')", message, token.lexeme),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse_source(source: &str) -> ParseResult<AstNode> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    #[test]
    fn test_simple_print() {
        let ast = parse_source(r#"seeAm "Hello, World!";"#).unwrap();
        
        match &ast {
            AstNode::Program(statements) => {
                assert_eq!(statements.len(), 1);
                
                match &statements[0] {
                    Statement::Print { expression } => {
                        match expression {
                            Expression::Literal(Literal::String(s)) => {
                                assert_eq!(s, "Hello, World!");
                            }
                            _ => panic!("Expected string literal"),
                        }
                    }
                    _ => panic!("Expected print statement"),
                }
            }
        }
    }

    #[test]
    fn test_assignment() {
        let ast = parse_source("name = \"Ovie\";").unwrap();
        match &ast {
            AstNode::Program(statements) => {
                assert_eq!(statements.len(), 1);
                
                match &statements[0] {
                    Statement::Assignment { mutable, identifier, value } => {
                        assert!(!mutable);
                        assert_eq!(identifier, "name");
                        match value {
                            Expression::Literal(Literal::String(s)) => {
                                assert_eq!(s, "Ovie");
                            }
                            _ => panic!("Expected string literal"),
                        }
                    }
                    _ => panic!("Expected assignment statement"),
                }
            }
        }
    }

    #[test]
    fn test_mutable_assignment() {
        let ast = parse_source("mut counter = 42;").unwrap();
        
        match &ast {
            AstNode::Program(statements) => {
                assert_eq!(statements.len(), 1);
                
                match &statements[0] {
                    Statement::Assignment { mutable, identifier, value } => {
                        assert!(mutable);
                        assert_eq!(identifier, "counter");
                        match value {
                            Expression::Literal(Literal::Number(n)) => {
                                assert_eq!(*n, 42.0);
                            }
                            _ => panic!("Expected number literal"),
                        }
                    }
                    _ => panic!("Expected assignment statement"),
                }
            }
        }
    }

    #[test]
    fn test_function_definition() {
        let ast = parse_source("fn greet(name) { seeAm \"Hello, \" + name + \"!\"; }").unwrap();
        
        match &ast {
            AstNode::Program(statements) => {
                assert_eq!(statements.len(), 1);
                
                match &statements[0] {
                    Statement::Function { name, parameters, body } => {
                        assert_eq!(name, "greet");
                        assert_eq!(parameters.len(), 1);
                        assert_eq!(parameters[0], "name");
                        assert_eq!(body.len(), 1);
                    }
                    _ => panic!("Expected function statement"),
                }
            }
        }
    }

    #[test]
    fn test_binary_expression() {
        let ast = parse_source("result = 10 + 5 * 2;").unwrap();
        
        match &ast {
            AstNode::Program(statements) => {
                assert_eq!(statements.len(), 1);
                
                match &statements[0] {
                    Statement::Assignment { identifier, value, .. } => {
                        assert_eq!(identifier, "result");
                        // Should parse as 10 + (5 * 2) due to precedence
                        match value {
                            Expression::Binary { left, operator, right } => {
                                assert_eq!(*operator, BinaryOperator::Add);
                                match left.as_ref() {
                                    Expression::Literal(Literal::Number(n)) => assert_eq!(*n, 10.0),
                                    _ => panic!("Expected number literal"),
                                }
                                match right.as_ref() {
                                    Expression::Binary { operator, .. } => {
                                        assert_eq!(*operator, BinaryOperator::Multiply);
                                    }
                                    _ => panic!("Expected binary expression"),
                                }
                            }
                            _ => panic!("Expected binary expression"),
                        }
                    }
                    _ => panic!("Expected assignment statement"),
                }
            }
        }
    }

    #[test]
    fn test_range_expression() {
        let ast = parse_source("for i in 1..6 { seeAm i; }").unwrap();
        
        match &ast {
            AstNode::Program(statements) => {
                assert_eq!(statements.len(), 1);
                
                match &statements[0] {
                    Statement::For { identifier, iterable, body } => {
                        assert_eq!(identifier, "i");
                        match iterable {
                            Expression::Range { start, end } => {
                                match start.as_ref() {
                                    Expression::Literal(Literal::Number(n)) => assert_eq!(*n, 1.0),
                                    _ => panic!("Expected number literal for range start"),
                                }
                                match end.as_ref() {
                                    Expression::Literal(Literal::Number(n)) => assert_eq!(*n, 6.0),
                                    _ => panic!("Expected number literal for range end"),
                                }
                            }
                            _ => panic!("Expected range expression"),
                        }
                        assert_eq!(body.len(), 1);
                    }
                    _ => panic!("Expected for statement"),
                }
            }
        }
    }
}