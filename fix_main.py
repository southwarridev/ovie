import sys

main_path = r"c:\Users\Home PC\Desktop\ovie\oviec\src\main.rs"

with open(main_path, encoding="utf-8") as f:
    src = f.read()

# The broken function that references AprokoEngine/Severity (no dep available)
OLD = '''/// Run Aproko static analysis directly (no subprocess \xe2\x80\x94 library is inlined).
/// Always runs before execution. Non-fatal \xe2\x80\x94 warnings are informational.
fn run_aproko_inline(file: &str) {
    let source = match fs::read_to_string(file) {
        Ok(s) => s,
        Err(_) => return,
    };

    let mut compiler = oviec::Compiler::new();
    let ast = match compiler.compile_to_ast(&source) {
        Ok(a) => a,
        Err(_) => return, // parse error will be caught in run_file \xe2\x80\x94 don\'t double-report
    };

    let engine = AprokoEngine::new();
    let results = match engine.analyze(&source, &ast) {
        Ok(r) => r,
        Err(_) => return,
    };

    // Filter to Warning and above \xe2\x80\x94 don\'t spam Info in normal runs
    let visible: Vec<_> = results.findings.iter()
        .filter(|f| f.severity >= Severity::Warning)
        .collect();

    if visible.is_empty() {
        return;
    }

    eprintln!("\\n== Aproko =====================================================");
    for f in &visible {
        let prefix = match f.severity {
            Severity::Critical | Severity::Error => "error",
            Severity::Warning                    => "warning",
            _                                    => "info",
        };
        eprintln!("  [{prefix}] line {}: {}", f.location.0, f.message);
        if let Some(ref s) = f.suggestion {
            eprintln!("    hint: {s}");
        }
    }
    eprintln!("===============================================================\\n");
}'''

