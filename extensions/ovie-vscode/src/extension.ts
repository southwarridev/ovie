import * as vscode from 'vscode';
import { OvieLanguageServer } from './language-server';
import { AprokoProvider } from './aproko-provider';
import { AIProvider } from './ai-provider';
import { OvieCompiler } from './compiler';
import { OvieWebCompiler } from './web-compiler';
import { OvieMobileCompiler } from './mobile-compiler';
import { OvieDebugger } from './debugger';
import { OvieFormatter } from './formatter';
import { OvieProjectManager } from './project-manager';

export function activate(context: vscode.ExtensionContext) {
    console.log('Ovie extension is now active!');

    // Initialize core components
    const languageServer = new OvieLanguageServer(context);
    const aprokoProvider = new AprokoProvider(context);
    const aiProvider = new AIProvider(context);
    const compiler = new OvieCompiler(context);
    const webCompiler = new OvieWebCompiler(context);
    const mobileCompiler = new OvieMobileCompiler(context);
    const ovieDebugger = new OvieDebugger(context);
    const formatter = new OvieFormatter(context);
    const projectManager = new OvieProjectManager(context);

    // Register language server
    languageServer.start();

    // Register commands
    registerCommands(context, {
        aprokoProvider,
        aiProvider,
        compiler,
        webCompiler,
        mobileCompiler,
        ovieDebugger,
        formatter,
        projectManager
    });

    // Register providers
    registerProviders(context, {
        aprokoProvider,
        aiProvider,
        formatter
    });

    // Register event handlers
    registerEventHandlers(context);

    // Show welcome message for first-time users
    showWelcomeMessage(context);
}

function registerCommands(context: vscode.ExtensionContext, providers: any) {
    const commands = [
        // Compilation commands
        vscode.commands.registerCommand('ovie.compile', async (uri?: vscode.Uri) => {
            const document = uri ? await vscode.workspace.openTextDocument(uri) : vscode.window.activeTextEditor?.document;
            if (document && document.languageId === 'ovie') {
                await providers.compiler.compile(document);
            }
        }),

        vscode.commands.registerCommand('ovie.run', async (uri?: vscode.Uri) => {
            const document = uri ? await vscode.workspace.openTextDocument(uri) : vscode.window.activeTextEditor?.document;
            if (document && document.languageId === 'ovie') {
                await providers.compiler.run(document);
            }
        }),

        vscode.commands.registerCommand('ovie.test', async () => {
            await providers.compiler.runTests();
        }),

        // Web compilation commands
        vscode.commands.registerCommand('ovie.compile.web', async (uri?: vscode.Uri) => {
            const document = uri ? await vscode.workspace.openTextDocument(uri) : vscode.window.activeTextEditor?.document;
            if (document && document.languageId === 'ovie') {
                await providers.webCompiler.compileToWasm(document);
            }
        }),

        vscode.commands.registerCommand('ovie.project.web', async () => {
            await providers.webCompiler.createWebProject();
        }),

        // Mobile compilation commands
        vscode.commands.registerCommand('ovie.compile.android', async (uri?: vscode.Uri) => {
            const document = uri ? await vscode.workspace.openTextDocument(uri) : vscode.window.activeTextEditor?.document;
            if (document && document.languageId === 'ovie') {
                await providers.mobileCompiler.compileForAndroid(document);
            }
        }),

        vscode.commands.registerCommand('ovie.compile.ios', async (uri?: vscode.Uri) => {
            const document = uri ? await vscode.workspace.openTextDocument(uri) : vscode.window.activeTextEditor?.document;
            if (document && document.languageId === 'ovie') {
                await providers.mobileCompiler.compileForIOS(document);
            }
        }),

        vscode.commands.registerCommand('ovie.project.mobile', async () => {
            await providers.mobileCompiler.createMobileProject();
        }),

        // Aproko commands
        vscode.commands.registerCommand('ovie.aproko.analyze', async () => {
            const editor = vscode.window.activeTextEditor;
            if (editor && editor.document.languageId === 'ovie') {
                await providers.aprokoProvider.analyze(editor.document);
            }
        }),

        vscode.commands.registerCommand('ovie.aproko.fix', async () => {
            const editor = vscode.window.activeTextEditor;
            if (editor && editor.document.languageId === 'ovie') {
                await providers.aprokoProvider.applyFixes(editor.document);
            }
        }),

        vscode.commands.registerCommand('ovie.aproko.explain', async () => {
            const editor = vscode.window.activeTextEditor;
            if (editor && editor.document.languageId === 'ovie') {
                const selection = editor.selection;
                await providers.aprokoProvider.explain(editor.document, selection);
            }
        }),

        // AI commands
        vscode.commands.registerCommand('ovie.ai.complete', async () => {
            const editor = vscode.window.activeTextEditor;
            if (editor && editor.document.languageId === 'ovie') {
                await providers.aiProvider.complete(editor);
            }
        }),

        vscode.commands.registerCommand('ovie.ai.explain', async () => {
            const editor = vscode.window.activeTextEditor;
            if (editor && editor.document.languageId === 'ovie') {
                const selection = editor.selection;
                await providers.aiProvider.explain(editor.document, selection);
            }
        }),

        vscode.commands.registerCommand('ovie.ai.translate', async () => {
            const editor = vscode.window.activeTextEditor;
            if (editor) {
                await providers.aiProvider.translateToOvie(editor);
            }
        }),

        // Project commands
        vscode.commands.registerCommand('ovie.project.new', async () => {
            await providers.projectManager.createNewProject();
        }),

        vscode.commands.registerCommand('ovie.project.build', async () => {
            await providers.projectManager.buildProject();
        })
    ];

    commands.forEach(command => context.subscriptions.push(command));
}

