//! Hardware Abstraction Layer (HAL) for Ovie
//! 
//! This module provides safe, mathematical abstractions for hardware interaction.
//! It prevents direct register access and models devices as mathematical objects
//! that can be analyzed by both humans and automated reasoning systems.

use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize};
use crate::error::{OvieError, OvieResult};

/// Mathematical representation of a hardware device
/// 
/// Devices are modeled as mathematical objects with:
/// - State: Current configuration and data
/// - Operations: Available transformations
/// - Constraints: Safety and validity rules
/// - Invariants: Properties that must always hold
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceModel {
    /// Unique device identifier
    pub device_id: String,
    /// Device type classification
    pub device_type: DeviceType,
    /// Current device state as mathematical properties
    pub state: DeviceState,
    /// Available operations on this device
    pub operations: Vec<DeviceOperation>,
    /// Safety constraints that must be maintained
    pub constraints: Vec<SafetyConstraint>,
    /// Mathematical invariants that must hold
    pub invariants: Vec<DeviceInvariant>,
    /// Platform-specific implementation details (hidden from language level)
    pub platform_impl: PlatformImplementation,
}

/// Classification of hardware device types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceType {
    /// Memory management unit
    MemoryController,
    /// Input/output controller
    IoController,
    /// Timer and clock devices
    TimerDevice,
    /// Interrupt controller
    InterruptController,
    /// Network interface (offline-first compliant)
    NetworkInterface,
    /// Storage device
    StorageDevice,
    /// Display/graphics device
    DisplayDevice,
    /// Audio device
    AudioDevice,
    /// Sensor device
    SensorDevice,
    /// Custom device type
    Custom(String),
}

/// Mathematical state representation of a device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceState {
    /// Named properties with their current values
    pub properties: HashMap<String, StateValue>,
    /// State transition history for deterministic behavior
    pub transition_history: Vec<StateTransition>,
    /// Current state validity
    pub is_valid: bool,
    /// State checksum for integrity verification
    pub checksum: u64,
}

/// Typed values that can be stored in device state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateValue {
    /// Boolean flag
    Boolean(bool),
    /// Integer value with range constraints
    Integer { value: i64, min: i64, max: i64 },
    /// Floating-point value with precision constraints
    Float { value: f64, precision: u32 },
    /// String value with length constraints
    String { value: String, max_length: usize },
    /// Binary data with size constraints
    Binary { data: Vec<u8>, max_size: usize },
    /// Enumerated value from a fixed set
    Enum { value: String, valid_values: Vec<String> },
}

/// Record of a state transition for deterministic behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    /// Timestamp of the transition
    pub timestamp: u64,
    /// Operation that caused the transition
    pub operation: String,
    /// Previous state checksum
    pub previous_checksum: u64,
    /// New state checksum
    pub new_checksum: u64,
    /// Transition validity
    pub is_valid: bool,
}

/// Safe operation that can be performed on a device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceOperation {
    /// Operation name
    pub name: String,
    /// Operation description for human understanding
    pub description: String,
    /// Input parameters with their types and constraints
    pub parameters: Vec<OperationParameter>,
    /// Expected output type
    pub output_type: Option<StateValue>,
    /// Preconditions that must be met
    pub preconditions: Vec<String>,
    /// Postconditions that will be established
    pub postconditions: Vec<String>,
    /// Side effects on device state
    pub side_effects: Vec<String>,
    /// Deterministic behavior guarantee
    pub is_deterministic: bool,
}

/// Parameter for a device operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationParameter {
    /// Parameter name
    pub name: String,
    /// Parameter type and constraints
    pub param_type: StateValue,
    /// Whether parameter is required
    pub required: bool,
    /// Default value if optional
    pub default_value: Option<StateValue>,
}

/// Safety constraint that must be maintained
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyConstraint {
    /// Constraint identifier
    pub id: String,
    /// Human-readable description
    pub description: String,
    /// Mathematical expression of the constraint
    pub expression: String,
    /// Severity if violated
    pub severity: ConstraintSeverity,
    /// Whether constraint can be checked at compile time
    pub compile_time_checkable: bool,
}

