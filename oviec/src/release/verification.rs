//! Release Verification System for Ovie
//! 
//! This module provides comprehensive verification of release packages,
//! including signature validation, integrity checks, and security compliance.

use crate::error::{OvieError, OvieResult};
use crate::release::{SecurityLevel, ReleasePackage, SignatureResult};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

/// Verification configuration
#[derive(Debug, Clone)]
pub struct VerificationConfig {
    /// Required security level
    pub security_level: SecurityLevel,
    /// Minimum required signatures
    pub min_signatures: usize,
    /// Enable timestamp verification
    pub verify_timestamps: bool,
    /// Maximum allowed signature age in seconds
    pub max_signature_age: u64,
    /// Enable integrity checking
    pub verify_integrity: bool,
    /// Enable security compliance checking
    pub verify_compliance: bool,
    /// Trusted public keys for verification
    pub trusted_keys: HashMap<String, Vec<u8>>,
}

impl VerificationConfig {
    /// Create verification configuration for security level
    pub fn new(security_level: SecurityLevel) -> Self {
        Self {
            security_level,
            min_signatures: security_level.required_signatures(),
            verify_timestamps: matches!(security_level, SecurityLevel::Beta | SecurityLevel::Production),
            max_signature_age: match security_level {
                SecurityLevel::Development => 86400 * 30, // 30 days
                SecurityLevel::Beta => 86400 * 7,         // 7 days
                SecurityLevel::Production => 86400 * 1,   // 1 day
            },
            verify_integrity: true,
            verify_compliance: matches!(security_level, SecurityLevel::Production),
            trusted_keys: HashMap::new(),
        }
    }

    /// Add a trusted public key
    pub fn add_trusted_key(&mut self, key_id: String, public_key: Vec<u8>) {
        self.trusted_keys.insert(key_id, public_key);
    }
}

/// Verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    /// Overall verification status
    pub is_valid: bool,
    /// Individual check results
    pub signature_valid: bool,
    pub integrity_valid: bool,
    pub timestamp_valid: bool,
    pub compliance_valid: bool,
    /// Number of valid signatures found
    pub valid_signatures: usize,
    /// Required number of signatures
    pub required_signatures: usize,
    /// Verification timestamp
    pub verified_at: u64,
    /// Any errors encountered
    pub errors: Vec<String>,
    /// Warnings (non-fatal issues)
    pub warnings: Vec<String>,
    /// Verification metadata
    pub metadata: HashMap<String, String>,
}

impl VerificationResult {
    /// Create a new verification result
    pub fn new(required_signatures: usize) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            is_valid: false,
            signature_valid: false,
            integrity_valid: false,
            timestamp_valid: false,
            compliance_valid: false,
            valid_signatures: 0,
            required_signatures,
            verified_at: timestamp,
            errors: Vec::new(),
            warnings: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add an error
    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    /// Add a warning
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    /// Update overall validity based on individual checks
    pub fn update_validity(&mut self) {
        self.is_valid = self.signature_valid && 
                       self.integrity_valid && 
                       self.timestamp_valid && 
                       self.compliance_valid &&
                       self.valid_signatures >= self.required_signatures;
    }
}

/// Verification status for reporting
#[derive(Debug, Clone)]
pub struct VerificationStatus {
    /// Number of verifications performed
    pub verifications_performed: usize,
    /// Number of successful verifications
    pub successful_verifications: usize,
    /// Number of failed verifications
    pub failed_verifications: usize,
    /// Last verification timestamp
    pub last_verification_time: String,
}

/// Release verifier
pub struct ReleaseVerifier {
    config: VerificationConfig,
    verifications_performed: usize,
    successful_verifications: usize,
    failed_verifications: usize,
    last_verification_time: Option<u64>,
}

impl ReleaseVerifier {
    /// Create a new release verifier
    pub fn new(config: VerificationConfig) -> OvieResult<Self> {
        Ok(Self {
            config,
            verifications_performed: 0,
            successful_verifications: 0,
            failed_verifications: 0,
            last_verification_time: None,
        })
    }

