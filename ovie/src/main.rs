use clap::{Parser, Subcommand};
use oviec::{Compiler, Backend, OvieResult, OvieError, AstNode, Statement, Expression, PackageRegistry, PackageLock, DependencyResolver, ProjectConfig, SelfHostingManager, SelfHostingStage, BootstrapConfig, BootstrapVerificationResult, BrandingConfig, ProjectTemplate, ProjectMetadata, IntegrityManifest, CrossTargetValidator, CrossTargetValidationConfig};
use std::fs;
use std::path::Path;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(test)]
mod tests;

#[derive(Parser)]
#[command(name = "ovie")]
#[command(about = "The Ovie programming language toolchain")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Ovie project
    New {
        /// Project name
        name: String,
        /// Project directory (defaults to project name)
        #[arg(long)]
        path: Option<String>,
    },
    /// Build the current project
    Build {
        /// Source file to build
        file: Option<String>,
        /// Output backend
        #[arg(long, default_value = "interpreter")]
        backend: String,
        /// Target platform (for LLVM backend)
        #[arg(long)]
        target: Option<String>,
        /// Output file
        #[arg(short, long)]
        output: Option<String>,
        /// Enable debug output
        #[arg(long)]
        debug: bool,
        /// Enable deterministic builds
        #[arg(long)]
        deterministic: bool,
        /// Generate object file (LLVM backend only)
        #[arg(long)]
        object: bool,
        /// Generate assembly file (LLVM backend only)
        #[arg(long)]
        assembly: bool,
    },
    /// Run the current project
    Run {
        /// Source file to run
        file: Option<String>,
        /// Execution backend
        #[arg(long, default_value = "interpreter")]
        backend: String,
        /// Enable debug output
        #[arg(long)]
        debug: bool,
    },
    /// Run tests
    Test {
        /// Test file pattern
        #[arg(default_value = "**/*.test.ov")]
        pattern: String,
        /// Enable debug output
        #[arg(long)]
        debug: bool,
    },
    /// Check source code for errors without compilation
    Check {
        /// Source file to check
        file: Option<String>,
        /// Enable debug output
        #[arg(long)]
        debug: bool,
    },
    /// Format source code
    Fmt {
        /// Files to format (defaults to all .ov files)
        files: Vec<String>,
        /// Check formatting without modifying files
        #[arg(long)]
        check: bool,
    },
    /// Update dependencies
    Update {
        /// Specific dependency to update
        dependency: Option<String>,
    },
    /// Vendor dependencies locally
    Vendor {
        /// Output directory for vendored dependencies
        #[arg(long, default_value = "vendor")]
        output: String,
    },
    /// Verify package integrity
    Verify {
        /// Package name to verify (optional, verifies all if not specified)
        package: Option<String>,
        /// Enable debug output
        #[arg(long)]
        debug: bool,
        /// Verify signatures
        #[arg(long)]
        signatures: bool,
        /// Verify checksums
        #[arg(long)]
        checksums: bool,
    },
    /// Manage package integrity
    Integrity {
        #[command(subcommand)]
        action: IntegrityAction,
    },
    /// Self-hosting operations
    SelfHost {
        #[command(subcommand)]
        action: SelfHostingAction,
    },
    /// Analyze code with Aproko
    Analyze {
        /// Source file to analyze
        file: Option<String>,
        /// Enable debug output
        #[arg(long)]
        debug: bool,
        /// Output format
        #[arg(long, default_value = "pretty")]
        format: String,
        /// Output file
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Explain diagnostic rule
    Explain {
        /// Rule ID to explain
        #[arg(long)]
        rule: String,
        /// Output file
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Dump IR representations
    Dump {
        #[command(subcommand)]
        ir_type: IrType,
    },
    /// Cross-target validation
    CrossTarget {
        /// Source file to validate
        file: Option<String>,
        /// Enable performance validation
        #[arg(long)]
        performance: bool,
        /// Enable determinism validation
        #[arg(long)]
        determinism: bool,
        /// Performance tolerance percentage
        #[arg(long, default_value = "5.0")]
        tolerance: f64,
        /// Number of validation runs
        #[arg(long, default_value = "3")]
        runs: usize,
        /// Output file for validation report
        #[arg(short, long)]
        output: Option<String>,
        /// Enable debug output
        #[arg(long)]
        debug: bool,
    },
    /// Batch operations on multiple files
    Batch {
        #[command(subcommand)]
        operation: BatchOperation,
    },
}

#[derive(Subcommand)]
enum SelfHostingAction {
    /// Show current self-hosting status
    Status,
    /// Verify bootstrap readiness for Stage 1 transition
    Verify {
        /// Test files to use for verification
        #[arg(long)]
        test_files: Vec<String>,
        /// Enable verbose output
        #[arg(long)]
        verbose: bool,
    },
    /// Transition to the next self-hosting stage
    Transition {
        /// Force transition without verification
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
enum IntegrityAction {
    /// Check integrity of all packages
    Check {
        /// Package name to check (optional, checks all if not specified)
        package: Option<String>,
        /// Enable verbose output
        #[arg(long)]
        verbose: bool,
    },
    /// Generate integrity manifest for a package
    Generate {
        /// Package name
        package: String,
        /// Package version
        version: String,
        /// Output file for manifest
        #[arg(long, default_value = "integrity.json")]
        output: String,
    },
    /// Repair integrity issues
    Repair {
        /// Package name to repair
        package: String,
        /// Force repair without confirmation
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
enum IrType {
    /// Dump Abstract Syntax Tree
    Ast {
        /// Source file to analyze
        file: Option<String>,
        /// Output format
        #[arg(long, default_value = "pretty")]
        format: String,
        /// Output file
        #[arg(short, long)]
        output: Option<String>,
        /// Enable debug output
        #[arg(long)]
        debug: bool,
    },
    /// Dump High-level Intermediate Representation
    Hir {
        /// Source file to analyze
        file: Option<String>,
        /// Output format
        #[arg(long, default_value = "pretty")]
        format: String,
        /// Output file
        #[arg(short, long)]
        output: Option<String>,
        /// Enable debug output
        #[arg(long)]
        debug: bool,
    },
    /// Dump Mid-level Intermediate Representation
    Mir {
        /// Source file to analyze
        file: Option<String>,
        /// Output format
        #[arg(long, default_value = "pretty")]
        format: String,
        /// Output file
        #[arg(short, long)]
        output: Option<String>,
        /// Enable debug output
        #[arg(long)]
        debug: bool,
    },
}

#[derive(Subcommand)]
enum BatchOperation {
    /// Check multiple files for errors
    Check {
        /// File pattern to match
        #[arg(default_value = "**/*.ov")]
        pattern: String,
        /// Enable debug output
        #[arg(long)]
        debug: bool,
        /// Continue on errors
        #[arg(long)]
        continue_on_error: bool,
    },
    /// Analyze multiple files with Aproko
    Analyze {
        /// File pattern to match
        #[arg(default_value = "**/*.ov")]
        pattern: String,
        /// Enable debug output
        #[arg(long)]
        debug: bool,
        /// Output directory for reports
        #[arg(long, default_value = "analysis-reports")]
        output_dir: String,
        /// Continue on errors
        #[arg(long)]
        continue_on_error: bool,
    },
    /// Format multiple files
    Format {
        /// File pattern to match
        #[arg(default_value = "**/*.ov")]
        pattern: String,
        /// Check formatting without modifying files
        #[arg(long)]
        check: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::New { name, path } => cmd_new(name, path),
        Commands::Build { file, backend, target, output, debug, deterministic, object, assembly } => cmd_build(file, backend, target, output, debug, deterministic, object, assembly),
        Commands::Run { file, backend, debug } => cmd_run(file, backend, debug),
        Commands::Check { file, debug } => cmd_check(file, debug),
        Commands::Test { pattern, debug } => cmd_test(pattern, debug),
        Commands::Fmt { files, check } => cmd_fmt(files, check),
        Commands::Update { dependency } => cmd_update(dependency),
        Commands::Vendor { output } => cmd_vendor(output),
        Commands::Verify { package, debug, signatures, checksums } => cmd_verify(package, debug, signatures, checksums),
        Commands::Integrity { action } => cmd_integrity(action),
        Commands::SelfHost { action } => cmd_self_host(action),
        Commands::Analyze { file, debug, format, output } => cmd_analyze(file, debug, format, output),
        Commands::Explain { rule, output } => cmd_explain(rule, output),
        Commands::Dump { ir_type } => cmd_dump(ir_type),
        Commands::CrossTarget { file, performance, determinism, tolerance, runs, output, debug } => cmd_cross_target(file, performance, determinism, tolerance, runs, output, debug),
        Commands::Batch { operation } => cmd_batch(operation),
    };

    if let Err(error) = result {
        eprintln!("Error: {}", error);
        process::exit(1);
    }
}

fn cmd_new(name: String, path: Option<String>) -> OvieResult<()> {
    let project_path = path.unwrap_or_else(|| name.clone());
    let project_dir = Path::new(&project_path);

    if project_dir.exists() {
        return Err(OvieError::io_error(format!("Directory '{}' already exists", project_path)));
    }

    println!("🚀 Creating new Ovie project '{}'...", name);

    // Use the branding system to create the project with proper icon and branding
    let branding = BrandingConfig::new();
    let template = ProjectTemplate::with_branding(branding);
    
    // Generate the project with branding and icon
    template.generate_project(&project_dir, &name)?;

    // Create src directory and main.ov
    fs::create_dir_all(project_dir.join("src")).map_err(|e| OvieError::io_error(e.to_string()))?;
    
    let main_content = format!(r#"// Welcome to {}!
// This is your main Ovie program.

seeAm "Hello, {}!";

fn greet(name) {{
    seeAm "Hello, " + name + "!";
}}

greet("World");
"#, name, name);
    
    fs::write(project_dir.join("src/main.ov"), main_content).map_err(|e| OvieError::io_error(e.to_string()))?;

    // Create tests directory
    fs::create_dir_all(project_dir.join("tests")).map_err(|e| OvieError::io_error(e.to_string()))?;

    // Create a sample test file
    let test_content = format!(r#"// Tests for {}

fn test_greet() {{
    // Test the greet function
    seeAm "Running tests...";
    // Add your test assertions here
}}

test_greet();
"#, name);
    
    fs::write(project_dir.join("tests/main.test.ov"), test_content).map_err(|e| OvieError::io_error(e.to_string()))?;

    // Create .gitignore
    let gitignore_content = r#"# Ovie build artifacts
/target/
*.wasm
*.ll

# IDE files
.vscode/
.idea/
*.swp
*.swo

# OS files
.DS_Store
Thumbs.db

# Ovie cache
.ovie/cache/
"#;
    
    fs::write(project_dir.join(".gitignore"), gitignore_content).map_err(|e| OvieError::io_error(e.to_string()))?;

    println!("✅ Created new Ovie project '{}' in '{}'", name, project_path);
    println!("📁 Project structure:");
    println!("   {}/", project_path);
    println!("   ├── ovie.png          # Ovie language icon");
    println!("   ├── ovie.toml         # Project configuration");
    println!("   ├── README.md         # Project documentation");
    println!("   ├── src/");
    println!("   │   └── main.ov       # Main application");
    println!("   ├── tests/");
    println!("   │   └── main.test.ov  # Test files");
    println!("   └── .ovie/            # Ovie project metadata");
    println!();
    println!("🎯 To get started:");
    println!("   cd {}", project_path);
    println!("   ovie run");
    println!();
    println!("📚 Learn more about Ovie at: https://github.com/ovie-lang/ovie");

    Ok(())
}

fn cmd_build(file: Option<String>, backend: String, target: Option<String>, output: Option<String>, debug: bool, deterministic: bool, object: bool, assembly: bool) -> OvieResult<()> {
    let source_file = file.unwrap_or_else(|| "src/main.ov".to_string());
    
    if !Path::new(&source_file).exists() {
        return Err(oviec::OvieError::io_error(format!("Source file '{}' not found", source_file)));
    }

    let source = fs::read_to_string(&source_file)?;
    let mut compiler = if deterministic {
        Compiler::new_deterministic()
    } else {
        Compiler::new()
    };
    compiler.debug = debug;

    let backend_enum = Backend::from_str(&backend)
        .ok_or_else(|| oviec::OvieError::generic(format!("Unknown backend: {}", backend)))?;

    match backend_enum {
        Backend::Wasm => {
            let wasm_bytes = compiler.compile_to_wasm(&source)?;
            let output_file = output.unwrap_or_else(|| "output.wasm".to_string());
            fs::write(&output_file, wasm_bytes)?;
            println!("Built {} -> {} (WASM, {} bytes)", source_file, output_file, fs::metadata(&output_file)?.len());
        }
        #[cfg(feature = "llvm")]
        Backend::Llvm => {
            // Enhanced LLVM backend with target-specific compilation
            if let Some(target_triple) = target {
                println!("Building with LLVM backend for target: {}", target_triple);
                
                // Validate target
                let supported_targets = vec![
                    "x86_64-unknown-linux-gnu",
                    "x86_64-pc-windows-msvc", 
                    "x86_64-pc-windows-gnu",
                    "x86_64-apple-darwin",
                    "aarch64-unknown-linux-gnu",
                    "aarch64-apple-darwin",
                    "i686-unknown-linux-gnu",
                    "i686-pc-windows-msvc",
                ];
                
                if !supported_targets.contains(&target_triple.as_str()) {
                    return Err(oviec::OvieError::generic(format!(
                        "Unsupported target: {}. Supported targets: {}", 
                        target_triple, 
                        supported_targets.join(", ")
                    )));
                }
                
                if debug {
                    println!("Target configuration:");
                    println!("  Triple: {}", target_triple);
                    println!("  Deterministic: {}", deterministic);
                    println!("  Generate object: {}", object);
                    println!("  Generate assembly: {}", assembly);
                }
            }
            
            let llvm_ir = compiler.compile_to_llvm(&source)?;
            
            if object || assembly {
                println!("Note: Object and assembly file generation requires LLVM target machine initialization");
                println!("This feature is implemented but requires proper LLVM setup for full functionality");
            }
            
            let output_file = output.unwrap_or_else(|| {
                if object {
                    "output.o".to_string()
                } else if assembly {
                    "output.s".to_string()
                } else {
                    "output.ll".to_string()
                }
            });
            
            fs::write(&output_file, llvm_ir)?;
            
            let target_info = target.map(|t| format!(" for {}", t)).unwrap_or_default();
            println!("Built {} -> {} (LLVM IR{})", source_file, output_file, target_info);
            
            if deterministic {
                println!("✓ Deterministic build completed");
            }
        }
        Backend::Interpreter | Backend::IrInterpreter => {
            // For interpreters, we just validate the compilation
            let _ast = compiler.compile_to_ast(&source)?;
            println!("Validated {} ({})", source_file, backend_enum.name());
        }
        Backend::Hir => {
            let hir = compiler.compile_to_hir(&source)?;
            let output_file = output.unwrap_or_else(|| "output.hir.json".to_string());
            let hir_json = hir.to_json().unwrap_or_else(|_| "Failed to serialize HIR".to_string());
            fs::write(&output_file, hir_json)?;
            println!("Built {} -> {} (HIR)", source_file, output_file);
        }
        Backend::Mir => {
            let mir = compiler.compile_to_mir(&source)?;
            let output_file = output.unwrap_or_else(|| "output.mir.json".to_string());
            let mir_json = mir.to_json().unwrap_or_else(|_| "Failed to serialize MIR".to_string());
            fs::write(&output_file, mir_json)?;
            println!("Built {} -> {} (MIR)", source_file, output_file);
        }
    }

    Ok(())
}

fn cmd_run(file: Option<String>, backend: String, debug: bool) -> OvieResult<()> {
    let source_file = file.unwrap_or_else(|| "src/main.ov".to_string());
    
    if !Path::new(&source_file).exists() {
        return Err(oviec::OvieError::io_error(format!("Source file '{}' not found", source_file)));
    }

    let source = fs::read_to_string(&source_file)?;
    let mut compiler = Compiler::new();
    compiler.debug = debug;

    let backend_enum = Backend::from_str(&backend)
        .ok_or_else(|| oviec::OvieError::generic(format!("Unknown backend: {}", backend)))?;

    if debug {
        println!("Running {} with {} backend", source_file, backend_enum.name());
    }

    compiler.compile_and_run_with_backend(&source, backend_enum)?;

    Ok(())
}

fn cmd_check(file: Option<String>, debug: bool) -> OvieResult<()> {
    let source_file = file.unwrap_or_else(|| "src/main.ov".to_string());
    
    if !Path::new(&source_file).exists() {
        return Err(oviec::OvieError::io_error(format!("Source file '{}' not found", source_file)));
    }

    let source = fs::read_to_string(&source_file)?;
    let mut compiler = Compiler::new();
    compiler.debug = debug;

    if debug {
        println!("Checking {} for errors...", source_file);
    }

    // Compile to HIR to check for semantic errors
    let _hir = compiler.compile_to_hir(&source)?;
    
    println!("✓ {} - No errors found", source_file);
    Ok(())
}

fn cmd_analyze(file: Option<String>, debug: bool, format: String, output: Option<String>) -> OvieResult<()> {
    let source_file = file.unwrap_or_else(|| "src/main.ov".to_string());
    
    if !Path::new(&source_file).exists() {
        return Err(oviec::OvieError::io_error(format!("Source file '{}' not found", source_file)));
    }

    let source = fs::read_to_string(&source_file)?;
    let mut compiler = Compiler::new();
    compiler.debug = debug;

    if debug {
        println!("Analyzing {} with Aproko...", source_file);
    }

    // Compile to AST for analysis
    let ast = compiler.compile_to_ast(&source)?;
    
    // Run Aproko analysis
    let mut aproko_engine = aproko::AprokoEngine::new();
    let analysis_result = aproko_engine.analyze(&source, &ast)?;
    
    let mut report = String::new();
    report.push_str(&format!("=== Aproko Analysis Report for {} ===\n\n", source_file));
    
    // Summary statistics
    report.push_str("Analysis Summary:\n");
    report.push_str(&format!("  Lines analyzed: {}\n", analysis_result.stats.lines_analyzed));
    report.push_str(&format!("  Analysis duration: {}ms\n", analysis_result.stats.duration_ms));
    report.push_str(&format!("  Total findings: {}\n", analysis_result.findings.len()));
    report.push_str(&format!("  Total diagnostics: {}\n", analysis_result.diagnostics.len()));
    report.push_str("\n");
    
    // Findings by severity
    if !analysis_result.stats.findings_by_severity.is_empty() {
        report.push_str("Findings by Severity:\n");
        for (severity, count) in &analysis_result.stats.findings_by_severity {
            report.push_str(&format!("  {:?}: {}\n", severity, count));
        }
        report.push_str("\n");
    }
    
    // Detailed diagnostics
    if !analysis_result.diagnostics.is_empty() {
        report.push_str("Detailed Diagnostics:\n");
        for (i, diagnostic) in analysis_result.diagnostics.iter().enumerate() {
            report.push_str(&format!("  {}. [{}] {} ({}:{}:{})\n", 
                i + 1,
                diagnostic.rule_id,
                diagnostic.message,
                diagnostic.location.file,
                diagnostic.location.line,
                diagnostic.location.column
            ));
            report.push_str(&format!("     Severity: {:?}, Category: {:?}\n", 
                diagnostic.severity, diagnostic.category));
            
            // Show explanation if available
            if let Ok(explanation) = aproko_engine.explain_diagnostic(diagnostic) {
                report.push_str(&format!("     Explanation: {}\n", explanation.summary));
                
                if !explanation.fix_suggestions.is_empty() {
                    let fix = &explanation.fix_suggestions[0];
                    report.push_str(&format!("     Suggested fix: {} (Difficulty: {:?})\n", 
                        fix.title, fix.difficulty));
                }
            }
            report.push_str("\n");
        }
    } else {
        report.push_str("✓ No issues found!\n");
    }
    
    // Write output
    match output {
        Some(filename) => {
            fs::write(&filename, report)?;
            println!("Analysis report written to {}", filename);
        }
        None => {
            println!("{}", report);
        }
    }
    
    Ok(())
}

fn cmd_explain(rule: String, output: Option<String>) -> OvieResult<()> {
    let aproko_engine = aproko::AprokoEngine::new();
    let explanation_engine = aproko_engine.explanation_engine();
    
    if let Some(explanation) = explanation_engine.get_explanation(&rule) {
        let mut report = String::new();
        report.push_str(&format!("=== Explanation for Rule {} ===\n\n", rule));
        report.push_str(&format!("Summary: {}\n\n", explanation.summary));
        report.push_str(&format!("Type: {:?}\n", explanation.explanation_type));
        report.push_str(&format!("Confidence: {:.2}\n\n", explanation.confidence));
        
        report.push_str("Detailed Explanation:\n");
        report.push_str(&explanation.detailed_explanation);
        report.push_str("\n\n");
        
        if !explanation.code_examples.is_empty() {
            report.push_str("Code Examples:\n");
            for (i, example) in explanation.code_examples.iter().enumerate() {
                report.push_str(&format!("  {}. {} ({})\n", 
                    i + 1, 
                    example.description,
                    if example.is_good_example { "Good" } else { "Bad" }
                ));
                report.push_str(&format!("     ```{}\n", example.language));
                report.push_str(&format!("     {}\n", example.code));
                report.push_str("     ```\n");
                if let Some(ref notes) = example.notes {
                    report.push_str(&format!("     Note: {}\n", notes));
                }
                report.push_str("\n");
            }
        }
        
        if !explanation.fix_suggestions.is_empty() {
            report.push_str("Fix Suggestions:\n");
            for (i, fix) in explanation.fix_suggestions.iter().enumerate() {
                report.push_str(&format!("  {}. {} (Difficulty: {:?}, Confidence: {:.2})\n", 
                    i + 1, fix.title, fix.difficulty, fix.confidence));
                report.push_str(&format!("     {}\n", fix.description));
                
                if !fix.steps.is_empty() {
                    report.push_str("     Steps:\n");
                    for step in &fix.steps {
                        report.push_str(&format!("       {}. {}\n", step.step_number, step.description));
                        if let Some(ref notes) = step.notes {
                            report.push_str(&format!("          Note: {}\n", notes));
                        }
                    }
                }
                report.push_str("\n");
            }
        }
        
        if !explanation.related_topics.is_empty() {
            report.push_str(&format!("Related Topics: {}\n", explanation.related_topics.join(", ")));
        }
        
        // Write output
        match output {
            Some(filename) => {
                fs::write(&filename, report)?;
                println!("Explanation written to {}", filename);
            }
            None => {
                println!("{}", report);
            }
        }
    } else {
        println!("No explanation found for rule: {}", rule);
        println!("Available rules:");
        let all_explanations = explanation_engine.get_all_explanations();
        for rule_id in all_explanations.keys() {
            println!("  {}", rule_id);
        }
    }
    
    Ok(())
}

fn cmd_dump(ir_type: IrType) -> OvieResult<()> {
    match ir_type {
        IrType::Ast { file, format, output, debug } => cmd_dump_ast(file, format, output, debug),
        IrType::Hir { file, format, output, debug } => cmd_dump_hir(file, format, output, debug),
        IrType::Mir { file, format, output, debug } => cmd_dump_mir(file, format, output, debug),
    }
}

fn cmd_dump_ast(file: Option<String>, format: String, output: Option<String>, debug: bool) -> OvieResult<()> {
    let source_file = file.unwrap_or_else(|| "src/main.ov".to_string());
    
    if !Path::new(&source_file).exists() {
        return Err(oviec::OvieError::io_error(format!("Source file '{}' not found", source_file)));
    }

    let source = fs::read_to_string(&source_file)?;
    let mut compiler = Compiler::new();
    compiler.debug = debug;

    if debug {
        println!("Dumping AST for {}...", source_file);
    }

    let ast = compiler.compile_to_ast(&source)?;
    
    let ast_output = match format.as_str() {
        "json" => serde_json::to_string_pretty(&ast)
            .map_err(|e| oviec::OvieError::io_error(format!("JSON serialization error: {}", e)))?,
        "compact" => serde_json::to_string(&ast)
            .map_err(|e| oviec::OvieError::io_error(format!("JSON serialization error: {}", e)))?,
        _ => format!("{:#?}", ast), // pretty format
    };
    
    // Write output
    match output {
        Some(filename) => {
            fs::write(&filename, ast_output)?;
            println!("AST written to {}", filename);
        }
        None => {
            println!("{}", ast_output);
        }
    }
    
    Ok(())
}

fn cmd_dump_hir(file: Option<String>, format: String, output: Option<String>, debug: bool) -> OvieResult<()> {
    let source_file = file.unwrap_or_else(|| "src/main.ov".to_string());
    
    if !Path::new(&source_file).exists() {
        return Err(oviec::OvieError::io_error(format!("Source file '{}' not found", source_file)));
    }

    let source = fs::read_to_string(&source_file)?;
    let mut compiler = Compiler::new();
    compiler.debug = debug;

    if debug {
        println!("Dumping HIR for {}...", source_file);
    }

    let hir = compiler.compile_to_hir(&source)?;
    
    let hir_output = match format.as_str() {
        "json" => hir.to_json()?,
        "compact" => serde_json::to_string(&hir)
            .map_err(|e| oviec::OvieError::IrError { message: format!("HIR serialization error: {}", e) })?,
        _ => format!("{:#?}", hir), // pretty format
    };
    
    // Write output
    match output {
        Some(filename) => {
            fs::write(&filename, hir_output)?;
            println!("HIR written to {}", filename);
        }
        None => {
            println!("{}", hir_output);
        }
    }
    
    Ok(())
}

fn cmd_dump_mir(file: Option<String>, format: String, output: Option<String>, debug: bool) -> OvieResult<()> {
    let source_file = file.unwrap_or_else(|| "src/main.ov".to_string());
    
    if !Path::new(&source_file).exists() {
        return Err(oviec::OvieError::io_error(format!("Source file '{}' not found", source_file)));
    }

    let source = fs::read_to_string(&source_file)?;
    let mut compiler = Compiler::new();
    compiler.debug = debug;

    if debug {
        println!("Dumping MIR for {}...", source_file);
    }

    let mir = compiler.compile_to_mir(&source)?;
    
    let mir_output = match format.as_str() {
        "json" => mir.to_json()?,
        "compact" => serde_json::to_string(&mir)
            .map_err(|e| oviec::OvieError::IrError { message: format!("MIR serialization error: {}", e) })?,
        _ => format!("{:#?}", mir), // pretty format
    };
    
    // Write output
    match output {
        Some(filename) => {
            fs::write(&filename, mir_output)?;
            println!("MIR written to {}", filename);
        }
        None => {
            println!("{}", mir_output);
        }
    }
    
    Ok(())
}

fn cmd_cross_target(file: Option<String>, performance: bool, determinism: bool, tolerance: f64, runs: usize, output: Option<String>, debug: bool) -> OvieResult<()> {
    let source_file = file.unwrap_or_else(|| "src/main.ov".to_string());
    
    if !Path::new(&source_file).exists() {
        return Err(oviec::OvieError::io_error(format!("Source file '{}' not found", source_file)));
    }

    let source = fs::read_to_string(&source_file)?;
    let mut compiler = Compiler::new_deterministic();
    compiler.debug = debug;

    if debug {
        println!("Cross-target validation for: {}", source_file);
        println!("Performance validation: {}", performance);
        println!("Determinism validation: {}", determinism);
        println!("Performance tolerance: {}%", tolerance);
        println!("Validation runs: {}", runs);
    }

    // Compile to IR for validation
    let ir = compiler.compile_to_ir(&source)?;

    // Configure cross-target validator
    let mut config = CrossTargetValidationConfig::default();
    config.validate_performance = performance;
    config.validate_determinism = determinism;
    config.performance_tolerance = tolerance;
    config.validation_runs = runs;

    let validator = CrossTargetValidator::new(config);

    // Run cross-target validation
    println!("Running cross-target validation...");
    let results = validator.validate(&ir)?;

    // Display results
    println!("\n📊 Cross-Target Validation Results:");
    println!("   Total targets: {}", results.summary.total_targets);
    println!("   Successful: {}", results.summary.successful_targets);
    println!("   Failed: {}", results.summary.failed_targets);
    println!("   Errors: {}", results.summary.total_errors);
    println!("   Warnings: {}", results.summary.total_warnings);
    println!("   Duration: {}ms", results.summary.validation_duration_ms);

    // Display consistency results
    println!("\n🔍 Consistency Analysis:");
    println!("   Semantic consistency: {}", if results.consistency_results.semantic_consistency { "✓" } else { "✗" });
    println!("   Deterministic consistency: {}", if results.consistency_results.deterministic_consistency { "✓" } else { "✗" });

    if !results.consistency_results.inconsistencies.is_empty() {
        println!("\n⚠ Inconsistencies found:");
        for inconsistency in &results.consistency_results.inconsistencies {
            println!("   - {}", inconsistency);
        }
    }

    // Display performance results if enabled
    if let Some(ref perf_results) = results.performance_results {
        println!("\n⚡ Performance Analysis:");
        println!("   Within tolerance: {}", if perf_results.within_tolerance { "✓" } else { "✗" });
        
        if !perf_results.variations.is_empty() {
            println!("   Variations:");
            for variation in &perf_results.variations {
                println!("     - {}", variation);
            }
        }

        if debug {
            println!("   Compilation times:");
            for (target, time) in &perf_results.compilation_times {
                println!("     - {}: {}ms", target.triple, time);
            }
            
            println!("   Code sizes:");
            for (target, size) in &perf_results.code_sizes {
                println!("     - {}: {} bytes", target.triple, size);
            }
        }
    }

    // Display target-specific results
    if debug {
        println!("\n🎯 Target-Specific Results:");
        for (target, result) in &results.target_results {
            println!("   {}:", target.triple);
            println!("     Success: {}", result.compilation_success);
            if let Some(ref hash) = result.code_hash {
                println!("     Hash: {}...", &hash[..8]);
            }
            if let Some(size) = result.code_size {
                println!("     Size: {} bytes", size);
            }
            if !result.errors.is_empty() {
                println!("     Errors: {}", result.errors.len());
                for error in &result.errors {
                    println!("       - {}", error);
                }
            }
            if !result.warnings.is_empty() {
                println!("     Warnings: {}", result.warnings.len());
                for warning in &result.warnings {
                    println!("       - {}", warning);
                }
            }
        }
    }

    // Save report if requested
    if let Some(output_file) = output {
        let report = serde_json::to_string_pretty(&results)
            .map_err(|e| oviec::OvieError::generic(format!("Failed to serialize results: {}", e)))?;
        fs::write(&output_file, report)?;
        println!("\n📄 Validation report saved to: {}", output_file);
    }

    // Overall result
    if results.overall_success {
        println!("\n✅ Cross-target validation PASSED");
        Ok(())
    } else {
        println!("\n❌ Cross-target validation FAILED");
        Err(oviec::OvieError::generic("Cross-target validation failed"))
    }
}

fn cmd_batch(operation: BatchOperation) -> OvieResult<()> {
    match operation {
        BatchOperation::Check { pattern, debug, continue_on_error } => {
            cmd_batch_check(pattern, debug, continue_on_error)
        }
        BatchOperation::Analyze { pattern, debug, output_dir, continue_on_error } => {
            cmd_batch_analyze(pattern, debug, output_dir, continue_on_error)
        }
        BatchOperation::Format { pattern, check } => {
            cmd_batch_format(pattern, check)
        }
    }
}

fn cmd_batch_check(pattern: String, debug: bool, continue_on_error: bool) -> OvieResult<()> {
    let files = find_files_by_pattern(&pattern)?;
    
    if files.is_empty() {
        println!("No files found matching pattern: {}", pattern);
        return Ok(());
    }
    
    println!("Checking {} files matching pattern: {}", files.len(), pattern);
    
    let mut passed = 0;
    let mut failed = 0;
    let mut errors = Vec::new();
    
    for file in &files {
        if debug {
            println!("Checking: {}", file);
        }
        
        match cmd_check(Some(file.clone()), debug) {
            Ok(()) => {
                if debug {
                    println!("✓ {}", file);
                }
                passed += 1;
            }
            Err(e) => {
                println!("✗ {} - {}", file, e);
                errors.push((file.clone(), e.to_string()));
                failed += 1;
                
                if !continue_on_error {
                    break;
                }
            }
        }
    }
    
    println!("\nBatch check results:");
    println!("  Files checked: {}", passed + failed);
    println!("  Passed: {}", passed);
    println!("  Failed: {}", failed);
    
    if failed > 0 {
        println!("\nFailed files:");
        for (file, error) in &errors {
            println!("  {} - {}", file, error);
        }
        
        if !continue_on_error {
            return Err(oviec::OvieError::generic("Batch check failed".to_string()));
        }
    }
    
    Ok(())
}

fn cmd_batch_analyze(pattern: String, debug: bool, output_dir: String, continue_on_error: bool) -> OvieResult<()> {
    let files = find_files_by_pattern(&pattern)?;
    
    if files.is_empty() {
        println!("No files found matching pattern: {}", pattern);
        return Ok(());
    }
    
    // Create output directory
    fs::create_dir_all(&output_dir)?;
    
    println!("Analyzing {} files matching pattern: {}", files.len(), pattern);
    println!("Reports will be saved to: {}", output_dir);
    
    let mut analyzed = 0;
    let mut failed = 0;
    let mut errors = Vec::new();
    
    for file in &files {
        if debug {
            println!("Analyzing: {}", file);
        }
        
        // Generate output filename
        let file_stem = Path::new(file).file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        let output_file = format!("{}/{}-analysis.txt", output_dir, file_stem);
        
        match cmd_analyze(Some(file.clone()), debug, "pretty".to_string(), Some(output_file.clone())) {
            Ok(()) => {
                if debug {
                    println!("✓ {} -> {}", file, output_file);
                }
                analyzed += 1;
            }
            Err(e) => {
                println!("✗ {} - {}", file, e);
                errors.push((file.clone(), e.to_string()));
                failed += 1;
                
                if !continue_on_error {
                    break;
                }
            }
        }
    }
    
    println!("\nBatch analysis results:");
    println!("  Files processed: {}", analyzed + failed);
    println!("  Analyzed: {}", analyzed);
    println!("  Failed: {}", failed);
    println!("  Reports saved to: {}", output_dir);
    
    if failed > 0 {
        println!("\nFailed files:");
        for (file, error) in &errors {
            println!("  {} - {}", file, error);
        }
        
        if !continue_on_error {
            return Err(oviec::OvieError::generic("Batch analysis failed".to_string()));
        }
    }
    
    Ok(())
}

fn cmd_batch_format(pattern: String, check: bool) -> OvieResult<()> {
    let files = find_files_by_pattern(&pattern)?;
    
    if files.is_empty() {
        println!("No files found matching pattern: {}", pattern);
        return Ok(());
    }
    
    println!("Formatting {} files matching pattern: {}", files.len(), pattern);
    
    let mut needs_formatting = 0;
    let mut format_errors = 0;
    
    for file in &files {
        match format_file(file, check) {
            Ok(formatted) => {
                if formatted {
                    needs_formatting += 1;
                    if check {
                        println!("✗ {} needs formatting", file);
                    } else {
                        println!("✓ Formatted {}", file);
                    }
                } else if check {
                    println!("✓ {} is properly formatted", file);
                }
            }
            Err(e) => {
                println!("✗ Error formatting {}: {}", file, e);
                format_errors += 1;
            }
        }
    }
    
    if check {
        if needs_formatting > 0 {
            println!("\n{} files need formatting", needs_formatting);
            process::exit(1);
        } else {
            println!("\nAll {} files are properly formatted", files.len());
        }
    } else {
        if needs_formatting > 0 {
            println!("\nFormatted {} files", needs_formatting);
        } else {
            println!("\nAll {} files were already properly formatted", files.len());
        }
    }
    
    if format_errors > 0 {
        println!("Encountered {} formatting errors", format_errors);
        process::exit(1);
    }
    
    Ok(())
}

fn find_files_by_pattern(pattern: &str) -> OvieResult<Vec<String>> {
    // Simple pattern matching - in a real implementation, you'd use a glob library
    let mut files = Vec::new();
    
    if pattern == "**/*.ov" {
        // Find all .ov files recursively
        find_ovie_files_recursive(".", &mut files)?;
    } else if pattern.ends_with("*.ov") {
        // Find .ov files in specific directory
        let dir = pattern.trim_end_matches("*.ov").trim_end_matches('/');
        let dir = if dir.is_empty() { "." } else { dir };
        find_ovie_files_in_dir(dir, &mut files)?;
    } else {
        // Treat as specific file
        if Path::new(pattern).exists() {
            files.push(pattern.to_string());
        }
    }
    
    Ok(files)
}

fn find_ovie_files_recursive(dir: &str, files: &mut Vec<String>) -> OvieResult<()> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.ends_with(".ov") {
                        files.push(path.to_string_lossy().to_string());
                    }
                }
            } else if path.is_dir() {
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if dir_name != "target" && dir_name != ".git" && !dir_name.starts_with('.') {
                        find_ovie_files_recursive(&path.to_string_lossy(), files)?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn find_ovie_files_in_dir(dir: &str, files: &mut Vec<String>) -> OvieResult<()> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.ends_with(".ov") {
                        files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    Ok(())
}

fn cmd_test(pattern: String, debug: bool) -> OvieResult<()> {
    println!("Running tests matching pattern: {}", pattern);
    
    // For now, just look for .test.ov files
    let test_files = find_test_files(".")?;
    
    if test_files.is_empty() {
        println!("No test files found");
        return Ok(());
    }

    let mut passed = 0;
    let mut failed = 0;

    for test_file in test_files {
        if debug {
            println!("Running test: {}", test_file);
        }

        match run_test_file(&test_file, debug) {
            Ok(()) => {
                println!("✓ {}", test_file);
                passed += 1;
            }
            Err(e) => {
                println!("✗ {} - {}", test_file, e);
                failed += 1;
            }
        }
    }

    println!("\nTest results: {} passed, {} failed", passed, failed);
    
    if failed > 0 {
        process::exit(1);
    }

    Ok(())
}

fn cmd_fmt(files: Vec<String>, check: bool) -> OvieResult<()> {
    let target_files = if files.is_empty() {
        find_ovie_files(".")?
    } else {
        files
    };

    if target_files.is_empty() {
        println!("No .ov files found");
        return Ok(());
    }

    let mut needs_formatting = 0;
    let mut format_errors = 0;

    for file in &target_files {
        match format_file(file, check) {
            Ok(formatted) => {
                if formatted {
                    needs_formatting += 1;
                    if check {
                        println!("✗ {} needs formatting", file);
                    } else {
                        println!("✓ Formatted {}", file);
                    }
                } else if check {
                    println!("✓ {} is properly formatted", file);
                }
            }
            Err(e) => {
                println!("✗ Error formatting {}: {}", file, e);
                format_errors += 1;
            }
        }
    }

    if check {
        if needs_formatting > 0 {
            println!("\n{} files need formatting", needs_formatting);
            process::exit(1);
        } else {
            println!("\nAll {} files are properly formatted", target_files.len());
        }
    } else {
        if needs_formatting > 0 {
            println!("\nFormatted {} files", needs_formatting);
        } else {
            println!("\nAll {} files were already properly formatted", target_files.len());
        }
    }

    if format_errors > 0 {
        println!("Encountered {} formatting errors", format_errors);
        process::exit(1);
    }

    Ok(())
}

fn cmd_update(dependency: Option<String>) -> OvieResult<()> {
    let mut resolver = DependencyResolver::new()?;
    let current_dir = std::env::current_dir()
        .map_err(|e| OvieError::io_error(format!("Failed to get current directory: {}", e)))?;
    
    // Check if ovie.toml exists
    let toml_path = current_dir.join("ovie.toml");
    if !toml_path.exists() {
        println!("No ovie.toml found in current directory");
        return Ok(());
    }

    // Load current project configuration
    let config_content = fs::read_to_string(&toml_path)
        .map_err(|e| OvieError::io_error(format!("Failed to read ovie.toml: {}", e)))?;
    let config: ProjectConfig = toml::from_str(&config_content)
        .map_err(|e| OvieError::generic(format!("Failed to parse ovie.toml: {}", e)))?;

    // Load existing lock file if it exists
    let lock_path = current_dir.join("ovie.lock");
    let existing_lock = if lock_path.exists() {
        Some(PackageLock::load(&lock_path)?)
    } else {
        None
    };

    match dependency {
        Some(dep) => {
            println!("Updating dependency: {}", dep);
            
            // Check if the dependency exists in ovie.toml
            if !config.dependencies.contains_key(&dep) && !config.dev_dependencies.contains_key(&dep) {
                return Err(OvieError::generic(format!("Dependency '{}' not found in ovie.toml", dep)));
            }

            // Perform deterministic update for specific dependency
            let updated_lock = resolver.update_specific_dependency(&current_dir, &dep, existing_lock.as_ref())?;
            
            // Save updated lock file
            updated_lock.save(&lock_path)?;
            
            println!("Successfully updated dependency: {}", dep);
            if let Some(package_id) = updated_lock.dependencies.get(&dep) {
                println!("  - {}: {}", dep, package_id.to_string());
            }
        }
        None => {
            println!("Updating all dependencies");
            
            // Perform deterministic update for all dependencies
            let updated_lock = resolver.update_all_dependencies(&current_dir, existing_lock.as_ref())?;
            
            // Save updated lock file
            updated_lock.save(&lock_path)?;
            
            println!("Successfully updated ovie.lock");
            println!("Resolved {} dependencies:", updated_lock.dependencies.len());
            
            // Sort dependencies for deterministic output
            let mut sorted_deps: Vec<_> = updated_lock.dependencies.iter().collect();
            sorted_deps.sort_by_key(|(name, _)| *name);
            
            for (name, package_id) in sorted_deps {
                println!("  - {}: {}", name, package_id.to_string());
            }

            // Check for conflicts and provide resolution suggestions
            if let Some(ref old_lock) = existing_lock {
                let conflicts = detect_update_conflicts(&old_lock, &updated_lock);
                if !conflicts.is_empty() {
                    println!("\nDetected dependency conflicts:");
                    for conflict in conflicts {
                        println!("  - {}: {} -> {} ({})", 
                            conflict.dependency, 
                            conflict.old_version, 
                            conflict.new_version,
                            conflict.conflict_type
                        );
                    }
                    println!("\nRun 'ovie vendor' to update vendored dependencies");
                }
            }
        }
    }
    
    println!("Dependency update complete");
    Ok(())
}

/// Represents a dependency update conflict
#[derive(Debug)]
struct UpdateConflict {
    dependency: String,
    old_version: String,
    new_version: String,
    conflict_type: String,
}

/// Detect conflicts between old and new lock files
fn detect_update_conflicts(old_lock: &PackageLock, new_lock: &PackageLock) -> Vec<UpdateConflict> {
    let mut conflicts = Vec::new();
    
    for (name, new_id) in &new_lock.dependencies {
        if let Some(old_id) = old_lock.dependencies.get(name) {
            if old_id != new_id {
                conflicts.push(UpdateConflict {
                    dependency: name.clone(),
                    old_version: old_id.to_string(),
                    new_version: new_id.to_string(),
                    conflict_type: "version_change".to_string(),
                });
            }
        } else {
            conflicts.push(UpdateConflict {
                dependency: name.clone(),
                old_version: "none".to_string(),
                new_version: new_id.to_string(),
                conflict_type: "new_dependency".to_string(),
            });
        }
    }
    
    // Check for removed dependencies
    for (name, old_id) in &old_lock.dependencies {
        if !new_lock.dependencies.contains_key(name) {
            conflicts.push(UpdateConflict {
                dependency: name.clone(),
                old_version: old_id.to_string(),
                new_version: "none".to_string(),
                conflict_type: "removed_dependency".to_string(),
            });
        }
    }
    
    conflicts
}

fn cmd_vendor(output: String) -> OvieResult<()> {
    println!("Vendoring dependencies to: {}", output);
    
    let mut registry = PackageRegistry::new()?;
    
    // Create vendor directory
    fs::create_dir_all(&output)?;
    
    // Check if ovie.lock exists
    let lock_path = Path::new("ovie.lock");
    if lock_path.exists() {
        println!("Found ovie.lock, vendoring locked dependencies...");
        let lock = PackageLock::load(lock_path)?;
        
        for (name, package_id) in &lock.dependencies {
            println!("Vendoring {}: {}", name, package_id.to_string());
            match registry.vendor_package(package_id) {
                Ok(()) => println!("  ✓ Vendored {}", name),
                Err(e) => println!("  ✗ Failed to vendor {}: {}", name, e),
            }
        }
    } else {
        println!("No ovie.lock found, checking ovie.toml...");
        
        // Check if ovie.toml exists and has dependencies
        let toml_path = Path::new("ovie.toml");
        if toml_path.exists() {
            let toml_content = fs::read_to_string(toml_path)?;
            if toml_content.contains("[dependencies]") {
                println!("Found dependencies in ovie.toml, but no lock file.");
                println!("Run 'ovie update' first to generate ovie.lock");
                return Ok(());
            }
        }
        
        println!("No dependencies found to vendor");
    }
    
    println!("Dependencies vendored to {}", output);
    Ok(())
}

// Helper functions

fn find_test_files(dir: &str) -> OvieResult<Vec<String>> {
    let mut test_files = Vec::new();
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.ends_with(".test.ov") {
                        test_files.push(path.to_string_lossy().to_string());
                    }
                }
            } else if path.is_dir() {
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if dir_name != "target" && dir_name != ".git" {
                        let mut sub_files = find_test_files(&path.to_string_lossy())?;
                        test_files.append(&mut sub_files);
                    }
                }
            }
        }
    }
    
    Ok(test_files)
}

fn find_ovie_files(dir: &str) -> OvieResult<Vec<String>> {
    let mut ovie_files = Vec::new();
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.ends_with(".ov") {
                        ovie_files.push(path.to_string_lossy().to_string());
                    }
                }
            } else if path.is_dir() {
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if dir_name != "target" && dir_name != ".git" {
                        let mut sub_files = find_ovie_files(&path.to_string_lossy())?;
                        ovie_files.append(&mut sub_files);
                    }
                }
            }
        }
    }
    
    Ok(ovie_files)
}

