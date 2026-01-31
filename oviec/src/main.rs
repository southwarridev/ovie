use oviec::{Compiler, OvieResult, Backend};
use std::env;
use std::fs;
use std::process;
use std::path::Path;

#[derive(Debug)]
struct CliArgs {
    command: Command,
    input_file: Option<String>,
    output_file: Option<String>,
    backend: Option<Backend>,
    debug: bool,
    format: OutputFormat,
    rule_id: Option<String>,
}

#[derive(Debug)]
enum Command {
    Compile,
    Run,
    DumpAst,
    DumpHir,
    DumpMir,
    ReportHir,
    ReportMir,
    AnalyzeCfg,
    ExportDot,
    Check,
    Explain,
    Analyze,
    Help,
}

#[derive(Debug)]
enum OutputFormat {
    Json,
    Pretty,
    Compact,
}

fn main() {
    let args = parse_args();
    
    match run_command(args) {
        Ok(()) => {}
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(1);
        }
    }
}

fn parse_args() -> CliArgs {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help(&args[0]);
        process::exit(1);
    }
    
    let mut cli_args = CliArgs {
        command: Command::Run,
        input_file: None,
        output_file: None,
        backend: None,
        debug: false,
        format: OutputFormat::Pretty,
        rule_id: None,
    };
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "compile" => cli_args.command = Command::Compile,
            "run" => cli_args.command = Command::Run,
            "dump-ast" => cli_args.command = Command::DumpAst,
            "dump-hir" => cli_args.command = Command::DumpHir,
            "dump-mir" => cli_args.command = Command::DumpMir,
            "report-hir" => cli_args.command = Command::ReportHir,
            "report-mir" => cli_args.command = Command::ReportMir,
            "analyze-cfg" => cli_args.command = Command::AnalyzeCfg,
            "export-dot" => cli_args.command = Command::ExportDot,
            "check" => cli_args.command = Command::Check,
            "explain" => cli_args.command = Command::Explain,
            "analyze" => cli_args.command = Command::Analyze,
            "help" | "--help" | "-h" => cli_args.command = Command::Help,
            "--backend" | "-b" => {
                i += 1;
                if i < args.len() {
                    cli_args.backend = Backend::from_str(&args[i]);
                }
            }
            "--output" | "-o" => {
                i += 1;
                if i < args.len() {
                    cli_args.output_file = Some(args[i].clone());
                }
            }
            "--format" | "-f" => {
                i += 1;
                if i < args.len() {
                    cli_args.format = match args[i].as_str() {
                        "json" => OutputFormat::Json,
                        "pretty" => OutputFormat::Pretty,
                        "compact" => OutputFormat::Compact,
                        _ => OutputFormat::Pretty,
                    };
                }
            }
            "--debug" | "-d" => cli_args.debug = true,
            "--rule" | "-r" => {
                i += 1;
                if i < args.len() {
                    cli_args.rule_id = Some(args[i].clone());
                }
            }
            arg if !arg.starts_with('-') => {
                if cli_args.input_file.is_none() {
                    cli_args.input_file = Some(arg.to_string());
                }
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                process::exit(1);
            }
        }
        i += 1;
    }
    
    cli_args
}

fn run_command(args: CliArgs) -> OvieResult<()> {
    match args.command {
        Command::Help => {
            print_help("oviec");
            Ok(())
        }
        Command::Compile => compile_file(args),
        Command::Run => run_file(args),
        Command::DumpAst => dump_ast(args),
        Command::DumpHir => dump_hir(args),
        Command::DumpMir => dump_mir(args),
        Command::ReportHir => report_hir(args),
        Command::ReportMir => report_mir(args),
        Command::AnalyzeCfg => analyze_cfg(args),
        Command::ExportDot => export_dot(args),
        Command::Check => check_file(args),
        Command::Explain => explain_rule(args),
        Command::Analyze => analyze_file(args),
    }
}

