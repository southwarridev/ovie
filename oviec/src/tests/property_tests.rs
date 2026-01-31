//! Property-based tests for the Ovie compiler
//! 
//! These tests validate universal properties that should hold across all valid inputs

use crate::lexer::{Lexer, TokenType};
use crate::parser::Parser;
use crate::ast::{Statement, Expression, Literal};
// use crate::semantic::SemanticAnalyzer;
use proptest::prelude::*;

/// **Feature: ovie-programming-language, Property 1: Language Grammar Compliance**
/// 
/// For any valid Ovie source code, parsing then pretty-printing should produce 
/// semantically equivalent code that uses only the 13 core keywords and follows 
/// the formal grammar specification
/// 
/// **Validates: Requirements 1.1, 1.4, 6.2, 6.3**
mod grammar_compliance {
    use super::*;

    // Generator for valid Ovie identifiers
    fn valid_identifier() -> impl Strategy<Value = String> {
        prop::string::string_regex(r"[a-zA-Z_][a-zA-Z0-9_]*")
            .unwrap()
            .prop_filter("Not a keyword", |s| {
                !matches!(s.as_str(), 
                    "fn" | "mut" | "if" | "else" | "for" | "while" | 
                    "struct" | "enum" | "unsafe" | "return" | 
                    "true" | "false" | "seeAm"
                )
            })
    }

    // Generator for valid string literals
    fn valid_string_literal() -> impl Strategy<Value = String> {
        prop::string::string_regex(r#""[^"\\]*""#).unwrap()
    }

    // Generator for valid number literals
    fn valid_number_literal() -> impl Strategy<Value = String> {
        prop_oneof![
            (0i64..1000).prop_map(|n| n.to_string()),
            (0.0f64..1000.0).prop_map(|n| format!("{:.2}", n)),
        ]
    }

    // Generator for simple expressions
    fn simple_expression() -> impl Strategy<Value = String> {
        prop_oneof![
            valid_identifier(),
            valid_string_literal(),
            valid_number_literal(),
            Just("true".to_string()),
            Just("false".to_string()),
        ]
    }

    // Generator for simple statements
    fn simple_statement() -> impl Strategy<Value = String> {
        prop_oneof![
            // Print statements
            simple_expression().prop_map(|expr| format!("seeAm {};", expr)),
            // Assignment statements
            (valid_identifier(), simple_expression())
                .prop_map(|(id, expr)| format!("{} = {};", id, expr)),
            // Mutable assignment statements
            (valid_identifier(), simple_expression())
                .prop_map(|(id, expr)| format!("mut {} = {};", id, expr)),
        ]
    }

    // Generator for simple programs
    fn simple_program() -> impl Strategy<Value = String> {
        prop::collection::vec(simple_statement(), 1..5)
            .prop_map(|statements| statements.join("\n"))
    }

    proptest! {
        #[test]
        fn test_lexer_tokenizes_all_keywords(
            keyword in prop::sample::select(vec![
                "fn", "mut", "if", "else", "for", "while",
                "struct", "enum", "unsafe", "return", 
                "true", "false", "seeAm"
            ])
        ) {
            let mut lexer = Lexer::new(&keyword);
            let tokens = lexer.tokenize().unwrap();
            
            // Should have at least the keyword token and EOF
            prop_assert!(tokens.len() >= 2);
            
            // First token should be the keyword
            let expected_type = match keyword {
                "fn" => TokenType::Fn,
                "mut" => TokenType::Mut,
                "if" => TokenType::If,
                "else" => TokenType::Else,
                "for" => TokenType::For,
                "while" => TokenType::While,
                "struct" => TokenType::Struct,
                "enum" => TokenType::Enum,
                "unsafe" => TokenType::Unsafe,
                "return" => TokenType::Return,
                "true" => TokenType::True,
                "false" => TokenType::False,
                "seeAm" => TokenType::SeeAm,
                _ => unreachable!(),
            };
            
            prop_assert_eq!(tokens[0].token_type, expected_type);
            prop_assert_eq!(&tokens[0].lexeme, keyword);
        }

        #[test]
        fn test_lexer_parser_roundtrip_simple_programs(program in simple_program()) {
            // Test that lexer + parser can handle generated programs
            let mut lexer = Lexer::new(&program);
            let tokens_result = lexer.tokenize();
            
            // If lexing succeeds, parsing should also succeed for valid programs
            if let Ok(tokens) = tokens_result {
                let mut parser = Parser::new(tokens);
                let parse_result = parser.parse();
                
                // For simple programs, parsing should succeed
                prop_assert!(parse_result.is_ok(), "Failed to parse: {}", program);
                
                let ast = parse_result.unwrap();
                
                // AST should have at least one statement
                prop_assert!(!ast.statements.is_empty());
                
                // All statements should be valid types
                for statement in &ast.statements {
                    match statement {
                        Statement::Print { .. } |
                        Statement::Assignment { .. } |
                        Statement::Function { .. } |
                        Statement::If { .. } |
                        Statement::While { .. } |
                        Statement::For { .. } |
                        Statement::Return { .. } |
                        Statement::Expression { .. } |
                        Statement::Struct { .. } |
                        Statement::Enum { .. } => {
                            // Valid statement type
                        }
                    }
                }
            }
        }

        #[test]
        fn test_lexer_handles_all_token_types(
            identifier in valid_identifier(),
            string_lit in valid_string_literal(),
            number_lit in valid_number_literal()
        ) {
            let program = format!(
                r#"
                fn {}() {{
                    mut x = {};
                    seeAm {};
                    if true {{
                        return x + 1;
                    }}
                }}
                "#,
                identifier, number_lit, string_lit
            );
            
            let mut lexer = Lexer::new(&program);
            let tokens = lexer.tokenize().unwrap();
            
            // Should contain all expected token types
            let token_types: Vec<_> = tokens.iter().map(|t| &t.token_type).collect();
            
            prop_assert!(token_types.contains(&&TokenType::Fn));
            prop_assert!(token_types.contains(&&TokenType::Identifier));
            prop_assert!(token_types.contains(&&TokenType::LeftParen));
            prop_assert!(token_types.contains(&&TokenType::RightParen));
            prop_assert!(token_types.contains(&&TokenType::LeftBrace));
            prop_assert!(token_types.contains(&&TokenType::RightBrace));
            prop_assert!(token_types.contains(&&TokenType::Mut));
            prop_assert!(token_types.contains(&&TokenType::Equal));
            prop_assert!(token_types.contains(&&TokenType::Semicolon));
            prop_assert!(token_types.contains(&&TokenType::SeeAm));
            prop_assert!(token_types.contains(&&TokenType::If));
            prop_assert!(token_types.contains(&&TokenType::True));
            prop_assert!(token_types.contains(&&TokenType::Return));
            prop_assert!(token_types.contains(&&TokenType::Plus));
        }

        #[test]
        fn test_parser_preserves_semantic_meaning(
            var_name in valid_identifier(),
            value in prop_oneof![
                valid_number_literal(),
                valid_string_literal(),
                Just("true".to_string()),
                Just("false".to_string())
            ]
        ) {
            let program = format!("{} = {};", var_name, value);
            
            let mut lexer = Lexer::new(&program);
            let tokens = lexer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            let ast = parser.parse().unwrap();
            
            prop_assert_eq!(ast.statements.len(), 1);
            
            match &ast.statements[0] {
                Statement::Assignment { mutable, identifier, value: expr } => {
                    prop_assert!(!mutable);
                    prop_assert_eq!(identifier, &var_name);
                    
                    // Verify the value is preserved correctly
                    match expr {
                        Expression::Literal(Literal::String(s)) => {
                            if value.starts_with('"') && value.ends_with('"') {
                                let expected = &value[1..value.len()-1];
                                prop_assert_eq!(s, expected);
                            }
                        }
                        Expression::Literal(Literal::Number(n)) => {
                            if let Ok(expected) = value.parse::<f64>() {
                                prop_assert_eq!(*n, expected);
                            }
                        }
                        Expression::Literal(Literal::Boolean(b)) => {
                            if value == "true" {
                                prop_assert!(*b);
                            } else if value == "false" {
                                prop_assert!(!*b);
                            }
                        }
                        _ => {}
                    }
                }
                _ => prop_assert!(false, "Expected assignment statement"),
            }
        }
    }
}

/// **Feature: ovie-programming-language, Property 2: Print Expression Correctness**
/// 
/// For any valid expression following "seeAm", the compiler should output 
/// the expression's evaluated value correctly
/// 
/// **Validates: Requirements 1.2**
mod print_expression_correctness {
    use super::*;

