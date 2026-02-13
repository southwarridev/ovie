//! Security and Network Isolation for Ovie Package Management
//! 
//! This module implements security features including network call monitoring,
//! cryptographic verification, and supply chain isolation.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use crate::{OvieResult, OvieError, AstNode, Statement, Expression};

/// Unsafe operation types that require explicit handling
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnsafeOperation {
    /// Raw memory access
    RawMemoryAccess,
    /// Foreign function interface calls
    ForeignFunctionCall,
    /// Pointer arithmetic
    PointerArithmetic,
    /// Unsafe type casting
    UnsafeCast,
    /// Direct system calls
    SystemCall,
    /// Network operations without verification
    UnverifiedNetworkAccess,
    /// File system operations outside sandbox
    UnsafeFileAccess,
}

/// Unsafe operation audit entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsafeAuditEntry {
    /// Timestamp of the operation
    pub timestamp: u64,
    /// Type of unsafe operation
    pub operation: UnsafeOperation,
    /// Source location (file, line, column)
    pub location: SourceLocation,
    /// Description of the operation
    pub description: String,
    /// Whether the operation was explicitly marked as unsafe
    pub explicitly_unsafe: bool,
    /// Justification provided by the developer
    pub justification: Option<String>,
}

/// Source code location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    pub file: String,
    pub line: u32,
    pub column: u32,
}

/// Unsafe operation analyzer and auditor
#[derive(Debug)]
pub struct UnsafeOperationAnalyzer {
    /// Audit log of unsafe operations
    audit_log: Arc<Mutex<Vec<UnsafeAuditEntry>>>,
    /// Whether unsafe operations are allowed
    unsafe_allowed: Arc<Mutex<bool>>,
    /// Require explicit unsafe blocks
    require_explicit_unsafe: Arc<Mutex<bool>>,
}

impl UnsafeOperationAnalyzer {
    /// Create a new unsafe operation analyzer
    pub fn new() -> Self {
        Self {
            audit_log: Arc::new(Mutex::new(Vec::new())),
            unsafe_allowed: Arc::new(Mutex::new(false)),
            require_explicit_unsafe: Arc::new(Mutex::new(true)),
        }
    }

    /// Enable unsafe operations (with explicit blocks)
    pub fn allow_unsafe_operations(&self) {
        *self.unsafe_allowed.lock().unwrap() = true;
    }

    /// Disable unsafe operations
    pub fn disallow_unsafe_operations(&self) {
        *self.unsafe_allowed.lock().unwrap() = false;
    }

    /// Check if unsafe operations are allowed
    pub fn are_unsafe_operations_allowed(&self) -> bool {
        *self.unsafe_allowed.lock().unwrap()
    }

    /// Set whether explicit unsafe blocks are required
    pub fn set_require_explicit_unsafe(&self, required: bool) {
        *self.require_explicit_unsafe.lock().unwrap() = required;
    }

    /// Analyze AST for unsafe operations
    pub fn analyze_ast(&self, ast: &AstNode, file_name: &str) -> OvieResult<Vec<UnsafeAuditEntry>> {
        let mut unsafe_operations = Vec::new();
        self.analyze_node(ast, file_name, &mut unsafe_operations)?;
        
        // Add to audit log
        {
            let mut audit_log = self.audit_log.lock().unwrap();
            audit_log.extend(unsafe_operations.clone());
        }
        
        Ok(unsafe_operations)
    }

    /// Analyze a single AST node for unsafe operations
    fn analyze_node(&self, node: &AstNode, file_name: &str, unsafe_ops: &mut Vec<UnsafeAuditEntry>) -> OvieResult<()> {
        // Analyze all statements in the AST
        match node {
            AstNode::Program(statements) => {
                for stmt in statements {
                    self.analyze_statement(stmt, file_name, unsafe_ops)?;
                }
            }
        }
        Ok(())
    }

