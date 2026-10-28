# Ovie — The Complete Book
## Ovie in Southern Kaduna

A practical guide to the Ovie programming language.
Written for developers in Southern Kaduna and everywhere else.

---

## About This Book

This book teaches Ovie from first principles. You will learn by building real programs.
Every example runs. Every concept is explained in plain language.

Ovie is a systems programming language with natural syntax.
It was built for developers who want low-level control without fighting the language to get it.

The two keywords you need to know before anything else:

```ovie
oya name = "Ovie"    // immutable — this never changes
mut count = 0        // mutable — this can be updated
```

---

## Chapters

| Chapter | Title | What You Learn |
|---------|-------|----------------|
| 1 | [Introduction](chapters/01-introduction.md) | What Ovie is, oya vs mut, the Aproko engine |
| 2 | [Installation](chapters/02-installation.md) | Install on Windows, Linux, macOS, or build from source |
| 3 | [Language Fundamentals](chapters/03-fundamentals.md) | oya, mut, fn, if, loops, structs, enums, match |
| 4 | [Module System](chapters/04-modules.md) | use, import, export, packages, ovie.toml |
| 5 | [Standard Library](chapters/05-stdlib.md) | All 11 stdlib modules with examples |
| 6 | [Aproko](chapters/06-aproko.md) | Built-in analyser, best practices, error explanations |
| 7 | [Production Programs](chapters/07-production.md) | Error handling, structs, real programs |
| 8 | [Testing](chapters/08-testing.md) | Unit tests, property tests, std::testing |
| 9 | [Performance](chapters/09-performance.md) | Profiling, benchmarking, optimization |
| 10 | [Deployment](chapters/10-deployment.md) | Packaging, containers, cross-platform |
| 11 | [Community](chapters/11-community.md) | Contributing, GitHub, Discord |
| 12 | [Stories](chapters/12-stories.md) | Real programs built with Ovie |

---

## Quick Start

Install Ovie, then run:

```ovie
oya APP = "My first Ovie program"
seeAm APP
```

```
oviec run hello.ov
```

---

## Copyright

Copyright (c) 2026 Nashedy Ltd. MIT License.