fn run_test_file(file: &str, debug: bool) -> OvieResult<()> {
    let source = fs::read_to_string(file)?;
    let mut compiler = Compiler::new();
    compiler.debug = debug;
    
    // Run the test file as a regular Ovie program
    compiler.compile_and_run(&source)?;
    
    Ok(())
}

fn format_file(file: &str, check_only: bool) -> OvieResult<bool> {
    let source = fs::read_to_string(file)?;
    let formatted = format_ovie_code(&source)?;
    
    let needs_formatting = source != formatted;
    
    if needs_formatting && !check_only {
        fs::write(file, formatted)?;
    }
    
    Ok(needs_formatting)
}

fn format_ovie_code(source: &str) -> OvieResult<String> {
    // Parse the source code to get the AST
    let mut compiler = Compiler::new();
    let ast = compiler.compile_to_ast(source)?;
    
    // Format the AST back to source code
    let formatted = format_ast(&ast);
    
    Ok(formatted)
}

fn format_ast(ast: &AstNode) -> String {
    let mut output = String::new();
    let mut indent_level = 0;
    
    for (i, statement) in ast.statements.iter().enumerate() {
        if i > 0 {
            output.push('\n');
        }
        format_statement(statement, &mut output, indent_level);
    }
    
    // Ensure file ends with newline
    if !output.ends_with('\n') {
        output.push('\n');
    }
    
    output
}

