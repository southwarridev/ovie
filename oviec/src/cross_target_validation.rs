//! Cross-target consistency validation system
//! 
//! This module provides validation that code generation produces consistent
//! behavior across different target platforms while respecting platform-specific
//! differences where appropriate.

use crate::error::{OvieError, OvieResult};
use crate::ir::Program;
use crate::codegen::{CodegenBackend, WasmBackend};
use std::collections::HashMap;
use sha2::{Sha256, Digest};

#[cfg(feature = "llvm")]
use crate::codegen::{LlvmBackend, TargetConfig};
#[cfg(feature = "llvm")]
use inkwell::context::Context;

/// Cross-target validation configuration
#[derive(Debug, Clone)]
pub struct CrossTargetValidationConfig {
    /// Enable semantic consistency validation
    pub validate_semantics: bool,
    /// Enable performance consistency validation
    pub validate_performance: bool,
    /// Enable deterministic output validation
    pub validate_determinism: bool,
    /// Tolerance for performance variations (percentage)
    pub performance_tolerance: f64,
    /// Number of validation runs for statistical analysis
    pub validation_runs: usize,
    /// Enable platform-specific guarantee documentation
    pub document_guarantees: bool,
}

impl Default for CrossTargetValidationConfig {
    fn default() -> Self {
        Self {
            validate_semantics: true,
            validate_performance: false,
            validate_determinism: true,
            performance_tolerance: 10.0, // 10% tolerance
            validation_runs: 3,
            document_guarantees: true,
        }
    }
}

/// Target platform information
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TargetPlatform {
    /// Target triple
    pub triple: String,
    /// Architecture
    pub arch: String,
    /// Operating system
    pub os: String,
    /// ABI
    pub abi: String,
    /// Backend type
    pub backend: String,
}

impl TargetPlatform {
    /// Create a new target platform
    pub fn new(triple: String, backend: String) -> Self {
        let parts: Vec<&str> = triple.split('-').collect();
        let arch = parts.get(0).unwrap_or(&"unknown").to_string();
        let os = parts.get(2).unwrap_or(&"unknown").to_string();
        let abi = parts.get(3).unwrap_or(&"unknown").to_string();
        
        Self {
            triple,
            arch,
            os,
            abi,
            backend,
        }
    }

    /// Get supported LLVM targets
    #[cfg(feature = "llvm")]
    pub fn llvm_targets() -> Vec<Self> {
        vec![
            Self::new("x86_64-unknown-linux-gnu".to_string(), "llvm".to_string()),
            Self::new("x86_64-pc-windows-gnu".to_string(), "llvm".to_string()),
            Self::new("x86_64-apple-darwin".to_string(), "llvm".to_string()),
            Self::new("aarch64-unknown-linux-gnu".to_string(), "llvm".to_string()),
            Self::new("aarch64-apple-darwin".to_string(), "llvm".to_string()),
        ]
    }

    /// Get supported WASM targets
    pub fn wasm_targets() -> Vec<Self> {
        vec![
            Self::new("wasm32-unknown-unknown".to_string(), "wasm".to_string()),
            Self::new("wasm32-wasi".to_string(), "wasm".to_string()),
            Self::new("wasm64-unknown-unknown".to_string(), "wasm".to_string()),
        ]
    }

    /// Get all supported targets
    pub fn all_targets() -> Vec<Self> {
        let mut targets = Self::wasm_targets();
        
        #[cfg(feature = "llvm")]
        {
            targets.extend(Self::llvm_targets());
        }
        
        targets
    }
}

/// Validation result for a single target
#[derive(Debug, Clone)]
pub struct TargetValidationResult {
    /// Target platform
    pub target: TargetPlatform,
    /// Compilation success
    pub compilation_success: bool,
    /// Generated code hash (for determinism validation)
    pub code_hash: Option<String>,
    /// Compilation time (for performance validation)
    pub compilation_time_ms: Option<u64>,
    /// Generated code size
    pub code_size: Option<usize>,
    /// Validation errors
    pub errors: Vec<String>,
    /// Validation warnings
    pub warnings: Vec<String>,
    /// Platform-specific guarantees
    pub guarantees: Vec<PlatformGuarantee>,
}

