// HIR (High-level IR) Tests
// Feature: ovie-programming-language-stage-2, HIR Pipeline

use crate::hir::{HirBuilder, HirProgram};
use crate::ast::*;

#[cfg(test)]
mod hir_tests {
    use super::*;

    #[test]
    fn test_hir_simple_program() {
        let statements = vec![
            Statement::Print {
                expression: Expression::Literal(Literal::String("Hello, HIR!".to_string())),
            }
        ];
        let ast = AstNode::new(statements);
        
        let mut hir_builder = HirBuilder::new();
        let result = hir_builder.transform_ast(&ast);
        
        assert!(result.is_ok(), "HIR transformation should succeed");
        let hir = result.unwrap();
        assert!(!hir.items.is_empty(), "HIR should contain items");
    }

    #[test]
    fn test_hir_variable_assignment() {
        let statements = vec![
            Statement::Assignment {
                mutable: false,
                identifier: "x".to_string(),
                value: Expression::Literal(Literal::Number(42.0)),
            },
            Statement::Print {
                expression: Expression::Identifier("x".to_string()),
            }
        ];
        let ast = AstNode::new(statements);
        
        let mut hir_builder = HirBuilder::new();
        let result = hir_builder.transform_ast(&ast);
        
        assert!(result.is_ok(), "HIR transformation with variables should succeed");
    }

    #[test]
    fn test_hir_function_definition() {
        let statements = vec![
            Statement::Function {
                name: "test_func".to_string(),
                parameters: vec!["a".to_string(), "b".to_string()],
                body: vec![
                    Statement::Return {
                        value: Some(Expression::Binary {
                            left: Box::new(Expression::Identifier("a".to_string())),
                            operator: BinaryOperator::Add,
                            right: Box::new(Expression::Identifier("b".to_string())),
                        }),
                    }
                ],
            }
        ];
        let ast = AstNode::new(statements);
        
        let mut hir_builder = HirBuilder::new();
        let result = hir_builder.transform_ast(&ast);
        
        assert!(result.is_ok(), "HIR transformation with functions should succeed");
    }

    #[test]
    fn test_hir_struct_definition() {
        let statements = vec![
            Statement::Struct {
                name: "Person".to_string(),
                fields: vec![
                    StructField {
                        name: "name".to_string(),
                        type_annotation: "String".to_string(),
                    },
                    StructField {
                        name: "age".to_string(),
                        type_annotation: "Number".to_string(),
                    },
                ],
            }
        ];
        let ast = AstNode::new(statements);
        
        let mut hir_builder = HirBuilder::new();
        let result = hir_builder.transform_ast(&ast);
        
        assert!(result.is_ok(), "HIR transformation with structs should succeed");
    }

    #[test]
    fn test_hir_serialization() {
        let statements = vec![
            Statement::Print {
                expression: Expression::Literal(Literal::String("Test".to_string())),
            }
        ];
        let ast = AstNode::new(statements);
        
        let mut hir_builder = HirBuilder::new();
        let hir = hir_builder.transform_ast(&ast).unwrap();
        
        let json_result = hir.to_json();
        assert!(json_result.is_ok(), "HIR should serialize to JSON");
        
        let json = json_result.unwrap();
        assert!(!json.is_empty(), "HIR JSON should not be empty");
        
        // Test deserialization
        let deserialized = HirProgram::from_json(&json);
        assert!(deserialized.is_ok(), "HIR should deserialize from JSON");
    }

    #[test]
    fn test_hir_validation() {
        let statements = vec![
            Statement::Print {
                expression: Expression::Literal(Literal::String("Valid HIR".to_string())),
            }
        ];
        let ast = AstNode::new(statements);
        
        let mut hir_builder = HirBuilder::new();
        let hir = hir_builder.transform_ast(&ast).unwrap();
        
        let validation_result = hir.validate();
        assert!(validation_result.is_ok(), "Valid HIR should pass validation");
    }

    #[test]
    fn test_hir_type_checking() {
        let statements = vec![
            Statement::Assignment {
                mutable: false,
                identifier: "x".to_string(),
                value: Expression::Literal(Literal::Number(42.0)),
            },
            Statement::Assignment {
                mutable: false,
                identifier: "y".to_string(),
                value: Expression::Binary {
                    left: Box::new(Expression::Identifier("x".to_string())),
                    operator: BinaryOperator::Add,
                    right: Box::new(Expression::Literal(Literal::Number(8.0))),
                },
            }
        ];
        let ast = AstNode::new(statements);
        
        let mut hir_builder = HirBuilder::new();
        let result = hir_builder.transform_ast(&ast);
        
        assert!(result.is_ok(), "HIR type checking should succeed for valid types");
    }
}