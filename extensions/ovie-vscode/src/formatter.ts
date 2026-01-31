import * as vscode from 'vscode';
import { exec } from 'child_process';
import { promisify } from 'util';
import * as path from 'path';

const execAsync = promisify(exec);

export class OvieFormatter implements vscode.DocumentFormattingEditProvider, vscode.DocumentRangeFormattingEditProvider {
    private context: vscode.ExtensionContext;

    constructor(context: vscode.ExtensionContext) {
        this.context = context;
    }

    // Document formatting provider implementation
    async provideDocumentFormattingEdits(
        document: vscode.TextDocument,
        options: vscode.FormattingOptions,
        token: vscode.CancellationToken
    ): Promise<vscode.TextEdit[]> {
        return this.formatDocument(document, options);
    }

    // Range formatting provider implementation
    async provideDocumentRangeFormattingEdits(
        document: vscode.TextDocument,
        range: vscode.Range,
        options: vscode.FormattingOptions,
        token: vscode.CancellationToken
    ): Promise<vscode.TextEdit[]> {
        return this.formatRange(document, range, options);
    }

    private async formatDocument(
        document: vscode.TextDocument,
        options: vscode.FormattingOptions
    ): Promise<vscode.TextEdit[]> {
        try {
            const config = vscode.workspace.getConfiguration('ovie');
            const compilerPath = config.get<string>('compiler.path', 'ovie');
            
            const workspaceFolder = vscode.workspace.getWorkspaceFolder(document.uri);
            const cwd = workspaceFolder?.uri.fsPath || path.dirname(document.fileName);

            // Use Ovie's built-in formatter
            const { spawn } = require('child_process');
            const formatter = spawn(compilerPath, ['fmt', '--stdin'], { cwd });
            
            let stdout = '';
            let stderr = '';
            
            formatter.stdout.on('data', (data: any) => {
                stdout += data.toString();
            });
            
            formatter.stderr.on('data', (data: any) => {
                stderr += data.toString();
            });
            
            // Write input to stdin
            formatter.stdin.write(document.getText());
            formatter.stdin.end();
            
            // Wait for process to complete
            await new Promise((resolve, reject) => {
                formatter.on('close', (code: number) => {
                    if (code === 0) {
                        resolve(code);
                    } else {
                        reject(new Error(`Formatter exited with code ${code}`));
                    }
                });
            });

            if (stderr) {
                console.error('Formatting error:', stderr);
                vscode.window.showWarningMessage('Code formatting completed with warnings');
            }

            const formattedCode = stdout;
            
            if (formattedCode !== document.getText()) {
                // Return edit that replaces entire document
                const fullRange = new vscode.Range(
                    document.positionAt(0),
                    document.positionAt(document.getText().length)
                );
                
                return [vscode.TextEdit.replace(fullRange, formattedCode)];
            }

            return [];

        } catch (error: any) {
            console.error('Failed to format document:', error);
            
            // Fallback to basic formatting
            return this.basicFormat(document, options);
        }
    }

    private async formatRange(
        document: vscode.TextDocument,
        range: vscode.Range,
        options: vscode.FormattingOptions
    ): Promise<vscode.TextEdit[]> {
        try {
            const config = vscode.workspace.getConfiguration('ovie');
            const compilerPath = config.get<string>('compiler.path', 'ovie');
            
            const workspaceFolder = vscode.workspace.getWorkspaceFolder(document.uri);
            const cwd = workspaceFolder?.uri.fsPath || path.dirname(document.fileName);

            const selectedText = document.getText(range);
            
            // Format selected range
            const { spawn } = require('child_process');
            const formatter = spawn(compilerPath, ['fmt', '--stdin', '--range'], { cwd });
            
            let stdout = '';
            let stderr = '';
            
            formatter.stdout.on('data', (data: any) => {
                stdout += data.toString();
            });
            
            formatter.stderr.on('data', (data: any) => {
                stderr += data.toString();
            });
            
            // Write input to stdin
            formatter.stdin.write(selectedText);
            formatter.stdin.end();
            
            // Wait for process to complete
            await new Promise((resolve, reject) => {
                formatter.on('close', (code: number) => {
                    if (code === 0) {
                        resolve(code);
                    } else {
                        reject(new Error(`Range formatter exited with code ${code}`));
                    }
                });
            });

            if (stderr) {
                console.error('Range formatting error:', stderr);
            }

            const formattedCode = stdout;
            
            if (formattedCode !== selectedText) {
                return [vscode.TextEdit.replace(range, formattedCode)];
            }

            return [];

        } catch (error: any) {
            console.error('Failed to format range:', error);
            
            // Fallback to basic range formatting
            return this.basicFormatRange(document, range, options);
        }
    }

