import re, sys

BASE = r"c:\Users\Home PC\Desktop\ovie"

# ── 1. Add aproko as dependency to oviec/Cargo.toml ─────────────────────────
cargo_path = BASE + r"\oviec\Cargo.toml"
with open(cargo_path, encoding="utf-8") as f:
    cargo = f.read()

if 'aproko = { path' not in cargo:
    cargo = cargo.replace('[dependencies]', '[dependencies]\naproko = { path = "../aproko" }', 1)
    with open(cargo_path, "w", encoding="utf-8") as f:
        f.write(cargo)
    print("Cargo.toml: aproko dep added")
else:
    print("Cargo.toml: aproko dep already present")

# ── 2. Replace run_aproko_inline in main.rs with direct library call ─────────
main_path = BASE + r"\oviec\src\main.rs"
with open(main_path, encoding="utf-8") as f:
    main_src = f.read()

# Add the aproko use statement at the top if not present
if "use aproko::" not in main_src:
    main_src = main_src.replace(
        "use oviec::{Compiler, OvieResult, Backend, AstInvariantValidation};",
        "use oviec::{Compiler, OvieResult, Backend, AstInvariantValidation};\nuse aproko::{AprokoEngine, AprokoConfig, Severity};"
    )
    print("main.rs: use aproko added")

# Replace the run_aproko_inline function with a direct library call
old_inline = '''/// Non-fatal — silently skips if oviec-analyze is not installed.
fn run_aproko_inline(file: &str) {
    let bin_name = if cfg!(windows) { "oviec-analyze.exe" } else { "oviec-analyze" };
    let analyze_path = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join(bin_name)))
        .filter(|p| p.exists())
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| bin_name.to_string());

    match std::process::Command::new(&analyze_path)
        .args(["analyze", file])
        .output()
    {
        Ok(output) => {
            let out = String::from_utf8_lossy(&output.stdout);
            // Only print if there are actual findings (non-empty beyond header)
            let lines: Vec<&str> = out.lines().collect();
            let has_findings = lines.iter().any(|l| {
                l.contains("warning") || l.contains("error") || l.contains("hint")
            });
            if has_findings {
                eprintln!("\\n== Aproko Analysis ==================================");
                for line in &lines {
                    eprintln!("{}", line);
                }
                eprintln!("=====================================================\\n");
            }
        }
        Err(_) => {} // oviec-analyze not available, skip silently
    }
}'''

new_inline = '''/// Run Aproko static analysis directly (no subprocess — library is inlined).
/// Always runs before execution. Non-fatal — warnings are informational.
fn run_aproko_inline(file: &str) {
    let source = match fs::read_to_string(file) {
        Ok(s) => s,
        Err(_) => return,
    };

    let mut compiler = oviec::Compiler::new();
    let ast = match compiler.compile_to_ast(&source) {
        Ok(a) => a,
        Err(_) => return, // parse error will be caught in run_file — don\'t double-report
    };

    let engine = AprokoEngine::new();
    let results = match engine.analyze(&source, &ast) {
        Ok(r) => r,
        Err(_) => return,
    };

    // Filter to Warning and above — don\'t spam Info in normal runs
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

if old_inline in main_src:
    main_src = main_src.replace(old_inline, new_inline)
    print("main.rs: run_aproko_inline replaced with direct call")
else:
    print("main.rs: old run_aproko_inline not found verbatim — skipping (may already be updated)")

with open(main_path, "w", encoding="utf-8") as f:
    f.write(main_src)
print("main.rs written")