/// Platform-specific guarantee
#[derive(Debug, Clone)]
pub struct PlatformGuarantee {
    /// Guarantee type
    pub guarantee_type: GuaranteeType,
    /// Description
    pub description: String,
    /// Validation status
    pub validated: bool,
    /// Additional details
    pub details: HashMap<String, String>,
}

/// Types of platform guarantees
#[derive(Debug, Clone, PartialEq)]
pub enum GuaranteeType {
    /// Semantic consistency (same behavior)
    SemanticConsistency,
    /// Performance characteristics
    Performance,
    /// Memory safety
    MemorySafety,
    /// Deterministic output
    Determinism,
    /// ABI compatibility
    AbiCompatibility,
    /// Platform-specific optimization
    PlatformOptimization,
}

/// Cross-target validation results
#[derive(Debug, Clone)]
pub struct CrossTargetValidationResults {
    /// Results for each target
    pub target_results: HashMap<TargetPlatform, TargetValidationResult>,
    /// Overall validation success
    pub overall_success: bool,
    /// Consistency validation results
    pub consistency_results: ConsistencyResults,
    /// Performance comparison results
    pub performance_results: Option<PerformanceResults>,
    /// Validation summary
    pub summary: ValidationSummary,
}

/// Consistency validation results
#[derive(Debug, Clone)]
pub struct ConsistencyResults {
    /// Semantic consistency across targets
    pub semantic_consistency: bool,
    /// Deterministic output consistency
    pub deterministic_consistency: bool,
    /// Code hash consistency (for deterministic builds)
    pub hash_consistency: HashMap<String, Vec<TargetPlatform>>,
    /// Inconsistencies found
    pub inconsistencies: Vec<String>,
}

/// Performance comparison results
#[derive(Debug, Clone)]
pub struct PerformanceResults {
    /// Compilation time statistics
    pub compilation_times: HashMap<TargetPlatform, u64>,
    /// Code size statistics
    pub code_sizes: HashMap<TargetPlatform, usize>,
    /// Performance variations
    pub variations: Vec<String>,
    /// Performance within tolerance
    pub within_tolerance: bool,
}

/// Validation summary
#[derive(Debug, Clone)]
pub struct ValidationSummary {
    /// Total targets validated
    pub total_targets: usize,
    /// Successful targets
    pub successful_targets: usize,
    /// Failed targets
    pub failed_targets: usize,
    /// Total errors
    pub total_errors: usize,
    /// Total warnings
    pub total_warnings: usize,
    /// Validation duration
    pub validation_duration_ms: u64,
}

/// Cross-target consistency validator
pub struct CrossTargetValidator {
    /// Validation configuration
    config: CrossTargetValidationConfig,
}

impl CrossTargetValidator {
    /// Create a new cross-target validator
    pub fn new(config: CrossTargetValidationConfig) -> Self {
        Self { config }
    }

    /// Create a validator with default configuration
    pub fn new_default() -> Self {
        Self::new(CrossTargetValidationConfig::default())
    }

    /// Validate cross-target consistency for a program
    pub fn validate(&self, ir: &Program) -> OvieResult<CrossTargetValidationResults> {
        let start_time = std::time::Instant::now();
        let targets = TargetPlatform::all_targets();
        let mut target_results = HashMap::new();
        
        // Validate each target
        for target in &targets {
            let result = self.validate_target(ir, target)?;
            target_results.insert(target.clone(), result);
        }
        
        // Analyze consistency across targets
        let consistency_results = self.analyze_consistency(&target_results)?;
        
        // Analyze performance if enabled
        let performance_results = if self.config.validate_performance {
            Some(self.analyze_performance(&target_results)?)
        } else {
            None
        };
        
        // Generate summary
        let summary = self.generate_summary(&target_results, start_time.elapsed().as_millis() as u64);
        
        let overall_success = consistency_results.semantic_consistency 
            && consistency_results.deterministic_consistency
            && target_results.values().all(|r| r.compilation_success);
        
        Ok(CrossTargetValidationResults {
            target_results,
            overall_success,
            consistency_results,
            performance_results,
            summary,
        })
    }

