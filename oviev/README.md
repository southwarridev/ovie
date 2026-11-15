# oviev

The self-hosted Ovie compiler, written in Ovie (.ov files).
Part of Ovie Lang 1.0. MIT License. Nashedy Ltd.

## What is oviev?

oviev is Ovie compiling itself.
Every file in oviec/src/*.rs has a mirror .ov file in oviev/src/.
Stage 1: bootstraps via oviec (Rust).
Stage 2: oviev compiles its own .ov files without Rust.

## Run oviev

    oviev --version        # Ovie Lang 1.0
    oviev run hello.ov     # Run any .ov program
    oviev check hello.ov   # Check for errors
    oviev analyze file.ov  # Run Aproko analysis

## File map

| Rust (oviec/src/) | Ovie (oviev/src/) | Purpose |
|------------------|------------------|---------|
| lexer.rs | lexer.ov | Tokenizer |
| parser.rs | parser.ov | Token -> AST |
| ast.rs | ast.ov | AST node definitions |
| hir.rs | hir.ov | High-level IR |
| mir.rs | mir.ov | Mid-level IR |
| ir.rs | ir.ov | Shared IR types |
| interpreter.rs | interpreter.ov | Runtime |
| semantic.rs | semantic.ov | Type checking |
| error.rs | error.ov | Error types |
| main.rs | main.ov | CLI entry point |
| codegen/llvm.rs | backend/llvm.ov | LLVM backend |
| codegen/wasm.rs | backend/wasm.ov | WASM backend |

## Stage plan

| Stage | Status | Description |
|-------|--------|-------------|
| 1 | Now | .ov files written, bootstrap via oviec |
| 2 | Planned | oviev compiles its own .ov files |
| 3 | Future | oviec retired, oviev is primary |

## Tests

All 15 keyword tests pass. Aproko clean on all 13 source files.

    oviec run oviev/src/tests/test_all.ov

## License

MIT License. Copyright (c) 2026 Nashedy Ltd.
Official: https://github.com/Ovie-Lang/ovie