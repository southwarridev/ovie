import * as vscode from 'vscode';

export class OvieMobileCompiler {
    private context: vscode.ExtensionContext;
    private outputChannel: vscode.OutputChannel;

    constructor(context: vscode.ExtensionContext) {
        this.context = context;
        this.outputChannel = vscode.window.createOutputChannel('Ovie Mobile Compiler');
        context.subscriptions.push(this.outputChannel);
    }

    async compileForAndroid(document: vscode.TextDocument): Promise<boolean> {
        return this.compileForMobile(document, 'android');
    }

    async compileForIOS(document: vscode.TextDocument): Promise<boolean> {
        return this.compileForMobile(document, 'ios');
    }

    private async compileForMobile(document: vscode.TextDocument, platform: 'android' | 'ios'): Promise<boolean> {
        const config = vscode.workspace.getConfiguration('ovie');
        const compilerPath = config.get<string>('compiler.path', 'ovie');
        
        this.outputChannel.clear();
        this.outputChannel.show(true);
        this.outputChannel.appendLine(`Compiling ${document.fileName} for ${platform.toUpperCase()}...`);

        try {
            const workspaceFolder = vscode.workspace.getWorkspaceFolder(document.uri);
            const cwd = workspaceFolder?.uri.fsPath || require('path').dirname(document.fileName);

            // Save document if it has unsaved changes
            if (document.isDirty) {
                await document.save();
            }

            // Determine target architecture based on platform
            const target = platform === 'android' ? 'aarch64-linux-android' : 'aarch64-apple-ios';
            
            const { exec } = require('child_process');
            const { promisify } = require('util');
            const execAsync = promisify(exec);

            const { stdout, stderr } = await execAsync(
                `"${compilerPath}" compile --target=${target} "${document.fileName}"`,
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

            this.outputChannel.appendLine(`✅ ${platform.toUpperCase()} compilation completed successfully`);
            
            // Show success notification with platform-specific options
            const actions = platform === 'android' 
                ? ['Open APK Folder', 'Generate Android Project', 'Install on Device']
                : ['Open IPA Folder', 'Generate Xcode Project', 'Install on Simulator'];

            vscode.window.showInformationMessage(
                `Successfully compiled ${require('path').basename(document.fileName)} for ${platform.toUpperCase()}`,
                ...actions
            ).then(selection => {
                this.handleMobileAction(selection, document, cwd, platform);
            });

            return true;

        } catch (error: any) {
            this.outputChannel.appendLine(`❌ ${platform.toUpperCase()} compilation failed`);
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
                `Failed to compile ${require('path').basename(document.fileName)} for ${platform.toUpperCase()}`,
                'View Output'
            ).then(selection => {
                if (selection === 'View Output') {
                    this.outputChannel.show();
                }
            });

            return false;
        }
    }

    private async handleMobileAction(
        action: string | undefined, 
        document: vscode.TextDocument, 
        cwd: string, 
        platform: 'android' | 'ios'
    ): Promise<void> {
        if (!action) return;

        switch (action) {
            case 'Open APK Folder':
            case 'Open IPA Folder':
                await this.openOutputFolder(cwd);
                break;
            case 'Generate Android Project':
                await this.generateAndroidProject(document, cwd);
                break;
            case 'Generate Xcode Project':
                await this.generateXcodeProject(document, cwd);
                break;
            case 'Install on Device':
                await this.installOnAndroidDevice(document, cwd);
                break;
            case 'Install on Simulator':
                await this.installOnIOSSimulator(document, cwd);
                break;
        }
    }

