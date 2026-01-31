import * as vscode from 'vscode';
import axios from 'axios';

export class AIProvider implements vscode.CompletionItemProvider {
    private context: vscode.ExtensionContext;
    private apiKey: string | undefined;

    constructor(context: vscode.ExtensionContext) {
        this.context = context;
        this.loadConfiguration();
    }

    private loadConfiguration(): void {
        const config = vscode.workspace.getConfiguration('ovie.ai');
        this.apiKey = config.get<string>('apiKey');
    }

    // Completion provider implementation
    async provideCompletionItems(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken,
        context: vscode.CompletionContext
    ): Promise<vscode.CompletionItem[]> {
        const config = vscode.workspace.getConfiguration('ovie');
        if (!config.get<boolean>('ai.enabled', true)) {
            return [];
        }

        const provider = config.get<string>('ai.provider', 'github-copilot');
        
        switch (provider) {
            case 'github-copilot':
                return this.getCopilotCompletions(document, position);
            case 'openai':
                return this.getOpenAICompletions(document, position);
            case 'anthropic':
                return this.getAnthropicCompletions(document, position);
            case 'local':
                return this.getLocalCompletions(document, position);
            default:
                return this.getBuiltInCompletions(document, position);
        }
    }

    async complete(editor: vscode.TextEditor): Promise<void> {
        const document = editor.document;
        const position = editor.selection.active;
        
        try {
            const completions = await this.provideCompletionItems(
                document,
                position,
                new vscode.CancellationTokenSource().token,
                { triggerKind: vscode.CompletionTriggerKind.Invoke, triggerCharacter: undefined }
            );

            if (completions && completions.length > 0) {
                const quickPick = vscode.window.createQuickPick();
                quickPick.items = completions.map(item => ({
                    label: item.label as string,
                    description: item.detail,
                    detail: item.documentation?.toString()
                }));
                
                quickPick.onDidChangeSelection(selection => {
                    if (selection[0]) {
                        const selectedCompletion = completions.find(c => c.label === selection[0].label);
                        if (selectedCompletion && selectedCompletion.insertText) {
                            editor.edit(editBuilder => {
                                editBuilder.insert(position, selectedCompletion.insertText as string);
                            });
                        }
                    }
                    quickPick.hide();
                });
                
                quickPick.show();
            }
        } catch (error) {
            console.error('AI completion error:', error);
            vscode.window.showErrorMessage('Failed to get AI completions.');
        }
    }

    async explain(document: vscode.TextDocument, selection: vscode.Selection): Promise<void> {
        const selectedText = document.getText(selection);
        if (!selectedText.trim()) {
            vscode.window.showWarningMessage('Please select some code to explain.');
            return;
        }

        try {
            const explanation = await this.getAIExplanation(selectedText);
            this.showExplanationPanel(explanation, selectedText);
        } catch (error) {
            console.error('AI explanation error:', error);
            vscode.window.showErrorMessage('Failed to get AI explanation.');
        }
    }

    async translateToOvie(editor: vscode.TextEditor): Promise<void> {
        const document = editor.document;
        const selectedText = document.getText(editor.selection);
        
        if (!selectedText.trim()) {
            vscode.window.showWarningMessage('Please select code to translate to Ovie.');
            return;
        }

        try {
            const sourceLanguage = await vscode.window.showQuickPick([
                'JavaScript',
                'Python',
                'Java',
                'C++',
                'Rust',
                'Go',
                'TypeScript',
                'C#',
                'Other'
            ], {
                placeHolder: 'Select the source language'
            });

            if (!sourceLanguage) {
                return;
            }

            const ovieCode = await this.translateCode(selectedText, sourceLanguage.toLowerCase());
            
            // Show translation in a new editor
            const newDocument = await vscode.workspace.openTextDocument({
                content: ovieCode,
                language: 'ovie'
            });
            
            await vscode.window.showTextDocument(newDocument, vscode.ViewColumn.Beside);
            
        } catch (error) {
            console.error('Translation error:', error);
            vscode.window.showErrorMessage('Failed to translate code to Ovie.');
        }
    }