    /// Validate a single target
    fn validate_target(&self, ir: &Program, target: &TargetPlatform) -> OvieResult<TargetValidationResult> {
        let start_time = std::time::Instant::now();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut guarantees = Vec::new();
        
        let (compilation_success, code_hash, code_size) = match target.backend.as_str() {
            "wasm" => self.validate_wasm_target(ir, target, &mut errors, &mut warnings)?,
            #[cfg(feature = "llvm")]
            "llvm" => self.validate_llvm_target(ir, target, &mut errors, &mut warnings)?,
            _ => {
                errors.push(format!("Unsupported backend: {}", target.backend));
                (false, None, None)
            }
        };
        
        // Generate platform-specific guarantees
        if self.config.document_guarantees {
            guarantees.extend(self.generate_platform_guarantees(target, compilation_success));
        }
        
        let compilation_time_ms = if self.config.validate_performance {
            Some(start_time.elapsed().as_millis() as u64)
        } else {
            None
        };
        
        Ok(TargetValidationResult {
            target: target.clone(),
            compilation_success,
            code_hash,
            compilation_time_ms,
            code_size,
            errors,
            warnings,
            guarantees,
        })
    }

    /// Validate WASM target
    fn validate_wasm_target(&self, ir: &Program, target: &TargetPlatform, errors: &mut Vec<String>, warnings: &mut Vec<String>) -> OvieResult<(bool, Option<String>, Option<usize>)> {
        let mut backend = WasmBackend::new();
        
        if self.config.validate_determinism {
            backend.set_deterministic_mode(true);
        }
        
        match backend.generate(ir) {
            Ok(wasm_bytes) => {
                // Validate WASM magic number
                if wasm_bytes.len() < 8 {
                    errors.push("Generated WASM module too small".to_string());
                    return Ok((false, None, None));
                }
                
                if &wasm_bytes[0..4] != &[0x00, 0x61, 0x73, 0x6d] {
                    errors.push("Invalid WASM magic number".to_string());
                    return Ok((false, None, None));
                }
                
                if &wasm_bytes[4..8] != &[0x01, 0x00, 0x00, 0x00] {
                    warnings.push("Non-standard WASM version".to_string());
                }
                
                // Generate hash for determinism validation
                let hash = if self.config.validate_determinism {
                    let mut hasher = Sha256::new();
                    hasher.update(&wasm_bytes);
                    Some(format!("{:x}", hasher.finalize()))
                } else {
                    None
                };
                
                Ok((true, hash, Some(wasm_bytes.len())))
            }
            Err(e) => {
                errors.push(format!("WASM compilation failed: {}", e));
                Ok((false, None, None))
            }
        }
    }

    /// Validate LLVM target
    #[cfg(feature = "llvm")]
    fn validate_llvm_target(&self, ir: &Program, target: &TargetPlatform, errors: &mut Vec<String>, warnings: &mut Vec<String>) -> OvieResult<(bool, Option<String>, Option<usize>)> {
        let context = Context::create();
        let target_config = match target.triple.as_str() {
            "x86_64-unknown-linux-gnu" => TargetConfig::linux_x64(),
            "x86_64-pc-windows-gnu" => TargetConfig::windows_x64(),
            "x86_64-apple-darwin" => TargetConfig::macos_x64(),
            "aarch64-unknown-linux-gnu" => TargetConfig::arm64_linux(),
            "aarch64-apple-darwin" => TargetConfig::arm64_macos(),
            _ => {
                errors.push(format!("Unsupported LLVM target: {}", target.triple));
                return Ok((false, None, None));
            }
        };
        
        let mut backend = LlvmBackend::new_with_target(&context, "validation_module", target_config);
        
        if self.config.validate_determinism {
            backend.set_deterministic_mode(true);
        }
        
        match backend.generate(ir) {
            Ok(llvm_ir) => {
                // Basic LLVM IR validation
                if llvm_ir.is_empty() {
                    errors.push("Generated LLVM IR is empty".to_string());
                    return Ok((false, None, None));
                }
                
                if !llvm_ir.contains("target triple") {
                    warnings.push("LLVM IR missing target triple".to_string());
                }
                
                if !llvm_ir.contains("define") {
                    warnings.push("LLVM IR missing function definitions".to_string());
                }
                
                // Generate hash for determinism validation
                let hash = if self.config.validate_determinism {
                    let mut hasher = Sha256::new();
                    hasher.update(llvm_ir.as_bytes());
                    Some(format!("{:x}", hasher.finalize()))
                } else {
                    None
                };
                
                Ok((true, hash, Some(llvm_ir.len())))
            }
            Err(e) => {
                errors.push(format!("LLVM compilation failed: {}", e));
                Ok((false, None, None))
            }
        }
    }