fn compile_file(args: CliArgs) -> OvieResult<()> {
    let input_file = args.input_file.ok_or_else(|| {
        oviec::OvieError::io_error("No input file specified".to_string())
    })?;
    
    let source = read_source_file(&input_file)?;
    let mut compiler = create_compiler(args.backend, args.debug);
    
    // Compile to specified backend or default
    let backend = args.backend.unwrap_or(Backend::Interpreter);
    match backend {
        Backend::Wasm => {
            let _wasm_bytes = compiler.compile_to_wasm(&source)?;
            println!("WASM compilation successful");
        }
        #[cfg(feature = "llvm")]
        Backend::Llvm => {
            let _llvm_ir = compiler.compile_to_llvm(&source)?;
            println!("LLVM compilation successful");
        }
        Backend::Hir => {
            let _hir = compiler.compile_to_hir(&source)?;
            println!("HIR compilation successful");
        }
        Backend::Mir => {
            let _mir = compiler.compile_to_mir(&source)?;
            println!("MIR compilation successful");
        }
        _ => {
            compiler.compile_and_run_with_backend(&source, backend)?;
        }
    }
    
    println!("Compilation successful");
    Ok(())
}

fn run_file(args: CliArgs) -> OvieResult<()> {
    let input_file = args.input_file.ok_or_else(|| {
        oviec::OvieError::io_error("No input file specified".to_string())
    })?;
    
    let source = read_source_file(&input_file)?;
    let mut compiler = create_compiler(args.backend, args.debug);
    
    compiler.compile_and_run(&source)?;
    Ok(())
}

fn dump_ast(args: CliArgs) -> OvieResult<()> {
    let input_file = args.input_file.ok_or_else(|| {
        oviec::OvieError::io_error("No input file specified".to_string())
    })?;
    
    let source = read_source_file(&input_file)?;
    let mut compiler = create_compiler(None, args.debug);
    
    let ast = compiler.compile_to_ast(&source)?;
    let output = format_ast_output(&ast, &args.format)?;
    
    write_output(output, args.output_file)?;
    Ok(())
}

fn dump_hir(args: CliArgs) -> OvieResult<()> {
    let input_file = args.input_file.ok_or_else(|| {
        oviec::OvieError::io_error("No input file specified".to_string())
    })?;
    
    let source = read_source_file(&input_file)?;
    let mut compiler = create_compiler(None, args.debug);
    
    let hir = compiler.compile_to_hir(&source)?;
    let output = format_hir_output(&hir, &args.format)?;
    
    write_output(output, args.output_file)?;
    Ok(())
}

fn dump_mir(args: CliArgs) -> OvieResult<()> {
    let input_file = args.input_file.ok_or_else(|| {
        oviec::OvieError::io_error("No input file specified".to_string())
    })?;
    
    let source = read_source_file(&input_file)?;
    let mut compiler = create_compiler(None, args.debug);
    
    let mir = compiler.compile_to_mir(&source)?;
    let output = format_mir_output(&mir, &args.format)?;
    
    write_output(output, args.output_file)?;
    Ok(())
}

fn check_file(args: CliArgs) -> OvieResult<()> {
    let input_file = args.input_file.ok_or_else(|| {
        oviec::OvieError::io_error("No input file specified".to_string())
    })?;
    
    let source = read_source_file(&input_file)?;
    let mut compiler = create_compiler(None, args.debug);
    
    // Compile to HIR to check for semantic errors
    let _hir = compiler.compile_to_hir(&source)?;
    
    println!("✓ {} - No errors found", input_file);
    Ok(())
}

fn report_hir(args: CliArgs) -> OvieResult<()> {
    let input_file = args.input_file.ok_or_else(|| {
        oviec::OvieError::io_error("No input file specified".to_string())
    })?;
    
    let source = read_source_file(&input_file)?;
    let mut compiler = create_compiler(None, args.debug);
    
    let hir = compiler.compile_to_hir(&source)?;
    let report = hir.generate_hir_report()?;
    
    write_output(report, args.output_file)?;
    Ok(())
}

fn report_mir(args: CliArgs) -> OvieResult<()> {
    let input_file = args.input_file.ok_or_else(|| {
        oviec::OvieError::io_error("No input file specified".to_string())
    })?;
    
    let source = read_source_file(&input_file)?;
    let mut compiler = create_compiler(None, args.debug);
    
    let mir = compiler.compile_to_mir(&source)?;
    let report = mir.generate_ir_report()?;
    
    write_output(report, args.output_file)?;
    Ok(())
}

