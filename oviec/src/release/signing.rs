//! Cryptographic Signing System for Ovie Releases
//! 
//! This module provides RSA-based digital signatures for release artifacts,
//! ensuring authenticity and integrity of distributed Ovie components.

use crate::error::{OvieError, OvieResult};
use crate::release::SecurityLevel;
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

/// RSA signing key for release artifacts
#[derive(Debug, Clone)]
pub struct SigningKey {
    /// Key identifier
    pub key_id: String,
    /// Key strength in bits
    pub key_strength: usize,
    /// Creation timestamp
    pub created_at: u64,
    /// Key purpose/role
    pub purpose: KeyPurpose,
    /// Mock private key data (in real implementation, this would be secure)
    private_key_data: Vec<u8>,
    /// Public key data for verification
    pub public_key_data: Vec<u8>,
}

/// Key purpose enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KeyPurpose {
    /// Primary release signing key
    Primary,
    /// Secondary/backup signing key
    Secondary,
    /// Development/testing key
    Development,
    /// Emergency revocation key
    Emergency,
}

/// Signing configuration
#[derive(Debug, Clone)]
pub struct SigningConfig {
    /// Required security level
    pub security_level: SecurityLevel,
    /// Minimum key strength
    pub min_key_strength: usize,
    /// Required number of signatures
    pub required_signatures: usize,
    /// Enable timestamp verification
    pub timestamp_verification: bool,
    /// Maximum signature age in seconds
    pub max_signature_age: u64,
}

impl SigningConfig {
    /// Create signing configuration for security level
    pub fn new(security_level: SecurityLevel) -> Self {
        Self {
            security_level,
            min_key_strength: security_level.required_key_strength(),
            required_signatures: security_level.required_signatures(),
            timestamp_verification: matches!(security_level, SecurityLevel::Beta | SecurityLevel::Production),
            max_signature_age: match security_level {
                SecurityLevel::Development => 86400 * 30, // 30 days
                SecurityLevel::Beta => 86400 * 7,         // 7 days
                SecurityLevel::Production => 86400 * 1,   // 1 day
            },
        }
    }
}

/// Digital signature result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureResult {
    /// Signature data
    pub signature: Vec<u8>,
    /// Key ID used for signing
    pub key_id: String,
    /// Signature algorithm
    pub algorithm: String,
    /// Timestamp when signature was created
    pub timestamp: u64,
    /// Hash of the signed data
    pub data_hash: String,
    /// Signature metadata
    pub metadata: HashMap<String, String>,
}

/// Signing manager status
#[derive(Debug, Clone)]
pub struct SigningStatus {
    /// Number of available keys
    pub available_keys: usize,
    /// Number of signatures created
    pub signatures_created: usize,
    /// Last signing timestamp
    pub last_signing_time: String,
    /// Key health status
    pub key_health: HashMap<String, bool>,
}

/// Release signing manager
pub struct ReleaseSigningManager {
    config: SigningConfig,
    signing_keys: HashMap<String, SigningKey>,
    signatures_created: usize,
    last_signing_time: Option<u64>,
}

impl ReleaseSigningManager {
    /// Create a new signing manager
    pub fn new(config: SigningConfig) -> OvieResult<Self> {
        let mut manager = Self {
            config,
            signing_keys: HashMap::new(),
            signatures_created: 0,
            last_signing_time: None,
        };

        // Generate initial signing keys
        manager.generate_initial_keys()?;

        Ok(manager)
    }

    /// Generate initial signing keys for the security level
    fn generate_initial_keys(&mut self) -> OvieResult<()> {
        let key_strength = self.config.min_key_strength;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Generate primary key
        let primary_key = self.generate_key("primary", KeyPurpose::Primary, key_strength, timestamp)?;
        self.signing_keys.insert(primary_key.key_id.clone(), primary_key);

        // Generate additional keys based on security level requirements
        let required_signatures = self.config.required_signatures;
        
        // Generate secondary key for beta and production
        if matches!(self.config.security_level, SecurityLevel::Beta | SecurityLevel::Production) && required_signatures > 1 {
            let secondary_key = self.generate_key("secondary", KeyPurpose::Secondary, key_strength, timestamp + 1)?;
            self.signing_keys.insert(secondary_key.key_id.clone(), secondary_key);
        }

        // Generate additional keys for production if needed
        if matches!(self.config.security_level, SecurityLevel::Production) && required_signatures > 2 {
            for i in 2..required_signatures {
                let key_name = format!("key-{}", i);
                let key = self.generate_key(&key_name, KeyPurpose::Secondary, key_strength, timestamp + i as u64)?;
                self.signing_keys.insert(key.key_id.clone(), key);
            }
        }

        // Generate development key for development/beta (in addition to others)
        if matches!(self.config.security_level, SecurityLevel::Development | SecurityLevel::Beta) {
            let dev_key = self.generate_key("development", KeyPurpose::Development, key_strength, timestamp + 10)?;
            self.signing_keys.insert(dev_key.key_id.clone(), dev_key);
        }

        Ok(())
    }