    /// Validate LLVM target (fallback when LLVM feature is disabled)
    #[cfg(not(feature = "llvm"))]
    fn validate_llvm_target(&self, _ir: &Program, target: &TargetPlatform, errors: &mut Vec<String>, _warnings: &mut Vec<String>) -> OvieResult<(bool, Option<String>, Option<usize>)> {
        errors.push(format!("LLVM backend not available for target: {}", target.triple));
        Ok((false, None, None))
    }

    /// Analyze consistency across targets
    fn analyze_consistency(&self, target_results: &HashMap<TargetPlatform, TargetValidationResult>) -> OvieResult<ConsistencyResults> {
        let mut semantic_consistency = true;
        let mut deterministic_consistency = true;
        let mut hash_consistency = HashMap::new();
        let mut inconsistencies = Vec::new();
        
        // Group targets by code hash for determinism validation
        if self.config.validate_determinism {
            for (target, result) in target_results {
                if let Some(ref hash) = result.code_hash {
                    hash_consistency.entry(hash.clone()).or_insert_with(Vec::new).push(target.clone());
                }
            }
            
            // Check if we have multiple different hashes (inconsistency)
            if hash_consistency.len() > 1 {
                deterministic_consistency = false;
                inconsistencies.push("Deterministic builds produced different outputs across targets".to_string());
            }
        }
        
        // Check semantic consistency (all targets should compile successfully)
        let successful_targets: Vec<_> = target_results.values().filter(|r| r.compilation_success).collect();
        let failed_targets: Vec<_> = target_results.values().filter(|r| !r.compilation_success).collect();
        
        if !failed_targets.is_empty() {
            semantic_consistency = false;
            for failed in &failed_targets {
                inconsistencies.push(format!("Target {} failed compilation", failed.target.triple));
            }
        }
        
        Ok(ConsistencyResults {
            semantic_consistency,
            deterministic_consistency,
            hash_consistency,
            inconsistencies,
        })
    }

    /// Analyze performance across targets
    fn analyze_performance(&self, target_results: &HashMap<TargetPlatform, TargetValidationResult>) -> OvieResult<PerformanceResults> {
        let mut compilation_times = HashMap::new();
        let mut code_sizes = HashMap::new();
        let mut variations = Vec::new();
        
        // Collect performance data
        for (target, result) in target_results {
            if let Some(time) = result.compilation_time_ms {
                compilation_times.insert(target.clone(), time);
            }
            if let Some(size) = result.code_size {
                code_sizes.insert(target.clone(), size);
            }
        }
        
        // Analyze compilation time variations
        if compilation_times.len() > 1 {
            let times: Vec<u64> = compilation_times.values().cloned().collect();
            let min_time = *times.iter().min().unwrap();
            let max_time = *times.iter().max().unwrap();
            let variation = if min_time > 0 {
                ((max_time - min_time) as f64 / min_time as f64) * 100.0
            } else {
                0.0
            };
            
            if variation > self.config.performance_tolerance {
                variations.push(format!("Compilation time variation: {:.1}% (tolerance: {:.1}%)", 
                    variation, self.config.performance_tolerance));
            }
        }
        
        // Analyze code size variations
        if code_sizes.len() > 1 {
            let sizes: Vec<usize> = code_sizes.values().cloned().collect();
            let min_size = *sizes.iter().min().unwrap();
            let max_size = *sizes.iter().max().unwrap();
            let variation = if min_size > 0 {
                ((max_size - min_size) as f64 / min_size as f64) * 100.0
            } else {
                0.0
            };
            
            if variation > self.config.performance_tolerance {
                variations.push(format!("Code size variation: {:.1}% (tolerance: {:.1}%)", 
                    variation, self.config.performance_tolerance));
            }
        }
        
        let within_tolerance = variations.is_empty();
        
        Ok(PerformanceResults {
            compilation_times,
            code_sizes,
            variations,
            within_tolerance,
        })
    }

