//! Ovie Compiler Library
//! 
//! This is the core library for the Ovie programming language compiler.
//! It provides the complete compilation pipeline from source code to executable output.

pub mod lexer;
pub mod parser;
pub mod ast;
pub mod hir;
pub mod mir;
pub mod error;
pub mod normalizer;
pub mod ir;
pub mod interpreter;
pub mod semantic;
pub mod codegen;
pub mod package;
pub mod security;
pub mod self_hosting;
pub mod branding;
pub mod release;
pub mod cross_target_validation;
pub mod hardware;
pub mod hardware_impl;
pub mod hardware_safety;
pub mod runtime_environment;
pub mod stdlib;

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};

/// Deterministic build configuration for reproducible builds
#[derive(Debug, Clone)]
pub struct DeterministicBuildConfig {
    /// Fixed timestamp for reproducible builds (Unix timestamp)
    pub fixed_timestamp: Option<u64>,
    /// Source file hash for build verification
    pub source_hash: Option<String>,
    /// Build environment variables to include in hash
    pub env_vars: HashMap<String, String>,
    /// Enable deterministic output ordering
    pub deterministic_output: bool,
    /// Build metadata for verification
    pub build_metadata: BuildMetadata,
}

/// Build metadata for verification and reproducibility
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BuildMetadata {
    /// Compiler version
    pub compiler_version: String,
    /// Target platform
    pub target_platform: String,
    /// Build timestamp (fixed for reproducible builds)
    pub build_timestamp: u64,
    /// Source file hash
    pub source_hash: String,
    /// Dependency hashes
    pub dependency_hashes: HashMap<String, String>,
    /// Build flags and configuration
    pub build_flags: Vec<String>,
}

impl DeterministicBuildConfig {
    /// Create a new build configuration with deterministic defaults
    pub fn new_deterministic() -> Self {
        Self {
            fixed_timestamp: Some(1640995200), // Fixed timestamp: 2022-01-01 00:00:00 UTC
            source_hash: None,
            env_vars: HashMap::new(),
            deterministic_output: true,
            build_metadata: BuildMetadata::new(),
        }
    }

    /// Create a new build configuration with current timestamp
    pub fn new() -> Self {
        let current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            fixed_timestamp: None,
            source_hash: None,
            env_vars: HashMap::new(),
            deterministic_output: false,
            build_metadata: BuildMetadata::new_with_timestamp(current_timestamp),
        }
    }

    /// Set source code and compute hash
    pub fn with_source(&mut self, source: &str) -> &mut Self {
        let mut hasher = Sha256::new();
        hasher.update(source.as_bytes());
        self.source_hash = Some(format!("{:x}", hasher.finalize()));
        self.build_metadata.source_hash = self.source_hash.clone().unwrap_or_default();
        self
    }

    /// Add environment variable to build context
    pub fn with_env_var(&mut self, key: String, value: String) -> &mut Self {
        self.env_vars.insert(key, value);
        self
    }

    /// Get build timestamp (fixed or current)
    pub fn get_timestamp(&self) -> u64 {
        self.fixed_timestamp.unwrap_or_else(|| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        })
    }

    /// Compute build hash for verification
    pub fn compute_build_hash(&self) -> String {
        let mut hasher = Sha256::new();
        
        // Include source hash
        if let Some(ref source_hash) = self.source_hash {
            hasher.update(source_hash.as_bytes());
        }
        
        // Include timestamp
        hasher.update(self.get_timestamp().to_string().as_bytes());
        
        // Include environment variables (sorted for determinism)
        let mut env_keys: Vec<_> = self.env_vars.keys().collect();
        env_keys.sort();
        for key in env_keys {
            hasher.update(key.as_bytes());
            hasher.update(self.env_vars[key].as_bytes());
        }
        
        // Include build metadata
        hasher.update(self.build_metadata.compiler_version.as_bytes());
        hasher.update(self.build_metadata.target_platform.as_bytes());
        
        format!("{:x}", hasher.finalize())
    }
}

impl BuildMetadata {
    /// Create new build metadata with current defaults
    pub fn new() -> Self {
        Self {
            compiler_version: env!("CARGO_PKG_VERSION").to_string(),
            target_platform: std::env::consts::ARCH.to_string(),
            build_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            source_hash: String::new(),
            dependency_hashes: HashMap::new(),
            build_flags: Vec::new(),
        }
    }

