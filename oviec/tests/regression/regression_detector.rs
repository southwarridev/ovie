//! Comprehensive Regression Detection System
//! 
//! This module provides advanced regression detection capabilities for the Ovie compiler,
//! including behavioral regression detection, performance regression analysis, and
//! cross-platform consistency validation.

use crate::{Compiler, Backend, OvieResult, OvieError};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

/// Comprehensive regression detector for compiler behavior and performance
pub struct RegressionDetector {
    /// Historical baseline data for comparison
    baseline_data: BaselineData,
    /// Configuration for regression detection
    config: RegressionConfig,
    /// Cross-platform validation results
    cross_platform_cache: HashMap<String, CrossPlatformResult>,
}

/// Configuration for regression detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionConfig {
    /// Performance regression threshold (percentage)
    pub performance_threshold: f64,
    /// Behavioral change tolerance
    pub behavioral_tolerance: BehavioralTolerance,
    /// Cross-platform consistency requirements
    pub cross_platform_requirements: CrossPlatformRequirements,
    /// Test case selection strategy
    pub test_selection: TestSelectionStrategy,
}

/// Tolerance levels for behavioral changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralTolerance {
    /// Allow minor AST structure changes
    pub allow_ast_structure_changes: bool,
    /// Allow error message improvements
    pub allow_error_message_improvements: bool,
    /// Allow performance optimizations
    pub allow_performance_optimizations: bool,
    /// Require exact output matching
    pub require_exact_output_match: bool,
}

/// Cross-platform consistency requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossPlatformRequirements {
    /// Required platforms for consistency validation
    pub required_platforms: Vec<String>,
    /// Minimum consistency percentage
    pub minimum_consistency_percentage: f64,
    /// Allow platform-specific optimizations
    pub allow_platform_optimizations: bool,
    /// Require identical error messages across platforms
    pub require_identical_errors: bool,
}

/// Test selection strategy for regression detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestSelectionStrategy {
    /// Test all available test cases
    Comprehensive,
    /// Test a representative sample
    Representative { sample_size: usize },
    /// Test only critical functionality
    Critical,
    /// Custom test selection
    Custom { test_patterns: Vec<String> },
}

/// Historical baseline data for regression comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineData {
    /// Compiler version when baseline was created
    pub compiler_version: String,
    /// Timestamp when baseline was created
    pub created_at: u64,
    /// Compilation behavior baselines
    pub compilation_baselines: HashMap<String, CompilationBaseline>,
    /// Performance baselines
    pub performance_baselines: HashMap<String, PerformanceBaseline>,
    /// Cross-platform consistency baselines
    pub cross_platform_baselines: HashMap<String, CrossPlatformBaseline>,
}

/// Baseline data for compilation behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationBaseline {
    /// Source code hash
    pub source_hash: String,
    /// Expected compilation result
    pub expected_result: CompilationResult,
    /// AST structure hash (if compilation succeeds)
    pub ast_hash: Option<String>,
    /// HIR structure hash (if type checking succeeds)
    pub hir_hash: Option<String>,
    /// MIR structure hash (if MIR generation succeeds)
    pub mir_hash: Option<String>,
    /// Generated code hash (if code generation succeeds)
    pub code_hash: Option<String>,
    /// Error signature (if compilation fails)
    pub error_signature: Option<String>,
}

/// Expected compilation result
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompilationResult {
    Success,
    SyntaxError,
    TypeError,
    NameResolutionError,
    BorrowCheckError,
    CodeGenError,
    RuntimeError,
}

/// Performance baseline data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    /// Lexing time baseline
    pub lexing_time: Duration,
    /// Parsing time baseline
    pub parsing_time: Duration,
    /// Type checking time baseline
    pub type_checking_time: Duration,
    /// Code generation time baseline
    pub code_generation_time: Duration,
    /// End-to-end compilation time baseline
    pub end_to_end_time: Duration,
    /// Memory usage baseline
    pub memory_usage: u64,
    /// Throughput baseline (operations per second)
    pub throughput: f64,
}

/// Cross-platform consistency baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossPlatformBaseline {
    /// Platform-specific results
    pub platform_results: HashMap<String, PlatformResult>,
    /// Overall consistency percentage
    pub consistency_percentage: f64,
    /// Known platform-specific differences
    pub known_differences: Vec<PlatformDifference>,
}

/// Result for a specific platform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformResult {
    /// Compilation success/failure
    pub compilation_result: CompilationResult,
    /// Generated code hash
    pub code_hash: Option<String>,
    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,
    /// Platform-specific metadata
    pub platform_metadata: HashMap<String, String>,
}

