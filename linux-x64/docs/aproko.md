# Aproko Assistant Guide

Aproko is Ovie's built-in assistant engine that provides real-time guidance, auto-correction, and code analysis. Think of Aproko as your personal programming mentor that helps you write better, safer code.

## What is Aproko?

**Aproko** (pronounced "ah-PROH-koh") is a Yoruba word meaning "gossip" or "one who tells stories." In the context of Ovie, Aproko is your code's storyteller - it tells you what's happening in your code, what could be improved, and what might go wrong.

## Core Features

### 1. Safe Auto-Correction

Aproko automatically fixes common typos and syntax errors:

```ovie
// You type:
seeam "Hello World"

// Aproko corrects to:
seeAm "Hello World"
// ✓ Auto-corrected: 'seeam' → 'seeAm'
```

**Safety First**: Aproko only makes corrections when it's 100% certain about the intent. Ambiguous cases are flagged for your review.

### 2. Real-Time Analysis

Aproko analyzes your code across six categories:

#### Syntax Analysis
- Grammar compliance
- Keyword usage
- Punctuation and formatting

#### Logic Analysis
- Control flow validation
- Unreachable code detection
- Infinite loop warnings

#### Performance Analysis
- Algorithmic complexity warnings
- Memory usage optimization
- Inefficient patterns detection

#### Security Analysis
- Unsafe operation detection
- Input validation reminders
- Potential vulnerability warnings

#### Correctness Analysis
- Type safety validation
- Ownership rule enforcement
- State transition verification

#### Style Analysis
- Code formatting consistency
- Naming convention compliance
- Best practice recommendations

### 3. Intelligent Feedback

Aproko provides context-aware suggestions:

```ovie
// Your code:
fn calculate_total(items) {
    mut total = 0
    for item in items {
        total = total + item.price
    }
    return total
}

// Aproko suggests:
// ℹ️  Consider adding type annotations for better clarity:
//    fn calculate_total(items: &[Item]) -> f64 {
//
// ⚡ Performance: Consider using iterator methods:
//    return items.iter().map(|item| item.price).sum()
//
// 🔒 Safety: Ensure items is not empty before processing
```

## Configuration

### Basic Configuration

Aproko is configured through `.ovie/aproko.toml` in your project root:

```toml
[aproko]
# Enable/disable Aproko entirely
enabled = true

# Auto-correction settings
[aproko.correction]
# Automatically fix safe typos
auto_fix_typos = true
# Show what was corrected
show_corrections = true
# Require confirmation for ambiguous corrections
confirm_ambiguous = true

# Analysis categories
[aproko.analysis]
syntax = true
logic = true
performance = true
security = true
correctness = true
style = true

# Feedback settings
[aproko.feedback]
# Verbosity level: "minimal", "normal", "detailed"
verbosity = "normal"
# Show suggestions inline in code
inline_suggestions = true
# Generate AI-friendly structured feedback
ai_friendly = false
```

### Advanced Configuration

#### Custom Rules

```toml
[aproko.rules]
# Custom naming conventions
function_naming = "snake_case"  # snake_case, camelCase, PascalCase
variable_naming = "snake_case"
constant_naming = "SCREAMING_SNAKE_CASE"

# Line length limits
max_line_length = 100
max_function_length = 50

# Complexity limits
max_cyclomatic_complexity = 10
max_nesting_depth = 4
```

#### Analysis Sensitivity

```toml
[aproko.sensitivity]
# How aggressive should analysis be?
# "strict", "normal", "relaxed"
performance = "normal"
security = "strict"
style = "relaxed"

# Minimum confidence for suggestions (0.0 - 1.0)
suggestion_threshold = 0.7
```

#### Integration Settings

```toml
[aproko.integration]
# IDE integration
show_in_editor = true
highlight_issues = true

# CI/CD integration
fail_on_errors = true
fail_on_warnings = false

# AI/LLM integration
generate_training_data = false
structured_output = false
```

## Working with Aproko

### Understanding Feedback

Aproko uses a consistent format for all feedback:

```
[CATEGORY] [SEVERITY] [LOCATION]: [MESSAGE]
  Suggestion: [SPECIFIC_RECOMMENDATION]
  Example: [CODE_EXAMPLE]
  Learn more: [DOCUMENTATION_LINK]
```

Example:
```
[PERFORMANCE] [WARNING] line 15: Inefficient string concatenation in loop
  Suggestion: Use string builder or collect into vector first
  Example: 
    // Instead of:
    for item in items { result = result + item }
    // Try:
    result = items.join("")
  Learn more: docs.ovie-lang.org/performance/strings
```

### Severity Levels

- **🔴 ERROR**: Must be fixed before compilation
- **🟡 WARNING**: Should be addressed for better code quality
- **🔵 INFO**: Helpful suggestions and tips
- **🟢 HINT**: Style and convention recommendations

