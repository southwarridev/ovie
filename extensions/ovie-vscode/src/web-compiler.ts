import * as vscode from 'vscode';

export class OvieWebCompiler {
    private context: vscode.ExtensionContext;
    private outputChannel: vscode.OutputChannel;

    constructor(context: vscode.ExtensionContext) {
        this.context = context;
        this.outputChannel = vscode.window.createOutputChannel('Ovie Web Compiler');
        context.subscriptions.push(this.outputChannel);
    }

    async compileToWasm(document: vscode.TextDocument): Promise<boolean> {
        const config = vscode.workspace.getConfiguration('ovie');
        const compilerPath = config.get<string>('compiler.path', 'ovie');
        
        this.outputChannel.clear();
        this.outputChannel.show(true);
        this.outputChannel.appendLine(`Compiling ${document.fileName} to WebAssembly...`);

        try {
            const workspaceFolder = vscode.workspace.getWorkspaceFolder(document.uri);
            const cwd = workspaceFolder?.uri.fsPath || require('path').dirname(document.fileName);

            // Save document if it has unsaved changes
            if (document.isDirty) {
                await document.save();
            }

            // Use Ovie's WASM compilation target
            const { exec } = require('child_process');
            const { promisify } = require('util');
            const execAsync = promisify(exec);

            const { stdout, stderr } = await execAsync(
                `"${compilerPath}" compile --target=wasm "${document.fileName}"`,
                { cwd }
            );

            if (stdout) {
                this.outputChannel.appendLine('STDOUT:');
                this.outputChannel.appendLine(stdout);
            }

            if (stderr) {
                this.outputChannel.appendLine('STDERR:');
                this.outputChannel.appendLine(stderr);
            }

            this.outputChannel.appendLine('✅ WebAssembly compilation completed successfully');
            
            // Show success notification with options
            vscode.window.showInformationMessage(
                `Successfully compiled ${require('path').basename(document.fileName)} to WebAssembly`,
                'Open Output Folder',
                'Generate HTML Wrapper',
                'Test in Browser'
            ).then(selection => {
                switch (selection) {
                    case 'Open Output Folder':
                        this.openOutputFolder(cwd);
                        break;
                    case 'Generate HTML Wrapper':
                        this.generateHtmlWrapper(document, cwd);
                        break;
                    case 'Test in Browser':
                        this.testInBrowser(document, cwd);
                        break;
                }
            });

            return true;

        } catch (error: any) {
            this.outputChannel.appendLine('❌ WebAssembly compilation failed');
            this.outputChannel.appendLine(error.message);
            
            if (error.stdout) {
                this.outputChannel.appendLine('STDOUT:');
                this.outputChannel.appendLine(error.stdout);
            }
            
            if (error.stderr) {
                this.outputChannel.appendLine('STDERR:');
                this.outputChannel.appendLine(error.stderr);
            }

            vscode.window.showErrorMessage(
                `Failed to compile ${require('path').basename(document.fileName)} to WebAssembly`,
                'View Output'
            ).then(selection => {
                if (selection === 'View Output') {
                    this.outputChannel.show();
                }
            });

            return false;
        }
    }

    async generateHtmlWrapper(document: vscode.TextDocument, outputDir: string): Promise<void> {
        const path = require('path');
        const fs = require('fs');
        
        const baseName = path.basename(document.fileName, '.ov');
        const wasmFile = `${baseName}.wasm`;
        const htmlFile = `${baseName}.html`;
        const htmlPath = path.join(outputDir, htmlFile);

        const htmlContent = this.generateHtmlWrapperContent(baseName, wasmFile);
        
        try {
            fs.writeFileSync(htmlPath, htmlContent);
            
            vscode.window.showInformationMessage(
                `HTML wrapper generated: ${htmlFile}`,
                'Open File',
                'Open in Browser'
            ).then(selection => {
                switch (selection) {
                    case 'Open File':
                        vscode.workspace.openTextDocument(htmlPath).then(doc => {
                            vscode.window.showTextDocument(doc);
                        });
                        break;
                    case 'Open in Browser':
                        vscode.env.openExternal(vscode.Uri.file(htmlPath));
                        break;
                }
            });
            
        } catch (error: any) {
            vscode.window.showErrorMessage(`Failed to generate HTML wrapper: ${error.message}`);
        }
    }