/// Performance metrics for a platform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Compilation time
    pub compilation_time: Duration,
    /// Memory usage
    pub memory_usage: u64,
    /// Generated code size
    pub code_size: u64,
}

/// Known platform-specific difference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformDifference {
    /// Platforms affected
    pub platforms: Vec<String>,
    /// Description of the difference
    pub description: String,
    /// Whether this difference is acceptable
    pub acceptable: bool,
    /// Justification for the difference
    pub justification: String,
}

/// Cross-platform validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossPlatformResult {
    /// Test case identifier
    pub test_case: String,
    /// Results per platform
    pub platform_results: HashMap<String, PlatformResult>,
    /// Consistency analysis
    pub consistency_analysis: ConsistencyAnalysis,
    /// Detected inconsistencies
    pub inconsistencies: Vec<Inconsistency>,
}

/// Consistency analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyAnalysis {
    /// Overall consistency percentage
    pub consistency_percentage: f64,
    /// Number of consistent behaviors
    pub consistent_behaviors: usize,
    /// Number of inconsistent behaviors
    pub inconsistent_behaviors: usize,
    /// Consistency trend compared to baseline
    pub trend: ConsistencyTrend,
}

/// Consistency trend analysis
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsistencyTrend {
    Improving,
    Stable,
    Degrading,
}

/// Detected inconsistency between platforms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inconsistency {
    /// Platforms involved in the inconsistency
    pub platforms: Vec<String>,
    /// Type of inconsistency
    pub inconsistency_type: InconsistencyType,
    /// Severity of the inconsistency
    pub severity: InconsistencySeverity,
    /// Detailed description
    pub description: String,
    /// Suggested resolution
    pub suggested_resolution: String,
}

/// Types of cross-platform inconsistencies
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InconsistencyType {
    CompilationResult,
    GeneratedCode,
    Performance,
    ErrorMessage,
    Behavior,
}

/// Severity levels for inconsistencies
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InconsistencySeverity {
    Critical,  // Breaks functionality
    Major,     // Significant difference in behavior
    Minor,     // Acceptable difference
    Cosmetic,  // Only affects non-functional aspects
}

/// Comprehensive regression detection results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionDetectionResults {
    /// Behavioral regressions detected
    pub behavioral_regressions: Vec<BehavioralRegression>,
    /// Performance regressions detected
    pub performance_regressions: Vec<PerformanceRegression>,
    /// Cross-platform consistency regressions
    pub consistency_regressions: Vec<ConsistencyRegression>,
    /// Overall regression risk assessment
    pub risk_assessment: RegressionRiskAssessment,
    /// Recommended actions
    pub recommended_actions: Vec<RecommendedAction>,
}

/// Behavioral regression detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralRegression {
    /// Test case that detected the regression
    pub test_case: String,
    /// Component affected
    pub component: String,
    /// Type of behavioral change
    pub change_type: BehavioralChangeType,
    /// Severity of the regression
    pub severity: RegressionSeverity,
    /// Detailed description
    pub description: String,
    /// Evidence of the regression
    pub evidence: RegressionEvidence,
}

/// Types of behavioral changes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BehavioralChangeType {
    CompilationSuccess,    // Previously failing code now compiles
    CompilationFailure,    // Previously compiling code now fails
    OutputChange,          // Generated code or output changed
    ErrorMessageChange,    // Error messages changed
    PerformanceChange,     // Significant performance change
    StructuralChange,      // AST/HIR/MIR structure changed
}

/// Evidence supporting a regression detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionEvidence {
    /// Baseline hash or signature
    pub baseline_signature: String,
    /// Current hash or signature
    pub current_signature: String,
    /// Diff or change description
    pub change_description: String,
    /// Supporting data
    pub supporting_data: HashMap<String, String>,
}

/// Performance regression detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRegression {
    /// Benchmark name
    pub benchmark_name: String,
    /// Performance metric affected
    pub metric: PerformanceMetric,
    /// Baseline value
    pub baseline_value: f64,
    /// Current value
    pub current_value: f64,
    /// Percentage change (negative = regression)
    pub percentage_change: f64,
    /// Severity assessment
    pub severity: RegressionSeverity,
}

/// Performance metrics that can regress
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerformanceMetric {
    CompilationTime,
    MemoryUsage,
    CodeSize,
    Throughput,
    Latency,
}

