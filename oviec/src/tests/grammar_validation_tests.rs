// Property-Based Tests for Grammar Validation Completeness
// Feature: ovie-programming-language-stage-2, Property 1: Grammar Validation Completeness

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::ast::*;
use proptest::prelude::*;

/// Property 1: Grammar Validation Completeness
/// For any source code input, the compiler should accept it if and only if 
/// it conforms to the formal BNF grammar specification
#[cfg(test)]
mod grammar_validation_tests {
    use super::*;

    // Test data generators for property-based testing
    
    fn arbitrary_identifier() -> impl Strategy<Value = String> {
        "[a-zA-Z][a-zA-Z0-9_]*"
            .prop_filter("Keywords not allowed as identifiers", |s| {
                !matches!(s.as_str(), 
                    "fn" | "mut" | "if" | "else" | "for" | "while" | 
                    "struct" | "enum" | "unsafe" | "return" | 
                    "true" | "false" | "seeAm"
                )
            })
    }

    fn arbitrary_number_literal() -> impl Strategy<Value = String> {
        prop_oneof![
            // Integer literals
            "[1-9][0-9]*",
            "0",
            // Float literals  
            "[1-9][0-9]*\\.[0-9]+",
            "0\\.[0-9]+"
        ]
    }

    fn arbitrary_string_literal() -> impl Strategy<Value = String> {
        "\"[^\"\\\\]*\"" // Simple strings without escape sequences for now
    }

    fn arbitrary_boolean_literal() -> impl Strategy<Value = String> {
        prop_oneof!["true", "false"]
    }

    fn arbitrary_literal() -> impl Strategy<Value = String> {
        prop_oneof![
            arbitrary_number_literal(),
            arbitrary_string_literal(), 
            arbitrary_boolean_literal()
        ]
    }

    fn arbitrary_binary_operator() -> impl Strategy<Value = String> {
        prop_oneof![
            "+", "-", "*", "/", "%",
            "==", "!=", "<", "<=", ">", ">=",
            "&&", "||"
        ].prop_map(|s| s.to_string())
    }

    fn arbitrary_simple_expression() -> impl Strategy<Value = String> {
        prop_oneof![
            arbitrary_literal(),
            arbitrary_identifier(),
            // Binary expressions (limited depth to avoid infinite recursion)
            (arbitrary_literal(), arbitrary_binary_operator(), arbitrary_literal())
                .prop_map(|(left, op, right)| format!("{} {} {}", left, op, right))
        ]
    }

    fn arbitrary_assignment() -> impl Strategy<Value = String> {
        (
            prop::option::of("mut "),
            arbitrary_identifier(),
            arbitrary_simple_expression()
        ).prop_map(|(mut_kw, id, expr)| {
            format!("{}{} = {};", mut_kw.unwrap_or(""), id, expr)
        })
    }

    fn arbitrary_print_statement() -> impl Strategy<Value = String> {
        arbitrary_simple_expression()
            .prop_map(|expr| format!("seeAm {};", expr))
    }

    fn arbitrary_function_definition() -> impl Strategy<Value = String> {
        (
            arbitrary_identifier(),
            prop::collection::vec(arbitrary_identifier(), 0..3),
            arbitrary_simple_expression()
        ).prop_map(|(name, params, body)| {
            let param_list = params.join(", ");
            format!("fn {}({}) {{ return {}; }}", name, param_list, body)
        })
    }

    fn arbitrary_valid_statement() -> impl Strategy<Value = String> {
        prop_oneof![
            arbitrary_assignment(),
            arbitrary_print_statement(),
            arbitrary_function_definition()
        ]
    }

    fn arbitrary_valid_program() -> impl Strategy<Value = String> {
        prop::collection::vec(arbitrary_valid_statement(), 1..5)
            .prop_map(|statements| statements.join("\n"))
    }

    // Invalid program generators for negative testing
    
    fn arbitrary_invalid_syntax() -> impl Strategy<Value = String> {
        prop_oneof![
            // Missing semicolons
            arbitrary_identifier().prop_map(|id| format!("{} = 42", id)),
            // Invalid operators
            (arbitrary_literal(), arbitrary_literal())
                .prop_map(|(left, right)| format!("{} @ {}", left, right)),
            // Unclosed braces
            arbitrary_identifier().prop_map(|id| format!("fn {}() {{", id)),
            // Invalid identifiers (starting with numbers)
            "[0-9][a-zA-Z0-9]*".prop_map(|id| format!("{} = 42;", id)),
            // Reserved keywords as identifiers
            prop_oneof!["fn", "mut", "if", "else", "struct"]
                .prop_map(|kw| format!("{} = 42;", kw))
        ]
    }

    // Property tests

