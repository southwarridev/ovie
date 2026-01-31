//! Simple test to verify the testing framework structure

use crate::{Compiler, OvieResult};

#[test]
fn test_basic_compilation() -> OvieResult<()> {
    let mut compiler = Compiler::new();
    let source = "let x = 42;";
    
    let _ast = compiler.compile_to_ast(source)?;
    
    Ok(())
}

#[test]
fn test_test_framework_structure() {
    // Just verify that the test framework modules are accessible
    use crate::tests::{TestRunner, TestSuiteConfig};
    
    let config = TestSuiteConfig::default();
    let _runner = TestRunner::with_config(config);
    
    // Test passes if we can create the structures
    assert!(true);
}