/// Cross-platform consistency regression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyRegression {
    /// Test case affected
    pub test_case: String,
    /// Platforms with inconsistent behavior
    pub affected_platforms: Vec<String>,
    /// Baseline consistency percentage
    pub baseline_consistency: f64,
    /// Current consistency percentage
    pub current_consistency: f64,
    /// Consistency change
    pub consistency_change: f64,
    /// New inconsistencies detected
    pub new_inconsistencies: Vec<Inconsistency>,
}

/// Overall regression risk assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionRiskAssessment {
    /// Overall risk level
    pub risk_level: RiskLevel,
    /// Risk factors identified
    pub risk_factors: Vec<RiskFactor>,
    /// Confidence in the assessment
    pub confidence: f64,
    /// Recommended response
    pub recommended_response: ResponseLevel,
}

/// Risk levels for regression assessment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,       // Minor regressions, low impact
    Medium,    // Some significant regressions
    High,      // Major regressions affecting core functionality
    Critical,  // Severe regressions breaking essential features
}

/// Risk factors contributing to regression assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    /// Factor description
    pub description: String,
    /// Impact level
    pub impact: ImpactLevel,
    /// Likelihood of causing issues
    pub likelihood: f64,
    /// Affected components
    pub affected_components: Vec<String>,
}

/// Impact levels for risk factors
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpactLevel {
    Negligible,
    Minor,
    Moderate,
    Major,
    Severe,
}

/// Recommended response levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseLevel {
    Monitor,      // Continue monitoring, no immediate action needed
    Investigate,  // Investigate regressions before proceeding
    Fix,          // Fix regressions before release
    Block,        // Block release until critical issues resolved
}

/// Recommended actions for addressing regressions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedAction {
    /// Action description
    pub description: String,
    /// Priority level
    pub priority: ActionPriority,
    /// Estimated effort
    pub estimated_effort: EffortLevel,
    /// Target components
    pub target_components: Vec<String>,
    /// Success criteria
    pub success_criteria: Vec<String>,
}

/// Priority levels for recommended actions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionPriority {
    Critical,  // Must be done immediately
    High,      // Should be done soon
    Medium,    // Should be done eventually
    Low,       // Nice to have
}

/// Effort levels for recommended actions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffortLevel {
    Trivial,   // < 1 hour
    Small,     // 1-4 hours
    Medium,    // 1-2 days
    Large,     // 1-2 weeks
    Massive,   // > 2 weeks
}

/// Regression severity classification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegressionSeverity {
    Critical,  // Breaks core functionality
    Major,     // Significant impact on functionality
    Minor,     // Small impact, may be acceptable
    Cosmetic,  // No functional impact
}

impl RegressionDetector {
    /// Create a new regression detector with default configuration
    pub fn new() -> Self {
        Self {
            baseline_data: BaselineData::new(),
            config: RegressionConfig::default(),
            cross_platform_cache: HashMap::new(),
        }
    }

    /// Create a regression detector with custom configuration
    pub fn with_config(config: RegressionConfig) -> Self {
        Self {
            baseline_data: BaselineData::new(),
            config,
            cross_platform_cache: HashMap::new(),
        }
    }

    /// Load baseline data from storage
    pub fn load_baseline(&mut self, baseline_data: BaselineData) {
        self.baseline_data = baseline_data;
    }

    /// Create a new baseline from current compiler behavior
    pub fn create_baseline(&mut self, test_cases: &[String]) -> OvieResult<BaselineData> {
        let mut baseline = BaselineData::new();
        
        for test_case in test_cases {
            // Create compilation baseline
            let compilation_baseline = self.create_compilation_baseline(test_case)?;
            baseline.compilation_baselines.insert(test_case.clone(), compilation_baseline);
            
            // Create performance baseline
            let performance_baseline = self.create_performance_baseline(test_case)?;
            baseline.performance_baselines.insert(test_case.clone(), performance_baseline);
            
            // Create cross-platform baseline
            let cross_platform_baseline = self.create_cross_platform_baseline(test_case)?;
            baseline.cross_platform_baselines.insert(test_case.clone(), cross_platform_baseline);
        }
        
        Ok(baseline)
    }

