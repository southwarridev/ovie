//! Lexical analyzer for the Ovie programming language

use crate::error::{OvieError, OvieResult, SourceLocation};
use logos::Logos;
use std::fmt;

/// Token types for the Ovie language
#[derive(Logos, Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Keywords (exactly 13 as per spec)
    #[token("fn")]
    Fn,
    #[token("mut")]
    Mut,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("for")]
    For,
    #[token("while")]
    While,
    #[token("struct")]
    Struct,
    #[token("enum")]
    Enum,
    #[token("unsafe")]
    Unsafe,
    #[token("return")]
    Return,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("seeAm")]
    SeeAm,
    #[token("let")]
    Let,

    // Identifiers and literals
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    
    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral,
    
    #[regex(r"\d+\.\d+")]
    FloatLiteral,
    
    #[regex(r"\d+")]
    IntegerLiteral,
    
    // Number token (alias for IntegerLiteral)
    Number,
    
    // String token (alias for StringLiteral)
    String,

    // Operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    
    #[token("==")]
    EqualEqual,
    #[token("!=")]
    NotEqual,
    
    // BangEqual token (alias for NotEqual)
    BangEqual,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,
    
    #[token("&&")]
    AndAnd,
    #[token("||")]
    OrOr,
    #[token("!")]
    Bang,
    
    #[token("=")]
    Equal,
    
    // Assignment operator (alias for Equal)
    Assign,

    // Delimiters
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(".")]
    Dot,
    #[token("..")]
    DotDot,

    // Special tokens
    #[token("in")]
    In,

    // Whitespace and comments (ignored)
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    #[regex(r"//[^\r\n]*", logos::skip)]
    Whitespace,

    // End of file
    Eof,

    // Error token
    Error,
}

/// A token with its type, value, and location
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub location: SourceLocation,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, location: SourceLocation) -> Self {
        Self {
            token_type,
            lexeme,
            location,
        }
    }
}

