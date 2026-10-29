# Chapter 1: Introduction and Philosophy

## What is Ovie?

Ovie is a systems programming language built for people who want direct control over their machine without fighting the language to get it. It gives you low-level power — memory control, hardware access, predictable performance — wrapped in syntax that reads like what it means.

The name comes from a name common in Southern Nigeria. That's intentional. Ovie was built to be accessible to developers everywhere, not just those with fast internet, expensive laptops, or a computer science degree.

## Why Ovie Exists

Most systems languages punish beginners. Most beginner languages hide the machine from you. Ovie tries to sit in the middle: the power of a systems language, syntax that doesn't fight you.

Key principles behind the design:

- **Offline-first** — build and run software with no internet. All dependencies are local.
- **Deterministic** — the same code always produces the same output, always.
- **Self-hosted** — Ovie compiles itself. That is proof the language works.
- **Accessible** — `seeAm` comes from Nigerian Pidgin English. `oya` means "go ahead" or "now". The language borrows natural language patterns deliberately.

## The Two Declaration Keywords

If there is one thing to understand before writing a single line of Ovie, it is this:

```ovie
oya name = "Ovie"    // immutable — this value never changes
mut count = 0        // mutable — this value can change
```

`oya` is Ovie's word for "this is fixed." It comes from Urhobo — it roughly means "go now" or "it is set." When you declare something with `oya`, you are saying: I know this will never change. Use `oya` by default. Reach for `mut` only when you actually need to reassign something.

This distinction matters more as your programs grow. Constants, labels, error messages, configuration — all of these belong to `oya`. Loop counters, accumulators, input buffers — those are `mut`.

## Language Philosophy

Ovie is built on a few firm commitments:

1. No network required to build or run programs
2. Identical inputs always produce identical outputs
3. The compiler explains itself — through Aproko, its built-in analysis engine
4. No telemetry, no tracking, no hidden calls home

## The Aproko Engine

Aproko (a Urhobo word for someone who tells things straight) is Ovie's built-in analysis engine. It catches mistakes before your program runs, explains why something went wrong, and suggests how to fix it. When Ovie gives you an error message, it is Aproko talking.

## The Southern Kaduna Context

This book was written with Southern Kaduna developers in mind — a region in Kaduna State, Nigeria, where developers work with intermittent power, limited bandwidth, and shared devices. Ovie's offline-first design is not academic. It is a direct response to how software actually gets built in many parts of the world.

The examples in this book are practical. They assume you are building something real.

## Your First Program

```ovie
oya greeting = "Hello from Ovie!"
seeAm greeting
```

Save it as `hello.ov`. Run it:

```
oviec run hello.ov
```

Output:
```
Hello from Ovie!
```

That is it. `oya` declares the value. `seeAm` shows it.

## What is in This Book

- Chapter 2 — Installation and setup
- Chapter 3 — Language fundamentals: oya, mut, fn, if, loops
- Chapter 4 — The module system
- Chapter 5 — Standard library
- Chapter 6 — Aproko: understanding the analysis engine
- Chapter 7 — Building production programs
- Chapter 8 — Testing
- Chapter 9 — Performance
- Chapter 10 — Deployment
- Chapter 11 — Community
- Chapter 12 — Stories from the field
## oviev - The Ovie Self-Hosted Compiler

Ovie now ships two compilers. `oviec` is Rust. `oviev` is the same compiler rewritten in Ovie.

Stage 1: oviev bootstraps via oviec. Stage 2: oviev compiles itself.

```ovie
oya STAGE = "1 - bootstrapped via oviec"
seeAm "oviev is coming"
```