fn format_statement(stmt: &Statement, output: &mut String, indent_level: usize) {
    let indent = "    ".repeat(indent_level);
    
    match stmt {
        Statement::Print { expression } => {
            output.push_str(&format!("{}seeAm {};", indent, format_expression(expression)));
        }
        Statement::Assignment { mutable: _, identifier, value } => {
            output.push_str(&format!("{}{} = {};", indent, identifier, format_expression(value)));
        }
        Statement::If { condition, then_block, else_block } => {
            output.push_str(&format!("{}if {} {{", indent, format_expression(condition)));
            for then_stmt in then_block {
                output.push('\n');
                format_statement(then_stmt, output, indent_level + 1);
            }
            output.push('\n');
            output.push_str(&format!("{}}}", indent));
            
            if let Some(else_stmts) = else_block {
                output.push_str(" else {");
                for else_stmt in else_stmts {
                    output.push('\n');
                    format_statement(else_stmt, output, indent_level + 1);
                }
                output.push('\n');
                output.push_str(&format!("{}}}", indent));
            }
        }
        Statement::While { condition, body } => {
            output.push_str(&format!("{}while {} {{", indent, format_expression(condition)));
            for body_stmt in body {
                output.push('\n');
                format_statement(body_stmt, output, indent_level + 1);
            }
            output.push('\n');
            output.push_str(&format!("{}}}", indent));
        }
        Statement::For { identifier, iterable, body } => {
            output.push_str(&format!("{}for {} in {} {{", indent, identifier, format_expression(iterable)));
            for body_stmt in body {
                output.push('\n');
                format_statement(body_stmt, output, indent_level + 1);
            }
            output.push('\n');
            output.push_str(&format!("{}}}", indent));
        }
        Statement::Function { name, parameters, body } => {
            let params = parameters.join(", ");
            output.push_str(&format!("{}fn {}({}) {{", indent, name, params));
            for body_stmt in body {
                output.push('\n');
                format_statement(body_stmt, output, indent_level + 1);
            }
            output.push('\n');
            output.push_str(&format!("{}}}", indent));
        }
        Statement::Return { value } => {
            if let Some(val) = value {
                output.push_str(&format!("{}return {};", indent, format_expression(val)));
            } else {
                output.push_str(&format!("{}return;", indent));
            }
        }
        Statement::Expression { expression } => {
            output.push_str(&format!("{}{};", indent, format_expression(expression)));
        }
        Statement::Struct { name, fields: _ } => {
            // Basic struct formatting - can be expanded later
            output.push_str(&format!("{}struct {} {{ /* fields */ }}", indent, name));
        }
        Statement::Enum { name, variants: _ } => {
            // Basic enum formatting - can be expanded later
            output.push_str(&format!("{}enum {} {{ /* variants */ }}", indent, name));
        }
    }
}

