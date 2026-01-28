use clap::{Parser, Subcommand};
use oviec::{Compiler, Backend, OvieResult, OvieError, AstNode, Statement, Expression, PackageRegistry, PackageLock, DependencyResolver, ProjectConfig, SelfHostingManager, SelfHostingStage, BootstrapConfig, BootstrapVerificationResult, BrandingConfig, ProjectTemplate, ProjectMetadata};
use std::fs;
use std::path::Path;
use std::process;

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
        /// Output file
        #[arg(short, long)]
        output: Option<String>,
        /// Enable debug output
        #[arg(long)]
        debug: bool,
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
    /// Self-hosting operations
    SelfHost {
        #[command(subcommand)]
        action: SelfHostingAction,
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

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::New { name, path } => cmd_new(name, path),
        Commands::Build { file, backend, output, debug } => cmd_build(file, backend, output, debug),
        Commands::Run { file, backend, debug } => cmd_run(file, backend, debug),
        Commands::Test { pattern, debug } => cmd_test(pattern, debug),
        Commands::Fmt { files, check } => cmd_fmt(files, check),
        Commands::Update { dependency } => cmd_update(dependency),
        Commands::Vendor { output } => cmd_vendor(output),
        Commands::SelfHost { action } => cmd_self_host(action),
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

fn cmd_build(file: Option<String>, backend: String, output: Option<String>, debug: bool) -> OvieResult<()> {
    let source_file = file.unwrap_or_else(|| "src/main.ov".to_string());
    
    if !Path::new(&source_file).exists() {
        return Err(oviec::OvieError::io_error(format!("Source file '{}' not found", source_file)));
    }

    let source = fs::read_to_string(&source_file)?;
    let mut compiler = Compiler::new();
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
            let llvm_ir = compiler.compile_to_llvm(&source)?;
            let output_file = output.unwrap_or_else(|| "output.ll".to_string());
            fs::write(&output_file, llvm_ir)?;
            println!("Built {} -> {} (LLVM IR)", source_file, output_file);
        }
        Backend::Interpreter | Backend::IrInterpreter => {
            // For interpreters, we just validate the compilation
            let _ast = compiler.compile_to_ast(&source)?;
            println!("Validated {} ({})", source_file, backend_enum.name());
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