    private async getCopilotCompletions(
        document: vscode.TextDocument,
        position: vscode.Position
    ): Promise<vscode.CompletionItem[]> {
        try {
            // Use GitHub Copilot API if available
            const context = this.buildOvieContext(document, position);
            
            const result = await vscode.commands.executeCommand(
                'github.copilot.generate',
                {
                    languageId: 'ovie',
                    prompt: context.prompt,
                    context: {
                        ...context,
                        constraints: {
                            keywords: ['fn', 'mut', 'if', 'else', 'for', 'while', 'struct', 'enum', 'unsafe', 'return', 'true', 'false', 'seeAm'],
                            patterns: ['natural_language', 'pidgin_english'],
                            output_function: 'seeAm'
                        }
                    }
                }
            );

            if (result && Array.isArray(result)) {
                return result.map((item: any) => {
                    const completion = new vscode.CompletionItem(
                        item.text || item.label,
                        vscode.CompletionItemKind.Text
                    );
                    completion.insertText = item.text;
                    completion.detail = 'GitHub Copilot';
                    return completion;
                });
            }
        } catch (error) {
            console.log('Copilot not available, falling back to built-in completions');
        }

        return this.getBuiltInCompletions(document, position);
    }

    private async getOpenAICompletions(
        document: vscode.TextDocument,
        position: vscode.Position
    ): Promise<vscode.CompletionItem[]> {
        if (!this.apiKey) {
            vscode.window.showErrorMessage('OpenAI API key not configured. Please set ovie.ai.apiKey in settings.');
            return [];
        }

        try {
            const context = this.buildOvieContext(document, position);
            
            const response = await axios.post('https://api.openai.com/v1/completions', {
                model: 'gpt-3.5-turbo-instruct',
                prompt: context.prompt,
                max_tokens: 150,
                temperature: 0.3,
                stop: ['\n\n', '```']
            }, {
                headers: {
                    'Authorization': `Bearer ${this.apiKey}`,
                    'Content-Type': 'application/json'
                }
            });

            const completions = response.data.choices.map((choice: any) => {
                const completion = new vscode.CompletionItem(
                    choice.text.trim(),
                    vscode.CompletionItemKind.Text
                );
                completion.insertText = choice.text.trim();
                completion.detail = 'OpenAI GPT';
                return completion;
            });

            return completions;
        } catch (error) {
            console.error('OpenAI API error:', error);
            return [];
        }
    }

    private async getAnthropicCompletions(
        document: vscode.TextDocument,
        position: vscode.Position
    ): Promise<vscode.CompletionItem[]> {
        // Similar implementation for Anthropic Claude
        return this.getBuiltInCompletions(document, position);
    }

    private async getLocalCompletions(
        document: vscode.TextDocument,
        position: vscode.Position
    ): Promise<vscode.CompletionItem[]> {
        // Implementation for local AI model
        return this.getBuiltInCompletions(document, position);
    }

