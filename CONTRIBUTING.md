# Contributing to Ovie

Thank you for your interest in contributing to the Ovie programming language! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Contributing Guidelines](#contributing-guidelines)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)
- [Community](#community)

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

- Rust 1.70+ (managed via `rust-toolchain.toml`)
- Git
- Basic understanding of compiler design (helpful but not required)

### Repository Structure

```
ovie/
├── oviec/          # Ovie compiler
├── aproko/         # Assistant engine
├── ovie/           # CLI toolchain
├── docs/           # Documentation
├── examples/       # Example programs
└── spec/           # Language specification
```

## Development Setup

1. **Clone the repository**:
   ```bash
   git clone https://github.com/ovie-lang/ovie.git
   cd ovie
   ```

2. **Install Rust toolchain**:
   ```bash
   rustup toolchain install stable
   rustup override set stable
   ```

3. **Build the project**:
   ```bash
   cargo build
   ```

4. **Run tests**:
   ```bash
   cargo test
   ```

## Contributing Guidelines

### Types of Contributions

We welcome contributions in several areas:

- **Core Language**: Compiler improvements, new features
- **Aproko Engine**: Assistant functionality, analysis rules
- **Tooling**: CLI improvements, IDE integration
- **Documentation**: Guides, examples, API docs
- **Testing**: Unit tests, property-based tests, integration tests
- **Examples**: Sample programs, tutorials

### Before You Start

1. **Check existing issues**: Look for related issues or discussions
2. **Create an issue**: For new features or significant changes
3. **Discuss approach**: Get feedback before implementing large changes
4. **Follow RFCs**: Major language changes require RFC approval

### RFC Process

For significant changes to the language core:

1. Create an RFC document in `rfcs/` directory
2. Follow the RFC template
3. Submit as pull request for discussion
4. Address feedback and iterate
5. Final approval by core team

## Pull Request Process

### Before Submitting

- [ ] Code follows style guidelines
- [ ] Tests pass locally
- [ ] Documentation updated
- [ ] Commit messages are clear
- [ ] No merge conflicts

### PR Guidelines

1. **Clear title**: Describe what the PR does
2. **Detailed description**: Explain the changes and motivation
3. **Link issues**: Reference related issues
4. **Small scope**: Keep PRs focused and reviewable
5. **Tests included**: Add tests for new functionality

### Review Process

1. **Automated checks**: CI must pass
2. **Code review**: At least one maintainer approval
3. **Testing**: Comprehensive test coverage
4. **Documentation**: Updated as needed

## Coding Standards

### Rust Code Style

- Follow `rustfmt` formatting
- Use `clippy` for linting
- Prefer explicit types when clarity improves
- Document public APIs with `///` comments
- Use meaningful variable and function names

### Example:

```rust
/// Tokenizes Ovie source code into a stream of tokens.
/// 
/// # Arguments
/// 
/// * `source` - The source code string to tokenize
/// 
/// # Returns
/// 
/// A `Result` containing either a `Vec<Token>` or a `LexError`
/// 
/// # Examples
/// 
/// ```
/// let tokens = tokenize("seeAm \"hello world\"")?;
/// assert_eq!(tokens.len(), 2);
/// ```
pub fn tokenize(source: &str) -> Result<Vec<Token>, LexError> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}
```

### Commit Messages

Follow conventional commits format:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting changes
- `refactor`: Code restructuring
- `test`: Adding tests
- `chore`: Maintenance tasks

Examples:
- `feat(lexer): add support for unicode identifiers`
- `fix(parser): handle empty function bodies correctly`
- `docs(aproko): add configuration examples`

## Testing

### Test Categories

1. **Unit Tests**: Test individual functions and modules
2. **Property Tests**: Test universal properties with random inputs
3. **Integration Tests**: Test component interactions
4. **End-to-End Tests**: Test complete workflows

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_tokenize_simple_print() {
        let tokens = tokenize("seeAm \"hello\"").unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::SeeAm);
    }

    proptest! {
        #[test]
        fn test_tokenize_preserves_semantics(source in ".*") {
            if let Ok(tokens) = tokenize(&source) {
                let reconstructed = reconstruct_source(&tokens);
                // Property: tokenizing then reconstructing preserves semantics
                assert_semantically_equivalent(&source, &reconstructed);
            }
        }
    }
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_tokenize_simple_print

# Property tests with more iterations
cargo test -- --test-threads=1 PROPTEST_CASES=1000
```

## Documentation

### Types of Documentation

1. **API Documentation**: Inline code documentation
2. **User Guides**: How to use Ovie
3. **Developer Guides**: How to contribute
4. **Language Reference**: Complete language specification
5. **Examples**: Sample programs and tutorials

### Writing Documentation

- Use clear, simple language
- Include code examples
- Explain both "what" and "why"
- Keep examples up-to-date
- Consider non-technical users

### Building Documentation

```bash
# API docs
cargo doc --open

# User documentation (if using mdbook)
mdbook build docs/
mdbook serve docs/
```

## Community

### Communication Channels

- **GitHub Issues**: Bug reports, feature requests
- **GitHub Discussions**: General questions, ideas
- **Discord**: Real-time chat (link in README)
- **Forum**: Long-form discussions

### Getting Help

- Check existing documentation
- Search GitHub issues
- Ask in Discord or discussions
- Create a new issue if needed

### Mentorship

New contributors can:

- Look for "good first issue" labels
- Ask for mentorship in Discord
- Pair program with maintainers
- Start with documentation contributions

## Recognition

Contributors are recognized through:

- Contributor list in README
- Release notes acknowledgments
- Annual contributor awards
- Speaking opportunities at conferences

## License

By contributing to Ovie, you agree that your contributions will be licensed under the Apache License 2.0.

---

Thank you for contributing to Ovie! Together, we're building a programming language that's accessible, secure, and powerful for everyone.