    /// Analyze a statement for unsafe operations
    fn analyze_statement(&self, stmt: &Statement, file_name: &str, unsafe_ops: &mut Vec<UnsafeAuditEntry>) -> OvieResult<()> {
        match stmt {
            Statement::Assignment { mutable: _, identifier: _, value } => {
                self.analyze_expression(value, file_name, unsafe_ops)?;
            }
            Statement::VariableDeclaration { mutable: _, identifier: _, value } => {
                self.analyze_expression(value, file_name, unsafe_ops)?;
            }
            Statement::Print { expression } => {
                self.analyze_expression(expression, file_name, unsafe_ops)?;
            }
            Statement::If { condition, then_block, else_block } => {
                self.analyze_expression(condition, file_name, unsafe_ops)?;
                for stmt in then_block {
                    self.analyze_statement(stmt, file_name, unsafe_ops)?;
                }
                if let Some(else_stmts) = else_block {
                    for stmt in else_stmts {
                        self.analyze_statement(stmt, file_name, unsafe_ops)?;
                    }
                }
            }
            Statement::While { condition, body } => {
                self.analyze_expression(condition, file_name, unsafe_ops)?;
                for stmt in body {
                    self.analyze_statement(stmt, file_name, unsafe_ops)?;
                }
            }
            Statement::Function { name: _, parameters: _, body } => {
                for stmt in body {
                    self.analyze_statement(stmt, file_name, unsafe_ops)?;
                }
            }
            Statement::FunctionDeclaration { name: _, parameters: _, body } => {
                for stmt in body {
                    self.analyze_statement(stmt, file_name, unsafe_ops)?;
                }
            }
            Statement::For { identifier: _, iterable, body } => {
                self.analyze_expression(iterable, file_name, unsafe_ops)?;
                for stmt in body {
                    self.analyze_statement(stmt, file_name, unsafe_ops)?;
                }
            }
            Statement::Return { value } => {
                if let Some(expr) = value {
                    self.analyze_expression(expr, file_name, unsafe_ops)?;
                }
            }
            Statement::Expression { expression } => {
                self.analyze_expression(expression, file_name, unsafe_ops)?;
            }
            Statement::Struct { name: _, fields: _ } => {
                // Struct definitions are safe
            }
            Statement::Enum { name: _, variants: _ } => {
                // Enum definitions are safe
            }
        }
        Ok(())
    }

    /// Analyze an expression for unsafe operations
    fn analyze_expression(&self, expr: &Expression, file_name: &str, unsafe_ops: &mut Vec<UnsafeAuditEntry>) -> OvieResult<()> {
        match expr {
            Expression::Call { function, arguments } => {
                if self.is_unsafe_function(function) {
                    let entry = UnsafeAuditEntry {
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                        operation: self.classify_unsafe_function(function),
                        location: SourceLocation {
                            file: file_name.to_string(),
                            line: 1,
                            column: 1,
                        },
                        description: format!("Call to potentially unsafe function: {}", function),
                        explicitly_unsafe: false,
                        justification: None,
                    };
                    unsafe_ops.push(entry);
                }

                for arg in arguments {
                    self.analyze_expression(arg, file_name, unsafe_ops)?;
                }
            }
            Expression::Binary { left, right, .. } => {
                self.analyze_expression(left, file_name, unsafe_ops)?;
                self.analyze_expression(right, file_name, unsafe_ops)?;
            }
            Expression::Unary { operand, .. } => {
                self.analyze_expression(operand, file_name, unsafe_ops)?;
            }
            Expression::FieldAccess { object, field: _ } => {
                self.analyze_expression(object, file_name, unsafe_ops)?;
            }
            Expression::StructInstantiation { struct_name: _, fields } => {
                for field in fields {
                    self.analyze_expression(&field.value, file_name, unsafe_ops)?;
                }
            }
            Expression::Range { start, end } => {
                self.analyze_expression(start, file_name, unsafe_ops)?;
                self.analyze_expression(end, file_name, unsafe_ops)?;
            }
            _ => {} // Other expressions are safe
        }
        Ok(())
    }

    /// Check if a function name represents an unsafe operation
    fn is_unsafe_function(&self, name: &str) -> bool {
        matches!(name, 
            "malloc" | "free" | "memcpy" | "memset" |
            "system" | "exec" | "fork" |
            "ptr_read" | "ptr_write" | "cast_ptr" |
            "raw_socket" | "bind_socket" |
            "file_raw_read" | "file_raw_write"
        )
    }

    /// Classify the type of unsafe operation
    fn classify_unsafe_function(&self, name: &str) -> UnsafeOperation {
        match name {
            "malloc" | "free" | "memcpy" | "memset" => UnsafeOperation::RawMemoryAccess,
            "system" | "exec" | "fork" => UnsafeOperation::SystemCall,
            "ptr_read" | "ptr_write" => UnsafeOperation::PointerArithmetic,
            "cast_ptr" => UnsafeOperation::UnsafeCast,
            "raw_socket" | "bind_socket" => UnsafeOperation::UnverifiedNetworkAccess,
            "file_raw_read" | "file_raw_write" => UnsafeOperation::UnsafeFileAccess,
            _ => UnsafeOperation::ForeignFunctionCall,
        }
    }

    /// Get the complete audit log
    pub fn get_audit_log(&self) -> Vec<UnsafeAuditEntry> {
        self.audit_log.lock().unwrap().clone()
    }

    /// Clear the audit log
    pub fn clear_audit_log(&self) {
        self.audit_log.lock().unwrap().clear();
    }

