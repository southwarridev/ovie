# Chapter 11: Community and Contribution

## The Ovie Community

Ovie is open source and community-driven. Contributions are welcome from everyone.

- GitHub: [https://github.com/southwarridev/ovie](https://github.com/southwarridev/ovie)
- GitLab: [https://gitlab.com/ovie1/ovie](https://gitlab.com/ovie1/ovie)
- Issues: Report bugs and request features on either platform
- Discussions: GitHub Discussions for questions and ideas

## Contributing to Ovie

### Development Setup

```bash
git clone https://github.com/southwarridev/ovie.git
cd ovie
make dev-setup
make dev
```

`make dev` runs: clean → build → test. All three must pass before submitting a PR.

### Areas for Contribution

- Core language features and bug fixes
- Standard library expansion
- Aproko reasoning rules
- Documentation and examples
- Testing and benchmarks
- IDE integration
- This book

### Code Review Process

1. Fork the repository
2. Create a feature branch
3. Write code and tests
4. Run `make dev` — all tests must pass
5. Run `oviec analyze` — no new errors
6. Submit a pull request with a clear description

### Documentation Standards

All exported functions must have doc comments. The compiler enforces this. Format:

```ovie
/// Brief one-line summary
///
/// Longer description if needed.
///
/// # Parameters
/// - param_name: Description of the parameter
///
/// # Returns
/// Description of the return value
///
/// # Examples
/// ```ovie
/// mut result = my_function(42)
/// seeAm result
/// ```
export fn my_function(x: Number) -> Number {
    return x * 2
}
```

## Package Development

Create packages that others can use:

1. Initialize: `ovie init my-package`
2. Write your module with exported functions
3. Document everything (required)
4. Test thoroughly
5. Publish by sharing the repository URL

Others can use your package:

```toml
[dependencies]
my-package = { path = "./my-package" }
# or
my-package = { git = "https://github.com/you/my-package" }
```

## Community Guidelines

- Be respectful and constructive
- Help newcomers — everyone was a beginner once
- Report bugs with minimal reproducible examples
- Discuss breaking changes before implementing them
- Credit contributors in commit messages

## Getting Help

- Check the docs first: `ovie book serve`
- Search existing issues before opening a new one
- Include your Ovie version (`ovie --version`) in bug reports
- Share the smallest code that reproduces the problem
