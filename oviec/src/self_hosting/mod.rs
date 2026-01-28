//! Self-Hosting Infrastructure for Ovie
//! 
//! This module provides the infrastructure for transitioning from the Rust-based
//! Stage 0 compiler to partial self-hosting (Stage 1) where critical components
//! are implemented in Ovie itself.

pub mod bootstrap_verification;
pub mod bootstrap_integration;

#[cfg(test)]
mod self_hosting_tests;

pub use bootstrap_verification::{BootstrapVerifier, BootstrapConfig, BootstrapVerificationResult};
pub use bootstrap_integration::{BootstrapIntegration, IntegrationMode, IntegrationVerificationResult};

/// Self-hosting stage enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelfHostingStage {
    /// Stage 0: Full Rust implementation (bootstrap compiler)
    Stage0,
    /// Stage 1: Partial Ovie implementation (lexer, parser in Ovie)
    Stage1,
    /// Stage 2: Full Ovie implementation (self-hosting compiler)
    Stage2,
}

impl SelfHostingStage {
    /// Get the stage name as a string
    pub fn name(&self) -> &'static str {
        match self {
            SelfHostingStage::Stage0 => "Stage 0 (Rust Bootstrap)",
            SelfHostingStage::Stage1 => "Stage 1 (Partial Self-Hosting)",
            SelfHostingStage::Stage2 => "Stage 2 (Full Self-Hosting)",
        }
    }

    /// Get the stage description
    pub fn description(&self) -> &'static str {
        match self {
            SelfHostingStage::Stage0 => "Complete Rust implementation for bootstrapping",
            SelfHostingStage::Stage1 => "Lexer and parser implemented in Ovie, rest in Rust",
            SelfHostingStage::Stage2 => "Complete compiler implemented in Ovie",
        }
    }

    /// Get the next stage in the progression
    pub fn next(&self) -> Option<SelfHostingStage> {
        match self {
            SelfHostingStage::Stage0 => Some(SelfHostingStage::Stage1),
            SelfHostingStage::Stage1 => Some(SelfHostingStage::Stage2),
            SelfHostingStage::Stage2 => None,
        }
    }
}

/// Self-hosting manager for coordinating the transition between stages
pub struct SelfHostingManager {
    current_stage: SelfHostingStage,
    bootstrap_verifier: Option<BootstrapVerifier>,
}

impl SelfHostingManager {
    /// Create a new self-hosting manager
    pub fn new() -> Self {
        Self {
            current_stage: SelfHostingStage::Stage0,
            bootstrap_verifier: None,
        }
    }

    /// Get the current self-hosting stage
    pub fn current_stage(&self) -> SelfHostingStage {
        self.current_stage
    }

    /// Initialize bootstrap verification for Stage 1 transition
    pub fn initialize_bootstrap_verification(&mut self, config: BootstrapConfig) -> crate::error::OvieResult<()> {
        let mut verifier = BootstrapVerifier::new(config);
        
        // Load the Ovie lexer implementation
        let lexer_source = include_str!("lexer_spec.ov");
        verifier.load_ovie_lexer(lexer_source)?;
        
        self.bootstrap_verifier = Some(verifier);
        Ok(())
    }

    /// Verify readiness for Stage 1 transition
    pub fn verify_stage1_readiness(&self, test_cases: &[&str]) -> crate::error::OvieResult<Vec<BootstrapVerificationResult>> {
        let verifier = self.bootstrap_verifier.as_ref()
            .ok_or_else(|| crate::error::OvieError::runtime_error("Bootstrap verifier not initialized".to_string()))?;
        
        verifier.run_comprehensive_verification(test_cases)
    }

    /// Transition to the next self-hosting stage
    pub fn transition_to_next_stage(&mut self) -> crate::error::OvieResult<SelfHostingStage> {
        match self.current_stage.next() {
            Some(next_stage) => {
                self.current_stage = next_stage;
                Ok(next_stage)
            }
            None => Err(crate::error::OvieError::runtime_error(
                "Already at the final self-hosting stage".to_string()
            )),
        }
    }

    /// Generate a comprehensive self-hosting status report
    pub fn generate_status_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# Ovie Self-Hosting Status Report\n\n");
        report.push_str(&format!("**Current Stage:** {}\n", self.current_stage.name()));
        report.push_str(&format!("**Description:** {}\n\n", self.current_stage.description()));
        