    /// Generate a security report
    pub fn generate_security_report(&self) -> SecurityReport {
        let audit_log = self.audit_log.lock().unwrap();
        let total_unsafe_operations = audit_log.len();
        let explicit_unsafe_count = audit_log.iter().filter(|entry| entry.explicitly_unsafe).count();
        let implicit_unsafe_count = total_unsafe_operations - explicit_unsafe_count;

        let mut operation_counts = HashMap::new();
        for entry in audit_log.iter() {
            *operation_counts.entry(entry.operation.clone()).or_insert(0) += 1;
        }

        SecurityReport {
            total_unsafe_operations,
            explicit_unsafe_count,
            implicit_unsafe_count,
            operation_counts,
            recommendations: self.generate_recommendations(&audit_log),
        }
    }

    /// Generate security recommendations
    fn generate_recommendations(&self, audit_log: &[UnsafeAuditEntry]) -> Vec<String> {
        let mut recommendations = Vec::new();

        let implicit_count = audit_log.iter().filter(|entry| !entry.explicitly_unsafe).count();
        if implicit_count > 0 {
            recommendations.push(format!(
                "Found {} implicit unsafe operations. Consider wrapping them in explicit unsafe blocks.",
                implicit_count
            ));
        }

        let memory_ops = audit_log.iter().filter(|entry| 
            matches!(entry.operation, UnsafeOperation::RawMemoryAccess | UnsafeOperation::PointerArithmetic)
        ).count();
        if memory_ops > 0 {
            recommendations.push(
                "Memory operations detected. Ensure proper bounds checking and memory safety.".to_string()
            );
        }

        let network_ops = audit_log.iter().filter(|entry| 
            matches!(entry.operation, UnsafeOperation::UnverifiedNetworkAccess)
        ).count();
        if network_ops > 0 {
            recommendations.push(
                "Unverified network operations detected. Consider using verified network APIs.".to_string()
            );
        }

        if recommendations.is_empty() {
            recommendations.push("No security issues detected.".to_string());
        }

        recommendations
    }
}

/// Security report generated by the unsafe operation analyzer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityReport {
    /// Total number of unsafe operations
    pub total_unsafe_operations: usize,
    /// Number of explicitly marked unsafe operations
    pub explicit_unsafe_count: usize,
    /// Number of implicitly unsafe operations
    pub implicit_unsafe_count: usize,
    /// Count of each type of unsafe operation
    pub operation_counts: HashMap<UnsafeOperation, usize>,
    /// Security recommendations
    pub recommendations: Vec<String>,
}

/// Network activity monitor
#[derive(Debug, Clone)]
pub struct NetworkMonitor {
    /// Allowed network operations
    allowed_operations: Arc<Mutex<Vec<AllowedOperation>>>,
    /// Recorded network calls
    network_calls: Arc<Mutex<Vec<NetworkCall>>>,
    /// Whether network access is currently allowed
    network_enabled: Arc<Mutex<bool>>,
}

/// Allowed network operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowedOperation {
    /// Operation type (download, upload, etc.)
    pub operation_type: String,
    /// Target URL pattern
    pub url_pattern: String,
    /// Maximum allowed size in bytes
    pub max_size: Option<u64>,
    /// Expiration timestamp
    pub expires_at: Option<u64>,
}

/// Recorded network call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkCall {
    /// Timestamp of the call
    pub timestamp: u64,
    /// Operation type
    pub operation_type: String,
    /// Target URL
    pub url: String,
    /// Data size in bytes
    pub size: u64,
    /// Whether the call was allowed
    pub allowed: bool,
    /// Reason for denial (if denied)
    pub denial_reason: Option<String>,
}

impl NetworkMonitor {
    /// Create a new network monitor
    pub fn new() -> Self {
        Self {
            allowed_operations: Arc::new(Mutex::new(Vec::new())),
            network_calls: Arc::new(Mutex::new(Vec::new())),
            network_enabled: Arc::new(Mutex::new(false)),
        }
    }

    /// Enable network access
    pub fn enable_network(&self) {
        *self.network_enabled.lock().unwrap() = true;
    }

    /// Disable network access
    pub fn disable_network(&self) {
        *self.network_enabled.lock().unwrap() = false;
    }

    /// Check if network access is enabled
    pub fn is_network_enabled(&self) -> bool {
        *self.network_enabled.lock().unwrap()
    }

    /// Add an allowed network operation
    pub fn allow_operation(&self, operation: AllowedOperation) {
        self.allowed_operations.lock().unwrap().push(operation);
    }

    /// Check if a network operation is allowed
    pub fn is_operation_allowed(&self, operation_type: &str, url: &str, size: u64) -> OvieResult<bool> {
        if !self.is_network_enabled() {
            return Ok(false);
        }

        let allowed_ops = self.allowed_operations.lock().unwrap();
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| OvieError::generic(format!("Failed to get current time: {}", e)))?
            .as_secs();

