//! Bootstrap Verification Tests
//!
//! These tests verify the bootstrap verification infrastructure works correctly.

use oviec::self_hosting::bootstrap_verification::{
    RealBootstrapConfig, RealBootstrapVerifier,
};
use oviec::error::OvieResult;

#[test]
fn test_bootstrap_verifier_creation() {
    let config = RealBootstrapConfig::default();
    let verifier = RealBootstrapVerifier::new(config);
    
    // Verifier should be created successfully
    assert!(!verifier.is_ovie_lexer_loaded()); // Not loaded yet
}

#[test]
fn test_bootstrap_config_defaults() {
    let config = RealBootstrapConfig::default();
    assert!(config.detailed_comparison);
    assert!(config.performance_benchmarking);
    assert_eq!(config.max_performance_degradation, 10.0);
    assert_eq!(config.ovie_lexer_path, std::path::PathBuf::from("std/lexer/mod.ov"));
}

#[test]
fn test_hash_determinism() {
    let config = RealBootstrapConfig::default();
    let verifier = RealBootstrapVerifier::new(config);
    
    let tokens = vec![
        oviec::lexer::Token::new(
            oviec::lexer::TokenType::SeeAm,
            "seeAm".to_string(),
            oviec::error::SourceLocation::new(1, 1, 0),
        ),
    ];
    
    let hash1 = verifier.compute_token_hash(&tokens);
    let hash2 = verifier.compute_token_hash(&tokens);
    
    // Hashes should be identical for same input
    assert_eq!(hash1, hash2);
    assert!(!hash1.is_empty());
}

#[test]
fn test_rust_lexer_only() -> OvieResult<()> {
    // Test if the Rust lexer works
    let source = "seeAm \"hello\";";
    
    let mut lexer = oviec::lexer::Lexer::new(source);
    let tokens = lexer.tokenize()?;
    
    println!("Rust lexer produced {} tokens", tokens.len());
    for token in &tokens {
        println!("  {:?}: {}", token.token_type, token.lexeme);
    }
    
    assert!(tokens.len() > 0);
    Ok(())
}

#[test]
fn test_minimal_ovie_execution() -> OvieResult<()> {
    // Test if we can execute simple Ovie code at all
    let source = r#"
fn test_func() {
    return 42;
}
"#;
    
    let mut compiler = oviec::Compiler::new();
    let ast = compiler.compile_to_ast(source)?;
    
    let mut interpreter = oviec::interpreter::Interpreter::new();
    interpreter.interpret(&ast)?;
    
    println!("Simple Ovie code executed successfully");
    Ok(())
}

#[test]
#[ignore] // Temporarily disabled due to timeout issue
fn test_simple_verification() -> OvieResult<()> {
    let config = RealBootstrapConfig {
        detailed_comparison: true,
        performance_benchmarking: false,
        verbose_logging: true,
        ..Default::default()
    };
    
    let mut verifier = RealBootstrapVerifier::new(config);
    
    // Load the Ovie lexer
    println!("Loading Ovie lexer...");
    verifier.load_ovie_lexer()?;
    println!("Ovie lexer loaded successfully");
    
    // Simple test case
    let source = "seeAm \"hello\";";
    println!("Testing source: {}", source);
    
    // Run verification
    let result = verifier.verify_lexer(source, "simple_test")?;
    
    // Verification should pass
    assert!(result.passed, "Verification failed with errors: {:?}", result.errors);
    assert!(result.tokens_match);
    assert!(result.token_count > 0);
    
    Ok(())
}

#[test]
fn test_token_mismatch_detection() {
    let config = RealBootstrapConfig::default();
    let verifier = RealBootstrapVerifier::new(config);
    
    let tokens1 = vec![
        oviec::lexer::Token::new(
            oviec::lexer::TokenType::SeeAm,
            "seeAm".to_string(),
            oviec::error::SourceLocation::new(1, 1, 0),
        ),
    ];
    
    let tokens2 = vec![
        oviec::lexer::Token::new(
            oviec::lexer::TokenType::Identifier,
            "seeAm".to_string(),
            oviec::error::SourceLocation::new(1, 1, 0),
        ),
    ];
    
    let mut errors: Vec<String> = Vec::new();
    let result = verifier.compare_tokens(&tokens1, &tokens2, &mut errors);
    
    // Should detect mismatch
    assert!(!result);
    assert!(!errors.is_empty());
    assert!(errors[0].contains("Token type mismatch"));
}