    private basicFormat(
        document: vscode.TextDocument,
        options: vscode.FormattingOptions
    ): vscode.TextEdit[] {
        const edits: vscode.TextEdit[] = [];
        const text = document.getText();
        const lines = text.split('\n');
        
        let indentLevel = 0;
        const indentString = options.insertSpaces ? ' '.repeat(options.tabSize) : '\t';

        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            const trimmedLine = line.trim();
            
            if (trimmedLine === '') continue;

            // Adjust indent level based on braces
            if (trimmedLine.endsWith('{')) {
                const expectedIndent = indentString.repeat(indentLevel);
                const formattedLine = expectedIndent + trimmedLine;
                
                if (line !== formattedLine) {
                    const lineRange = new vscode.Range(i, 0, i, line.length);
                    edits.push(vscode.TextEdit.replace(lineRange, formattedLine));
                }
                
                indentLevel++;
            } else if (trimmedLine.startsWith('}')) {
                indentLevel = Math.max(0, indentLevel - 1);
                const expectedIndent = indentString.repeat(indentLevel);
                const formattedLine = expectedIndent + trimmedLine;
                
                if (line !== formattedLine) {
                    const lineRange = new vscode.Range(i, 0, i, line.length);
                    edits.push(vscode.TextEdit.replace(lineRange, formattedLine));
                }
            } else {
                const expectedIndent = indentString.repeat(indentLevel);
                const formattedLine = expectedIndent + trimmedLine;
                
                if (line !== formattedLine) {
                    const lineRange = new vscode.Range(i, 0, i, line.length);
                    edits.push(vscode.TextEdit.replace(lineRange, formattedLine));
                }
            }
        }