        for allowed_op in allowed_ops.iter() {
            // Check if operation type matches
            if allowed_op.operation_type != operation_type {
                continue;
            }

            // Check if URL matches pattern (simplified pattern matching)
            if !self.url_matches_pattern(url, &allowed_op.url_pattern) {
                continue;
            }

            // Check size limit
            if let Some(max_size) = allowed_op.max_size {
                if size > max_size {
                    continue;
                }
            }

            // Check expiration
            if let Some(expires_at) = allowed_op.expires_at {
                if current_time > expires_at {
                    continue;
                }
            }

            return Ok(true);
        }

        Ok(false)
    }

    /// Record a network call
    pub fn record_network_call(&self, operation_type: &str, url: &str, size: u64, allowed: bool, denial_reason: Option<String>) -> OvieResult<()> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| OvieError::generic(format!("Failed to get current time: {}", e)))?
            .as_secs();

        let call = NetworkCall {
            timestamp,
            operation_type: operation_type.to_string(),
            url: url.to_string(),
            size,
            allowed,
            denial_reason,
        };

        self.network_calls.lock().unwrap().push(call);
        Ok(())
    }

    /// Get all recorded network calls
    pub fn get_network_calls(&self) -> Vec<NetworkCall> {
        self.network_calls.lock().unwrap().clone()
    }

    /// Clear all recorded network calls
    pub fn clear_network_calls(&self) {
        self.network_calls.lock().unwrap().clear();
    }

    /// Simple URL pattern matching (supports wildcards)
    fn url_matches_pattern(&self, url: &str, pattern: &str) -> bool {
        if pattern == "*" {
            return true;
        }

        // Simple wildcard matching
        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.len() == 2 {
                let prefix = parts[0];
                let suffix = parts[1];
                return url.starts_with(prefix) && url.ends_with(suffix);
            }
        }

        url == pattern
    }
}

/// Cryptographic verifier for packages and downloads
#[derive(Debug)]
pub struct CryptographicVerifier {
    /// Trusted public keys
    trusted_keys: HashMap<String, PublicKey>,
    /// Verification cache
    verification_cache: HashMap<String, VerificationResult>,
}

/// Public key for signature verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicKey {
    /// Key identifier
    pub key_id: String,
    /// Key algorithm (e.g., "ed25519", "rsa")
    pub algorithm: String,
    /// Key data (base64 encoded)
    pub key_data: String,
}

/// Verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    /// Whether verification succeeded
    pub verified: bool,
    /// Key used for verification
    pub key_id: Option<String>,
    /// Verification timestamp
    pub timestamp: u64,
    /// Error message (if verification failed)
    pub error: Option<String>,
}

impl CryptographicVerifier {
    /// Create a new cryptographic verifier
    pub fn new() -> Self {
        Self {
            trusted_keys: HashMap::new(),
            verification_cache: HashMap::new(),
        }
    }

    /// Add a trusted public key
    pub fn add_trusted_key(&mut self, key: PublicKey) {
        self.trusted_keys.insert(key.key_id.clone(), key);
    }

    /// Verify content hash
    pub fn verify_content_hash(&self, content: &[u8], expected_hash: &str) -> OvieResult<bool> {
        let mut hasher = Sha256::new();
        hasher.update(content);
        let computed_hash = format!("{:x}", hasher.finalize());
        
        Ok(computed_hash == expected_hash)
    }

    /// Verify digital signature (placeholder implementation)
    pub fn verify_signature(&mut self, content: &[u8], signature: &str, key_id: &str) -> OvieResult<VerificationResult> {
        let cache_key = format!("{}:{}", key_id, signature);
        
        // Check cache first
        if let Some(cached_result) = self.verification_cache.get(&cache_key) {
            return Ok(cached_result.clone());
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| OvieError::generic(format!("Failed to get current time: {}", e)))?
            .as_secs();

        // Check if we have the trusted key
        if !self.trusted_keys.contains_key(key_id) {
            let result = VerificationResult {
                verified: false,
                key_id: Some(key_id.to_string()),
                timestamp,
                error: Some("Trusted key not found".to_string()),
            };
            self.verification_cache.insert(cache_key, result.clone());
            return Ok(result);
        }

        // TODO: Implement actual signature verification
        // For now, this is a placeholder that always succeeds for demonstration
        let result = VerificationResult {
            verified: true,
            key_id: Some(key_id.to_string()),
            timestamp,
            error: None,
        };

        self.verification_cache.insert(cache_key, result.clone());
        Ok(result)
    }

    /// Verify package integrity
    pub fn verify_package_integrity(&mut self, content: &[u8], expected_hash: &str, signature: Option<&str>, key_id: Option<&str>) -> OvieResult<bool> {
        // First verify content hash
        if !self.verify_content_hash(content, expected_hash)? {
            return Ok(false);
        }

        // If signature is provided, verify it
        if let (Some(sig), Some(kid)) = (signature, key_id) {
            let sig_result = self.verify_signature(content, sig, kid)?;
            return Ok(sig_result.verified);
        }

        // Hash verification passed and no signature required
        Ok(true)
    }
}

