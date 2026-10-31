import re
p = r'c:\Users\Home PC\Desktop\ovie\oviec\src\main.rs'
src = open(p, encoding='utf-8').read()
original_len = len(src)
Q = chr(34); NL = chr(10)

# -- FIX 1: get_inline_hint -----------------------------------------
OLD_HINT_START = 'if msg.contains(' + Q + 'undefined variable' + Q + ') {'
OLD_HINT_END = 'Run ' + chr(39) + 'oviec analyze <file.ov>' + chr(39) + ' for a detailed breakdown of this error.'
hs = src.find(OLD_HINT_START)
he = src.find(OLD_HINT_END, hs) + len(OLD_HINT_END)
# Find the closing brace of the function after OLD_HINT_END
fn_end = src.find(NL + '}', he) + 2
print(f'hint fn body: {hs} to {fn_end}')

# Build the new get_inline_hint body replacement
new_hint_body = (
    'if msg.contains(' + Q + 'undefined variable' + Q + ') {' + NL
    + '        let parts: Vec<&str> = msg.split(' + chr(39) + ':' + chr(39) + ').collect();' + NL
    + '        if parts.len() > 1 {' + NL
    + '            let var_name = parts[1].trim();' + NL
    + '            let bad_kw = [' + Q + 'see' + Q + ',' + Q + 'var' + Q + ',' + Q + 'let' + Q + ',' + Q + 'def' + Q + ',' + Q + 'val' + Q + ',' + Q + 'dim' + Q + ',' + Q + 'function' + Q + '];' + NL
    + '            if bad_kw.contains(&var_name) {' + NL
    + '                return format!(' + Q + chr(39) + '{}' + chr(39) + ' is not an Ovie keyword. Use oya (immutable) or mut (mutable). Try: oya name = value' + Q + ', var_name);' + NL
    + '            }' + NL
    + '            return format!(' + Q + 'Variable ' + chr(39) + '{}' + chr(39) + ' is not declared. Use: oya {} = value (immutable) or mut {} = value (mutable)' + Q + ', var_name, var_name, var_name);' + NL
    + '        }' + NL
    + '        return ' + Q + 'Variable not declared. Use oya name = value (immutable) or mut name = value (mutable).' + Q + '.to_string();' + NL
    + '    }' + NL
    + '    if msg.contains(' + Q + 'undefined function' + Q + ') {' + NL
    + '        let parts: Vec<&str> = msg.split(' + chr(39) + ':' + chr(39) + ').collect();' + NL
    + '        if parts.len() > 1 { let fn_name = parts[1].trim(); return format!(' + Q + 'Function ' + chr(39) + '{}' + chr(39) + ' is not defined. Check spelling or import the module.' + Q + ', fn_name); }' + NL
    + '        return ' + Q + 'Function not defined. Check spelling or use std::module::{fn_name}.' + Q + '.to_string();' + NL
    + '    }' + NL
    + '    if msg.contains(' + Q + 'parse error' + Q + ') || msg.contains(' + Q + 'expected' + Q + ') {' + NL
    + '        return ' + Q + 'Syntax error. Common causes: wrong keyword (use oya not see, use fn not function), missing brace, missing colon.' + Q + '.to_string();' + NL
    + '    }' + NL
    + '    if msg.contains(' + Q + 'division by zero' + Q + ') { return ' + Q + 'Division by zero. Add guard: if b != 0 { result = a / b }' + Q + '.to_string(); }' + NL
    + '    if msg.contains(' + Q + 'type' + Q + ') || msg.contains(' + Q + 'cannot' + Q + ') { return ' + Q + 'Type mismatch. Use number_to_string() to convert numbers before concatenating with strings.' + Q + '.to_string(); }' + NL
    + '    format!(' + Q + 'Error: {}. Check the line shown above.' + Q + ', msg)' + NL
)

src = src[:hs] + new_hint_body + src[fn_end:]
print('FIX 1 done: hint body replaced')

# -- FIX 2: explain_error_inline ------------------------------------
OLD_EXPLAIN = 'let bin_name = if cfg!(windows) { ' + Q + 'oviec-analyze.exe' + Q + ' } else { ' + Q + 'oviec-analyze' + Q + ' };'
es = src.find(OLD_EXPLAIN)
# Walk back to find fn signature start
fn_sig = 'fn explain_error_inline(file: &str, error: &oviec::OvieError) {'
es_fn = src.rfind(fn_sig, 0, es)
# Walk back to doc comment
es_doc = src.rfind('/// After a runtime', 0, es_fn)
# Find end of this function by counting braces
brace_start = src.find('{', es_fn)
depth = 0; ee = brace_start
for i, ch in enumerate(src[brace_start:], brace_start):
    if ch == '{': depth += 1
    elif ch == '}': depth -= 1
    if depth == 0: ee = i; break
print(f'explain fn: doc={es_doc} fn={es_fn} end={ee}')

new_explain = (
    '/// Aproko explains what went wrong in plain language.' + NL
    + '/// Shows source line + specific hint. No subprocess.' + NL
    + 'fn explain_error_inline(file: &str, error: &oviec::OvieError) {' + NL
    + '    let error_msg = error.to_string();' + NL
    + '    let hint = get_inline_hint(error);' + NL
    + '    if let Ok(source) = fs::read_to_string(file) {' + NL
    + '        let line_num = extract_line_number(&error_msg);' + NL
    + '        if let Some(ln) = line_num {' + NL
    + '            let lines: Vec<&str> = source.lines().collect();' + NL
    + '            if ln > 0 && ln <= lines.len() {' + NL
    + '                let line_content = lines[ln - 1].trim();' + NL
    + '                eprintln!(' + Q + chr(92) + 'n== Aproko Explanation =============================|' + Q + ');' + NL
    + '                eprintln!(' + Q + '  Line {}: {}' + Q + ', ln, line_content);' + NL
    + '                eprintln!(' + Q + '  Error: {}' + Q + ', &error_msg);' + NL
    + '                eprintln!(' + Q + '  What happened: {}' + Q + ', &hint);' + NL
    + '                eprintln!(' + Q + '==============================================|' + Q + ');' + NL
    + '                return;' + NL
    + '            }' + NL
    + '        }' + NL
    + '    }' + NL
    + '    eprintln!(' + Q + chr(92) + 'n== Aproko Explanation =============================|' + Q + ');' + NL
    + '    eprintln!(' + Q + '  Error: {}' + Q + ', &error_msg);' + NL
    + '    eprintln!(' + Q + '  What happened: {}' + Q + ', &hint);' + NL
    + '    eprintln!(' + Q + '==============================================|' + Q + ');' + NL
    + '}'
)

src = src[:es_doc] + new_explain + src[ee+1:]
print('FIX 2 done: explain_error_inline replaced')

open(p, 'w', encoding='utf-8').write(src)
print('main.rs written. Size:', len(src))
