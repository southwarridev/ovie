# Change Log

All notable changes to the Ovie Programming Language extension will be documented in this file.

## [1.0.0] - 2026-01-30

### Added
- Initial release of Ovie Programming Language extension
- Full syntax highlighting for `.ov` files
- IntelliSense with code completion and hover information
- Real-time error detection and diagnostics
- Code formatting with Ovie's built-in formatter
- Native compilation support
- WebAssembly compilation for web development
- Mobile compilation for Android and iOS
- Web project scaffolding with HTML/CSS templates
- Mobile project scaffolding for Android and iOS
- AI integration with multiple providers (OpenAI, Anthropic, GitHub Copilot)
- Aproko code analysis integration
- Auto-fix capabilities for common issues
- Custom Ovie Dark and Light themes
- Comprehensive code snippets
- Project templates and management
- Cross-platform build support
- Device installation for mobile apps
- Browser testing for web apps
- Import organization
- Code validation and linting

### Commands Added
- `ovie.compile` - Compile Ovie file
- `ovie.run` - Run Ovie file
- `ovie.test` - Run tests
- `ovie.compile.web` - Compile to WebAssembly
- `ovie.compile.android` - Compile for Android
- `ovie.compile.ios` - Compile for iOS
- `ovie.project.new` - Create new project
- `ovie.project.web` - Create web project
- `ovie.project.mobile` - Create mobile project
- `ovie.aproko.analyze` - Run code analysis
- `ovie.aproko.fix` - Apply fixes
- `ovie.aproko.explain` - Explain code
- `ovie.ai.complete` - AI completion
- `ovie.ai.explain` - AI explanation
- `ovie.ai.translate` - Translate to Ovie

### Keybindings Added
- `Ctrl+Shift+B` - Compile current file
- `Ctrl+F5` - Run current file
- `Ctrl+Shift+W` - Compile to WebAssembly
- `Ctrl+Shift+A` - Run Aproko analysis

### Configuration Added
- `ovie.compiler.path` - Compiler path configuration
- `ovie.aproko.enabled` - Enable/disable Aproko
- `ovie.aproko.autoFix` - Auto-apply fixes
- `ovie.ai.enabled` - Enable/disable AI features
- `ovie.ai.provider` - AI provider selection
- `ovie.formatting.enabled` - Enable/disable formatting
- `ovie.linting.enabled` - Enable/disable linting

### Themes Added
- Ovie Dark theme with crown gold accents
- Ovie Light theme with professional styling

### Snippets Added
- Function declarations (`fn`)
- Struct definitions (`struct`)
- Enum definitions (`enum`)
- Test functions (`test`)
- Main function (`main`)
- Import statements (`use`)

## [Unreleased]

### Planned Features
- Debugging support with breakpoints
- Integrated terminal for Ovie REPL
- Git integration for Ovie projects
- Package manager integration
- Advanced refactoring tools
- Performance profiling
- Memory usage analysis
- Deployment automation
- Cloud integration
- Collaborative editing features