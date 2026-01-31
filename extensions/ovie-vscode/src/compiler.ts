import * as vscode from 'vscode';
import { exec, spawn } from 'child_process';
import { promisify } from 'util';
import * as path from 'path';
import * as fs from 'fs';

const execAsync = promisify(exec);

export class OvieCompiler {
    private context: vscode.ExtensionContext;
    private outputChannel: vscode.OutputChannel;
    private terminal: vscode.Terminal | undefined;

    constructor(context: vscode.ExtensionContext) {
        this.context = context;
        this.outputChannel = vscode.window.createOutputChannel('Ovie Compiler');
        context.subscriptions.push(this.outputChannel);
    }

    async compile(document: vscode.TextDocument): Promise<boolean> {
        const config = vscode.workspace.getConfiguration('ovie');
        const compilerPath = config.get<string>('compiler.path', 'ovie');
        
        this.outputChannel.clear();
        this.outputChannel.show(true);
        this.outputChannel.appendLine(`Compiling ${document.fileName}...`);

        try {
            const workspaceFolder = vscode.workspace.getWorkspaceFolder(document.uri);
            const cwd = workspaceFolder?.uri.fsPath || path.dirname(document.fileName);

            // Save document if it has unsaved changes
            if (document.isDirty) {
                await document.save();
            }

            const { stdout, stderr } = await execAsync(
                `"${compilerPath}" compile "${document.fileName}"`,
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

            this.outputChannel.appendLine('✅ Compilation completed successfully');
            
            // Show success notification
            vscode.window.showInformationMessage(
                `Successfully compiled ${path.basename(document.fileName)}`,
                'Run',
                'View Output'
            ).then(selection => {
                switch (selection) {
                    case 'Run':
                        this.run(document);
                        break;
                    case 'View Output':
                        this.outputChannel.show();
                        break;
                }
            });

            return true;

        } catch (error: any) {
            this.outputChannel.appendLine('❌ Compilation failed');
            this.outputChannel.appendLine(error.message);
            
            if (error.stdout) {
                this.outputChannel.appendLine('STDOUT:');
                this.outputChannel.appendLine(error.stdout);
            }
            
            if (error.stderr) {
                this.outputChannel.appendLine('STDERR:');
                this.outputChannel.appendLine(error.stderr);
            }

            // Parse and show compilation errors
            this.parseCompilationErrors(error.stderr || error.message, document);

            vscode.window.showErrorMessage(
                `Failed to compile ${path.basename(document.fileName)}`,
                'View Output',
                'Show Problems'
            ).then(selection => {
                switch (selection) {
                    case 'View Output':
                        this.outputChannel.show();
                        break;
                    case 'Show Problems':
                        vscode.commands.executeCommand('workbench.panel.markers.view.focus');
                        break;
                }
            });

            return false;
        }
    }

    async run(document: vscode.TextDocument): Promise<void> {
        const config = vscode.workspace.getConfiguration('ovie');
        const compilerPath = config.get<string>('compiler.path', 'ovie');
        
        const workspaceFolder = vscode.workspace.getWorkspaceFolder(document.uri);
        const cwd = workspaceFolder?.uri.fsPath || path.dirname(document.fileName);

        // Create or reuse terminal
        if (!this.terminal || this.terminal.exitStatus !== undefined) {
            this.terminal = vscode.window.createTerminal({
                name: 'Ovie Runner',
                cwd: cwd
            });
            this.context.subscriptions.push(this.terminal);
        }

        this.terminal.show();
        this.terminal.sendText(`"${compilerPath}" run "${document.fileName}"`);
    }

    async runTests(): Promise<void> {
        const config = vscode.workspace.getConfiguration('ovie');
        const compilerPath = config.get<string>('compiler.path', 'ovie');
        
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders || workspaceFolders.length === 0) {
            vscode.window.showErrorMessage('No workspace folder found');
            return;
        }

        const cwd = workspaceFolders[0].uri.fsPath;

        this.outputChannel.clear();
        this.outputChannel.show(true);
        this.outputChannel.appendLine('Running Ovie tests...');

        try {
            const { stdout, stderr } = await execAsync(
                `"${compilerPath}" test`,
                { cwd }
            );

            if (stdout) {
                this.outputChannel.appendLine(stdout);
            }

            if (stderr) {
                this.outputChannel.appendLine('STDERR:');
                this.outputChannel.appendLine(stderr);
            }

            // Parse test results
            this.parseTestResults(stdout);

        } catch (error: any) {
            this.outputChannel.appendLine('❌ Test execution failed');
            this.outputChannel.appendLine(error.message);
            
            if (error.stdout) {
                this.outputChannel.appendLine(error.stdout);
            }
            
            if (error.stderr) {
                this.outputChannel.appendLine('STDERR:');
                this.outputChannel.appendLine(error.stderr);
            }

            vscode.window.showErrorMessage('Test execution failed', 'View Output').then(selection => {
                if (selection === 'View Output') {
                    this.outputChannel.show();
                }
            });
        }
    }

