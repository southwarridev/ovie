// Cross-Platform System Tests
// Tests for Task 15.2: Cross-platform validation

use oviec::Compiler;
use std::env;

#[test]
fn test_platform_detection() {
    // Test that we can detect the current platform
    let os = env::consts::OS;
    let arch = env::consts::ARCH;
    
    println!("Running on OS: {}, Architecture: {}", os, arch);
    
    // Verify we're on a supported platform
    assert!(
        os == "windows" || os == "linux" || os == "macos",
        "Should be running on a supported platform"
    );
    
    assert!(
        arch == "x86_64" || arch == "aarch64",
        "Should be running on a supported architecture"
    );
}

#[test]
fn test_path_handling_cross_platform() {
    // Test that path handling works correctly on all platforms
    let current_dir = env::current_dir().unwrap();
    
    // Path should be absolute
    assert!(current_dir.is_absolute());
    
    // Should be able to join paths
    let test_path = current_dir.join("test").join("file.txt");
    assert!(test_path.to_string_lossy().contains("test"));
    assert!(test_path.to_string_lossy().contains("file.txt"));
}

#[test]
fn test_line_ending_handling() {
    // Test that the compiler handles different line endings correctly
    let sources = vec![
        "fn main() {\n    print(\"unix\");\n}",           // Unix (LF)
        "fn main() {\r\n    print(\"windows\");\r\n}",   // Windows (CRLF)
        "fn main() {\r    print(\"old mac\");\r}",       // Old Mac (CR)
    ];
    
    let mut compiler = Compiler::new_deterministic();
    
    for source in sources {
        let result = compiler.compile_to_ast(source);
        assert!(result.is_ok(), "Should handle different line endings");
    }
}

#[test]
fn test_deterministic_compilation_cross_platform() {
    // Test that compilation produces deterministic results
    let source = r#"
        fn main() {
            let n = 5;
            let result = n * n;
            print(result);
        }
    "#;
    
    let mut compiler = Compiler::new_deterministic();
    
    // Compile multiple times
    let results: Vec<_> = (0..5)
        .map(|_| compiler.compile_to_ir(source))
        .collect();
    
    // All should succeed
    for result in &results {
        assert!(result.is_ok(), "Compilation should succeed");
    }
    
    // All should produce identical output
    let first = format!("{:?}", results[0]);
    for result in &results[1..] {
        assert_eq!(format!("{:?}", result), first, "Results should be deterministic");
    }
}

#[test]
fn test_unicode_handling() {
    // Test that the compiler handles Unicode correctly
    let source = r#"
        fn main() {
            print("Hello, 世界!");
            print("Привет, мир!");
            print("مرحبا بالعالم!");
        }
    "#;
    
    let mut compiler = Compiler::new_deterministic();
    let result = compiler.compile_to_ast(source);
    assert!(result.is_ok(), "Should handle Unicode strings");
}

#[test]
fn test_file_path_separators() {
    // Test that file paths work correctly on all platforms
    use std::path::Path;
    
    let test_paths = vec![
        "src/main.ov",
        "std/core/mod.ov",
        "examples/hello.ov",
    ];
    
    for path_str in test_paths {
        let path = Path::new(path_str);
        
        // Should be able to get components
        let components: Vec<_> = path.components().collect();
        assert!(!components.is_empty(), "Path should have components");
        
        // Should be able to convert to string
        let as_str = path.to_string_lossy();
        assert!(!as_str.is_empty(), "Path should convert to string");
    }
}

#[test]
fn test_environment_variable_handling() {
    // Test environment variable handling
    let test_var = "OVIE_TEST_VAR";
    let test_value = "test_value_123";
    
    // Set a test environment variable
    env::set_var(test_var, test_value);
    
    // Retrieve it
    let retrieved = env::var(test_var);
    assert!(retrieved.is_ok(), "Should be able to retrieve env var");
    assert_eq!(retrieved.unwrap(), test_value, "Value should match");
    
    // Clean up
    env::remove_var(test_var);
}

#[test]
fn test_temp_directory_access() {
    // Test that we can access temp directory on all platforms
    let temp_dir = env::temp_dir();
    
    assert!(temp_dir.exists(), "Temp directory should exist");
    assert!(temp_dir.is_dir(), "Temp directory should be a directory");
}

#[test]
fn test_current_exe_path() {
    // Test that we can get the current executable path
    let exe_result = env::current_exe();
    
    if let Ok(exe_path) = exe_result {
        assert!(exe_path.is_absolute(), "Exe path should be absolute");
        println!("Current executable: {:?}", exe_path);
    }
}

#[test]
fn test_compilation_with_various_integer_sizes() {
    // Test that integer types work correctly across platforms
    let source = r#"
        fn main() {
            let a = 127;
            let b = 32767;
            let c = 2147483647;
            let d = 255;
            let e = 65535;
        }
    "#;
    
    let mut compiler = Compiler::new_deterministic();
    let result = compiler.compile_to_ast(source);
    assert!(result.is_ok(), "Integer types should work on all platforms");
}

#[test]
fn test_floating_point_determinism() {
    // Test that floating point operations are deterministic
    let source = r#"
        fn main() {
            let x = 3.14159;
            let y = 2.71828;
            let z = x + y;
            print(z);
        }
    "#;
    
    let mut compiler = Compiler::new_deterministic();
    
    // Compile multiple times
    let results: Vec<_> = (0..3)
        .map(|_| compiler.compile_to_ir(source))
        .collect();
    
    // All should produce identical results
    let first = format!("{:?}", results[0]);
    for result in &results[1..] {
        assert_eq!(format!("{:?}", result), first, "Float operations should be deterministic");
    }
}
