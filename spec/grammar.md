# Ovie Programming Language - Formal Grammar Specification

**Version:** 2.1.0  
**Status:** FROZEN (Stage 2)  
**Last Updated:** 2024-01-28  

## Overview

This document defines the complete formal grammar for the Ovie Programming Language using Extended Backus-Naur Form (EBNF). This grammar is **FROZEN** as part of Stage 2 development - no syntax changes are permitted without formal RFC process.

## Grammar Notation

- `|` - Alternation (choice)
- `*` - Zero or more repetitions
- `+` - One or more repetitions
- `?` - Optional (zero or one)
- `()` - Grouping
- `[]` - Character class
- `""` - Terminal symbols (literals)
- `(* *)` - Comments

## Complete Grammar Specification

### Program Structure

```ebnf
program = statement* ;

statement = assignment
          | function_definition
          | print_statement
          | if_statement
          | loop_statement
          | struct_definition
          | enum_definition
          | expression_statement
          | return_statement
          | unsafe_block
          ;
```

### Expressions (with Precedence)

The expression hierarchy encodes operator precedence from lowest to highest:

```ebnf
expression = logical_or ;

logical_or = logical_and ( "||" logical_and )* ;

logical_and = equality ( "&&" equality )* ;

equality = comparison ( ( "==" | "!=" ) comparison )* ;

comparison = range ( ( ">" | ">=" | "<" | "<=" ) range )* ;

range = term ( ".." term )? ;

term = factor ( ( "+" | "-" ) factor )* ;

factor = unary ( ( "*" | "/" | "%" ) unary )* ;

unary = ( "!" | "-" ) unary
      | postfix
      ;

postfix = primary ( field_access | function_call )* ;

primary = literal
        | identifier
        | "(" expression ")"
        | struct_instantiation
        ;
```

### Statements

```ebnf
assignment = [ "mut" ] identifier "=" expression ";" ;

function_definition = "fn" identifier "(" parameter_list? ")" block ;

parameter_list = identifier ( "," identifier )* ;

print_statement = "seeAm" expression ";" ;

if_statement = "if" expression block ( "else" block )? ;

loop_statement = for_loop | while_loop ;

for_loop = "for" identifier "in" expression block ;

while_loop = "while" expression block ;

struct_definition = "struct" identifier "{" field_list? "}" ;

field_list = field ( "," field )* ","? ;

field = identifier ":" type_annotation ;

enum_definition = "enum" identifier "{" variant_list? "}" ;

variant_list = variant ( "," variant )* ","? ;

variant = identifier ( "(" type_annotation ")" )? ;

expression_statement = expression ";" ;

return_statement = "return" expression? ";" ;

unsafe_block = "unsafe" block ;
```

### Blocks and Control Flow

```ebnf
block = "{" statement* "}" ;
```

### Function Calls and Access

```ebnf
function_call = "(" argument_list? ")" ;

argument_list = expression ( "," expression )* ;

field_access = "." identifier ;

struct_instantiation = identifier "{" field_initializer_list? "}" ;

field_initializer_list = field_initializer ( "," field_initializer )* ","? ;

field_initializer = identifier ":" expression ;
```

### Literals

```ebnf
literal = string_literal
        | number_literal
        | boolean_literal
        ;

string_literal = '"' string_char* '"' ;

string_char = [^"\\] | escape_sequence ;

escape_sequence = "\\" ( '"' | "\\" | "n" | "r" | "t" | "0" ) ;

number_literal = integer_literal | float_literal ;

integer_literal = digit+ ;

float_literal = digit+ "." digit+ ;

boolean_literal = "true" | "false" ;
```

### Identifiers and Types

```ebnf
identifier = letter ( letter | digit | "_" )* ;

type_annotation = identifier ;
```

### Lexical Elements

```ebnf
letter = [a-zA-Z] ;

digit = [0-9] ;

whitespace = ( " " | "\t" | "\n" | "\r" )+ ;

comment = "//" [^\n]* "\n" ;
```

## Keywords (LOCKED - Exactly 13)

The Ovie language has exactly 13 keywords, and this number is **FROZEN**:

```ebnf
keyword = "fn"      (* Function definition *)
        | "mut"     (* Mutable variable *)
        | "if"      (* Conditional *)
        | "else"    (* Alternative branch *)
        | "for"     (* For loop *)
        | "while"   (* While loop *)
        | "struct"  (* Structure definition *)
        | "enum"    (* Enumeration definition *)
        | "unsafe"  (* Unsafe code block *)
        | "return"  (* Return statement *)
        | "true"    (* Boolean literal *)
        | "false"   (* Boolean literal *)
        | "seeAm"   (* Print statement - pidgin English *)
        ;
```

## Operators