/// Severity levels for constraint violations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintSeverity {
    /// Must never be violated - causes compilation error
    Critical,
    /// Should not be violated - causes warning
    Warning,
    /// Performance hint - causes note
    Hint,
}

/// Mathematical invariant that must always hold
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInvariant {
    /// Invariant identifier
    pub id: String,
    /// Human-readable description
    pub description: String,
    /// Mathematical expression of the invariant
    pub expression: String,
    /// Whether invariant can be verified statically
    pub statically_verifiable: bool,
    /// Proof or verification method
    pub verification_method: String,
}

/// Platform-specific implementation details (hidden from language level)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformImplementation {
    /// Platform identifier (e.g., "x86_64-linux", "aarch64-macos")
    pub platform_id: String,
    /// Implementation-specific configuration
    pub config: HashMap<String, String>,
    /// Whether direct register access is available (should always be false)
    pub direct_register_access: bool,
    /// Safe abstraction layer functions
    pub abstraction_functions: Vec<String>,
}

/// Platform Abstraction Layer (PAL) - the core of hardware abstraction
#[derive(Debug, Clone)]
pub struct PlatformAbstractionLayer {
    /// Registered device models
    devices: HashMap<String, DeviceModel>,
    /// Platform-specific configuration
    platform_config: PlatformConfiguration,
    /// Safety analyzer for hardware operations
    safety_analyzer: HardwareSafetyAnalyzer,
    /// Deterministic behavior enforcer
    determinism_enforcer: DeterminismEnforcer,
}

/// Platform-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformConfiguration {
    /// Target platform identifier
    pub platform_id: String,
    /// Architecture-specific settings
    pub arch_settings: HashMap<String, String>,
    /// Available hardware features
    pub hardware_features: Vec<String>,
    /// Safety level configuration
    pub safety_level: SafetyLevel,
    /// Deterministic mode enabled
    pub deterministic_mode: bool,
}

/// Safety levels for hardware interaction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SafetyLevel {
    /// Maximum safety - no direct hardware access
    Maximum,
    /// High safety - limited controlled access
    High,
    /// Medium safety - more access with validation
    Medium,
    /// Low safety - minimal restrictions (not recommended)
    Low,
}

/// Hardware safety analyzer
#[derive(Debug, Clone)]
pub struct HardwareSafetyAnalyzer {
    /// Safety rules database
    pub safety_rules: Vec<SafetyRule>,
    /// Violation history for learning
    pub violation_history: Vec<SafetyViolation>,
}

/// Safety rule for hardware operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyRule {
    /// Rule identifier
    pub id: String,
    /// Rule description
    pub description: String,
    /// Conditions that trigger this rule
    pub conditions: Vec<String>,
    /// Actions to take when rule is triggered
    pub actions: Vec<SafetyAction>,
    /// Rule priority
    pub priority: u32,
}

/// Actions to take when safety rule is triggered
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SafetyAction {
    /// Block the operation
    Block,
    /// Issue a warning
    Warn,
    /// Log the operation
    Log,
    /// Request user confirmation
    Confirm,
    /// Apply automatic fix
    AutoFix(String),
}

/// Record of a safety violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyViolation {
    /// Timestamp of violation
    pub timestamp: u64,
    /// Rule that was violated
    pub rule_id: String,
    /// Operation that caused violation
    pub operation: String,
    /// Severity of violation
    pub severity: ConstraintSeverity,
    /// Action taken
    pub action_taken: SafetyAction,
}

/// Determinism enforcer for consistent behavior
#[derive(Debug, Clone)]
pub struct DeterminismEnforcer {
    /// Seed for deterministic operations
    pub deterministic_seed: u64,
    /// Operation history for reproducibility
    pub operation_history: Vec<DeterministicOperation>,
    /// Current state hash
    pub state_hash: u64,
}

/// Record of a deterministic operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeterministicOperation {
    /// Operation identifier
    pub operation_id: String,
    /// Input parameters
    pub inputs: HashMap<String, String>,
    /// Output result
    pub output: String,
    /// State hash before operation
    pub pre_state_hash: u64,
    /// State hash after operation
    pub post_state_hash: u64,
    /// Timestamp (deterministic)
    pub timestamp: u64,
}