    /// Detect regressions by comparing current behavior with baseline
    pub fn detect_regressions(&mut self, test_cases: &[String]) -> OvieResult<RegressionDetectionResults> {
        let mut behavioral_regressions = Vec::new();
        let mut performance_regressions = Vec::new();
        let mut consistency_regressions = Vec::new();
        
        for test_case in test_cases {
            // Detect behavioral regressions
            if let Some(behavioral_regression) = self.detect_behavioral_regression(test_case)? {
                behavioral_regressions.push(behavioral_regression);
            }
            
            // Detect performance regressions
            if let Some(performance_regression) = self.detect_performance_regression(test_case)? {
                performance_regressions.push(performance_regression);
            }
            
            // Detect consistency regressions
            if let Some(consistency_regression) = self.detect_consistency_regression(test_case)? {
                consistency_regressions.push(consistency_regression);
            }
        }
        
        // Assess overall risk
        let risk_assessment = self.assess_regression_risk(
            &behavioral_regressions,
            &performance_regressions,
            &consistency_regressions,
        );
        
        // Generate recommended actions
        let recommended_actions = self.generate_recommended_actions(
            &behavioral_regressions,
            &performance_regressions,
            &consistency_regressions,
            &risk_assessment,
        );
        
        Ok(RegressionDetectionResults {
            behavioral_regressions,
            performance_regressions,
            consistency_regressions,
            risk_assessment,
            recommended_actions,
        })
    }

    /// Validate cross-platform consistency for all supported targets
    pub fn validate_cross_platform_consistency(&mut self, test_cases: &[String]) -> OvieResult<Vec<CrossPlatformResult>> {
        let mut results = Vec::new();
        
        for test_case in test_cases {
            let result = self.validate_single_test_cross_platform(test_case)?;
            results.push(result);
        }
        
        Ok(results)
    }

    /// Validate cross-platform consistency for a single test case
    fn validate_single_test_cross_platform(&mut self, test_case: &str) -> OvieResult<CrossPlatformResult> {
        let mut platform_results = HashMap::new();
        let mut inconsistencies = Vec::new();
        
        // Test on each required platform
        for platform in &self.config.cross_platform_requirements.required_platforms {
            let platform_result = self.test_on_platform(test_case, platform)?;
            platform_results.insert(platform.clone(), platform_result);
        }
        
        // Analyze consistency across platforms
        let consistency_analysis = self.analyze_cross_platform_consistency(&platform_results);
        
        // Detect inconsistencies
        inconsistencies.extend(self.detect_cross_platform_inconsistencies(&platform_results)?);
        
        Ok(CrossPlatformResult {
            test_case: test_case.to_string(),
            platform_results,
            consistency_analysis,
            inconsistencies,
        })
    }

    /// Test a single case on a specific platform
    fn test_on_platform(&self, test_case: &str, platform: &str) -> OvieResult<PlatformResult> {
        let mut compiler = Compiler::new_deterministic();
        
        let start_time = Instant::now();
        
        // Attempt compilation
        let compilation_result = match platform {
            "wasm32-unknown-unknown" => {
                match compiler.compile_to_wasm(test_case) {
                    Ok(wasm_bytes) => {
                        let code_hash = self.compute_hash(&wasm_bytes);
                        (CompilationResult::Success, Some(code_hash), wasm_bytes.len() as u64)
                    }
                    Err(_) => (CompilationResult::CodeGenError, None, 0),
                }
            }
            #[cfg(feature = "llvm")]
            "x86_64-pc-windows-gnu" | "x86_64-unknown-linux-gnu" => {
                match compiler.compile_to_llvm(test_case) {
                    Ok(llvm_ir) => {
                        let code_hash = self.compute_hash(llvm_ir.as_bytes());
                        (CompilationResult::Success, Some(code_hash), llvm_ir.len() as u64)
                    }
                    Err(_) => (CompilationResult::CodeGenError, None, 0),
                }
            }
            "interpreter" => {
                match compiler.compile_and_run(test_case) {
                    Ok(_) => (CompilationResult::Success, None, 0),
                    Err(_) => (CompilationResult::RuntimeError, None, 0),
                }
            }
            _ => {
                // Fallback to AST compilation for unknown platforms
                match compiler.compile_to_ast(test_case) {
                    Ok(ast) => {
                        let ast_json = serde_json::to_string(&ast).unwrap_or_default();
                        let code_hash = self.compute_hash(ast_json.as_bytes());
                        (CompilationResult::Success, Some(code_hash), ast_json.len() as u64)
                    }
                    Err(_) => (CompilationResult::SyntaxError, None, 0),
                }
            }
        };
        
        let compilation_time = start_time.elapsed();
        
        Ok(PlatformResult {
            compilation_result: compilation_result.0,
            code_hash: compilation_result.1,
            performance_metrics: PerformanceMetrics {
                compilation_time,
                memory_usage: 0, // Would measure actual memory usage
                code_size: compilation_result.2,
            },
            platform_metadata: HashMap::new(),
        })
    }