    /// Create new build metadata with specific timestamp
    pub fn new_with_timestamp(timestamp: u64) -> Self {
        Self {
            compiler_version: env!("CARGO_PKG_VERSION").to_string(),
            target_platform: std::env::consts::ARCH.to_string(),
            build_timestamp: timestamp,
            source_hash: String::new(),
            dependency_hashes: HashMap::new(),
            build_flags: Vec::new(),
        }
    }

    /// Add dependency hash
    pub fn add_dependency_hash(&mut self, name: String, hash: String) {
        self.dependency_hashes.insert(name, hash);
    }

    /// Add build flag
    pub fn add_build_flag(&mut self, flag: String) {
        self.build_flags.push(flag);
    }
}
pub use error::{OvieError, OvieResult, Diagnostic, ErrorReporter, ErrorSeverity, ErrorCategory, ErrorSuggestion, CodeFix, TextReplacement, SourcePosition, SourceLocation};
// pub use self::{BuildConfig, BuildMetadata}; // Remove duplicate export
pub use lexer::{Lexer, Token, TokenType};
pub use parser::{Parser, ParseResult};
pub use ast::{AstNode, Statement, Expression, AstInvariantValidation};
pub use hir::{HirProgram, HirBuilder, HirItem, HirFunction, HirStatement, HirExpression, HirType, HirInvariantValidation};
pub use mir::{MirProgram, MirBuilder, MirFunction, MirBasicBlock, MirStatement, MirTerminator, MirType, MirInvariantValidation};
pub use interpreter::{Interpreter, IrInterpreter};
// pub use semantic::{SemanticAnalyzer, TypedAst, Type};
pub use ir::{IrBuilder, Program as IR, Instruction, Value, BackendInvariantValidation};
pub use normalizer::Normalizer;
pub use codegen::CodegenBackend;
pub use codegen::WasmBackend;
pub use package::{PackageRegistry, PackageId, PackageMetadata, PackageLock, DependencyResolver, ProjectConfig, DependencySpec, IntegrityManifest, PackageSignature, OfflineMetadata};
pub use security::{NetworkMonitor, CryptographicVerifier, SupplyChainSecurity, SecurityPolicies, SecurityReport, UnsafeOperationAnalyzer, UnsafeOperation, UnsafeAuditEntry, TelemetryMonitor, TelemetryAttempt, PrivacySettings, PrivacyComplianceReport, NetworkSecurityReport, ComprehensiveSecurityReport};
pub use self_hosting::{SelfHostingManager, SelfHostingStage, BootstrapVerifier, BootstrapConfig, BootstrapVerificationResult, BootstrapIntegration, IntegrationMode, IntegrationVerificationResult};
pub use branding::{BrandingConfig, ProjectTemplate, ProjectMetadata};
pub use release::{ReleaseManager, SecurityLevel, ReleaseMetadata, DistributionConfig, DistributionManager, ReleasePackage, SignatureResult, VerificationResult};
pub use cross_target_validation::{CrossTargetValidator, CrossTargetValidationConfig, CrossTargetValidationResults, TargetPlatform, TargetValidationResult, PlatformGuarantee, GuaranteeType, ConsistencyResults, PerformanceResults, ValidationSummary};

// Export the main Compiler interface
// pub use self::{DeterministicBuildConfig, BuildMetadata};
// Hardware abstraction layer - temporarily disabled for compilation
// use hardware::{PlatformAbstractionLayer, DeviceModel, DeviceType, DeviceState, StateValue, DeviceOperation, SafetyConstraint, DeviceInvariant, PlatformConfiguration, SafetyLevel, HardwareSafetyAnalyzer, DeterminismEnforcer, DeviceFactory, HardwareBehaviorAnalyzer, AutomatedHardwareAnalyzer, HardwareConfiguration, BehaviorPattern, AnalysisResult};
pub use runtime_environment::{OvieRuntimeEnvironment, OreError, HealthReport, HealthStatus, ComponentHealth};
#[cfg(feature = "llvm")]
pub use codegen::LlvmBackend;

#[cfg(test)]
mod tests {
    pub mod property_tests;
    pub mod grammar_validation_tests;
    pub mod hir_tests;
    pub mod mir_tests;
}

// Comprehensive test framework (always available for test runner binary)
pub mod tests {
    include!("../tests/mod.rs");
}