    /// Generate a new signing key
    fn generate_key(&self, name: &str, purpose: KeyPurpose, strength: usize, timestamp: u64) -> OvieResult<SigningKey> {
        // In a real implementation, this would use proper RSA key generation
        // For now, we'll create mock keys with deterministic data
        
        let key_id = format!("ovie-{}-{}-{}", name, strength, timestamp);
        
        // Mock key generation (deterministic for testing)
        let mut hasher = Sha256::new();
        hasher.update(key_id.as_bytes());
        hasher.update(&strength.to_le_bytes());
        hasher.update(&timestamp.to_le_bytes());
        let seed = hasher.finalize();
        
        // Generate mock private key (in real implementation, this would be secure RSA)
        let private_key_data = seed.to_vec();
        
        // Generate mock public key (derived from private key)
        let mut pub_hasher = Sha256::new();
        pub_hasher.update(&private_key_data);
        pub_hasher.update(b"public");
        let public_key_data = pub_hasher.finalize().to_vec();

        Ok(SigningKey {
            key_id,
            key_strength: strength,
            created_at: timestamp,
            purpose,
            private_key_data,
            public_key_data,
        })
    }

    /// Sign data with the primary key
    pub fn sign_data(&mut self, data: &[u8]) -> OvieResult<SignatureResult> {
        let primary_key_id = {
            let primary_key = self.get_primary_key()?;
            primary_key.key_id.clone()
        };
        self.sign_data_with_key(data, &primary_key_id)
    }