    /// Analyze cross-platform consistency
    fn analyze_cross_platform_consistency(&self, platform_results: &HashMap<String, PlatformResult>) -> ConsistencyAnalysis {
        let total_platforms = platform_results.len();
        if total_platforms < 2 {
            return ConsistencyAnalysis {
                consistency_percentage: 100.0,
                consistent_behaviors: 1,
                inconsistent_behaviors: 0,
                trend: ConsistencyTrend::Stable,
            };
        }
        
        let mut consistent_behaviors = 0;
        let mut inconsistent_behaviors = 0;
        
        // Check compilation result consistency
        let compilation_results: Vec<_> = platform_results.values()
            .map(|r| &r.compilation_result)
            .collect();
        
        if compilation_results.iter().all(|&r| r == compilation_results[0]) {
            consistent_behaviors += 1;
        } else {
            inconsistent_behaviors += 1;
        }
        
        // Check code hash consistency (if available)
        let code_hashes: Vec<_> = platform_results.values()
            .filter_map(|r| r.code_hash.as_ref())
            .collect();
        
        if !code_hashes.is_empty() {
            if code_hashes.iter().all(|&h| h == code_hashes[0]) {
                consistent_behaviors += 1;
            } else {
                inconsistent_behaviors += 1;
            }
        }
        
        let total_behaviors = consistent_behaviors + inconsistent_behaviors;
        let consistency_percentage = if total_behaviors > 0 {
            (consistent_behaviors as f64 / total_behaviors as f64) * 100.0
        } else {
            100.0
        };
        
        // Determine trend (would compare with baseline in real implementation)
        let trend = ConsistencyTrend::Stable;
        
        ConsistencyAnalysis {
            consistency_percentage,
            consistent_behaviors,
            inconsistent_behaviors,
            trend,
        }
    }

    /// Detect cross-platform inconsistencies
    fn detect_cross_platform_inconsistencies(&self, platform_results: &HashMap<String, PlatformResult>) -> OvieResult<Vec<Inconsistency>> {
        let mut inconsistencies = Vec::new();
        
        // Group platforms by compilation result
        let mut result_groups: HashMap<CompilationResult, Vec<String>> = HashMap::new();
        for (platform, result) in platform_results {
            result_groups.entry(result.compilation_result.clone())
                .or_insert_with(Vec::new)
                .push(platform.clone());
        }
        
        // If we have multiple groups, there's an inconsistency
        if result_groups.len() > 1 {
            for (result, platforms) in result_groups {
                if platforms.len() < platform_results.len() {
                    inconsistencies.push(Inconsistency {
                        platforms,
                        inconsistency_type: InconsistencyType::CompilationResult,
                        severity: InconsistencySeverity::Major,
                        description: format!("Compilation result differs: {:?}", result),
                        suggested_resolution: "Investigate platform-specific compilation differences".to_string(),
                    });
                }
            }
        }
        
        // Check for code hash inconsistencies
        let mut hash_groups: HashMap<Option<String>, Vec<String>> = HashMap::new();
        for (platform, result) in platform_results {
            hash_groups.entry(result.code_hash.clone())
                .or_insert_with(Vec::new)
                .push(platform.clone());
        }
        
        if hash_groups.len() > 1 {
            for (hash, platforms) in hash_groups {
                if platforms.len() < platform_results.len() {
                    inconsistencies.push(Inconsistency {
                        platforms,
                        inconsistency_type: InconsistencyType::GeneratedCode,
                        severity: InconsistencySeverity::Minor,
                        description: format!("Generated code differs: {:?}", hash),
                        suggested_resolution: "Review platform-specific code generation".to_string(),
                    });
                }
            }
        }
        
        Ok(inconsistencies)
    }