# Self-contained replacement - zero external deps
NEW = r'''/// Aproko built-in analysis: runs on every oviec run before execution.
/// Self-contained — no separate binary or crate needed.
/// Checks best practices and common mistakes, prints warnings to stderr.
fn run_aproko_inline(file: &str) {
    let source = match fs::read_to_string(file) {
        Ok(s) => s,
        Err(_) => return,
    };

    #[derive(Debug)]
    struct AprokoHint { line: usize, level: &'static str, msg: String, hint: String }

    let mut hints: Vec<AprokoHint> = Vec::new();
    let mut mut_vars: std::collections::HashMap<String, (usize, usize)> = std::collections::HashMap::new(); // name -> (declare_line, reassign_count)
    let mut brace_depth: i32 = 0;

    for (i, raw) in source.lines().enumerate() {
        let ln = i + 1;
        let t = raw.trim();
        if t.is_empty() || t.starts_with("//") { continue; }

        for ch in t.chars() {
            match ch { '{' => brace_depth += 1, '}' => brace_depth -= 1, _ => {} }
        }

        // 1. Bare top-level assignment that looks like a constant -> suggest oya
        let is_top = brace_depth == 0;
        let not_keyword = !t.starts_with("mut ") && !t.starts_with("oya ")
            && !t.starts_with("const ") && !t.starts_with("fn ") && !t.starts_with("return ")
            && !t.starts_with("seeAm ") && !t.starts_with("print(") && !t.starts_with("if ")
            && !t.starts_with("while ") && !t.starts_with("for ") && !t.starts_with("struct ")
            && !t.starts_with("enum ") && !t.starts_with("use ") && !t.starts_with("import ")
            && !t.starts_with("export ") && !t.starts_with('}') && !t.starts_with('{');
        let looks_const = (t.contains(" = \"") || t.contains(" = true") || t.contains(" = false"))
            && !t.contains("==") && !t.contains("+=") && !t.contains("-=") && !t.contains("*=");

        if is_top && not_keyword && looks_const {
            let varname = t.split_whitespace().next().unwrap_or("").to_string();
            hints.push(AprokoHint {
                line: ln, level: "warning",
                msg: format!("'{}' looks like a constant — use oya to make immutability clear", varname),
                hint: format!("oya {} = ...", varname),
            });
        }

        // 2. Track mut vars for never-reassigned check
        if t.starts_with("mut ") {
            if let Some(name) = t.split_whitespace().nth(1)
                .map(|s| s.trim_end_matches(':').trim_end_matches('=').trim().to_string())
            {
                if !name.is_empty() { mut_vars.insert(name, (ln, 0)); }
            }
        }
        // Count reassignments
        let varnames: Vec<String> = mut_vars.keys().cloned().collect();
        for var in &varnames {
            if (t.starts_with(&format!("{} =", var)) || t.starts_with(&format!("{} +=", var))
                || t.starts_with(&format!("{} -=", var)) || t.starts_with(&format!("{} *=", var)))
                && !t.starts_with("mut ")
            {
                if let Some(entry) = mut_vars.get_mut(var) { entry.1 += 1; }
            }
        }

        // 3. while true warning
        if t == "while true {" || t == "while true{" {
            hints.push(AprokoHint {
                line: ln, level: "warning",
                msg: "while true loop — make sure there is a reak or set a flag to exit".to_string(),
                hint: "Use mut running = true and unning = false to exit cleanly".to_string(),
            });
        }

        // 4. Division by literal zero
        if t.contains("/ 0") && !t.starts_with("//") {
            hints.push(AprokoHint {
                line: ln, level: "error",
                msg: "Division by literal zero — this will crash at runtime".to_string(),
                hint: "Add a guard: if b != 0 { result = a / b }".to_string(),
            });
        }

        // 5. read_line() without string_trim
        if t.contains("read_line()") && !t.contains("string_trim") {
            hints.push(AprokoHint {
                line: ln, level: "info",
                msg: "read_line() without string_trim — user input may have trailing whitespace".to_string(),
                hint: "mut input = string_trim(read_line())".to_string(),
            });
        }

        // 6. string_to_number without handling empty string
        if t.contains("string_to_number(") && !t.contains("string_trim") {
            hints.push(AprokoHint {
                line: ln, level: "info",
                msg: "string_to_number without trimming first — empty/whitespace input will give 0".to_string(),
                hint: "string_to_number(string_trim(input))".to_string(),
            });
        }
    }

    // 7. mut vars never reassigned -> suggest oya
    for (var, (decl_ln, count)) in &mut_vars {
        if *count == 0 {
            hints.push(AprokoHint {
                line: *decl_ln, level: "info",
                msg: format!("'mut {}' is declared but never reassigned — prefer oya", var),
                hint: format!("oya {} = ...", var),
            });
        }
    }

    if hints.is_empty() { return; }

    // Sort by line number
    hints.sort_by_key(|h| h.line);

    // Only show warnings and errors unless verbose
    let visible: Vec<&AprokoHint> = hints.iter()
        .filter(|h| h.level == "warning" || h.level == "error")
        .collect();

    if visible.is_empty() { return; }

    eprintln!("\n== Aproko =====================================================");
    for h in &visible {
        eprintln!("  [{}] line {}: {}", h.level, h.line, h.msg);
        if !h.hint.is_empty() {
            eprintln!("    hint: {}", h.hint);
        }
    }
    eprintln!("===============================================================\n");
}'''

if OLD.encode('utf-8').decode('unicode_escape', errors='replace') in src or OLD in src:
    src = src.replace(OLD, NEW)
    print("replaced: aproko_inline is now self-contained")
else:
    # Try finding by first line
    marker = "/// Run Aproko static analysis directly (no subprocess"
    if marker in src:
        # Find start and end of that function
        start = src.find(marker)
        # Find the closing brace of the function
        pos = src.find('\nfn ', start + 1)
        if pos == -1:
            pos = len(src)
        old_block = src[start:pos].rstrip()
        src = src.replace(old_block, NEW)
        print("replaced by marker: aproko_inline is now self-contained")
    else:
        print("ERROR: could not find run_aproko_inline to replace")
        sys.exit(1)

# Also remove any stray use aproko:: lines
src_lines = src.splitlines(keepends=True)
src_lines = [l for l in src_lines if 'use aproko::' not in l]
src = ''.join(src_lines)

with open(main_path, "w", encoding="utf-8") as f:
    f.write(src)
print("main.rs written OK")
