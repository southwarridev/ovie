import * as vscode from 'vscode';
import { exec } from 'child_process';
import { promisify } from 'util';
import * as path from 'path';
import * as fs from 'fs';

const execAsync = promisify(exec);

export class OvieProjectManager {
    private context: vscode.ExtensionContext;
    private outputChannel: vscode.OutputChannel;

    constructor(context: vscode.ExtensionContext) {
        this.context = context;
        this.outputChannel = vscode.window.createOutputChannel('Ovie Project Manager');
        context.subscriptions.push(this.outputChannel);
    }

    async createNewProject(): Promise<void> {
        try {
            // Get project details from user
            const projectName = await vscode.window.showInputBox({
                prompt: 'Enter project name',
                placeHolder: 'my-ovie-project',
                validateInput: (value) => {
                    if (!value || value.trim() === '') {
                        return 'Project name cannot be empty';
                    }
                    if (!/^[a-zA-Z0-9_-]+$/.test(value)) {
                        return 'Project name can only contain letters, numbers, hyphens, and underscores';
                    }
                    return null;
                }
            });

            if (!projectName) {
                return;
            }

            const projectType = await vscode.window.showQuickPick([
                {
                    label: 'Application',
                    description: 'Executable application project',
                    detail: 'Creates a project with a main.ov file'
                },
                {
                    label: 'Library',
                    description: 'Reusable library project',
                    detail: 'Creates a library project with lib.ov'
                },
                {
                    label: 'CLI Tool',
                    description: 'Command-line tool project',
                    detail: 'Creates a CLI application with argument parsing'
                },
                {
                    label: 'Web Service',
                    description: 'Web service/API project',
                    detail: 'Creates a web service with HTTP handling'
                }
            ], {
                placeHolder: 'Select project type'
            });

            if (!projectType) {
                return;
            }

            // Select project location
            const folderUri = await vscode.window.showOpenDialog({
                canSelectFolders: true,
                canSelectFiles: false,
                canSelectMany: false,
                openLabel: 'Select Project Location'
            });

            if (!folderUri || folderUri.length === 0) {
                return;
            }

            const projectPath = path.join(folderUri[0].fsPath, projectName);

            // Check if directory already exists
            if (fs.existsSync(projectPath)) {
                const overwrite = await vscode.window.showWarningMessage(
                    `Directory "${projectName}" already exists. Do you want to continue?`,
                    'Yes',
                    'No'
                );
                
                if (overwrite !== 'Yes') {
                    return;
                }
            }

            await this.generateProject(projectPath, projectName, projectType.label.toLowerCase());

            // Open the new project
            const openProject = await vscode.window.showInformationMessage(
                `Project "${projectName}" created successfully!`,
                'Open Project',
                'Open in New Window'
            );

            if (openProject === 'Open Project') {
                await vscode.commands.executeCommand('vscode.openFolder', vscode.Uri.file(projectPath));
            } else if (openProject === 'Open in New Window') {
                await vscode.commands.executeCommand('vscode.openFolder', vscode.Uri.file(projectPath), true);
            }

        } catch (error: any) {
            vscode.window.showErrorMessage(`Failed to create project: ${error.message}`);
            console.error('Project creation error:', error);
        }
    }

    private async generateProject(projectPath: string, projectName: string, projectType: string): Promise<void> {
        const config = vscode.workspace.getConfiguration('ovie');
        const compilerPath = config.get<string>('compiler.path', 'ovie');

        this.outputChannel.clear();
        this.outputChannel.show(true);
        this.outputChannel.appendLine(`Creating ${projectType} project: ${projectName}`);
        this.outputChannel.appendLine(`Location: ${projectPath}`);

        try {
            // Use Ovie's project generator if available
            const { stdout, stderr } = await execAsync(
                `"${compilerPath}" new ${projectType} "${projectName}"`,
                { cwd: path.dirname(projectPath) }
            );

            if (stdout) {
                this.outputChannel.appendLine(stdout);
            }

            if (stderr) {
                this.outputChannel.appendLine('STDERR:');
                this.outputChannel.appendLine(stderr);
            }

        } catch (error: any) {
            // Fallback to manual project generation
            this.outputChannel.appendLine('Ovie project generator not available, creating manually...');
            await this.createProjectManually(projectPath, projectName, projectType);
        }
    }