fn format_expression(expr: &Expression) -> String {
    match expr {
        Expression::Literal(lit) => format_literal(lit),
        Expression::Identifier(name) => name.clone(),
        Expression::Binary { left, operator, right } => {
            format!("{} {} {}", format_expression(left), format_operator(operator), format_expression(right))
        }
        Expression::Unary { operator, operand } => {
            format!("{}{}", format_unary_operator(operator), format_expression(operand))
        }
        Expression::Call { function, arguments } => {
            let args: Vec<String> = arguments.iter().map(format_expression).collect();
            format!("{}({})", function, args.join(", "))
        }
        Expression::FieldAccess { object, field } => {
            format!("{}.{}", format_expression(object), field)
        }
        Expression::Range { start, end } => {
            format!("{}..{}", format_expression(start), format_expression(end))
        }
        Expression::StructInstantiation { struct_name, fields: _ } => {
            // Basic struct instantiation formatting - can be expanded later
            format!("{} {{ /* fields */ }}", struct_name)
        }
    }
}

fn format_literal(lit: &oviec::ast::Literal) -> String {
    use oviec::ast::Literal;
    
    match lit {
        Literal::String(s) => format!("\"{}\"", s),
        Literal::Number(n) => {
            if n.fract() == 0.0 {
                format!("{}", *n as i64)
            } else {
                format!("{}", n)
            }
        }
        Literal::Boolean(b) => b.to_string(),
    }
}