fn analyze_cfg(args: CliArgs) -> OvieResult<()> {
    let input_file = args.input_file.ok_or_else(|| {
        oviec::OvieError::io_error("No input file specified".to_string())
    })?;
    
    let source = read_source_file(&input_file)?;
    let mut compiler = create_compiler(None, args.debug);
    
    let mir = compiler.compile_to_mir(&source)?;
    let cfg_analysis = mir.analyze_cfg()?;
    
    let mut report = String::new();
    report.push_str("=== Control Flow Graph Analysis ===\n\n");
    
    for (func_id, func_analysis) in &cfg_analysis.function_analyses {
        if let Some(function) = mir.functions.get(func_id) {
            report.push_str(&format!("Function: {} (ID: {})\n", function.name, func_id));
            report.push_str(&format!("  Basic Blocks: {}\n", function.basic_blocks.len()));
            report.push_str(&format!("  Loops Detected: {}\n", func_analysis.loops.len()));
            
            // Show predecessor/successor relationships
            report.push_str("  Control Flow:\n");
            for (block_id, successors) in &func_analysis.successors {
                if !successors.is_empty() {
                    report.push_str(&format!("    Block {} -> {:?}\n", block_id, successors));
                }
            }
            
            // Show loop information
            if !func_analysis.loops.is_empty() {
                report.push_str("  Loops:\n");
                for (i, loop_info) in func_analysis.loops.iter().enumerate() {
                    report.push_str(&format!("    Loop {}: Header={}, Back Edge Source={}\n", 
                                           i, loop_info.header, loop_info.back_edge_source));
                }
            }
            
            report.push_str("\n");
        }
    }
    
    write_output(report, args.output_file)?;
    Ok(())
}

fn export_dot(args: CliArgs) -> OvieResult<()> {
    let input_file = args.input_file.ok_or_else(|| {
        oviec::OvieError::io_error("No input file specified".to_string())
    })?;
    
    let source = read_source_file(&input_file)?;
    let mut compiler = create_compiler(None, args.debug);
    
    let mir = compiler.compile_to_mir(&source)?;
    let dot_output = mir.to_dot()?;
    
    write_output(dot_output, args.output_file)?;
    Ok(())
}

fn read_source_file(filename: &str) -> OvieResult<String> {
    fs::read_to_string(filename)
        .map_err(|e| oviec::OvieError::io_error(format!("Could not read file '{}': {}", filename, e)))
}

fn create_compiler(backend: Option<Backend>, debug: bool) -> Compiler {
    let mut compiler = Compiler::new();
    compiler.debug = debug;
    compiler
}

fn create_compiler(backend: Option<Backend>, debug: bool) -> Compiler {
    let mut compiler = Compiler::new();
    compiler.debug = debug;
    compiler
}

fn explain_rule(args: CliArgs) -> OvieResult<()> {
    let rule_id = args.rule_id.ok_or_else(|| {
        oviec::OvieError::io_error("No rule ID specified. Use --rule <RULE_ID>".to_string())
    })?;
    
    let aproko_engine = aproko::AprokoEngine::new();
    let explanation_engine = aproko_engine.explanation_engine();
    
    if let Some(explanation) = explanation_engine.get_explanation(&rule_id) {
        let mut output = String::new();
        output.push_str(&format!("=== Explanation for Rule {} ===\n\n", rule_id));
        output.push_str(&format!("Summary: {}\n\n", explanation.summary));
        output.push_str(&format!("Type: {:?}\n", explanation.explanation_type));
        output.push_str(&format!("Confidence: {:.2}\n\n", explanation.confidence));
        
        output.push_str("Detailed Explanation:\n");
        output.push_str(&explanation.detailed_explanation);
        output.push_str("\n\n");
        
        if !explanation.code_examples.is_empty() {
            output.push_str("Code Examples:\n");
            for (i, example) in explanation.code_examples.iter().enumerate() {
                output.push_str(&format!("  {}. {} ({})\n", 
                    i + 1, 
                    example.description,
                    if example.is_good_example { "Good" } else { "Bad" }
                ));
                output.push_str(&format!("     ```{}\n", example.language));
                output.push_str(&format!("     {}\n", example.code));
                output.push_str("     ```\n");
                if let Some(ref notes) = example.notes {
                    output.push_str(&format!("     Note: {}\n", notes));
                }
                output.push_str("\n");
            }
        }
        
        if !explanation.fix_suggestions.is_empty() {
            output.push_str("Fix Suggestions:\n");
            for (i, fix) in explanation.fix_suggestions.iter().enumerate() {
                output.push_str(&format!("  {}. {} (Difficulty: {:?}, Confidence: {:.2})\n", 
                    i + 1, fix.title, fix.difficulty, fix.confidence));
                output.push_str(&format!("     {}\n", fix.description));
                
                if !fix.steps.is_empty() {
                    output.push_str("     Steps:\n");
                    for step in &fix.steps {
                        output.push_str(&format!("       {}. {}\n", step.step_number, step.description));
                        if let Some(ref notes) = step.notes {
                            output.push_str(&format!("          Note: {}\n", notes));
                        }
                    }
                }
                output.push_str("\n");
            }
        }
        
        if !explanation.related_topics.is_empty() {
            output.push_str(&format!("Related Topics: {}\n", explanation.related_topics.join(", ")));
        }
        
        write_output(output, args.output_file)?;
    } else {
        println!("No explanation found for rule: {}", rule_id);
        println!("Available rules:");
        let all_explanations = explanation_engine.get_all_explanations();
        for rule_id in all_explanations.keys() {
            println!("  {}", rule_id);
        }
    }
    
    Ok(())
}