    /// Sign data with a specific key
    pub fn sign_data_with_key(&mut self, data: &[u8], key_id: &str) -> OvieResult<SignatureResult> {
        let key = self.signing_keys.get(key_id)
            .ok_or_else(|| OvieError::runtime_error(format!("Signing key not found: {}", key_id)))?;

        // Verify key strength meets requirements
        if key.key_strength < self.config.min_key_strength {
            return Err(OvieError::runtime_error(format!(
                "Key strength {} below minimum required {}",
                key.key_strength, self.config.min_key_strength
            )));
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Compute data hash
        let mut hasher = Sha256::new();
        hasher.update(data);
        let data_hash = format!("{:x}", hasher.finalize());

        // Create signature (mock RSA signature)
        let signature = self.create_signature(data, &key.private_key_data, timestamp)?;

        // Create metadata
        let mut metadata = HashMap::new();
        metadata.insert("key_purpose".to_string(), format!("{:?}", key.purpose));
        metadata.insert("key_strength".to_string(), key.key_strength.to_string());
        metadata.insert("security_level".to_string(), self.config.security_level.name().to_string());

        let result = SignatureResult {
            signature,
            key_id: key_id.to_string(),
            algorithm: format!("RSA-{}-SHA256", key.key_strength),
            timestamp,
            data_hash,
            metadata,
        };

        // Update statistics
        self.signatures_created += 1;
        self.last_signing_time = Some(timestamp);

        Ok(result)
    }

    /// Create mock RSA signature
    fn create_signature(&self, data: &[u8], private_key: &[u8], timestamp: u64) -> OvieResult<Vec<u8>> {
        // Mock signature creation (in real implementation, this would be proper RSA signing)
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.update(private_key);
        hasher.update(&timestamp.to_le_bytes());
        hasher.update(b"signature");
        
        Ok(hasher.finalize().to_vec())
    }

    /// Sign a release package
    pub fn sign_package(&mut self, package: &crate::release::ReleasePackage) -> OvieResult<SignatureResult> {
        let package_data = package.serialize()?;
        self.sign_data(&package_data)
    }

    /// Get the primary signing key
    fn get_primary_key(&self) -> OvieResult<&SigningKey> {
        self.signing_keys.values()
            .find(|key| key.purpose == KeyPurpose::Primary)
            .ok_or_else(|| OvieError::runtime_error("No primary signing key available".to_string()))
    }

    /// Get all available keys
    pub fn get_available_keys(&self) -> Vec<&SigningKey> {
        self.signing_keys.values().collect()
    }

    /// Get key by ID
    pub fn get_key(&self, key_id: &str) -> Option<&SigningKey> {
        self.signing_keys.get(key_id)
    }

    /// Add a new signing key
    pub fn add_key(&mut self, key: SigningKey) -> OvieResult<()> {
        if key.key_strength < self.config.min_key_strength {
            return Err(OvieError::runtime_error(format!(
                "Key strength {} below minimum required {}",
                key.key_strength, self.config.min_key_strength
            )));
        }

        self.signing_keys.insert(key.key_id.clone(), key);
        Ok(())
    }

    /// Remove a signing key
    pub fn remove_key(&mut self, key_id: &str) -> OvieResult<()> {
        if let Some(key) = self.signing_keys.get(key_id) {
            if key.purpose == KeyPurpose::Primary && self.signing_keys.len() == 1 {
                return Err(OvieError::runtime_error(
                    "Cannot remove the last primary key".to_string()
                ));
            }
        }

        self.signing_keys.remove(key_id)
            .ok_or_else(|| OvieError::runtime_error(format!("Key not found: {}", key_id)))?;

        Ok(())
    }

    /// Get signing manager status
    pub fn get_status(&self) -> SigningStatus {
        let last_signing_time = self.last_signing_time
            .map(|ts| {
                let datetime = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(ts);
                format!("{:?}", datetime)
            })
            .unwrap_or_else(|| "Never".to_string());

        let key_health = self.signing_keys.iter()
            .map(|(id, key)| {
                // Mock key health check
                let is_healthy = key.key_strength >= self.config.min_key_strength;
                (id.clone(), is_healthy)
            })
            .collect();

        SigningStatus {
            available_keys: self.signing_keys.len(),
            signatures_created: self.signatures_created,
            last_signing_time,
            key_health,
        }
    }

    /// Verify a signature (for testing purposes)
    pub fn verify_signature(&self, data: &[u8], signature: &SignatureResult) -> OvieResult<bool> {
        let key = self.get_key(&signature.key_id)
            .ok_or_else(|| OvieError::runtime_error(format!("Key not found: {}", signature.key_id)))?;

        // Verify data hash
        let mut hasher = Sha256::new();
        hasher.update(data);
        let computed_hash = format!("{:x}", hasher.finalize());
        
        if computed_hash != signature.data_hash {
            return Ok(false);
        }

        // Verify signature (mock verification)
        let expected_signature = self.create_signature(data, &key.private_key_data, signature.timestamp)?;
        
        Ok(expected_signature == signature.signature)
    }

    /// Export public keys for distribution
    pub fn export_public_keys(&self) -> HashMap<String, Vec<u8>> {
        self.signing_keys.iter()
            .map(|(id, key)| (id.clone(), key.public_key_data.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signing_config_creation() {
        let config = SigningConfig::new(SecurityLevel::Production);
        assert_eq!(config.min_key_strength, 4096);
        assert_eq!(config.required_signatures, 3);
        assert!(config.timestamp_verification);
    }

    #[test]
    fn test_signing_manager_creation() {
        let config = SigningConfig::new(SecurityLevel::Development);
        let manager = ReleaseSigningManager::new(config);
        
        assert!(manager.is_ok());
        let manager = manager.unwrap();
        assert!(!manager.get_available_keys().is_empty());
    }

    #[test]
    fn test_data_signing() {
        let config = SigningConfig::new(SecurityLevel::Development);
        let mut manager = ReleaseSigningManager::new(config).unwrap();
        
        let test_data = b"Hello, Ovie Release!";
        let signature = manager.sign_data(test_data);
        
        assert!(signature.is_ok());
        let signature = signature.unwrap();
        assert!(!signature.signature.is_empty());
        assert!(!signature.key_id.is_empty());
        assert_eq!(signature.algorithm, "RSA-2048-SHA256");
    }

    #[test]
    fn test_signature_verification() {
        let config = SigningConfig::new(SecurityLevel::Development);
        let mut manager = ReleaseSigningManager::new(config).unwrap();
        
        let test_data = b"Test data for verification";
        let signature = manager.sign_data(test_data).unwrap();
        
        let is_valid = manager.verify_signature(test_data, &signature).unwrap();
        assert!(is_valid);
        
        // Test with modified data
        let modified_data = b"Modified test data";
        let is_valid_modified = manager.verify_signature(modified_data, &signature).unwrap();
        assert!(!is_valid_modified);
    }

    #[test]
    fn test_key_management() {
        let config = SigningConfig::new(SecurityLevel::Development);
        let mut manager = ReleaseSigningManager::new(config).unwrap();
        
        let initial_count = manager.get_available_keys().len();
        
        // Generate a new key
        let new_key = manager.generate_key("test", KeyPurpose::Development, 2048, 1234567890).unwrap();
        let key_id = new_key.key_id.clone();
        
        // Add the key
        manager.add_key(new_key).unwrap();
        assert_eq!(manager.get_available_keys().len(), initial_count + 1);
        
        // Remove the key
        manager.remove_key(&key_id).unwrap();
        assert_eq!(manager.get_available_keys().len(), initial_count);
    }

    #[test]
    fn test_status_reporting() {
        let config = SigningConfig::new(SecurityLevel::Production);
        let mut manager = ReleaseSigningManager::new(config).unwrap();
        
        let status = manager.get_status();
        assert!(status.available_keys > 0);
        assert_eq!(status.signatures_created, 0);
        
        // Create a signature
        let test_data = b"Status test data";
        let _signature = manager.sign_data(test_data).unwrap();
        
        let updated_status = manager.get_status();
        assert_eq!(updated_status.signatures_created, 1);
        assert_ne!(updated_status.last_signing_time, "Never");
    }

    #[test]
    fn test_public_key_export() {
        let config = SigningConfig::new(SecurityLevel::Development);
        let manager = ReleaseSigningManager::new(config).unwrap();
        
        let public_keys = manager.export_public_keys();
        assert!(!public_keys.is_empty());
        
        for (key_id, public_key) in &public_keys {
            assert!(!key_id.is_empty());
            assert!(!public_key.is_empty());
        }
    }
}