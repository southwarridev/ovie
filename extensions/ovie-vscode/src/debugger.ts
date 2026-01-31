import * as vscode from 'vscode';
import { exec } from 'child_process';
import { promisify } from 'util';
import * as path from 'path';

const execAsync = promisify(exec);

export class OvieDebugger implements vscode.DebugConfigurationProvider {
    private context: vscode.ExtensionContext;

    constructor(context: vscode.ExtensionContext) {
        this.context = context;
        this.registerDebugProvider();
    }

    private registerDebugProvider(): void {
        // Register debug configuration provider
        this.context.subscriptions.push(
            vscode.debug.registerDebugConfigurationProvider('ovie', this)
        );

        // Register debug adapter descriptor factory
        this.context.subscriptions.push(
            vscode.debug.registerDebugAdapterDescriptorFactory('ovie', new OvieDebugAdapterFactory())
        );
    }

    // DebugConfigurationProvider implementation
    provideDebugConfigurations(
        folder: vscode.WorkspaceFolder | undefined,
        token?: vscode.CancellationToken
    ): vscode.ProviderResult<vscode.DebugConfiguration[]> {
        return [
            {
                name: 'Debug Ovie File',
                type: 'ovie',
                request: 'launch',
                program: '${file}',
                cwd: '${workspaceFolder}',
                console: 'integratedTerminal',
                stopOnEntry: false
            },
            {
                name: 'Debug Ovie Project',
                type: 'ovie',
                request: 'launch',
                program: '${workspaceFolder}/src/main.ov',
                cwd: '${workspaceFolder}',
                console: 'integratedTerminal',
                stopOnEntry: false
            },
            {
                name: 'Debug Ovie Tests',
                type: 'ovie',
                request: 'launch',
                program: 'test',
                cwd: '${workspaceFolder}',
                console: 'integratedTerminal',
                stopOnEntry: false,
                args: []
            }
        ];
    }

    resolveDebugConfiguration(
        folder: vscode.WorkspaceFolder | undefined,
        debugConfiguration: vscode.DebugConfiguration,
        token?: vscode.CancellationToken
    ): vscode.ProviderResult<vscode.DebugConfiguration> {
        // If no configuration is provided, create a default one
        if (!debugConfiguration.type && !debugConfiguration.request && !debugConfiguration.name) {
            const editor = vscode.window.activeTextEditor;
            if (editor && editor.document.languageId === 'ovie') {
                debugConfiguration.type = 'ovie';
                debugConfiguration.name = 'Debug Current Ovie File';
                debugConfiguration.request = 'launch';
                debugConfiguration.program = editor.document.fileName;
                debugConfiguration.cwd = folder?.uri.fsPath || path.dirname(editor.document.fileName);
                debugConfiguration.console = 'integratedTerminal';
                debugConfiguration.stopOnEntry = false;
            }
        }

        // Validate configuration
        if (!debugConfiguration.program) {
            vscode.window.showErrorMessage('No program specified for debugging');
            return undefined;
        }

        // Set default values
        debugConfiguration.cwd = debugConfiguration.cwd || folder?.uri.fsPath || '${workspaceFolder}';
        debugConfiguration.console = debugConfiguration.console || 'integratedTerminal';
        debugConfiguration.stopOnEntry = debugConfiguration.stopOnEntry || false;

        return debugConfiguration;
    }

    async startDebugging(document: vscode.TextDocument): Promise<void> {
        const workspaceFolder = vscode.workspace.getWorkspaceFolder(document.uri);
        
        const debugConfig: vscode.DebugConfiguration = {
            type: 'ovie',
            name: 'Debug Current File',
            request: 'launch',
            program: document.fileName,
            cwd: workspaceFolder?.uri.fsPath || path.dirname(document.fileName),
            console: 'integratedTerminal',
            stopOnEntry: false
        };

        await vscode.debug.startDebugging(workspaceFolder, debugConfig);
    }
}

class OvieDebugAdapterFactory implements vscode.DebugAdapterDescriptorFactory {
    createDebugAdapterDescriptor(
        session: vscode.DebugSession,
        executable: vscode.DebugAdapterExecutable | undefined
    ): vscode.ProviderResult<vscode.DebugAdapterDescriptor> {
        const config = vscode.workspace.getConfiguration('ovie');
        const compilerPath = config.get<string>('compiler.path', 'ovie');

        // Use Ovie's built-in debug adapter
        return new vscode.DebugAdapterExecutable(compilerPath, ['debug-adapter']);
    }
}

export class OvieDebugSession {
    private session: vscode.DebugSession | undefined;
    private context: vscode.ExtensionContext;

    constructor(context: vscode.ExtensionContext) {
        this.context = context;
        this.registerDebugEvents();
    }

