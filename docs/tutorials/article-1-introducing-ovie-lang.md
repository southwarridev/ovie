# Introducing Ovie Lang

## A Systems Programming Language

Ovie is a systems programming language created by Shedrack Erhabor and owned by Nashedy Ltd.

Ovie combines low-level programming capabilities with high-level language features,
providing developers with direct control, expressive syntax and a structured development environment.

## A Language with Its Own Identity

Ovie introduces its own declaration and output vocabulary:

    oya name = "Shedrack"
    oya language = "Ovie"
    mut age = 31
    seeAm "Hello, " + name
    seeAm language
    seeAm age

- oya: immutable declaration.
- mut: mutable declaration.
- seeAm: output with a newline.
- print: output without a newline.

## Compiler Architecture

Ovie has two compiler implementations:

- oviec: The Rust-based compiler.
- oviev: The self-hosted compiler written in Ovie (.ov).

## Aproko

Aproko (an Urhobo word) is the built-in analyser. It runs on every oviec run.
It catches bad keywords, explains runtime errors, and teaches best practices.

## The Official Project

Ovie Lang is the official organization for the language.
Developed under Nashedy Ltd.

Official repository: https://github.com/Ovie-Lang/ovie
X: https://x.com/Ovielang
GitHub org: https://github.com/Ovie-Lang

## License

MIT License. Copyright (c) 2026 Nashedy Ltd.