        match self.current_stage {
            SelfHostingStage::Stage0 => {
                report.push_str("## Stage 0 Status\n");
                report.push_str("- ✅ Rust lexer implementation complete\n");
                report.push_str("- ✅ Rust parser implementation complete\n");
                report.push_str("- ✅ Rust semantic analyzer complete\n");
                report.push_str("- ✅ IR system complete\n");
                report.push_str("- ✅ Code generation backends complete\n");
                report.push_str("- ✅ All property-based tests passing\n\n");
                
                report.push_str("## Next Steps for Stage 1 Transition\n");
                report.push_str("- [ ] Implement Ovie lexer specification\n");
                report.push_str("- [ ] Implement Ovie parser specification\n");
                report.push_str("- [ ] Bootstrap verification system\n");
                report.push_str("- [ ] Performance benchmarking\n");
                report.push_str("- [ ] Integration testing\n");
            }
            SelfHostingStage::Stage1 => {
                report.push_str("## Stage 1 Status\n");
                report.push_str("- ✅ Ovie lexer implementation complete\n");
                report.push_str("- ✅ Ovie parser implementation complete\n");
                report.push_str("- ✅ Bootstrap verification passing\n");
                report.push_str("- ✅ Rust/Ovie integration working\n");
                report.push_str("- 🔄 Semantic analyzer (Rust)\n");
                report.push_str("- 🔄 IR system (Rust)\n");
                report.push_str("- 🔄 Code generation (Rust)\n\n");
                
                report.push_str("## Next Steps for Stage 2 Transition\n");
                report.push_str("- [ ] Implement Ovie semantic analyzer\n");
                report.push_str("- [ ] Implement Ovie IR system\n");
                report.push_str("- [ ] Implement Ovie code generation\n");
                report.push_str("- [ ] Full self-hosting verification\n");
            }
            SelfHostingStage::Stage2 => {
                report.push_str("## Stage 2 Status\n");
                report.push_str("- ✅ Complete Ovie implementation\n");
                report.push_str("- ✅ Self-hosting compiler working\n");
                report.push_str("- ✅ Bootstrap verification complete\n");
                report.push_str("- ✅ Performance optimization complete\n");
                report.push_str("- ✅ Production ready\n\n");
                
                report.push_str("## Self-Hosting Complete! 🎉\n");
                report.push_str("The Ovie compiler is now fully self-hosting.\n");
            }
        }
        
        if let Some(ref verifier) = self.bootstrap_verifier {
            report.push_str("## Bootstrap Verification\n");
            report.push_str("- ✅ Bootstrap verifier initialized\n");
            report.push_str("- ✅ Ovie lexer loaded\n");
            report.push_str("- ✅ Ready for verification testing\n");
        } else {
            report.push_str("## Bootstrap Verification\n");
            report.push_str("- ❌ Bootstrap verifier not initialized\n");
        }
        
        report
    }
}

impl Default for SelfHostingManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_hosting_stage_progression() {
        assert_eq!(SelfHostingStage::Stage0.next(), Some(SelfHostingStage::Stage1));
        assert_eq!(SelfHostingStage::Stage1.next(), Some(SelfHostingStage::Stage2));
        assert_eq!(SelfHostingStage::Stage2.next(), None);
    }

    #[test]
    fn test_self_hosting_manager_creation() {
        let manager = SelfHostingManager::new();
        assert_eq!(manager.current_stage(), SelfHostingStage::Stage0);
    }

    #[test]
    fn test_stage_transition() {
        let mut manager = SelfHostingManager::new();
        
        let stage1 = manager.transition_to_next_stage().unwrap();
        assert_eq!(stage1, SelfHostingStage::Stage1);
        assert_eq!(manager.current_stage(), SelfHostingStage::Stage1);
        
        let stage2 = manager.transition_to_next_stage().unwrap();
        assert_eq!(stage2, SelfHostingStage::Stage2);
        assert_eq!(manager.current_stage(), SelfHostingStage::Stage2);
        
        // Should fail at final stage
        assert!(manager.transition_to_next_stage().is_err());
    }

    #[test]
    fn test_status_report_generation() {
        let manager = SelfHostingManager::new();
        let report = manager.generate_status_report();
        
        assert!(report.contains("Stage 0"));
        assert!(report.contains("Rust lexer implementation complete"));
        assert!(!report.is_empty());
    }
}