    private getBuiltInCompletions(
        document: vscode.TextDocument,
        position: vscode.Position
    ): vscode.CompletionItem[] {
        const completions: vscode.CompletionItem[] = [];

        // Ovie keywords
        const keywords = [
            { label: 'fn', detail: 'Function definition', insertText: 'fn ${1:name}(${2:params}) {\n    ${3:// body}\n}' },
            { label: 'mut', detail: 'Mutable variable', insertText: 'mut ${1:name} = ${2:value}' },
            { label: 'seeAm', detail: 'Print output', insertText: 'seeAm ${1:value}' },
            { label: 'if', detail: 'Conditional statement', insertText: 'if ${1:condition} {\n    ${2:// body}\n}' },
            { label: 'for', detail: 'Loop construct', insertText: 'for ${1:item} in ${2:items} {\n    ${3:// body}\n}' },
            { label: 'struct', detail: 'Data structure', insertText: 'struct ${1:Name} {\n    ${2:field}: ${3:Type},\n}' },
            { label: 'test', detail: 'Test case', insertText: 'test "${1:description}" {\n    ${2:// test body}\n}' }
        ];

        keywords.forEach(keyword => {
            const item = new vscode.CompletionItem(keyword.label, vscode.CompletionItemKind.Keyword);
            item.detail = keyword.detail;
            item.insertText = new vscode.SnippetString(keyword.insertText);
            completions.push(item);
        });

        // Common patterns
        const patterns = [
            {
                label: 'main function',
                insertText: 'fn main() {\n    seeAm "Hello, World!"\n}',
                detail: 'Main function template'
            },
            {
                label: 'error handling',
                insertText: 'if ${1:result}.is_error() {\n    seeAm "Error: " + ${1:result}.error()\n    return\n}',
                detail: 'Error handling pattern'
            }
        ];

        patterns.forEach(pattern => {
            const item = new vscode.CompletionItem(pattern.label, vscode.CompletionItemKind.Snippet);
            item.detail = pattern.detail;
            item.insertText = new vscode.SnippetString(pattern.insertText);
            completions.push(item);
        });

        return completions;
    }

    private buildOvieContext(document: vscode.TextDocument, position: vscode.Position): any {
        const text = document.getText();
        const beforeCursor = text.substring(0, document.offsetAt(position));
        const afterCursor = text.substring(document.offsetAt(position));

        return {
            prompt: `Complete this Ovie code (use 'seeAm' for output, 'mut' for mutable variables):\n${beforeCursor}`,
            language: 'ovie',
            before: beforeCursor,
            after: afterCursor,
            syntax_hints: [
                'Use seeAm instead of print',
                'Use mut for mutable variables',
                'Functions start with fn',
                'Natural English-like syntax'
            ]
        };
    }

    private async getAIExplanation(code: string): Promise<string> {
        const config = vscode.workspace.getConfiguration('ovie.ai');
        const provider = config.get<string>('provider', 'github-copilot');

        const prompt = `Explain this Ovie programming language code in simple terms:

\`\`\`ovie
${code}
\`\`\`

Ovie is a self-hosted programming language with natural language syntax. Key features:
- Uses 'seeAm' for output instead of 'print'
- Uses 'mut' for mutable variables
- Functions defined with 'fn'
- Natural English-like patterns

Please explain what this code does and how it works:`;

        try {
            if (provider === 'openai' && this.apiKey) {
                const response = await axios.post('https://api.openai.com/v1/chat/completions', {
                    model: 'gpt-3.5-turbo',
                    messages: [
                        { role: 'system', content: 'You are an expert in the Ovie programming language. Explain code clearly and concisely.' },
                        { role: 'user', content: prompt }
                    ],
                    max_tokens: 300,
                    temperature: 0.3
                }, {
                    headers: {
                        'Authorization': `Bearer ${this.apiKey}`,
                        'Content-Type': 'application/json'
                    }
                });

                return response.data.choices[0].message.content;
            }
        } catch (error) {
            console.error('AI explanation error:', error);
        }

        // Fallback explanation
        return `This Ovie code appears to be well-structured. Ovie uses natural language patterns to make programming more accessible. Key elements you might see:

- 'seeAm' for output/printing
- 'mut' for mutable variables
- 'fn' for function definitions
- Natural English-like syntax

For more detailed explanations, configure an AI provider in the extension settings.`;
    }

