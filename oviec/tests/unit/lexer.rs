//! Unit tests for the Ovie lexer component

use crate::{Lexer, TokenType, OvieResult};

/// Test basic token recognition
pub fn test_basic_tokens() -> OvieResult<()> {
    let mut lexer = Lexer::new("let x = 42;");
    let tokens = lexer.tokenize()?;
    
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].token_type, TokenType::Let);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[2].token_type, TokenType::Assign);
    assert_eq!(tokens[3].token_type, TokenType::Number);
    assert_eq!(tokens[4].token_type, TokenType::Semicolon);
    
    Ok(())
}

/// Test string literal tokenization
pub fn test_string_literals() -> OvieResult<()> {
    let mut lexer = Lexer::new(r#""hello world""#);
    let tokens = lexer.tokenize()?;
    
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::String);
    assert_eq!(tokens[0].lexeme, "hello world");
    
    Ok(())
}

/// Test number tokenization
pub fn test_numbers() -> OvieResult<()> {
    let mut lexer = Lexer::new("42 3.14 0xFF");
    let tokens = lexer.tokenize()?;
    
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[1].token_type, TokenType::Number);
    assert_eq!(tokens[2].token_type, TokenType::Number);
    
    Ok(())
}

/// Test identifier tokenization
pub fn test_identifiers() -> OvieResult<()> {
    let mut lexer = Lexer::new("variable _private __internal");
    let tokens = lexer.tokenize()?;
    
    assert_eq!(tokens.len(), 3);
    for token in &tokens {
        assert_eq!(token.token_type, TokenType::Identifier);
    }
    
    Ok(())
}

/// Test keyword recognition
pub fn test_keywords() -> OvieResult<()> {
    let mut lexer = Lexer::new("let fn if else while for");
    let tokens = lexer.tokenize()?;
    
    assert_eq!(tokens[0].token_type, TokenType::Let);
    assert_eq!(tokens[1].token_type, TokenType::Fn);
    assert_eq!(tokens[2].token_type, TokenType::If);
    assert_eq!(tokens[3].token_type, TokenType::Else);
    assert_eq!(tokens[4].token_type, TokenType::While);
    assert_eq!(tokens[5].token_type, TokenType::For);
    
    Ok(())
}

/// Test operator tokenization
pub fn test_operators() -> OvieResult<()> {
    let mut lexer = Lexer::new("+ - * / == != < > <= >=");
    let tokens = lexer.tokenize()?;
    
    assert_eq!(tokens[0].token_type, TokenType::Plus);
    assert_eq!(tokens[1].token_type, TokenType::Minus);
    assert_eq!(tokens[2].token_type, TokenType::Star);
    assert_eq!(tokens[3].token_type, TokenType::Slash);
    assert_eq!(tokens[4].token_type, TokenType::EqualEqual);
    assert_eq!(tokens[5].token_type, TokenType::BangEqual);
    
    Ok(())
}