function registerProviders(context: vscode.ExtensionContext, providers: any) {
    // Register completion provider
    const completionProvider = vscode.languages.registerCompletionItemProvider(
        'ovie',
        providers.aiProvider,
        '.', ' ', '('
    );

    // Register hover provider
    const hoverProvider = vscode.languages.registerHoverProvider(
        'ovie',
        providers.aprokoProvider
    );

    // Register diagnostic provider
    const diagnosticCollection = vscode.languages.createDiagnosticCollection('ovie');
    providers.aprokoProvider.setDiagnosticCollection(diagnosticCollection);

    // Register formatter
    const formatterProvider = vscode.languages.registerDocumentFormattingEditProvider(
        'ovie',
        providers.formatter
    );

    // Register code actions
    const codeActionProvider = vscode.languages.registerCodeActionsProvider(
        'ovie',
        providers.aprokoProvider
    );

    context.subscriptions.push(
        completionProvider,
        hoverProvider,
        diagnosticCollection,
        formatterProvider,
        codeActionProvider
    );
}

function registerEventHandlers(context: vscode.ExtensionContext) {
    // Auto-save and analysis
    const onDidSaveDocument = vscode.workspace.onDidSaveTextDocument(async (document) => {
        if (document.languageId === 'ovie') {
            const config = vscode.workspace.getConfiguration('ovie');
            if (config.get('aproko.enabled')) {
                vscode.commands.executeCommand('ovie.aproko.analyze');
            }
        }
    });

    // Auto-format on type
    const onDidChangeTextDocument = vscode.workspace.onDidChangeTextDocument(async (event) => {
        if (event.document.languageId === 'ovie') {
            const config = vscode.workspace.getConfiguration('ovie');
            if (config.get('formatting.enabled')) {
                // Debounced formatting
                setTimeout(() => {
                    vscode.commands.executeCommand('editor.action.formatDocument');
                }, 500);
            }
        }
    });

    context.subscriptions.push(onDidSaveDocument, onDidChangeTextDocument);
}

function showWelcomeMessage(context: vscode.ExtensionContext) {
    const hasShownWelcome = context.globalState.get('ovie.hasShownWelcome', false);
    
    if (!hasShownWelcome) {
        vscode.window.showInformationMessage(
            'Welcome to Ovie! 🎉 The self-hosted programming language is ready to use.',
            'Get Started',
            'View Examples',
            'Don\'t Show Again'
        ).then(selection => {
            switch (selection) {
                case 'Get Started':
                    vscode.env.openExternal(vscode.Uri.parse('https://ovie-lang.org/docs/getting-started.html'));
                    break;
                case 'View Examples':
                    vscode.env.openExternal(vscode.Uri.parse('https://ovie-lang.org/examples/'));
                    break;
                case 'Don\'t Show Again':
                    context.globalState.update('ovie.hasShownWelcome', true);
                    break;
            }
        });
    }
}

export function deactivate() {
    console.log('Ovie extension is now deactivated.');
}