fn analyze_file(args: CliArgs) -> OvieResult<()> {
    let input_file = args.input_file.ok_or_else(|| {
        oviec::OvieError::io_error("No input file specified".to_string())
    })?;
    
    let source = read_source_file(&input_file)?;
    let mut compiler = create_compiler(None, args.debug);
    
    // Compile to AST for analysis
    let ast = compiler.compile_to_ast(&source)?;
    
    // Run dedicated Aproko analysis
    let mut aproko_engine = aproko::AprokoEngine::new();
    let analysis_result = aproko_engine.analyze(&source, &ast)?;
    
    let mut output = String::new();
    output.push_str(&format!("=== Aproko Analysis Report for {} ===\n\n", input_file));
    
    // Summary statistics
    output.push_str("Analysis Summary:\n");
    output.push_str(&format!("  Lines analyzed: {}\n", analysis_result.stats.lines_analyzed));
    output.push_str(&format!("  Analysis duration: {}ms\n", analysis_result.stats.duration_ms));
    output.push_str(&format!("  Total findings: {}\n", analysis_result.findings.len()));
    output.push_str(&format!("  Total diagnostics: {}\n", analysis_result.diagnostics.len()));
    output.push_str("\n");
    
    // Findings by severity
    if !analysis_result.stats.findings_by_severity.is_empty() {
        output.push_str("Findings by Severity:\n");
        for (severity, count) in &analysis_result.stats.findings_by_severity {
            output.push_str(&format!("  {:?}: {}\n", severity, count));
        }
        output.push_str("\n");
    }
    
    // Findings by category
    if !analysis_result.stats.findings_by_category.is_empty() {
        output.push_str("Findings by Category:\n");
        for (category, count) in &analysis_result.stats.findings_by_category {
            output.push_str(&format!("  {:?}: {}\n", category, count));
        }
        output.push_str("\n");
    }
    
    // Detailed diagnostics
    if !analysis_result.diagnostics.is_empty() {
        output.push_str("Detailed Diagnostics:\n");
        for (i, diagnostic) in analysis_result.diagnostics.iter().enumerate() {
            output.push_str(&format!("  {}. [{}] {} ({}:{}:{})\n", 
                i + 1,
                diagnostic.rule_id,
                diagnostic.message,
                diagnostic.location.file,
                diagnostic.location.line,
                diagnostic.location.column
            ));
            output.push_str(&format!("     Severity: {:?}, Category: {:?}\n", 
                diagnostic.severity, diagnostic.category));
            
            // Show explanation if available
            if let Ok(explanation) = aproko_engine.explain_diagnostic(diagnostic) {
                output.push_str(&format!("     Explanation: {}\n", explanation.summary));
                
                if !explanation.fix_suggestions.is_empty() {
                    let fix = &explanation.fix_suggestions[0];
                    output.push_str(&format!("     Suggested fix: {} (Difficulty: {:?})\n", 
                        fix.title, fix.difficulty));
                }
            }
            output.push_str("\n");
        }
    } else {
        output.push_str("✓ No issues found!\n");
    }
    
    write_output(output, args.output_file)?;
    Ok(())
}