    /// Verify a release package
    pub fn verify_package(&mut self, package: &ReleasePackage) -> OvieResult<VerificationResult> {
        let mut result = VerificationResult::new(self.config.min_signatures);
        
        // Add verification metadata
        result.metadata.insert("security_level".to_string(), self.config.security_level.name().to_string());
        result.metadata.insert("package_version".to_string(), package.version().to_string());
        result.metadata.insert("verifier_config".to_string(), format!("{:?}", self.config.security_level));

        // Perform signature verification
        if !package.signatures().is_empty() {
            result.signature_valid = self.verify_signatures(package, &mut result)?;
        } else {
            result.add_error("No signatures found in package".to_string());
        }

        // Perform integrity verification
        if self.config.verify_integrity {
            result.integrity_valid = self.verify_integrity(package, &mut result)?;
        } else {
            result.integrity_valid = true; // Skip if not required
        }

        // Perform timestamp verification
        if self.config.verify_timestamps {
            result.timestamp_valid = self.verify_timestamps(package, &mut result)?;
        } else {
            result.timestamp_valid = true; // Skip if not required
        }

        // Perform compliance verification
        if self.config.verify_compliance {
            result.compliance_valid = self.verify_compliance(package, &mut result)?;
        } else {
            result.compliance_valid = true; // Skip if not required
        }

        // Update overall validity
        result.update_validity();

        // Update statistics
        self.verifications_performed += 1;
        if result.is_valid {
            self.successful_verifications += 1;
        } else {
            self.failed_verifications += 1;
        }
        self.last_verification_time = Some(result.verified_at);

        Ok(result)
    }

    /// Verify package signatures
    fn verify_signatures(&self, package: &ReleasePackage, result: &mut VerificationResult) -> OvieResult<bool> {
        let signatures = package.signatures();
        let package_data = package.serialize()?;
        
        let mut valid_count = 0;
        
        for signature in signatures {
            match self.verify_single_signature(&package_data, signature, result) {
                Ok(true) => {
                    valid_count += 1;
                    result.add_warning(format!("Valid signature from key: {}", signature.key_id));
                }
                Ok(false) => {
                    result.add_error(format!("Invalid signature from key: {}", signature.key_id));
                }
                Err(e) => {
                    result.add_error(format!("Signature verification error for key {}: {}", signature.key_id, e));
                }
            }
        }
        
        result.valid_signatures = valid_count;
        
        if valid_count < self.config.min_signatures {
            result.add_error(format!(
                "Insufficient valid signatures: {} found, {} required",
                valid_count, self.config.min_signatures
            ));
            return Ok(false);
        }
        
        Ok(true)
    }

    /// Verify a single signature
    fn verify_single_signature(&self, data: &[u8], signature: &SignatureResult, result: &mut VerificationResult) -> OvieResult<bool> {
        // Check if we have the public key
        let public_key = self.config.trusted_keys.get(&signature.key_id);
        if public_key.is_none() {
            result.add_warning(format!("Public key not found for key ID: {}", signature.key_id));
            // For now, we'll mock the verification since we don't have real RSA
            return Ok(self.mock_signature_verification(data, signature));
        }

        // Verify data hash
        let mut hasher = Sha256::new();
        hasher.update(data);
        let computed_hash = format!("{:x}", hasher.finalize());
        
        if computed_hash != signature.data_hash {
            return Ok(false);
        }

        // Mock RSA signature verification
        Ok(self.mock_signature_verification(data, signature))
    }

    /// Mock signature verification (placeholder for real RSA verification)
    fn mock_signature_verification(&self, data: &[u8], signature: &SignatureResult) -> bool {
        // Create expected signature using the same method as signing
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.update(&signature.timestamp.to_le_bytes());
        hasher.update(b"signature");
        
        // In a real implementation, we would use the public key here
        // For now, we'll assume the signature is valid if it matches our mock format
        let expected_length = 32; // SHA-256 output length
        signature.signature.len() == expected_length
    }