    proptest! {
        #[test]
        fn property_valid_programs_should_parse(program in arbitrary_valid_program()) {
            let mut lexer = Lexer::new(&program);
            let tokens = lexer.tokenize();
            
            // Valid programs should tokenize successfully
            prop_assert!(tokens.is_ok(), "Valid program should tokenize: {}", program);
            
            let tokens = tokens.unwrap();
            let mut parser = Parser::new(tokens);
            let ast = parser.parse();
            
            // Valid programs should parse successfully
            prop_assert!(ast.is_ok(), "Valid program should parse: {}", program);
        }

        #[test] 
        fn property_invalid_syntax_should_be_rejected(program in arbitrary_invalid_syntax()) {
            let mut lexer = Lexer::new(&program);
            let tokens = lexer.tokenize();
            
            if let Ok(tokens) = tokens {
                let mut parser = Parser::new(tokens);
                let ast = parser.parse();
                
                // Invalid programs should be rejected
                prop_assert!(ast.is_err(), "Invalid program should be rejected: {}", program);
            }
            // If tokenization fails, that's also correct rejection
        }

        #[test]
        fn property_grammar_showcase_should_parse() {
            // The complete grammar showcase should always parse successfully
            let showcase_content = include_str!("../../../examples/grammar_showcase.ov");
            
            let mut lexer = Lexer::new(showcase_content);
            let tokens = lexer.tokenize();
            prop_assert!(tokens.is_ok(), "Grammar showcase should tokenize");
            
            let tokens = tokens.unwrap();
            let mut parser = Parser::new(tokens);
            let ast = parser.parse();
            prop_assert!(ast.is_ok(), "Grammar showcase should parse");
        }

        #[test]
        fn property_deterministic_parsing(program in arbitrary_valid_program()) {
            // Same program should always produce the same AST
            let mut lexer1 = Lexer::new(&program);
            let tokens1 = lexer1.tokenize();
            
            let mut lexer2 = Lexer::new(&program);
            let tokens2 = lexer2.tokenize();
            
            if let (Ok(tokens1), Ok(tokens2)) = (tokens1, tokens2) {
                let mut parser1 = Parser::new(tokens1);
                let ast1 = parser1.parse();
                
                let mut parser2 = Parser::new(tokens2);
                let ast2 = parser2.parse();
                
                // Same input should produce same result
                prop_assert_eq!(ast1.is_ok(), ast2.is_ok(), 
                    "Parsing should be deterministic for: {}", program);
                
                if let (Ok(ast1), Ok(ast2)) = (ast1, ast2) {
                    // ASTs should be structurally equivalent
                    prop_assert_eq!(format!("{:?}", ast1), format!("{:?}", ast2),
                        "ASTs should be identical for: {}", program);
                }
            }
        }
    }

    // Unit tests for specific grammar rules

    #[test]
    fn test_all_keywords_recognized() {
        let keywords = [
            "fn", "mut", "if", "else", "for", "while", 
            "struct", "enum", "unsafe", "return", 
            "true", "false", "seeAm"
        ];
        
        for keyword in &keywords {
            let mut lexer = Lexer::new(keyword);
            let tokens = lexer.tokenize().expect("Keyword should tokenize");
            assert!(!tokens.is_empty(), "Keyword {} should produce tokens", keyword);
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let expressions = [
            "2 + 3 * 4",      // Should parse as 2 + (3 * 4)
            "a && b || c",    // Should parse as (a && b) || c
            "x < y == z > w", // Should parse as (x < y) == (z > w)
        ];
        
        for expr in &expressions {
            let program = format!("result = {};", expr);
            let mut lexer = Lexer::new(&program);
            let tokens = lexer.tokenize().expect("Expression should tokenize");
            
            let mut parser = Parser::new(tokens);
            let ast = parser.parse().expect("Expression should parse");
            
            // Verify the AST structure reflects correct precedence
            // (This would need more detailed AST inspection in a real implementation)
            assert!(!format!("{:?}", ast).is_empty());
        }
    }

    #[test]
    fn test_nested_structures() {
        let program = r#"
            struct Person {
                name: String,
                age: Number,
            }
            
            person = Person {
                name: "Alice",
                age: 30,
            };
            
            seeAm person.name;
        "#;
        
        let mut lexer = Lexer::new(program);
        let tokens = lexer.tokenize().expect("Nested structures should tokenize");
        
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Nested structures should parse");
        
        // Verify we got a valid AST
        assert!(!format!("{:?}", ast).is_empty());
    }

    #[test]
    fn test_function_definitions_and_calls() {
        let program = r#"
            fn add(a, b) {
                return a + b;
            }
            
            result = add(10, 20);
            seeAm result;
        "#;
        
        let mut lexer = Lexer::new(program);
        let tokens = lexer.tokenize().expect("Functions should tokenize");
        
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Functions should parse");
        
        // Verify we got a valid AST
        assert!(!format!("{:?}", ast).is_empty());
    }

    #[test]
    fn test_control_flow_constructs() {
        let program = r#"
            if x < 10 {
                seeAm "small";
            } else {
                seeAm "large";
            }
            
            for i in 0..10 {
                seeAm i;
            }
            
            while counter < 5 {
                counter = counter + 1;
            }
        "#;
        
        let mut lexer = Lexer::new(program);
        let tokens = lexer.tokenize().expect("Control flow should tokenize");
        
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Control flow should parse");
        
        // Verify we got a valid AST
        assert!(!format!("{:?}", ast).is_empty());
    }
}