impl PlatformAbstractionLayer {
    /// Create a new PAL instance
    pub fn new(platform_config: PlatformConfiguration) -> Self {
        Self {
            devices: HashMap::new(),
            platform_config,
            safety_analyzer: HardwareSafetyAnalyzer::new(),
            determinism_enforcer: DeterminismEnforcer::new(),
        }
    }

    /// Register a new device model
    pub fn register_device(&mut self, device: DeviceModel) -> OvieResult<()> {
        // Validate device model
        self.validate_device_model(&device)?;
        
        // Check for conflicts with existing devices
        if self.devices.contains_key(&device.device_id) {
            return Err(OvieError::HardwareError(format!(
                "Device '{}' is already registered", device.device_id
            )));
        }
        
        // Register the device
        self.devices.insert(device.device_id.clone(), device);
        
        Ok(())
    }

    /// Get a device model by ID
    pub fn get_device(&self, device_id: &str) -> Option<&DeviceModel> {
        self.devices.get(device_id)
    }

    /// Execute a safe operation on a device
    pub fn execute_operation(
        &mut self,
        device_id: &str,
        operation_name: &str,
        parameters: HashMap<String, StateValue>,
    ) -> OvieResult<StateValue> {
        // First, validate the operation exists and get its details without holding a mutable borrow
        let operation = {
            let device = self.devices.get(device_id)
                .ok_or_else(|| OvieError::HardwareError(format!("Device '{}' not found", device_id)))?;

            device.operations.iter()
                .find(|op| op.name == operation_name)
                .cloned()
                .ok_or_else(|| OvieError::HardwareError(format!(
                    "Operation '{}' not found on device '{}'", operation_name, device_id
                )))?
        };

        // Validate parameters (doesn't need device)
        self.validate_operation_parameters(&operation, &parameters)?;

        // Check safety constraints (needs device but not mutably)
        {
            let device = self.devices.get(device_id)
                .ok_or_else(|| OvieError::HardwareError(format!("Device '{}' not found", device_id)))?;
            self.safety_analyzer.check_operation_safety(device, &operation, &parameters)?;
            
            // Check preconditions
            self.check_preconditions(device, &operation)?;
        }

        // Execute the operation (needs mutable device)
        let result = {
            let device = self.devices.get_mut(device_id)
                .ok_or_else(|| OvieError::HardwareError(format!("Device '{}' not found", device_id)))?;
            
            // Execute mathematical operation inline to avoid borrow conflict
            match operation.name.as_str() {
                "add" | "subtract" | "multiply" | "divide" => {
                    // Mathematical operations - get parameters from HashMap
                    let param_vec: Vec<&StateValue> = parameters.values().collect();
                    if param_vec.len() != 2 {
                        return Err(OvieError::HardwareError(
                            format!("Mathematical operation requires 2 parameters, got {}", param_vec.len())
                        ));
                    }
                    
                    let a = match param_vec[0] {
                        StateValue::Integer { value, .. } => *value as f64,
                        StateValue::Float { value, .. } => *value,
                        _ => return Err(OvieError::HardwareError("Invalid parameter type for mathematical operation".to_string())),
                    };
                    
                    let b = match param_vec[1] {
                        StateValue::Integer { value, .. } => *value as f64,
                        StateValue::Float { value, .. } => *value,
                        _ => return Err(OvieError::HardwareError("Invalid parameter type for mathematical operation".to_string())),
                    };
                    
                    let result_value = match operation.name.as_str() {
                        "add" => a + b,
                        "subtract" => a - b,
                        "multiply" => a * b,
                        "divide" => {
                            if b == 0.0 {
                                return Err(OvieError::HardwareError("Division by zero".to_string()));
                            }
                            a / b
                        }
                        _ => unreachable!(),
                    };
                    
                    StateValue::Float { value: result_value, precision: 10 }
                }
                _ => {
                    // For other operations, return a placeholder
                    StateValue::String { 
                        value: format!("Operation '{}' executed", operation.name),
                        max_length: 1000
                    }
                }
            }
        };

        // Verify postconditions (needs device but not mutably)
        {
            let device = self.devices.get(device_id)
                .ok_or_else(|| OvieError::HardwareError(format!("Device '{}' not found", device_id)))?;
            self.verify_postconditions(device, &operation)?;
        }

        // Update determinism enforcer
        self.determinism_enforcer.record_operation(device_id, operation_name, &result);

        Ok(result)
    }

