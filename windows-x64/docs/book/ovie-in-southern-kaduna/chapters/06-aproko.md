# Chapter 6: Aproko Integration

## What is Aproko?

Aproko is Ovie's built-in static analysis and reasoning engine. It runs during compilation and provides feedback across six categories:

- **Syntax** — grammar compliance
- **Logic** — control flow and correctness
- **Performance** — algorithmic complexity hints
- **Security** — unsafe operations and vulnerabilities
- **Correctness** — ownership and memory safety
- **Style** — code quality and best practices

The name "Aproko" comes from Nigerian slang for someone who notices everything — fitting for a tool that watches your code closely.

## How Aproko Works

Aproko runs automatically when you compile. It analyzes your AST and produces findings with severity levels: `Info`, `Warning`, `Error`, `Critical`.

```bash
oviec analyze my_program.ov
```

Example output:

```
=== Aproko Analysis: my_program.ov ===

[WARNING] Style: Variable 'x' is single-character — consider a descriptive name (line 3)
[ERROR] Security: Unsafe block without justification comment (line 12)
[INFO] Performance: Loop over array could use early exit (line 20)
```

## Configuring Aproko

Create or edit `.ovie/aproko.toml`:

```toml
[analysis]
min_severity = "Warning"   # Info | Warning | Error | Critical

[categories]
syntax = true
logic = true
performance = true
security = true
correctness = true
style = false              # Disable style checks

[custom_rules]
# Add project-specific rules here
```

## Understanding Aproko Reports

Each finding includes:
- Category and severity
- Human-readable message
- Source location (line, column)
- Suggestion for fixing

## Knowledge Base Integration

Aproko stores analysis results in `.ovie/aproko/knowledge/` — a persistent knowledge base that AI tools and LLMs can query.

Query the knowledge base:

```bash
ovie aproko query --category TypeInformation --symbol add
```

Export for AI consumption:

```bash
ovie aproko export --output knowledge.json
```

The exported JSON contains type information, reasoning rules, and code patterns — structured for LLM consumption.

## Using Aproko in Your Workflow

Run analysis before committing:

```bash
oviec analyze src/main.ov
```

Check documentation completeness:

```bash
ovie doc check
```

Generate documentation:

```bash
ovie doc
```

## Custom Analysis Rules

Add custom rules to `.ovie/aproko.toml`:

```toml
[[custom_rules]]
id = "no-magic-numbers"
description = "Avoid magic numbers — use named constants"
pattern = "= [0-9]+"
suggestion = "Extract magic number into a named constant"
severity = "Warning"
```

## AI-Assisted Development

The knowledge base makes Ovie programs more accessible to AI tools. When an LLM queries the knowledge base, it gets:

- Type signatures for all exported functions
- Reasoning rules the compiler applied
- Code patterns detected in your codebase

This means AI assistants can give more accurate suggestions for your specific codebase.

## Aproko and the Module System

When analyzing modular programs, Aproko:

1. Loads all imported modules
2. Tracks data flow across module boundaries
3. Detects unused imports
4. Verifies exported symbols are documented
5. Checks for breaking API changes between versions

```ovie
use std::math::{sqrt}
use std::io::{println}

// Aproko will warn if sqrt or println are imported but never used
fn main() {
    println("Hello")
    // sqrt imported but unused — Aproko will flag this
}
```