/// Backend selection for code generation
#[derive(Debug, Clone, PartialEq)]
pub enum Backend {
    /// WebAssembly backend
    Wasm,
    /// LLVM backend (requires llvm feature)
    #[cfg(feature = "llvm")]
    Llvm,
    /// Interpreter (AST-based)
    Interpreter,
    /// IR Interpreter
    IrInterpreter,
    /// HIR (High-level IR) output
    Hir,
    /// MIR (Mid-level IR) output
    Mir,
}

impl Backend {
    /// Get backend from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "wasm" | "webassembly" => Some(Backend::Wasm),
            #[cfg(feature = "llvm")]
            "llvm" => Some(Backend::Llvm),
            "interpreter" | "ast" => Some(Backend::Interpreter),
            "ir" | "ir-interpreter" => Some(Backend::IrInterpreter),
            "hir" => Some(Backend::Hir),
            "mir" => Some(Backend::Mir),
            _ => None,
        }
    }

    /// Get backend name
    pub fn name(&self) -> &'static str {
        match self {
            Backend::Wasm => "wasm",
            #[cfg(feature = "llvm")]
            Backend::Llvm => "llvm",
            Backend::Interpreter => "interpreter",
            Backend::IrInterpreter => "ir-interpreter",
            Backend::Hir => "hir",
            Backend::Mir => "mir",
        }
    }
}

/// The main compiler interface
pub struct Compiler {
    /// Enable debug output
    pub debug: bool,
    /// Default backend for compilation
    pub default_backend: Backend,
    /// Build configuration for deterministic builds
    pub build_config: DeterministicBuildConfig,
    /// Supply chain security manager
    pub security_manager: SupplyChainSecurity,
    /// Enable strict invariant checking (panic on violation)
    pub strict_invariants: bool,
}

impl Compiler {
    /// Create a new compiler instance
    pub fn new() -> Self {
        Self {
            debug: false,
            default_backend: Backend::Interpreter,
            build_config: DeterministicBuildConfig::new(),
            security_manager: SupplyChainSecurity::new(),
            strict_invariants: false,
        }
    }

    /// Create a new compiler instance with debug enabled
    pub fn new_with_debug() -> Self {
        Self {
            debug: true,
            default_backend: Backend::Interpreter,
            build_config: DeterministicBuildConfig::new(),
            security_manager: SupplyChainSecurity::new(),
            strict_invariants: false,
        }
    }

    /// Create a new compiler instance with specific backend
    pub fn new_with_backend(backend: Backend) -> Self {
        Self {
            debug: false,
            default_backend: backend,
            build_config: DeterministicBuildConfig::new(),
            security_manager: SupplyChainSecurity::new(),
            strict_invariants: false,
        }
    }

    /// Create a new compiler instance with deterministic build configuration
    pub fn new_deterministic() -> Self {
        Self {
            debug: false,
            default_backend: Backend::Interpreter,
            build_config: DeterministicBuildConfig::new_deterministic(),
            security_manager: SupplyChainSecurity::new(),
            strict_invariants: false,
        }
    }

    /// Create a new compiler instance with strict invariant checking enabled
    pub fn new_with_strict_invariants() -> Self {
        Self {
            debug: false,
            default_backend: Backend::Interpreter,
            build_config: DeterministicBuildConfig::new(),
            security_manager: SupplyChainSecurity::new(),
            strict_invariants: true,
        }
    }

    /// Enable or disable strict invariant checking
    pub fn set_strict_invariants(&mut self, enabled: bool) {
        self.strict_invariants = enabled;
    }

    /// Set build configuration
    pub fn with_build_config(mut self, config: DeterministicBuildConfig) -> Self {
        self.build_config = config;
        self
    }

    /// Set security manager
    pub fn with_security_manager(mut self, security_manager: SupplyChainSecurity) -> Self {
        self.security_manager = security_manager;
        self
    }

    /// Get security manager
    pub fn security_manager(&self) -> &SupplyChainSecurity {
        &self.security_manager
    }

    /// Get mutable security manager
    pub fn security_manager_mut(&mut self) -> &mut SupplyChainSecurity {
        &mut self.security_manager
    }