    private async createProjectManually(projectPath: string, projectName: string, projectType: string): Promise<void> {
        // Create project directory structure
        fs.mkdirSync(projectPath, { recursive: true });
        fs.mkdirSync(path.join(projectPath, 'src'), { recursive: true });
        fs.mkdirSync(path.join(projectPath, 'tests'), { recursive: true });

        // Create ovie.toml
        const ovieToml = this.generateOvieToml(projectName, projectType);
        fs.writeFileSync(path.join(projectPath, 'ovie.toml'), ovieToml);

        // Create main source file
        const mainFile = this.generateMainFile(projectType);
        const mainFileName = projectType === 'library' ? 'lib.ov' : 'main.ov';
        fs.writeFileSync(path.join(projectPath, 'src', mainFileName), mainFile);

        // Create test file
        const testFile = this.generateTestFile(projectType);
        fs.writeFileSync(path.join(projectPath, 'tests', 'main.test.ov'), testFile);

        // Create README.md
        const readme = this.generateReadme(projectName, projectType);
        fs.writeFileSync(path.join(projectPath, 'README.md'), readme);

        // Create .gitignore
        const gitignore = this.generateGitignore();
        fs.writeFileSync(path.join(projectPath, '.gitignore'), gitignore);

        this.outputChannel.appendLine('✅ Project created successfully');
    }

    private generateOvieToml(projectName: string, projectType: string): string {
        const isLibrary = projectType === 'library';
        
        return `[package]
name = "${projectName}"
version = "0.1.0"
description = "A new Ovie ${projectType} project"
authors = ["Your Name <your.email@example.com>"]
license = "MIT"
edition = "2026"

${isLibrary ? '[lib]' : '[bin]'}
${isLibrary ? 'name = "' + projectName + '"' : 'name = "' + projectName + '"'}
${isLibrary ? 'path = "src/lib.ov"' : 'path = "src/main.ov"'}

[dependencies]
# Add your dependencies here
# example_lib = "1.0.0"

[dev-dependencies]
# Add development dependencies here

[features]
default = []

[profile.dev]
debug = true
optimization = "none"

[profile.release]
debug = false
optimization = "speed"

[aproko]
enabled = true
auto_fix = true
categories = ["syntax", "logic", "performance", "security", "correctness", "style"]
`;
    }

    private generateMainFile(projectType: string): string {
        switch (projectType) {
            case 'library':
                return `// ${new Date().getFullYear()} Ovie Library
// This is the main library file

/// Main library module
pub mod lib {
    /// A simple greeting function
    pub fn greet(name: string) -> string {
        return "Hello, " + name + "!"
    }
    
    /// Add two numbers together
    pub fn add(a: i32, b: i32) -> i32 {
        return a + b
    }
}

// Re-export main functions
pub use lib::*;
`;

            case 'cli tool':
                return `// ${new Date().getFullYear()} Ovie CLI Tool
use std::cli

fn main() {
    mut args = cli::args()
    
    if args.length() < 2 {
        seeAm "Usage: " + args[0] + " <command> [options]"
        seeAm ""
        seeAm "Commands:"
        seeAm "  help    Show this help message"
        seeAm "  version Show version information"
        return
    }
    
    mut command = args[1]
    
    match command {
        "help" => show_help(),
        "version" => show_version(),
        _ => {
            seeAm "Unknown command: " + command
            seeAm "Use 'help' to see available commands"
        }
    }
}

fn show_help() {
    seeAm "Ovie CLI Tool"
    seeAm "A command-line tool built with Ovie"
    seeAm ""
    seeAm "Usage: [command] [options]"
    seeAm ""
    seeAm "Commands:"
    seeAm "  help    Show this help message"
    seeAm "  version Show version information"
}

fn show_version() {
    seeAm "Ovie CLI Tool v0.1.0"
    seeAm "Built with Ovie programming language"
}
`;

            case 'web service':
                return `// ${new Date().getFullYear()} Ovie Web Service
use std::http
use std::json

fn main() {
    mut server = http::Server::new("127.0.0.1", 8080)
    
    seeAm "Starting web service on http://127.0.0.1:8080"
    
    // Define routes
    server.get("/", handle_root)
    server.get("/health", handle_health)
    server.post("/api/data", handle_data)
    
    // Start server
    server.listen()
}

fn handle_root(request: http::Request) -> http::Response {
    return http::Response::ok("Welcome to Ovie Web Service!")
}

fn handle_health(request: http::Request) -> http::Response {
    mut health_data = json::object()
    health_data.set("status", "healthy")
    health_data.set("timestamp", std::time::now())
    
    return http::Response::json(health_data)
}

fn handle_data(request: http::Request) -> http::Response {
    mut body = request.body()
    
    if body.is_empty() {
        return http::Response::bad_request("Request body is required")
    }
    
    // Process the data
    mut response_data = json::object()
    response_data.set("message", "Data received successfully")
    response_data.set("received_at", std::time::now())
    
    return http::Response::json(response_data)
}
`;

            default: // application
                return `// ${new Date().getFullYear()} Ovie Application
// This is the main entry point for your Ovie application

fn main() {
    seeAm "Hello, World!"
    seeAm "Welcome to your new Ovie application!"
    
    // Your application logic goes here
    mut name = get_user_name()
    greet_user(name)
    
    // Example of basic functionality
    demonstrate_features()
}

fn get_user_name() -> string {
    seeAm "What's your name?"
    // In a real application, you would get input from the user
    // For now, we'll use a default name
    return "Ovie Developer"
}

fn greet_user(name: string) {
    seeAm "Nice to meet you, " + name + "!"
    seeAm "Let's build something amazing with Ovie!"
}

fn demonstrate_features() {
    seeAm ""
    seeAm "=== Ovie Features Demo ==="
    
    // Variables and mutability
    mut counter = 0
    seeAm "Counter starts at: " + counter
    
    counter = counter + 1
    seeAm "Counter after increment: " + counter
    
    // Control flow
    if counter > 0 {
        seeAm "Counter is positive!"
    }
    
    // Loops
    seeAm "Counting to 3:"
    for i in 1..4 {
        seeAm "  " + i
    }
    
    seeAm "Demo complete!"
}
`;
        }
    }