    async buildProject(): Promise<boolean> {
        const config = vscode.workspace.getConfiguration('ovie');
        const compilerPath = config.get<string>('compiler.path', 'ovie');
        
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders || workspaceFolders.length === 0) {
            vscode.window.showErrorMessage('No workspace folder found');
            return false;
        }

        const cwd = workspaceFolders[0].uri.fsPath;

        // Check for ovie.toml
        const configPath = path.join(cwd, 'ovie.toml');
        if (!fs.existsSync(configPath)) {
            vscode.window.showErrorMessage(
                'No ovie.toml found in workspace root',
                'Create Project'
            ).then(selection => {
                if (selection === 'Create Project') {
                    vscode.commands.executeCommand('ovie.project.new');
                }
            });
            return false;
        }

        this.outputChannel.clear();
        this.outputChannel.show(true);
        this.outputChannel.appendLine('Building Ovie project...');

        return vscode.window.withProgress({
            location: vscode.ProgressLocation.Notification,
            title: 'Building Ovie Project',
            cancellable: true
        }, async (progress, token) => {
            try {
                const buildProcess = spawn(compilerPath, ['build'], { cwd });
                
                let stdout = '';
                let stderr = '';

                buildProcess.stdout?.on('data', (data) => {
                    const output = data.toString();
                    stdout += output;
                    this.outputChannel.append(output);
                    
                    // Update progress based on output
                    if (output.includes('Compiling')) {
                        progress.report({ message: 'Compiling sources...' });
                    } else if (output.includes('Linking')) {
                        progress.report({ message: 'Linking...' });
                    }
                });

                buildProcess.stderr?.on('data', (data) => {
                    const output = data.toString();
                    stderr += output;
                    this.outputChannel.append(output);
                });

                return new Promise<boolean>((resolve) => {
                    buildProcess.on('close', (code) => {
                        if (code === 0) {
                            this.outputChannel.appendLine('✅ Build completed successfully');
                            vscode.window.showInformationMessage('Build completed successfully');
                            resolve(true);
                        } else {
                            this.outputChannel.appendLine(`❌ Build failed with exit code ${code}`);
                            this.parseCompilationErrors(stderr, undefined);
                            vscode.window.showErrorMessage('Build failed', 'View Output').then(selection => {
                                if (selection === 'View Output') {
                                    this.outputChannel.show();
                                }
                            });
                            resolve(false);
                        }
                    });

                    token.onCancellationRequested(() => {
                        buildProcess.kill();
                        this.outputChannel.appendLine('Build cancelled by user');
                        resolve(false);
                    });
                });

            } catch (error: any) {
                this.outputChannel.appendLine('❌ Build process failed to start');
                this.outputChannel.appendLine(error.message);
                vscode.window.showErrorMessage('Failed to start build process');
                return false;
            }
        });
    }

    async checkSyntax(document: vscode.TextDocument): Promise<boolean> {
        const config = vscode.workspace.getConfiguration('ovie');
        const compilerPath = config.get<string>('compiler.path', 'ovie');
        
        try {
            const workspaceFolder = vscode.workspace.getWorkspaceFolder(document.uri);
            const cwd = workspaceFolder?.uri.fsPath || path.dirname(document.fileName);

            const { stdout, stderr } = await execAsync(
                `"${compilerPath}" check "${document.fileName}"`,
                { cwd }
            );

            // Parse syntax check results
            if (stderr) {
                this.parseCompilationErrors(stderr, document);
                return false;
            }

            return true;

        } catch (error: any) {
            this.parseCompilationErrors(error.stderr || error.message, document);
            return false;
        }
    }

    private parseCompilationErrors(errorOutput: string, document?: vscode.TextDocument): void {
        if (!document) return;

        const diagnostics: vscode.Diagnostic[] = [];
        const lines = errorOutput.split('\n');

        for (const line of lines) {
            // Parse Ovie error format: filename:line:column: error: message
            const match = line.match(/^(.+):(\d+):(\d+):\s*(error|warning|note):\s*(.+)$/);
            if (match) {
                const [, file, lineStr, colStr, severity, message] = match;
                const lineNum = parseInt(lineStr) - 1; // VS Code uses 0-based line numbers
                const colNum = parseInt(colStr) - 1;

                if (path.resolve(file) === path.resolve(document.fileName)) {
                    const range = new vscode.Range(
                        lineNum,
                        colNum,
                        lineNum,
                        colNum + 1
                    );

                    const diagnostic = new vscode.Diagnostic(
                        range,
                        message,
                        severity === 'error' ? vscode.DiagnosticSeverity.Error :
                        severity === 'warning' ? vscode.DiagnosticSeverity.Warning :
                        vscode.DiagnosticSeverity.Information
                    );

                    diagnostic.source = 'ovie';
                    diagnostics.push(diagnostic);
                }
            }
        }

        // Update diagnostics
        const diagnosticCollection = vscode.languages.createDiagnosticCollection('ovie-compiler');
        this.context.subscriptions.push(diagnosticCollection);
        diagnosticCollection.set(document.uri, diagnostics);
    }

    private parseTestResults(output: string): void {
        const lines = output.split('\n');
        let testCount = 0;
        let passedCount = 0;
        let failedCount = 0;

        for (const line of lines) {
            if (line.includes('test result:')) {
                const match = line.match(/(\d+) passed.*?(\d+) failed/);
                if (match) {
                    passedCount = parseInt(match[1]);
                    failedCount = parseInt(match[2]);
                    testCount = passedCount + failedCount;
                }
            }
        }

        if (testCount > 0) {
            const message = `Tests completed: ${passedCount} passed, ${failedCount} failed`;
            if (failedCount === 0) {
                this.outputChannel.appendLine(`✅ ${message}`);
                vscode.window.showInformationMessage(message);
            } else {
                this.outputChannel.appendLine(`❌ ${message}`);
                vscode.window.showWarningMessage(message, 'View Details').then(selection => {
                    if (selection === 'View Details') {
                        this.outputChannel.show();
                    }
                });
            }
        }
    }

    async getCompilerVersion(): Promise<string | null> {
        try {
            const config = vscode.workspace.getConfiguration('ovie');
            const compilerPath = config.get<string>('compiler.path', 'ovie');
            
            const { stdout } = await execAsync(`"${compilerPath}" --version`);
            return stdout.trim();
        } catch (error) {
            console.error('Failed to get compiler version:', error);
            return null;
        }
    }

    async isCompilerAvailable(): Promise<boolean> {
        try {
            const version = await this.getCompilerVersion();
            return version !== null;
        } catch {
            return false;
        }
    }

    dispose(): void {
        this.outputChannel.dispose();
        if (this.terminal) {
            this.terminal.dispose();
        }
    }
}