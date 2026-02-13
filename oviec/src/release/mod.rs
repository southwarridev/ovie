//! Release and Distribution System for Ovie
//! 
//! This module provides cryptographic signing, verification, and reproducible
//! release capabilities for the Ovie programming language ecosystem.

pub mod signing;
pub mod verification;
pub mod distribution;
pub mod builder;
pub mod installer;

pub use signing::{ReleaseSigningManager, SigningKey, SigningConfig, SignatureResult};
pub use verification::{ReleaseVerifier, VerificationResult, VerificationConfig};
pub use distribution::{DistributionManager, ReleasePackage, ReleaseMetadata, DistributionConfig};
pub use builder::{DistributionBuilder, Platform, PackageStructure};
pub use installer::{Installer, InstallConfig, InstallationResult};

/// Release security level
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SecurityLevel {
    /// Development releases (minimal security)
    Development,
    /// Beta releases (moderate security)
    Beta,
    /// Production releases (maximum security)
    Production,
}

impl SecurityLevel {
    /// Get the security level name
    pub fn name(&self) -> &'static str {
        match self {
            SecurityLevel::Development => "Development",
            SecurityLevel::Beta => "Beta",
            SecurityLevel::Production => "Production",
        }
    }

    /// Get required signature count for this security level
    pub fn required_signatures(&self) -> usize {
        match self {
            SecurityLevel::Development => 1,
            SecurityLevel::Beta => 2,
            SecurityLevel::Production => 3,
        }
    }

    /// Get required key strength for this security level
    pub fn required_key_strength(&self) -> usize {
        match self {
            SecurityLevel::Development => 2048,
            SecurityLevel::Beta => 3072,
            SecurityLevel::Production => 4096,
        }
    }
}

/// Release manager coordinating all release operations
pub struct ReleaseManager {
    signing_manager: ReleaseSigningManager,
    verifier: ReleaseVerifier,
    distribution_manager: DistributionManager,
    security_level: SecurityLevel,
}

impl ReleaseManager {
    /// Create a new release manager
    pub fn new(security_level: SecurityLevel) -> crate::error::OvieResult<Self> {
        let signing_config = SigningConfig::new(security_level);
        let verification_config = VerificationConfig::new(security_level);
        let distribution_config = DistributionConfig::new(security_level);

        Ok(Self {
            signing_manager: ReleaseSigningManager::new(signing_config)?,
            verifier: ReleaseVerifier::new(verification_config)?,
            distribution_manager: DistributionManager::new(distribution_config)?,
            security_level,
        })
    }

    /// Get the current security level
    pub fn security_level(&self) -> SecurityLevel {
        self.security_level
    }

    /// Create a signed release package
    pub fn create_release(&mut self, 
        version: &str, 
        artifacts: &[(&str, Vec<u8>)],
        metadata: ReleaseMetadata
    ) -> crate::error::OvieResult<ReleasePackage> {
        // Create the release package
        let mut package = self.distribution_manager.create_package(version, artifacts, metadata)?;
        
        // Get the package data for signing
        let package_data = package.serialize()?;
        
        // Sign the package with all available keys to meet security requirements
        let available_key_ids: Vec<String> = self.signing_manager.get_available_keys()
            .iter()
            .map(|key| key.key_id.clone())
            .collect();
        
        let required_signatures = self.security_level.required_signatures();
        
        // Sign with as many keys as we have available, up to the required amount
        let keys_to_use = std::cmp::min(available_key_ids.len(), required_signatures);
        
        for i in 0..keys_to_use {
            let key_id = &available_key_ids[i];
            let signature = self.signing_manager.sign_data_with_key(&package_data, key_id)?;
            package.add_signature(signature);
        }
        
        // For development, we can proceed with fewer signatures
        // For production, we need to ensure we have enough signatures
        if matches!(self.security_level, SecurityLevel::Production) && package.signatures().len() < required_signatures {
            return Err(crate::error::OvieError::runtime_error(
                format!("Insufficient signing keys available: {} required, {} available", 
                    required_signatures, available_key_ids.len())
            ));
        }
        
        // Verify the package
        let verification = self.verifier.verify_package(&package)?;
        if !verification.is_valid && matches!(self.security_level, SecurityLevel::Production) {
            return Err(crate::error::OvieError::runtime_error(
                format!("Release verification failed: {:?}", verification.errors)
            ));
        }
        
        Ok(package)
    }

    /// Verify an existing release package
    pub fn verify_release(&mut self, package: &ReleasePackage) -> crate::error::OvieResult<VerificationResult> {
        self.verifier.verify_package(package)
    }

    /// Generate release status report
    pub fn generate_release_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# Ovie Release System Status\n\n");
        report.push_str(&format!("**Security Level:** {}\n", self.security_level.name()));
        report.push_str(&format!("**Required Signatures:** {}\n", self.security_level.required_signatures()));
        report.push_str(&format!("**Key Strength:** {} bits\n\n", self.security_level.required_key_strength()));
        
        // Signing status
        report.push_str("## Cryptographic Signing\n");
        let signing_status = self.signing_manager.get_status();
        report.push_str(&format!("- **Keys Available:** {}\n", signing_status.available_keys));
        report.push_str(&format!("- **Signatures Created:** {}\n", signing_status.signatures_created));
        report.push_str(&format!("- **Last Signing:** {}\n\n", signing_status.last_signing_time));
        
        // Verification status
        report.push_str("## Verification System\n");
        let verification_status = self.verifier.get_status();
        report.push_str(&format!("- **Verifications Performed:** {}\n", verification_status.verifications_performed));
        report.push_str(&format!("- **Successful Verifications:** {}\n", verification_status.successful_verifications));
        report.push_str(&format!("- **Failed Verifications:** {}\n\n", verification_status.failed_verifications));
        
        // Distribution status
        report.push_str("## Distribution System\n");
        let distribution_status = self.distribution_manager.get_status();
        report.push_str(&format!("- **Packages Created:** {}\n", distribution_status.packages_created));
        report.push_str(&format!("- **Total Size:** {} MB\n", distribution_status.total_size_mb));
        report.push_str(&format!("- **Last Release:** {}\n", distribution_status.last_release_time));
        
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_level_properties() {
        assert_eq!(SecurityLevel::Development.required_signatures(), 1);
        assert_eq!(SecurityLevel::Beta.required_signatures(), 2);
        assert_eq!(SecurityLevel::Production.required_signatures(), 3);
        
        assert_eq!(SecurityLevel::Development.required_key_strength(), 2048);
        assert_eq!(SecurityLevel::Beta.required_key_strength(), 3072);
        assert_eq!(SecurityLevel::Production.required_key_strength(), 4096);
    }

    #[test]
    fn test_release_manager_creation() {
        let manager = ReleaseManager::new(SecurityLevel::Development);
        assert!(manager.is_ok());
        
        let manager = manager.unwrap();
        assert_eq!(manager.security_level(), SecurityLevel::Development);
    }

    #[test]
    fn test_release_report_generation() {
        let manager = ReleaseManager::new(SecurityLevel::Production).unwrap();
        let report = manager.generate_release_report();
        
        assert!(report.contains("Ovie Release System Status"));
        assert!(report.contains("Production"));
        assert!(report.contains("4096 bits"));
        assert!(report.contains("Cryptographic Signing"));
    }
}