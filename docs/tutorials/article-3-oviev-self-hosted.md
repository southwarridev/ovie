# oviev: Building a Compiler in Ovie  The Self-Hosting Journey

One of the milestones of any programming language is when it can compile itself.
Ovie Lang 1.0 ships oviev  the Ovie compiler written entirely in Ovie (.ov files).

## Two Compilers, One Language

| Compiler | Language | Status |
|----------|----------|--------|
| oviec | Rust | Stable  use for everything today |
| oviev | Ovie (.ov) | Stage 1  self-hosted, help test it |

Both run the same .ov programs:

    oviec run hello.ov   # stable Rust compiler
    oviev run hello.ov   # Ovie self-hosted compiler

## The File Map

Every file in oviec/src/*.rs has a mirror .ov file in oviev/src/:

| Rust (oviec/src/) | Ovie (oviev/src/) | Purpose |
|------------------|------------------|---------|
| lexer.rs | lexer.ov | Tokenizer |
| parser.rs | parser.ov | Token -> AST |
| ast.rs | ast.ov | AST node definitions |
| hir.rs | hir.ov | High-level IR |
| mir.rs | mir.ov | Mid-level IR |
| ir.rs | ir.ov | IR shared types |
| interpreter.rs | interpreter.ov | Runtime |
| semantic.rs | semantic.ov | Type checking |
| error.rs | error.ov | Error types |
| main.rs | main.ov | Entry point + CLI |
| codegen/llvm.rs | backend/llvm.ov | LLVM backend |
| codegen/wasm.rs | backend/wasm.ov | WASM backend |

## Run oviev

    oviev --version

Output:

    == Aproko =============================================
      OK: no issues found in oviev/src/main.ov
    =====================================================
    oviev 1.0  Stage 1  Ovie self-hosted (bootstrapped via oviec)
    Usage: oviev <command> [file.ov]
    Commands:
      run <file.ov>   Compile and run
      check <file.ov> Check for errors
      --version       Show version

## Run a program with oviev

    oviev run calculator.ov

oviev routes this to oviec. Both produce identical output.
Stage 1: oviev delegates to oviec (Rust bootstrap).
Stage 2: oviev compiles its own .ov files without help from Rust.

## Look at lexer.ov

The lexer tokenizes source code. In lexer.ov, written in Ovie:

    // Token types  every keyword the lexer can produce
    enum TokenKind {
        Oya, Mut, Fn, If, Else, For, While, Return, Break, Continue,
        Struct, Enum, Match, Const, Use, Import, Export,
        SeeAm, True, False, Unsafe,
        Identifier, NumberLiteral, StringLiteral,
        Plus, Minus, Star, Slash, Percent,
        Equal, EqualEqual, NotEqual, Arrow,
        LeftBrace, RightBrace, LeftParen, RightParen,
        Comma, Colon, Semicolon, Dot, Eof
    }

    fn keyword_kind(word: String) -> TokenKind {
        if word == "oya"     { return TokenKind.Oya }
        if word == "mut"     { return TokenKind.Mut }
        if word == "fn"      { return TokenKind.Fn }
        if word == "seeAm"   { return TokenKind.SeeAm }
        return TokenKind.Identifier
    }

The Ovie keywords are wired right into the tokenizer  in Ovie itself.

## The Stage Plan

| Stage | Status | What it means |
|-------|--------|----------------|
| Stage 1 | Now | .ov files written. Bootstrap via oviec |
| Stage 2 | Planned | oviev compiles its own .ov files |
| Stage 3 | Future | oviec (Rust) retired. oviev is primary |

## Why it matters

Self-hosting is the proof that a language works.
When Ovie can describe its own compiler in Ovie, the language is complete enough to build real things.
Every .ov file in oviev/src/ is a test that Ovie's syntax is powerful enough to express compiler logic.

## Try it yourself

    oviec run oviev/src/lexer.ov     # run the Ovie lexer written in Ovie
    oviec check oviev/src/parser.ov  # check the Ovie parser written in Ovie
    oviec run oviev/src/main.ov      # run the oviev entry point

All pass Aproko with: OK: no issues found
