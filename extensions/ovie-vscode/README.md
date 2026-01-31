# Ovie Programming Language Extension

Official VS Code extension for the Ovie self-hosted programming language with AI integration, web compilation, and mobile development support.

## Features

### 🎯 Language Support
- **Syntax Highlighting**: Full syntax highlighting for `.ov` files
- **IntelliSense**: Smart code completion and suggestions
- **Error Detection**: Real-time error highlighting and diagnostics
- **Code Formatting**: Automatic code formatting with Ovie's built-in formatter
- **Hover Information**: Contextual information on hover

### 🚀 Compilation & Building
- **Native Compilation**: Compile Ovie code to native executables
- **WebAssembly**: Compile to WASM for web deployment
- **Mobile Targets**: Compile for Android (ARM64) and iOS (ARM64)
- **Cross-Platform**: Support for multiple target architectures

### 🌐 Web Development
- **WASM Projects**: Create web projects with WebAssembly compilation
- **HTML Generation**: Automatic HTML wrapper generation for web apps
- **Browser Testing**: One-click browser testing and deployment
- **Progressive Web Apps**: Support for PWA development

### 📱 Mobile Development
- **Android Projects**: Generate complete Android Studio projects
- **iOS Projects**: Generate Xcode projects for iOS development
- **Device Installation**: Direct installation on connected devices
- **Simulator Support**: iOS Simulator and Android Emulator integration

### 🤖 AI Integration
- **Code Completion**: AI-powered intelligent code suggestions
- **Code Explanation**: Natural language explanations of code
- **Language Translation**: Convert code from other languages to Ovie
- **Multiple Providers**: Support for OpenAI, Anthropic, GitHub Copilot

### 🔍 Code Analysis (Aproko)
- **Static Analysis**: Comprehensive code quality analysis
- **Auto-Fix**: Automatic fixes for common issues
- **Performance Hints**: Optimization suggestions
- **Security Analysis**: Security vulnerability detection
- **Style Checking**: Code style and convention enforcement

## Getting Started

### Prerequisites
- VS Code 1.74.0 or higher
- Ovie compiler installed and in PATH

### Installation
1. Install from VS Code Marketplace: Search for "Ovie Programming Language"
2. Or install from VSIX: Download the `.vsix` file and install via VS Code

### Quick Start
1. Create a new `.ov` file
2. Start coding with full IntelliSense support
3. Use `Ctrl+Shift+B` to compile
4. Use `Ctrl+F5` to run your program

## Commands

### Compilation
- `Ovie: Compile Ovie File` - Compile current file
- `Ovie: Run Ovie File` - Compile and run current file
- `Ovie: Run Ovie Tests` - Run project tests

### Web Development
- `Ovie Web: Compile to WebAssembly` (`Ctrl+Shift+W`) - Compile to WASM
- `Ovie Web: New Web Project` - Create new web project

### Mobile Development
- `Ovie Mobile: Compile for Android` - Compile for Android
- `Ovie Mobile: Compile for iOS` - Compile for iOS
- `Ovie Mobile: New Mobile Project` - Create new mobile project

### Code Analysis
- `Ovie: Run Aproko Analysis` (`Ctrl+Shift+A`) - Analyze code quality
- `Ovie: Apply Aproko Fixes` - Apply suggested fixes
- `Ovie: Explain with Aproko` - Get code explanations

### AI Features
- `Ovie AI: AI Code Completion` - Trigger AI completion
- `Ovie AI: AI Explain Code` - Get AI code explanation
- `Ovie AI: Translate to Ovie` - Convert code to Ovie

### Project Management
- `Ovie: New Ovie Project` - Create new project
- `Ovie: Build Project` - Build entire project

## Configuration

Configure the extension in VS Code settings:

```json
{
  "ovie.compiler.path": "ovie",
  "ovie.aproko.enabled": true,
  "ovie.aproko.autoFix": false,
  "ovie.ai.enabled": true,
  "ovie.ai.provider": "github-copilot",
  "ovie.formatting.enabled": true,
  "ovie.linting.enabled": true
}
```

### Settings Reference

| Setting | Default | Description |
|---------|---------|-------------|
| `ovie.compiler.path` | `"ovie"` | Path to Ovie compiler |
| `ovie.aproko.enabled` | `true` | Enable Aproko analysis |
| `ovie.aproko.autoFix` | `false` | Auto-apply fixes |
| `ovie.ai.enabled` | `true` | Enable AI features |
| `ovie.ai.provider` | `"github-copilot"` | AI provider |
| `ovie.formatting.enabled` | `true` | Enable formatting |
| `ovie.linting.enabled` | `true` | Enable linting |

## Project Types

### Standard Project
```bash
my-project/
├── src/
│   └── main.ov
├── tests/
│   └── main.test.ov
├── ovie.toml
└── README.md
```

### Web Project
```bash
my-web-app/
├── src/
│   └── main.ov
├── web/
│   ├── index.html
│   └── styles.css
├── tests/
├── ovie.toml
└── README.md
```

### Mobile Project
```bash
my-mobile-app/
├── src/
│   └── main.ov
├── android/          # Android project files
├── ios/              # iOS project files
├── tests/
├── ovie.toml
└── README.md
```

## Themes

The extension includes custom themes optimized for Ovie:
- **Ovie Dark**: Dark theme with crown gold accents
- **Ovie Light**: Light theme with professional styling

## Snippets

Built-in code snippets for common Ovie patterns:
- `fn` - Function declaration
- `struct` - Struct definition
- `enum` - Enum definition
- `test` - Test function
- `main` - Main function
- `use` - Import statement

## Troubleshooting

### Common Issues

**Compiler not found**
- Ensure Ovie compiler is installed and in PATH
- Set `ovie.compiler.path` to full path if needed

**Extension not activating**
- Check VS Code version (requires 1.74.0+)
- Restart VS Code after installation

**Compilation errors**
- Check Ovie compiler version compatibility
- Verify project structure and `ovie.toml`

**AI features not working**
- Configure AI provider in settings
- Check API keys and authentication

## Contributing

We welcome contributions! Please see our [Contributing Guide](https://github.com/southwarridev/ovie/blob/main/CONTRIBUTING.md).

### Development Setup
1. Clone the repository
2. `cd extensions/ovie-vscode`
3. `npm install`
4. `npm run compile`
5. Press F5 to launch extension development host

## Support

- **Documentation**: [ovie-lang.org/docs](https://ovie-lang.org/docs)
- **Issues**: [GitHub Issues](https://github.com/southwarridev/ovie/issues)
- **Discussions**: [GitHub Discussions](https://github.com/southwarridev/ovie/discussions)

## License

MIT License - see [LICENSE](LICENSE) for details.

## Changelog

### 1.0.0
- Initial release
- Full language support with syntax highlighting
- Compilation support for native, web, and mobile targets
- AI integration with multiple providers
- Aproko code analysis integration
- Project templates and scaffolding
- Custom themes and snippets

---

**Enjoy coding with Ovie! 🎯**

Built with ❤️ by the Ovie team