/// Supply chain security manager
#[derive(Debug)]
pub struct SupplyChainSecurity {
    /// Network monitor
    network_monitor: NetworkMonitor,
    /// Cryptographic verifier
    verifier: CryptographicVerifier,
    /// Security policies
    policies: SecurityPolicies,
    /// Telemetry prevention system
    telemetry_monitor: TelemetryMonitor,
}

/// Telemetry prevention and privacy compliance system
#[derive(Debug, Clone)]
pub struct TelemetryMonitor {
    /// Whether telemetry collection is explicitly disabled
    telemetry_disabled: Arc<Mutex<bool>>,
    /// Blocked telemetry endpoints
    blocked_endpoints: Arc<Mutex<Vec<String>>>,
    /// Detected telemetry attempts
    telemetry_attempts: Arc<Mutex<Vec<TelemetryAttempt>>>,
    /// Privacy compliance settings
    privacy_settings: Arc<Mutex<PrivacySettings>>,
}

/// Detected telemetry attempt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryAttempt {
    /// Timestamp of the attempt
    pub timestamp: u64,
    /// Target endpoint
    pub endpoint: String,
    /// Data being sent (if detectable)
    pub data_type: String,
    /// Whether the attempt was blocked
    pub blocked: bool,
    /// Source of the telemetry attempt
    pub source: String,
}

/// Privacy compliance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    /// Disable all data collection
    pub disable_all_telemetry: bool,
    /// Disable usage analytics
    pub disable_usage_analytics: bool,
    /// Disable error reporting
    pub disable_error_reporting: bool,
    /// Disable performance metrics
    pub disable_performance_metrics: bool,
    /// Require explicit consent for any data collection
    pub require_explicit_consent: bool,
    /// Log all network activity for transparency
    pub log_network_activity: bool,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            disable_all_telemetry: true, // Privacy-first by default
            disable_usage_analytics: true,
            disable_error_reporting: true,
            disable_performance_metrics: true,
            require_explicit_consent: true,
            log_network_activity: true,
        }
    }
}

impl TelemetryMonitor {
    /// Create a new telemetry monitor with privacy-first defaults
    pub fn new() -> Self {
        Self {
            telemetry_disabled: Arc::new(Mutex::new(true)), // Disabled by default
            blocked_endpoints: Arc::new(Mutex::new(vec![
                // Common telemetry endpoints to block
                "telemetry.microsoft.com".to_string(),
                "analytics.google.com".to_string(),
                "api.mixpanel.com".to_string(),
                "api.segment.io".to_string(),
                "collect.tealiumiq.com".to_string(),
                "stats.wp.com".to_string(),
                "api.amplitude.com".to_string(),
                "api.bugsnag.com".to_string(),
                "api.rollbar.com".to_string(),
                "sentry.io".to_string(),
            ])),
            telemetry_attempts: Arc::new(Mutex::new(Vec::new())),
            privacy_settings: Arc::new(Mutex::new(PrivacySettings::default())),
        }
    }

    /// Enable telemetry (requires explicit user consent)
    pub fn enable_telemetry_with_consent(&self, user_consent: bool) -> OvieResult<()> {
        if !user_consent {
            return Err(OvieError::generic("Telemetry cannot be enabled without explicit user consent"));
        }
        
        *self.telemetry_disabled.lock().unwrap() = false;
        Ok(())
    }

    /// Disable all telemetry
    pub fn disable_telemetry(&self) {
        *self.telemetry_disabled.lock().unwrap() = true;
    }

    /// Check if telemetry is disabled
    pub fn is_telemetry_disabled(&self) -> bool {
        *self.telemetry_disabled.lock().unwrap()
    }

    /// Add a blocked telemetry endpoint
    pub fn add_blocked_endpoint(&self, endpoint: String) {
        self.blocked_endpoints.lock().unwrap().push(endpoint);
    }

    /// Check if an endpoint is blocked for telemetry
    pub fn is_endpoint_blocked(&self, url: &str) -> bool {
        let blocked_endpoints = self.blocked_endpoints.lock().unwrap();
        
        // Check exact matches and domain matches
        for blocked in blocked_endpoints.iter() {
            if url.contains(blocked) {
                return true;
            }
        }
        
        false
    }