    private registerDebugEvents(): void {
        // Listen for debug session events
        this.context.subscriptions.push(
            vscode.debug.onDidStartDebugSession((session) => {
                if (session.type === 'ovie') {
                    this.session = session;
                    this.onDebugSessionStarted(session);
                }
            }),

            vscode.debug.onDidTerminateDebugSession((session) => {
                if (session.type === 'ovie' && session === this.session) {
                    this.session = undefined;
                    this.onDebugSessionTerminated(session);
                }
            }),

            vscode.debug.onDidChangeBreakpoints((event) => {
                this.onBreakpointsChanged(event);
            })
        );
    }

    private onDebugSessionStarted(session: vscode.DebugSession): void {
        console.log('Ovie debug session started:', session.name);
        
        // Show debug console
        vscode.commands.executeCommand('workbench.debug.action.toggleRepl');
        
        // Set up custom debug commands
        this.registerDebugCommands();
    }

    private onDebugSessionTerminated(session: vscode.DebugSession): void {
        console.log('Ovie debug session terminated:', session.name);
    }

    private onBreakpointsChanged(event: vscode.BreakpointsChangeEvent): void {
        // Handle breakpoint changes for Ovie files
        const ovieBreakpoints = event.added.filter(bp => 
            bp instanceof vscode.SourceBreakpoint && 
            bp.location.uri.path.endsWith('.ov')
        );

        if (ovieBreakpoints.length > 0) {
            console.log(`Added ${ovieBreakpoints.length} Ovie breakpoints`);
        }
    }

    private registerDebugCommands(): void {
        // Register debug-specific commands
        const commands = [
            vscode.commands.registerCommand('ovie.debug.inspectVariable', async (variableName: string) => {
                if (this.session) {
                    try {
                        const result = await this.session.customRequest('evaluate', {
                            expression: variableName,
                            context: 'watch'
                        });
                        
                        vscode.window.showInformationMessage(`${variableName} = ${result.result}`);
                    } catch (error) {
                        vscode.window.showErrorMessage(`Failed to inspect variable: ${error}`);
                    }
                }
            }),

            vscode.commands.registerCommand('ovie.debug.evaluateExpression', async () => {
                const expression = await vscode.window.showInputBox({
                    prompt: 'Enter Ovie expression to evaluate',
                    placeHolder: 'e.g., variable_name or function_call()'
                });

                if (expression && this.session) {
                    try {
                        const result = await this.session.customRequest('evaluate', {
                            expression: expression,
                            context: 'repl'
                        });
                        
                        vscode.window.showInformationMessage(`Result: ${result.result}`);
                    } catch (error) {
                        vscode.window.showErrorMessage(`Evaluation failed: ${error}`);
                    }
                }
            }),

            vscode.commands.registerCommand('ovie.debug.showCallStack', async () => {
                if (this.session) {
                    try {
                        const threads = await this.session.customRequest('threads');
                        if (threads.threads && threads.threads.length > 0) {
                            const stackTrace = await this.session.customRequest('stackTrace', {
                                threadId: threads.threads[0].id
                            });
                            
                            const stackInfo = stackTrace.stackFrames
                                .map((frame: any) => `${frame.name} (${frame.source?.name}:${frame.line})`)
                                .join('\n');
                            
                            vscode.window.showInformationMessage(`Call Stack:\n${stackInfo}`);
                        }
                    } catch (error) {
                        vscode.window.showErrorMessage(`Failed to get call stack: ${error}`);
                    }
                }
            })
        ];

        commands.forEach(command => this.context.subscriptions.push(command));
    }

    async setBreakpoint(document: vscode.TextDocument, line: number): Promise<void> {
        const breakpoint = new vscode.SourceBreakpoint(
            new vscode.Location(document.uri, new vscode.Position(line, 0))
        );

        const currentBreakpoints = vscode.debug.breakpoints;
        vscode.debug.addBreakpoints([breakpoint]);
        
        vscode.window.showInformationMessage(
            `Breakpoint set at line ${line + 1} in ${path.basename(document.fileName)}`
        );
    }

    async removeBreakpoint(document: vscode.TextDocument, line: number): Promise<void> {
        const breakpoints = vscode.debug.breakpoints.filter(bp => 
            bp instanceof vscode.SourceBreakpoint &&
            bp.location.uri.toString() === document.uri.toString() &&
            bp.location.range.start.line === line
        );

        if (breakpoints.length > 0) {
            vscode.debug.removeBreakpoints(breakpoints);
            vscode.window.showInformationMessage(
                `Breakpoint removed from line ${line + 1} in ${path.basename(document.fileName)}`
            );
        }
    }

    async toggleBreakpoint(document: vscode.TextDocument, line: number): Promise<void> {
        const existingBreakpoints = vscode.debug.breakpoints.filter(bp => 
            bp instanceof vscode.SourceBreakpoint &&
            bp.location.uri.toString() === document.uri.toString() &&
            bp.location.range.start.line === line
        );

        if (existingBreakpoints.length > 0) {
            await this.removeBreakpoint(document, line);
        } else {
            await this.setBreakpoint(document, line);
        }
    }

    isDebugging(): boolean {
        return this.session !== undefined;
    }

    async stopDebugging(): Promise<void> {
        if (this.session) {
            await vscode.debug.stopDebugging(this.session);
        }
    }
}