    /// Create compilation baseline for a test case
    fn create_compilation_baseline(&self, test_case: &str) -> OvieResult<CompilationBaseline> {
        let mut compiler = Compiler::new_deterministic();
        let source_hash = self.compute_hash(test_case.as_bytes());
        
        // Attempt compilation and capture results
        let (expected_result, ast_hash, hir_hash, mir_hash, code_hash, error_signature) = 
            match compiler.compile_to_ast(test_case) {
                Ok(ast) => {
                    let ast_json = serde_json::to_string(&ast).unwrap_or_default();
                    let ast_hash = Some(self.compute_hash(ast_json.as_bytes()));
                    
                    match compiler.compile_to_hir(test_case) {
                        Ok(hir) => {
                            let hir_json = hir.to_json().unwrap_or_default();
                            let hir_hash = Some(self.compute_hash(hir_json.as_bytes()));
                            
                            match compiler.compile_to_mir(test_case) {
                                Ok(mir) => {
                                    let mir_json = mir.to_json().unwrap_or_default();
                                    let mir_hash = Some(self.compute_hash(mir_json.as_bytes()));
                                    
                                    match compiler.compile_to_wasm(test_case) {
                                        Ok(wasm_bytes) => {
                                            let code_hash = Some(self.compute_hash(&wasm_bytes));
                                            (CompilationResult::Success, ast_hash, hir_hash, mir_hash, code_hash, None)
                                        }
                                        Err(error) => {
                                            let error_sig = Some(self.compute_hash(error.to_string().as_bytes()));
                                            (CompilationResult::CodeGenError, ast_hash, hir_hash, mir_hash, None, error_sig)
                                        }
                                    }
                                }
                                Err(error) => {
                                    let error_sig = Some(self.compute_hash(error.to_string().as_bytes()));
                                    (CompilationResult::BorrowCheckError, ast_hash, hir_hash, None, None, error_sig)
                                }
                            }
                        }
                        Err(error) => {
                            let error_sig = Some(self.compute_hash(error.to_string().as_bytes()));
                            (CompilationResult::TypeError, ast_hash, None, None, None, error_sig)
                        }
                    }
                }
                Err(error) => {
                    let error_sig = Some(self.compute_hash(error.to_string().as_bytes()));
                    (CompilationResult::SyntaxError, None, None, None, None, error_sig)
                }
            };
        
        Ok(CompilationBaseline {
            source_hash,
            expected_result,
            ast_hash,
            hir_hash,
            mir_hash,
            code_hash,
            error_signature,
        })
    }

    /// Create performance baseline for a test case
    fn create_performance_baseline(&self, test_case: &str) -> OvieResult<PerformanceBaseline> {
        let mut compiler = Compiler::new_deterministic();
        
        // Measure lexing time
        let start = Instant::now();
        for _ in 0..100 {
            let _ = compiler.compile_to_ast(test_case);
        }
        let lexing_time = start.elapsed() / 100;
        
        // For simplicity, use lexing time as baseline for all stages
        // In a real implementation, each stage would be measured separately
        Ok(PerformanceBaseline {
            lexing_time,
            parsing_time: lexing_time,
            type_checking_time: lexing_time * 2,
            code_generation_time: lexing_time * 3,
            end_to_end_time: lexing_time * 6,
            memory_usage: 1024 * 1024, // 1MB placeholder
            throughput: 100.0, // 100 ops/sec placeholder
        })
    }

    /// Create cross-platform baseline for a test case
    fn create_cross_platform_baseline(&self, test_case: &str) -> OvieResult<CrossPlatformBaseline> {
        let mut platform_results = HashMap::new();
        
        // Test on each platform
        for platform in &self.config.cross_platform_requirements.required_platforms {
            let result = self.test_on_platform(test_case, platform)?;
            platform_results.insert(platform.clone(), result);
        }
        
        // Analyze consistency
        let consistency_analysis = self.analyze_cross_platform_consistency(&platform_results);
        
        Ok(CrossPlatformBaseline {
            platform_results,
            consistency_percentage: consistency_analysis.consistency_percentage,
            known_differences: Vec::new(), // Would be populated with known acceptable differences
        })
    }

    /// Detect behavioral regression for a test case
    fn detect_behavioral_regression(&self, test_case: &str) -> OvieResult<Option<BehavioralRegression>> {
        if let Some(baseline) = self.baseline_data.compilation_baselines.get(test_case) {
            let current_baseline = self.create_compilation_baseline(test_case)?;
            
            // Compare results
            if baseline.expected_result != current_baseline.expected_result {
                return Ok(Some(BehavioralRegression {
                    test_case: test_case.to_string(),
                    component: "Compiler".to_string(),
                    change_type: if baseline.expected_result == CompilationResult::Success {
                        BehavioralChangeType::CompilationFailure
                    } else {
                        BehavioralChangeType::CompilationSuccess
                    },
                    severity: RegressionSeverity::Major,
                    description: format!("Compilation result changed from {:?} to {:?}", 
                        baseline.expected_result, current_baseline.expected_result),
                    evidence: RegressionEvidence {
                        baseline_signature: format!("{:?}", baseline.expected_result),
                        current_signature: format!("{:?}", current_baseline.expected_result),
                        change_description: "Compilation result differs from baseline".to_string(),
                        supporting_data: HashMap::new(),
                    },
                }));
            }
            
            // Compare code hashes if both succeeded
            if baseline.expected_result == CompilationResult::Success && 
               current_baseline.expected_result == CompilationResult::Success {
                if baseline.code_hash != current_baseline.code_hash {
                    return Ok(Some(BehavioralRegression {
                        test_case: test_case.to_string(),
                        component: "CodeGen".to_string(),
                        change_type: BehavioralChangeType::OutputChange,
                        severity: RegressionSeverity::Minor,
                        description: "Generated code hash changed".to_string(),
                        evidence: RegressionEvidence {
                            baseline_signature: baseline.code_hash.clone().unwrap_or_default(),
                            current_signature: current_baseline.code_hash.clone().unwrap_or_default(),
                            change_description: "Code generation output differs".to_string(),
                            supporting_data: HashMap::new(),
                        },
                    }));
                }
            }
        }
        
        Ok(None)
    }