    /// Record a telemetry attempt
    pub fn record_telemetry_attempt(&self, endpoint: &str, data_type: &str, source: &str) -> OvieResult<bool> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| OvieError::generic(format!("Failed to get current time: {}", e)))?
            .as_secs();

        let blocked = self.is_telemetry_disabled() || self.is_endpoint_blocked(endpoint);

        let attempt = TelemetryAttempt {
            timestamp,
            endpoint: endpoint.to_string(),
            data_type: data_type.to_string(),
            blocked,
            source: source.to_string(),
        };

        self.telemetry_attempts.lock().unwrap().push(attempt);
        Ok(blocked)
    }

    /// Get all recorded telemetry attempts
    pub fn get_telemetry_attempts(&self) -> Vec<TelemetryAttempt> {
        self.telemetry_attempts.lock().unwrap().clone()
    }

    /// Clear telemetry attempt log
    pub fn clear_telemetry_attempts(&self) {
        self.telemetry_attempts.lock().unwrap().clear();
    }

    /// Update privacy settings
    pub fn update_privacy_settings(&self, settings: PrivacySettings) {
        *self.privacy_settings.lock().unwrap() = settings;
    }

    /// Get current privacy settings
    pub fn get_privacy_settings(&self) -> PrivacySettings {
        self.privacy_settings.lock().unwrap().clone()
    }

    /// Generate privacy compliance report
    pub fn generate_privacy_report(&self) -> PrivacyComplianceReport {
        let attempts = self.telemetry_attempts.lock().unwrap();
        let blocked_attempts = attempts.iter().filter(|a| a.blocked).count();
        let allowed_attempts = attempts.len() - blocked_attempts;
        
        let mut endpoint_attempts = HashMap::new();
        for attempt in attempts.iter() {
            *endpoint_attempts.entry(attempt.endpoint.clone()).or_insert(0) += 1;
        }

        PrivacyComplianceReport {
            total_telemetry_attempts: attempts.len(),
            blocked_telemetry_attempts: blocked_attempts,
            allowed_telemetry_attempts: allowed_attempts,
            telemetry_disabled: self.is_telemetry_disabled(),
            privacy_settings: self.get_privacy_settings(),
            endpoint_attempts,
            compliance_status: if allowed_attempts == 0 {
                "COMPLIANT".to_string()
            } else {
                "NON_COMPLIANT".to_string()
            },
        }
    }

    /// Verify no hidden data collection is occurring
    pub fn verify_no_hidden_collection(&self) -> bool {
        let attempts = self.telemetry_attempts.lock().unwrap();
        let hidden_attempts = attempts.iter().filter(|a| !a.blocked && a.source == "hidden").count();
        hidden_attempts == 0
    }
}

/// Privacy compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyComplianceReport {
    /// Total number of telemetry attempts
    pub total_telemetry_attempts: usize,
    /// Number of blocked telemetry attempts
    pub blocked_telemetry_attempts: usize,
    /// Number of allowed telemetry attempts
    pub allowed_telemetry_attempts: usize,
    /// Whether telemetry is disabled
    pub telemetry_disabled: bool,
    /// Current privacy settings
    pub privacy_settings: PrivacySettings,
    /// Attempts per endpoint
    pub endpoint_attempts: HashMap<String, usize>,
    /// Overall compliance status
    pub compliance_status: String,
}

/// Security policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicies {
    /// Require signature verification for all packages
    pub require_signatures: bool,
    /// Allow network access during builds
    pub allow_network_during_build: bool,
    /// Maximum package size in bytes
    pub max_package_size: u64,
    /// Allowed package sources (URL patterns)
    pub allowed_sources: Vec<String>,
    /// Blocked package sources (URL patterns)
    pub blocked_sources: Vec<String>,
}

impl Default for SecurityPolicies {
    fn default() -> Self {
        Self {
            require_signatures: false, // Start permissive for development
            allow_network_during_build: false, // Strict by default
            max_package_size: 100 * 1024 * 1024, // 100MB default limit
            allowed_sources: vec!["*".to_string()], // Allow all sources initially
            blocked_sources: Vec::new(),
        }
    }
}

impl SupplyChainSecurity {
    /// Create a new supply chain security manager
    pub fn new() -> Self {
        Self {
            network_monitor: NetworkMonitor::new(),
            verifier: CryptographicVerifier::new(),
            policies: SecurityPolicies::default(),
            telemetry_monitor: TelemetryMonitor::new(),
        }
    }

    /// Create with custom policies
    pub fn with_policies(policies: SecurityPolicies) -> Self {
        Self {
            network_monitor: NetworkMonitor::new(),
            verifier: CryptographicVerifier::new(),
            policies,
            telemetry_monitor: TelemetryMonitor::new(),
        }
    }

    /// Get network monitor
    pub fn network_monitor(&self) -> &NetworkMonitor {
        &self.network_monitor
    }

    /// Get cryptographic verifier
    pub fn verifier(&mut self) -> &mut CryptographicVerifier {
        &mut self.verifier
    }

    /// Get telemetry monitor
    pub fn telemetry_monitor(&self) -> &TelemetryMonitor {
        &self.telemetry_monitor
    }

    /// Get security policies
    pub fn policies(&self) -> &SecurityPolicies {
        &self.policies
    }

    /// Update security policies
    pub fn update_policies(&mut self, policies: SecurityPolicies) {
        self.policies = policies;
    }