    private async translateCode(sourceCode: string, fromLanguage: string): Promise<string> {
        const prompt = `Translate this ${fromLanguage} code to Ovie programming language:

Source (${fromLanguage}):
\`\`\`${fromLanguage}
${sourceCode}
\`\`\`

Ovie translation guidelines:
- Use 'seeAm' instead of print/console.log
- Use 'mut' for mutable variables
- Use 'fn' for functions
- Use natural English-like syntax
- Include type annotations where helpful

Ovie code:`;

        try {
            const config = vscode.workspace.getConfiguration('ovie.ai');
            const provider = config.get<string>('provider', 'github-copilot');

            if (provider === 'openai' && this.apiKey) {
                const response = await axios.post('https://api.openai.com/v1/chat/completions', {
                    model: 'gpt-3.5-turbo',
                    messages: [
                        { role: 'system', content: 'You are an expert programmer who specializes in translating code to the Ovie programming language.' },
                        { role: 'user', content: prompt }
                    ],
                    max_tokens: 500,
                    temperature: 0.2
                }, {
                    headers: {
                        'Authorization': `Bearer ${this.apiKey}`,
                        'Content-Type': 'application/json'
                    }
                });

                return response.data.choices[0].message.content.replace(/```ovie\n?|```\n?/g, '').trim();
            }
        } catch (error) {
            console.error('Translation error:', error);
        }

        // Fallback basic translation
        return `// Translated from ${fromLanguage} to Ovie
// Note: Configure an AI provider for better translations

// Original code:
// ${sourceCode.split('\n').map(line => `// ${line}`).join('\n')}

// Basic Ovie equivalent:
fn main() {
    seeAm "Translation requires AI provider configuration"
    seeAm "Please set up OpenAI or another provider in settings"
}`;
    }

    private showExplanationPanel(explanation: string, selectedText: string): void {
        const panel = vscode.window.createWebviewPanel(
            'ovieAIExplanation',
            'AI Code Explanation',
            vscode.ViewColumn.Beside,
            {
                enableScripts: true,
                retainContextWhenHidden: true
            }
        );

        panel.webview.html = this.getExplanationHtml(explanation, selectedText);
    }

    private getExplanationHtml(explanation: string, selectedText: string): string {
        return `
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>AI Code Explanation</title>
            <style>
                body { 
                    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
                    padding: 20px;
                    background: var(--vscode-editor-background);
                    color: var(--vscode-editor-foreground);
                    line-height: 1.6;
                }
                .code-block {
                    background: var(--vscode-textCodeBlock-background);
                    padding: 15px;
                    border-radius: 6px;
                    font-family: 'Courier New', monospace;
                    margin: 15px 0;
                    border: 1px solid var(--vscode-panel-border);
                }
                .explanation {
                    background: var(--vscode-textBlockQuote-background);
                    padding: 20px;
                    border-left: 4px solid var(--vscode-textBlockQuote-border);
                    margin: 15px 0;
                    border-radius: 4px;
                }
                .ai-badge {
                    display: inline-block;
                    background: linear-gradient(135deg, #f59e0b, #d97706);
                    color: white;
                    padding: 4px 12px;
                    border-radius: 12px;
                    font-size: 12px;
                    font-weight: 600;
                    margin-bottom: 15px;
                }
                h2 {
                    color: var(--vscode-foreground);
                    margin-bottom: 10px;
                }
                h3 {
                    color: var(--vscode-foreground);
                    margin-top: 25px;
                    margin-bottom: 10px;
                }
            </style>
        </head>
        <body>
            <div class="ai-badge">🤖 AI Powered</div>
            <h2>Code Explanation</h2>
            
            <h3>Selected Code:</h3>
            <div class="code-block">${this.escapeHtml(selectedText)}</div>
            
            <h3>AI Explanation:</h3>
            <div class="explanation">
                ${this.formatExplanation(explanation)}
            </div>
        </body>
        </html>
        `;
    }

    private escapeHtml(text: string): string {
        return text
            .replace(/&/g, '&amp;')
            .replace(/</g, '&lt;')
            .replace(/>/g, '&gt;')
            .replace(/"/g, '&quot;')
            .replace(/'/g, '&#39;');
    }

    private formatExplanation(explanation: string): string {
        return explanation
            .replace(/\n/g, '<br>')
            .replace(/`([^`]+)`/g, '<code>$1</code>');
    }
}