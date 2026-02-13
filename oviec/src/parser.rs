//! Parser for the Ovie programming language

use crate::ast::{
    AstNode, Statement, Expression, Literal, BinaryOperator, UnaryOperator,
    StructField, EnumVariant, FieldInitializer
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
            TokenType::Mut => self.assignment_statement(true),
            TokenType::Identifier => {
                // Look ahead to see if this is an assignment
                if self.tokens.get(self.current + 1)
                    .map(|t| &t.token_type) == Some(&TokenType::Equal) {
                    self.assignment_statement(false)
                } else {
                    self.expression_statement()
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
                parameters.push(self.consume_identifier("Expected parameter name")?);
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }
        
        self.consume(&TokenType::RightParen, "Expected ')' after parameters")?;
        
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
        let condition = self.expression()?;
        let then_block = self.block_statement()?;
        
        let else_block = if self.match_token(&TokenType::Else) {
            Some(self.block_statement()?)
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
        let identifier = self.consume_identifier("Expected loop variable name")?;
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
        
        let value = if self.check(&TokenType::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };
        
        self.consume(&TokenType::Semicolon, "Expected ';' after return statement")?;
        
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
            let type_annotation = self.consume_identifier("Expected field type")?;
            
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

    /// Parse an enum definition
    fn enum_statement(&mut self) -> ParseResult<Statement> {
        self.consume(&TokenType::Enum, "Expected 'enum'")?;
        let name = self.consume_identifier("Expected enum name")?;
        
        self.consume(&TokenType::LeftBrace, "Expected '{' after enum name")?;
        
        let mut variants = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let variant_name = self.consume_identifier("Expected variant name")?;
            
            let data_type = if self.match_token(&TokenType::LeftParen) {
                let type_name = self.consume_identifier("Expected variant data type")?;
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
        self.consume(&TokenType::Equal, "Expected '=' in assignment")?;
        let value = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expected ';' after assignment")?;
        
        Ok(Statement::Assignment {
            mutable,
            identifier,
            value,
        })
    }

    /// Parse an expression statement
    fn expression_statement(&mut self) -> ParseResult<Statement> {
        let expression = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expected ';' after expression")?;
        
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
        
        // Handle field access, array indexing, and enum variant construction
        loop {
            if self.match_token(&TokenType::Dot) {
                let field = self.consume_identifier("Expected field name after '.'")?;
                
                // Check if this is an enum variant construction with data
                if self.match_token(&TokenType::LeftParen) {
                    // This is EnumName.VariantName(data)
                    // The expr should be an identifier (enum name)
                    if let Expression::Identifier(enum_name) = expr {
                        let data = self.expression()?;
                        self.consume(&TokenType::RightParen, "Expected ')' after enum variant data")?;
                        expr = Expression::EnumVariantConstruction {
                            enum_name,
                            variant_name: field,
                            data: Some(Box::new(data)),
                        };
                    } else {
                        return Err(self.error("Enum variant construction requires enum name before '.'"));
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
                } else {
                    // Simple identifier
                    Ok(Expression::Identifier(name))
                }
            }
            TokenType::LeftParen => {
                self.advance(); // consume '('
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