    /// Check if a package source is allowed
    pub fn is_source_allowed(&self, url: &str) -> bool {
        // Check blocked sources first
        for blocked_pattern in &self.policies.blocked_sources {
            if self.network_monitor.url_matches_pattern(url, blocked_pattern) {
                return false;
            }
        }

        // Check allowed sources
        for allowed_pattern in &self.policies.allowed_sources {
            if self.network_monitor.url_matches_pattern(url, allowed_pattern) {
                return true;
            }
        }

        false
    }

    /// Validate package before installation
    pub fn validate_package(&mut self, content: &[u8], source_url: &str, expected_hash: &str, signature: Option<&str>, key_id: Option<&str>) -> OvieResult<bool> {
        // Check source URL
        if !self.is_source_allowed(source_url) {
            return Ok(false);
        }

        // Check package size
        if content.len() as u64 > self.policies.max_package_size {
            return Ok(false);
        }

        // Check signature requirement
        if self.policies.require_signatures && (signature.is_none() || key_id.is_none()) {
            return Ok(false);
        }

        // Verify package integrity
        self.verifier.verify_package_integrity(content, expected_hash, signature, key_id)
    }

    /// Monitor network call for telemetry
    pub fn monitor_network_call(&self, url: &str, data_type: &str, source: &str) -> OvieResult<bool> {
        // Record telemetry attempt
        let blocked = self.telemetry_monitor.record_telemetry_attempt(url, data_type, source)?;
        
        // Also record in network monitor
        let denial_reason = if blocked {
            Some("Telemetry blocked by privacy settings".to_string())
        } else {
            None
        };
        
        self.network_monitor.record_network_call("telemetry", url, 0, !blocked, denial_reason)?;
        
        Ok(!blocked) // Return true if allowed, false if blocked
    }

    /// Generate comprehensive security report
    pub fn generate_comprehensive_security_report(&self) -> ComprehensiveSecurityReport {
        let network_report = self.generate_security_report();
        let privacy_report = self.telemetry_monitor.generate_privacy_report();
        
        let overall_status = if privacy_report.compliance_status == "COMPLIANT" && network_report.unauthorized_network_calls == 0 {
            "SECURE".to_string()
        } else {
            "NEEDS_ATTENTION".to_string()
        };
        
        ComprehensiveSecurityReport {
            network_security: network_report,
            privacy_compliance: privacy_report,
            overall_security_status: overall_status,
        }
    }

    /// Generate security report
    pub fn generate_security_report(&self) -> NetworkSecurityReport {
        let network_calls = self.network_monitor.get_network_calls();
        let unauthorized_calls = network_calls.iter().filter(|call| !call.allowed).count();
        
        NetworkSecurityReport {
            total_network_calls: network_calls.len(),
            unauthorized_network_calls: unauthorized_calls,
            network_enabled: self.network_monitor.is_network_enabled(),
            policies: self.policies.clone(),
            network_calls,
        }
    }
}

/// Network security report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSecurityReport {
    /// Total number of network calls
    pub total_network_calls: usize,
    /// Number of unauthorized network calls
    pub unauthorized_network_calls: usize,
    /// Whether network access is enabled
    pub network_enabled: bool,
    /// Current security policies
    pub policies: SecurityPolicies,
    /// All network calls
    pub network_calls: Vec<NetworkCall>,
}

/// Comprehensive security report combining network and privacy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveSecurityReport {
    /// Network security report
    pub network_security: NetworkSecurityReport,
    /// Privacy compliance report
    pub privacy_compliance: PrivacyComplianceReport,
    /// Overall security status
    pub overall_security_status: String,
}

impl Default for CryptographicVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SupplyChainSecurity {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TelemetryMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_monitor_creation() {
        let monitor = NetworkMonitor::new();
        assert!(!monitor.is_network_enabled());
        assert_eq!(monitor.get_network_calls().len(), 0);
    }

    #[test]
    fn test_network_monitor_enable_disable() {
        let monitor = NetworkMonitor::new();
        assert!(!monitor.is_network_enabled());
        
        monitor.enable_network();
        assert!(monitor.is_network_enabled());
        
        monitor.disable_network();
        assert!(!monitor.is_network_enabled());
    }

    #[test]
    fn test_allowed_operation() {
        let monitor = NetworkMonitor::new();
        monitor.enable_network();
        
        let operation = AllowedOperation {
            operation_type: "download".to_string(),
            url_pattern: "https://example.com/*".to_string(),
            max_size: Some(1024),
            expires_at: None,
        };
        
        monitor.allow_operation(operation);
        
        let allowed = monitor.is_operation_allowed("download", "https://example.com/package.tar.gz", 512).unwrap();
        assert!(allowed);
        
        let not_allowed = monitor.is_operation_allowed("download", "https://malicious.com/package.tar.gz", 512).unwrap();
        assert!(!not_allowed);
    }

