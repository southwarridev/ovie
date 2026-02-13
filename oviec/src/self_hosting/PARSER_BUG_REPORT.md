# Ovie Compiler Parser Bug Report

## Bug Summary
The Ovie compiler parser incorrectly interprets certain if-statement conditions as struct instantiations, causing parse errors.

## Bug Description
When an if-statement condition contains a binary expression where the right operand is an identifier, followed by a block `{`, the parser incorrectly interprets the identifier + block as a struct instantiation instead of recognizing the block as the then-block of the if-statement.

## Symptoms
- **Error Message**: `Parse error at line X, column Y: Expected field name (found 'seeAm')`
- The error occurs at the first statement inside the then-block
- The parser consumed the `{` thinking it was starting a struct instantiation

## Affected Patterns

### Pattern 1: Binary expression with identifier on right side
```ovie
fn test(a, b) {
    if a == b {  // FAILS
        seeAm "equal";
    }
}
```

### Pattern 2: Any binary operator
```ovie
if a + b {   // FAILS
if a < b {   // FAILS  
if a * b {   // FAILS
```

### Pattern 3: With or without else block
```ovie
if a == b {      // FAILS
    seeAm "yes";
}

if a == b {      // ALSO FAILS
    seeAm "yes";
} else {
    seeAm "no";
}
```

### Pattern 4: Local variables or parameters
```ovie
// With local variables - FAILS
fn test() {
    let a = 1;
    let b = 2;
    if a == b {
        seeAm "equal";
    }
}

// With parameters - ALSO FAILS
fn test(a, b) {
    if a == b {
        seeAm "equal";
    }
}
```

## Working Patterns

### Pattern 1: Literal on right side
```ovie
fn test() {
    let a = 1;
    if a == 2 {  // WORKS
        seeAm "yes";
    }
}
```

### Pattern 2: Literals on both sides
```ovie
if 1 == 2 {  // WORKS
    seeAm "yes";
}
```

### Pattern 3: Simple boolean
```ovie
if true {  // WORKS
    seeAm "yes";
}
```

### Pattern 4: Assignment (not if-statement)
```ovie
let x = a == b;  // WORKS
```

## Root Cause Analysis

The bug is in `oviec/src/parser.rs` in the `primary_base()` function around line 447:

```rust
TokenType::Identifier => {
    let name = self.advance().lexeme.clone();
    
    // Check for function call
    if self.check(&TokenType::LeftParen) {
        // ... function call parsing
    } else if self.check(&TokenType::LeftBrace) {
        // Struct instantiation
        self.advance(); // consume '{'
        
        let mut fields = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let field_name = self.consume_identifier("Expected field name")?;
            // ...
        }
        // ...
    } else {
        // Simple identifier
        Ok(Expression::Identifier(name))
    }
}
```

The issue is that when parsing the condition `a == b`, after the identifier `b` is parsed and returned from the expression parsing, somehow the parser re-evaluates or re-parses the identifier in a context where it sees `b {` and interprets it as a struct instantiation.

The exact mechanism of how the parser gets into this state is unclear, but it appears to be related to:
1. How the expression parsing completes and returns to the if-statement parsing
2. Possible issues with parser position/state after parsing binary expressions
3. Potential lookahead or backtracking issues

## Workaround

**Use parentheses around the condition:**

```ovie
// Instead of:
if a == b {
    seeAm "equal";
}

// Use:
if (a == b) {
    seeAm "equal";
}
```

This workaround successfully compiles and runs correctly.

## Impact

This bug significantly impacts the self-hosting implementation because:
1. Error handling functions need to compare parameters
2. Parser validation logic requires conditional checks
3. Many common programming patterns are affected

## Test Cases

### Minimal Failing Test
```ovie
fn test() {
    let b = 2;
    if 1 == b {
        seeAm "yes";
    }
}
```

### Minimal Working Test (with workaround)
```ovie
fn test() {
    let b = 2;
    if (1 == b) {
        seeAm "yes";
    }
}
```

## Files Affected

- `oviec/src/self_hosting/parser_minimal.ov` - Updated to use parentheses workaround
- All error handling functions now use `if (param == param)` instead of `if param == param`

## Recommendation

This bug should be fixed in the Ovie compiler parser (`oviec/src/parser.rs`) to properly handle binary expressions in if-statement conditions. The fix should ensure that after parsing an expression, the parser correctly identifies the following `{` as the start of a block statement, not as part of a struct instantiation.

## Status

- **Discovered**: During implementation of Task 7.1.2.3 (Add syntax error handling to parser)
- **Workaround Applied**: Yes (using parentheses) - NO LONGER NEEDED
- **Compiler Fix**: ✅ **FIXED** in `oviec/src/parser.rs`
- **Self-Hosting Impact**: Bug is now resolved

## Fix Details

**Date Fixed**: Current session

**Fix Location**: `oviec/src/parser.rs`

**Fix Description**: 
Added a lookahead check `looks_like_struct_instantiation()` that verifies the pattern `{ identifier : ...` before treating `identifier {` as struct instantiation. This prevents the parser from incorrectly interpreting if-statement blocks as struct instantiations.

**Changes Made**:
1. Modified the identifier parsing in `primary_base()` to check `self.looks_like_struct_instantiation()` before assuming struct instantiation
2. Added helper method `looks_like_struct_instantiation()` that looks ahead to verify the pattern matches struct field initialization (identifier followed by colon)

**Code Changes**:
```rust
// Before (buggy):
} else if self.check(&TokenType::LeftBrace) {
    // Struct instantiation
    self.advance(); // consume '{'
    // ...
}

// After (fixed):
} else if self.check(&TokenType::LeftBrace) && self.looks_like_struct_instantiation() {
    // Struct instantiation - only if it looks like field initialization
    // This prevents treating "if a == b {" as struct instantiation
    self.advance(); // consume '{'
    // ...
}
```

**Verification**:
All previously failing test cases now pass:
- ✅ `test_right_identifier.ov` - identifier on right side of binary expression
- ✅ `test_local_vars.ov` - local variables in if condition
- ✅ `test_if_variations.ov` - multiple if-statement patterns
- ✅ `parser_bug_minimal.ov` - minimal reproduction case
- ✅ Struct instantiation still works correctly (parser recognizes it, interpreter not yet implemented)

## Related Files

- `oviec/src/parser.rs` - Parser implementation (needs fix)
- `oviec/src/self_hosting/parser_minimal.ov` - Self-hosting parser (uses workaround)
- `oviec/src/self_hosting/parser_minimal_fixed.ov` - Minimal demonstration of bug
- `oviec/src/self_hosting/parser_bug_test.ov` - Test cases for bug
- `test_*.ov` - Various test files created during bug investigation