    private generateTestFile(projectType: string): string {
        const isLibrary = projectType === 'library';
        
        return `// Test file for ${projectType} project
${isLibrary ? 'use super::*;' : ''}

test "basic functionality works" {
    ${isLibrary ? `
    // Test library functions
    mut result = add(2, 3)
    assert_equal(result, 5)
    
    mut greeting = greet("World")
    assert_equal(greeting, "Hello, World!")
    ` : `
    // Test application functionality
    mut name = "Test User"
    // Add your application tests here
    assert_true(name.length() > 0)
    `}
}

${isLibrary ? `
test "greet function returns correct format" {
    mut result = greet("Alice")
    assert_true(result.starts_with("Hello, "))
    assert_true(result.ends_with("!"))
}

test "add function handles different numbers" {
    assert_equal(add(0, 0), 0)
    assert_equal(add(-1, 1), 0)
    assert_equal(add(10, 20), 30)
}
` : `
test "user name handling" {
    mut test_name = "Test User"
    assert_true(test_name.length() > 0)
    assert_false(test_name.is_empty())
}

test "counter functionality" {
    mut counter = 0
    counter = counter + 1
    assert_equal(counter, 1)
}
`}

test "error handling works" {
    // Add error handling tests here
    assert_true(true) // Placeholder test
}
`;
    }

    private generateReadme(projectName: string, projectType: string): string {
        return `# ${projectName}

A ${projectType} project built with the Ovie programming language.

## Description

This is a new Ovie ${projectType} project. Ovie is a self-hosted programming language with natural language syntax that makes programming more accessible and AI-friendly.

## Getting Started

### Prerequisites

- Ovie compiler installed ([Installation Guide](https://ovie-lang.org/docs/installation.html))

### Building

\`\`\`bash
# Build the project
ovie build

# Run the project
ovie run
\`\`\`

### Testing

\`\`\`bash
# Run tests
ovie test
\`\`\`

### Development

\`\`\`bash
# Check code with Aproko (Ovie's assistant)
ovie aproko

# Format code
ovie fmt

# Watch for changes and rebuild
ovie watch
\`\`\`

## Project Structure

\`\`\`
${projectName}/
├── src/
│   ${projectType === 'library' ? '├── lib.ov          # Main library file' : '├── main.ov         # Main application file'}
├── tests/
│   └── main.test.ov    # Test files
├── ovie.toml           # Project configuration
├── README.md           # This file
└── .gitignore          # Git ignore rules
\`\`\`

## Features

- 🎯 **Natural Syntax**: Ovie uses natural language patterns
- 🤖 **AI-Friendly**: Built for seamless AI integration
- 🔒 **Memory Safe**: Ownership system prevents common bugs
- ⚡ **Fast**: Compiles to efficient native code
- 🧪 **Testing**: Built-in testing framework
- 📊 **Analysis**: Aproko assistant for code quality

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: \`ovie test\`
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Learn More

- [Ovie Documentation](https://ovie-lang.org/docs/)
- [Language Guide](https://ovie-lang.org/docs/language-guide.html)
- [Examples](https://ovie-lang.org/examples/)
- [Community](https://github.com/southwarridev/ovie/discussions)

---

Built with ❤️ using [Ovie Programming Language](https://ovie-lang.org)
`;
    }

    private generateGitignore(): string {
        return `# Ovie build artifacts
/target/
/build/
*.exe
*.dll
*.so
*.dylib

# Ovie cache
/.ovie/cache/
/.ovie/build/

# IDE files
.vscode/
.idea/
*.swp
*.swo
*~

# OS files
.DS_Store
Thumbs.db

# Logs
*.log

# Temporary files
*.tmp
*.temp

# Dependencies
/vendor/
/node_modules/

# Environment files
.env
.env.local
.env.production

# Test artifacts
/test-results/
/coverage/
`;
    }

