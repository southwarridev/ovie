import * as vscode from 'vscode';
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

export class AprokoProvider implements vscode.HoverProvider, vscode.CodeActionProvider {
    private diagnosticCollection: vscode.DiagnosticCollection | undefined;
    private context: vscode.ExtensionContext;

    constructor(context: vscode.ExtensionContext) {
        this.context = context;
    }

    setDiagnosticCollection(collection: vscode.DiagnosticCollection) {
        this.diagnosticCollection = collection;
    }

    async analyze(document: vscode.TextDocument): Promise<void> {
        try {
            const config = vscode.workspace.getConfiguration('ovie');
            const compilerPath = config.get<string>('compiler.path', 'ovie');
            
            // Run Aproko analysis
            const { stdout, stderr } = await execAsync(
                `${compilerPath} aproko --json "${document.fileName}"`
            );

            if (stderr) {
                console.error('Aproko analysis error:', stderr);
                return;
            }

            const analysis = JSON.parse(stdout);
            this.updateDiagnostics(document, analysis);
            
            // Show analysis results
            this.showAnalysisResults(analysis);

        } catch (error) {
            console.error('Failed to run Aproko analysis:', error);
            vscode.window.showErrorMessage('Failed to run Aproko analysis. Make sure Ovie is installed.');
        }
    }

    async applyFixes(document: vscode.TextDocument): Promise<void> {
        try {
            const config = vscode.workspace.getConfiguration('ovie');
            const compilerPath = config.get<string>('compiler.path', 'ovie');
            
            // Run Aproko with auto-fix
            const { stdout, stderr } = await execAsync(
                `${compilerPath} aproko --fix --json "${document.fileName}"`
            );

            if (stderr) {
                console.error('Aproko fix error:', stderr);
                return;
            }

            const result = JSON.parse(stdout);
            
            if (result.fixes_applied > 0) {
                vscode.window.showInformationMessage(
                    `Aproko applied ${result.fixes_applied} fixes to your code.`
                );
                
                // Reload the document
                const uri = document.uri;
                await vscode.commands.executeCommand('workbench.action.files.revert', uri);
            } else {
                vscode.window.showInformationMessage('No automatic fixes available.');
            }

        } catch (error) {
            console.error('Failed to apply Aproko fixes:', error);
            vscode.window.showErrorMessage('Failed to apply fixes.');
        }
    }

    async explain(document: vscode.TextDocument, selection: vscode.Selection): Promise<void> {
        try {
            const selectedText = document.getText(selection);
            const line = selection.start.line + 1;
            const column = selection.start.character + 1;

            const config = vscode.workspace.getConfiguration('ovie');
            const compilerPath = config.get<string>('compiler.path', 'ovie');
            
            // Get explanation from Aproko
            const { stdout } = await execAsync(
                `${compilerPath} aproko --explain --line ${line} --column ${column} "${document.fileName}"`
            );

            const explanation = JSON.parse(stdout);
            
            // Show explanation in a webview
            this.showExplanationPanel(explanation, selectedText);

        } catch (error) {
            console.error('Failed to get explanation:', error);
            vscode.window.showErrorMessage('Failed to get code explanation.');
        }
    }

    // Hover provider implementation
    provideHover(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken
    ): vscode.ProviderResult<vscode.Hover> {
        const wordRange = document.getWordRangeAtPosition(position);
        if (!wordRange) {
            return;
        }

        const word = document.getText(wordRange);
        
        // Provide hover information for Ovie keywords and functions
        const hoverInfo = this.getHoverInfo(word);
        if (hoverInfo) {
            return new vscode.Hover(hoverInfo);
        }
    }

    // Code actions provider implementation
    provideCodeActions(
        document: vscode.TextDocument,
        range: vscode.Range | vscode.Selection,
        context: vscode.CodeActionContext,
        token: vscode.CancellationToken
    ): vscode.ProviderResult<vscode.CodeAction[]> {
        const actions: vscode.CodeAction[] = [];

        // Add Aproko-specific code actions
        for (const diagnostic of context.diagnostics) {
            if (diagnostic.source === 'aproko') {
                const action = new vscode.CodeAction(
                    `Fix: ${diagnostic.message}`,
                    vscode.CodeActionKind.QuickFix
                );
                action.command = {
                    command: 'ovie.aproko.fix',
                    title: 'Apply Aproko Fix'
                };
                actions.push(action);
            }
        }

        // Add general Ovie actions
        const analyzeAction = new vscode.CodeAction(
            'Run Aproko Analysis',
            vscode.CodeActionKind.Source
        );
        analyzeAction.command = {
            command: 'ovie.aproko.analyze',
            title: 'Analyze with Aproko'
        };
        actions.push(analyzeAction);

        return actions;
    }

