//! Unit tests for the Ovie parser component

use crate::{Parser, Lexer, AstNode, Statement, Expression, OvieResult};

/// Test basic expression parsing
pub fn test_basic_expressions() -> OvieResult<()> {
    let mut lexer = Lexer::new("42 + 3");
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    
    // Verify AST structure
    match ast {
        AstNode::Program(statements) => {
            assert_eq!(statements.len(), 1);
            // Additional AST structure verification would go here
        }
        _ => panic!("Expected program node"),
    }
    
    Ok(())
}

/// Test variable declaration parsing
pub fn test_variable_declarations() -> OvieResult<()> {
    let mut lexer = Lexer::new("let x = 42;");
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    
    match ast {
        AstNode::Program(statements) => {
            assert_eq!(statements.len(), 1);
            match &statements[0] {
                Statement::VariableDeclaration { .. } => {
                    // Verify variable declaration structure
                }
                _ => panic!("Expected variable declaration"),
            }
        }
        _ => panic!("Expected program node"),
    }
    
    Ok(())
}

/// Test function declaration parsing
pub fn test_function_declarations() -> OvieResult<()> {
    let source = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    
    match ast {
        AstNode::Program(statements) => {
            assert_eq!(statements.len(), 1);
            match &statements[0] {
                Statement::FunctionDeclaration { .. } => {
                    // Verify function declaration structure
                }
                _ => panic!("Expected function declaration"),
            }
        }
        _ => panic!("Expected program node"),
    }
    
    Ok(())
}

/// Test control flow parsing
pub fn test_control_flow() -> OvieResult<()> {
    let source = r#"
        if x > 0 {
            print("positive");
        } else {
            print("non-positive");
        }
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    
    // Verify if statement structure
    match ast {
        AstNode::Program(statements) => {
            assert_eq!(statements.len(), 1);
            // Additional verification would go here
        }
        _ => panic!("Expected program node"),
    }
    
    Ok(())
}