    /// Compile Ovie source code to an AST
    pub fn compile_to_ast(&mut self, source: &str) -> OvieResult<AstNode> {
        // Update build config with source hash
        self.build_config.with_source(source);
        
        if self.debug {
            println!("Build hash: {}", self.build_config.compute_build_hash());
            println!("Build timestamp: {}", self.build_config.get_timestamp());
        }

        // Step 0: Source-level normalization (fix typos before lexing)
        let mut normalizer = Normalizer::new();
        let (normalized_source, source_corrections) = normalizer.normalize_source(source);
        
        if self.debug && !source_corrections.is_empty() {
            println!("Source corrections:");
            for correction in &source_corrections {
                println!("  {}", correction.reason);
            }
        }

        // Step 1: Lexical analysis
        let mut lexer = Lexer::new(&normalized_source);
        let tokens = lexer.tokenize()?;

        if self.debug {
            println!("Tokens: {:?}", tokens);
        }

        // Step 2: Parsing
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        if self.debug {
            println!("AST: {:?}", ast);
        }

        // Step 3: AST-level normalization (structural corrections)
        let (normalized_ast, ast_corrections) = normalizer.normalize(ast)?;
        
        if self.debug && !ast_corrections.is_empty() {
            println!("AST corrections:");
            for correction in &ast_corrections {
                println!("  {}", correction.reason);
            }
        }

        // Step 4: Unsafe operation analysis
        let unsafe_analyzer = crate::security::UnsafeOperationAnalyzer::new();
        let unsafe_operations = unsafe_analyzer.analyze_ast(&normalized_ast, "source.ov")?;
        
        if self.debug && !unsafe_operations.is_empty() {
            println!("Unsafe operations detected:");
            for op in &unsafe_operations {
                println!("  {:?}: {}", op.operation, op.description);
            }
            
            let security_report = unsafe_analyzer.generate_security_report();
            println!("Security report: {} total unsafe operations", security_report.total_unsafe_operations);
            for recommendation in &security_report.recommendations {
                println!("  Recommendation: {}", recommendation);
            }
        }

        // Step 5: Privacy and telemetry monitoring
        // Monitor any potential telemetry attempts during compilation
        let telemetry_blocked = self.security_manager.monitor_network_call(
            "compiler://internal", 
            "compilation_metrics", 
            "ovie_compiler"
        )?;
        
        if self.debug {
            println!("Telemetry monitoring: {}", if telemetry_blocked { "No telemetry sent" } else { "Telemetry allowed" });
            
            let privacy_report = self.security_manager.telemetry_monitor().generate_privacy_report();
            println!("Privacy compliance: {}", privacy_report.compliance_status);
        }

        // Step 6: Semantic analysis
        // TODO: Implement semantic analyzer

        // Step 7: AST invariant validation
        if let Err(e) = normalized_ast.validate() {
            let error = OvieError::InvariantViolation {
                stage: "AST".to_string(),
                message: format!("AST invariant violation: {} (Source: {}, Build: {})", 
                    e, 
                    self.build_config.source_hash.as_deref().unwrap_or("unknown"),
                    self.build_config.compute_build_hash()
                ),
            };
            
            if self.strict_invariants {
                panic!("AST invariant violation: {} (Source: {}, Build: {})", 
                    e,
                    self.build_config.source_hash.as_deref().unwrap_or("unknown"),
                    self.build_config.compute_build_hash()
                );
            } else {
                return Err(error);
            }
        }

        if self.debug {
            println!("AST invariants validated successfully");
        }

        Ok(normalized_ast)
    }

    /// Compile Ovie source code to HIR (High-level IR)
    pub fn compile_to_hir(&mut self, source: &str) -> OvieResult<HirProgram> {
        let ast = self.compile_to_ast(source)?;
        
        // Step 6: HIR generation (semantic analysis and type checking)
        let mut hir_builder = HirBuilder::new();
        let hir = hir_builder.transform_ast(&ast)?;
        
        // Step 7: HIR invariant validation
        if let Err(e) = hir.validate() {
            let error = OvieError::InvariantViolation {
                stage: "HIR".to_string(),
                message: format!("HIR invariant violation: {} (Source: {}, Build: {})", 
                    e,
                    self.build_config.source_hash.as_deref().unwrap_or("unknown"),
                    self.build_config.compute_build_hash()
                ),
            };
            
            if self.strict_invariants {
                panic!("HIR invariant violation: {} (Source: {}, Build: {})", 
                    e,
                    self.build_config.source_hash.as_deref().unwrap_or("unknown"),
                    self.build_config.compute_build_hash()
                );
            } else {
                return Err(error);
            }
        }
        
        if self.debug {
            println!("HIR: {}", hir.to_json().unwrap_or_else(|_| "Failed to serialize HIR".to_string()));
            println!("HIR invariants validated successfully");
        }
        
        Ok(hir)
    }