### Interactive Mode

Run Aproko interactively to get detailed explanations:

```bash
ovie aproko --interactive src/main.ov
```

This opens an interactive session where you can:
- Ask questions about specific suggestions
- Get detailed explanations of rules
- See examples of better code patterns
- Configure settings on the fly

## Common Scenarios

### Learning Mode

When you're learning Ovie, enable detailed feedback:

```toml
[aproko.feedback]
verbosity = "detailed"
show_examples = true
explain_rules = true
```

### Production Mode

For production code, focus on errors and critical warnings:

```toml
[aproko.feedback]
verbosity = "minimal"
show_only_errors = true

[aproko.analysis]
performance = true
security = true
correctness = true
style = false  # Disable style checks in production
```

### AI Development Mode

When working with AI/LLM systems:

```toml
[aproko.integration]
ai_friendly = true
structured_output = true
generate_training_data = true

[aproko.feedback]
format = "json"  # Machine-readable format
include_confidence = true
include_suggestions = true
```

## Aproko Commands

### CLI Commands

```bash
# Run Aproko analysis on specific files
ovie aproko src/main.ov

# Run on entire project
ovie aproko .

# Interactive mode
ovie aproko --interactive src/

# Generate report
ovie aproko --report=json src/ > analysis.json

# Check specific categories only
ovie aproko --categories=security,performance src/

# Fix auto-correctable issues
ovie aproko --fix src/main.ov
```

### Integration with Build Process

```bash
# Run Aproko as part of build
ovie build --with-aproko

# Fail build on Aproko warnings
ovie build --aproko-strict

# Generate Aproko report during CI
ovie aproko --ci --format=junit src/ > aproko-report.xml
```

## Customizing Aproko

### Writing Custom Rules

You can extend Aproko with custom analysis rules:

```ovie
// .ovie/aproko_rules/custom.ov
fn check_function_naming(function_name: string) -> AnalysisResult {
    if !function_name.starts_with("do_") {
        return AnalysisResult.warning(
            "Functions should start with 'do_' in this project",
            "Consider renaming to 'do_" + function_name + "'"
        )
    }
    return AnalysisResult.ok()
}
```

### Team Configuration

Share Aproko configuration across your team:

```toml
# .ovie/aproko.toml
[aproko.team]
# Enforce team-wide rules
enforce_naming_conventions = true
require_documentation = true
max_function_complexity = 8

# Shared rule sets
rule_sets = ["team_security", "team_performance"]
```

## AI/LLM Integration

### Structured Feedback for AI

When `ai_friendly = true`, Aproko generates structured feedback suitable for AI training:

```json
{
  "analysis": {
    "file": "src/main.ov",
    "issues": [
      {
        "category": "performance",
        "severity": "warning",
        "line": 15,
        "column": 8,
        "rule": "inefficient_string_concat",
        "message": "Inefficient string concatenation in loop",
        "suggestion": "Use string builder pattern",
        "confidence": 0.95,
        "example": {
          "before": "for item in items { result = result + item }",
          "after": "result = items.join(\"\")"
        }
      }
    ]
  }
}
```

### Training Data Generation

Enable training data generation for AI model improvement:

```toml
[aproko.ai]
generate_training_data = true
training_data_path = ".ovie/training_data/"
include_corrections = true
include_suggestions = true
anonymize_data = true
```

## Troubleshooting

### Common Issues

**Aproko is too noisy**:
```toml
[aproko.feedback]
verbosity = "minimal"
show_only_errors = true
```

**Aproko missed an obvious issue**:
```toml
[aproko.sensitivity]
suggestion_threshold = 0.5  # Lower threshold for more suggestions
```

**False positives in analysis**:
```toml
[aproko.rules]
# Disable specific rules
disable = ["rule_name_here"]
```

### Performance Tuning

For large codebases:

```toml
[aproko.performance]
# Analyze only changed files
incremental_analysis = true
# Cache analysis results
cache_results = true
# Parallel analysis
parallel_workers = 4
```

## Best Practices

1. **Start with default settings** and adjust based on your needs
2. **Enable auto-correction** for typos but review ambiguous changes
3. **Use different configurations** for development vs. production
4. **Share team configurations** to maintain consistency
5. **Regularly update** Aproko rules and configurations
6. **Provide feedback** to improve Aproko's suggestions

## Next Steps

- **[Aproko Configuration Reference](aproko-config.md)**: Detailed configuration options
- **[Analysis Categories](aproko-analysis.md)**: Deep dive into each analysis type
- **[CLI Reference](cli.md)**: Command-line tools and options
- **[AI Integration](ai-integration.md)**: Using Aproko with AI systems

---

*Aproko is continuously learning and improving. Your feedback helps make it better for everyone!*