    /// Detect performance regression for a test case
    fn detect_performance_regression(&self, test_case: &str) -> OvieResult<Option<PerformanceRegression>> {
        if let Some(baseline) = self.baseline_data.performance_baselines.get(test_case) {
            let current_baseline = self.create_performance_baseline(test_case)?;
            
            // Check end-to-end compilation time
            let baseline_time = baseline.end_to_end_time.as_secs_f64();
            let current_time = current_baseline.end_to_end_time.as_secs_f64();
            let percentage_change = ((current_time - baseline_time) / baseline_time) * 100.0;
            
            if percentage_change > self.config.performance_threshold {
                return Ok(Some(PerformanceRegression {
                    benchmark_name: test_case.to_string(),
                    metric: PerformanceMetric::CompilationTime,
                    baseline_value: baseline_time,
                    current_value: current_time,
                    percentage_change,
                    severity: if percentage_change > 50.0 {
                        RegressionSeverity::Critical
                    } else if percentage_change > 20.0 {
                        RegressionSeverity::Major
                    } else {
                        RegressionSeverity::Minor
                    },
                }));
            }
        }
        
        Ok(None)
    }

    /// Detect consistency regression for a test case
    fn detect_consistency_regression(&mut self, test_case: &str) -> OvieResult<Option<ConsistencyRegression>> {
        // Clone baseline data to avoid borrow conflicts
        let baseline = self.baseline_data.cross_platform_baselines.get(test_case).cloned();
        
        if let Some(baseline) = baseline {
            let current_result = self.validate_single_test_cross_platform(test_case)?;
            
            let consistency_change = current_result.consistency_analysis.consistency_percentage - 
                baseline.consistency_percentage;
            
            if consistency_change < -self.config.cross_platform_requirements.minimum_consistency_percentage {
                return Ok(Some(ConsistencyRegression {
                    test_case: test_case.to_string(),
                    affected_platforms: current_result.inconsistencies.iter()
                        .flat_map(|i| i.platforms.clone())
                        .collect(),
                    baseline_consistency: baseline.consistency_percentage,
                    current_consistency: current_result.consistency_analysis.consistency_percentage,
                    consistency_change,
                    new_inconsistencies: current_result.inconsistencies,
                }));
            }
        }
        
        Ok(None)
    }

    /// Assess overall regression risk
    fn assess_regression_risk(
        &self,
        behavioral_regressions: &[BehavioralRegression],
        performance_regressions: &[PerformanceRegression],
        consistency_regressions: &[ConsistencyRegression],
    ) -> RegressionRiskAssessment {
        let mut risk_factors = Vec::new();
        
        // Assess behavioral regression risk
        let critical_behavioral = behavioral_regressions.iter()
            .filter(|r| r.severity == RegressionSeverity::Critical)
            .count();
        
        if critical_behavioral > 0 {
            risk_factors.push(RiskFactor {
                description: format!("{} critical behavioral regressions", critical_behavioral),
                impact: ImpactLevel::Severe,
                likelihood: 0.9,
                affected_components: behavioral_regressions.iter()
                    .map(|r| r.component.clone())
                    .collect(),
            });
        }
        
        // Assess performance regression risk
        let critical_performance = performance_regressions.iter()
            .filter(|r| r.severity == RegressionSeverity::Critical)
            .count();
        
        if critical_performance > 0 {
            risk_factors.push(RiskFactor {
                description: format!("{} critical performance regressions", critical_performance),
                impact: ImpactLevel::Major,
                likelihood: 0.8,
                affected_components: vec!["Performance".to_string()],
            });
        }
        
        // Assess consistency regression risk
        if !consistency_regressions.is_empty() {
            risk_factors.push(RiskFactor {
                description: format!("{} cross-platform consistency regressions", consistency_regressions.len()),
                impact: ImpactLevel::Moderate,
                likelihood: 0.7,
                affected_components: vec!["CrossPlatform".to_string()],
            });
        }
        
        // Determine overall risk level
        let risk_level = if critical_behavioral > 0 {
            RiskLevel::Critical
        } else if critical_performance > 0 || behavioral_regressions.len() > 5 {
            RiskLevel::High
        } else if !performance_regressions.is_empty() || !consistency_regressions.is_empty() {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        };
        
        // Determine recommended response
        let recommended_response = match risk_level {
            RiskLevel::Critical => ResponseLevel::Block,
            RiskLevel::High => ResponseLevel::Fix,
            RiskLevel::Medium => ResponseLevel::Investigate,
            RiskLevel::Low => ResponseLevel::Monitor,
        };
        
        RegressionRiskAssessment {
            risk_level,
            risk_factors,
            confidence: 0.85, // High confidence in assessment
            recommended_response,
        }
    }