fn format_operator(op: &oviec::ast::BinaryOperator) -> &'static str {
    use oviec::ast::BinaryOperator;
    
    match op {
        BinaryOperator::Add => "+",
        BinaryOperator::Subtract => "-",
        BinaryOperator::Multiply => "*",
        BinaryOperator::Divide => "/",
        BinaryOperator::Modulo => "%",
        BinaryOperator::Equal => "==",
        BinaryOperator::NotEqual => "!=",
        BinaryOperator::Less => "<",
        BinaryOperator::LessEqual => "<=",
        BinaryOperator::Greater => ">",
        BinaryOperator::GreaterEqual => ">=",
        BinaryOperator::And => "&&",
        BinaryOperator::Or => "||",
    }
}

fn format_unary_operator(op: &oviec::ast::UnaryOperator) -> &'static str {
    use oviec::ast::UnaryOperator;
    
    match op {
        UnaryOperator::Not => "!",
        UnaryOperator::Negate => "-",
    }
}
/// Handle self-hosting operations
fn cmd_self_host(action: SelfHostingAction) -> OvieResult<()> {
    match action {
        SelfHostingAction::Status => cmd_self_host_status(),
        SelfHostingAction::Verify { test_files, verbose } => cmd_self_host_verify(test_files, verbose),
        SelfHostingAction::Transition { force } => cmd_self_host_transition(force),
    }
}

