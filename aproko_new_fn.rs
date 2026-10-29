/// Aproko built-in analyser - always runs on every oviec run.
/// Self-contained Rust only, no subprocess, never skips.
fn run_aproko_inline(file: &str) {
    struct H { line: usize, level: &'static str, msg: String, hint: String }
    let mut hints: Vec<H> = Vec::new();
    let mut mut_vars: std::collections::HashMap<String,(usize,usize)> = std::collections::HashMap::new();
    let mut brace_depth: i32 = 0;
    for (idx, raw) in source.lines().enumerate() {
        let ln = idx + 1;
        let t = raw.trim();
        if t.is_empty() || t.starts_with("//") { continue; }
        for ch in t.chars() {
            if ch == '{' { brace_depth += 1; } else if ch == '}' { brace_depth -= 1; }
        }
        let kws: &[&str] = &[" mut "," oya "," const "," fn "," return "," seeAm "," print("," if "," while "," for "," struct "," enum "," use "," import "," export ","}(","{"];
        let not_kw = !kws.iter().any(|k| t.starts_with(k));
        let lc1 = t.contains(" = true") || t.contains(" = false");
        let lc2 = t.contains(" = \"");
        let no_eq = !t.contains("==") && !t.contains("+=") && !t.contains("-=") && !t.contains("*=");
        if brace_depth == 0 && not_kw && (lc1 || lc2) && no_eq {
            let var = t.split_whitespace().next().unwrap_or("").to_string();
            if !var.is_empty() {
                hints.push(H{line:ln,level:"warning",
                    msg:format!("'{}' looks like a constant - use oya to prevent accidental changes",var),
                    hint:format!("oya {} = ...",var)});
            }
        }
        if t.starts_with("mut ") {
            if let Some(name) = t.split_whitespace().nth(1)
                .map(|s| s.trim_end_matches(':').trim_end_matches('=').trim().to_string())
                .filter(|n| !n.is_empty()) { mut_vars.insert(name,(ln,0)); }
        }
        let keys: Vec<String> = mut_vars.keys().cloned().collect();
        for var in &keys {
            let ra = !t.starts_with("mut ") && (
                t.starts_with(&format!("{} =",var))||t.starts_with(&format!("{} +=",var))||
                t.starts_with(&format!("{} -=",var))||t.starts_with(&format!("{} *=",var)));
            if ra { if let Some(e)=mut_vars.get_mut(var){e.1+=1;} }
        }
        if t=="while true {" || t=="while true{" {
            hints.push(H{line:ln,level:"warning",
                msg:"while true loop - add a break or use a mut running flag".to_string(),
                hint:"mut running=true; while running { ... running=false; }".to_string()});
        }
        if t.contains("/ 0") && !t.starts_with("//") {
            hints.push(H{line:ln,level:"error",
                msg:"Division by literal zero - will crash at runtime".to_string(),
                hint:"Guard it: if b != 0 { result = a / b }".to_string()});
        }
        if t.contains("read_line()") && !t.contains("string_trim") {
            hints.push(H{line:ln,level:"warning",
                msg:"read_line() not trimmed - whitespace breaks comparisons".to_string(),
                hint:"string_trim(read_line())".to_string()});
        }
        if t.contains("string_to_number(") && !t.contains("string_trim") {
            hints.push(H{line:ln,level:"warning",
                msg:"string_to_number without string_trim - whitespace silently gives 0".to_string(),
                hint:"string_to_number(string_trim(input))".to_string()});
        }
    }
    for (var,(decl_ln,count)) in &mut_vars {
        if *count==0 {
            hints.push(H{line:*decl_ln,level:"info",
                msg:format!("mut {} never reassigned - use oya for immutable values",var),
                hint:format!("oya {} = ...",var)});
        }
    }
    hints.sort_by_key(|h| h.line);
    let warn: Vec<&H> = hints.iter().filter(|h| h.level=="warning" || h.level=="error").collect();
    let show: Vec<&H> = if !warn.is_empty() { warn } else { hints.iter().collect() };
    eprintln!("\n== Aproko =============================================");
    if show.is_empty() {
        eprintln!("  OK: no issues found in {}", file);
    } else {
        for h in &show {
            eprintln!("  [{}] line {}: {}", h.level, h.line, h.msg);
            if !h.hint.is_empty() { eprintln!("    hint: {}", h.hint); }
        }
    }
    eprintln!("=====================================================\n");
}