    /// Compile Ovie source code to MIR (Mid-level IR)
    pub fn compile_to_mir(&mut self, source: &str) -> OvieResult<MirProgram> {
        let hir = self.compile_to_hir(source)?;
        
        // Step 7: MIR generation (control flow explicit)
        let mut mir_builder = MirBuilder::new();
        let mir = mir_builder.transform_hir(&hir)?;
        
        // Step 8: MIR invariant validation
        if let Err(e) = mir.validate() {
            let error = OvieError::InvariantViolation {
                stage: "MIR".to_string(),
                message: format!("MIR invariant violation: {} (Source: {}, Build: {})", 
                    e,
                    self.build_config.source_hash.as_deref().unwrap_or("unknown"),
                    self.build_config.compute_build_hash()
                ),
            };
            
            if self.strict_invariants {
                panic!("MIR invariant violation: {} (Source: {}, Build: {})", 
                    e,
                    self.build_config.source_hash.as_deref().unwrap_or("unknown"),
                    self.build_config.compute_build_hash()
                );
            } else {
                return Err(error);
            }
        }
        
        if self.debug {
            println!("MIR: {}", mir.to_json().unwrap_or_else(|_| "Failed to serialize MIR".to_string()));
            println!("MIR invariants validated successfully");
        }
        
        Ok(mir)
    }

    /// Compile Ovie source code to IR (legacy - now uses MIR)
    pub fn compile_to_ir(&mut self, source: &str) -> OvieResult<IR> {
        let mir = self.compile_to_mir(source)?;
        
        // Convert MIR to legacy IR format for backward compatibility
        let mut ir_builder = IrBuilder::new();
        if self.build_config.deterministic_output {
            ir_builder.set_deterministic_mode(true);
        }
        
        // For now, create a simple IR from MIR
        // In a full implementation, this would be a proper MIR to IR conversion
        let ir = ir_builder.build();
        
        // Step 9: Backend invariant validation
        if let Err(e) = ir.validate_backend_invariants() {
            let error = OvieError::InvariantViolation {
                stage: "Backend".to_string(),
                message: format!("Backend invariant violation: {} (Source: {}, Build: {})", 
                    e,
                    self.build_config.source_hash.as_deref().unwrap_or("unknown"),
                    self.build_config.compute_build_hash()
                ),
            };
            
            if self.strict_invariants {
                panic!("Backend invariant violation: {} (Source: {}, Build: {})", 
                    e,
                    self.build_config.source_hash.as_deref().unwrap_or("unknown"),
                    self.build_config.compute_build_hash()
                );
            } else {
                return Err(error);
            }
        }
        
        if self.debug {
            println!("Legacy IR: {}", ir.to_json().unwrap_or_else(|_| "Failed to serialize IR".to_string()));
            println!("Backend invariants validated successfully");
        }
        
        Ok(ir)
    }

    /// Compile and run using IR interpreter
    pub fn compile_and_run_ir(&mut self, source: &str) -> OvieResult<()> {
        let ir = self.compile_to_ir(source)?;
        
        let mut ir_interpreter = crate::interpreter::IrInterpreter::new();
        ir_interpreter.execute(&ir)?;
        
        Ok(())
    }

    /// Compile and interpret Ovie source code using AST interpreter
    pub fn compile_and_run(&mut self, source: &str) -> OvieResult<()> {
        let ast = self.compile_to_ast(source)?;
        
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&ast)?;
        