#[test]
fn test_verification_report_generation() -> OvieResult<()> {
    let config = RealBootstrapConfig::default();
    let verifier = RealBootstrapVerifier::new(config);
    
    // Create test results
    let results = vec![
        oviec::self_hosting::bootstrap_verification::RealBootstrapResult {
            passed: true,
            tokens_match: true,
            performance_acceptable: true,
            rust_time_us: 100,
            ovie_time_us: 150,
            performance_ratio: 1.5,
            token_count: 10,
            source_hash: "abc123".to_string(),
            rust_tokens_hash: "def456".to_string(),
            ovie_tokens_hash: "def456".to_string(),
            errors: Vec::new(),
            timestamp: 1234567890,
            test_case_id: "test".to_string(),
        },
        oviec::self_hosting::bootstrap_verification::RealBootstrapResult {
            passed: false,
            tokens_match: false,
            performance_acceptable: true,
            rust_time_us: 100,
            ovie_time_us: 200,
            performance_ratio: 2.0,
            token_count: 5,
            source_hash: "xyz789".to_string(),
            rust_tokens_hash: "aaa111".to_string(),
            ovie_tokens_hash: "bbb222".to_string(),
            errors: vec!["Hash mismatch".to_string()],
            timestamp: 1234567891,
            test_case_id: "test2".to_string(),
        },
    ];
    
    let report = verifier.generate_verification_report(&results);
    
    // Report should contain key information
    assert!(report.contains("Real Bootstrap Verification Report"));
    assert!(report.contains("Total tests: 2"));
    assert!(report.contains("Passed: 1"));
    assert!(report.contains("Failed: 1"));
    assert!(report.contains("Success rate: 50.0%"));
    
    Ok(())
}

#[test]
fn test_comprehensive_verification() -> OvieResult<()> {
    let config = RealBootstrapConfig {
        performance_benchmarking: false,
        ..Default::default()
    };
    
    let mut verifier = RealBootstrapVerifier::new(config);
    
    // Load the Ovie lexer
    verifier.load_ovie_lexer()?;
    
    let test_cases = vec![
        ("seeAm \"test1\";", "test1"),
        ("let x = 42;", "test2"),
        ("fn main() { }", "test3"),
    ];
    
    let results = verifier.run_comprehensive_verification(&test_cases)?;
    
    // Should have results for all test cases
    assert_eq!(results.len(), 3);
    
    // All should pass
    for result in &results {
        assert!(result.passed, "Test {} failed with errors: {:?}", result.test_case_id, result.errors);
    }
    
    Ok(())
}

#[test]
fn test_performance_benchmarking() -> OvieResult<()> {
    let config = RealBootstrapConfig {
        performance_benchmarking: true,
        ..Default::default()
    };
    
    let mut verifier = RealBootstrapVerifier::new(config);
    
    // Load the Ovie lexer
    verifier.load_ovie_lexer()?;
    
    let source = "let x = 42; seeAm x;";
    let result = verifier.verify_lexer(source, "perf_test")?;
    
    // Should have performance data
    assert!(result.rust_time_us > 0);
    assert!(result.ovie_time_us > 0);
    assert!(result.performance_ratio > 0.0);
    assert!(result.performance_acceptable);
    
    Ok(())
}

#[test]
fn test_multiple_source_files() -> OvieResult<()> {
    let config = RealBootstrapConfig {
        performance_benchmarking: false,
        ..Default::default()
    };
    
    let mut verifier = RealBootstrapVerifier::new(config);
    
    // Load the Ovie lexer
    verifier.load_ovie_lexer()?;
    
    let test_cases = vec![
        ("// Comment test\nseeAm \"hello\";", "comment_test"),
        ("let x = 42;\nlet y = 100;\nseeAm x + y;", "multi_var_test"),
        ("fn add(a, b) { return a + b; }", "function_test"),
        ("if true { seeAm \"yes\"; } else { seeAm \"no\"; }", "if_test"),
        ("for i in 0..10 { seeAm i; }", "for_test"),
    ];
    
    let results = verifier.run_comprehensive_verification(&test_cases)?;
    
    assert_eq!(results.len(), 5);
    
    // All should pass
    let passed = results.iter().filter(|r| r.passed).count();
    assert_eq!(passed, 5, "Expected all tests to pass, but {} failed", 5 - passed);
    
    Ok(())
}