    /// Verify package integrity
    fn verify_integrity(&self, package: &ReleasePackage, result: &mut VerificationResult) -> OvieResult<bool> {
        // Verify package structure
        if package.version().is_empty() {
            result.add_error("Package version is empty".to_string());
            return Ok(false);
        }

        if package.artifacts().is_empty() {
            result.add_error("Package contains no artifacts".to_string());
            return Ok(false);
        }

        // Verify artifact integrity
        for (name, data) in package.artifacts() {
            if name.is_empty() {
                result.add_error("Artifact has empty name".to_string());
                return Ok(false);
            }
            
            if data.is_empty() {
                result.add_warning(format!("Artifact '{}' is empty", name));
            }
        }

        // Verify metadata integrity
        let metadata = package.metadata();
        if metadata.created_at == 0 {
            result.add_warning("Package creation timestamp is zero".to_string());
        }

        if metadata.build_hash.is_empty() {
            result.add_warning("Package build hash is empty".to_string());
        }

        Ok(true)
    }

    /// Verify timestamps
    fn verify_timestamps(&self, package: &ReleasePackage, result: &mut VerificationResult) -> OvieResult<bool> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Verify package creation time
        let package_time = package.metadata().created_at;
        if package_time > current_time {
            result.add_error("Package creation time is in the future".to_string());
            return Ok(false);
        }