    /// Validate a device model
    fn validate_device_model(&self, device: &DeviceModel) -> OvieResult<()> {
        // Check that device doesn't allow direct register access
        if device.platform_impl.direct_register_access {
            return Err(OvieError::HardwareError(
                "Direct register access is not allowed in device models".to_string()
            ));
        }

        // Validate all operations are deterministic if required
        if self.platform_config.deterministic_mode {
            for operation in &device.operations {
                if !operation.is_deterministic {
                    return Err(OvieError::HardwareError(format!(
                        "Non-deterministic operation '{}' not allowed in deterministic mode",
                        operation.name
                    )));
                }
            }
        }

        // Validate safety constraints
        for constraint in &device.constraints {
            if constraint.severity == ConstraintSeverity::Critical && !constraint.compile_time_checkable {
                return Err(OvieError::HardwareError(format!(
                    "Critical constraint '{}' must be compile-time checkable",
                    constraint.id
                )));
            }
        }

        Ok(())
    }

    /// Validate operation parameters
    fn validate_operation_parameters(
        &self,
        operation: &DeviceOperation,
        parameters: &HashMap<String, StateValue>,
    ) -> OvieResult<()> {
        // Check required parameters
        for param in &operation.parameters {
            if param.required && !parameters.contains_key(&param.name) {
                return Err(OvieError::HardwareError(format!(
                    "Required parameter '{}' missing for operation '{}'",
                    param.name, operation.name
                )));
            }
        }

        // Validate parameter types and constraints
        for (name, value) in parameters {
            if let Some(param) = operation.parameters.iter().find(|p| p.name == *name) {
                self.validate_state_value_constraints(value, &param.param_type)?;
            }
        }

        Ok(())
    }

    /// Validate state value constraints
    fn validate_state_value_constraints(&self, value: &StateValue, constraint: &StateValue) -> OvieResult<()> {
        match (value, constraint) {
            (StateValue::Integer { value: v, .. }, StateValue::Integer { min, max, .. }) => {
                if *v < *min || *v > *max {
                    return Err(OvieError::HardwareError(format!(
                        "Integer value {} out of range [{}, {}]", v, min, max
                    )));
                }
            }
            (StateValue::String { value: v, .. }, StateValue::String { max_length, .. }) => {
                if v.len() > *max_length {
                    return Err(OvieError::HardwareError(format!(
                        "String length {} exceeds maximum {}", v.len(), max_length
                    )));
                }
            }
            (StateValue::Binary { data, .. }, StateValue::Binary { max_size, .. }) => {
                if data.len() > *max_size {
                    return Err(OvieError::HardwareError(format!(
                        "Binary data size {} exceeds maximum {}", data.len(), max_size
                    )));
                }
            }
            (StateValue::Enum { value: v, .. }, StateValue::Enum { valid_values, .. }) => {
                if !valid_values.contains(v) {
                    return Err(OvieError::HardwareError(format!(
                        "Enum value '{}' not in valid set: {:?}", v, valid_values
                    )));
                }
            }
            _ => {} // Other combinations are valid or will be caught by type system
        }
        Ok(())
    }

    /// Check operation preconditions
    fn check_preconditions(&self, device: &DeviceModel, operation: &DeviceOperation) -> OvieResult<()> {
        for precondition in &operation.preconditions {
            // In a full implementation, this would evaluate the precondition expression
            // For now, we'll assume all preconditions are met
            if precondition.contains("FAIL") {
                return Err(OvieError::HardwareError(format!(
                    "Precondition failed: {}", precondition
                )));
            }
        }
        Ok(())
    }

    /// Verify operation postconditions
    fn verify_postconditions(&self, device: &DeviceModel, operation: &DeviceOperation) -> OvieResult<()> {
        for postcondition in &operation.postconditions {
            // In a full implementation, this would verify the postcondition
            // For now, we'll assume all postconditions are satisfied
            if postcondition.contains("FAIL") {
                return Err(OvieError::HardwareError(format!(
                    "Postcondition failed: {}", postcondition
                )));
            }
        }
        Ok(())
    }

