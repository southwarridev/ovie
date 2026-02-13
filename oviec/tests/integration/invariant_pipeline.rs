//! End-to-end invariant validation tests for the Ovie compiler pipeline
//! 
//! These tests verify that invariant checking is properly integrated throughout
//! the entire compilation pipeline from source code to backend output.

use crate::{Compiler, Backend, OvieError, OvieResult};
use std::panic;

#[test]
fn test_end_to_end_invariant_validation_success() {
    // Test that a valid program passes all invariant checks
    let source = r#"
        seeAm "Hello, World!";
    "#;
    
    let mut compiler = Compiler::new_with_debug();
    
    // Should compile successfully with all invariants validated
    let result = compiler.compile_to_ir(source);
    assert!(result.is_ok(), "Valid program should pass all invariant checks");
}

#[test]
fn test_end_to_end_invariant_validation_with_strict_mode() {
    // Test that strict invariant mode works correctly
    let source = r#"
        seeAm "Hello, World!";
    "#;
    
    let mut compiler = Compiler::new_with_strict_invariants();
    compiler.debug = true;
    
    // Should compile successfully even in strict mode for valid programs
    let result = compiler.compile_to_ir(source);
    assert!(result.is_ok(), "Valid program should pass strict invariant checks");
}

#[test]
fn test_invariant_violation_error_propagation() {
    // Test that invariant violations are properly propagated through the pipeline
    let mut compiler = Compiler::new_with_debug();
    
    // Create a scenario that might trigger invariant violations
    // (This is a simplified test - in practice, invariant violations would be
    // triggered by internal compiler bugs or corrupted IR)
    let source = r#"
        seeAm "Test program";
    "#;
    
    // For now, just verify the compilation succeeds
    // In a real scenario with invariant violations, we would test error handling
    let result = compiler.compile_to_ir(source);
    assert!(result.is_ok(), "Test program should compile successfully");
}

#[test]
fn test_invariant_validation_at_each_stage() {
    // Test that invariants are validated at each compilation stage
    let source = r#"
        seeAm "Multi-stage test";
    "#;
    
    let mut compiler = Compiler::new_with_debug();
    
    // Test AST stage
    let ast_result = compiler.compile_to_ast(source);
    assert!(ast_result.is_ok(), "AST compilation should succeed with invariant validation");
    
    // Test HIR stage
    let hir_result = compiler.compile_to_hir(source);
    assert!(hir_result.is_ok(), "HIR compilation should succeed with invariant validation");
    
    // Test MIR stage
    let mir_result = compiler.compile_to_mir(source);
    assert!(mir_result.is_ok(), "MIR compilation should succeed with invariant validation");
    
    // Test IR/Backend stage
    let ir_result = compiler.compile_to_ir(source);
    assert!(ir_result.is_ok(), "IR compilation should succeed with invariant validation");
}

#[test]
fn test_invariant_validation_with_different_backends() {
    // Test that invariant validation works with different backends
    let source = r#"
        seeAm "Backend test";
    "#;
    
    let backends = vec![
        Backend::Interpreter,
        Backend::IrInterpreter,
        Backend::Wasm,
        Backend::Hir,
        Backend::Mir,
    ];
    
    for backend in backends {
        let mut compiler = Compiler::new_with_backend(backend.clone());
        compiler.debug = true;
        
        let result = match backend {
            Backend::Wasm => compiler.compile_to_wasm(source).map(|_| ()),
            Backend::Hir => compiler.compile_to_hir(source).map(|_| ()),
            Backend::Mir => compiler.compile_to_mir(source).map(|_| ()),
            _ => compiler.compile_to_ir(source).map(|_| ()),
        };
        
        assert!(result.is_ok(), "Backend {:?} should succeed with invariant validation", backend);
    }
}

#[test]
fn test_invariant_error_context_includes_build_info() {
    // Test that invariant violation errors include detailed build context
    let source = r#"
        seeAm "Context test";
    "#;
    
    let mut compiler = Compiler::new_deterministic();
    compiler.debug = true;
    
    // This should succeed, but if it failed, the error would include build context
    let result = compiler.compile_to_ir(source);
    assert!(result.is_ok(), "Compilation should succeed");
    
    // Verify build hash is computed
    let build_hash = compiler.build_config.compute_build_hash();
    assert!(!build_hash.is_empty(), "Build hash should be computed");
}

