import re

BASE = r"c:\Users\Home PC\Desktop\ovie"

# ── logic.rs: add best-practice checks ──────────────────────────────────────
logic_path = BASE + r"\aproko\src\analyzers\logic.rs"
with open(logic_path, encoding="utf-8") as f:
    logic = f.read()

# Inject best-practice checker before the `impl Analyzer for LogicAnalyzer`
best_practice_fn = r'''
    /// Check for common best-practice patterns and recommend improvements
    fn check_best_practices(&self, source: &str) -> Vec<Finding> {
        let mut findings = Vec::new();
        for (i, line) in source.lines().enumerate() {
            let ln = i + 1;
            let trimmed = line.trim();

            // Recommend oya over bare assignment for fixed values
            // e.g. `name = "Ovie"` at top level without mut or oya
            if !trimmed.starts_with("//") && !trimmed.starts_with("fn ")
                && !trimmed.starts_with("mut ") && !trimmed.starts_with("oya ")
                && !trimmed.starts_with("const ") && !trimmed.starts_with("let ")
                && !trimmed.starts_with("return ") && !trimmed.starts_with("seeAm ")
                && !trimmed.starts_with("print(") && !trimmed.starts_with("if ")
                && !trimmed.starts_with("while ") && !trimmed.starts_with("for ")
                && !trimmed.starts_with("struct ") && !trimmed.starts_with("enum ")
                && !trimmed.starts_with("use ") && !trimmed.starts_with("import ")
                && !trimmed.starts_with("export ") && !trimmed.starts_with("}")
                && !trimmed.starts_with("{") && !trimmed.is_empty()
                && (trimmed.contains(" = \"") || trimmed.contains(" = true") || trimmed.contains(" = false"))
                && !trimmed.contains("==") && !trimmed.contains("+=")
                && !trimmed.contains("-=") && !trimmed.contains("*=")
            {
                findings.push(Finding {
                    category: AnalysisCategory::Logic,
                    severity: Severity::Warning,
                    message: format!("Line {ln}: bare assignment looks like a constant — consider using `oya`"),
                    suggestion: Some(format!("Change to: oya {}", trimmed)),
                    location: (ln, 1),
                    span_length: trimmed.len(),
                    rule_id: "prefer_oya_for_constants".to_string(),
                });
            }

            // Detect `while true` without a break — infinite loop warning
            if trimmed == "while true {" || trimmed == "while true{" {
                findings.push(Finding {
                    category: AnalysisCategory::Logic,
                    severity: Severity::Warning,
                    message: format!("Line {ln}: `while true` detected — make sure there is a `break` or return inside"),
                    suggestion: Some("Add a `break` condition or use a `mut running = true` pattern".to_string()),
                    location: (ln, 1),
                    span_length: trimmed.len(),
                    rule_id: "while_true_loop".to_string(),
                });
            }

            // Detect division without zero check
            if trimmed.contains("/ ") && !trimmed.starts_with("//") {
                if source[..source.lines().take(ln.saturating_sub(5)).map(|l| l.len()+1).sum::<usize>()].contains("== 0") {
                    // zero check present nearby, skip
                } else if trimmed.contains("/ 0") {
                    findings.push(Finding {
                        category: AnalysisCategory::Logic,
                        severity: Severity::Error,
                        message: format!("Line {ln}: division by literal zero"),
                        suggestion: Some("Remove or replace with a safe value".to_string()),
                        location: (ln, 1),
                        span_length: trimmed.len(),
                        rule_id: "division_by_zero".to_string(),
                    });
                }
            }

            // Recommend extracting magic numbers into oya constants
            let magic_re = regex::Regex::new(r"\b(\d{3,})\b").unwrap();
            for cap in magic_re.captures_iter(trimmed) {
                let n = &cap[1];
                if !trimmed.starts_with("oya ") && !trimmed.starts_with("const ") {
                    findings.push(Finding {
                        category: AnalysisCategory::Logic,
                        severity: Severity::Info,
                        message: format!("Line {ln}: magic number {n} — consider naming it with `oya LIMIT = {n}`"),
                        suggestion: Some(format!("oya YOUR_CONSTANT = {n}")),
                        location: (ln, 1),
                        span_length: n.len(),
                        rule_id: "magic_number".to_string(),
                    });
                }
            }
        }
        findings
    }
'''

if "fn check_best_practices" not in logic:
    logic = logic.replace(
        "    /// Try to statically evaluate a condition",
        best_practice_fn + "\n    /// Try to statically evaluate a condition"
    )

# Also inject the call inside analyze()
logic = logic.replace(
    "        // Validate control flow\n        findings.extend(self.validate_control_flow(ast));\n        \n        Ok(findings)",
    "        // Validate control flow\n        findings.extend(self.validate_control_flow(ast));\n\n        // Best-practice recommendations\n        findings.extend(self.check_best_practices(_source));\n        \n        Ok(findings)"
)

with open(logic_path, "w", encoding="utf-8") as f:
    f.write(logic)
print("logic.rs updated with best-practice checks")

# ── Add regex to aproko Cargo.toml ───────────────────────────────────────────
aproko_cargo = BASE + r"\aproko\Cargo.toml"
with open(aproko_cargo, encoding="utf-8") as f:
    acargo = f.read()
if 'regex' not in acargo:
    acargo = acargo.replace("[dependencies]", "[dependencies]\nregex = \"1.10\"")
    with open(aproko_cargo, "w", encoding="utf-8") as f:
        f.write(acargo)
    print("aproko Cargo.toml: regex added")
else:
    print("aproko Cargo.toml: regex already present")

# ── Add `use regex;` to logic.rs if not there ────────────────────────────────
with open(logic_path, encoding="utf-8") as f:
    logic2 = f.read()
if "use regex" not in logic2:
    logic2 = logic2.replace(
        "use std::collections::HashSet;",
        "use std::collections::HashSet;\n#[allow(unused_imports)]\nextern crate regex;"
    )
    with open(logic_path, "w", encoding="utf-8") as f:
        f.write(logic2)
    print("logic.rs: regex extern added")