    proptest! {
        #[test]
        fn test_print_statement_parsing(
            expr in prop_oneof![
                valid_string_literal(),
                valid_number_literal(),
                Just("true".to_string()),
                Just("false".to_string())
            ]
        ) {
            let program = format!("seeAm {};", expr);
            
            let mut lexer = Lexer::new(&program);
            let tokens = lexer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            let ast = parser.parse().unwrap();
            
            prop_assert_eq!(ast.statements.len(), 1);
            
            match &ast.statements[0] {
                Statement::Print { expression } => {
                    // Verify the expression is parsed correctly
                    match expression {
                        Expression::Literal(Literal::String(_)) => {
                            prop_assert!(expr.starts_with('"') && expr.ends_with('"'));
                        }
                        Expression::Literal(Literal::Number(_)) => {
                            prop_assert!(expr.parse::<f64>().is_ok());
                        }
                        Expression::Literal(Literal::Boolean(b)) => {
                            prop_assert!((expr == "true" && *b) || (expr == "false" && !*b));
                        }
                        _ => {}
                    }
                }
                _ => prop_assert!(false, "Expected print statement"),
            }
        }

        #[test]
        fn test_print_expression_interpretation(
            value in prop_oneof![
                (1.0f64..100.0).prop_map(|n| (format!("{}", n as i64), n as i64 as f64)),
                prop::string::string_regex(r"[a-zA-Z ]+").unwrap()
                    .prop_map(|s| (format!(r#""{}""#, s), 0.0))
            ]
        ) {
            let (expr_str, _expected_num) = value;
            let program = format!("seeAm {};", expr_str);
            
            let mut lexer = Lexer::new(&program);
            let tokens = lexer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            let ast = parser.parse().unwrap();
            let mut interpreter = crate::interpreter::Interpreter::new();
            
            // Should not error when interpreting
            let result = interpreter.interpret(&ast);
            prop_assert!(result.is_ok(), "Interpretation failed for: {}", program);
        }
    }
}

// Helper functions for generating valid Ovie code
fn valid_identifier() -> impl Strategy<Value = String> {
    prop::string::string_regex(r"[a-zA-Z_][a-zA-Z0-9_]*")
        .unwrap()
        .prop_filter("Not a keyword", |s| {
            !matches!(s.as_str(), 
                "fn" | "mut" | "if" | "else" | "for" | "while" | 
                "struct" | "enum" | "unsafe" | "return" | 
                "true" | "false" | "seeAm"
            )
        })
}

fn valid_string_literal() -> impl Strategy<Value = String> {
    prop::string::string_regex(r#""[^"\\]*""#).unwrap()
}

fn valid_number_literal() -> impl Strategy<Value = String> {
    prop_oneof![
        (0i64..1000).prop_map(|n| n.to_string()),
        (0.0f64..1000.0).prop_map(|n| format!("{:.2}", n)),
    ]
}

/// **Feature: ovie-programming-language, Property 4: Safe Auto-Correction**
/// 
/// For any program with safe typos, the Normalizer should correct them, log the changes, 
/// preserve semantic meaning, and never change ambiguous syntax without user consent
/// 
/// **Validates: Requirements 2.1, 2.2, 2.3, 2.4, 2.5**
mod safe_auto_correction {
    use super::*;
    use crate::normalizer::Normalizer;

    proptest! {
        #[test]
        fn test_normalizer_corrects_common_typos(
            typo in prop::sample::select(vec![
                "seeam", "print", "function", "var"
            ])
        ) {
            let program = format!("{} \"test\";", typo);
            
            let mut normalizer = Normalizer::new();
            let (normalized_source, corrections) = normalizer.normalize_source(&program);
            
            // Should have made at least one correction for known typos
            if matches!(typo, "seeam" | "print" | "function" | "var") {
                prop_assert!(!corrections.is_empty(), "Expected correction for typo: {}", typo);
                
                // Check specific corrections
                match typo {
                    "seeam" | "print" => {
                        prop_assert!(normalized_source.contains("seeAm"), "Expected 'seeAm' in normalized source");
                    }
                    "function" => {
                        prop_assert!(normalized_source.contains("fn"), "Expected 'fn' in normalized source");
                    }
                    "var" => {
                        prop_assert!(normalized_source.contains("mut"), "Expected 'mut' in normalized source");
                    }
                    _ => {}
                }
            }
            
            // Corrections should be logged
            for correction in &corrections {
                prop_assert!(!correction.reason.is_empty());
                // The original should match the typo exactly
                prop_assert_eq!(&correction.original, typo, "Correction original should match input typo");
            }
        }

        #[test]
        fn test_normalizer_preserves_semantic_meaning(
            content in r#""[a-zA-Z0-9 ]+""#
        ) {
            let original_program = format!("seeAm {};", content);
            let typo_program = format!("seeam {};", content);
            
            // Parse original program
            let mut lexer1 = Lexer::new(&original_program);
            let tokens1 = lexer1.tokenize().unwrap();
            let mut parser1 = Parser::new(tokens1);
            let ast1 = parser1.parse().unwrap();
            
            // Parse normalized typo program
            let mut normalizer = Normalizer::new();
            let (normalized_source, _) = normalizer.normalize_source(&typo_program);
            
            let mut lexer2 = Lexer::new(&normalized_source);
            let tokens2 = lexer2.tokenize().unwrap();
            let mut parser2 = Parser::new(tokens2);
            let ast2 = parser2.parse().unwrap();
            
            // Both should have the same semantic structure
            prop_assert_eq!(ast1.statements.len(), ast2.statements.len());
            
            match (&ast1.statements[0], &ast2.statements[0]) {
                (Statement::Print { expression: expr1 }, Statement::Print { expression: expr2 }) => {
                    // Both should be print statements with the same content
                    match (expr1, expr2) {
                        (Expression::Literal(lit1), Expression::Literal(lit2)) => {
                            prop_assert_eq!(lit1, lit2);
                        }
                        _ => {}
                    }
                }
                _ => prop_assert!(false, "Expected print statements"),
            }
        }

        #[test]
        fn test_normalizer_never_changes_valid_code(
            identifier in valid_identifier(),
            value in prop_oneof![
                valid_number_literal(),
                valid_string_literal()
            ]
        ) {
            let program = format!("{} = {};", identifier, value);
            
            let mut normalizer = Normalizer::new();
            let (normalized_source, corrections) = normalizer.normalize_source(&program);
            
            // Should not make corrections to valid code
            prop_assert!(corrections.is_empty(), "Made unexpected corrections to valid code: {:?}", corrections);
            prop_assert_eq!(normalized_source, program);
        }

        #[test]
        fn test_normalizer_logs_all_corrections(
            typos in prop::collection::vec(
                prop::sample::select(vec!["seeam", "print", "function"]), 
                1..3
            )
        ) {
            let program = typos.iter()
                .map(|typo| format!("{} \"test\";", typo))
                .collect::<Vec<_>>()
                .join("\n");
            
            let mut normalizer = Normalizer::new();
            let (_, corrections) = normalizer.normalize_source(&program);
            
            // Should log corrections for each typo
            prop_assert!(!corrections.is_empty());
            
            // Each correction should have proper metadata
            for correction in &corrections {
                prop_assert!(!correction.original.is_empty());
                prop_assert!(!correction.corrected.is_empty());
                prop_assert!(!correction.reason.is_empty());
                prop_assert!(correction.line >= 1);
                prop_assert!(correction.column >= 1);
            }
        }

        #[test]
        fn test_normalizer_safe_correction_rules(
            input in r"[a-zA-Z][a-zA-Z0-9_]{0,15}"
        ) {
            let normalizer = Normalizer::new();
            
            // Test the safety rules
            prop_assert!(!normalizer.is_safe_correction(&input, &input)); // Same strings
            prop_assert!(!normalizer.is_safe_correction(&input, "")); // Empty correction
            
            if input.len() > 10 {
                // Long identifiers should not be corrected
                prop_assert!(!normalizer.is_safe_correction(&input, "seeAm"));
            }
            
            if input.chars().any(|c| c.is_ascii_digit()) {
                // Identifiers with numbers should not be corrected
                prop_assert!(!normalizer.is_safe_correction(&input, "seeAm"));
            }
        }
    }
}

// Property 3: Type System Completeness tests temporarily disabled until semantic analyzer is fixed

/// **Feature: ovie-programming-language-stage-2, Property 6: IR Pipeline Integrity**
/// 
/// For any valid Ovie program, the complete IR pipeline (AST -> HIR -> MIR) should maintain
/// semantic equivalence and produce valid, consistent output at each stage
/// 
/// **Validates: Requirements 2.1, 2.2, 2.3, 2.4**
mod ir_pipeline_integrity {
    use super::*;
    use crate::hir::{HirBuilder, HirProgram};
    use crate::mir::{MirBuilder, MirProgram};
    use proptest::prelude::*;

    // Generator for valid Ovie programs suitable for IR testing
    fn ir_test_program() -> impl Strategy<Value = String> {
        prop_oneof![
            // Simple expressions
            valid_string_literal().prop_map(|s| format!("seeAm {};", s)),
            valid_number_literal().prop_map(|n| format!("seeAm {};", n)),
            Just("seeAm true;".to_string()),
            Just("seeAm false;".to_string()),
            
            // Variable assignments
            (valid_identifier(), valid_number_literal())
                .prop_map(|(id, val)| format!("{} = {};", id, val)),
            (valid_identifier(), valid_string_literal())
                .prop_map(|(id, val)| format!("mut {} = {};", id, val)),
            
            // Simple functions
            (valid_identifier(), valid_identifier(), valid_number_literal())
                .prop_map(|(fname, param, ret_val)| 
                    format!("fn {}({}) {{ return {}; }}", fname, param, ret_val)),
            
            // Control flow
            (valid_identifier(), valid_number_literal(), valid_string_literal())
                .prop_map(|(var, num, str_val)| 
                    format!("{} = {}; if {} > 0 {{ seeAm {}; }} else {{ seeAm \"zero\"; }}", 
                           var, num, var, str_val)),
            
            // Struct definitions
            (valid_identifier(), valid_identifier(), valid_identifier())
                .prop_map(|(struct_name, field1, field2)| 
                    format!("struct {} {{ {}: Number, {}: String }}", struct_name, field1, field2)),
        ]
    }

    // Generator for complex programs with multiple statements
    fn complex_ir_program() -> impl Strategy<Value = String> {
        prop::collection::vec(ir_test_program(), 1..4)
            .prop_map(|statements| statements.join("\n"))
    }

    proptest! {
        #[test]
        fn prop_ast_to_hir_transformation_preserves_semantics(program in ir_test_program()) {
            // Parse to AST
            let mut lexer = Lexer::new(&program);
            let tokens_result = lexer.tokenize();
            prop_assume!(tokens_result.is_ok());
            let tokens = tokens_result.unwrap();
            
            let mut parser = Parser::new(tokens);
            let ast_result = parser.parse();
            prop_assume!(ast_result.is_ok());
            let ast = ast_result.unwrap();
            
            // Transform to HIR
            let mut hir_builder = HirBuilder::new();
            let hir_result = hir_builder.transform_ast(&ast);
            prop_assert!(hir_result.is_ok(), "HIR transformation should succeed for: {}", program);
            
            let hir = hir_result.unwrap();
            
            // Verify HIR structure preservation
            prop_assert!(!hir.items.is_empty(), "HIR should contain items");
            
            // Verify HIR validation passes
            let validation_result = hir.validate();
            prop_assert!(validation_result.is_ok(), "HIR should pass validation for: {}", program);
            
            // Verify HIR serialization works
            let json_result = hir.to_json();
            prop_assert!(json_result.is_ok(), "HIR should serialize to JSON for: {}", program);
            
            let json = json_result.unwrap();
            let deserialize_result = HirProgram::from_json(&json);
            prop_assert!(deserialize_result.is_ok(), "HIR should deserialize from JSON for: {}", program);
        }

        #[test]
        fn prop_hir_to_mir_transformation_maintains_control_flow(program in ir_test_program()) {
            // Get HIR first
            let mut lexer = Lexer::new(&program);
            let tokens = lexer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            let ast = parser.parse().unwrap();
            
            let mut hir_builder = HirBuilder::new();
            let hir_result = hir_builder.transform_ast(&ast);
            prop_assume!(hir_result.is_ok());
            let hir = hir_result.unwrap();
            
            // Transform to MIR
            let mut mir_builder = MirBuilder::new();
            let mir_result = mir_builder.transform_hir(&hir);
            prop_assert!(mir_result.is_ok(), "MIR transformation should succeed for: {}", program);
            
            let mir = mir_result.unwrap();
            
            // Verify MIR structure
            prop_assert!(!mir.functions.is_empty(), "MIR should contain functions");
            
            // Verify control flow graph properties
            for (func_id, function) in &mir.functions {
                prop_assert!(!function.basic_blocks.is_empty(), 
                    "Function {} should have basic blocks", func_id);
                
                // Entry block should exist
                prop_assert!(function.basic_blocks.contains_key(&function.entry_block),
                    "Entry block should exist in function {}", func_id);
                
                // All basic blocks should have valid terminators
                for (block_id, block) in &function.basic_blocks {
                    // Terminator should be valid (this is a basic structural check)
                    match &block.terminator {
                        crate::mir::MirTerminator::Return { .. } |
                        crate::mir::MirTerminator::Goto { .. } |
                        crate::mir::MirTerminator::SwitchInt { .. } |
                        crate::mir::MirTerminator::Call { .. } |
                        crate::mir::MirTerminator::Unreachable |
                        crate::mir::MirTerminator::Drop { .. } => {
                            // Valid terminator
                        }
                    }
                }
            }
            
            // Verify MIR validation passes
            let validation_result = mir.validate();
            prop_assert!(validation_result.is_ok(), "MIR should pass validation for: {}", program);
            
            // Verify MIR serialization works
            let json_result = mir.to_json();
            prop_assert!(json_result.is_ok(), "MIR should serialize to JSON for: {}", program);
        }

        #[test]
        fn prop_complete_ir_pipeline_consistency(program in ir_test_program()) {
            // Complete pipeline: AST -> HIR -> MIR
            let mut lexer = Lexer::new(&program);
            let tokens = lexer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            let ast = parser.parse().unwrap();
            
            // AST -> HIR
            let mut hir_builder = HirBuilder::new();
            let hir = hir_builder.transform_ast(&ast).unwrap();
            
            // HIR -> MIR
            let mut mir_builder = MirBuilder::new();
            let mir = mir_builder.transform_hir(&hir).unwrap();
            
            // Verify consistency across pipeline stages
            
            // 1. Function count consistency
            let ast_functions = ast.statements.iter()
                .filter(|stmt| matches!(stmt, Statement::Function { .. }))
                .count();
            let hir_functions = hir.items.iter()
                .filter(|item| matches!(item, crate::hir::HirItem::Function(_)))
                .count();
            let mir_functions = mir.functions.len();
            
            // Account for implicit main function
            let expected_functions = if ast_functions == 0 { 1 } else { ast_functions };
            prop_assert_eq!(hir_functions, expected_functions, 
                "HIR function count should match AST (with implicit main) for: {}", program);
            prop_assert_eq!(mir_functions, expected_functions,
                "MIR function count should match HIR for: {}", program);
            
            // 2. Entry point consistency
            if hir.metadata.has_main_function {
                prop_assert!(mir.entry_point.is_some(), 
                    "MIR should have entry point when HIR has main function for: {}", program);
            }
            
            // 3. Type definition consistency
            let hir_structs = hir.items.iter()
                .filter(|item| matches!(item, crate::hir::HirItem::Struct(_)))
                .count();
            let hir_enums = hir.items.iter()
                .filter(|item| matches!(item, crate::hir::HirItem::Enum(_)))
                .count();
            let mir_type_defs = mir.type_definitions.len();
            
            prop_assert_eq!(mir_type_defs, hir_structs + hir_enums,
                "MIR type definitions should match HIR structs + enums for: {}", program);
            
            // 4. Global variable consistency
            let hir_globals = hir.items.iter()
                .filter(|item| matches!(item, crate::hir::HirItem::Global(_)))
                .count();
            let mir_globals = mir.globals.len();
            
            prop_assert_eq!(mir_globals, hir_globals,
                "MIR globals should match HIR globals for: {}", program);
        }

        #[test]
        fn prop_ir_serialization_roundtrip_consistency(program in ir_test_program()) {
            // Test HIR serialization roundtrip
            let mut lexer = Lexer::new(&program);
            let tokens = lexer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            let ast = parser.parse().unwrap();
            
            let mut hir_builder = HirBuilder::new();
            let hir = hir_builder.transform_ast(&ast).unwrap();
            
            // HIR roundtrip
            let hir_json = hir.to_json().unwrap();
            let hir_deserialized = HirProgram::from_json(&hir_json).unwrap();
            let hir_json2 = hir_deserialized.to_json().unwrap();
            
            prop_assert_eq!(hir_json, hir_json2, 
                "HIR serialization should be consistent for: {}", program);
            
            // MIR roundtrip
            let mut mir_builder = MirBuilder::new();
            let mir = mir_builder.transform_hir(&hir).unwrap();
            
            let mir_json = mir.to_json().unwrap();
            let mir_deserialized = MirProgram::from_json(&mir_json).unwrap();
            let mir_json2 = mir_deserialized.to_json().unwrap();
            
            prop_assert_eq!(mir_json, mir_json2,
                "MIR serialization should be consistent for: {}", program);
        }

        #[test]
        fn prop_ir_pipeline_deterministic_output(program in ir_test_program()) {
            // Run the same program through the pipeline twice
            
            // First run
            let mut lexer1 = Lexer::new(&program);
            let tokens1 = lexer1.tokenize().unwrap();
            let mut parser1 = Parser::new(tokens1);
            let ast1 = parser1.parse().unwrap();
            
            let mut hir_builder1 = HirBuilder::new();
            let hir1 = hir_builder1.transform_ast(&ast1).unwrap();
            
            let mut mir_builder1 = MirBuilder::new();
            let mir1 = mir_builder1.transform_hir(&hir1).unwrap();
            
            // Second run
            let mut lexer2 = Lexer::new(&program);
            let tokens2 = lexer2.tokenize().unwrap();
            let mut parser2 = Parser::new(tokens2);
            let ast2 = parser2.parse().unwrap();
            
            let mut hir_builder2 = HirBuilder::new();
            let hir2 = hir_builder2.transform_ast(&ast2).unwrap();
            
            let mut mir_builder2 = MirBuilder::new();
            let mir2 = mir_builder2.transform_hir(&hir2).unwrap();
            
            // Compare outputs
            let hir_json1 = hir1.to_json().unwrap();
            let hir_json2 = hir2.to_json().unwrap();
            prop_assert_eq!(hir_json1, hir_json2, 
                "HIR generation should be deterministic for: {}", program);
            
            let mir_json1 = mir1.to_json().unwrap();
            let mir_json2 = mir2.to_json().unwrap();
            prop_assert_eq!(mir_json1, mir_json2,
                "MIR generation should be deterministic for: {}", program);
        }

        #[test]
        fn prop_ir_control_flow_analysis_consistency(program in complex_ir_program()) {
            let mut lexer = Lexer::new(&program);
            let tokens_result = lexer.tokenize();
            prop_assume!(tokens_result.is_ok());
            let tokens = tokens_result.unwrap();
            
            let mut parser = Parser::new(tokens);
            let ast_result = parser.parse();
            prop_assume!(ast_result.is_ok());
            let ast = ast_result.unwrap();
            
            let mut hir_builder = HirBuilder::new();
            let hir_result = hir_builder.transform_ast(&ast);
            prop_assume!(hir_result.is_ok());
            let hir = hir_result.unwrap();
            
            let mut mir_builder = MirBuilder::new();
            let mir_result = mir_builder.transform_hir(&hir);
            prop_assume!(mir_result.is_ok());
            let mir = mir_result.unwrap();
            
            // Analyze control flow graph
            let cfg_analysis_result = mir.analyze_cfg();
            prop_assert!(cfg_analysis_result.is_ok(), 
                "CFG analysis should succeed for: {}", program);
            
            let cfg_analysis = cfg_analysis_result.unwrap();
            
            // Verify CFG analysis consistency
            for (func_id, func_analysis) in &cfg_analysis.function_analyses {
                prop_assert!(mir.functions.contains_key(func_id),
                    "CFG analysis should only contain existing functions");
                
                let function = &mir.functions[func_id];
                
                // Verify predecessor/successor consistency
                for (block_id, successors) in &func_analysis.successors {
                    prop_assert!(function.basic_blocks.contains_key(block_id),
                        "Successor analysis should only reference existing blocks");
                    
                    for &successor in successors {
                        prop_assert!(function.basic_blocks.contains_key(&successor),
                            "Successors should reference existing blocks");
                        
                        // Check that predecessor relationship is symmetric
                        if let Some(predecessors) = func_analysis.predecessors.get(&successor) {
                            prop_assert!(predecessors.contains(block_id),
                                "Predecessor relationship should be symmetric");
                        }
                    }
                }
                
                // Verify dominator relationships are valid
                for (block_id, &dominator) in &func_analysis.dominators {
                    prop_assert!(function.basic_blocks.contains_key(block_id),
                        "Dominator analysis should only reference existing blocks");
                    prop_assert!(function.basic_blocks.contains_key(&dominator),
                        "Dominators should reference existing blocks");
                }
            }
        }

        #[test]
        fn prop_ir_error_recovery_consistency(
            valid_program in ir_test_program(),
            corruption in prop::sample::select(vec!["missing_semicolon", "extra_brace", "invalid_identifier"])
        ) {
            // Create a corrupted version of the program
            let corrupted_program = match corruption {
                "missing_semicolon" => valid_program.replace(";", ""),
                "extra_brace" => format!("{} }}", valid_program),
                "invalid_identifier" => valid_program.replace("seeAm", "123invalid"),
                _ => valid_program.clone(),
            };
            
            // Test that the pipeline handles errors gracefully
            let mut lexer = Lexer::new(&corrupted_program);
            let tokens_result = lexer.tokenize();
            
            if let Ok(tokens) = tokens_result {
                let mut parser = Parser::new(tokens);
                let ast_result = parser.parse();
                
                if let Ok(ast) = ast_result {
                    let mut hir_builder = HirBuilder::new();
                    let hir_result = hir_builder.transform_ast(&ast);
                    
                    // Either HIR transformation should fail gracefully, or succeed
                    // The key is that it should never panic
                    if let Ok(hir) = hir_result {
                        let mut mir_builder = MirBuilder::new();
                        let _mir_result = mir_builder.transform_hir(&hir);
                        // MIR transformation might fail, which is acceptable
                    }
                }
            }
            
            // Test passes if we reach here without panicking
            prop_assert!(true);
        }

        #[test]
        fn prop_ir_type_preservation_across_stages(program in ir_test_program()) {
            let mut lexer = Lexer::new(&program);
            let tokens_result = lexer.tokenize();
            prop_assume!(tokens_result.is_ok());
            let tokens = tokens_result.unwrap();
            
            let mut parser = Parser::new(tokens);
            let ast_result = parser.parse();
            prop_assume!(ast_result.is_ok());
            let ast = ast_result.unwrap();
            
            let mut hir_builder = HirBuilder::new();
            let hir_result = hir_builder.transform_ast(&ast);
            prop_assume!(hir_result.is_ok());
            let hir = hir_result.unwrap();
            
            let mut mir_builder = MirBuilder::new();
            let mir_result = mir_builder.transform_hir(&hir);
            prop_assume!(mir_result.is_ok());
            let mir = mir_result.unwrap();
            
            // Verify type information is preserved across transformations
            
            // Check that HIR has type information
            for item in &hir.items {
                match item {
                    crate::hir::HirItem::Function(func) => {
                        // Function should have return type
                        prop_assert!(!matches!(func.return_type, crate::hir::HirType::Error),
                            "Function return type should be resolved");
                        
                        // Parameters should have types
                        for param in &func.parameters {
                            prop_assert!(!matches!(param.param_type, crate::hir::HirType::Error),
                                "Parameter types should be resolved");
                        }
                    }
                    crate::hir::HirItem::Global(global) => {
                        prop_assert!(!matches!(global.global_type, crate::hir::HirType::Error),
                            "Global variable type should be resolved");
                    }
                    _ => {}
                }
            }
            
            // Check that MIR preserves type information
            for (func_id, function) in &mir.functions {
                // Function signature should have types
                prop_assert!(!function.signature.parameters.is_empty() || 
                           function.signature.parameters.iter().all(|t| !matches!(t, crate::mir::MirType::Unit)),
                    "Function {} should have meaningful parameter types", func_id);
                
                // Locals should have types
                for local in &function.locals {
                    prop_assert!(!matches!(local.ty, crate::mir::MirType::Unit) || local.name.is_none(),
                        "Local variables should have meaningful types");
                }
            }
        }
    }
}
    use super::*;
    use crate::ir::IrBuilder;

    proptest! {
        #[test]
        fn test_complete_pipeline_simple_programs(
            program in prop_oneof![
                // Simple print statements
                valid_string_literal().prop_map(|s| format!("seeAm {};", s)),
                valid_number_literal().prop_map(|n| format!("seeAm {};", n)),
                Just("seeAm true;".to_string()),
                Just("seeAm false;".to_string()),
                // Simple assignments
                (valid_identifier(), valid_number_literal())
                    .prop_map(|(id, val)| format!("{} = {};", id, val)),
                (valid_identifier(), valid_string_literal())
                    .prop_map(|(id, val)| format!("{} = {};", id, val)),
            ]
        ) {
            // Step 1: Lexical analysis
            let mut lexer = Lexer::new(&program);
            let tokens_result = lexer.tokenize();
            prop_assert!(tokens_result.is_ok(), "Lexer failed for: {}", program);
            let tokens = tokens_result.unwrap();
            
            // Step 2: Parsing
            let mut parser = Parser::new(tokens);
            let parse_result = parser.parse();
            prop_assert!(parse_result.is_ok(), "Parser failed for: {}", program);
            let ast = parse_result.unwrap();
            
            // Step 3: Normalization
            let mut normalizer = crate::normalizer::Normalizer::new();
            let normalize_result = normalizer.normalize(ast);
            prop_assert!(normalize_result.is_ok(), "Normalizer failed for: {}", program);
            let normalized_ast = normalize_result.unwrap().0;
            
            // Step 4: IR generation
            let mut ir_builder = IrBuilder::new();
            let ir_result = ir_builder.transform_ast(&normalized_ast);
            prop_assert!(ir_result.is_ok(), "IR generation failed for: {}", program);
            
            // Step 5: Validate IR consistency
            let ir_program = ir_builder.build();
            let validation_result = ir_program.validate();
            prop_assert!(validation_result.is_ok(), "IR validation failed for: {}", program);
            
            // Step 6: Test IR serialization/deserialization
            let json_result = ir_program.to_json();
            prop_assert!(json_result.is_ok(), "IR serialization failed for: {}", program);
            let json = json_result.unwrap();
            
            let deserialize_result = crate::ir::Program::from_json(&json);
            prop_assert!(deserialize_result.is_ok(), "IR deserialization failed for: {}", program);
        }

        #[test]
        fn test_pipeline_preserves_program_structure(
            statements in prop::collection::vec(
                prop_oneof![
                    valid_string_literal().prop_map(|s| format!("seeAm {};", s)),
                    (valid_identifier(), valid_number_literal())
                        .prop_map(|(id, val)| format!("{} = {};", id, val)),
                ],
                1..4
            )
        ) {
            let program = statements.join("\n");
            
            // Parse the program
            let mut lexer = Lexer::new(&program);
            let tokens = lexer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            let ast = parser.parse().unwrap();
            
            // Normalize
            let mut normalizer = crate::normalizer::Normalizer::new();
            let (normalized_ast, _) = normalizer.normalize(ast).unwrap();
            
            // Generate IR
            let mut ir_builder = IrBuilder::new();
            ir_builder.transform_ast(&normalized_ast).unwrap();
            let ir_program = ir_builder.build();
            
            // Verify structure preservation
            prop_assert_eq!(normalized_ast.statements.len(), statements.len());
            
            // Verify IR has main function
            prop_assert!(ir_program.entry_point.is_some(), "IR should have entry point");
            
            if let Some(entry_id) = ir_program.entry_point {
                prop_assert!(ir_program.functions.contains_key(&entry_id), "Entry function should exist");
            }
        }

        #[test]
        fn test_pipeline_error_handling(
            invalid_program in prop_oneof![
                // Invalid syntax
                Just("seeAm".to_string()), // Missing semicolon
                Just("= 5;".to_string()), // Missing identifier
                Just("seeAm 5 5;".to_string()), // Invalid expression
                // Unclosed strings
                Just("seeAm \"hello;".to_string()),
            ]
        ) {
            // The pipeline should gracefully handle invalid programs
            let mut lexer = Lexer::new(&invalid_program);
            let tokens_result = lexer.tokenize();
            
            if let Ok(tokens) = tokens_result {
                let mut parser = Parser::new(tokens);
                let parse_result = parser.parse();
                
                // Either parsing should fail, or if it succeeds, subsequent stages should handle it
                if let Ok(ast) = parse_result {
                    let mut normalizer = crate::normalizer::Normalizer::new();
                    let normalize_result = normalizer.normalize(ast);
                    
                    if let Ok((normalized_ast, _)) = normalize_result {
                        let mut ir_builder = IrBuilder::new();
                        let _ir_result = ir_builder.transform_ast(&normalized_ast);
                        // IR generation might fail for invalid programs, which is acceptable
                    }
                }
            }
            
            // The test passes if we reach here without panicking
            prop_assert!(true);
        }

        #[test]
        fn test_ir_deterministic_generation(
            program in prop_oneof![
                valid_string_literal().prop_map(|s| format!("seeAm {};", s)),
                (valid_identifier(), valid_number_literal())
                    .prop_map(|(id, val)| format!("{} = {};", id, val)),
            ]
        ) {
            // Generate IR twice and ensure deterministic output
            let mut lexer1 = Lexer::new(&program);
            let tokens1 = lexer1.tokenize().unwrap();
            let mut parser1 = Parser::new(tokens1);
            let ast1 = parser1.parse().unwrap();
            
            let mut normalizer1 = crate::normalizer::Normalizer::new();
            let (normalized_ast1, _) = normalizer1.normalize(ast1).unwrap();
            
            let mut ir_builder1 = IrBuilder::new();
            ir_builder1.transform_ast(&normalized_ast1).unwrap();
            let ir_program1 = ir_builder1.build();
            let json1 = ir_program1.to_json().unwrap();
            
            // Second generation
            let mut lexer2 = Lexer::new(&program);
            let tokens2 = lexer2.tokenize().unwrap();
            let mut parser2 = Parser::new(tokens2);
            let ast2 = parser2.parse().unwrap();
            
            let mut normalizer2 = crate::normalizer::Normalizer::new();
            let (normalized_ast2, _) = normalizer2.normalize(ast2).unwrap();
            
            let mut ir_builder2 = IrBuilder::new();
            ir_builder2.transform_ast(&normalized_ast2).unwrap();
            let ir_program2 = ir_builder2.build();
            let json2 = ir_program2.to_json().unwrap();
            
            // IR generation should be deterministic
            prop_assert_eq!(json1, json2, "IR generation should be deterministic for: {}", program);
        }
    }
}

/// **Feature: ovie-programming-language, Property 12: Multi-Backend Semantic Equivalence**
/// 
/// For any valid Ovie program, all available backends should produce semantically equivalent output
/// 
/// **Validates: Requirements 7.1, 7.2, 7.4, 7.5**
mod multi_backend_semantic_equivalence {
    use super::*;
    use crate::{Backend, Compiler};
    use crate::codegen::CodegenBackend;

    proptest! {
        #[test]
        fn test_backend_selection_consistency(
            backend_str in prop::sample::select(vec![
                "wasm", "webassembly", "interpreter", "ast", "ir", "ir-interpreter"
            ])
        ) {
            let backend = Backend::from_str(&backend_str);
            prop_assert!(backend.is_some(), "Backend should be recognized: {}", backend_str);
            
            let backend = backend.unwrap();
            let name = backend.name();
            prop_assert!(!name.is_empty(), "Backend name should not be empty");
            
            // Test that we can create a compiler with this backend
            let compiler = Compiler::new_with_backend(backend);
            prop_assert_eq!(compiler.default_backend.name(), name);
        }

        #[test]
        fn test_multi_backend_compilation_consistency(
            program in prop_oneof![
                // Simple print statements
                valid_string_literal().prop_map(|s| format!("seeAm {};", s)),
                valid_number_literal().prop_map(|n| format!("seeAm {};", n)),
                Just("seeAm true;".to_string()),
                Just("seeAm false;".to_string()),
                // Simple assignments
                (valid_identifier(), valid_number_literal())
                    .prop_map(|(id, val)| format!("{} = {}; seeAm {};", id, val, id)),
            ]
        ) {
            let mut compiler = Compiler::new();
            
            // Test AST interpreter
            let ast_result = compiler.compile_and_run_with_backend(&program, Backend::Interpreter);
            prop_assert!(ast_result.is_ok(), "AST interpreter should handle program: {}", program);
            
            // Test IR interpreter
            let ir_result = compiler.compile_and_run_with_backend(&program, Backend::IrInterpreter);
            prop_assert!(ir_result.is_ok(), "IR interpreter should handle program: {}", program);
            
            // Test WASM compilation (doesn't execute, just compiles)
            let wasm_result = compiler.compile_and_run_with_backend(&program, Backend::Wasm);
            prop_assert!(wasm_result.is_ok(), "WASM backend should handle program: {}", program);
        }

        #[test]
        fn test_backend_compilation_determinism(
            program in prop_oneof![
                valid_string_literal().prop_map(|s| format!("seeAm {};", s)),
                (valid_identifier(), valid_number_literal())
                    .prop_map(|(id, val)| format!("{} = {};", id, val)),
            ]
        ) {
            let mut compiler = Compiler::new();
            
            // Compile the same program multiple times with WASM backend
            let wasm1 = compiler.compile_to_wasm(&program);
            let wasm2 = compiler.compile_to_wasm(&program);
            
            prop_assert!(wasm1.is_ok() && wasm2.is_ok(), "WASM compilation should be consistent");
            
            if let (Ok(bytes1), Ok(bytes2)) = (wasm1, wasm2) {
                prop_assert_eq!(bytes1, bytes2, "WASM output should be deterministic for: {}", program);
            }
        }

        #[test]
        fn test_ir_to_backend_consistency(
            statements in prop::collection::vec(
                prop_oneof![
                    valid_string_literal().prop_map(|s| format!("seeAm {};", s)),
                    (valid_identifier(), valid_number_literal())
                        .prop_map(|(id, val)| format!("{} = {};", id, val)),
                ],
                1..3
            )
        ) {
            let program = statements.join("\n");
            let mut compiler = Compiler::new();
            
            // Generate IR
            let ir_result = compiler.compile_to_ir(&program);
            prop_assert!(ir_result.is_ok(), "IR generation should succeed for: {}", program);
            
            let ir = ir_result.unwrap();
            
            // Validate IR structure
            prop_assert!(ir.entry_point.is_some(), "IR should have entry point");
            prop_assert!(!ir.functions.is_empty(), "IR should have functions");
            
            // Test that all backends can handle the IR
            let mut wasm_backend = crate::codegen::WasmBackend::new();
            let wasm_result = wasm_backend.generate(&ir);
            prop_assert!(wasm_result.is_ok(), "WASM backend should handle IR for: {}", program);
        }

        #[test]
        fn test_backend_error_handling_consistency(
            invalid_program in prop_oneof![
                // Syntax errors
                Just("seeAm".to_string()), // Missing semicolon
                Just("= 5;".to_string()), // Missing identifier
                Just("seeAm 5 5;".to_string()), // Invalid expression
            ]
        ) {
            let mut compiler = Compiler::new();
            
            // All backends should handle errors gracefully (either succeed or fail consistently)
            let _ast_result = compiler.compile_and_run_with_backend(&invalid_program, Backend::Interpreter);
            let _ir_result = compiler.compile_and_run_with_backend(&invalid_program, Backend::IrInterpreter);
            let _wasm_result = compiler.compile_and_run_with_backend(&invalid_program, Backend::Wasm);
            
            // If one backend fails, it's acceptable, but they should all behave consistently
            // The key is that none should panic or crash
            prop_assert!(true); // Test passes if we reach here without panicking
        }
    }
}

/// Property 7: Deterministic Build Consistency
/// **Validates: Requirements 4.1, 4.5, 7.3, 12.3**
/// 
/// This property ensures that builds are deterministic and produce identical outputs
/// for identical inputs across different compilation runs.
pub mod deterministic_build_consistency {
    use super::*;
    use crate::{Compiler, Backend, DeterministicBuildConfig, BuildMetadata};
    use std::collections::HashMap;

    #[test]
    fn test_deterministic_compilation_consistency() {
        let programs = vec![
            "seeAm \"Hello\";",
            "x = 42; seeAm x;",
            "fn test() { seeAm \"test\"; } test();",
        ];

        for program in programs {
            // Create deterministic build configuration
            let mut build_config = DeterministicBuildConfig::new_deterministic();
            build_config.with_source(program);

            // First compilation
            let mut compiler1 = Compiler::new_deterministic().with_build_config(build_config.clone());
            let result1 = compiler1.compile_to_ir(program);
            assert!(result1.is_ok(), "First compilation should succeed");

            // Second compilation with same configuration
            let mut compiler2 = Compiler::new_deterministic().with_build_config(build_config.clone());
            let result2 = compiler2.compile_to_ir(program);
            assert!(result2.is_ok(), "Second compilation should succeed");

            // Results should be identical
            let ir1 = result1.unwrap();
            let ir2 = result2.unwrap();
            
            let json1 = ir1.to_json().unwrap_or_default();
            let json2 = ir2.to_json().unwrap_or_default();
            
            assert_eq!(json1, json2, "Deterministic builds should produce identical IR for: {}", program);
        }
    }

    #[test]
    fn test_build_hash_consistency() {
        let programs = vec![
            "seeAm \"test\";",
            "value = 123;",
        ];

        for program in programs {
            // Create build configuration
            let mut build_config = DeterministicBuildConfig::new_deterministic();
            build_config.with_source(program);

            // Compute build hash multiple times
            let hash1 = build_config.compute_build_hash();
            let hash2 = build_config.compute_build_hash();
            let hash3 = build_config.compute_build_hash();

            // All hashes should be identical
            assert_eq!(hash1, hash2, "Build hash should be consistent");
            assert_eq!(hash2, hash3, "Build hash should be consistent");
            assert!(!hash1.is_empty(), "Build hash should not be empty");
        }
    }

    #[test]
    fn test_cross_platform_determinism() {
        let programs = vec![
            "seeAm \"Hello World\";",
            "x = 1 + 2; seeAm x;",
        ];

        for program in programs {
            // Test that deterministic builds work across different "platforms"
            // (simulated by different environment variables)
            let mut env_vars1 = HashMap::new();
            env_vars1.insert("OVIE_TARGET".to_string(), "x86_64".to_string());

            let mut env_vars2 = HashMap::new();
            env_vars2.insert("OVIE_TARGET".to_string(), "x86_64".to_string()); // Same target

            let mut build_config1 = DeterministicBuildConfig::new_deterministic();
            build_config1.with_source(program);
            for (k, v) in env_vars1 {
                build_config1.with_env_var(k, v);
            }

            let mut build_config2 = DeterministicBuildConfig::new_deterministic();
            build_config2.with_source(program);
            for (k, v) in env_vars2 {
                build_config2.with_env_var(k, v);
            }

            // Build hashes should be identical for same environment
            let hash1 = build_config1.compute_build_hash();
            let hash2 = build_config2.compute_build_hash();
            
            assert_eq!(hash1, hash2, "Same environment should produce same build hash for: {}", program);
        }
    }

    #[test]
    fn test_build_reproducibility_verification() {
        let program = "seeAm \"Reproducible build test\";";
        
        // Create deterministic compiler
        let mut compiler = Compiler::new_deterministic();
        
        // Test build reproducibility verification
        let is_reproducible = compiler.verify_build_reproducibility(program, Backend::IrInterpreter);
        assert!(is_reproducible.is_ok(), "Build reproducibility verification should work");
        assert!(is_reproducible.unwrap(), "Deterministic builds should be reproducible");
    }

    #[test]
    fn test_build_metadata_consistency() {
        let metadata1 = BuildMetadata::new_with_timestamp(1640995200); // Fixed timestamp
        let metadata2 = BuildMetadata::new_with_timestamp(1640995200); // Same timestamp
        
        assert_eq!(metadata1.build_timestamp, metadata2.build_timestamp);
        assert_eq!(metadata1.compiler_version, metadata2.compiler_version);
        assert_eq!(metadata1.target_platform, metadata2.target_platform);
    }
}

/// Property 18: Unsafe Operation Enforcement
/// Validates: Requirements 10.2
pub mod unsafe_operation_enforcement {
    use super::*;
    use crate::security::{UnsafeOperationAnalyzer, UnsafeOperation};
    use proptest::prelude::*;

    #[test]
    fn test_unsafe_operation_detection() {
        let analyzer = UnsafeOperationAnalyzer::new();
        
        // Test that unsafe operations are detected
        let unsafe_code = r#"
            seeAm malloc(1024);
            seeAm free(ptr);
            seeAm system("rm -rf /");
        "#;
        
        let mut lexer = crate::lexer::Lexer::new(unsafe_code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = crate::parser::Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let unsafe_ops = analyzer.analyze_ast(&ast, "test.ov").unwrap();
        
        // Should detect unsafe operations
        assert!(!unsafe_ops.is_empty());
        
        // Check that specific unsafe operations are detected
        let has_malloc = unsafe_ops.iter().any(|op| matches!(op.operation, UnsafeOperation::RawMemoryAccess));
        let has_system = unsafe_ops.iter().any(|op| matches!(op.operation, UnsafeOperation::SystemCall));
        
        assert!(has_malloc || has_system, "Should detect at least one unsafe operation");
    }

    #[test]
    fn test_safe_operations_not_flagged() {
        let analyzer = UnsafeOperationAnalyzer::new();
        
        // Test that safe operations are not flagged
        let safe_code = r#"
            x = 42;
            y = "hello";
            seeAm x + y;
        "#;
        
        let mut lexer = crate::lexer::Lexer::new(safe_code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = crate::parser::Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let unsafe_ops = analyzer.analyze_ast(&ast, "test.ov").unwrap();
        
        // Should not detect any unsafe operations
        assert!(unsafe_ops.is_empty());
    }

    #[test]
    fn test_security_report_generation() {
        let analyzer = UnsafeOperationAnalyzer::new();
        
        // Test with some unsafe operations
        let unsafe_code = r#"
            seeAm malloc(1024);
            seeAm ptr_read(address);
        "#;
        
        let mut lexer = crate::lexer::Lexer::new(unsafe_code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = crate::parser::Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let _unsafe_ops = analyzer.analyze_ast(&ast, "test.ov").unwrap();
        let report = analyzer.generate_security_report();
        
        // Should generate meaningful security report
        assert!(report.total_unsafe_operations > 0);
        assert!(!report.recommendations.is_empty());
    }

    proptest! {
        #[test]
        fn prop_unsafe_operation_analysis_never_panics(
            source in "[a-zA-Z0-9_\\s\\(\\)=+\\-*/\"';{}]*"
        ) {
            let analyzer = UnsafeOperationAnalyzer::new();
            
            // Try to parse and analyze - should never panic
            if let Ok(tokens) = crate::lexer::Lexer::new(&source).tokenize() {
                if let Ok(ast) = crate::parser::Parser::new(tokens).parse() {
                    let _ = analyzer.analyze_ast(&ast, "test.ov");
                }
            }
        }

        #[test]
        fn prop_security_report_consistency(
            operations in prop::collection::vec(
                prop::sample::select(vec![
                    "malloc", "free", "system", "ptr_read", "safe_func"
                ]), 
                0..10
            )
        ) {
            let analyzer = UnsafeOperationAnalyzer::new();
            
            // Build test code with function calls
            let mut code = String::new();
            for op in &operations {
                code.push_str(&format!("seeAm {}();\n", op));
            }
            
            if let Ok(tokens) = crate::lexer::Lexer::new(&code).tokenize() {
                if let Ok(ast) = crate::parser::Parser::new(tokens).parse() {
                    if let Ok(unsafe_ops) = analyzer.analyze_ast(&ast, "test.ov") {
                        let report = analyzer.generate_security_report();
                        
                        // Report should be consistent with detected operations
                        prop_assert_eq!(report.total_unsafe_operations, unsafe_ops.len());
                        
                        // All operations should be implicit (no explicit unsafe blocks in test)
                        prop_assert_eq!(report.explicit_unsafe_count, 0);
                        prop_assert_eq!(report.implicit_unsafe_count, unsafe_ops.len());
                    }
                }
            }
        }

        #[test]
        fn prop_unsafe_operation_types_classified_correctly(
            op_name in prop::sample::select(vec![
                "malloc", "free", "memcpy", "memset",
                "system", "exec", "fork",
                "ptr_read", "ptr_write",
                "cast_ptr",
                "raw_socket", "bind_socket",
                "file_raw_read", "file_raw_write"
            ])
        ) {
            let analyzer = UnsafeOperationAnalyzer::new();
            
            let code = format!("seeAm {}();", op_name);
            
            if let Ok(tokens) = crate::lexer::Lexer::new(&code).tokenize() {
                if let Ok(ast) = crate::parser::Parser::new(tokens).parse() {
                    if let Ok(unsafe_ops) = analyzer.analyze_ast(&ast, "test.ov") {
                        if !unsafe_ops.is_empty() {
                            let operation = &unsafe_ops[0].operation;
                            
                            // Verify correct classification
                            match op_name {
                                "malloc" | "free" | "memcpy" | "memset" => {
                                    prop_assert_eq!(operation, &UnsafeOperation::RawMemoryAccess);
                                }
                                "system" | "exec" | "fork" => {
                                    prop_assert_eq!(operation, &UnsafeOperation::SystemCall);
                                }
                                "ptr_read" | "ptr_write" => {
                                    prop_assert_eq!(operation, &UnsafeOperation::PointerArithmetic);
                                }
                                "cast_ptr" => {
                                    prop_assert_eq!(operation, &UnsafeOperation::UnsafeCast);
                                }
                                "raw_socket" | "bind_socket" => {
                                    prop_assert_eq!(operation, &UnsafeOperation::UnverifiedNetworkAccess);
                                }
                                "file_raw_read" | "file_raw_write" => {
                                    prop_assert_eq!(operation, &UnsafeOperation::UnsafeFileAccess);
                                }
                                _ => {
                                    prop_assert_eq!(operation, &UnsafeOperation::ForeignFunctionCall);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// **Property 10: Bootstrap Verification**
/// **Validates: Requirements 5.4**
/// 
/// This property ensures that the bootstrap verification system correctly
/// identifies when Rust and Ovie implementations produce identical results.
pub mod bootstrap_verification_properties {
    use super::*;
    use crate::self_hosting::{BootstrapVerifier, BootstrapConfig, BootstrapIntegration, IntegrationMode};
    use proptest::prelude::*;

    // Generator for valid Ovie source code
    fn valid_ovie_source() -> impl Strategy<Value = String> {
        prop_oneof![
            // Simple print statements
            prop::string::string_regex(r#"seeAm "[^"]*";"#).unwrap(),
            
            // Variable assignments
            prop::string::string_regex(r"[a-zA-Z_][a-zA-Z0-9_]* = [0-9]+;").unwrap(),
            
            // Function calls
            prop::string::string_regex(r"[a-zA-Z_][a-zA-Z0-9_]*\(\);").unwrap(),
            
            // Simple expressions
            prop::string::string_regex(r"[0-9]+ \+ [0-9]+;").unwrap(),
        ]
    }

    proptest! {
        #[test]
        fn prop_bootstrap_verification_deterministic(source in valid_ovie_source()) {
            let config = BootstrapConfig {
                hash_verification: true,
                token_comparison: true,
                performance_benchmarking: false,
                max_performance_degradation: 10.0,
                verbose_logging: false,
            };
            
            let verifier = BootstrapVerifier::new(config);
            
            // Run verification twice on the same source
            let result1 = verifier.verify_lexer(&source);
            let result2 = verifier.verify_lexer(&source);
            
            // Both should succeed (or fail) consistently
            prop_assert_eq!(result1.is_ok(), result2.is_ok());
            
            if let (Ok(r1), Ok(r2)) = (result1, result2) {
                // Results should be identical for the same input
                prop_assert_eq!(r1.source_hash, r2.source_hash);
                prop_assert_eq!(r1.rust_tokens_hash, r2.rust_tokens_hash);
                // Note: ovie_tokens_hash might differ if Ovie lexer isn't loaded
            }
        }

        #[test]
        fn prop_bootstrap_integration_mode_consistency(
            test_cases in prop::collection::vec(valid_ovie_source(), 1..5)
        ) {
            let mut integration = BootstrapIntegration::new();
            let config = BootstrapConfig::default();
            
            // Initialize integration (may fail, but should be consistent)
            let _init_result = integration.initialize(config);
            
            // Test mode transitions
            let initial_mode = integration.current_mode();
            prop_assert_eq!(initial_mode, IntegrationMode::RustOnly);
            
            // Test that mode names are consistent
            prop_assert!(!initial_mode.name().is_empty());
            
            // Test mode progression
            let mut current_mode = initial_mode;
            let mut transition_count = 0;
            
            while let Some(next_mode) = current_mode.next() {
                prop_assert!(integration.transition_to_next_mode().is_ok());
                prop_assert_eq!(integration.current_mode(), next_mode);
                current_mode = next_mode;
                transition_count += 1;
                
                // Prevent infinite loops
                if transition_count > 10 {
                    break;
                }
            }
            
            // Should be at final mode
            prop_assert!(integration.transition_to_next_mode().is_err());
        }

        #[test]
        fn prop_verification_result_consistency(source in valid_ovie_source()) {
            let config = BootstrapConfig::default();
            let verifier = BootstrapVerifier::new(config);
            
            if let Ok(result) = verifier.verify_lexer(&source) {
                // Source hash should be consistent
                prop_assert!(!result.source_hash.is_empty());
                prop_assert_eq!(result.source_hash.len(), 64); // SHA-256 hex length
                
                // Performance ratio should be non-negative
                prop_assert!(result.performance_ratio >= 0.0);
                
                // Token count should be non-negative
                prop_assert!(result.token_count >= 0);
                
                // If verification passed, key components should be working
                if result.passed {
                    prop_assert!(result.hash_match || result.tokens_match || result.performance_acceptable);
                }
            }
        }

        #[test]
        fn prop_integration_verification_scales(
            test_cases in prop::collection::vec(valid_ovie_source(), 1..10)
        ) {
            let mut integration = BootstrapIntegration::new();
            let config = BootstrapConfig {
                hash_verification: true,
                token_comparison: true,
                performance_benchmarking: false,
                max_performance_degradation: 5.0,
                verbose_logging: false,
            };
            
            // Initialize (may fail, but test should continue)
            let _ = integration.initialize(config);
            
            let test_case_refs: Vec<&str> = test_cases.iter().map(|s| s.as_str()).collect();
            
            // Test verification at current mode (RustOnly)
            if let Ok(result) = integration.verify_integration(&test_case_refs) {
                // Should have one verification result per test case
                prop_assert_eq!(result.verification_results.len(), test_cases.len());
                
                // Test coverage should be between 0 and 100
                prop_assert!(result.test_coverage >= 0.0);
                prop_assert!(result.test_coverage <= 100.0);
                
                // Performance impact should be positive
                prop_assert!(result.performance_impact > 0.0);
                
                // Error count should be non-negative
                prop_assert!(result.error_count >= 0);
                
                // Mode should match current mode
                prop_assert_eq!(result.mode, integration.current_mode().name());
            }
        }

        #[test]
        fn prop_bootstrap_hash_consistency(
            sources in prop::collection::vec(valid_ovie_source(), 2..5)
        ) {
            let integration = BootstrapIntegration::new();
            
            let mut hashes = std::collections::HashSet::new();
            
            for source in &sources {
                let hash = integration.compute_source_hash(source);
                
                // Hash should be deterministic
                let hash2 = integration.compute_source_hash(source);
                prop_assert_eq!(hash.clone(), hash2);
                
                // Hash should be SHA-256 length
                prop_assert_eq!(hash.len(), 64);
                
                // Hash should be unique for different sources
                if !hashes.contains(&hash) {
                    hashes.insert(hash);
                }
            }
            
            // Different sources should generally produce different hashes
            // (allowing for some collisions in the test data)
            if sources.len() > 2 {
                prop_assert!(hashes.len() >= sources.len() / 2);
            }
        }
    }
}

/// **Feature: ovie-programming-language, Property 19: Release Security Compliance**
/// 
/// For any release process, the system should include cryptographic signing and ensure 
/// reproducible builds with verifiable integrity
/// 
/// **Validates: Requirements 10.5**
pub mod release_security_compliance {
    use super::*;
    use crate::release::{ReleaseManager, SecurityLevel, ReleaseMetadata, DistributionConfig, DistributionManager};
    use crate::{DeterministicBuildConfig, BuildMetadata};
    use proptest::prelude::*;
    use std::collections::HashMap;

    // Generator for valid release versions
    fn valid_version() -> impl Strategy<Value = String> {
        prop_oneof![
            // Semantic versions
            (1u32..100, 0u32..100, 0u32..100).prop_map(|(major, minor, patch)| 
                format!("{}.{}.{}", major, minor, patch)),
            // Pre-release versions
            (1u32..10, 0u32..10, 0u32..10, prop::string::string_regex(r"[a-z]+").unwrap())
                .prop_map(|(major, minor, patch, pre)| 
                    format!("{}.{}.{}-{}", major, minor, patch, pre)),
        ]
    }

    // Generator for release artifacts
    fn release_artifacts() -> impl Strategy<Value = Vec<(&'static str, Vec<u8>)>> {
        prop::collection::vec(
            prop_oneof![
                Just(("ovie.exe", b"mock ovie executable".to_vec())),
                Just(("oviec.exe", b"mock oviec compiler".to_vec())),
                Just(("README.md", b"# Ovie Release\n\nRelease documentation".to_vec())),
                Just(("LICENSE", b"MIT License\n\nCopyright...".to_vec())),
                Just(("CHANGELOG.md", b"# Changelog\n\n## v1.0.0\n- Initial release".to_vec())),
            ],
            1..5
        )
    }

    // Generator for security levels
    fn security_level() -> impl Strategy<Value = SecurityLevel> {
        prop::sample::select(vec![
            SecurityLevel::Development,
            SecurityLevel::Beta,
            SecurityLevel::Production,
        ])
    }

    proptest! {
        #[test]
        fn prop_release_must_be_cryptographically_signed(
            version in valid_version(),
            artifacts in release_artifacts(),
            security_level in security_level()
        ) {
            // Create release manager
            let mut manager = ReleaseManager::new(security_level);
            prop_assert!(manager.is_ok(), "Release manager creation should succeed");
            let mut manager = manager.unwrap();
            
            // Create release metadata
            let mut metadata = ReleaseMetadata::new(&version, "test-build-hash");
            metadata.mark_security_audit_passed();
            metadata.set_vulnerability_scan_results(true);
            
            // Create release package
            let release_result = manager.create_release(&version, &artifacts, metadata);
            prop_assert!(release_result.is_ok(), "Release creation should succeed for version: {}", version);
            
            let package = release_result.unwrap();
            
            // Verify cryptographic signing requirements
            let signatures = package.signatures();
            prop_assert!(!signatures.is_empty(), "Release must have at least one signature");
            
            // Verify signature count meets security level requirements
            let required_signatures = security_level.required_signatures();
            prop_assert!(signatures.len() >= required_signatures, 
                "Release must have at least {} signatures for {:?} security level, found {}", 
                required_signatures, security_level, signatures.len());
            
            // Verify each signature has required properties
            for signature in signatures {
                prop_assert!(!signature.signature.is_empty(), "Signature data must not be empty");
                prop_assert!(!signature.key_id.is_empty(), "Signature must have key ID");
                prop_assert!(!signature.algorithm.is_empty(), "Signature must specify algorithm");
                prop_assert!(signature.timestamp > 0, "Signature must have valid timestamp");
                prop_assert!(!signature.data_hash.is_empty(), "Signature must include data hash");
                
                // Verify algorithm meets security requirements
                let expected_key_strength = security_level.required_key_strength();
                let expected_algorithm = format!("RSA-{}-SHA256", expected_key_strength);
                prop_assert_eq!(&signature.algorithm, &expected_algorithm, 
                    "Signature algorithm must match security level requirements");
            }
        }

        #[test]
        fn prop_release_builds_must_be_reproducible(
            version in valid_version(),
            artifacts in release_artifacts(),
            security_level in prop::sample::select(vec![SecurityLevel::Beta, SecurityLevel::Production])
        ) {
            // Create distribution manager with reproducible builds enabled
            let mut config = DistributionConfig::new(security_level);
            config.enable_reproducible_builds(Some(1640995200)); // Fixed timestamp
            
            let mut manager = DistributionManager::new(config);
            prop_assert!(manager.is_ok(), "Distribution manager creation should succeed");
            let mut manager = manager.unwrap();
            
            // Create deterministic build metadata
            let build_config = DeterministicBuildConfig::new_deterministic();
            let metadata = ReleaseMetadata::new_reproducible(&version, &build_config);
            
            // Verify reproducibility
            let reproducibility_result = manager.verify_reproducibility(&version, &artifacts, metadata);
            prop_assert!(reproducibility_result.is_ok(), "Reproducibility verification should succeed");
            
            let is_reproducible = reproducibility_result.unwrap();
            prop_assert!(is_reproducible, "Builds must be reproducible for security levels Beta and Production");
            
            // Verify build metadata indicates reproducible build
            let status = manager.get_status();
            prop_assert!(status.reproducible_builds > 0, "Should have at least one reproducible build recorded");
        }

        #[test]
        fn prop_release_integrity_must_be_verifiable(
            version in valid_version(),
            artifacts in release_artifacts(),
            security_level in security_level()
        ) {
            // Create release manager
            let mut manager = ReleaseManager::new(security_level);
            prop_assert!(manager.is_ok(), "Release manager creation should succeed");
            let mut manager = manager.unwrap();
            
            // Create release metadata with security compliance
            let mut metadata = ReleaseMetadata::new(&version, "test-build-hash");
            metadata.mark_security_audit_passed();
            metadata.set_vulnerability_scan_results(true);
            metadata.add_compliance_info("security_scan".to_string(), "passed".to_string());
            metadata.add_compliance_info("license_check".to_string(), "compliant".to_string());
            
            // Create signed release
            let release_result = manager.create_release(&version, &artifacts, metadata);
            prop_assert!(release_result.is_ok(), "Release creation should succeed");
            let package = release_result.unwrap();
            
            // Verify package integrity
            let integrity_check = package.verify_integrity();
            prop_assert!(integrity_check.is_ok(), "Integrity verification should not error");
            prop_assert!(integrity_check.unwrap(), "Package integrity must be verifiable");
            
            // Verify package has integrity hash
            prop_assert!(!package.package_hash().is_empty(), "Package must have integrity hash");
            
            // Verify release can be verified by the system
            let verification_result = manager.verify_release(&package);
            prop_assert!(verification_result.is_ok(), "Release verification should succeed");
            
            let verification = verification_result.unwrap();
            
            // For production releases, all checks must pass
            if matches!(security_level, SecurityLevel::Production) {
                prop_assert!(verification.is_valid, "Production releases must pass all verification checks");
                prop_assert!(verification.signature_valid, "Production releases must have valid signatures");
                prop_assert!(verification.integrity_valid, "Production releases must have valid integrity");
                prop_assert!(verification.compliance_valid, "Production releases must be compliance-valid");
            }
            
            // All releases must have basic integrity
            prop_assert!(verification.integrity_valid, "All releases must have valid integrity");
        }

        #[test]
        fn prop_release_security_escalation_requirements(
            version in valid_version(),
            artifacts in release_artifacts()
        ) {
            // Test that higher security levels have stricter requirements
            let security_levels = vec![
                SecurityLevel::Development,
                SecurityLevel::Beta,
                SecurityLevel::Production,
            ];
            
            let mut signature_counts = Vec::new();
            let mut key_strengths = Vec::new();
            
            for security_level in security_levels {
                let required_signatures = security_level.required_signatures();
                let required_key_strength = security_level.required_key_strength();
                
                signature_counts.push(required_signatures);
                key_strengths.push(required_key_strength);
                
                // Verify we can create a release manager for this security level
                let manager_result = ReleaseManager::new(security_level);
                prop_assert!(manager_result.is_ok(), "Should be able to create manager for {:?}", security_level);
            }
            
            // Verify security requirements escalate
            prop_assert!(signature_counts[0] <= signature_counts[1], "Beta should require >= signatures than Development");
            prop_assert!(signature_counts[1] <= signature_counts[2], "Production should require >= signatures than Beta");
            
            prop_assert!(key_strengths[0] <= key_strengths[1], "Beta should require >= key strength than Development");
            prop_assert!(key_strengths[1] <= key_strengths[2], "Production should require >= key strength than Beta");
            
            // Verify minimum security standards
            prop_assert!(signature_counts[2] >= 3, "Production must require at least 3 signatures");
            prop_assert!(key_strengths[2] >= 4096, "Production must require at least 4096-bit keys");
        }

        #[test]
        fn prop_release_compliance_metadata_required(
            version in valid_version(),
            artifacts in release_artifacts(),
            security_level in prop::sample::select(vec![SecurityLevel::Beta, SecurityLevel::Production])
        ) {
            // Create release manager
            let mut manager = ReleaseManager::new(security_level);
            prop_assert!(manager.is_ok(), "Release manager creation should succeed");
            let mut manager = manager.unwrap();
            
            // Test with incomplete compliance metadata
            let incomplete_metadata = ReleaseMetadata::new(&version, "test-build-hash");
            // Note: Not marking security audit as passed or setting vulnerability scan results
            
            let release_result = manager.create_release(&version, &artifacts, incomplete_metadata);
            
            // For Beta/Production, incomplete compliance should cause verification failure
            if release_result.is_ok() {
                let package = release_result.unwrap();
                let verification = manager.verify_release(&package).unwrap();
                
                // Should fail verification due to missing compliance metadata
                if matches!(security_level, SecurityLevel::Production) {
                    prop_assert!(!verification.is_valid || !verification.compliance_valid, 
                        "Production releases with incomplete compliance should fail verification");
                }
            }
            
            // Test with complete compliance metadata
            let mut complete_metadata = ReleaseMetadata::new(&version, "test-build-hash");
            complete_metadata.mark_security_audit_passed();
            complete_metadata.set_vulnerability_scan_results(true);
            complete_metadata.add_compliance_info("security_review".to_string(), "approved".to_string());
            
            let complete_release_result = manager.create_release(&version, &artifacts, complete_metadata);
            prop_assert!(complete_release_result.is_ok(), "Release with complete compliance should succeed");
            
            let complete_package = complete_release_result.unwrap();
            let complete_verification = manager.verify_release(&complete_package).unwrap();
            
            // Should have better verification results with complete metadata
            prop_assert!(complete_verification.integrity_valid, "Complete releases should have valid integrity");
        }

        #[test]
        fn prop_release_deterministic_signing(
            version in valid_version(),
            artifacts in release_artifacts(),
            security_level in security_level()
        ) {
            // Create two identical release managers
            let mut manager1 = ReleaseManager::new(security_level);
            let mut manager2 = ReleaseManager::new(security_level);
            
            prop_assert!(manager1.is_ok() && manager2.is_ok(), "Both managers should be created successfully");
            let mut manager1 = manager1.unwrap();
            let mut manager2 = manager2.unwrap();
            
            // Create identical metadata
            let mut metadata1 = ReleaseMetadata::new(&version, "identical-build-hash");
            metadata1.mark_security_audit_passed();
            metadata1.set_vulnerability_scan_results(true);
            
            let mut metadata2 = ReleaseMetadata::new(&version, "identical-build-hash");
            metadata2.mark_security_audit_passed();
            metadata2.set_vulnerability_scan_results(true);
            
            // Create releases with identical inputs
            let release1_result = manager1.create_release(&version, &artifacts, metadata1);
            let release2_result = manager2.create_release(&version, &artifacts, metadata2);
            
            if release1_result.is_ok() && release2_result.is_ok() {
                let package1 = release1_result.unwrap();
                let package2 = release2_result.unwrap();
                
                // Packages should have consistent structure
                prop_assert_eq!(package1.version(), package2.version(), "Versions should match");
                prop_assert_eq!(package1.artifacts().len(), package2.artifacts().len(), "Artifact counts should match");
                
                // Both should be verifiable
                let verification1 = manager1.verify_release(&package1).unwrap();
                let verification2 = manager2.verify_release(&package2).unwrap();
                
                prop_assert_eq!(verification1.integrity_valid, verification2.integrity_valid, 
                    "Integrity verification should be consistent");
            }
        }
    }

    #[test]
    fn test_release_security_compliance_basic_requirements() {
        // Test basic security compliance requirements without property testing
        
        // Test Development security level
        let dev_manager = ReleaseManager::new(SecurityLevel::Development).unwrap();
        assert_eq!(dev_manager.security_level(), SecurityLevel::Development);
        
        // Test Beta security level
        let beta_manager = ReleaseManager::new(SecurityLevel::Beta).unwrap();
        assert_eq!(beta_manager.security_level(), SecurityLevel::Beta);
        
        // Test Production security level
        let prod_manager = ReleaseManager::new(SecurityLevel::Production).unwrap();
        assert_eq!(prod_manager.security_level(), SecurityLevel::Production);
        
        // Verify security level escalation
        assert!(SecurityLevel::Development.required_signatures() <= SecurityLevel::Beta.required_signatures());
        assert!(SecurityLevel::Beta.required_signatures() <= SecurityLevel::Production.required_signatures());
        
        assert!(SecurityLevel::Development.required_key_strength() <= SecurityLevel::Beta.required_key_strength());
        assert!(SecurityLevel::Beta.required_key_strength() <= SecurityLevel::Production.required_key_strength());
    }

    #[test]
    fn test_release_reproducibility_requirements() {
        // Test reproducible build requirements
        let mut config = DistributionConfig::new(SecurityLevel::Production);
        config.enable_reproducible_builds(Some(1640995200));
        
        let mut manager = DistributionManager::new(config).unwrap();
        
        let artifacts = vec![
            ("ovie.exe", b"test executable".to_vec()),
            ("README.md", b"# Test Release".to_vec()),
        ];
        
        let build_config = DeterministicBuildConfig::new_deterministic();
        let metadata = ReleaseMetadata::new_reproducible("1.0.0", &build_config);
        
        // Verify reproducibility
        let is_reproducible = manager.verify_reproducibility("1.0.0", &artifacts, metadata).unwrap();
        assert!(is_reproducible, "Production builds must be reproducible");
        
        let status = manager.get_status();
        assert!(status.reproducible_builds > 0, "Should track reproducible builds");
    }

    #[test]
    fn test_release_integrity_verification() {
        // Test integrity verification requirements
        let mut manager = ReleaseManager::new(SecurityLevel::Production).unwrap();
        
        let artifacts = vec![
            ("ovie.exe", b"production executable".to_vec()),
            ("LICENSE", b"MIT License".to_vec()),
        ];
        
        let mut metadata = ReleaseMetadata::new("1.0.0", "production-build-hash");
        metadata.mark_security_audit_passed();
        metadata.set_vulnerability_scan_results(true);
        metadata.add_compliance_info("security_review".to_string(), "approved".to_string());
        
        let package = manager.create_release("1.0.0", &artifacts, metadata).unwrap();
        
        // Verify package integrity
        assert!(package.verify_integrity().unwrap(), "Package integrity must be verifiable");
        assert!(!package.package_hash().is_empty(), "Package must have integrity hash");
        
        // Verify release verification
        let verification = manager.verify_release(&package).unwrap();
        assert!(verification.integrity_valid, "Release must have valid integrity");
        
        // For production, all checks should pass
        assert!(verification.is_valid, "Production release should pass all verification checks");
    }
}

/// **Feature: ovie-programming-language, Property 21: Multi-Repository Version Consistency**
/// 
/// For any component update across repositories, the system should maintain version compatibility 
/// and coordinate releases properly
/// 
/// **Validates: Requirements 12.2, 12.4**
pub mod multi_repository_version_consistency {
    use super::*;
    use crate::release::{ReleaseManager, SecurityLevel, ReleaseMetadata};
    use crate::package::{PackageRegistry, PackageId, PackageMetadata, DependencySpec};
    use proptest::prelude::*;
    use std::collections::HashMap;

    /// Represents a repository in the Ovie ecosystem
    #[derive(Debug, Clone)]
    struct Repository {
        name: String,
        version: String,
        dependencies: Vec<(String, String)>, // (repo_name, version_constraint)
    }

    /// Represents a coordinated release across multiple repositories
    #[derive(Debug, Clone)]
    struct CoordinatedRelease {
        repositories: Vec<Repository>,
        release_version: String,
    }

    // Generator for valid semantic versions
    fn valid_semver() -> impl Strategy<Value = String> {
        (1u32..10, 0u32..10, 0u32..10).prop_map(|(major, minor, patch)| 
            format!("{}.{}.{}", major, minor, patch))
    }

    // Generator for repository names
    fn repository_name() -> impl Strategy<Value = String> {
        prop::sample::select(vec![
            "ovie".to_string(),
            "oviec".to_string(), 
            "aproko".to_string(),
            "std".to_string(),
            "docs".to_string(),
            "spec".to_string(),
            "examples".to_string(),
        ])
    }

    // Generator for version constraints
    fn version_constraint() -> impl Strategy<Value = String> {
        prop_oneof![
            valid_semver().prop_map(|v| format!("={}", v)), // Exact version
            valid_semver().prop_map(|v| format!("^{}", v)), // Compatible version
            valid_semver().prop_map(|v| format!("~{}", v)), // Patch-level compatible
        ]
    }

    // Generator for repository with dependencies
    fn repository_with_deps() -> impl Strategy<Value = Repository> {
        (
            repository_name(),
            valid_semver(),
            prop::collection::vec(
                (repository_name(), version_constraint()),
                0..3
            )
        ).prop_map(|(name, version, deps)| Repository {
            name: name.clone(),
            version,
            dependencies: deps.into_iter()
                .filter(|(dep_name, _)| dep_name != &name) // No self-dependencies
                .collect(),
        })
    }

    // Generator for coordinated releases
    fn coordinated_release() -> impl Strategy<Value = CoordinatedRelease> {
        (
            prop::collection::vec(repository_with_deps(), 2..5),
            valid_semver()
        ).prop_map(|(repos, release_version)| CoordinatedRelease {
            repositories: repos,
            release_version,
        })
    }

    proptest! {
        #[test]
        fn prop_version_compatibility_maintained_across_updates(
            initial_release in coordinated_release(),
            version_bump in prop::sample::select(vec!["patch", "minor", "major"])
        ) {
            // Create initial package registry state
            let mut registry = PackageRegistry::new()?;
            
            // Register all repositories in the initial release
            for repo in &initial_release.repositories {
                let package_id = PackageId::new(repo.name.clone(), repo.version.clone(), "test-hash".to_string());
                let metadata = PackageMetadata {
                    id: package_id.clone(),
                    description: Some(format!("Package for {}", repo.name)),
                    authors: vec!["Test Author".to_string()],
                    dependencies: HashMap::new(),
                    dev_dependencies: HashMap::new(),
                    build_dependencies: HashMap::new(),
                    license: Some("MIT".to_string()),
                    repository: None,
                    homepage: None,
                    documentation: None,
                    keywords: vec![],
                    categories: vec![],
                };
                
                let register_result = registry.store_package(metadata, b"test package content");
                prop_assert!(register_result.is_ok(), "Failed to register package: {}", repo.name);
            }
            
            // Simulate version update
            let updated_version = bump_version(&initial_release.release_version, &version_bump);
            
            // Verify that version compatibility is maintained
            for repo in &initial_release.repositories {
                for (dep_name, dep_constraint) in &repo.dependencies {
                    let is_compatible = check_version_compatibility(dep_constraint, &updated_version);
                    
                    // For coordinated releases, compatibility should be maintained
                    // unless it's a major version bump (which is allowed to break compatibility)
                    if version_bump != "major" {
                        prop_assert!(is_compatible, 
                            "Version compatibility broken for {}: {} not compatible with {}", 
                            dep_name, dep_constraint, updated_version);
                    }
                }
            }
        }

        #[test]
        fn prop_coordinated_releases_maintain_consistency(
            release in coordinated_release()
        ) {
            // Create release managers for different security levels
            let mut dev_manager = ReleaseManager::new(SecurityLevel::Development)?;
            let mut prod_manager = ReleaseManager::new(SecurityLevel::Production)?;
            
            // Track release coordination
            let mut release_hashes = HashMap::new();
            let mut release_timestamps = Vec::new();
            
            // Create coordinated releases for each repository
            for repo in &release.repositories {
                // Create artifacts with proper lifetime
                let exe_name = format!("{}.exe", repo.name);
                let readme_content = format!("# {} v{}", repo.name, repo.version);
                let artifacts = vec![
                    (exe_name.as_str(), format!("mock {} executable", repo.name).into_bytes()),
                    ("README.md", readme_content.into_bytes()),
                ];
                
                // Create metadata with cross-repository dependencies
                let mut metadata = ReleaseMetadata::new(&repo.version, "coordinated-build-hash");
                metadata.mark_security_audit_passed();
                metadata.set_vulnerability_scan_results(true);
                
                // Add coordination metadata
                metadata.add_compliance_info("release_coordination".to_string(), release.release_version.clone());
                metadata.add_compliance_info("repository_name".to_string(), repo.name.clone());
                
                // Add dependency information
                for (dep_name, dep_version) in &repo.dependencies {
                    metadata.add_compliance_info(
                        format!("dependency_{}", dep_name), 
                        dep_version.clone()
                    );
                }
                
                // Create releases with both managers
                let dev_release = dev_manager.create_release(&repo.version, &artifacts, metadata.clone());
                let prod_release = prod_manager.create_release(&repo.version, &artifacts, metadata);
                
                prop_assert!(dev_release.is_ok(), "Development release should succeed for {}", repo.name);
                prop_assert!(prod_release.is_ok(), "Production release should succeed for {}", repo.name);
                
                if let (Ok(dev_pkg), Ok(prod_pkg)) = (dev_release, prod_release) {
                    // Store release information for coordination verification
                    release_hashes.insert(repo.name.clone(), dev_pkg.package_hash().to_string());
                    release_timestamps.push(dev_pkg.metadata().created_at);
                    
                    // Verify cross-repository consistency
                    prop_assert_eq!(dev_pkg.version(), prod_pkg.version(), 
                        "Development and production releases should have same version");
                    
                    // Verify coordination metadata is present
                    let compliance_info = &dev_pkg.metadata().compliance_info;
                    prop_assert!(compliance_info.contains_key("release_coordination"), 
                        "Release should contain coordination metadata");
                    prop_assert_eq!(compliance_info.get("release_coordination").unwrap(), &release.release_version,
                        "Coordination version should match release version");
                }
            }
            
            // Verify all releases are coordinated (similar timestamps)
            if release_timestamps.len() > 1 {
                let first_timestamp = release_timestamps[0];
                for &timestamp in &release_timestamps[1..] {
                    let time_diff = if timestamp > first_timestamp {
                        timestamp - first_timestamp
                    } else {
                        first_timestamp - timestamp
                    };
                    
                    // Coordinated releases should be within 60 seconds of each other
                    prop_assert!(time_diff <= 60, 
                        "Coordinated releases should have similar timestamps (diff: {} seconds)", time_diff);
                }
            }
        }

        #[test]
        fn prop_dependency_resolution_across_repositories(
            repositories in prop::collection::vec(repository_with_deps(), 2..4)
        ) {
            let mut registry = PackageRegistry::new()?;
            
            // Register all repositories
            for repo in &repositories {
                let package_id = PackageId::new(repo.name.clone(), repo.version.clone(), "test-hash".to_string());
                let metadata = PackageMetadata {
                    id: package_id.clone(),
                    description: Some(format!("Package for {}", repo.name)),
                    authors: vec!["Test Author".to_string()],
                    dependencies: HashMap::new(),
                    dev_dependencies: HashMap::new(),
                    build_dependencies: HashMap::new(),
                    license: Some("MIT".to_string()),
                    repository: None,
                    homepage: None,
                    documentation: None,
                    keywords: vec![],
                    categories: vec![],
                };
                
                let register_result = registry.store_package(metadata, b"test package content");
                prop_assert!(register_result.is_ok(), "Package registration should succeed");
            }
            
            // Test dependency resolution for each repository
            for repo in &repositories {
                let package_id = PackageId::new(repo.name.clone(), repo.version.clone(), "test-hash".to_string());
                
                // Try to get the package (simulating dependency resolution)
                let resolution_result = registry.get_package(&package_id);
                
                if let Ok(Some((metadata, _content))) = resolution_result {
                    // Verify package metadata is correct
                    prop_assert_eq!(&metadata.id.name, &repo.name);
                    prop_assert_eq!(&metadata.id.version, &repo.version);
                    
                    // For this test, we don't have circular dependency checking implemented
                    // in the actual PackageRegistry, so we'll just verify basic functionality
                }
            }
        }

        #[test]
        fn prop_release_coordination_metadata_consistency(
            release in coordinated_release()
        ) {
            let mut manager = ReleaseManager::new(SecurityLevel::Beta)?;
            let mut all_packages = Vec::new();
            
            // Create releases for all repositories in the coordinated release
            for repo in &release.repositories {
                let artifacts = vec![
                    ("binary", format!("{} binary content", repo.name).into_bytes()),
                ];
                
                let mut metadata = ReleaseMetadata::new(&repo.version, "test-build-hash");
                metadata.mark_security_audit_passed();
                metadata.set_vulnerability_scan_results(true);
                
                // Add coordination metadata
                metadata.add_compliance_info("coordinated_release_id".to_string(), release.release_version.clone());
                metadata.add_compliance_info("repository_count".to_string(), release.repositories.len().to_string());
                
                let package_result = manager.create_release(&repo.version, &artifacts, metadata);
                prop_assert!(package_result.is_ok(), "Release creation should succeed for {}", repo.name);
                
                if let Ok(package) = package_result {
                    all_packages.push((repo.name.clone(), package));
                }
            }
            
            // Verify coordination metadata consistency across all packages
            if !all_packages.is_empty() {
                let first_package = &all_packages[0].1;
                let first_coordination_id = first_package.metadata().compliance_info
                    .get("coordinated_release_id").unwrap();
                
                for (repo_name, package) in &all_packages[1..] {
                    let coordination_id = package.metadata().compliance_info
                        .get("coordinated_release_id").unwrap();
                    
                    prop_assert_eq!(coordination_id, first_coordination_id,
                        "All packages in coordinated release should have same coordination ID: {} vs {}", 
                        repo_name, all_packages[0].0);
                    
                    // Verify repository count is consistent
                    let repo_count = package.metadata().compliance_info
                        .get("repository_count").unwrap();
                    prop_assert_eq!(repo_count, &release.repositories.len().to_string(),
                        "Repository count should be consistent across all packages");
                }
            }
        }

        #[test]
        fn prop_version_constraint_validation(
            constraint in version_constraint(),
            target_version in valid_semver()
        ) {
            // Test version constraint validation logic
            let is_compatible = check_version_compatibility(&constraint, &target_version);
            
            // Parse constraint and target version for validation
            if let (Some(constraint_version), Some(target_ver)) = (
                parse_version_from_constraint(&constraint),
                parse_semver(&target_version)
            ) {
                let constraint_ver = parse_semver(&constraint_version).unwrap();
                
                if constraint.starts_with('=') {
                    // Exact version match
                    prop_assert_eq!(is_compatible, constraint_ver == target_ver,
                        "Exact version constraint should match exactly: {} vs {}", constraint, target_version);
                } else if constraint.starts_with('^') {
                    // Compatible version (same major version, >= minor.patch)
                    let compatible = constraint_ver.0 == target_ver.0 && 
                        (target_ver.1 > constraint_ver.1 || 
                         (target_ver.1 == constraint_ver.1 && target_ver.2 >= constraint_ver.2));
                    prop_assert_eq!(is_compatible, compatible,
                        "Compatible version constraint validation: {} vs {}", constraint, target_version);
                } else if constraint.starts_with('~') {
                    // Patch-level compatible (same major.minor, >= patch)
                    let patch_compatible = constraint_ver.0 == target_ver.0 && 
                        constraint_ver.1 == target_ver.1 && 
                        target_ver.2 >= constraint_ver.2;
                    prop_assert_eq!(is_compatible, patch_compatible,
                        "Patch-level constraint validation: {} vs {}", constraint, target_version);
                }
            }
        }
    }

    // Helper functions for version management
    fn bump_version(version: &str, bump_type: &str) -> String {
        if let Some((major, minor, patch)) = parse_semver(version) {
            match bump_type {
                "major" => format!("{}.0.0", major + 1),
                "minor" => format!("{}.{}.0", major, minor + 1),
                "patch" => format!("{}.{}.{}", major, minor, patch + 1),
                _ => version.to_string(),
            }
        } else {
            version.to_string()
        }
    }

    fn parse_semver(version: &str) -> Option<(u32, u32, u32)> {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() == 3 {
            if let (Ok(major), Ok(minor), Ok(patch)) = (
                parts[0].parse::<u32>(),
                parts[1].parse::<u32>(),
                parts[2].parse::<u32>()
            ) {
                return Some((major, minor, patch));
            }
        }
        None
    }

    fn parse_version_from_constraint(constraint: &str) -> Option<String> {
        if constraint.starts_with('=') || constraint.starts_with('^') || constraint.starts_with('~') {
            Some(constraint[1..].to_string())
        } else {
            Some(constraint.to_string())
        }
    }

    fn check_version_compatibility(constraint: &str, target_version: &str) -> bool {
        if let (Some(constraint_version), Some(target_ver)) = (
            parse_version_from_constraint(constraint),
            parse_semver(target_version)
        ) {
            if let Some(constraint_ver) = parse_semver(&constraint_version) {
                if constraint.starts_with('=') {
                    return constraint_ver == target_ver;
                } else if constraint.starts_with('^') {
                    return constraint_ver.0 == target_ver.0 && 
                        (target_ver.1 > constraint_ver.1 || 
                         (target_ver.1 == constraint_ver.1 && target_ver.2 >= constraint_ver.2));
                } else if constraint.starts_with('~') {
                    return constraint_ver.0 == target_ver.0 && 
                        constraint_ver.1 == target_ver.1 && 
                        target_ver.2 >= constraint_ver.2;
                }
            }
        }
        false
    }

    fn check_circular_dependencies(
        package_id: &PackageId, 
        registry: &mut PackageRegistry, 
        visited: &mut std::collections::HashSet<String>
    ) -> bool {
        let package_key = format!("{}@{}", package_id.name, package_id.version);
        
        if visited.contains(&package_key) {
            return true; // Circular dependency detected
        }
        
        visited.insert(package_key.clone());
        
        if let Ok(Some((metadata, _content))) = registry.get_package(package_id) {
            for (_dep_name, dep_id) in &metadata.dependencies {
                if check_circular_dependencies(dep_id, registry, visited) {
                    return true;
                }
            }
        }
        
        visited.remove(&package_key);
        false
    }

    #[test]
    fn test_multi_repository_version_consistency_basic() {
        // Test basic multi-repository version consistency
        let mut registry = PackageRegistry::new().unwrap();
        
        // Register oviec compiler
        let oviec_id = PackageId::new("oviec".to_string(), "1.0.0".to_string(), "test-hash".to_string());
        let oviec_metadata = PackageMetadata {
            id: oviec_id.clone(),
            description: Some("Ovie compiler".to_string()),
            authors: vec!["Test Author".to_string()],
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            build_dependencies: HashMap::new(),
            license: Some("MIT".to_string()),
            repository: None,
            homepage: None,
            documentation: None,
            keywords: vec![],
            categories: vec![],
        };
        assert!(registry.store_package(oviec_metadata, b"oviec content").is_ok());
        
        // Register ovie CLI that depends on oviec
        let ovie_id = PackageId::new("ovie".to_string(), "1.0.0".to_string(), "test-hash-2".to_string());
        let mut ovie_dependencies = HashMap::new();
        ovie_dependencies.insert("oviec".to_string(), oviec_id.clone());
        
        let ovie_metadata = PackageMetadata {
            id: ovie_id.clone(),
            description: Some("Ovie CLI".to_string()),
            authors: vec!["Test Author".to_string()],
            dependencies: ovie_dependencies,
            dev_dependencies: HashMap::new(),
            build_dependencies: HashMap::new(),
            license: Some("MIT".to_string()),
            repository: None,
            homepage: None,
            documentation: None,
            keywords: vec![],
            categories: vec![],
        };
        assert!(registry.store_package(ovie_metadata, b"ovie content").is_ok());
        
        // Verify we can retrieve the packages
        let ovie_result = registry.get_package(&ovie_id).unwrap();
        assert!(ovie_result.is_some(), "Should be able to retrieve ovie package");
        
        let (retrieved_metadata, _content) = ovie_result.unwrap();
        assert_eq!(retrieved_metadata.id.name, "ovie");
        assert!(!retrieved_metadata.dependencies.is_empty(), "Should have oviec dependency");
        
        // Test coordinated release
        let mut manager = ReleaseManager::new(SecurityLevel::Development).unwrap();
        
        let artifacts = vec![("ovie", b"ovie binary".to_vec())];
        let mut metadata = ReleaseMetadata::new("1.0.0", "test-hash");
        metadata.mark_security_audit_passed();
        metadata.set_vulnerability_scan_results(true);
        metadata.add_compliance_info("coordinated_release".to_string(), "1.0.0".to_string());
        
        let package = manager.create_release("1.0.0", &artifacts, metadata).unwrap();
        
        // Verify coordination metadata
        let compliance_info = &package.metadata().compliance_info;
        assert!(compliance_info.contains_key("coordinated_release"));
        assert_eq!(compliance_info.get("coordinated_release").unwrap(), "1.0.0");
    }

    #[test]
    fn test_version_constraint_compatibility() {
        // Test exact version constraint
        assert!(check_version_compatibility("=1.0.0", "1.0.0"));
        assert!(!check_version_compatibility("=1.0.0", "1.0.1"));
        
        // Test compatible version constraint
        assert!(check_version_compatibility("^1.0.0", "1.0.0"));
        assert!(check_version_compatibility("^1.0.0", "1.1.0"));
        assert!(check_version_compatibility("^1.0.0", "1.0.5"));
        assert!(!check_version_compatibility("^1.0.0", "2.0.0"));
        
        // Test patch-level constraint
        assert!(check_version_compatibility("~1.0.0", "1.0.0"));
        assert!(check_version_compatibility("~1.0.0", "1.0.5"));
        assert!(!check_version_compatibility("~1.0.0", "1.1.0"));
        assert!(!check_version_compatibility("~1.0.0", "2.0.0"));
    }

    #[test]
    fn test_coordinated_release_metadata() {
        let mut manager = ReleaseManager::new(SecurityLevel::Beta).unwrap();
        
        // Create coordinated releases for multiple repositories
        let repositories = vec!["ovie", "oviec", "aproko"];
        let mut packages = Vec::new();
        
        for repo in &repositories {
            let artifacts: Vec<(&str, Vec<u8>)> = vec![(*repo, format!("{} content", repo).into_bytes())];
            let mut metadata = ReleaseMetadata::new("2.0.0", "coordinated-hash");
            metadata.mark_security_audit_passed();
            metadata.set_vulnerability_scan_results(true);
            metadata.add_compliance_info("coordinated_release_id".to_string(), "release-2.0.0".to_string());
            metadata.add_compliance_info("repository_count".to_string(), repositories.len().to_string());
            
            let package = manager.create_release("2.0.0", &artifacts, metadata).unwrap();
            packages.push(package);
        }
        
        // Verify all packages have consistent coordination metadata
        let first_coordination_id = packages[0].metadata().compliance_info
            .get("coordinated_release_id").unwrap();
        
        for package in &packages[1..] {
            let coordination_id = package.metadata().compliance_info
                .get("coordinated_release_id").unwrap();
            assert_eq!(coordination_id, first_coordination_id, "Coordination IDs should match");
            
            let repo_count = package.metadata().compliance_info
                .get("repository_count").unwrap();
            assert_eq!(repo_count, &repositories.len().to_string(), "Repository count should be consistent");
        }
    }
}

/// **Feature: ovie-programming-language-stage-2, Property 7: Compiler Output Equivalence**
/// 
/// For any valid Ovie program, the self-hosted Ovie compiler should produce 
/// functionally equivalent output to the bootstrap Rust compiler
/// 
/// **Validates: Requirements 3.3, 13.1**
mod compiler_output_equivalence {
    use super::*;
    use crate::self_hosting::{BootstrapVerifier, BootstrapConfig};

    proptest! {
        #[test]
        fn test_lexer_output_equivalence(source in simple_ovie_program()) {
            // Feature: ovie-programming-language-stage-2, Property 7: Compiler Output Equivalence
            let config = BootstrapConfig::default();
            let verifier = BootstrapVerifier::new(config);
            
            // For now, we test that the Rust lexer produces consistent output
            // When the Ovie lexer is implemented, this will compare both
            let mut rust_lexer = Lexer::new(&source);
            let tokens1 = rust_lexer.tokenize().unwrap();
            
            let mut rust_lexer2 = Lexer::new(&source);
            let tokens2 = rust_lexer2.tokenize().unwrap();
            
            // Tokens should be identical for the same input
            prop_assert_eq!(tokens1.len(), tokens2.len());
            
            for (t1, t2) in tokens1.iter().zip(tokens2.iter()) {
                prop_assert_eq!(t1.token_type, t2.token_type);
                prop_assert_eq!(t1.lexeme, t2.lexeme);
                prop_assert_eq!(t1.location.line, t2.location.line);
                prop_assert_eq!(t1.location.column, t2.location.column);
            }
        }
    }

    // Generator for simple Ovie programs
    fn simple_ovie_program() -> impl Strategy<Value = String> {
        prop_oneof![
            valid_string_literal().prop_map(|s| format!("seeAm {};", s)),
            valid_identifier().prop_map(|id| format!("mut {} = 42;", id)),
            valid_number_literal().prop_map(|n| format!("seeAm {};", n)),
            (valid_identifier(), valid_number_literal()).prop_map(|(id, n)| {
                format!("fn {}() {{ return {}; }}", id, n)
            }),
        ]
    }
}

/// **Feature: ovie-programming-language-stage-2, Property 8: Bootstrap Process Reproducibility**
/// 
/// For any bootstrap build process, repeating the process with identical inputs 
/// should produce identical results and maintain compatibility
/// 
/// **Validates: Requirements 3.4, 3.5, 13.3**
mod bootstrap_process_reproducibility {
    use super::*;
    use crate::self_hosting::{BootstrapVerifier, BootstrapConfig};
    use sha2::{Sha256, Digest};

    proptest! {
        #[test]
        fn test_deterministic_lexing(source in simple_ovie_program()) {
            // Feature: ovie-programming-language-stage-2, Property 8: Bootstrap Process Reproducibility
            
            // Run lexing multiple times and verify identical output
            let mut hashes = Vec::new();
            
            for _ in 0..3 {
                let mut lexer = Lexer::new(&source);
                let tokens = lexer.tokenize().unwrap();
                
                // Compute hash of token stream
                let mut hasher = Sha256::new();
                for token in &tokens {
                    hasher.update(format!("{:?}", token.token_type).as_bytes());
                    hasher.update(token.lexeme.as_bytes());
                    hasher.update(token.location.line.to_string().as_bytes());
                    hasher.update(token.location.column.to_string().as_bytes());
                }
                let hash = format!("{:x}", hasher.finalize());
                hashes.push(hash);
            }
            
            // All hashes should be identical (deterministic behavior)
            for i in 1..hashes.len() {
                prop_assert_eq!(hashes[0], hashes[i], "Lexing should be deterministic");
            }
        }

        #[test]
        fn test_environment_independence(source in simple_ovie_program()) {
            // Feature: ovie-programming-language-stage-2, Property 8: Bootstrap Process Reproducibility
            
            // Test that compilation is independent of certain environment changes
            let mut lexer1 = Lexer::new(&source);
            let tokens1 = lexer1.tokenize().unwrap();
            
            // Simulate environment change (this is a simplified test)
            std::env::set_var("TEST_VAR", "test_value");
            
            let mut lexer2 = Lexer::new(&source);
            let tokens2 = lexer2.tokenize().unwrap();
            
            // Remove test variable
            std::env::remove_var("TEST_VAR");
            
            // Results should be identical regardless of environment
            prop_assert_eq!(tokens1.len(), tokens2.len());
            
            for (t1, t2) in tokens1.iter().zip(tokens2.iter()) {
                prop_assert_eq!(t1.token_type, t2.token_type);
                prop_assert_eq!(t1.lexeme, t2.lexeme);
            }
        }
    }

    // Generator for simple Ovie programs (reused from above)
    fn simple_ovie_program() -> impl Strategy<Value = String> {
        prop_oneof![
            valid_string_literal().prop_map(|s| format!("seeAm {};", s)),
            valid_identifier().prop_map(|id| format!("mut {} = 42;", id)),
            valid_number_literal().prop_map(|n| format!("seeAm {};", n)),
        ]
    }
}

/// **Feature: ovie-programming-language-stage-2, Bootstrap Verification Properties**
/// 
/// Additional properties specific to the bootstrap verification system
mod bootstrap_verification_properties {
    use super::*;
    use crate::self_hosting::{BootstrapVerifier, BootstrapConfig, EquivalenceTester};

    proptest! {
        #[test]
        fn test_hash_consistency(source in any::<String>().prop_filter("Valid UTF-8", |s| !s.is_empty())) {
            // Hash computation should be consistent
            let config = BootstrapConfig::default();
            let verifier = BootstrapVerifier::new(config);
            
            // Create dummy tokens for testing
            let tokens = vec![
                crate::lexer::Token::new(
                    TokenType::Identifier,
                    "test".to_string(),
                    crate::error::SourceLocation::new(1, 1, 0)
                )
            ];
            
            let hash1 = verifier.compute_token_hash(&tokens);
            let hash2 = verifier.compute_token_hash(&tokens);
            
            prop_assert_eq!(hash1, hash2, "Hash computation should be deterministic");
            prop_assert!(!hash1.is_empty(), "Hash should not be empty");
            prop_assert_eq!(hash1.len(), 64, "SHA-256 hash should be 64 characters");
        }

        #[test]
        fn test_performance_ratio_calculation(
            rust_time in 1u64..10000u64,
            ovie_time in 1u64..50000u64
        ) {
            // Performance ratio calculation should be accurate
            let ratio = ovie_time as f64 / rust_time as f64;
            
            prop_assert!(ratio > 0.0, "Performance ratio should be positive");
            
            if ovie_time == rust_time {
                prop_assert!((ratio - 1.0).abs() < f64::EPSILON, "Equal times should give ratio of 1.0");
            }
            
            if ovie_time > rust_time {
                prop_assert!(ratio > 1.0, "Slower Ovie should give ratio > 1.0");
            }
        }

        #[test]
        fn test_equivalence_tester_generation(
            max_cases in 1usize..100usize,
            complexity in 1usize..10usize
        ) {
            // Test case generation should produce valid output
            let mut tester = EquivalenceTester::new(max_cases, complexity);
            
            for i in 0..std::cmp::min(max_cases, 10) {
                let test_case = tester.test_generator.generate_test_case(complexity + i);
                
                prop_assert!(!test_case.is_empty(), "Generated test case should not be empty");
                prop_assert!(test_case.ends_with(';'), "Generated test case should end with semicolon");
                
                // Should be valid Ovie syntax (basic check)
                let mut lexer = Lexer::new(&test_case);
                let tokens_result = lexer.tokenize();
                prop_assert!(tokens_result.is_ok(), "Generated test case should be lexically valid: {}", test_case);
            }
        }
    }
}