        return edits;
    }

    private basicFormatRange(
        document: vscode.TextDocument,
        range: vscode.Range,
        options: vscode.FormattingOptions
    ): vscode.TextEdit[] {
        const edits: vscode.TextEdit[] = [];
        const selectedText = document.getText(range);
        const lines = selectedText.split('\n');
        
        const indentString = options.insertSpaces ? ' '.repeat(options.tabSize) : '\t';
        
        // Basic formatting for selected range
        const formattedLines = lines.map(line => {
            const trimmed = line.trim();
            if (trimmed === '') return line;
            
            // Simple indentation based on context
            if (trimmed.endsWith('{') || trimmed.endsWith('(') || trimmed.endsWith('[')) {
                return indentString + trimmed;
            } else if (trimmed.startsWith('}') || trimmed.startsWith(')') || trimmed.startsWith(']')) {
                return trimmed;
            } else {
                return indentString + trimmed;
            }
        });
        
        const formattedText = formattedLines.join('\n');
        
        if (formattedText !== selectedText) {
            edits.push(vscode.TextEdit.replace(range, formattedText));
        }
        
        return edits;
    }

    async formatOnSave(document: vscode.TextDocument): Promise<void> {
        const config = vscode.workspace.getConfiguration('ovie');
        if (!config.get<boolean>('formatting.enabled', true)) {
            return;
        }

        try {
            const edits = await this.formatDocument(document, {
                insertSpaces: true,
                tabSize: 4
            });

            if (edits.length > 0) {
                const workspaceEdit = new vscode.WorkspaceEdit();
                workspaceEdit.set(document.uri, edits);
                await vscode.workspace.applyEdit(workspaceEdit);
            }
        } catch (error) {
            console.error('Format on save failed:', error);
        }
    }

    async formatOnType(
        document: vscode.TextDocument,
        position: vscode.Position,
        ch: string
    ): Promise<vscode.TextEdit[]> {
        const config = vscode.workspace.getConfiguration('ovie');
        if (!config.get<boolean>('formatting.enabled', true)) {
            return [];
        }

        // Format on specific characters
        if (ch === '}' || ch === ';' || ch === '\n') {
            const line = document.lineAt(position.line);
            const range = new vscode.Range(position.line, 0, position.line, line.text.length);
            
            return this.basicFormatRange(document, range, {
                insertSpaces: true,
                tabSize: 4
            });
        }

        return [];
    }

    async organizeImports(document: vscode.TextDocument): Promise<vscode.TextEdit[]> {
        try {
            const config = vscode.workspace.getConfiguration('ovie');
            const compilerPath = config.get<string>('compiler.path', 'ovie');
            
            const workspaceFolder = vscode.workspace.getWorkspaceFolder(document.uri);
            const cwd = workspaceFolder?.uri.fsPath || path.dirname(document.fileName);

            // Use Ovie's import organizer
            const { spawn } = require('child_process');
            const formatter = spawn(compilerPath, ['fmt', '--organize-imports', '--stdin'], { cwd });
            
            let stdout = '';
            let stderr = '';
            
            formatter.stdout.on('data', (data: any) => {
                stdout += data.toString();
            });
            
            formatter.stderr.on('data', (data: any) => {
                stderr += data.toString();
            });
            
            // Write input to stdin
            formatter.stdin.write(document.getText());
            formatter.stdin.end();
            
            // Wait for process to complete
            await new Promise((resolve, reject) => {
                formatter.on('close', (code: number) => {
                    if (code === 0) {
                        resolve(code);
                    } else {
                        reject(new Error(`Import organizer exited with code ${code}`));
                    }
                });
            });

            if (stderr) {
                console.error('Import organization error:', stderr);
            }

            const organizedCode = stdout;
            
            if (organizedCode !== document.getText()) {
                const fullRange = new vscode.Range(
                    document.positionAt(0),
                    document.positionAt(document.getText().length)
                );
                
                return [vscode.TextEdit.replace(fullRange, organizedCode)];
            }

            return [];

        } catch (error: any) {
            console.error('Failed to organize imports:', error);
            return [];
        }
    }

    async validateFormatting(document: vscode.TextDocument): Promise<boolean> {
        try {
            const config = vscode.workspace.getConfiguration('ovie');
            const compilerPath = config.get<string>('compiler.path', 'ovie');
            
            const workspaceFolder = vscode.workspace.getWorkspaceFolder(document.uri);
            const cwd = workspaceFolder?.uri.fsPath || path.dirname(document.fileName);

            // Check if code is properly formatted
            const { spawn } = require('child_process');
            const formatter = spawn(compilerPath, ['fmt', '--check', '--stdin'], { cwd });
            
            let stdout = '';
            let stderr = '';
            
            formatter.stdout.on('data', (data: any) => {
                stdout += data.toString();
            });
            
            formatter.stderr.on('data', (data: any) => {
                stderr += data.toString();
            });
            
            // Write input to stdin
            formatter.stdin.write(document.getText());
            formatter.stdin.end();
            
            // Wait for process to complete
            await new Promise((resolve, reject) => {
                formatter.on('close', (code: number) => {
                    resolve(code);
                });
            });

            return stderr === '';

        } catch (error: any) {
            // If formatter returns non-zero exit code, formatting is needed
            return false;
        }
    }

    getFormattingOptions(): vscode.FormattingOptions {
        const config = vscode.workspace.getConfiguration('ovie.formatting');
        
        return {
            insertSpaces: config.get<boolean>('insertSpaces', true),
            tabSize: config.get<number>('tabSize', 4)
        };
    }
}