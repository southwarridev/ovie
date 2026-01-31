<div align="center">
  <img src="../ovie.png" alt="Ovie Programming Language" width="120" height="120">
  
  # Ovie Programming Language Examples
  
  ### ✅ **SELF-HOSTED PROGRAMMING LANGUAGE**
</div>

This directory contains comprehensive examples demonstrating all major features of the Ovie Programming Language. The examples are organized by complexity and use case, making it easy to learn Ovie progressively.

**🎉 Status: All examples work with the self-hosted Ovie compiler! (January 30, 2026)**

## Table of Contents

### 🚀 Basic Examples
- **[hello.ov](hello.ov)** - The classic "Hello, World!" program
- **[variables.ov](variables.ov)** - Variables, mutability, and basic types
- **[math.ov](math.ov)** - Mathematical operations and functions
- **[control_flow.ov](control_flow.ov)** - Conditionals and loops

### 📊 Data Structures
- **[struct.ov](struct.ov)** - Structs and nested data structures
- **[enums.ov](enums.ov)** - Enumerations and pattern matching
- **[collections.ov](collections.ov)** - Arrays and collections

### 🔧 Functions and Control
- **[functions.ov](functions.ov)** - Function definitions and calls
- **[recursion.ov](recursion.ov)** - Recursive algorithms
- **[higher_order.ov](higher_order.ov)** - Higher-order functions

### 🛡️ Error Handling
- **[errors.ov](errors.ov)** - Error handling and edge cases
- **[result_types.ov](result_types.ov)** - Result and Option types
- **[validation.ov](validation.ov)** - Input validation patterns

### 🏢 Enterprise Use Cases
- **[calculator.ov](calculator.ov)** - Simple calculator application
- **[bank_account.ov](bank_account.ov)** - Banking system simulation
- **[inventory.ov](inventory.ov)** - Inventory management system
- **[employee_management.ov](employee_management.ov)** - HR system example
- **[config_parser.ov](config_parser.ov)** - Configuration file parser

### 🤖 AI-Friendly Patterns
- **[natural_language.ov](natural_language.ov)** - Natural language coding patterns
- **[ai_training_data.ov](ai_training_data.ov)** - Code suitable for AI training
- **[llm_friendly.ov](llm_friendly.ov)** - LLM-optimized code examples
- **[code_generation.ov](code_generation.ov)** - Patterns for AI code generation

### 🔄 Self-Hosting Examples
- **[lexer_demo.ov](lexer_demo.ov)** - Simple lexer implementation
- **[parser_demo.ov](parser_demo.ov)** - Basic parser example
- **[ast_builder.ov](ast_builder.ov)** - AST construction example
- **[compiler_stages.ov](compiler_stages.ov)** - Compilation pipeline demo

### 🎯 Advanced Examples
- **[memory_safety.ov](memory_safety.ov)** - Memory safety demonstrations
- **[performance.ov](performance.ov)** - Performance optimization examples
- **[testing.ov](testing.ov)** - Unit and property-based testing
- **[aproko_integration.ov](aproko_integration.ov)** - Aproko assistant usage

### 🌐 Real-World Applications
- **[web_server.ov](web_server.ov)** - Simple web server (conceptual)
- **[cli_tool.ov](cli_tool.ov)** - Command-line application
- **[data_processing.ov](data_processing.ov)** - Data analysis pipeline
- **[game_logic.ov](game_logic.ov)** - Simple game implementation

## Running the Examples

### Prerequisites

Make sure you have Ovie installed:

```bash
ovie --version
```

### Running Individual Examples

```bash
# Run a specific example
ovie run examples/hello.ov
ovie run examples/calculator.ov

# Run with different backends
ovie run --backend=wasm examples/math.ov
ovie run --backend=llvm examples/performance.ov
```

### Running All Examples

```bash
# Create a test project and copy examples
ovie new example-test
cd example-test

# Copy any example to src/main.ov and run
cp ../examples/calculator.ov src/main.ov
ovie build
ovie run
```

### Testing Examples

```bash
# Run Aproko analysis on examples
ovie aproko examples/

# Check for compilation errors
ovie check examples/
```

## Learning Path

### For Beginners
1. Start with **hello.ov** and **variables.ov**
2. Learn control flow with **control_flow.ov**
3. Understand data structures with **struct.ov** and **enums.ov**
4. Practice with **calculator.ov**

### For Experienced Developers
1. Review **functions.ov** and **error_handling.ov**
2. Explore enterprise examples like **bank_account.ov**
3. Study **memory_safety.ov** and **performance.ov**
4. Examine self-hosting examples

### For AI/LLM Developers
1. Study **natural_language.ov** patterns
2. Review **ai_training_data.ov** for training examples
3. Explore **llm_friendly.ov** for integration patterns
4. Use **code_generation.ov** for generation templates

## Contributing Examples

We welcome contributions! To add a new example:

1. Create a well-commented `.ov` file
2. Include a brief description at the top
3. Demonstrate clear, idiomatic Ovie code
4. Add it to the appropriate category in this README
5. Test that it compiles and runs correctly

### Example Template

```ovie
// Brief description of what this example demonstrates
// Key concepts: concept1, concept2, concept3

// Your example code here
fn main() {
    seeAm "Example output"
}
```

## Example Categories Explained

### Basic Examples
Simple programs demonstrating fundamental language features. Perfect for learning Ovie syntax and core concepts.

### Enterprise Use Cases
Real-world applications showing how Ovie can be used in business contexts. These examples demonstrate best practices for production code.

### AI-Friendly Patterns
Examples specifically designed to work well with AI systems and LLMs. These show natural language patterns and clear, predictable code structures.

### Self-Hosting Examples
Demonstrations of compiler components written in Ovie itself. These examples show the progression toward full self-hosting.

## Getting Help

- **Documentation**: See the [main documentation](../docs/README.md)
- **Discord**: Join our [community server](https://discord.gg/ovie-lang)
- **GitHub**: Report issues or ask questions on [GitHub](https://github.com/ovie-lang/ovie)

---

*These examples cover the complete Ovie implementation including self-hosting capabilities. All examples are tested and verified to work with the Ovie compiler written in Ovie itself!*