import * as vscode from 'vscode';
import { LanguageClient, LanguageClientOptions, ServerOptions, TransportKind } from 'vscode-languageclient/node';
import * as path from 'path';

export class OvieLanguageServer {
    private client: LanguageClient | undefined;
    private context: vscode.ExtensionContext;

    constructor(context: vscode.ExtensionContext) {
        this.context = context;
    }

    async start(): Promise<void> {
        try {
            // Check if Ovie compiler is available
            const config = vscode.workspace.getConfiguration('ovie');
            const compilerPath = config.get<string>('compiler.path', 'ovie');
            
            // Server options - use Ovie's built-in language server
            const serverOptions: ServerOptions = {
                command: compilerPath,
                args: ['lsp'],
                options: {
                    env: {
                        ...process.env,
                        OVIE_LSP_MODE: 'true'
                    }
                }
            };

            // Client options
            const clientOptions: LanguageClientOptions = {
                documentSelector: [
                    { scheme: 'file', language: 'ovie' },
                    { scheme: 'untitled', language: 'ovie' }
                ],
                synchronize: {
                    fileEvents: [
                        vscode.workspace.createFileSystemWatcher('**/*.ov'),
                        vscode.workspace.createFileSystemWatcher('**/ovie.toml')
                    ]
                },
                initializationOptions: {
                    settings: config.get('ovie', {}),
                    workspaceFolders: vscode.workspace.workspaceFolders?.map(folder => folder.uri.toString())
                }
            };

            // Create and start the language client
            this.client = new LanguageClient(
                'ovie-language-server',
                'Ovie Language Server',
                serverOptions,
                clientOptions
            );

            // Start the client and server
            await this.client.start();
            
            console.log('Ovie Language Server started successfully');
            
            // Register additional capabilities
            this.registerCustomCapabilities();
            
        } catch (error) {
            console.error('Failed to start Ovie Language Server:', error);
            
            // Fallback to basic functionality without LSP
            vscode.window.showWarningMessage(
                'Ovie Language Server failed to start. Some features may be limited. Make sure Ovie is installed and accessible.',
                'Install Ovie',
                'Check Settings'
            ).then(selection => {
                switch (selection) {
                    case 'Install Ovie':
                        vscode.env.openExternal(vscode.Uri.parse('https://ovie-lang.org/docs/installation.html'));
                        break;
                    case 'Check Settings':
                        vscode.commands.executeCommand('workbench.action.openSettings', 'ovie.compiler.path');
                        break;
                }
            });
        }
    }

    async stop(): Promise<void> {
        if (this.client) {
            await this.client.stop();
            this.client = undefined;
        }
    }

    private registerCustomCapabilities(): void {
        if (!this.client) return;

        // Register custom commands that work with the language server
        this.context.subscriptions.push(
            vscode.commands.registerCommand('ovie.lsp.restart', async () => {
                await this.restart();
            }),

            vscode.commands.registerCommand('ovie.lsp.showOutput', () => {
                this.client?.outputChannel.show();
            }),

            vscode.commands.registerCommand('ovie.lsp.status', () => {
                const state = this.client?.state;
                const message = state === 1 ? 'Running' : state === 2 ? 'Starting' : 'Stopped';
                vscode.window.showInformationMessage(`Ovie Language Server: ${message}`);
            })
        );

        // Handle server notifications
        this.client.onNotification('ovie/progress', (params: any) => {
            // Handle progress notifications from the server
            if (params.kind === 'begin') {
                vscode.window.withProgress({
                    location: vscode.ProgressLocation.Window,
                    title: params.title || 'Ovie Language Server'
                }, (progress) => {
                    return new Promise((resolve) => {
                        const listener = this.client?.onNotification('ovie/progress', (endParams: any) => {
                            if (endParams.kind === 'end' && endParams.token === params.token) {
                                listener?.dispose();
                                resolve(undefined);
                            } else if (endParams.kind === 'report' && endParams.token === params.token) {
                                progress.report({
                                    message: endParams.message,
                                    increment: endParams.percentage
                                });
                            }
                        });
                    });
                });
            }
        });

        // Handle custom requests
        this.client.onRequest('ovie/showMessage', (params: any) => {
            switch (params.type) {
                case 'info':
                    vscode.window.showInformationMessage(params.message);
                    break;
                case 'warning':
                    vscode.window.showWarningMessage(params.message);
                    break;
                case 'error':
                    vscode.window.showErrorMessage(params.message);
                    break;
            }
        });
    }

    async restart(): Promise<void> {
        vscode.window.showInformationMessage('Restarting Ovie Language Server...');
        
        await this.stop();
        await new Promise(resolve => setTimeout(resolve, 1000)); // Wait a bit
        await this.start();
        
        vscode.window.showInformationMessage('Ovie Language Server restarted successfully');
    }

    isRunning(): boolean {
        return this.client?.state === 2; // Running state
    }

    async sendCustomRequest(method: string, params?: any): Promise<any> {
        if (!this.client || !this.isRunning()) {
            throw new Error('Language server is not running');
        }
        
        return await this.client.sendRequest(method, params);
    }

    async getProjectInfo(): Promise<any> {
        try {
            return await this.sendCustomRequest('ovie/projectInfo');
        } catch (error) {
            console.error('Failed to get project info:', error);
            return null;
        }
    }

    async getCompilerVersion(): Promise<string | null> {
        try {
            const result = await this.sendCustomRequest('ovie/version');
            return result?.version || null;
        } catch (error) {
            console.error('Failed to get compiler version:', error);
            return null;
        }
    }

    async validateProject(): Promise<any> {
        try {
            return await this.sendCustomRequest('ovie/validate');
        } catch (error) {
            console.error('Failed to validate project:', error);
            return null;
        }
    }
}