    private updateDiagnostics(document: vscode.TextDocument, analysis: any): void {
        if (!this.diagnosticCollection) {
            return;
        }

        const diagnostics: vscode.Diagnostic[] = [];

        if (analysis.issues) {
            for (const issue of analysis.issues) {
                const range = new vscode.Range(
                    issue.line - 1,
                    issue.column - 1,
                    issue.line - 1,
                    issue.column - 1 + (issue.length || 1)
                );

                const severity = this.getSeverity(issue.severity);
                const diagnostic = new vscode.Diagnostic(
                    range,
                    issue.message,
                    severity
                );
                
                diagnostic.source = 'aproko';
                diagnostic.code = issue.rule;
                
                if (issue.suggestion) {
                    diagnostic.relatedInformation = [
                        new vscode.DiagnosticRelatedInformation(
                            new vscode.Location(document.uri, range),
                            `Suggestion: ${issue.suggestion}`
                        )
                    ];
                }

                diagnostics.push(diagnostic);
            }
        }

        this.diagnosticCollection.set(document.uri, diagnostics);
    }

    private getSeverity(severity: string): vscode.DiagnosticSeverity {
        switch (severity.toLowerCase()) {
            case 'error':
                return vscode.DiagnosticSeverity.Error;
            case 'warning':
                return vscode.DiagnosticSeverity.Warning;
            case 'info':
                return vscode.DiagnosticSeverity.Information;
            default:
                return vscode.DiagnosticSeverity.Hint;
        }
    }

    private getHoverInfo(word: string): vscode.MarkdownString | undefined {
        const ovieKeywords: { [key: string]: string } = {
            'fn': 'Define a function\n\n```ovie\nfn greet(name) {\n    seeAm "Hello, " + name + "!"\n}\n```',
            'mut': 'Declare a mutable variable\n\n```ovie\nmut counter = 0\ncounter = counter + 1\n```',
            'seeAm': 'Print output to console\n\n```ovie\nseeAm "Hello, World!"\nseeAm variable_name\n```',
            'if': 'Conditional statement\n\n```ovie\nif condition {\n    // code\n} else {\n    // alternative\n}\n```',
            'for': 'Loop construct\n\n```ovie\nfor item in items {\n    seeAm item\n}\n```',
            'struct': 'Define a data structure\n\n```ovie\nstruct Person {\n    name: String,\n    age: Number\n}\n```',
            'test': 'Define a test case\n\n```ovie\ntest "addition works" {\n    assert_equal(add(2, 3), 5)\n}\n```'
        };

        if (ovieKeywords[word]) {
            const markdown = new vscode.MarkdownString(ovieKeywords[word]);
            markdown.isTrusted = true;
            return markdown;
        }
    }

    private showAnalysisResults(analysis: any): void {
        if (analysis.issues && analysis.issues.length > 0) {
            const issueCount = analysis.issues.length;
            const errorCount = analysis.issues.filter((i: any) => i.severity === 'error').length;
            const warningCount = analysis.issues.filter((i: any) => i.severity === 'warning').length;
            
            let message = `Aproko found ${issueCount} issue(s)`;
            if (errorCount > 0) {
                message += ` (${errorCount} error(s)`;
            }
            if (warningCount > 0) {
                message += `, ${warningCount} warning(s))`;
            } else if (errorCount > 0) {
                message += ')';
            }
            
            vscode.window.showWarningMessage(message, 'View Problems').then(selection => {
                if (selection === 'View Problems') {
                    vscode.commands.executeCommand('workbench.panel.markers.view.focus');
                }
            });
        } else {
            vscode.window.showInformationMessage('✅ Aproko analysis complete - no issues found!');
        }
    }

    private showExplanationPanel(explanation: any, selectedText: string): void {
        const panel = vscode.window.createWebviewPanel(
            'ovieExplanation',
            'Aproko Explanation',
            vscode.ViewColumn.Beside,
            {
                enableScripts: true,
                retainContextWhenHidden: true
            }
        );

        panel.webview.html = this.getExplanationHtml(explanation, selectedText);
    }

    private getExplanationHtml(explanation: any, selectedText: string): string {
        return `
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Aproko Explanation</title>
            <style>
                body { 
                    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
                    padding: 20px;
                    background: var(--vscode-editor-background);
                    color: var(--vscode-editor-foreground);
                }
                .code-block {
                    background: var(--vscode-textCodeBlock-background);
                    padding: 10px;
                    border-radius: 4px;
                    font-family: 'Courier New', monospace;
                    margin: 10px 0;
                }
                .explanation {
                    background: var(--vscode-textBlockQuote-background);
                    padding: 15px;
                    border-left: 4px solid var(--vscode-textBlockQuote-border);
                    margin: 10px 0;
                }
                .suggestion {
                    background: var(--vscode-inputValidation-infoBackground);
                    padding: 10px;
                    border-radius: 4px;
                    margin: 10px 0;
                }
            </style>
        </head>
        <body>
            <h2>🤖 Aproko Code Explanation</h2>
            
            <h3>Selected Code:</h3>
            <div class="code-block">${selectedText}</div>
            
            <h3>Explanation:</h3>
            <div class="explanation">
                ${explanation.explanation || 'This code appears to be well-structured Ovie code.'}
            </div>
            
            ${explanation.suggestions ? `
                <h3>Suggestions:</h3>
                <div class="suggestion">
                    ${explanation.suggestions}
                </div>
            ` : ''}
            
            ${explanation.examples ? `
                <h3>Examples:</h3>
                <div class="code-block">${explanation.examples}</div>
            ` : ''}
        </body>
        </html>
        `;
    }
}