    /// Execute mathematical operation (no direct hardware access)
    fn execute_mathematical_operation(
        &self,
        device: &mut DeviceModel,
        operation: &DeviceOperation,
        parameters: HashMap<String, StateValue>,
    ) -> OvieResult<StateValue> {
        // This is where the mathematical transformation happens
        // No direct register access - only mathematical operations on device state
        
        match operation.name.as_str() {
            "read_property" => {
                if let Some(StateValue::String { value: prop_name, .. }) = parameters.get("property") {
                    device.state.properties.get(prop_name)
                        .cloned()
                        .ok_or_else(|| OvieError::HardwareError(format!("Property '{}' not found", prop_name)))
                } else {
                    Err(OvieError::HardwareError("Invalid property name parameter".to_string()))
                }
            }
            "write_property" => {
                if let (Some(StateValue::String { value: prop_name, .. }), Some(prop_value)) = 
                    (parameters.get("property"), parameters.get("value")) {
                    device.state.properties.insert(prop_name.clone(), prop_value.clone());
                    device.state.checksum = self.compute_state_checksum(&device.state);
                    Ok(StateValue::Boolean(true))
                } else {
                    Err(OvieError::HardwareError("Invalid parameters for write_property".to_string()))
                }
            }
            "get_state" => {
                Ok(StateValue::String { 
                    value: format!("Device state: {} properties", device.state.properties.len()),
                    max_length: 1000 
                })
            }
            _ => {
                // For unknown operations, return a default success value
                Ok(StateValue::Boolean(true))
            }
        }
    }

    /// Compute state checksum for integrity verification
    fn compute_state_checksum(&self, state: &DeviceState) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        
        // Hash all properties in a deterministic order
        let mut keys: Vec<_> = state.properties.keys().collect();
        keys.sort();
        
        for key in keys {
            key.hash(&mut hasher);
            if let Some(value) = state.properties.get(key) {
                // Hash the value (simplified - would need proper StateValue hashing)
                format!("{:?}", value).hash(&mut hasher);
            }
        }
        
        hasher.finish()
    }

    /// Get platform configuration
    pub fn platform_config(&self) -> &PlatformConfiguration {
        &self.platform_config
    }

    /// Get safety analyzer
    pub fn safety_analyzer(&self) -> &HardwareSafetyAnalyzer {
        &self.safety_analyzer
    }

    /// Get determinism enforcer
    pub fn determinism_enforcer(&self) -> &DeterminismEnforcer {
        &self.determinism_enforcer
    }
}

// Include implementation details from sibling modules
pub use crate::hardware_impl::*;