    private generateHtmlWrapperContent(baseName: string, wasmFile: string): string {
        return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Ovie WebAssembly - ${baseName}</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            background: #0d1117;
            color: #f0f6fc;
        }
        
        .header {
            text-align: center;
            margin-bottom: 30px;
        }
        
        .logo {
            width: 64px;
            height: 64px;
            margin: 0 auto 20px;
            background: linear-gradient(135deg, #f59e0b, #d97706);
            border-radius: 12px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 24px;
            font-weight: bold;
            color: white;
        }
        
        .output {
            background: #161b22;
            border: 1px solid #30363d;
            border-radius: 8px;
            padding: 20px;
            margin: 20px 0;
            font-family: 'Courier New', monospace;
            white-space: pre-wrap;
            min-height: 200px;
        }
        
        .controls {
            text-align: center;
            margin: 20px 0;
        }
        
        button {
            background: linear-gradient(135deg, #f59e0b, #d97706);
            color: white;
            border: none;
            padding: 12px 24px;
            border-radius: 6px;
            font-size: 14px;
            font-weight: 600;
            cursor: pointer;
            margin: 0 10px;
            transition: all 0.2s ease;
        }
        
        button:hover {
            transform: translateY(-1px);
            box-shadow: 0 4px 12px rgba(245, 158, 11, 0.3);
        }
        
        button:disabled {
            opacity: 0.6;
            cursor: not-allowed;
            transform: none;
        }
        
        .status {
            text-align: center;
            margin: 10px 0;
            font-size: 14px;
        }
        
        .error {
            color: #da3633;
        }
        
        .success {
            color: #238636;
        }
    </style>
</head>
<body>
    <div class="header">
        <div class="logo">🎯</div>
        <h1>Ovie WebAssembly</h1>
        <p>Running <strong>${baseName}.ov</strong> in the browser</p>
    </div>
    
    <div class="controls">
        <button id="runBtn" onclick="runOvieProgram()">Run Program</button>
        <button id="clearBtn" onclick="clearOutput()">Clear Output</button>
    </div>
    
    <div class="status" id="status">Ready to run</div>
    
    <div class="output" id="output">Click "Run Program" to execute your Ovie code...</div>

    <script>
        let wasmModule = null;
        let outputElement = document.getElementById('output');
        let statusElement = document.getElementById('status');
        let runButton = document.getElementById('runBtn');
        
        // Load WebAssembly module
        async function loadWasm() {
            try {
                statusElement.textContent = 'Loading WebAssembly module...';
                statusElement.className = '';
                
                const wasmResponse = await fetch('${wasmFile}');
                const wasmBytes = await wasmResponse.arrayBuffer();
                
                // Create imports object for Ovie runtime
                const imports = {
                    env: {
                        // Ovie's seeAm function - print to output
                        ovie_print: function(ptr, len) {
                            const memory = wasmModule.exports.memory;
                            const buffer = new Uint8Array(memory.buffer, ptr, len);
                            const text = new TextDecoder().decode(buffer);
                            outputElement.textContent += text + '\\n';
                        },
                        
                        // Memory allocation functions
                        ovie_alloc: function(size) {
                            return wasmModule.exports.malloc(size);
                        },
                        
                        ovie_free: function(ptr) {
                            wasmModule.exports.free(ptr);
                        },
                        
                        // Math functions
                        sin: Math.sin,
                        cos: Math.cos,
                        tan: Math.tan,
                        sqrt: Math.sqrt,
                        pow: Math.pow,
                        log: Math.log,
                        exp: Math.exp,
                        
                        // Random number generation
                        random: Math.random
                    }
                };
                
                wasmModule = await WebAssembly.instantiate(wasmBytes, imports);
                
                statusElement.textContent = 'WebAssembly module loaded successfully';
                statusElement.className = 'success';
                runButton.disabled = false;
                
            } catch (error) {
                statusElement.textContent = 'Failed to load WebAssembly module: ' + error.message;
                statusElement.className = 'error';
                console.error('WASM loading error:', error);
            }
        }
        
        async function runOvieProgram() {
            if (!wasmModule) {
                statusElement.textContent = 'WebAssembly module not loaded';
                statusElement.className = 'error';
                return;
            }
            
            try {
                runButton.disabled = true;
                statusElement.textContent = 'Running Ovie program...';
                statusElement.className = '';
                
                // Clear previous output
                outputElement.textContent = '';
                
                // Call the main function
                if (wasmModule.exports.main) {
                    wasmModule.exports.main();
                } else if (wasmModule.exports._start) {
                    wasmModule.exports._start();
                } else {
                    throw new Error('No main function found in WebAssembly module');
                }
                
                statusElement.textContent = 'Program completed successfully';
                statusElement.className = 'success';
                
            } catch (error) {
                statusElement.textContent = 'Runtime error: ' + error.message;
                statusElement.className = 'error';
                outputElement.textContent += '\\nError: ' + error.message;
                console.error('Runtime error:', error);
            } finally {
                runButton.disabled = false;
            }
        }
        
        function clearOutput() {
            outputElement.textContent = 'Output cleared...';
            statusElement.textContent = 'Ready to run';
            statusElement.className = '';
        }
        
        // Load WASM module when page loads
        window.addEventListener('load', loadWasm);
    </script>
</body>
</html>`;
    }

    private async openOutputFolder(outputDir: string): Promise<void> {
        try {
            await vscode.commands.executeCommand('revealFileInOS', vscode.Uri.file(outputDir));
        } catch (error) {
            vscode.window.showErrorMessage('Failed to open output folder');
        }
    }

    private async testInBrowser(document: vscode.TextDocument, outputDir: string): Promise<void> {
        const path = require('path');
        const baseName = path.basename(document.fileName, '.ov');
        const htmlFile = `${baseName}.html`;
        const htmlPath = path.join(outputDir, htmlFile);
        
        try {
            // Check if HTML wrapper exists, create if not
            const fs = require('fs');
            if (!fs.existsSync(htmlPath)) {
                await this.generateHtmlWrapper(document, outputDir);
            }
            
            // Open in browser
            await vscode.env.openExternal(vscode.Uri.file(htmlPath));
            
        } catch (error: any) {
            vscode.window.showErrorMessage(`Failed to test in browser: ${error.message}`);
        }
    }

    async createWebProject(): Promise<void> {
        try {
            const projectName = await vscode.window.showInputBox({
                prompt: 'Enter web project name',
                placeHolder: 'my-ovie-web-app',
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

            const path = require('path');
            const fs = require('fs');
            const projectPath = path.join(folderUri[0].fsPath, projectName);

            // Create project structure
            fs.mkdirSync(projectPath, { recursive: true });
            fs.mkdirSync(path.join(projectPath, 'src'), { recursive: true });
            fs.mkdirSync(path.join(projectPath, 'web'), { recursive: true });
            fs.mkdirSync(path.join(projectPath, 'tests'), { recursive: true });

            // Create ovie.toml for web project
            const ovieToml = this.generateWebProjectConfig(projectName);
            fs.writeFileSync(path.join(projectPath, 'ovie.toml'), ovieToml);

            // Create main Ovie file
            const mainOvie = this.generateWebMainFile();
            fs.writeFileSync(path.join(projectPath, 'src', 'main.ov'), mainOvie);

            // Create HTML template
            const htmlTemplate = this.generateWebHtmlTemplate(projectName);
            fs.writeFileSync(path.join(projectPath, 'web', 'index.html'), htmlTemplate);

            // Create CSS file
            const cssFile = this.generateWebCssFile();
            fs.writeFileSync(path.join(projectPath, 'web', 'styles.css'), cssFile);

            // Create README
            const readme = this.generateWebReadme(projectName);
            fs.writeFileSync(path.join(projectPath, 'README.md'), readme);

            vscode.window.showInformationMessage(
                `Web project "${projectName}" created successfully!`,
                'Open Project'
            ).then(selection => {
                if (selection === 'Open Project') {
                    vscode.commands.executeCommand('vscode.openFolder', vscode.Uri.file(projectPath));
                }
            });

        } catch (error: any) {
            vscode.window.showErrorMessage(`Failed to create web project: ${error.message}`);
        }
    }

    private generateWebProjectConfig(projectName: string): string {
        return `[package]
name = "${projectName}"
version = "0.1.0"
description = "An Ovie web application"
authors = ["Your Name <your.email@example.com>"]
license = "MIT"
edition = "2026"

[bin]
name = "${projectName}"
path = "src/main.ov"

[build]
target = "wasm32-unknown-unknown"
output_dir = "web"

[dependencies]
std = { version = "1.0", features = ["web", "dom"] }

[dev-dependencies]

[features]
default = ["web"]
web = []

[profile.dev]
debug = true
optimization = "none"

[profile.release]
debug = false
optimization = "speed"
lto = true

[web]
html_template = "web/index.html"
assets_dir = "web"
serve_port = 8080
`;
    }

    private generateWebMainFile(): string {
        return `// Ovie Web Application
use std::web
use std::dom

fn main() {
    // Initialize web application
    web::init()
    
    seeAm "Ovie Web App Starting..."
    
    // Set up DOM event handlers
    setup_ui()
    
    // Start the application
    start_app()
}

fn setup_ui() {
    // Get DOM elements
    mut button = dom::get_element_by_id("run-button")
    mut output = dom::get_element_by_id("output")
    
    if button.is_some() {
        // Add click event listener
        button.unwrap().add_event_listener("click", handle_button_click)
    }
    
    // Update initial content
    if output.is_some() {
        output.unwrap().set_inner_html("Ovie Web App is ready! 🎯")
    }
}

fn handle_button_click(event: web::Event) {
    seeAm "Button clicked!"
    
    mut output = dom::get_element_by_id("output")
    if output.is_some() {
        mut current_time = web::get_current_time()
        output.unwrap().set_inner_html("Hello from Ovie! Time: " + current_time)
    }
    
    // Demonstrate Ovie features
    demonstrate_features()
}

fn start_app() {
    seeAm "Web application started successfully"
    
    // Set page title
    dom::set_title("Ovie Web Application")
    
    // Add some initial styling
    add_dynamic_styles()
}

fn demonstrate_features() {
    seeAm "=== Ovie Web Features Demo ==="
    
    // Variables and computation
    mut counter = get_counter()
    counter = counter + 1
    set_counter(counter)
    
    seeAm "Counter: " + counter
    
    // Update counter display
    mut counter_display = dom::get_element_by_id("counter")
    if counter_display.is_some() {
        counter_display.unwrap().set_inner_text("Clicks: " + counter)
    }
    
    // Array operations
    mut colors = ["#f59e0b", "#d97706", "#92400e", "#78350f"]
    mut random_color = colors[web::random_int(0, colors.length())]
    
    // Apply random color
    mut body = dom::get_element_by_tag("body")
    if body.is_some() {
        body.unwrap().set_style("background-color", random_color + "20")
    }
}

fn get_counter() -> u32 {
    mut stored = web::local_storage_get("ovie_counter")
    if stored.is_some() {
        return stored.unwrap().to_number()
    }
    return 0
}

fn set_counter(value: u32) {
    web::local_storage_set("ovie_counter", value.to_string())
}

fn add_dynamic_styles() {
    mut style = dom::create_element("style")
    style.set_inner_html("
        .ovie-highlight {
            background: linear-gradient(135deg, #f59e0b, #d97706);
            color: white;
            padding: 10px;
            border-radius: 8px;
            margin: 10px 0;
            animation: fadeIn 0.5s ease-in;
        }
        
        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(10px); }
            to { opacity: 1; transform: translateY(0); }
        }
    ")
    
    mut head = dom::get_element_by_tag("head")
    if head.is_some() {
        head.unwrap().append_child(style)
    }
}
`;
    }

    private generateWebHtmlTemplate(projectName: string): string {
        return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>${projectName} - Ovie Web App</title>
    <link rel="stylesheet" href="styles.css">
    <link rel="icon" type="image/png" href="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==">
</head>
<body>
    <div class="container">
        <header class="header">
            <div class="logo">🎯</div>
            <h1>${projectName}</h1>
            <p>Built with Ovie Programming Language</p>
        </header>
        
        <main class="main">
            <div class="card">
                <h2>Welcome to Ovie Web!</h2>
                <p>This is a web application built with the Ovie programming language and compiled to WebAssembly.</p>
                
                <div class="controls">
                    <button id="run-button" class="btn-primary">
                        Run Ovie Code
                    </button>
                    <span id="counter" class="counter">Clicks: 0</span>
                </div>
                
                <div id="output" class="output">
                    Loading Ovie application...
                </div>
            </div>
            
            <div class="card">
                <h3>Features</h3>
                <ul>
                    <li>🎯 Natural language syntax</li>
                    <li>🚀 Compiled to WebAssembly</li>
                    <li>🔒 Memory safe</li>
                    <li>⚡ High performance</li>
                    <li>🤖 AI-friendly</li>
                </ul>
            </div>
        </main>
        
        <footer class="footer">
            <p>Powered by <a href="https://ovie-lang.org" target="_blank">Ovie Programming Language</a></p>
        </footer>
    </div>
    
    <!-- Load the compiled WebAssembly module -->
    <script src="${projectName}.js"></script>
</body>
</html>`;
    }

    private generateWebCssFile(): string {
        return `/* Ovie Web Application Styles */
/* Using the same crown gold theme as the main website */

:root {
    /* Crown Gold Theme Colors */
    --bg-primary: #0d1117;
    --bg-secondary: #161b22;
    --bg-tertiary: #21262d;
    --text-primary: #f0f6fc;
    --text-secondary: #8b949e;
    --brand-primary: #f59e0b;
    --brand-secondary: #d97706;
    --brand-gradient: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
    --border-primary: #30363d;
    --shadow-md: 0 4px 8px rgba(0, 0, 0, 0.4);
    --border-radius: 8px;
}

* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', 'Inter', system-ui, sans-serif;
    background: var(--bg-primary);
    color: var(--text-primary);
    line-height: 1.6;
    min-height: 100vh;
    transition: background-color 0.3s ease;
}

.container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
    min-height: 100vh;
    display: flex;
    flex-direction: column;
}

.header {
    text-align: center;
    margin-bottom: 40px;
}

.logo {
    width: 80px;
    height: 80px;
    margin: 0 auto 20px;
    background: var(--brand-gradient);
    border-radius: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 32px;
    box-shadow: var(--shadow-md);
    animation: pulse 2s infinite;
}

@keyframes pulse {
    0%, 100% { transform: scale(1); }
    50% { transform: scale(1.05); }
}

.header h1 {
    font-size: 2.5rem;
    font-weight: 700;
    margin-bottom: 10px;
    background: var(--brand-gradient);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
}

.header p {
    color: var(--text-secondary);
    font-size: 1.1rem;
}

.main {
    flex: 1;
    display: grid;
    grid-template-columns: 2fr 1fr;
    gap: 30px;
    margin-bottom: 40px;
}

@media (max-width: 768px) {
    .main {
        grid-template-columns: 1fr;
        gap: 20px;
    }
}

.card {
    background: var(--bg-secondary);
    border: 1px solid var(--border-primary);
    border-radius: var(--border-radius);
    padding: 30px;
    box-shadow: var(--shadow-md);
    transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.card:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.5);
}

.card h2, .card h3 {
    color: var(--brand-primary);
    margin-bottom: 20px;
    font-weight: 600;
}

.card p {
    color: var(--text-secondary);
    margin-bottom: 20px;
}

.controls {
    display: flex;
    align-items: center;
    gap: 20px;
    margin-bottom: 30px;
    flex-wrap: wrap;
}

.btn-primary {
    background: var(--brand-gradient);
    color: white;
    border: none;
    padding: 12px 24px;
    border-radius: 6px;
    font-size: 16px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
    box-shadow: 0 2px 4px rgba(245, 158, 11, 0.3);
}

.btn-primary:hover {
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(245, 158, 11, 0.4);
}

.btn-primary:active {
    transform: translateY(0);
}

.btn-primary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    transform: none;
}

.counter {
    background: var(--bg-tertiary);
    padding: 8px 16px;
    border-radius: 20px;
    font-size: 14px;
    font-weight: 500;
    color: var(--brand-primary);
    border: 1px solid var(--border-primary);
}

.output {
    background: var(--bg-primary);
    border: 1px solid var(--border-primary);
    border-radius: var(--border-radius);
    padding: 20px;
    font-family: 'SF Mono', 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
    font-size: 14px;
    line-height: 1.5;
    white-space: pre-wrap;
    min-height: 200px;
    overflow-y: auto;
    color: var(--text-primary);
}

.card ul {
    list-style: none;
    padding: 0;
}

.card li {
    padding: 10px 0;
    border-bottom: 1px solid var(--border-primary);
    color: var(--text-secondary);
    font-size: 15px;
}

.card li:last-child {
    border-bottom: none;
}

.footer {
    text-align: center;
    padding: 20px 0;
    border-top: 1px solid var(--border-primary);
    color: var(--text-secondary);
}

.footer a {
    color: var(--brand-primary);
    text-decoration: none;
    font-weight: 500;
}

.footer a:hover {
    text-decoration: underline;
}

/* Animations */
@keyframes fadeIn {
    from {
        opacity: 0;
        transform: translateY(20px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

.card {
    animation: fadeIn 0.6s ease-out;
}

.card:nth-child(2) {
    animation-delay: 0.2s;
}

/* Responsive design */
@media (max-width: 480px) {
    .container {
        padding: 15px;
    }
    
    .header h1 {
        font-size: 2rem;
    }
    
    .card {
        padding: 20px;
    }
    
    .controls {
        flex-direction: column;
        align-items: stretch;
    }
    
    .btn-primary {
        width: 100%;
        justify-content: center;
    }
}
`;
    }

    private generateWebReadme(projectName: string): string {
        return `# ${projectName}

A web application built with the Ovie programming language and compiled to WebAssembly.

## Features

- 🎯 **Natural Syntax**: Written in Ovie's natural language syntax
- 🚀 **WebAssembly**: Compiled to high-performance WebAssembly
- 🔒 **Memory Safe**: Ovie's ownership system prevents common web vulnerabilities
- ⚡ **Fast**: Near-native performance in the browser
- 🤖 **AI-Friendly**: Easy to understand and modify with AI assistance

## Getting Started

### Prerequisites

- Ovie compiler with WebAssembly support
- A modern web browser
- Local web server (for development)

### Building

\`\`\`bash
# Compile to WebAssembly
ovie compile --target=wasm src/main.ov

# Or use the build command
ovie build
\`\`\`

### Running

\`\`\`bash
# Serve the web application
ovie serve

# Or use a simple HTTP server
python -m http.server 8080
# Then open http://localhost:8080
\`\`\`

### Development

\`\`\`bash
# Watch for changes and rebuild
ovie watch

# Run tests
ovie test

# Check code quality
ovie aproko
\`\`\`

## Project Structure

\`\`\`
${projectName}/
├── src/
│   └── main.ov          # Main Ovie source file
├── web/
│   ├── index.html       # HTML template
│   └── styles.css       # Stylesheet
├── tests/
│   └── main.test.ov     # Test files
├── ovie.toml            # Project configuration
└── README.md            # This file
\`\`\`

## Ovie Web Features

This project demonstrates several Ovie web capabilities:

- **DOM Manipulation**: Direct interaction with HTML elements
- **Event Handling**: Responding to user interactions
- **Local Storage**: Persistent data storage
- **Dynamic Styling**: Runtime CSS modifications
- **WebAssembly Integration**: Seamless browser integration

## Deployment

### Static Hosting

Deploy to any static hosting service:

1. Build the project: \`ovie build\`
2. Upload the \`web/\` directory contents
3. Ensure WASM files are served with correct MIME type

### CDN Deployment

For better performance, serve WASM files from a CDN:

\`\`\`javascript
// Update the WASM loading path in your HTML
const wasmResponse = await fetch('https://your-cdn.com/path/to/app.wasm');
\`\`\`

## Browser Support

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

All modern browsers with WebAssembly support.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes in Ovie
4. Test in multiple browsers
5. Submit a pull request

## Learn More

- [Ovie Documentation](https://ovie-lang.org/docs/)
- [WebAssembly Guide](https://ovie-lang.org/docs/webassembly.html)
- [Web Development with Ovie](https://ovie-lang.org/docs/web-development.html)
- [Examples](https://ovie-lang.org/examples/web/)

---

Built with ❤️ using [Ovie Programming Language](https://ovie-lang.org)
`;
    }
}