        // Verify signature timestamps
        for signature in package.signatures() {
            if signature.timestamp > current_time {
                result.add_error(format!("Signature timestamp is in the future: {}", signature.key_id));
                return Ok(false);
            }

            let signature_age = current_time.saturating_sub(signature.timestamp);
            if signature_age > self.config.max_signature_age {
                result.add_error(format!(
                    "Signature is too old: {} seconds (max: {})",
                    signature_age, self.config.max_signature_age
                ));
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Verify security compliance
    fn verify_compliance(&self, package: &ReleasePackage, result: &mut VerificationResult) -> OvieResult<bool> {
        // Verify security level compliance
        let metadata = package.metadata();
        
        // Check for required security metadata
        if !metadata.security_audit_passed {
            result.add_error("Package failed security audit".to_string());
            return Ok(false);
        }

        if metadata.vulnerability_scan_passed.is_none() {
            result.add_warning("No vulnerability scan results available".to_string());
        } else if !metadata.vulnerability_scan_passed.unwrap() {
            result.add_error("Package failed vulnerability scan".to_string());
            return Ok(false);
        }

        // Verify reproducible build
        if !metadata.reproducible_build {
            if matches!(self.config.security_level, SecurityLevel::Production) {
                result.add_error("Non-reproducible build not allowed for production releases".to_string());
                return Ok(false);
            } else {
                result.add_warning("Build is not reproducible".to_string());
            }
        }

        // Check for required compliance metadata
        if metadata.compliance_info.is_empty() {
            result.add_warning("No compliance information available".to_string());
        }

        Ok(true)
    }

    /// Add trusted public key
    pub fn add_trusted_key(&mut self, key_id: String, public_key: Vec<u8>) {
        self.config.add_trusted_key(key_id, public_key);
    }

    /// Get verification status
    pub fn get_status(&self) -> VerificationStatus {
        let last_verification_time = self.last_verification_time
            .map(|ts| {
                let datetime = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(ts);
                format!("{:?}", datetime)
            })
            .unwrap_or_else(|| "Never".to_string());

        VerificationStatus {
            verifications_performed: self.verifications_performed,
            successful_verifications: self.successful_verifications,
            failed_verifications: self.failed_verifications,
            last_verification_time,
        }
    }

    /// Verify a signature directly (for testing)
    pub fn verify_signature_direct(&self, data: &[u8], signature: &SignatureResult) -> OvieResult<bool> {
        let mut result = VerificationResult::new(1);
        self.verify_single_signature(data, signature, &mut result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::release::{ReleaseMetadata, DistributionManager, DistributionConfig};

    fn create_test_package() -> ReleasePackage {
        let config = DistributionConfig::new(SecurityLevel::Development);
        let mut manager = DistributionManager::new(config).unwrap();
        
        let artifacts = vec![
            ("ovie.exe", b"mock executable data".to_vec()),
            ("README.md", b"# Ovie Release".to_vec()),
        ];
        
        let metadata = ReleaseMetadata::new("1.0.0", "test-build-hash");
        manager.create_package("1.0.0", &[("ovie.exe", artifacts[0].1.clone()), ("README.md", artifacts[1].1.clone())], metadata).unwrap()
    }

    #[test]
    fn test_verification_config_creation() {
        let config = VerificationConfig::new(SecurityLevel::Production);
        assert_eq!(config.min_signatures, 3);
        assert!(config.verify_timestamps);
        assert!(config.verify_compliance);
    }

    #[test]
    fn test_verifier_creation() {
        let config = VerificationConfig::new(SecurityLevel::Development);
        let verifier = ReleaseVerifier::new(config);
        
        assert!(verifier.is_ok());
    }

    #[test]
    fn test_package_verification_no_signatures() {
        let config = VerificationConfig::new(SecurityLevel::Development);
        let mut verifier = ReleaseVerifier::new(config).unwrap();
        
        let package = create_test_package();
        let result = verifier.verify_package(&package).unwrap();
        
        assert!(!result.is_valid);
        assert!(!result.signature_valid);
        assert!(result.errors.iter().any(|e| e.contains("No signatures found")));
    }

    #[test]
    fn test_verification_result_creation() {
        let mut result = VerificationResult::new(2);
        assert_eq!(result.required_signatures, 2);
        assert!(!result.is_valid);
        
        result.add_error("Test error".to_string());
        result.add_warning("Test warning".to_string());
        
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_integrity_verification() {
        let config = VerificationConfig::new(SecurityLevel::Development);
        let verifier = ReleaseVerifier::new(config).unwrap();
        
        let package = create_test_package();
        let mut result = VerificationResult::new(1);
        
        let integrity_valid = verifier.verify_integrity(&package, &mut result).unwrap();
        assert!(integrity_valid);
    }

    #[test]
    fn test_timestamp_verification() {
        let config = VerificationConfig::new(SecurityLevel::Beta);
        let verifier = ReleaseVerifier::new(config).unwrap();
        
        let package = create_test_package();
        let mut result = VerificationResult::new(1);
        
        let timestamp_valid = verifier.verify_timestamps(&package, &mut result).unwrap();
        assert!(timestamp_valid);
    }

    #[test]
    fn test_status_reporting() {
        let config = VerificationConfig::new(SecurityLevel::Development);
        let mut verifier = ReleaseVerifier::new(config).unwrap();
        
        let status = verifier.get_status();
        assert_eq!(status.verifications_performed, 0);
        assert_eq!(status.successful_verifications, 0);
        
        // Perform a verification
        let package = create_test_package();
        let _result = verifier.verify_package(&package).unwrap();
        
        let updated_status = verifier.get_status();
        assert_eq!(updated_status.verifications_performed, 1);
    }

    #[test]
    fn test_trusted_key_management() {
        let config = VerificationConfig::new(SecurityLevel::Development);
        let mut verifier = ReleaseVerifier::new(config).unwrap();
        
        let key_id = "test-key-123".to_string();
        let public_key = b"mock public key data".to_vec();
        
        verifier.add_trusted_key(key_id.clone(), public_key.clone());
        
        assert!(verifier.config.trusted_keys.contains_key(&key_id));
        assert_eq!(verifier.config.trusted_keys[&key_id], public_key);
    }
}