fn format_ast_output(ast: &oviec::ast::AstNode, format: &OutputFormat) -> OvieResult<String> {
    match format {
        OutputFormat::Json => {
            serde_json::to_string_pretty(ast)
                .map_err(|e| oviec::OvieError::io_error(format!("JSON serialization error: {}", e)))
        }
        OutputFormat::Pretty => {
            Ok(format!("{:#?}", ast))
        }
        OutputFormat::Compact => {
            serde_json::to_string(ast)
                .map_err(|e| oviec::OvieError::io_error(format!("JSON serialization error: {}", e)))
        }
    }
}

fn format_hir_output(hir: &oviec::hir::HirProgram, format: &OutputFormat) -> OvieResult<String> {
    match format {
        OutputFormat::Json => hir.to_json(),
        OutputFormat::Pretty => {
            Ok(format!("{:#?}", hir))
        }
        OutputFormat::Compact => {
            serde_json::to_string(hir)
                .map_err(|e| oviec::OvieError::IrError { message: format!("HIR serialization error: {}", e) })
        }
    }
}

fn format_mir_output(mir: &oviec::mir::MirProgram, format: &OutputFormat) -> OvieResult<String> {
    match format {
        OutputFormat::Json => mir.to_json(),
        OutputFormat::Pretty => {
            Ok(format!("{:#?}", mir))
        }
        OutputFormat::Compact => {
            serde_json::to_string(mir)
                .map_err(|e| oviec::OvieError::IrError { message: format!("MIR serialization error: {}", e) })
        }
    }
}

fn write_output(output: String, output_file: Option<String>) -> OvieResult<()> {
    match output_file {
        Some(filename) => {
            fs::write(&filename, output)
                .map_err(|e| oviec::OvieError::io_error(format!("Could not write to file '{}': {}", filename, e)))?;
            println!("Output written to {}", filename);
        }
        None => {
            println!("{}", output);
        }
    }
    Ok(())
}

fn print_help(program_name: &str) {
    println!("Ovie Compiler - Stage 2 Multi-IR Pipeline");
    println!();
    println!("USAGE:");
    println!("    {} <COMMAND> [OPTIONS] <INPUT_FILE>", program_name);
    println!();
    println!("COMMANDS:");
    println!("    run                 Compile and run the program (default)");
    println!("    compile             Compile the program without running");
    println!("    check               Check the program for errors without compilation");
    println!("    analyze             Run Aproko analysis and show detailed report");
    println!("    explain             Show explanation for a specific diagnostic rule");
    println!("    dump-ast            Dump the Abstract Syntax Tree");
    println!("    dump-hir            Dump the High-level Intermediate Representation");
    println!("    dump-mir            Dump the Mid-level Intermediate Representation");
    println!("    report-hir          Generate human-readable HIR analysis report");
    println!("    report-mir          Generate human-readable MIR analysis report");
    println!("    analyze-cfg         Analyze control flow graph and show analysis");
    println!("    export-dot          Export MIR control flow graph in GraphViz DOT format");
    println!("    help                Show this help message");
    println!();
    println!("OPTIONS:");
    println!("    -b, --backend <BACKEND>     Compilation backend [interpreter, llvm, wasm, hir, mir]");
    println!("    -o, --output <FILE>         Output file (default: stdout)");
    println!("    -f, --format <FORMAT>       Output format [json, pretty, compact] (default: pretty)");
    println!("    -r, --rule <RULE_ID>        Specific diagnostic rule ID for explain command");
    println!("    -d, --debug                 Enable debug output");
    println!("    -h, --help                  Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("    {} run hello.ov                    # Run a program");
    println!("    {} analyze hello.ov                # Run Aproko analysis");
    println!("    {} explain --rule E001             # Explain diagnostic rule E001");
    println!("    {} compile -b llvm hello.ov        # Compile with LLVM backend");
    println!("    {} dump-hir -f json hello.ov       # Dump HIR as JSON");
    println!("    {} dump-mir -o output.json hello.ov # Dump MIR to file");
    println!("    {} report-mir hello.ov             # Generate MIR analysis report");
    println!("    {} analyze-cfg hello.ov            # Analyze control flow graph");
    println!("    {} export-dot -o cfg.dot hello.ov  # Export CFG for visualization");
    println!("    {} check hello.ov                  # Check for errors");
}