    /// Generate recommended actions for addressing regressions
    fn generate_recommended_actions(
        &self,
        behavioral_regressions: &[BehavioralRegression],
        performance_regressions: &[PerformanceRegression],
        consistency_regressions: &[ConsistencyRegression],
        risk_assessment: &RegressionRiskAssessment,
    ) -> Vec<RecommendedAction> {
        let mut actions = Vec::new();
        
        // Actions for behavioral regressions
        for regression in behavioral_regressions {
            if regression.severity == RegressionSeverity::Critical {
                actions.push(RecommendedAction {
                    description: format!("Fix critical behavioral regression in {}", regression.component),
                    priority: ActionPriority::Critical,
                    estimated_effort: EffortLevel::Medium,
                    target_components: vec![regression.component.clone()],
                    success_criteria: vec![
                        "Regression test passes".to_string(),
                        "Baseline behavior restored".to_string(),
                    ],
                });
            }
        }
        
        // Actions for performance regressions
        for regression in performance_regressions {
            if regression.severity == RegressionSeverity::Critical {
                actions.push(RecommendedAction {
                    description: format!("Optimize {} performance", regression.benchmark_name),
                    priority: ActionPriority::High,
                    estimated_effort: EffortLevel::Large,
                    target_components: vec!["Performance".to_string()],
                    success_criteria: vec![
                        format!("Performance within {}% of baseline", self.config.performance_threshold),
                    ],
                });
            }
        }
        
        // Actions for consistency regressions
        if !consistency_regressions.is_empty() {
            actions.push(RecommendedAction {
                description: "Investigate cross-platform consistency issues".to_string(),
                priority: ActionPriority::Medium,
                estimated_effort: EffortLevel::Medium,
                target_components: vec!["CrossPlatform".to_string()],
                success_criteria: vec![
                    format!("Consistency above {}%", self.config.cross_platform_requirements.minimum_consistency_percentage),
                ],
            });
        }
        
        // General monitoring action
        if risk_assessment.risk_level == RiskLevel::Low {
            actions.push(RecommendedAction {
                description: "Continue monitoring for regressions".to_string(),
                priority: ActionPriority::Low,
                estimated_effort: EffortLevel::Trivial,
                target_components: vec!["All".to_string()],
                success_criteria: vec![
                    "No new critical regressions detected".to_string(),
                ],
            });
        }
        
        actions
    }

    /// Compute hash for data
    fn compute_hash(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}

impl Default for RegressionConfig {
    fn default() -> Self {
        Self {
            performance_threshold: 10.0, // 10% performance regression threshold
            behavioral_tolerance: BehavioralTolerance {
                allow_ast_structure_changes: false,
                allow_error_message_improvements: true,
                allow_performance_optimizations: true,
                require_exact_output_match: false,
            },
            cross_platform_requirements: CrossPlatformRequirements {
                required_platforms: vec![
                    "wasm32-unknown-unknown".to_string(),
                    "x86_64-pc-windows-gnu".to_string(),
                    "interpreter".to_string(),
                ],
                minimum_consistency_percentage: 95.0,
                allow_platform_optimizations: true,
                require_identical_errors: false,
            },
            test_selection: TestSelectionStrategy::Comprehensive,
        }
    }
}

impl BaselineData {
    /// Create new empty baseline data
    pub fn new() -> Self {
        Self {
            compiler_version: env!("CARGO_PKG_VERSION").to_string(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            compilation_baselines: HashMap::new(),
            performance_baselines: HashMap::new(),
            cross_platform_baselines: HashMap::new(),
        }
    }
}

impl Default for RegressionDetector {
    fn default() -> Self {
        Self::new()
    }
}