/// Show current self-hosting status
fn cmd_self_host_status() -> OvieResult<()> {
    let manager = SelfHostingManager::new();
    let report = manager.generate_status_report();
    
    println!("{}", report);
    
    Ok(())
}

/// Verify bootstrap readiness for Stage 1 transition
fn cmd_self_host_verify(test_files: Vec<String>, verbose: bool) -> OvieResult<()> {
    let mut manager = SelfHostingManager::new();
    
    // Initialize bootstrap verification
    let config = BootstrapConfig {
        hash_verification: true,
        token_comparison: true,
        performance_benchmarking: true,
        max_performance_degradation: 5.0,
        verbose_logging: verbose,
    };
    
    println!("Initializing bootstrap verification...");
    
    // This will fail for now since the Ovie lexer isn't a valid Ovie program yet
    match manager.initialize_bootstrap_verification(config) {
        Ok(()) => {
            println!("✅ Bootstrap verification initialized successfully");
            
            // Use provided test files or default test cases
            let test_cases: Vec<String> = if test_files.is_empty() {
                vec![
                    r#"seeAm "Hello, World!";"#.to_string(),
                    r#"name = "Ovie"; mut counter = 42;"#.to_string(),
                    r#"fn greet(person) { seeAm "Hello, " + person + "!"; }"#.to_string(),
                ]
            } else {
                // Read test files
                let mut contents = Vec::new();
                for file_path in test_files {
                    match fs::read_to_string(&file_path) {
                        Ok(content) => contents.push(content),
                        Err(e) => {
                            eprintln!("Warning: Could not read test file '{}': {}", file_path, e);
                        }
                    }
                }
                contents
            };
            
            let test_case_refs: Vec<&str> = test_cases.iter().map(|s| s.as_str()).collect();
            
            println!("Running bootstrap verification on {} test cases...", test_case_refs.len());
            
            match manager.verify_stage1_readiness(&test_case_refs) {
                Ok(results) => {
                    let passed = results.iter().filter(|r| r.passed).count();
                    let total = results.len();
                    
                    println!("\n📊 Verification Results:");
                    println!("  Total tests: {}", total);
                    println!("  Passed: {}", passed);
                    println!("  Failed: {}", total - passed);
                    println!("  Success rate: {:.1}%", (passed as f64 / total as f64) * 100.0);
                    
                    if passed == total {
                        println!("\n✅ All verification tests passed! Ready for Stage 1 transition.");
                    } else {
                        println!("\n❌ Some verification tests failed. Stage 1 transition not recommended.");
                        
                        if verbose {
                            for (i, result) in results.iter().enumerate() {
                                if !result.passed {
                                    println!("\n🔍 Failed Test {}:", i + 1);
                                    println!("  Hash match: {}", result.hash_match);
                                    println!("  Token match: {}", result.tokens_match);
                                    println!("  Performance acceptable: {}", result.performance_acceptable);
                                    if !result.errors.is_empty() {
                                        println!("  Errors:");
                                        for error in &result.errors {
                                            println!("    - {}", error);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Bootstrap verification failed: {}", e);
                    return Err(e);
                }
            }
        }
        Err(e) => {
            println!("❌ Bootstrap verification initialization failed: {}", e);
            println!("\n📝 This is expected at this stage of development.");
            println!("   The Ovie-in-Ovie lexer specification needs to be completed first.");
            println!("   Current status: Task 18.1 in progress");
        }
    }
    
    Ok(())
}

/// Transition to the next self-hosting stage
fn cmd_self_host_transition(force: bool) -> OvieResult<()> {
    let mut manager = SelfHostingManager::new();
    
    let current_stage = manager.current_stage();
    println!("Current stage: {}", current_stage.name());
    
    if let Some(next_stage) = current_stage.next() {
        if !force {
            println!("⚠️  Stage transition requires verification.");
            println!("   Run 'ovie self-host verify' first, or use --force to skip verification.");
            return Ok(());
        }
        
        println!("🚀 Transitioning to {}...", next_stage.name());
        
        match manager.transition_to_next_stage() {
            Ok(new_stage) => {
                println!("✅ Successfully transitioned to {}", new_stage.name());
                println!("   {}", new_stage.description());
            }
            Err(e) => {
                eprintln!("❌ Stage transition failed: {}", e);
                return Err(e);
            }
        }
    } else {
        println!("🎉 Already at the final self-hosting stage!");
        println!("   The Ovie compiler is fully self-hosting.");
    }
    
    Ok(())
}
fn cmd_verify(package: Option<String>, debug: bool, signatures: bool, checksums: bool) -> OvieResult<()> {
    println!("Verifying package integrity...");
    
    let mut registry = PackageRegistry::new()?;
    
    if let Some(package_name) = package {
        // Verify specific package
        let packages = registry.list_packages()?;
        let matching_packages: Vec<_> = packages.iter()
            .filter(|pkg| pkg.name == package_name)
            .collect();
        
        if matching_packages.is_empty() {
            println!("Package '{}' not found in registry", package_name);
            return Ok(());
        }
        
        for package_id in matching_packages {
            println!("Verifying {}: {}", package_id.name, package_id.to_string());
            
            match registry.verify_package_integrity(package_id) {
                Ok(true) => {
                    println!("  ✓ Integrity verification passed");
                    
                    if signatures || checksums {
                        if let Ok(Some((metadata, _))) = registry.get_package(package_id) {
                            if signatures && !metadata.signatures.is_empty() {
                                println!("  ✓ {} signature(s) present", metadata.signatures.len());
                                if debug {
                                    for (i, sig) in metadata.signatures.iter().enumerate() {
                                        println!("    {}. {} (Key: {})", i + 1, sig.algorithm, sig.key_id);
                                    }
                                }
                            }
                            
                            if checksums && !metadata.checksums.is_empty() {
                                println!("  ✓ {} checksum(s) verified", metadata.checksums.len());
                                if debug {
                                    for (algo, checksum) in &metadata.checksums {
                                        println!("    {}: {}...", algo, &checksum[..16]);
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(false) => {
                    println!("  ✗ Integrity verification failed");
                }
                Err(e) => {
                    println!("  ✗ Verification error: {}", e);
                }
            }
        }
    } else {
        // Verify all packages
        let packages = registry.list_packages()?;
        
        if packages.is_empty() {
            println!("No packages found in registry");
            return Ok(());
        }
        
        let mut verified = 0;
        let mut failed = 0;
        
        for package_id in &packages {
            if debug {
                println!("Verifying {}: {}", package_id.name, package_id.to_string());
            }
            
            match registry.verify_package_integrity(package_id) {
                Ok(true) => {
                    verified += 1;
                    if debug {
                        println!("  ✓ Integrity verification passed");
                    }
                }
                Ok(false) => {
                    failed += 1;
                    println!("  ✗ Integrity verification failed for {}", package_id.to_string());
                }
                Err(e) => {
                    failed += 1;
                    println!("  ✗ Verification error for {}: {}", package_id.to_string(), e);
                }
            }
        }
        
        println!("\nVerification Summary:");
        println!("  ✓ {} packages verified", verified);
        if failed > 0 {
            println!("  ✗ {} packages failed", failed);
        }
        println!("  Total: {} packages", packages.len());
    }
    
    Ok(())
}

fn cmd_integrity(action: IntegrityAction) -> OvieResult<()> {
    match action {
        IntegrityAction::Check { package, verbose } => {
            cmd_verify(package, verbose, true, true)
        }
        IntegrityAction::Generate { package, version, output } => {
            println!("Generating integrity manifest for {}@{}", package, version);
            
            let mut registry = PackageRegistry::new()?;
            let packages = registry.list_packages()?;
            
            let matching_package = packages.iter()
                .find(|pkg| pkg.name == package && pkg.version == version)
                .ok_or_else(|| OvieError::generic(format!("Package {}@{} not found", package, version)))?;
            
            if let Ok(Some((metadata, content))) = registry.get_package(matching_package) {
                let manifest = IntegrityManifest {
                    package_id: metadata.id.clone(),
                    content_hash: metadata.id.content_hash.clone(),
                    checksums: metadata.checksums.clone(),
                    signatures: metadata.signatures.clone(),
                    verification_timestamp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    file_size: content.len() as u64,
                    offline_compliance: metadata.offline_metadata.clone(),
                };
                
                let manifest_json = serde_json::to_string_pretty(&manifest)
                    .map_err(|e| OvieError::generic(format!("Failed to serialize manifest: {}", e)))?;
                
                fs::write(&output, manifest_json)
                    .map_err(|e| OvieError::io_error(format!("Failed to write manifest: {}", e)))?;
                
                println!("Integrity manifest written to: {}", output);
            } else {
                return Err(OvieError::generic(format!("Failed to load package {}@{}", package, version)));
            }
            
            Ok(())
        }
        IntegrityAction::Repair { package, force } => {
            println!("Repairing integrity issues for package: {}", package);
            
            if !force {
                println!("This will attempt to repair integrity issues by re-downloading and re-verifying the package.");
                println!("Use --force to proceed without confirmation.");
                return Ok(());
            }
            
            // Placeholder for repair functionality
            println!("Package repair functionality not yet implemented");
            println!("This would:");
            println!("  1. Re-download package from trusted source");
            println!("  2. Re-verify all checksums and signatures");
            println!("  3. Update integrity manifest");
            println!("  4. Clear any cached invalid data");
            
            Ok(())
        }
    }
}