    async createMobileProject(): Promise<void> {
        try {
            const platform = await vscode.window.showQuickPick(
                ['Android', 'iOS', 'Both'],
                { placeHolder: 'Select target platform(s)' }
            );

            if (!platform) {
                return;
            }

            const projectName = await vscode.window.showInputBox({
                prompt: 'Enter mobile project name',
                placeHolder: 'my-ovie-mobile-app',
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
            fs.mkdirSync(path.join(projectPath, 'tests'), { recursive: true });

            if (platform === 'Android' || platform === 'Both') {
                fs.mkdirSync(path.join(projectPath, 'android'), { recursive: true });
            }
            if (platform === 'iOS' || platform === 'Both') {
                fs.mkdirSync(path.join(projectPath, 'ios'), { recursive: true });
            }

            // Create ovie.toml for mobile project
            const ovieToml = this.generateMobileProjectConfig(projectName, platform.toLowerCase());
            fs.writeFileSync(path.join(projectPath, 'ovie.toml'), ovieToml);

            // Create main Ovie file
            const mainOvie = this.generateMobileMainFile();
            fs.writeFileSync(path.join(projectPath, 'src', 'main.ov'), mainOvie);

            // Create README
            const readme = this.generateMobileReadme(projectName, platform);
            fs.writeFileSync(path.join(projectPath, 'README.md'), readme);

            vscode.window.showInformationMessage(
                `Mobile project "${projectName}" created successfully!`,
                'Open Project'
            ).then(selection => {
                if (selection === 'Open Project') {
                    vscode.commands.executeCommand('vscode.openFolder', vscode.Uri.file(projectPath));
                }
            });

        } catch (error: any) {
            vscode.window.showErrorMessage(`Failed to create mobile project: ${error.message}`);
        }
    }

    private async openOutputFolder(outputDir: string): Promise<void> {
        try {
            await vscode.commands.executeCommand('revealFileInOS', vscode.Uri.file(outputDir));
        } catch (error) {
            vscode.window.showErrorMessage('Failed to open output folder');
        }
    }

    private async generateAndroidProject(document: vscode.TextDocument, outputDir: string): Promise<void> {
        // Simplified Android project generation
        vscode.window.showInformationMessage('Android project generation feature coming soon!');
    }

    private async generateXcodeProject(document: vscode.TextDocument, outputDir: string): Promise<void> {
        // Simplified Xcode project generation
        vscode.window.showInformationMessage('Xcode project generation feature coming soon!');
    }

    private async installOnAndroidDevice(document: vscode.TextDocument, cwd: string): Promise<void> {
        // Simplified Android installation
        vscode.window.showInformationMessage('Android device installation feature coming soon!');
    }

    private async installOnIOSSimulator(document: vscode.TextDocument, cwd: string): Promise<void> {
        // Simplified iOS simulator installation
        vscode.window.showInformationMessage('iOS simulator installation feature coming soon!');
    }

    private generateMobileProjectConfig(projectName: string, platform: string): string {
        const targets = platform === 'both' 
            ? ['aarch64-linux-android', 'aarch64-apple-ios']
            : platform === 'android' 
                ? ['aarch64-linux-android']
                : ['aarch64-apple-ios'];

        return `[package]
name = "${projectName}"
version = "0.1.0"
description = "An Ovie mobile application"
authors = ["Your Name <your.email@example.com>"]
license = "MIT"
edition = "2026"

[bin]
name = "${projectName}"
path = "src/main.ov"

[build]
targets = ${JSON.stringify(targets)}
output_dir = "build"

[dependencies]
std = { version = "1.0", features = ["mobile", "ui"] }

[mobile]
app_name = "${projectName}"
package_id = "com.ovie.${projectName.toLowerCase()}"
version = "1.0.0"
`;
    }

    private generateMobileMainFile(): string {
        return `// Ovie Mobile Application
use std::mobile
use std::ui

fn main() {
    // Initialize mobile application
    mobile::init()
    
    seeAm "Ovie Mobile App Starting..."
    
    // Create the main UI
    create_main_ui()
    
    // Start the application event loop
    mobile::run()
}

fn create_main_ui() {
    // Create main window
    mut window = ui::Window::new("Ovie Mobile App")
    window.set_size(400, 600)
    
    // Add header
    mut header = ui::Label::new("Welcome to Ovie Mobile! 🎯")
    header.set_style("font-size: 24px; font-weight: bold; color: #f59e0b;")
    
    // Add interactive button
    mut button = ui::Button::new("Tap Me!")
    button.set_style("background: #f59e0b; color: white; padding: 15px;")
    button.on_click(handle_button_tap)
    
    window.add(header)
    window.add(button)
    window.show()
}

fn handle_button_tap() {
    seeAm "Button tapped!"
    mobile::show_toast("Hello from Ovie!")
}
`;
    }

    private generateMobileReadme(projectName: string, platform: string): string {
        return `# ${projectName}

A mobile application built with the Ovie programming language.

## Platforms

${platform === 'Both' ? '- Android\n- iOS' : `- ${platform}`}

## Features

- 🎯 Natural language syntax
- 🚀 Native performance  
- 🔒 Memory safety
- 📱 Cross-platform support

## Getting Started

1. Install Ovie compiler with mobile targets
2. Build: \`ovie build\`
3. Run on device/simulator

Built with ❤️ using [Ovie Programming Language](https://ovie-lang.org)
`;
    }
}