### Arithmetic Operators
- `+` - Addition
- `-` - Subtraction  
- `*` - Multiplication
- `/` - Division
- `%` - Modulo

### Comparison Operators
- `==` - Equality
- `!=` - Inequality
- `<` - Less than
- `<=` - Less than or equal
- `>` - Greater than
- `>=` - Greater than or equal

### Logical Operators
- `&&` - Logical AND
- `||` - Logical OR
- `!` - Logical NOT

### Assignment Operator
- `=` - Assignment

### Range Operator
- `..` - Range (inclusive start, exclusive end)

## Delimiters

```ebnf
delimiter = "(" | ")" | "{" | "}" | "[" | "]"
          | "," | ";" | ":" | "." | ".."
          ;
```

## Operator Precedence (Highest to Lowest)

1. **Primary expressions**: literals, identifiers, parentheses
2. **Postfix**: field access (`.`), function calls (`()`)
3. **Unary**: logical NOT (`!`), negation (`-`)
4. **Multiplicative**: `*`, `/`, `%`
5. **Additive**: `+`, `-`
6. **Range**: `..`
7. **Comparison**: `<`, `<=`, `>`, `>=`
8. **Equality**: `==`, `!=`
9. **Logical AND**: `&&`
10. **Logical OR**: `||`
11. **Assignment**: `=`

## Associativity Rules

- **Left-associative**: All binary operators except assignment
- **Right-associative**: Assignment (`=`)
- **Non-associative**: Comparison operators (chaining not allowed)

## Grammar Properties

### LL(1) Compatibility
The grammar is designed to be LL(1) parseable:
- No left recursion
- No ambiguous productions
- Clear lookahead for all decisions

### Deterministic Parsing
- Each production has a unique first set
- No conflicts in FIRST/FOLLOW sets
- Predictive parsing is possible

## Semantic Constraints

### Identifier Rules
1. Identifiers cannot be keywords
2. Identifiers must start with a letter or underscore
3. Identifiers are case-sensitive
4. Maximum identifier length: 255 characters

### Literal Constraints
1. String literals support escape sequences: `\"`, `\\`, `\n`, `\r`, `\t`, `\0`
2. Integer literals are decimal only (no hex/octal/binary)
3. Float literals require digits on both sides of decimal point
4. No scientific notation support

### Statement Rules
1. All statements except blocks require semicolons
2. Empty statements (bare semicolons) are allowed
3. Blocks create new scopes
4. Return statements are only valid inside functions

## Example Programs

### Hello World
```ovie
seeAm "Hello, World!";
```

### Variables and Functions
```ovie
name = "Ovie";
mut counter = 0;

fn greet(person) {
    seeAm "Hello, " + person + "!";
}

greet(name);
```

### Control Flow
```ovie
if counter < 10 {
    seeAm "Counter is small";
} else {
    seeAm "Counter is big";
}

for i in 0..10 {
    seeAm i;
}

while counter < 5 {
    counter = counter + 1;
    seeAm counter;
}
```

### Data Structures
```ovie
struct Person {
    name: String,
    age: Number,
}

enum Color {
    Red,
    Green,
    Blue,
}

person = Person {
    name: "Alice",
    age: 30,
};

seeAm person.name;
```

### Unsafe Code
```ovie
unsafe {
    // Direct memory operations would go here
    // (Not implemented in Stage 2)
}
```

## Grammar Validation Rules

### Syntax Validation
1. All productions must match the formal grammar
2. Proper nesting of blocks and parentheses
3. Correct operator precedence and associativity
4. Valid identifier and literal formats

### Semantic Validation
1. No use of undefined variables
2. Function calls match parameter counts
3. Struct field access on valid types
4. Return statements in appropriate contexts

## Grammar Extensions (Future)

The following extensions are planned for post-Stage 2:

1. **Generics**: `struct List<T> { ... }`
2. **Pattern Matching**: `match expr { ... }`
3. **Modules**: `mod name { ... }`
4. **Imports**: `use std::io;`
5. **Traits**: `trait Display { ... }`

These extensions will require formal RFC process and grammar updates.

## Compliance Testing

The grammar compliance is validated through:

1. **Parser Tests**: Verify all valid programs parse correctly
2. **Rejection Tests**: Verify invalid programs are rejected
3. **Precedence Tests**: Verify operator precedence is correct
4. **Property Tests**: Random program generation and validation

## Grammar Stability Guarantee

This grammar specification is **FROZEN** for Stage 2. Any changes require:

1. Formal RFC process
2. Backward compatibility analysis
3. Migration guide for existing code
4. Community review and approval

The grammar stability ensures:
- Reliable tooling development
- Consistent language behavior
- Predictable upgrade paths
- Long-term code compatibility

---

**This grammar specification is the authoritative definition of Ovie syntax and must be implemented exactly as specified.**