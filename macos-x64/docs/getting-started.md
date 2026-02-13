<div align="center">
  <img src="../ovie.png" alt="Ovie Programming Language" width="120" height="120">
  
  # Getting Started with Ovie
  
  ### ✅ **COMPLETE PROGRAMMING LANGUAGE**
</div>

Welcome to Ovie! This guide will help you get started with the Ovie Programming Language, whether you're new to programming or an experienced developer.

**🎉 Status: You're learning a complete, trustworthy programming language (v2.2)!**

## What is Ovie?

Ovie is a **complete, self-hosted** programming language designed to be:
- **Trustworthy**: Enforced compiler invariants ensure correctness at every stage
- **Complete**: Full runtime environment with comprehensive standard library
- **Self-Diagnosing**: Aproko explains compiler decisions and type inference
- **Deterministic**: Reproducible builds with proven bootstrap verification
- **Accessible**: Uses natural language patterns that are easy to read and understand
- **Secure**: Built with enterprise-grade security and offline-first development
- **AI-Friendly**: Designed to work seamlessly with AI and code generation tools

## For Non-Technical Users

If you're new to programming, don't worry! Ovie was designed with you in mind.

### What Makes Ovie Different?

Instead of cryptic symbols and complex syntax, Ovie uses natural language patterns:

```ovie
// Instead of printf("Hello, World!");
seeAm "Hello, World!"

// Instead of complex variable declarations
mut name = "Alice"
mut age = 25

// Instead of confusing conditionals
if age > 18 {
    seeAm "You are an adult"
} else {
    seeAm "You are a minor"
}
```

### Your Built-in Assistant: Aproko

Ovie comes with a built-in assistant called **Aproko** that:
- Fixes common typos automatically (like "seeam" → "seeAm")
- Provides helpful suggestions when you make mistakes
- Explains what went wrong in plain English
- Helps you write better, safer code

## For Technical Users

If you're an experienced developer, here's what you need to know:

### Key Features

- **13 Core Keywords**: Minimal syntax with maximum expressiveness
- **Deterministic Builds**: Identical outputs for identical inputs, always
- **Offline-First**: No hidden network calls during compilation
- **Multi-Backend**: Compile to WASM or native code via LLVM
- **Property-Based Testing**: Built-in support for comprehensive testing
- **Enterprise Security**: No telemetry, cryptographic signing, supply chain isolation

### Quick Syntax Overview

```ovie
// Function definition
fn greet(name: string) -> string {
    return "Hello, " + name + "!"
}

// Struct definition
struct Person {
    name: string,
    age: mut u32,
}

// Enum definition
enum Status {
    Active,
    Inactive,
    Pending(string),
}

// Main function
fn main() {
    mut person = Person {
        name: "Alice",
        age: 25,
    }
    
    seeAm greet(person.name)
}
```

## Installation

### Prerequisites

- **Rust**: Version 1.70 or later (for Stage 0 compiler)
- **Git**: For cloning repositories
- **LLVM**: Optional, for native code generation

### Installing Ovie

1. **Clone the repository**:
   ```bash
   git clone https://github.com/ovie-lang/ovie.git
   cd ovie
   ```

2. **Build the toolchain**:
   ```bash
   cargo build --release
   ```

3. **Add to PATH** (optional):
   ```bash
   # Add the target/release directory to your PATH
   export PATH="$PWD/target/release:$PATH"
   ```

4. **Verify installation**:
   ```bash
   ovie --version
   ```

## Your First Ovie Program

Let's create your first Ovie program!

### Step 1: Create a New Project

```bash
ovie new hello-world
cd hello-world
```

This creates a new directory with the standard Ovie project structure:

```
hello-world/
├── ovie.toml          # Project configuration
├── src/
│   └── main.ov        # Your main program file
├── tests/             # Test files
└── .ovie/             # Ovie-specific configuration
    └── aproko.toml    # Aproko assistant settings
```

### Step 2: Write Your Program

Open `src/main.ov` and replace the contents with:

```ovie
fn main() {
    seeAm "Hello, Ovie World!"
    
    mut name = "Developer"
    seeAm "Welcome to Ovie, " + name + "!"
    
    // Let's do some math
    mut result = calculate_answer()
    seeAm "The answer is: " + result
}

fn calculate_answer() -> u32 {
    mut x = 6
    mut y = 7
    return x * y
}
```

### Step 3: Run Your Program

```bash
ovie run
```

You should see:
```
Hello, Ovie World!
Welcome to Ovie, Developer!
The answer is: 42
```

### Step 4: Understanding What Happened

1. **`seeAm`**: This is Ovie's print statement - it outputs whatever follows it
2. **`mut`**: Declares a mutable (changeable) variable
3. **Functions**: Defined with `fn`, can take parameters and return values
4. **String concatenation**: Use `+` to join strings together

## Next Steps

### For Everyone

1. **Explore Examples**: Check out the `examples/` directory for more programs
2. **Learn the Syntax**: Read the [Language Guide](language-guide.md)
3. **Meet Aproko**: Learn about your [built-in assistant](aproko.md)

### For Non-Technical Users

1. **Take it Slow**: Programming is a skill that takes time to develop
2. **Use Aproko**: Let the assistant guide you and fix your mistakes
3. **Start Simple**: Begin with basic programs and gradually add complexity
4. **Join the Community**: Ask questions in our [Discord](https://discord.gg/ovie-lang)

### For Technical Users

1. **Explore the CLI**: Learn all the [command-line tools](cli.md)
2. **Set Up Testing**: Understand [unit and property-based testing](testing.md)
3. **Configure Aproko**: Customize the [assistant for your workflow](aproko-config.md)
4. **Study Internals**: Dive into the [compiler architecture](internals.md)

## Getting Help

- **Built-in Help**: Run `ovie help` for command-line assistance
- **Aproko Assistant**: Your code will get real-time feedback and suggestions
- **Community**: Join our [Discord server](https://discord.gg/ovie-lang) for help and discussion
- **Documentation**: This documentation covers everything you need to know
- **GitHub Issues**: Report bugs or request features on [GitHub](https://github.com/ovie-lang/ovie/issues)

## What's Next?

- **[Language Guide](language-guide.md)**: Complete reference to Ovie syntax and features
- **[Project Structure](project-structure.md)**: Understanding Ovie project organization
- **[Testing](testing.md)**: Writing tests for your Ovie programs
- **[Aproko Guide](aproko.md)**: Getting the most out of your built-in assistant

Welcome to the Ovie community! We're excited to see what you'll build.

---

*Having trouble? Check our [troubleshooting guide](troubleshooting.md) or ask for help in our [Discord community](https://discord.gg/ovie-lang).*