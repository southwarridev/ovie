import * as vscode from 'vscode';
const BUILTINS: Record<string, string> = {
    string_length: 'Number', string_trim: 'String', string_split: 'Array',
    string_to_number: 'Number', number_to_string: 'String',
    string_contains: 'Boolean', string_replace: 'String',
    array_length: 'Number', array_get: 'Any', array_push: 'Array',
    read_line: 'String', print: 'Void', seeAm: 'Void'
};
interface VarInfo { name: string; type: string; mutable: boolean; line: number; }
function inferType(expr: string): string {
    expr = expr.trim();
    if (expr.startsWith("\"")) return 'String';
    if (expr === 'true' || expr === 'false') return 'Boolean';
    if (expr.startsWith('[')) return 'Array';
    if (/^-?\\d+(\\.\\d+)?$/.test(expr)) return 'Number';
    const m = expr.match(/^([a-zA-Z_]\\w*)\\s*\\(/);
    if (m && BUILTINS[m[1]]) return BUILTINS[m[1]];
    return 'Unknown';
}
function parseType(s: string): string {
    const known = ['Number','String','Boolean','Array','Any','Void'];
    return known.includes(s.trim()) ? s.trim() : 'Unknown';
}
export function analyzeDocument(doc: vscode.TextDocument) {
    const diags: vscode.Diagnostic[] = [];
    const vars = new Map<string, VarInfo>();
    const text = doc.getText().split('\n');
    for (let i = 0; i < text.length; i++) {
        const raw = text[i]; const t = raw.trim();
        if (!t || t.startsWith('//')) continue;
        const oyaM = t.match(/^oya\\s+(\\w+)(?:\\s*:\\s*(\\w+))?\\s*=\\s*(.+)/);
        if (oyaM) { vars.set(oyaM[1],{name:oyaM[1],type:oyaM[2]?parseType(oyaM[2]):inferType(oyaM[3]),mutable:false,line:i}); continue; }
        const mutM = t.match(/^mut\\s+(\\w+)(?:\\s*:\\s*(\\w+))?\\s*=\\s*(.+)/);
        if (mutM) { vars.set(mutM[1],{name:mutM[1],type:mutM[2]?parseType(mutM[2]):inferType(mutM[3]),mutable:true,line:i}); continue; }
        const reassM = t.match(/^(\\w+)\\s*=[^=]/);
        if (reassM && !t.startsWith('mut ') && !t.startsWith('oya ') && !t.startsWith('fn ')) {
            const v = vars.get(reassM[1]);
            if (v && !v.mutable) {
                const col = raw.indexOf(reassM[1]);
                const range = new vscode.Range(i, col, i, col + reassM[1].length);
                diags.push(new vscode.Diagnostic(range,"Cannot reassign " + reassM[1] + " - declared with oya (immutable). Change to mut.", vscode.DiagnosticSeverity.Error));
            }
        }
        if (t.includes('/ 0') && !t.startsWith('//')) {
            const col = raw.indexOf('/ 0');
            diags.push(new vscode.Diagnostic(new vscode.Range(i,col,i,col+3),"Division by zero - crashes at runtime.",vscode.DiagnosticSeverity.Error));
        }
    }
    return diags;
}
export class OvieTypeChecker {
    private dc = vscode.languages.createDiagnosticCollection('ovie-types');
    check(doc: vscode.TextDocument) {
        if (doc.languageId !== 'ovie') return;
        this.dc.set(doc.uri, analyzeDocument(doc));
    }
    clear(doc: vscode.TextDocument) { this.dc.delete(doc.uri); }
    dispose() { this.dc.dispose(); }
}
