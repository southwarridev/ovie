use crate::CliArgs;
use oviec::OvieResult;
use std::fs;

/// Ovie complete knowledge base as JSON.
/// Local only — no network. Feed to any LLM to teach it Ovie.
/// Contains: all keywords, types, builtins, Aproko rules, syntax patterns.
pub const KNOWLEDGE_JSON: &str = r##"
{
  "version": "2.3.0",
  "license": "MIT",
  "copyright": "Copyright (c) 2026 Nashedy Ltd",
  "aproko": "Urhobo word — one who notices everything and tells it straight",
  "keywords": {
    "oya":  {"kind":"declaration","mutability":"immutable","desc":"Immutable variable. Cannot be reassigned. Use by default.","example":"oya name = \"Ovie\""},
    "mut":  {"kind":"declaration","mutability":"mutable","desc":"Mutable variable. Can be reassigned.","example":"mut count = 0"},
    "seeAm":{"kind":"output","desc":"Print to terminal. Nigerian Pidgin English.","example":"seeAm \"Hello\""},
    "fn":   {"kind":"declaration","desc":"Define a function.","example":"fn add(a: Number, b: Number) -> Number { return a + b }"},
    "if":   {"kind":"control","desc":"Conditional branch."},
    "else": {"kind":"control","desc":"Alternative branch."},
    "while":{"kind":"control","desc":"Loop while condition is true."},
    "for":  {"kind":"control","desc":"Iterate over range or array.","example":"for i in 0..10 { seeAm i }"},
    "return":{"kind":"control","desc":"Return value from function."},
    "break":{"kind":"control","desc":"Exit a loop."},
    "continue":{"kind":"control","desc":"Skip to next loop iteration."},
    "match":{"kind":"control","desc":"Pattern matching."},
    "struct":{"kind":"declaration","desc":"Define a data structure.","example":"struct Person { name: String, age: Number }"},
    "enum": {"kind":"declaration","desc":"Define an enumeration.","example":"enum Color { Red, Green, Blue }"},
    "const":{"kind":"declaration","desc":"Compile-time constant.","example":"const MAX = 100"},
    "use":  {"kind":"module","desc":"Import from stdlib.","example":"use std::math::{sqrt}"},
    "import":{"kind":"module","desc":"Import from another .ov file."},
    "export":{"kind":"module","desc":"Export for other files."},
    "unsafe":{"kind":"safety","desc":"Mark an unsafe block."},
    "true": {"kind":"constant","desc":"Boolean true."},
    "false":{"kind":"constant","desc":"Boolean false."}
  },
  "types": {
    "Number":"integer or decimal — 42 or 3.14",
    "String":"text — \"hello\"",
    "Boolean":"true or false",
    "Array":"ordered list — [1, 2, 3]",
    "Result":"Result.Ok(val) or Result.Err(msg)",
    "Option":"Option.Some(val) or Option.None"
  },
  "builtins": {
    "string_length":"(s: String) -> Number",
    "string_trim":"(s: String) -> String",
    "string_split":"(s: String, sep: String) -> Array",
    "string_to_number":"(s: String) -> Number",
    "number_to_string":"(n: Number) -> String",
    "string_contains":"(s: String, sub: String) -> Boolean",
    "string_replace":"(s: String, from: String, to: String) -> String",
    "string_to_uppercase":"(s: String) -> String",
    "string_to_lowercase":"(s: String) -> String",
    "array_length":"(arr: Array) -> Number",
    "array_get":"(arr: Array, i: Number) -> Any",
    "array_push":"(arr: Array, val: Any) -> Array",
    "read_line":"() -> String — reads a line from stdin",
    "print":"(text: String) — prints without newline"
  },
  "aproko_rules": [
    {"rule":"prefer_oya","msg":"Use oya for values that never change","severity":"warning"},
    {"rule":"mut_never_reassigned","msg":"mut variable never reassigned — use oya","severity":"info"},
    {"rule":"while_true_loop","msg":"while true without visible break","severity":"warning"},
    {"rule":"division_by_zero","msg":"Division by literal zero","severity":"error"},
    {"rule":"read_line_not_trimmed","msg":"read_line not wrapped in string_trim","severity":"warning"},
    {"rule":"string_to_number_no_trim","msg":"string_to_number without string_trim","severity":"warning"}
  ],
  "patterns": {
    "immutable_constant":"oya NAME = value",
    "mutable_variable":"mut name = value",
    "function":"fn name(param: Type) -> ReturnType { return value }",
    "struct_create":"mut s = Name { field: value }",
    "input_loop":"mut running = true\nwhile running {\n    print(prompt)\n    mut input = string_trim(read_line())\n    if input == quit { running = false }\n}"
  },
  "stdlib": ["std::core","std::math","std::io","std::fs","std::time","std::env","std::cli","std::log","std::testing","std::module","std::aproko"]
}
"##;

/// Export Ovie knowledge base as local JSON for LLM consumption.
/// Usage:  oviec aproko export
///         oviec aproko export --output myfile.json
pub fn aproko_export(args: CliArgs) -> OvieResult<()> {
    let out = args.output_file.unwrap_or_else(|| String::from("ovie_knowledge.json"));
    // Restore any hash chars
    let json = KNOWLEDGE_JSON.replace("@HASH@", "#");
    fs::write(&out, json.as_bytes())
        .map_err(|e| oviec::OvieError::io_error(format!("Cannot write {}: {}", out, e)))?;
    println!("Aproko knowledge base exported to: {}", out);
    println!("Fully local. No network. Feed to any LLM to teach it Ovie.");
    println!("Contains: keywords, types, builtins, Aproko rules, syntax patterns.");
    Ok(())
}