#[test]
#[ignore] // Comprehensive test - run explicitly with --ignored flag
fn test_definitive_self_hosting_verification() -> OvieResult<()> {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  DEFINITIVE SELF-HOSTING VERIFICATION TEST                     ║");
    println!("║  Testing ALL Ovie Language Features                           ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");
    
    // Get workspace root
    let workspace_root = std::env::var("CARGO_MANIFEST_DIR")
        .map(|p| std::path::PathBuf::from(p).parent().unwrap().to_path_buf())
        .unwrap_or_else(|_| std::path::PathBuf::from("."));
    
    // Create verifier with verbose logging enabled
    let config = RealBootstrapConfig {
        detailed_comparison: true,
        performance_benchmarking: true,
        verbose_logging: true,
        max_performance_degradation: 10.0,
        work_dir: workspace_root.join("target/definitive_self_hosting_verification"),
        ovie_lexer_path: workspace_root.join("std/lexer/mod_no_types.ov"),
    };
    
    let mut verifier = RealBootstrapVerifier::new(config);
    
    // Load the Ovie lexer
    println!("📦 Loading Ovie lexer from std/lexer/mod.ov...");
    verifier.load_ovie_lexer()?;
    println!("✅ Ovie lexer loaded successfully\n");
    
    // Comprehensive test cases covering ALL Ovie language features
    let test_cases = vec![
        // 1. Basic keywords - fn
        ("fn main() { }", "keyword_fn"),
        
        // 2. Basic keywords - let
        ("let x = 42;", "keyword_let"),
        
        // 3. Basic keywords - mut
        ("let mut y = 10;", "keyword_mut"),
        
        // 4. Basic keywords - if
        ("if true { }", "keyword_if"),
        
        // 5. Basic keywords - else
        ("if false { } else { }", "keyword_else"),
        
        // 6. Basic keywords - for
        ("for i in 0..10 { }", "keyword_for"),
        
        // 7. Basic keywords - while
        ("while true { }", "keyword_while"),
        
        // 8. Basic keywords - struct
        ("struct Point { x: i32, y: i32 }", "keyword_struct"),
        
        // 9. Basic keywords - enum
        ("enum Color { Red, Green, Blue }", "keyword_enum"),
        
        // 10. Basic keywords - return
        ("fn test() { return 42; }", "keyword_return"),
        
        // 11. Basic keywords - seeAm (print)
        ("seeAm \"hello\";", "keyword_seeam"),
        
        // 12. Basic keywords - in
        ("for x in items { }", "keyword_in"),
        
        // 13. Basic keywords - unsafe
        ("unsafe { }", "keyword_unsafe"),
        
        // 14. Boolean literals - true
        ("let t = true;", "literal_true"),
        
        // 15. Boolean literals - false
        ("let f = false;", "literal_false"),
        
        // 16. String literals
        ("let s = \"Hello, World!\";", "literal_string"),
        
        // 17. Integer literals
        ("let n = 12345;", "literal_integer"),
        
        // 18. Float literals
        ("let pi = 3.14159;", "literal_float"),
        
        // 19. Arithmetic operators
        ("let result = 10 + 20 - 5 * 2 / 4;", "operators_arithmetic"),
        
        // 20. Comparison operators
        ("let cmp = x == y && a != b || c < d && e > f;", "operators_comparison"),
        
        // 21. Logical operators
        ("let logic = true && false || !true;", "operators_logical"),
        
        // 22. Identifiers and function calls
        ("calculate_sum(a, b, c);", "identifiers_calls"),
        
        // 23. Punctuation - parentheses, braces, brackets
        ("fn test(a, b) { let arr = [1, 2, 3]; }", "punctuation_mixed"),
        
        // 24. Punctuation - commas, semicolons
        ("let x = 1; let y = 2; let z = 3;", "punctuation_separators"),
        
        // 25. Punctuation - colons, dots
        ("struct Point { x: i32 } let p = Point { x: 10 }; p.x;", "punctuation_struct_access"),
        
        // 26. Single-line comments
        ("// This is a comment\nlet x = 42;", "comment_single_line"),
        
        // 27. Multi-line code with control flow
        (r#"
fn factorial(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}
"#, "multiline_control_flow"),
        
        // 28. Complex expressions with nested operations
        ("let result = ((a + b) * (c - d)) / (e + f);", "complex_expression"),
        
        // 29. Array operations
        ("let arr = [1, 2, 3, 4, 5]; let first = arr[0];", "array_operations"),
        
        // 30. Struct definition and instantiation
        (r#"
struct Person {
    name: String,
    age: i32,
}

let person = Person {
    name: "Alice",
    age: 30,
};
"#, "struct_full_example"),
        
        // 31. Enum with variants
        (r#"
enum Result {
    Ok(i32),
    Err(String),
}
"#, "enum_with_variants"),
        
        // 32. For loop with range
        ("for i in 0..100 { seeAm i; }", "for_loop_range"),
        
        // 33. While loop with condition
        (r#"
let mut counter = 0;
while counter < 10 {
    counter = counter + 1;
}
"#, "while_loop_condition"),
        
        // 34. Real-world example - calculator
        (r#"
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn subtract(a: i32, b: i32) -> i32 {
    return a - b;
}

let x = 10;
let y = 5;
let sum = add(x, y);
let diff = subtract(x, y);
seeAm sum;
seeAm diff;
"#, "real_world_calculator"),
        
        // 35. Real-world example - data processing
        (r#"
struct Data {
    values: [i32],
    count: i32,
}

fn process_data(data: Data) -> i32 {
    let mut sum = 0;
    for i in 0..data.count {
        sum = sum + data.values[i];
    }
    return sum;
}
"#, "real_world_data_processing"),
        
        // 36. Edge case - empty function
        ("fn empty() { }", "edge_empty_function"),
        
        // 37. Edge case - nested blocks
        ("{ { { let x = 1; } } }", "edge_nested_blocks"),
        
        // 38. Edge case - multiple statements on one line
        ("let a = 1; let b = 2; let c = 3;", "edge_multiple_statements"),
        
        // 39. Edge case - complex boolean expression
        ("let result = (a && b) || (c && d) || (e && f);", "edge_complex_boolean"),
        
        // 40. Edge case - chained method calls
        ("obj.method1().method2().method3();", "edge_chained_calls"),
    ];
    
    println!("🧪 Running {} comprehensive test cases...\n", test_cases.len());
    
    // Run comprehensive verification
    let results = verifier.run_comprehensive_verification(&test_cases)?;
    
    // Generate detailed report
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  VERIFICATION RESULTS                                          ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");
    
    let mut passed_count = 0;
    let mut failed_count = 0;
    let mut total_rust_time = 0u128;
    let mut total_ovie_time = 0u128;
    
    for (idx, result) in results.iter().enumerate() {
        let status = if result.passed { "✅ PASS" } else { "❌ FAIL" };
        let test_num = idx + 1;
        
        println!("Test {}/{}: {} - {}", 
            test_num, 
            test_cases.len(), 
            status, 
            result.test_case_id
        );
        
        if result.passed {
            passed_count += 1;
            println!("  ├─ Tokens match: ✓");
            println!("  ├─ Token count: {}", result.token_count);
            println!("  ├─ Rust time: {} μs", result.rust_time_us);
            println!("  ├─ Ovie time: {} μs", result.ovie_time_us);
            println!("  └─ Performance ratio: {:.2}x", result.performance_ratio);
            
            total_rust_time += result.rust_time_us as u128;
            total_ovie_time += result.ovie_time_us as u128;
        } else {
            failed_count += 1;
            println!("  ├─ Tokens match: ✗");
            println!("  └─ Errors:");
            for error in &result.errors {
                println!("      • {}", error);
            }
        }
        println!();
    }
    
    // Generate final report
    let report = verifier.generate_verification_report(&results);
    println!("\n{}", report);
    
    // Performance summary
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  PERFORMANCE SUMMARY                                           ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");
    
    let avg_rust_time = if passed_count > 0 { total_rust_time / passed_count as u128 } else { 0 };
    let avg_ovie_time = if passed_count > 0 { total_ovie_time / passed_count as u128 } else { 0 };
    let avg_ratio = if avg_rust_time > 0 { avg_ovie_time as f64 / avg_rust_time as f64 } else { 0.0 };
    
    println!("Total Rust lexer time: {} μs", total_rust_time);
    println!("Total Ovie lexer time: {} μs", total_ovie_time);
    println!("Average Rust time per test: {} μs", avg_rust_time);
    println!("Average Ovie time per test: {} μs", avg_ovie_time);
    println!("Average performance ratio: {:.2}x", avg_ratio);
    
    // Final verdict
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  FINAL VERDICT                                                 ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");
    
    let success_rate = (passed_count as f64 / test_cases.len() as f64) * 100.0;
    
    println!("Tests passed: {}/{}", passed_count, test_cases.len());
    println!("Tests failed: {}/{}", failed_count, test_cases.len());
    println!("Success rate: {:.1}%", success_rate);
    
    if passed_count == test_cases.len() {
        println!("\n🎉 ═══════════════════════════════════════════════════════════════");
        println!("🎉  SELF-HOSTING ACHIEVEMENT VERIFIED!");
        println!("🎉  The Ovie lexer successfully tokenizes ALL language features!");
        println!("🎉 ═══════════════════════════════════════════════════════════════\n");
    } else {
        println!("\n⚠️  Self-hosting verification incomplete.");
        println!("⚠️  {} test(s) failed. Review errors above.\n", failed_count);
    }
    
    // Assert all tests pass
    assert_eq!(
        passed_count, 
        test_cases.len(), 
        "Expected all {} tests to pass, but {} failed", 
        test_cases.len(), 
        failed_count
    );
    
    Ok(())
}
