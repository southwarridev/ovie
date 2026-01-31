// MIR (Mid-level IR) Tests
// Feature: ovie-programming-language-stage-2, MIR Pipeline

use crate::mir::{MirBuilder, MirProgram};
use crate::hir::{HirBuilder};
use crate::ast::*;

#[cfg(test)]
mod mir_tests {
    use super::*;

    fn create_test_hir() -> crate::hir::HirProgram {
        let statements = vec![
            Statement::Print {
                expression: Expression::Literal(Literal::String("Hello, MIR!".to_string())),
            }
        ];
        let ast = AstNode::new(statements);
        
        let mut hir_builder = HirBuilder::new();
        hir_builder.transform_ast(&ast).unwrap()
    }

    #[test]
    fn test_mir_simple_program() {
        let hir = create_test_hir();
        
        let mut mir_builder = MirBuilder::new();
        let result = mir_builder.transform_hir(&hir);
        
        assert!(result.is_ok(), "MIR transformation should succeed");
        let mir = result.unwrap();
        assert!(!mir.functions.is_empty(), "MIR should contain functions");
    }

    #[test]
    fn test_mir_function_transformation() {
        let statements = vec![
            Statement::Function {
                name: "test_func".to_string(),
                parameters: vec!["a".to_string()],
                body: vec![
                    Statement::Return {
                        value: Some(Expression::Identifier("a".to_string())),
                    }
                ],
            }
        ];
        let ast = AstNode::new(statements);
        
        let mut hir_builder = HirBuilder::new();
        let hir = hir_builder.transform_ast(&ast).unwrap();
        
        let mut mir_builder = MirBuilder::new();
        let result = mir_builder.transform_hir(&hir);
        
        assert!(result.is_ok(), "MIR transformation with functions should succeed");
        let mir = result.unwrap();
        
        // Check that function was created
        assert!(!mir.functions.is_empty(), "MIR should contain the function");
        
        // Check that function has basic blocks
        let function = mir.functions.values().next().unwrap();
        assert!(!function.basic_blocks.is_empty(), "Function should have basic blocks");
    }

    #[test]
    fn test_mir_control_flow() {
        let statements = vec![
            Statement::Assignment {
                mutable: false,
                identifier: "x".to_string(),
                value: Expression::Literal(Literal::Number(5.0)),
            },
            Statement::If {
                condition: Expression::Binary {
                    left: Box::new(Expression::Identifier("x".to_string())),
                    operator: BinaryOperator::Greater,
                    right: Box::new(Expression::Literal(Literal::Number(0.0))),
                },
                then_block: vec![
                    Statement::Print {
                        expression: Expression::Literal(Literal::String("Positive".to_string())),
                    }
                ],
                else_block: Some(vec![
                    Statement::Print {
                        expression: Expression::Literal(Literal::String("Non-positive".to_string())),
                    }
                ]),
            }
        ];
        let ast = AstNode::new(statements);
        
        let mut hir_builder = HirBuilder::new();
        let hir = hir_builder.transform_ast(&ast).unwrap();
        
        let mut mir_builder = MirBuilder::new();
        let result = mir_builder.transform_hir(&hir);
        
        assert!(result.is_ok(), "MIR transformation with control flow should succeed");
    }

    #[test]
    fn test_mir_basic_blocks() {
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
        let hir = hir_builder.transform_ast(&ast).unwrap();
        
        let mut mir_builder = MirBuilder::new();
        let mir = mir_builder.transform_hir(&hir).unwrap();
        
        // Check that we have at least one function with basic blocks
        assert!(!mir.functions.is_empty(), "MIR should have functions");
        
        let function = mir.functions.values().next().unwrap();
        assert!(!function.basic_blocks.is_empty(), "Function should have basic blocks");
        
        // Check that entry block exists
        assert!(function.basic_blocks.contains_key(&function.entry_block), 
                "Entry block should exist");
    }

    #[test]
    fn test_mir_locals() {
        let statements = vec![
            Statement::Assignment {
                mutable: true,
                identifier: "x".to_string(),
                value: Expression::Literal(Literal::Number(42.0)),
            },
            Statement::Assignment {
                mutable: false,
                identifier: "y".to_string(),
                value: Expression::Literal(Literal::String("test".to_string())),
            }
        ];
        let ast = AstNode::new(statements);
        
        let mut hir_builder = HirBuilder::new();
        let hir = hir_builder.transform_ast(&ast).unwrap();
        
        let mut mir_builder = MirBuilder::new();
        let mir = mir_builder.transform_hir(&hir).unwrap();
        
        // Check that locals were created
        let function = mir.functions.values().next().unwrap();
        assert!(!function.locals.is_empty(), "Function should have local variables");
        
        // Check mutability is preserved
        let mutable_locals: Vec<_> = function.locals.iter()
            .filter(|local| local.is_mutable)
            .collect();
        assert!(!mutable_locals.is_empty(), "Should have mutable locals");
    }

    #[test]
    fn test_mir_serialization() {
        let hir = create_test_hir();
        
        let mut mir_builder = MirBuilder::new();
        let mir = mir_builder.transform_hir(&hir).unwrap();
        
        let json_result = mir.to_json();
        assert!(json_result.is_ok(), "MIR should serialize to JSON");
        
        let json = json_result.unwrap();
        assert!(!json.is_empty(), "MIR JSON should not be empty");
        
        // Test deserialization
        let deserialized = MirProgram::from_json(&json);
        assert!(deserialized.is_ok(), "MIR should deserialize from JSON");
    }

    #[test]
    fn test_mir_validation() {
        let hir = create_test_hir();
        
        let mut mir_builder = MirBuilder::new();
        let mir = mir_builder.transform_hir(&hir).unwrap();
        
        let validation_result = mir.validate();
        assert!(validation_result.is_ok(), "Valid MIR should pass validation");
    }

    #[test]
    fn test_mir_entry_point() {
        let statements = vec![
            Statement::Function {
                name: "main".to_string(),
                parameters: vec![],
                body: vec![
                    Statement::Print {
                        expression: Expression::Literal(Literal::String("Main function".to_string())),
                    }
                ],
            }
        ];
        let ast = AstNode::new(statements);
        
        let mut hir_builder = HirBuilder::new();
        let hir = hir_builder.transform_ast(&ast).unwrap();
        
        let mut mir_builder = MirBuilder::new();
        let mir = mir_builder.transform_hir(&hir).unwrap();
        
        // Check that entry point is set
        assert!(mir.entry_point.is_some(), "MIR should have entry point for main function");
        
        let entry_id = mir.entry_point.unwrap();
        assert!(mir.functions.contains_key(&entry_id), "Entry point function should exist");
        
        let main_function = &mir.functions[&entry_id];
        assert!(main_function.is_main, "Entry point function should be marked as main");
    }

    #[test]
    fn test_mir_type_definitions() {
        let statements = vec![
            Statement::Struct {
                name: "Point".to_string(),
                fields: vec![
                    StructField {
                        name: "x".to_string(),
                        type_annotation: "Number".to_string(),
                    },
                    StructField {
                        name: "y".to_string(),
                        type_annotation: "Number".to_string(),
                    },
                ],
            }
        ];
        let ast = AstNode::new(statements);
        
        let mut hir_builder = HirBuilder::new();
        let hir = hir_builder.transform_ast(&ast).unwrap();
        
        let mut mir_builder = MirBuilder::new();
        let mir = mir_builder.transform_hir(&hir).unwrap();
        
        // Check that type definition was created
        assert!(mir.type_definitions.contains_key("Point"), 
                "MIR should contain struct type definition");
    }
}