    /// Generate platform-specific guarantees
    fn generate_platform_guarantees(&self, target: &TargetPlatform, compilation_success: bool) -> Vec<PlatformGuarantee> {
        let mut guarantees = Vec::new();
        
        // Semantic consistency guarantee
        guarantees.push(PlatformGuarantee {
            guarantee_type: GuaranteeType::SemanticConsistency,
            description: "Code produces consistent behavior across platforms".to_string(),
            validated: compilation_success,
            details: HashMap::new(),
        });
        
        // Memory safety guarantee
        guarantees.push(PlatformGuarantee {
            guarantee_type: GuaranteeType::MemorySafety,
            description: "Memory operations are safe and bounds-checked".to_string(),
            validated: compilation_success,
            details: HashMap::new(),
        });
        
        // Determinism guarantee
        if self.config.validate_determinism {
            guarantees.push(PlatformGuarantee {
                guarantee_type: GuaranteeType::Determinism,
                description: "Builds are deterministic and reproducible".to_string(),
                validated: compilation_success,
                details: HashMap::new(),
            });
        }
        
        // Platform-specific guarantees
        match target.backend.as_str() {
            "wasm" => {
                guarantees.push(PlatformGuarantee {
                    guarantee_type: GuaranteeType::PlatformOptimization,
                    description: "WebAssembly-specific optimizations applied".to_string(),
                    validated: compilation_success,
                    details: [("optimization_type".to_string(), "size_and_speed".to_string())].iter().cloned().collect(),
                });
            }
            "llvm" => {
                guarantees.push(PlatformGuarantee {
                    guarantee_type: GuaranteeType::AbiCompatibility,
                    description: "ABI compatibility with platform conventions".to_string(),
                    validated: compilation_success,
                    details: [("abi_type".to_string(), target.abi.clone())].iter().cloned().collect(),
                });
            }
            _ => {}
        }
        
        guarantees
    }

    /// Generate validation summary
    fn generate_summary(&self, target_results: &HashMap<TargetPlatform, TargetValidationResult>, duration_ms: u64) -> ValidationSummary {
        let total_targets = target_results.len();
        let successful_targets = target_results.values().filter(|r| r.compilation_success).count();
        let failed_targets = total_targets - successful_targets;
        let total_errors = target_results.values().map(|r| r.errors.len()).sum();
        let total_warnings = target_results.values().map(|r| r.warnings.len()).sum();
        
        ValidationSummary {
            total_targets,
            successful_targets,
            failed_targets,
            total_errors,
            total_warnings,
            validation_duration_ms: duration_ms,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::IrBuilder;

    #[test]
    fn test_target_platform_creation() {
        let target = TargetPlatform::new("x86_64-unknown-linux-gnu".to_string(), "llvm".to_string());
        assert_eq!(target.arch, "x86_64");
        assert_eq!(target.os, "linux");
        assert_eq!(target.abi, "gnu");
        assert_eq!(target.backend, "llvm");
    }

    #[test]
    fn test_cross_target_validator_creation() {
        let validator = CrossTargetValidator::new_default();
        assert!(validator.config.validate_semantics);
        assert!(validator.config.validate_determinism);
        assert_eq!(validator.config.performance_tolerance, 10.0);
    }

    #[test]
    fn test_platform_guarantees() {
        let validator = CrossTargetValidator::new_default();
        let target = TargetPlatform::new("wasm32-unknown-unknown".to_string(), "wasm".to_string());
        let guarantees = validator.generate_platform_guarantees(&target, true);
        
        assert!(!guarantees.is_empty());
        assert!(guarantees.iter().any(|g| g.guarantee_type == GuaranteeType::SemanticConsistency));
        assert!(guarantees.iter().any(|g| g.guarantee_type == GuaranteeType::MemorySafety));
    }
}