        Ok(())
    }

    /// Compile Ovie source code to WebAssembly
    pub fn compile_to_wasm(&mut self, source: &str) -> OvieResult<Vec<u8>> {
        let mir = self.compile_to_mir(source)?;
        
        // Convert MIR to legacy IR for WASM backend (temporary)
        let ir = self.compile_to_ir(source)?;
        
        let mut wasm_backend = crate::codegen::WasmBackend::new();
        if self.build_config.deterministic_output {
            wasm_backend.set_deterministic_mode(true);
        }
        let wasm_bytes = wasm_backend.generate(&ir)?;
        
        if self.debug {
            println!("Generated WASM module: {} bytes", wasm_bytes.len());
        }
        
        Ok(wasm_bytes)
    }

    /// Compile Ovie source code to LLVM IR
    #[cfg(feature = "llvm")]
    pub fn compile_to_llvm(&mut self, source: &str) -> OvieResult<String> {
        let mir = self.compile_to_mir(source)?;
        
        // Convert MIR to legacy IR for LLVM backend (temporary)
        let ir = self.compile_to_ir(source)?;
        
        let context = inkwell::context::Context::create();
        let mut llvm_backend = crate::codegen::LlvmBackend::new(&context, "ovie_module");
        if self.build_config.deterministic_output {
            llvm_backend.set_deterministic_mode(true);
        }
        let llvm_ir = llvm_backend.generate(&ir)?;
        
        if self.debug {
            println!("Generated LLVM IR: {} lines", llvm_ir.lines().count());
        }
        
        Ok(llvm_ir)
    }

    /// Compile and run using the default backend
    pub fn compile_and_run_with_backend(&mut self, source: &str, backend: Backend) -> OvieResult<()> {
        match backend {
            Backend::Interpreter => self.compile_and_run(source),
            Backend::IrInterpreter => self.compile_and_run_ir(source),
            Backend::Wasm => {
                let _wasm_bytes = self.compile_to_wasm(source)?;
                println!("WASM compilation successful (execution not implemented)");
                Ok(())
            }
            #[cfg(feature = "llvm")]
            Backend::Llvm => {
                let _llvm_ir = self.compile_to_llvm(source)?;
                println!("LLVM compilation successful (execution not implemented)");
                Ok(())
            }
            Backend::Hir => {
                let hir = self.compile_to_hir(source)?;
                println!("HIR compilation successful:");
                println!("{}", hir.to_json().unwrap_or_else(|_| "Failed to serialize HIR".to_string()));
                Ok(())
            }
            Backend::Mir => {
                let mir = self.compile_to_mir(source)?;
                println!("MIR compilation successful:");
                println!("{}", mir.to_json().unwrap_or_else(|_| "Failed to serialize MIR".to_string()));
                Ok(())
            }
        }
    }

    /// Compile and run using the default backend
    pub fn compile_and_run_default(&mut self, source: &str) -> OvieResult<()> {
        self.compile_and_run_with_backend(source, self.default_backend.clone())
    }

    /// Verify build reproducibility by compiling twice and comparing hashes
    pub fn verify_build_reproducibility(&mut self, source: &str, backend: Backend) -> OvieResult<bool> {
        // First build
        let first_result = match backend {
            Backend::Wasm => self.compile_to_wasm(source).map(|bytes| format!("{:x}", sha2::Sha256::digest(&bytes))),
            #[cfg(feature = "llvm")]
            Backend::Llvm => self.compile_to_llvm(source).map(|ir| format!("{:x}", sha2::Sha256::digest(ir.as_bytes()))),
            Backend::Hir => {
                let hir = self.compile_to_hir(source)?;
                let hir_json = hir.to_json().unwrap_or_default();
                Ok(format!("{:x}", sha2::Sha256::digest(hir_json.as_bytes())))
            }
            Backend::Mir => {
                let mir = self.compile_to_mir(source)?;
                let mir_json = mir.to_json().unwrap_or_default();
                Ok(format!("{:x}", sha2::Sha256::digest(mir_json.as_bytes())))
            }
            _ => {
                let ir = self.compile_to_ir(source)?;
                let ir_json = ir.to_json().unwrap_or_default();
                Ok(format!("{:x}", sha2::Sha256::digest(ir_json.as_bytes())))
            }
        }?;

        // Reset builder state for second build
        self.build_config = self.build_config.clone();

        // Second build
        let second_result = match backend {
            Backend::Wasm => self.compile_to_wasm(source).map(|bytes| format!("{:x}", sha2::Sha256::digest(&bytes))),
            #[cfg(feature = "llvm")]
            Backend::Llvm => self.compile_to_llvm(source).map(|ir| format!("{:x}", sha2::Sha256::digest(ir.as_bytes()))),
            Backend::Hir => {
                let hir = self.compile_to_hir(source)?;
                let hir_json = hir.to_json().unwrap_or_default();
                Ok(format!("{:x}", sha2::Sha256::digest(hir_json.as_bytes())))
            }
            Backend::Mir => {
                let mir = self.compile_to_mir(source)?;
                let mir_json = mir.to_json().unwrap_or_default();
                Ok(format!("{:x}", sha2::Sha256::digest(mir_json.as_bytes())))
            }
            _ => {
                let ir = self.compile_to_ir(source)?;
                let ir_json = ir.to_json().unwrap_or_default();
                Ok(format!("{:x}", sha2::Sha256::digest(ir_json.as_bytes())))
            }
        }?;

        if self.debug {
            println!("First build hash: {}", first_result);
            println!("Second build hash: {}", second_result);
        }

        Ok(first_result == second_result)
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}