// Include safety and determinism extensions from sibling modules
pub use crate::hardware_safety::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_model_creation() {
        let device = DeviceModel::new("test_device".to_string(), DeviceType::Custom("test".to_string()));
        assert_eq!(device.device_id, "test_device");
        assert_eq!(device.device_type, DeviceType::Custom("test".to_string()));
        assert!(device.operations.is_empty());
        assert!(device.constraints.is_empty());
        assert!(device.invariants.is_empty());
    }

    #[test]
    fn test_pal_device_registration() {
        let config = PlatformConfiguration {
            platform_id: "test_platform".to_string(),
            arch_settings: HashMap::new(),
            hardware_features: vec![],
            safety_level: SafetyLevel::Maximum,
            deterministic_mode: true,
        };
        
        let mut pal = PlatformAbstractionLayer::new(config);
        let device = DeviceFactory::create_memory_controller();
        
        assert!(pal.register_device(device).is_ok());
        assert!(pal.get_device("memory_controller_0").is_some());
    }

    #[test]
    fn test_safety_analyzer() {
        let mut analyzer = HardwareSafetyAnalyzer::new();
        let device = DeviceFactory::create_memory_controller();
        let operation = &device.operations[0];
        let parameters = HashMap::new();
        
        // Should pass safety checks for normal operations
        assert!(analyzer.check_operation_safety(&device, operation, &parameters).is_ok());
    }

    #[test]
    fn test_determinism_enforcer() {
        let mut enforcer = DeterminismEnforcer::new();
        let result = StateValue::Boolean(true);
        
        enforcer.record_operation("test_device", "test_operation", &result);
        assert_eq!(enforcer.operation_history().len(), 1);
        assert_eq!(enforcer.operation_history()[0].operation_id, "test_device::test_operation");
    }

    #[test]
    fn test_device_factory() {
        let memory_controller = DeviceFactory::create_memory_controller();
        assert_eq!(memory_controller.device_type, DeviceType::MemoryController);
        assert!(!memory_controller.operations.is_empty());
        assert!(!memory_controller.constraints.is_empty());
        assert!(!memory_controller.invariants.is_empty());

        let timer = DeviceFactory::create_timer_device();
        assert_eq!(timer.device_type, DeviceType::TimerDevice);
        assert!(!timer.operations.is_empty());

        let io_controller = DeviceFactory::create_io_controller();
        assert_eq!(io_controller.device_type, DeviceType::IoController);
        assert!(!io_controller.operations.is_empty());
    }

    #[test]
    fn test_state_value_constraints() {
        let integer_value = StateValue::Integer { value: 50, min: 0, max: 100 };
        let constraint = StateValue::Integer { value: 0, min: 0, max: 100 };
        
        let config = PlatformConfiguration {
            platform_id: "test".to_string(),
            arch_settings: HashMap::new(),
            hardware_features: vec![],
            safety_level: SafetyLevel::Maximum,
            deterministic_mode: true,
        };
        
        let pal = PlatformAbstractionLayer::new(config);
        assert!(pal.validate_state_value_constraints(&integer_value, &constraint).is_ok());
        
        let invalid_value = StateValue::Integer { value: 150, min: 0, max: 100 };
        assert!(pal.validate_state_value_constraints(&invalid_value, &constraint).is_err());
    }

    #[test]
    fn test_no_direct_register_access() {
        let config = PlatformConfiguration {
            platform_id: "test".to_string(),
            arch_settings: HashMap::new(),
            hardware_features: vec![],
            safety_level: SafetyLevel::Maximum,
            deterministic_mode: true,
        };
        
        let pal = PlatformAbstractionLayer::new(config);
        
        // Create a device that tries to allow direct register access (should fail)
        let mut bad_device = DeviceModel::new("bad_device".to_string(), DeviceType::Custom("test".to_string()));
        bad_device.platform_impl.direct_register_access = true;
        
        assert!(pal.validate_device_model(&bad_device).is_err());
    }

    #[test]
    fn test_deterministic_mode_validation() {
        let config = PlatformConfiguration {
            platform_id: "test".to_string(),
            arch_settings: HashMap::new(),
            hardware_features: vec![],
            safety_level: SafetyLevel::Maximum,
            deterministic_mode: true,
        };
        
        let pal = PlatformAbstractionLayer::new(config);
        
        // Create a device with non-deterministic operation (should fail in deterministic mode)
        let mut device = DeviceModel::new("test_device".to_string(), DeviceType::Custom("test".to_string()));
        device.add_operation(DeviceOperation {
            name: "non_deterministic_op".to_string(),
            description: "A non-deterministic operation".to_string(),
            parameters: vec![],
            output_type: None,
            preconditions: vec![],
            postconditions: vec![],
            side_effects: vec![],
            is_deterministic: false, // This should cause validation to fail
        });
        
        assert!(pal.validate_device_model(&device).is_err());
    }

    #[test]
    fn test_mathematical_operations() {
        let config = PlatformConfiguration {
            platform_id: "test".to_string(),
            arch_settings: HashMap::new(),
            hardware_features: vec![],
            safety_level: SafetyLevel::Maximum,
            deterministic_mode: true,
        };
        
        let mut pal = PlatformAbstractionLayer::new(config);
        let mut device = DeviceModel::new("test_device".to_string(), DeviceType::Custom("test".to_string()));
        
        // Add a simple property read operation
        device.add_operation(DeviceOperation {
            name: "read_property".to_string(),
            description: "Read a device property".to_string(),
            parameters: vec![
                OperationParameter {
                    name: "property".to_string(),
                    param_type: StateValue::String { value: "".to_string(), max_length: 100 },
                    required: true,
                    default_value: None,
                },
            ],
            output_type: Some(StateValue::String { value: "".to_string(), max_length: 1000 }),
            preconditions: vec![],
            postconditions: vec![],
            side_effects: vec![],
            is_deterministic: true,
        });
        
        // Set a property in the device state
        device.state.set_property("test_prop".to_string(), StateValue::String { 
            value: "test_value".to_string(), 
            max_length: 100 
        });
        
        assert!(pal.register_device(device).is_ok());
        
        // Execute the mathematical operation (no direct hardware access)
        let mut params = HashMap::new();
        params.insert("property".to_string(), StateValue::String { 
            value: "test_prop".to_string(), 
            max_length: 100 
        });
        
        let result = pal.execute_operation("test_device", "read_property", params);
        assert!(result.is_ok());
        
        if let Ok(StateValue::String { value, .. }) = result {
            assert_eq!(value, "test_value");
        } else {
            panic!("Expected string result");
        }
    }

    #[test]
    fn test_property_15_hardware_abstraction_safety() {
        // Property 15: Hardware Abstraction Safety
        // **Validates: Requirements 9.1, 9.2, 9.3, 9.4**
        
        // Test 15.1: No direct register access
        let device = DeviceFactory::create_memory_controller();
        
        // Verify no operations suggest direct hardware access
        for operation in &device.operations {
            assert!(!operation.name.contains("register"));
            assert!(!operation.name.contains("direct"));
            assert!(!operation.name.contains("raw"));
            assert!(!operation.name.contains("unsafe"));
        }
        
        // Test 15.2: Mathematical device modeling
        // All operations should be mathematical abstractions
        for operation in &device.operations {
            assert!(operation.name.contains("allocate") || 
                   operation.name.contains("read") ||
                   operation.name.contains("write") ||
                   operation.name.contains("safe") ||
                   operation.name.contains("mathematical") ||
                   operation.name.starts_with("custom_"));
        }
        
        // Test 15.3: Safety constraint consistency
        // Critical constraints must be compile-time checkable
        for constraint in &device.constraints {
            if constraint.severity == ConstraintSeverity::Critical {
                assert!(constraint.compile_time_checkable, 
                       "Critical constraint '{}' must be compile-time checkable", constraint.id);
            }
        }
        
        // Test 15.4: Deterministic behavior enforcement
        let config = PlatformConfiguration {
            platform_id: "test_deterministic".to_string(),
            arch_settings: HashMap::new(),
            hardware_features: vec![],
            safety_level: SafetyLevel::Maximum,
            deterministic_mode: true,
        };
        
        let pal = PlatformAbstractionLayer::new(config);
        
        // In deterministic mode, all operations must be deterministic
        let mut test_device = DeviceModel::new("test_device".to_string(), DeviceType::Custom("test".to_string()));
        test_device.add_operation(DeviceOperation {
            name: "deterministic_op".to_string(),
            description: "A deterministic operation".to_string(),
            parameters: vec![],
            output_type: None,
            preconditions: vec![],
            postconditions: vec![],
            side_effects: vec![],
            is_deterministic: true,
        });
        
        assert!(pal.validate_device_model(&test_device).is_ok());
        
        // Non-deterministic operations should be rejected in deterministic mode
        let mut non_det_device = DeviceModel::new("non_det_device".to_string(), DeviceType::Custom("test".to_string()));
        non_det_device.add_operation(DeviceOperation {
            name: "non_deterministic_op".to_string(),
            description: "A non-deterministic operation".to_string(),
            parameters: vec![],
            output_type: None,
            preconditions: vec![],
            postconditions: vec![],
            side_effects: vec![],
            is_deterministic: false,
        });
        
        assert!(pal.validate_device_model(&non_det_device).is_err());
        
        // Test 15.5: Platform abstraction safety
        // Maximum safety level should enforce strict constraints
        let max_safety_config = PlatformConfiguration {
            platform_id: "max_safety".to_string(),
            arch_settings: HashMap::new(),
            hardware_features: vec![],
            safety_level: SafetyLevel::Maximum,
            deterministic_mode: true, // Should be required for maximum safety
        };
        
        let max_safety_pal = PlatformAbstractionLayer::new(max_safety_config);
        assert_eq!(max_safety_pal.platform_config().safety_level, SafetyLevel::Maximum);
        assert!(max_safety_pal.platform_config().deterministic_mode);
        
        // Test 15.6: Hardware consistency validation
        let mut behavior_analyzer = HardwareBehaviorAnalyzer::new();
        
        // Add similar hardware configurations
        let config1 = HardwareConfiguration {
            config_id: "similar_config_1".to_string(),
            platform: "x86_64-linux".to_string(),
            cpu_arch: CpuArchitecture {
                name: "x86_64".to_string(),
                extensions: vec!["sse2".to_string()],
                cache_levels: vec![],
                execution_units: vec![],
                endianness: Endianness::Little,
            },
            memory_config: MemoryConfiguration {
                total_memory_bytes: 8 * 1024 * 1024 * 1024,
                page_size_bytes: 4096,
                alignment_bytes: 8,
                protection_features: vec![],
                numa_nodes: vec![],
            },
            features: vec!["deterministic_math".to_string()],
            performance_profile: PerformanceProfile {
                cpu_frequency_hz: 3_000_000_000,
                memory_bandwidth_bps: 25_600_000_000,
                instruction_latencies: HashMap::new(),
                cache_miss_penalties: HashMap::new(),
                io_throughput: HashMap::new(),
            },
            determinism_guarantees: vec![],
        };
        
        let config2 = HardwareConfiguration {
            config_id: "similar_config_2".to_string(),
            platform: "x86_64-linux".to_string(),
            cpu_arch: CpuArchitecture {
                name: "x86_64".to_string(),
                extensions: vec!["sse2".to_string()],
                cache_levels: vec![],
                execution_units: vec![],
                endianness: Endianness::Little,
            },
            memory_config: MemoryConfiguration {
                total_memory_bytes: 8 * 1024 * 1024 * 1024,
                page_size_bytes: 4096,
                alignment_bytes: 8,
                protection_features: vec![],
                numa_nodes: vec![],
            },
            features: vec!["deterministic_math".to_string()],
            performance_profile: PerformanceProfile {
                cpu_frequency_hz: 3_000_000_000,
                memory_bandwidth_bps: 25_600_000_000,
                instruction_latencies: HashMap::new(),
                cache_miss_penalties: HashMap::new(),
                io_throughput: HashMap::new(),
            },
            determinism_guarantees: vec![],
        };
        
        behavior_analyzer.add_hardware_config(config1);
        behavior_analyzer.add_hardware_config(config2);
        
        let configs = vec!["similar_config_1".to_string(), "similar_config_2".to_string()];
        assert!(behavior_analyzer.validate_hardware_consistency(&configs).unwrap());
        
        // Test 15.7: Analyzable hardware models
        let mut automated_analyzer = AutomatedHardwareAnalyzer::new();
        let analysis = automated_analyzer.analyze_model(&device).unwrap();
        
        // Model should be analyzable (high determinism score, few violations)
        assert!(analysis.determinism_score >= 0.8);
        
        // Test 15.8: Cross-platform determinism
        // Record consistent behavior across similar configurations
        let observation1 = BehaviorObservation {
            timestamp: 1000,
            config_id: "similar_config_1".to_string(),
            operation: "allocate_memory".to_string(),
            inputs: [("size".to_string(), "1024".to_string())].iter().cloned().collect(),
            output: "0x1000".to_string(),
            execution_time_ns: 100,
            anomalies: vec![],
        };
        
        let observation2 = BehaviorObservation {
            timestamp: 1001,
            config_id: "similar_config_2".to_string(),
            operation: "allocate_memory".to_string(),
            inputs: [("size".to_string(), "1024".to_string())].iter().cloned().collect(),
            output: "0x1000".to_string(), // Same output for determinism
            execution_time_ns: 105,
            anomalies: vec![],
        };
        
        assert!(behavior_analyzer.record_observation(observation1).is_ok());
        assert!(behavior_analyzer.record_observation(observation2).is_ok());
        
        let determinism_analysis = behavior_analyzer.analyze_determinism("allocate_memory").unwrap();
        assert!(determinism_analysis.determinism_score >= 0.95); // Should be highly deterministic
        
        println!("✓ Property 15: Hardware Abstraction Safety - All tests passed");
    }
}
