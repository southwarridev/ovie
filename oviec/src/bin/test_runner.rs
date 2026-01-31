//! Ovie Compiler Test Runner
//! 
//! This binary provides a comprehensive test runner for the Ovie compiler,
//! supporting unit tests, property-based tests, integration tests, conformance tests,
//! performance benchmarks, and regression detection.

use oviec::tests::{TestRunner, TestSuiteConfig, TestCategory, TestStatus};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Parse command line arguments
    let mut config = TestSuiteConfig::default();
    let mut test_filter = None;
    let mut verbose = false;
    let mut create_baseline = false;
    let mut load_baseline_path = None;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print_help();
                return;
            }
            "--verbose" | "-v" => {
                verbose = true;
            }
            "--no-property" => {
                config.enable_property_tests = false;
            }
            "--no-cross-platform" => {
                config.enable_cross_platform = false;
            }
            "--no-performance" => {
                config.enable_performance_tests = false;
            }
            "--no-regression" => {
                config.enable_regression_tests = false;
            }
            "--create-baseline" => {
                create_baseline = true;
            }
            "--load-baseline" => {
                if i + 1 < args.len() {
                    load_baseline_path = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: --load-baseline requires a file path");
                    process::exit(1);
                }
            }
            "--iterations" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<usize>() {
                        Ok(iterations) => {
                            config.property_test_iterations = iterations;
                            i += 1;
                        }
                        Err(_) => {
                            eprintln!("Error: Invalid iterations value: {}", args[i + 1]);
                            process::exit(1);
                        }
                    }
                } else {
                    eprintln!("Error: --iterations requires a value");
                    process::exit(1);
                }
            }
            "--timeout" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<u64>() {
                        Ok(timeout) => {
                            config.test_timeout_seconds = timeout;
                            i += 1;
                        }
                        Err(_) => {
                            eprintln!("Error: Invalid timeout value: {}", args[i + 1]);
                            process::exit(1);
                        }
                    }
                } else {
                    eprintln!("Error: --timeout requires a value");
                    process::exit(1);
                }
            }
            "--filter" => {
                if i + 1 < args.len() {
                    test_filter = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: --filter requires a value");
                    process::exit(1);
                }
            }
            "--seed" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<u64>() {
                        Ok(seed) => {
                            config.random_seed = Some(seed);
                            i += 1;
                        }
                        Err(_) => {
                            eprintln!("Error: Invalid seed value: {}", args[i + 1]);
                            process::exit(1);
                        }
                    }
                } else {
                    eprintln!("Error: --seed requires a value");
                    process::exit(1);
                }
            }
            arg if arg.starts_with("--") => {
                eprintln!("Error: Unknown option: {}", arg);
                process::exit(1);
            }
            _ => {
                eprintln!("Error: Unexpected argument: {}", args[i]);
                process::exit(1);
            }
        }
        i += 1;
    }
    
    // Create and configure test runner
    let mut runner = TestRunner::with_config(config.clone());
    
    // Load baseline if specified
    if let Some(baseline_path) = load_baseline_path {
        match std::fs::read_to_string(&baseline_path) {
            Ok(baseline_json) => {
                match serde_json::from_str(&baseline_json) {
                    Ok(baseline_data) => {
                        runner.load_regression_baseline(baseline_data);
                        if verbose {
                            println!("Loaded regression baseline from: {}", baseline_path);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error parsing baseline file: {}", e);
                        process::exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading baseline file: {}", e);
                process::exit(1);
            }
        }
    }
    
    // Create baseline if requested
    if create_baseline {
        let test_cases = vec![
            include_str!("../../examples/hello.ov").to_string(),
            include_str!("../../examples/calculator.ov").to_string(),
            include_str!("../../examples/functions.ov").to_string(),
            include_str!("../../examples/variables.ov").to_string(),
            include_str!("../../examples/control_flow.ov").to_string(),
        ];
        
        match runner.create_regression_baseline(&test_cases) {
            Ok(baseline_data) => {
                let baseline_json = serde_json::to_string_pretty(&baseline_data).unwrap();
                let baseline_filename = format!("regression_baseline_{}.json", 
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs());
                
                match std::fs::write(&baseline_filename, baseline_json) {
                    Ok(_) => {
                        println!("Created regression baseline: {}", baseline_filename);
                        return;
                    }
                    Err(e) => {
                        eprintln!("Error writing baseline file: {}", e);
                        process::exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error creating baseline: {}", e);
                process::exit(1);
            }
        }
    }
    
    // Run tests
    println!("Ovie Compiler Test Suite");
    println!("========================");
    println!();
    
    if verbose {
        println!("Configuration:");
        println!("  Property tests: {}", config.enable_property_tests);
        println!("  Property iterations: {}", config.property_test_iterations);
        println!("  Cross-platform tests: {}", config.enable_cross_platform);
        println!("  Performance tests: {}", config.enable_performance_tests);
        println!("  Regression tests: {}", config.enable_regression_tests);
        println!("  Test timeout: {}s", config.test_timeout_seconds);
        println!("  Deterministic execution: {}", config.deterministic_execution);
        if let Some(seed) = config.random_seed {
            println!("  Random seed: {}", seed);
        }
        if let Some(ref filter) = test_filter {
            println!("  Test filter: {}", filter);
        }
        println!();
    }
    
    let results = runner.run_all_tests();
    
    // Filter results if requested
    let filtered_results = if let Some(filter) = test_filter {
        let mut filtered = results.clone();
        filtered.test_results.retain(|result| result.name.contains(&filter));
        filtered
    } else {
        results
    };
    
    // Print detailed results if verbose
    if verbose {
        println!("\n=== Detailed Results ===");
        for result in &filtered_results.test_results {
            let status_symbol = match result.status {
                TestStatus::Passed => "✓",
                TestStatus::Failed => "✗",
                TestStatus::Skipped => "⊝",
                TestStatus::Timeout => "⏱",
            };
            
            println!("{} {} ({:?}) - {:.2}ms", 
                status_symbol, 
                result.name, 
                result.category,
                result.duration.as_millis());
            
            if let Some(ref error) = result.error_message {
                println!("    Error: {}", error);
            }
        }
        println!();
    }
    
    // Print summary report
    let report = filtered_results.generate_report();
    println!("{}", report);
    
    // Exit with appropriate code
    let exit_code = if filtered_results.summary.failed > 0 {
        1
    } else {
        0
    };
    
    process::exit(exit_code);
}

fn print_help() {
    println!("Ovie Compiler Test Runner");
    println!();
    println!("USAGE:");
    println!("    test_runner [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help              Print this help message");
    println!("    -v, --verbose           Enable verbose output");
    println!("    --no-property           Disable property-based tests");
    println!("    --no-cross-platform     Disable cross-platform tests");
    println!("    --no-performance        Disable performance tests");
    println!("    --no-regression         Disable regression tests");
    println!("    --create-baseline       Create new regression baseline and exit");
    println!("    --load-baseline <FILE>  Load regression baseline from file");
    println!("    --iterations <N>        Set property test iterations (default: 1000)");
    println!("    --timeout <SECONDS>     Set test timeout in seconds (default: 300)");
    println!("    --filter <PATTERN>      Run only tests matching pattern");
    println!("    --seed <SEED>           Set random seed for deterministic tests");
    println!();
    println!("EXAMPLES:");
    println!("    test_runner                           # Run all tests");
    println!("    test_runner --verbose                 # Run with detailed output");
    println!("    test_runner --no-performance          # Skip performance tests");
    println!("    test_runner --filter lexer            # Run only lexer tests");
    println!("    test_runner --iterations 500          # Run 500 property test iterations");
    println!("    test_runner --seed 12345              # Use specific random seed");
    println!("    test_runner --create-baseline         # Create regression baseline");
    println!("    test_runner --load-baseline base.json # Load regression baseline");
}