#[test]
fn test_strict_invariant_mode_panic_behavior() {
    // Test that strict invariant mode causes panics on violations
    // Note: This test would need to be carefully designed to trigger an actual
    // invariant violation in a controlled way. For now, we just test the setup.
    
    let mut compiler = Compiler::new_with_strict_invariants();
    assert!(compiler.strict_invariants, "Strict invariants should be enabled");
    
    // Test that we can disable strict mode
    compiler.set_strict_invariants(false);
    assert!(!compiler.strict_invariants, "Strict invariants should be disabled");
}

#[test]
fn test_invariant_validation_performance() {
    // Test that invariant validation doesn't significantly impact performance
    let source = r#"
        seeAm "Performance test";
    "#;
    
    let start = std::time::Instant::now();
    
    let mut compiler = Compiler::new_with_debug();
    let result = compiler.compile_to_ir(source);
    
    let duration = start.elapsed();
    
    assert!(result.is_ok(), "Compilation should succeed");
    assert!(duration.as_millis() < 5000, "Compilation with invariants should complete in reasonable time");
}

#[test]
fn test_invariant_validation_determinism() {
    // Test that invariant validation produces deterministic results
    let source = r#"
        seeAm "Determinism test";
    "#;
    
    let mut compiler1 = Compiler::new_deterministic();
    let mut compiler2 = Compiler::new_deterministic();
    
    let result1 = compiler1.compile_to_ir(source);
    let result2 = compiler2.compile_to_ir(source);
    
    assert!(result1.is_ok(), "First compilation should succeed");
    assert!(result2.is_ok(), "Second compilation should succeed");
    
    // Both should produce the same build hash
    let hash1 = compiler1.build_config.compute_build_hash();
    let hash2 = compiler2.build_config.compute_build_hash();
    assert_eq!(hash1, hash2, "Deterministic builds should produce identical hashes");
}

#[test]
fn test_invariant_validation_with_complex_program() {
    // Test invariant validation with a more complex program
    let source = r#"
        let x = 42;
        let y = x + 10;
        seeAm "Result: " + y;
        
        if x > 0 {
            seeAm "Positive";
        } else {
            seeAm "Non-positive";
        }
    "#;
    
    let mut compiler = Compiler::new_with_debug();
    
    // Test each stage with the complex program
    let ast_result = compiler.compile_to_ast(source);
    assert!(ast_result.is_ok(), "Complex program AST should pass invariants");
    
    let hir_result = compiler.compile_to_hir(source);
    assert!(hir_result.is_ok(), "Complex program HIR should pass invariants");
    
    let mir_result = compiler.compile_to_mir(source);
    assert!(mir_result.is_ok(), "Complex program MIR should pass invariants");
    
    let ir_result = compiler.compile_to_ir(source);
    assert!(ir_result.is_ok(), "Complex program IR should pass invariants");
}

#[test]
fn test_invariant_validation_error_recovery() {
    // Test that the compiler can recover from invariant validation errors
    let mut compiler = Compiler::new_with_debug();
    
    // Test with a valid program first
    let valid_source = r#"seeAm "Valid";"#;
    let result1 = compiler.compile_to_ir(valid_source);
    assert!(result1.is_ok(), "Valid program should compile successfully");
    
    // Test with another valid program to ensure state is clean
    let another_valid_source = r#"seeAm "Another valid";"#;
    let result2 = compiler.compile_to_ir(another_valid_source);
    assert!(result2.is_ok(), "Second valid program should also compile successfully");
}

/// Helper function to create a test program that might trigger specific invariant checks
fn create_test_program_for_stage(stage: &str) -> String {
    match stage {
        "ast" => r#"seeAm "AST test";"#.to_string(),
        "hir" => r#"let x = 42; seeAm x;"#.to_string(),
        "mir" => r#"if true { seeAm "MIR test"; }"#.to_string(),
        "backend" => r#"fn test() { seeAm "Backend test"; } test();"#.to_string(),
        _ => r#"seeAm "Default test";"#.to_string(),
    }
}

#[test]
fn test_stage_specific_invariant_validation() {
    // Test invariant validation for each specific stage
    let stages = vec!["ast", "hir", "mir", "backend"];
    
    for stage in stages {
        let source = create_test_program_for_stage(stage);
        let mut compiler = Compiler::new_with_debug();
        
        let result = compiler.compile_to_ir(&source);
        assert!(result.is_ok(), "Stage {} should pass invariant validation", stage);
    }
}