/// Lexer for tokenizing Ovie source code
pub struct Lexer<'a> {
    source: &'a str,
    current_line: usize,
    current_column: usize,
    current_offset: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given source code
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            current_line: 1,
            current_column: 1,
            current_offset: 0,
        }
    }

    /// Tokenize the entire source code
    pub fn tokenize(&mut self) -> OvieResult<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut lexer = TokenType::lexer(self.source);
        
        while let Some(token_type) = lexer.next() {
            let lexeme = lexer.slice().to_string();
            let span = lexer.span();
            
            // Calculate line and column from span
            let location = self.calculate_location(span.start);
            
            match token_type {
                Ok(token_type) => {
                    // Handle special cases for keywords vs identifiers
                    let final_token_type = match token_type {
                        TokenType::Identifier => {
                            self.classify_identifier(&lexeme)
                        }
                        _ => token_type,
                    };
                    
                    tokens.push(Token::new(final_token_type, lexeme, location));
                }
                Err(_) => {
                    return Err(OvieError::lex_error(
                        location.line,
                        location.column,
                        format!("Unexpected character: '{}'", lexeme),
                    ));
                }
            }
        }

        // Add EOF token
        let eof_location = self.calculate_location(self.source.len());
        tokens.push(Token::new(TokenType::Eof, String::new(), eof_location));

        Ok(tokens)
    }

    /// Calculate source location from byte offset
    fn calculate_location(&self, offset: usize) -> SourceLocation {
        let mut line = 1;
        let mut column = 1;
        
        for (i, ch) in self.source.char_indices() {
            if i >= offset {
                break;
            }
            
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        
        SourceLocation::new(line, column, offset)
    }

    /// Classify an identifier token (might be a keyword)
    fn classify_identifier(&self, lexeme: &str) -> TokenType {
        match lexeme {
            "fn" => TokenType::Fn,
            "mut" => TokenType::Mut,
            "let" => TokenType::Let,
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
            "in" => TokenType::In,
            _ => TokenType::Identifier,
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            TokenType::Fn => "fn",
            TokenType::Mut => "mut",
            TokenType::Let => "let",
            TokenType::If => "if",
            TokenType::Else => "else",
            TokenType::For => "for",
            TokenType::While => "while",
            TokenType::Struct => "struct",
            TokenType::Enum => "enum",
            TokenType::Unsafe => "unsafe",
            TokenType::Return => "return",
            TokenType::True => "true",
            TokenType::False => "false",
            TokenType::SeeAm => "seeAm",
            TokenType::Identifier => "identifier",
            TokenType::StringLiteral => "string",
            TokenType::FloatLiteral => "float",
            TokenType::IntegerLiteral => "integer",
            TokenType::Number => "number",
            TokenType::String => "string",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Star => "*",
            TokenType::Slash => "/",
            TokenType::Percent => "%",
            TokenType::EqualEqual => "==",
            TokenType::NotEqual => "!=",
            TokenType::BangEqual => "!=",
            TokenType::Less => "<",
            TokenType::LessEqual => "<=",
            TokenType::Greater => ">",
            TokenType::GreaterEqual => ">=",
            TokenType::AndAnd => "&&",
            TokenType::OrOr => "||",
            TokenType::Bang => "!",
            TokenType::Equal => "=",
            TokenType::Assign => "=",
            TokenType::LeftParen => "(",
            TokenType::RightParen => ")",
            TokenType::LeftBrace => "{",
            TokenType::RightBrace => "}",
            TokenType::LeftBracket => "[",
            TokenType::RightBracket => "]",
            TokenType::Comma => ",",
            TokenType::Semicolon => ";",
            TokenType::Colon => ":",
            TokenType::Dot => ".",
            TokenType::DotDot => "..",
            TokenType::In => "in",
            TokenType::Whitespace => "whitespace",
            TokenType::Eof => "EOF",
            TokenType::Error => "error",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} '{}' at {}:{}",
            self.token_type, self.lexeme, self.location.line, self.location.column
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokens() {
        let mut lexer = Lexer::new("seeAm \"hello\"");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 3); // seeAm, string, EOF
        assert_eq!(tokens[0].token_type, TokenType::SeeAm);
        assert_eq!(tokens[1].token_type, TokenType::StringLiteral);
        assert_eq!(tokens[2].token_type, TokenType::Eof);
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("fn mut if else for while struct enum unsafe return true false seeAm");
        let tokens = lexer.tokenize().unwrap();
        
        let expected = vec![
            TokenType::Fn,
            TokenType::Mut,
            TokenType::If,
            TokenType::Else,
            TokenType::For,
            TokenType::While,
            TokenType::Struct,
            TokenType::Enum,
            TokenType::Unsafe,
            TokenType::Return,
            TokenType::True,
            TokenType::False,
            TokenType::SeeAm,
            TokenType::Eof,
        ];
        
        for (i, expected_type) in expected.iter().enumerate() {
            assert_eq!(tokens[i].token_type, *expected_type);
        }
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::new("+ - * / % == != < <= > >= && || ! =");
        let tokens = lexer.tokenize().unwrap();
        
        let expected = vec![
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Star,
            TokenType::Slash,
            TokenType::Percent,
            TokenType::EqualEqual,
            TokenType::NotEqual,
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::AndAnd,
            TokenType::OrOr,
            TokenType::Bang,
            TokenType::Equal,
            TokenType::Eof,
        ];
        
        for (i, expected_type) in expected.iter().enumerate() {
            assert_eq!(tokens[i].token_type, *expected_type);
        }
    }

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("42 3.14 0 123.456");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::IntegerLiteral);
        assert_eq!(tokens[0].lexeme, "42");
        
        assert_eq!(tokens[1].token_type, TokenType::FloatLiteral);
        assert_eq!(tokens[1].lexeme, "3.14");
        
        assert_eq!(tokens[2].token_type, TokenType::IntegerLiteral);
        assert_eq!(tokens[2].lexeme, "0");
        
        assert_eq!(tokens[3].token_type, TokenType::FloatLiteral);
        assert_eq!(tokens[3].lexeme, "123.456");
    }

    #[test]
    fn test_strings() {
        let mut lexer = Lexer::new(r#""hello" "world with spaces" "escaped \"quote\"" "#);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::StringLiteral);
        assert_eq!(tokens[0].lexeme, r#""hello""#);
        
        assert_eq!(tokens[1].token_type, TokenType::StringLiteral);
        assert_eq!(tokens[1].lexeme, r#""world with spaces""#);
        
        assert_eq!(tokens[2].token_type, TokenType::StringLiteral);
        assert_eq!(tokens[2].lexeme, r#""escaped \"quote\"""#);
    }

    #[test]
    fn test_identifiers() {
        let mut lexer = Lexer::new("variable_name camelCase PascalCase _underscore name123");
        let tokens = lexer.tokenize().unwrap();
        
        for i in 0..5 {
            assert_eq!(tokens[i].token_type, TokenType::Identifier);
        }
        
        assert_eq!(tokens[0].lexeme, "variable_name");
        assert_eq!(tokens[1].lexeme, "camelCase");
        assert_eq!(tokens[2].lexeme, "PascalCase");
        assert_eq!(tokens[3].lexeme, "_underscore");
        assert_eq!(tokens[4].lexeme, "name123");
    }

    #[test]
    fn test_hello_world() {
        let mut lexer = Lexer::new(r#"seeAm "Hello, World!";"#);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 4); // seeAm, string, semicolon, EOF
        assert_eq!(tokens[0].token_type, TokenType::SeeAm);
        assert_eq!(tokens[1].token_type, TokenType::StringLiteral);
        assert_eq!(tokens[1].lexeme, r#""Hello, World!""#);
        assert_eq!(tokens[2].token_type, TokenType::Semicolon);
        assert_eq!(tokens[3].token_type, TokenType::Eof);
    }

    // Property-based tests for grammar compliance
    use proptest::prelude::*;

    /// **Property 1: Language Grammar Compliance (Lexer component)**
    /// **Validates: Requirements 1.1, 6.2**
    /// 
    /// This property ensures that the lexer correctly tokenizes all valid Ovie language constructs
    /// according to the formal grammar specification.
    mod property_tests {
        use super::*;
        use proptest::prelude::*;

        // Generator for valid identifiers
        fn valid_identifier() -> impl Strategy<Value = String> {
            prop::string::string_regex(r"[a-zA-Z_][a-zA-Z0-9_]*")
                .unwrap()
                .prop_filter("Not a keyword", |s| {
                    !matches!(s.as_str(), 
                        "fn" | "mut" | "if" | "else" | "for" | "while" | 
                        "struct" | "enum" | "unsafe" | "return" | "true" | 
                        "false" | "seeAm" | "in"
                    )
                })
        }

        // Generator for valid string literals
        fn valid_string_literal() -> impl Strategy<Value = String> {
            prop::string::string_regex(r#""([^"\\]|\\.)*""#).unwrap()
        }

        // Generator for valid integer literals
        fn valid_integer_literal() -> impl Strategy<Value = String> {
            prop::string::string_regex(r"\d+").unwrap()
        }

        // Generator for valid float literals
        fn valid_float_literal() -> impl Strategy<Value = String> {
            prop::string::string_regex(r"\d+\.\d+").unwrap()
        }

        // Generator for keywords
        fn keyword() -> impl Strategy<Value = String> {
            prop_oneof![
                Just("fn".to_string()),
                Just("mut".to_string()),
                Just("if".to_string()),
                Just("else".to_string()),
                Just("for".to_string()),
                Just("while".to_string()),
                Just("struct".to_string()),
                Just("enum".to_string()),
                Just("unsafe".to_string()),
                Just("return".to_string()),
                Just("true".to_string()),
                Just("false".to_string()),
                Just("seeAm".to_string()),
            ]
        }

        proptest! {
            #[test]
            fn prop_valid_identifiers_tokenize_correctly(identifier in valid_identifier()) {
                let mut lexer = Lexer::new(&identifier);
                let tokens = lexer.tokenize().unwrap();
                
                prop_assert_eq!(tokens.len(), 2); // identifier + EOF
                prop_assert_eq!(tokens[0].token_type, TokenType::Identifier);
                prop_assert_eq!(&tokens[0].lexeme, &identifier);
                prop_assert_eq!(tokens[1].token_type, TokenType::Eof);
            }

            #[test]
            fn prop_valid_string_literals_tokenize_correctly(string_lit in valid_string_literal()) {
                let mut lexer = Lexer::new(&string_lit);
                let tokens = lexer.tokenize().unwrap();
                
                prop_assert_eq!(tokens.len(), 2); // string + EOF
                prop_assert_eq!(tokens[0].token_type, TokenType::StringLiteral);
                prop_assert_eq!(&tokens[0].lexeme, &string_lit);
                prop_assert_eq!(tokens[1].token_type, TokenType::Eof);
            }

            #[test]
            fn prop_valid_integer_literals_tokenize_correctly(int_lit in valid_integer_literal()) {
                let mut lexer = Lexer::new(&int_lit);
                let tokens = lexer.tokenize().unwrap();
                
                prop_assert_eq!(tokens.len(), 2); // integer + EOF
                prop_assert_eq!(tokens[0].token_type, TokenType::IntegerLiteral);
                prop_assert_eq!(&tokens[0].lexeme, &int_lit);
                prop_assert_eq!(tokens[1].token_type, TokenType::Eof);
            }

            #[test]
            fn prop_valid_float_literals_tokenize_correctly(float_lit in valid_float_literal()) {
                let mut lexer = Lexer::new(&float_lit);
                let tokens = lexer.tokenize().unwrap();
                
                prop_assert_eq!(tokens.len(), 2); // float + EOF
                prop_assert_eq!(tokens[0].token_type, TokenType::FloatLiteral);
                prop_assert_eq!(&tokens[0].lexeme, &float_lit);
                prop_assert_eq!(tokens[1].token_type, TokenType::Eof);
            }

            #[test]
            fn prop_keywords_tokenize_correctly(kw in keyword()) {
                let mut lexer = Lexer::new(&kw);
                let tokens = lexer.tokenize().unwrap();
                
                prop_assert_eq!(tokens.len(), 2); // keyword + EOF
                prop_assert_ne!(tokens[0].token_type, TokenType::Identifier);
                prop_assert_eq!(&tokens[0].lexeme, &kw);
                prop_assert_eq!(tokens[1].token_type, TokenType::Eof);
            }

            #[test]
            fn prop_whitespace_is_ignored(
                content in r"[a-zA-Z]+",
                whitespace in r"[ \t\r\n]+"
            ) {
                let input = format!("{}{}{}", whitespace, content, whitespace);
                let mut lexer = Lexer::new(&input);
                let tokens = lexer.tokenize().unwrap();
                
                // Should only have the content token and EOF, whitespace ignored
                prop_assert_eq!(tokens.len(), 2);
                prop_assert_eq!(&tokens[0].lexeme, &content);
                prop_assert_eq!(tokens[1].token_type, TokenType::Eof);
            }

            #[test]
            fn prop_comments_are_ignored(
                content in r"[a-zA-Z]+",
                comment in r"//[^\r\n]*"
            ) {
                let input = format!("{}\n{}", comment, content);
                let mut lexer = Lexer::new(&input);
                let tokens = lexer.tokenize().unwrap();
                
                // Should only have the content token and EOF, comment ignored
                prop_assert_eq!(tokens.len(), 2);
                prop_assert_eq!(&tokens[0].lexeme, &content);
                prop_assert_eq!(tokens[1].token_type, TokenType::Eof);
            }

            #[test]
            fn prop_all_tokens_have_valid_locations(input in r"[a-zA-Z0-9 \t\n]+") {
                let mut lexer = Lexer::new(&input);
                if let Ok(tokens) = lexer.tokenize() {
                    for token in &tokens {
                        prop_assert!(token.location.line >= 1);
                        prop_assert!(token.location.column >= 1);
                    }
                }
            }
        }
    }
}