    #[test]
    fn test_cryptographic_verifier() {
        let mut verifier = CryptographicVerifier::new();
        
        let content = b"Hello, World!";
        let expected_hash = "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f";
        
        let result = verifier.verify_content_hash(content, expected_hash).unwrap();
        assert!(result);
        
        let wrong_hash = "0000000000000000000000000000000000000000000000000000000000000000";
        let result = verifier.verify_content_hash(content, wrong_hash).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_supply_chain_security() {
        let mut security = SupplyChainSecurity::new();
        
        // Test source validation
        assert!(security.is_source_allowed("https://example.com/package.tar.gz"));
        
        // Test package validation
        let content = b"Test package content";
        let mut hasher = Sha256::new();
        hasher.update(content);
        let hash = format!("{:x}", hasher.finalize());
        
        let valid = security.validate_package(
            content,
            "https://example.com/package.tar.gz",
            &hash,
            None,
            None,
        ).unwrap();
        assert!(valid);
    }

    #[test]
    fn test_security_policies() {
        let policies = SecurityPolicies::default();
        assert!(!policies.require_signatures);
        assert!(!policies.allow_network_during_build);
        assert_eq!(policies.max_package_size, 100 * 1024 * 1024);
    }

    #[test]
    fn test_url_pattern_matching() {
        let monitor = NetworkMonitor::new();
        
        assert!(monitor.url_matches_pattern("https://example.com/test", "*"));
        assert!(monitor.url_matches_pattern("https://example.com/test", "https://example.com/*"));
        assert!(monitor.url_matches_pattern("https://example.com/test.tar.gz", "*.tar.gz"));
        assert!(!monitor.url_matches_pattern("https://malicious.com/test", "https://example.com/*"));
    }

    #[test]
    fn test_telemetry_monitor_creation() {
        let monitor = TelemetryMonitor::new();
        assert!(monitor.is_telemetry_disabled());
        assert_eq!(monitor.get_telemetry_attempts().len(), 0);
    }

    #[test]
    fn test_telemetry_blocking() {
        let monitor = TelemetryMonitor::new();
        
        // Test blocking telemetry attempt
        let blocked = monitor.record_telemetry_attempt(
            "https://telemetry.microsoft.com/collect",
            "usage_data",
            "compiler"
        ).unwrap();
        assert!(blocked);
        
        // Test allowed endpoint (when telemetry is enabled)
        monitor.enable_telemetry_with_consent(true).unwrap();
        let allowed = monitor.record_telemetry_attempt(
            "https://allowed-endpoint.com/api",
            "error_report",
            "compiler"
        ).unwrap();
        assert!(!allowed); // Still blocked because endpoint is not in allowed list
    }

    #[test]
    fn test_privacy_settings() {
        let monitor = TelemetryMonitor::new();
        let settings = monitor.get_privacy_settings();
        
        assert!(settings.disable_all_telemetry);
        assert!(settings.disable_usage_analytics);
        assert!(settings.require_explicit_consent);
    }

    #[test]
    fn test_privacy_compliance_report() {
        let monitor = TelemetryMonitor::new();
        
        // Record some telemetry attempts
        monitor.record_telemetry_attempt("https://analytics.google.com", "usage", "test").unwrap();
        monitor.record_telemetry_attempt("https://api.mixpanel.com", "events", "test").unwrap();
        
        let report = monitor.generate_privacy_report();
        assert_eq!(report.total_telemetry_attempts, 2);
        assert_eq!(report.blocked_telemetry_attempts, 2);
        assert_eq!(report.allowed_telemetry_attempts, 0);
        assert_eq!(report.compliance_status, "COMPLIANT");
    }

    #[test]
    fn test_comprehensive_security_report() {
        let security = SupplyChainSecurity::new();
        
        // Test network call monitoring
        security.monitor_network_call("https://telemetry.microsoft.com", "usage", "compiler").unwrap();
        
        let report = security.generate_comprehensive_security_report();
        
        // The telemetry attempt should be blocked, but it still counts as a network call
        // So the overall status should be "NEEDS_ATTENTION" because there was a network call attempt
        assert_eq!(report.overall_security_status, "NEEDS_ATTENTION");
        assert_eq!(report.privacy_compliance.blocked_telemetry_attempts, 1);
        assert_eq!(report.network_security.unauthorized_network_calls, 1); // The blocked telemetry call
    }

    #[test]
    fn test_no_hidden_data_collection() {
        let monitor = TelemetryMonitor::new();
        
        // Record visible telemetry attempt
        monitor.record_telemetry_attempt("https://analytics.com", "usage", "compiler").unwrap();
        
        // Verify no hidden collection
        assert!(monitor.verify_no_hidden_collection());
    }
}

impl Default for NetworkMonitor {
    fn default() -> Self {
        Self::new()
    }
}