    async buildProject(): Promise<boolean> {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders || workspaceFolders.length === 0) {
            vscode.window.showErrorMessage('No workspace folder found');
            return false;
        }

        const config = vscode.workspace.getConfiguration('ovie');
        const compilerPath = config.get<string>('compiler.path', 'ovie');
        const cwd = workspaceFolders[0].uri.fsPath;

        // Check for ovie.toml
        const configPath = path.join(cwd, 'ovie.toml');
        if (!fs.existsSync(configPath)) {
            const createProject = await vscode.window.showErrorMessage(
                'No ovie.toml found. This doesn\'t appear to be an Ovie project.',
                'Create New Project',
                'Initialize Here'
            );
            
            if (createProject === 'Create New Project') {
                await this.createNewProject();
            } else if (createProject === 'Initialize Here') {
                await this.initializeProject(cwd);
            }
            return false;
        }

        this.outputChannel.clear();
        this.outputChannel.show(true);
        this.outputChannel.appendLine('Building Ovie project...');

        try {
            const { stdout, stderr } = await execAsync(`"${compilerPath}" build`, { cwd });

            if (stdout) {
                this.outputChannel.appendLine(stdout);
            }

            if (stderr) {
                this.outputChannel.appendLine('STDERR:');
                this.outputChannel.appendLine(stderr);
            }

            this.outputChannel.appendLine('✅ Build completed successfully');
            vscode.window.showInformationMessage('Project built successfully');
            return true;

        } catch (error: any) {
            this.outputChannel.appendLine('❌ Build failed');
            this.outputChannel.appendLine(error.message);
            
            if (error.stdout) {
                this.outputChannel.appendLine(error.stdout);
            }
            
            if (error.stderr) {
                this.outputChannel.appendLine('STDERR:');
                this.outputChannel.appendLine(error.stderr);
            }

            vscode.window.showErrorMessage('Build failed', 'View Output').then(selection => {
                if (selection === 'View Output') {
                    this.outputChannel.show();
                }
            });
            return false;
        }
    }

    private async initializeProject(projectPath: string): Promise<void> {
        const projectName = path.basename(projectPath);
        
        const projectType = await vscode.window.showQuickPick([
            'application',
            'library',
            'cli tool',
            'web service'
        ], {
            placeHolder: 'Select project type'
        });

        if (!projectType) {
            return;
        }

        await this.createProjectManually(projectPath, projectName, projectType);
        
        vscode.window.showInformationMessage(
            `Ovie project initialized in ${projectName}`,
            'Reload Window'
        ).then(selection => {
            if (selection === 'Reload Window') {
                vscode.commands.executeCommand('workbench.action.reloadWindow');
            }
        });
    }

    async getProjectInfo(): Promise<any> {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders || workspaceFolders.length === 0) {
            return null;
        }

        const cwd = workspaceFolders[0].uri.fsPath;
        const configPath = path.join(cwd, 'ovie.toml');

        if (!fs.existsSync(configPath)) {
            return null;
        }

        try {
            const configContent = fs.readFileSync(configPath, 'utf8');
            // Basic TOML parsing (in a real implementation, use a proper TOML parser)
            const nameMatch = configContent.match(/name\s*=\s*"([^"]+)"/);
            const versionMatch = configContent.match(/version\s*=\s*"([^"]+)"/);
            const descriptionMatch = configContent.match(/description\s*=\s*"([^"]+)"/);

            return {
                name: nameMatch ? nameMatch[1] : path.basename(cwd),
                version: versionMatch ? versionMatch[1] : '0.1.0',
                description: descriptionMatch ? descriptionMatch[1] : '',
                path: cwd,
                configPath: configPath
            };
        } catch (error) {
            console.error('Failed to parse project info:', error);
            return null;
        }
    }

    async showProjectInfo(): Promise<void> {
        const projectInfo = await this.getProjectInfo();
        
        if (!projectInfo) {
            vscode.window.showInformationMessage('No Ovie project found in current workspace');
            return;
        }

        const info = `
Project: ${projectInfo.name}
Version: ${projectInfo.version}
Description: ${projectInfo.description}
Path: ${projectInfo.path}
        `.trim();

        vscode.window.showInformationMessage(info, 'Open ovie.toml').then(selection => {
            if (selection === 'Open ovie.toml') {
                vscode.workspace.openTextDocument(projectInfo.configPath).then(doc => {
                    vscode.window.showTextDocument(doc);
                });
            }
        });
    }
}