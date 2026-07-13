//! oviec-analyze — Aproko static analysis CLI
//!
//! Called by `oviec analyze <file>` and `oviec explain error/type <file>`.
//! Installed alongside oviec.exe as a sibling binary.
//!
//! Usage:
//!   oviec-analyze analyze  <file.ov> [--format json|pretty]
//!   oviec-analyze explain  <file.ov>
//!   oviec-analyze type     <file.ov>
//!   oviec-analyze rule     <RULE_ID>

use aproko::{AprokoEngine, Severity};
use oviec::Compiler;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: oviec-analyze <command> <file> [options]");
        eprintln!("Commands: analyze, explain, type, rule");
        process::exit(1);
    }

    let command = args[1].as_str();
    let target  = &args[2];
    let json_out = args.iter().any(|a| a == "--format=json" || a == "json");

    match command {
        "analyze" => run_analyze(target, json_out),
        "explain" => run_explain_error(target),
        "type"    => run_explain_type(target),
        "rule"    => run_explain_rule(target),   // target is the rule ID here
        _ => {
            eprintln!("Unknown command: {}", command);
            process::exit(1);
        }
    }
}

fn run_analyze(file: &str, json: bool) {
    let source = match std::fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => { eprintln!("Error reading {}: {}", file, e); process::exit(1); }
    };

    let mut compiler = Compiler::new();
    let ast = match compiler.compile_to_ast(&source) {
        Ok(a) => a,
        Err(e) => { eprintln!("Parse error: {}", e); process::exit(1); }
    };

    let engine  = AprokoEngine::new();
    let results = match engine.analyze(&source, &ast) {
        Ok(r) => r,
        Err(e) => { eprintln!("Analysis error: {}", e); process::exit(1); }
    };

    if json {
        // Emit JSON array of findings
        let items: Vec<String> = results.findings.iter().map(|f| {
            format!(
                r#"{{"rule":"{}","severity":"{:?}","line":{},"col":{},"message":"{}","suggestion":{}}}"#,
                f.rule_id,
                f.severity,
                f.location.0,
                f.location.1,
                f.message.replace('"', "\\\""),
                f.suggestion.as_deref().map(|s| format!("\"{}\"", s.replace('"', "\\\""))).unwrap_or("null".to_string()),
            )
        }).collect();
        println!("[{}]", items.join(",\n "));
        return;
    }

    println!("=== Aproko Analysis: {} ===\n", file);
    println!("Lines  : {}", results.stats.lines_analyzed);
    println!("Time   : {}ms", results.stats.duration_ms);
    println!("Total  : {} finding(s)\n", results.findings.len());

    if results.findings.is_empty() {
        println!("✓ No issues found.");
        return;
    }

    for (label, icon, sev) in [
        ("Critical", "🔴", Severity::Critical),
        ("Errors",   "🟠", Severity::Error),
        ("Warnings", "🟡", Severity::Warning),
        ("Info",     "🔵", Severity::Info),
    ] {
        let group: Vec<_> = results.findings.iter().filter(|f| f.severity == sev).collect();
        if group.is_empty() { continue; }
        println!("--- {} {} ---", icon, label);
        for f in group {
            println!("  [{}] {}:{} — {}", f.rule_id, f.location.0, f.location.1, f.message);
            if let Some(ref s) = f.suggestion {
                println!("    💡 {}", s);
            }
        }
        println!();
    }
}

fn run_explain_error(file: &str) {
    let source = match std::fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => { eprintln!("Error reading {}: {}", file, e); process::exit(1); }
    };

    let mut compiler = Compiler::new();
    match compiler.compile_to_hir(&source) {
        Ok(_) => println!("✓ No errors found in {}", file),
        Err(ref error) => {
            println!("=== Error in {} ===\n", file);
            println!("{}\n", error);
            let es = error.to_string();
            if es.contains("Unexpected character") {
                println!("💡 Invalid token — check for unclosed strings or illegal characters.");
            } else if es.contains("Expected") || es.contains("Parse") {
                println!("💡 Syntax error — common causes:");
                println!("   • Missing semicolon, closing brace, or `{{`");
                println!("   • Wrong keyword (use `fn` not `function`)");
            } else if es.contains("Undefined") || es.contains("not found") {
                println!("💡 Name not found — check for typos or declaration order.");
            }

            // Aproko hints
            if let Ok(ast) = compiler.compile_to_ast(&source) {
                let engine = AprokoEngine::new();
                if let Ok(results) = engine.analyze(&source, &ast) {
                    if !results.findings.is_empty() {
                        println!("\n--- Aproko hints ---");
                        for f in results.findings.iter().take(5) {
                            println!("  {}:{} [{}] {}", f.location.0, f.location.1, f.rule_id, f.message);
                            if let Some(ref s) = f.suggestion { println!("    💡 {}", s); }
                        }
                    }
                }
            }
        }
    }
}

fn run_explain_type(file: &str) {
    let source = match std::fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => { eprintln!("Error reading {}: {}", file, e); process::exit(1); }
    };

    let mut compiler = Compiler::new();
    println!("=== Type Analysis: {} ===\n", file);

    match compiler.compile_to_hir(&source) {
        Ok(hir) => {
            println!("✓ Type checking passed\n");
            for item in &hir.items {
                if let oviec::hir::HirItem::Function(f) = item {
                    let params: Vec<String> = f.parameters.iter()
                        .map(|p| format!("{}: {:?}", p.name, p.param_type))
                        .collect();
                    println!("  fn {}({}) -> {:?}", f.name, params.join(", "), f.return_type);
                }
            }
        }
        Err(e) => {
            println!("✗ Type checking failed: {}", e);
            println!("💡 Run  oviec explain error {}  for details.", file);
        }
    }
}

fn run_explain_rule(rule_id: &str) {
    match rule_id {
        "use_after_move" => {
            println!("Rule: use_after_move");
            println!("A variable was used after being moved to another binding.");
            println!("Fix: restructure so the original is not needed after the move, or clone it.");
        }
        "hardcoded_sensitive_data" => {
            println!("Rule: hardcoded_sensitive_data");
            println!("A string literal contains an embedded secret (password=, api_key=, etc.).");
            println!("Fix: load the secret from an environment variable at runtime.");
        }
        "high_complexity" => {
            println!("Rule: high_complexity");
            println!("The function has too many independent execution paths.");
            println!("Fix: split into smaller, focused helper functions.");
        }
        "deep_nesting" => {
            println!("Rule: deep_nesting");
            println!("Code is nested more than 3 levels deep.");
            println!("Fix: use early returns (guard clauses) to flatten the structure.");
        }
        _ => {
            println!("No built-in explanation for rule '{}'.", rule_id);
            println!("Run  oviec analyze <file.ov>  to get rule IDs from your own code.");
        }
    }
}
