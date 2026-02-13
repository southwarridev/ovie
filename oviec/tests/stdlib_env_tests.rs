//! Integration tests for std::env module
//! 
//! Tests the environment variable access, system information, and process
//! information functionality of the Ovie standard library.

use oviec::stdlib::env::*;
use oviec::stdlib::{OvieResult, OvieOption, OvieVec, OvieHashMap, ok, err, some, none};

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_environment_creation_and_basic_operations() {
        // Test empty environment
        let mut env = OvieEnvironment::new();
        assert!(env.is_empty());
        assert_eq!(env.len(), 0);
        
        // Test set and get
        env.set("TEST_VAR", "test_value");
        assert_eq!(env.get("TEST_VAR"), some("test_value".to_string()));
        assert!(env.contains("TEST_VAR"));
        assert_eq!(env.len(), 1);
        assert!(!env.is_empty());
        
        // Test get_or
        assert_eq!(env.get_or("TEST_VAR", "default"), "test_value");
        assert_eq!(env.get_or("NONEXISTENT", "default"), "default");
        
        // Test remove
        assert_eq!(env.remove("TEST_VAR"), some("test_value".to_string()));
        assert_eq!(env.get("TEST_VAR"), none());
        assert!(!env.contains("TEST_VAR"));
        assert!(env.is_empty());
    }
    
    #[test]
    fn test_environment_from_system() {
        let system_env = OvieEnvironment::from_system();
        
        // System environment should have at least some variables
        assert!(!system_env.is_empty());
        assert!(system_env.len() > 0);
        
        // Should have PATH on all systems
        assert!(system_env.contains("PATH"));
        
        // Test case sensitivity based on OS
        #[cfg(windows)]
        {
            assert!(!system_env.case_sensitive);
        }
        #[cfg(not(windows))]
        {
            assert!(system_env.case_sensitive);
        }
    }
    
    #[test]
    fn test_environment_keys_and_iteration() {
        let mut env = OvieEnvironment::new();
        
        // Add some test variables
        env.set("VAR1", "value1");
        env.set("VAR2", "value2");
        env.set("VAR3", "value3");
        
        // Test keys
        let keys = env.keys();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"VAR1".to_string()));
        assert!(keys.contains(&"VAR2".to_string()));
        assert!(keys.contains(&"VAR3".to_string()));
        
        // Test to_vec
        let pairs = env.to_vec();
        assert_eq!(pairs.len(), 3);
        
        // Verify all pairs are present
        let mut found_pairs = 0;
        for (key, value) in pairs.iter() {
            match key.as_str() {
                "VAR1" => {
                    assert_eq!(value, "value1");
                    found_pairs += 1;
                }
                "VAR2" => {
                    assert_eq!(value, "value2");
                    found_pairs += 1;
                }
                "VAR3" => {
                    assert_eq!(value, "value3");
                    found_pairs += 1;
                }
                _ => panic!("Unexpected key: {}", key),
            }
        }
        assert_eq!(found_pairs, 3);
        
        // Test clear
        env.clear();
        assert!(env.is_empty());
        assert_eq!(env.len(), 0);
    }
    
    #[test]
    fn test_global_environment_functions() {
        // Test setting and getting environment variables
        set_var("OVIE_TEST_VAR", "test_value").unwrap();
        assert_eq!(var("OVIE_TEST_VAR"), some("test_value".to_string()));
        
        // Test var_or
        assert_eq!(var_or("OVIE_TEST_VAR", "default"), "test_value");
        assert_eq!(var_or("NONEXISTENT_VAR", "default"), "default");
        
        // Test vars
        let all_vars = vars();
        assert!(!all_vars.is_empty());
        assert!(all_vars.contains_key("OVIE_TEST_VAR"));
        
        // Clean up
        remove_var("OVIE_TEST_VAR").unwrap();
        assert_eq!(var("OVIE_TEST_VAR"), none());
    }
    
    #[test]
    fn test_system_info() {
        let system_info = OvieSystemInfo::current();
        
        // Basic checks
        assert!(!system_info.os_name.is_empty());
        assert!(!system_info.architecture.is_empty());
        assert!(!system_info.hostname.is_empty());
        assert!(!system_info.username.is_empty());
        assert!(!system_info.current_directory.is_empty());
        assert!(!system_info.temp_directory.is_empty());
        
        // OS detection should be consistent with Rust's cfg
        let is_windows = cfg!(windows);
        let is_unix = cfg!(unix);
        
        assert_eq!(system_info.is_windows(), is_windows);
        assert_eq!(system_info.is_unix_like(), is_unix);
        
        // Path separators should match OS
        if system_info.is_windows() {
            assert_eq!(system_info.path_separator(), "\\");
            assert_eq!(system_info.path_list_separator(), ";");
            assert_eq!(system_info.line_ending(), "\r\n");
        } else {
            assert_eq!(system_info.path_separator(), "/");
            assert_eq!(system_info.path_list_separator(), ":");
            assert_eq!(system_info.line_ending(), "\n");
        }
        
        // macOS detection
        #[cfg(target_os = "macos")]
        {
            assert!(system_info.is_macos());
        }
        
        // Linux detection
        #[cfg(target_os = "linux")]
        {
            assert!(system_info.is_linux());
        }
    }
    
    #[test]
    fn test_process_info() {
        let process_info = OvieProcessInfo::current();
        
        // Basic checks
        assert!(process_info.pid > 0);
        assert!(!process_info.executable_path.is_empty());
        assert!(!process_info.arguments.is_empty());
        assert!(!process_info.working_directory.is_empty());
        assert!(process_info.is_running());
        
        // Process should be running, so no exit code
        assert_eq!(process_info.exit_code(), none());
        
        // Test by_id with current process
        let same_process = OvieProcessInfo::by_id(process_info.pid);
        assert!(same_process.is_some());
        
        // Test by_id with different process (should return None for security)
        let other_process = OvieProcessInfo::by_id(99999);
        assert!(other_process.is_none());
        
        // Environment should not be empty
        assert!(!process_info.environment.is_empty());
    }
    
    #[test]
    fn test_path_utilities() {
        // Test join_path
        let components = vec!["home".to_string(), "user".to_string(), "documents".to_string()];
        let joined = join_path(&components);
        assert!(joined.contains("home"));
        assert!(joined.contains("user"));
        assert!(joined.contains("documents"));
        
        // Test empty components
        let empty_components = vec![];
        let empty_joined = join_path(&empty_components);
        assert!(empty_joined.is_empty());
        
        // Test single component
        let single_component = vec!["single".to_string()];
        let single_joined = join_path(&single_component);
        assert_eq!(single_joined, "single");
        
        // Test is_absolute_path
        #[cfg(windows)]
        {
            assert!(is_absolute_path("C:\\Windows"));
            assert!(is_absolute_path("\\\\server\\share"));
            assert!(!is_absolute_path("relative\\path"));
            assert!(!is_absolute_path("C:relative"));
        }
        #[cfg(unix)]
        {
            assert!(is_absolute_path("/usr/bin"));
            assert!(is_absolute_path("/"));
            assert!(!is_absolute_path("relative/path"));
            assert!(!is_absolute_path("./relative"));
        }
        
        // Test split_path
        #[cfg(windows)]
        {
            let components = split_path("C:\\Users\\test\\Documents");
            assert!(components.len() >= 3);
            assert!(components.contains(&"Users".to_string()));
            assert!(components.contains(&"test".to_string()));
            assert!(components.contains(&"Documents".to_string()));
        }
        #[cfg(unix)]
        {
            let components = split_path("/home/user/documents");
            assert!(components.len() >= 3);
            assert!(components.contains(&"home".to_string()));
            assert!(components.contains(&"user".to_string()));
            assert!(components.contains(&"documents".to_string()));
        }
    }
    
    #[test]
    fn test_normalize_path() {
        // Test current directory normalization
        let normalized = normalize_path("./test");
        assert_eq!(normalized, "test");
        
        // Test parent directory normalization
        let normalized = normalize_path("test/../other");
        assert_eq!(normalized, "other");
        
        // Test complex normalization
        let normalized = normalize_path("a/b/../c/./d");
        assert_eq!(normalized, "a/c/d");
        
        // Test root preservation on Unix
        #[cfg(unix)]
        {
            let normalized = normalize_path("/a/b/../c");
            assert_eq!(normalized, "/a/c");
        }
        
        // Test drive preservation on Windows
        #[cfg(windows)]
        {
            let normalized = normalize_path("C:\\a\\b\\..\\c");
            assert_eq!(normalized, "C:\\a\\c");
        }
    }
    
    #[test]
    fn test_directory_operations() {
        // Test current_dir
        let current = current_dir();
        assert!(current.is_ok());
        let current_path = current.unwrap();
        assert!(!current_path.is_empty());
        
        // Test temp_dir
        let temp = temp_dir();
        assert!(!temp.is_empty());
        
        // Test home_dir
        let home = home_dir();
        // Home directory should exist on most systems
        assert!(home.is_some());
        let home_path = home.unwrap();
        assert!(!home_path.is_empty());
    }
    
    #[test]
    fn test_command_line_args() {
        let args = args();
        assert!(!args.is_empty());
        
        let program = program_name();
        assert!(!program.is_empty());
        
        let args_without = args_without_program();
        assert_eq!(args_without.len(), args.len() - 1);
        
        // First argument should be program name
        assert_eq!(args.get(0).unwrap(), &program);
    }
    
    #[test]
    fn test_common_environment_variables() {
        // Test PATH
        let path_list = path();
        // PATH should exist on all systems
        assert!(!path_list.is_empty());
        
        // Test user
        let user_name = user();
        // Should have some user identification
        assert!(user_name.is_some());
        let user_str = user_name.unwrap();
        assert!(!user_str.is_empty());
        
        // Test home
        let home_path = home();
        // Should have home directory
        assert!(home_path.is_some());
        let home_str = home_path.unwrap();
        assert!(!home_str.is_empty());
        
        // Test shell (Unix-like systems)
        #[cfg(unix)]
        {
            let shell_path = shell();
            // Shell might not be set in all environments, so just check if present
            if let some(shell_str) = shell_path {
                assert!(!shell_str.is_empty());
            }
        }
        
        // Test editor (might not be set)
        let editor_path = editor();
        if let some(editor_str) = editor_path {
            assert!(!editor_str.is_empty());
        }
    }
    
    #[test]
    fn test_security_utilities() {
        // Test effective_user_id (Unix-like systems)
        #[cfg(unix)]
        {
            let euid = effective_user_id();
            assert!(euid.is_some());
            let euid_val = euid.unwrap();
            assert!(euid_val >= 0);
            
            let ruid = real_user_id();
            assert!(ruid.is_some());
            let ruid_val = ruid.unwrap();
            assert!(ruid_val >= 0);
            
            // Test is_root
            let is_root_user = is_root();
            assert_eq!(is_root_user, euid_val == 0);
        }
        
        // Test effective_user_id (Windows)
        #[cfg(windows)]
        {
            let euid = effective_user_id();
            assert!(euid.is_none());
            
            let ruid = real_user_id();
            assert!(ruid.is_none());
            
            // is_root should always be false on Windows
            assert!(!is_root());
        }
        
        // Test is_elevated (platform-specific behavior)
        let elevated = is_elevated();
        // Just ensure it returns a boolean without panicking
        assert!(elevated == true || elevated == false);
    }
    
    #[test]
    fn test_absolute_path_conversion() {
        // Test with relative path
        let relative = "test_file.txt";
        let absolute_result = absolute_path(relative);
        assert!(absolute_result.is_ok());
        let absolute_path_str = absolute_result.unwrap();
        assert!(is_absolute_path(&absolute_path_str));
        
        // Test with already absolute path
        #[cfg(unix)]
        {
            let already_absolute = "/usr/bin/test";
            let absolute_result = absolute_path(already_absolute);
            assert!(absolute_result.is_ok());
            let absolute_path_str = absolute_result.unwrap();
            assert!(is_absolute_path(&absolute_path_str));
        }
        
        #[cfg(windows)]
        {
            let already_absolute = "C:\\Windows\\System32";
            let absolute_result = absolute_path(already_absolute);
            assert!(absolute_result.is_ok());
            let absolute_path_str = absolute_result.unwrap();
            assert!(is_absolute_path(&absolute_path_str));
        }
    }
    
    #[test]
    fn test_deterministic_behavior() {
        // Test that same operations produce same results
        for _ in 0..10 {
            let system_info1 = OvieSystemInfo::current();
            let system_info2 = OvieSystemInfo::current();
            
            assert_eq!(system_info1.os_name, system_info2.os_name);
            assert_eq!(system_info1.architecture, system_info2.architecture);
            assert_eq!(system_info1.hostname, system_info2.hostname);
            assert_eq!(system_info1.username, system_info2.username);
            
            let args1 = args();
            let args2 = args();
            assert_eq!(args1.len(), args2.len());
            
            let path1 = path();
            let path2 = path();
            assert_eq!(path1.len(), path2.len());
        }
    }
    
    #[test]
    fn test_environment_case_sensitivity() {
        let mut env = OvieEnvironment::new();
        
        // Test case sensitive environment
        env.case_sensitive = true;
        env.set("TestVar", "value1");
        env.set("TESTVAR", "value2");
        
        assert_eq!(env.get("TestVar"), some("value1".to_string()));
        assert_eq!(env.get("TESTVAR"), some("value2".to_string()));
        assert_eq!(env.get("testvar"), none());
        assert_eq!(env.len(), 2);
        
        // Test case insensitive environment
        let mut env_insensitive = OvieEnvironment::new();
        env_insensitive.case_sensitive = false;
        env_insensitive.set("TestVar", "value1");
        env_insensitive.set("TESTVAR", "value2"); // Should overwrite
        
        assert_eq!(env_insensitive.get("TestVar"), some("value2".to_string()));
        assert_eq!(env_insensitive.get("TESTVAR"), some("value2".to_string()));
        assert_eq!(env_insensitive.get("testvar"), some("value2".to_string()));
        assert_eq!(env_insensitive.len(), 1);
    }
    
    #[test]
    fn test_error_handling() {
        // Test setting current directory to non-existent path
        let result = set_current_dir("/nonexistent/path/that/should/not/exist");
        assert!(result.is_err());
        
        // Test absolute_path with invalid path
        let result = absolute_path("/nonexistent/path/that/should/not/exist");
        // This might succeed (returning the normalized path) or fail depending on implementation
        // Just ensure it doesn't panic
        match result {
            ok(_) => {}, // Path was normalized successfully
            err(_) => {}, // Path resolution failed
        }
    }
    
    #[test]
    fn test_cross_platform_compatibility() {
        // Test that all functions work on current platform
        let system_info = OvieSystemInfo::current();
        let process_info = OvieProcessInfo::current();
        let env = OvieEnvironment::from_system();
        
        // Basic functionality should work on all platforms
        assert!(!system_info.os_name.is_empty());
        assert!(process_info.pid > 0);
        assert!(!env.is_empty());
        
        // Path operations should work on all platforms
        let components = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let joined = join_path(&components);
        assert!(joined.contains("a"));
        assert!(joined.contains("b"));
        assert!(joined.contains("c"));
        
        // Directory operations should work on all platforms
        let current = current_dir();
        assert!(current.is_ok());
        
        let temp = temp_dir();
        assert!(!temp.is_empty());
    }
}