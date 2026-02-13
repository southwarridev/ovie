//! Ovie Standard Library - Environment Module Runtime Implementation
//! 
//! This module provides the runtime implementation of the std::env module
//! as specified in std/env/mod.ov. All operations are offline-first and
//! designed with security considerations.

use crate::stdlib::{OvieResult, OvieOption, OvieVec, OvieHashMap, ok, err, some, none};
use std::env;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

// ===== ENVIRONMENT TYPES =====

/// Environment variable collection
#[derive(Debug, Clone)]
pub struct OvieEnvironment {
    pub variables: OvieHashMap<String, String>,
    pub case_sensitive: bool,
}

/// System information
#[derive(Debug, Clone)]
pub struct OvieSystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub architecture: String,
    pub hostname: String,
    pub username: String,
    pub home_directory: String,
    pub current_directory: String,
    pub temp_directory: String,
}

/// Process information
#[derive(Debug, Clone)]
pub struct OvieProcessInfo {
    pub pid: u32,
    pub parent_pid: u32,
    pub executable_path: String,
    pub arguments: OvieVec<String>,
    pub working_directory: String,
    pub environment: OvieEnvironment,
}

// ===== ENVIRONMENT OPERATIONS =====

impl OvieEnvironment {
    /// Create a new environment from system
    pub fn from_system() -> OvieEnvironment {
        let mut env_map = OvieHashMap::new();
        
        for (key, value) in env::vars() {
            env_map.insert(key, value);
        }
        
        OvieEnvironment {
            variables: env_map,
            case_sensitive: is_case_sensitive_os(),
        }
    }
    
    /// Create an empty environment
    pub fn new() -> OvieEnvironment {
        OvieEnvironment {
            variables: OvieHashMap::new(),
            case_sensitive: true,
        }
    }
    
    /// Get environment variable
    pub fn get(&self, name: &str) -> OvieOption<String> {
        let key = if self.case_sensitive {
            name.to_string()
        } else {
            name.to_uppercase()
        };
        
        match self.variables.get(&key) {
            OvieOption::Some(value) => some(value.clone()),
            OvieOption::None => none(),
        }
    }
    
    /// Get environment variable with default
    pub fn get_or(&self, name: &str, default: &str) -> String {
        match self.get(name) {
            OvieOption::Some(value) => value,
            OvieOption::None => default.to_string(),
        }
    }
    
    /// Set environment variable
    pub fn set(&mut self, name: &str, value: &str) {
        let key = if self.case_sensitive {
            name.to_string()
        } else {
            name.to_uppercase()
        };
        self.variables.insert(key, value.to_string());
    }
    
    /// Remove environment variable
    pub fn remove(&mut self, name: &str) -> OvieOption<String> {
        let key = if self.case_sensitive {
            name.to_string()
        } else {
            name.to_uppercase()
        };
        self.variables.remove(&key)
    }
    
    /// Check if environment variable exists
    pub fn contains(&self, name: &str) -> bool {
        match self.get(name) {
            OvieOption::Some(_) => true,
            OvieOption::None => false,
        }
    }
    
    /// Get all environment variable names
    pub fn keys(&self) -> OvieVec<String> {
        let mut keys = OvieVec::new();
        let mut keys_iter = self.variables.keys();
        
        // Use the iterator's next method to get all keys
        loop {
            match keys_iter.next_legacy() {
                OvieOption::Some(key) => keys.push(key),
                OvieOption::None => break,
            }
        }
        
        keys
    }
    
    /// Get all environment variables as key-value pairs
    pub fn to_vec(&self) -> OvieVec<(String, String)> {
        let mut pairs = OvieVec::new();
        let mut iter = self.variables.iter();
        
        // Use the iterator's next method to get all key-value pairs
        loop {
            match iter.next_legacy() {
                OvieOption::Some((key, value)) => pairs.push((key, value)),
                OvieOption::None => break,
            }
        }
        
        pairs
    }
    
    /// Clear all environment variables
    pub fn clear(&mut self) {
        self.variables.clear();
    }
    
    /// Get number of environment variables
    pub fn len(&self) -> usize {
        self.variables.len()
    }
    
    /// Check if environment is empty
    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }
}

// ===== GLOBAL ENVIRONMENT FUNCTIONS =====

/// Get environment variable from system
pub fn var(name: &str) -> OvieOption<String> {
    match env::var(name) {
        Ok(value) => some(value),
        Err(_) => none(),
    }
}

/// Get environment variable with default
pub fn var_or(name: &str, default: &str) -> String {
    match var(name) {
        OvieOption::Some(value) => value,
        OvieOption::None => default.to_string(),
    }
}

/// Set environment variable in current process
pub fn set_var(name: &str, value: &str) -> OvieResult<(), String> {
    env::set_var(name, value);
    ok(())
}

/// Remove environment variable from current process
pub fn remove_var(name: &str) -> OvieResult<(), String> {
    env::remove_var(name);
    ok(())
}

/// Get all environment variables
pub fn vars() -> OvieHashMap<String, String> {
    let mut env_map = OvieHashMap::new();
    for (key, value) in env::vars() {
        env_map.insert(key, value);
    }
    env_map
}

// ===== SYSTEM INFORMATION =====

impl OvieSystemInfo {
    /// Get current system information
    pub fn current() -> OvieSystemInfo {
        OvieSystemInfo {
            os_name: get_os_name(),
            os_version: get_os_version(),
            architecture: get_architecture(),
            hostname: get_hostname(),
            username: get_username(),
            home_directory: get_home_directory(),
            current_directory: get_current_directory(),
            temp_directory: get_temp_directory(),
        }
    }
    
    /// Check if running on Windows
    pub fn is_windows(&self) -> bool {
        self.os_name.to_lowercase().starts_with("windows")
    }
    
    /// Check if running on macOS
    pub fn is_macos(&self) -> bool {
        let lower_os = self.os_name.to_lowercase();
        lower_os == "macos" || lower_os == "darwin"
    }
    
    /// Check if running on Linux
    pub fn is_linux(&self) -> bool {
        self.os_name.to_lowercase() == "linux"
    }
    
    /// Check if running on Unix-like system
    pub fn is_unix_like(&self) -> bool {
        self.is_linux() || self.is_macos()
    }
    
    /// Get path separator for current OS
    pub fn path_separator(&self) -> String {
        if self.is_windows() {
            "\\".to_string()
        } else {
            "/".to_string()
        }
    }
    
    /// Get path list separator for current OS
    pub fn path_list_separator(&self) -> String {
        if self.is_windows() {
            ";".to_string()
        } else {
            ":".to_string()
        }
    }
    
    /// Get line ending for current OS
    pub fn line_ending(&self) -> String {
        if self.is_windows() {
            "\r\n".to_string()
        } else {
            "\n".to_string()
        }
    }
}

// ===== PROCESS INFORMATION =====

impl OvieProcessInfo {
    /// Get current process information
    pub fn current() -> OvieProcessInfo {
        let mut arguments = OvieVec::new();
        for arg in env::args() {
            arguments.push(arg);
        }
        
        OvieProcessInfo {
            pid: get_process_id(),
            parent_pid: get_parent_process_id(),
            executable_path: get_executable_path(),
            arguments,
            working_directory: get_current_directory(),
            environment: OvieEnvironment::from_system(),
        }
    }
    
    /// Get process by ID
    pub fn by_id(pid: u32) -> OvieOption<OvieProcessInfo> {
        // For security reasons, only allow access to current process
        if pid == get_process_id() {
            some(OvieProcessInfo::current())
        } else {
            none() // Security: don't expose other processes
        }
    }
    
    /// Check if process is still running
    pub fn is_running(&self) -> bool {
        is_process_running(self.pid)
    }
    
    /// Get process exit code (if terminated)
    pub fn exit_code(&self) -> OvieOption<i32> {
        if self.is_running() {
            none()
        } else {
            some(get_process_exit_code(self.pid))
        }
    }
}

// ===== PATH UTILITIES =====

/// Join path components using OS-appropriate separator
pub fn join_path(components: &[String]) -> String {
    if components.is_empty() {
        return String::new();
    }
    
    let mut path = PathBuf::from(&components[0]);
    for component in &components[1..] {
        path.push(component);
    }
    
    path.to_string_lossy().to_string()
}

/// Split path into components
pub fn split_path(path: &str) -> OvieVec<String> {
    let mut components = OvieVec::new();
    let path_buf = PathBuf::from(path);
    
    for component in path_buf.components() {
        if let Some(component_str) = component.as_os_str().to_str() {
            components.push(component_str.to_string());
        }
    }
    
    components
}

/// Get absolute path
pub fn absolute_path(path: &str) -> OvieResult<String, String> {
    match Path::new(path).canonicalize() {
        Ok(abs_path) => ok(abs_path.to_string_lossy().to_string()),
        Err(e) => err(format!("Failed to get absolute path: {}", e)),
    }
}

/// Check if path is absolute
pub fn is_absolute_path(path: &str) -> bool {
    Path::new(path).is_absolute()
}

/// Normalize path (resolve . and .. components)
pub fn normalize_path(path: &str) -> String {
    let path_buf = PathBuf::from(path);
    let mut normalized = PathBuf::new();
    
    for component in path_buf.components() {
        match component {
            std::path::Component::CurDir => {
                // Skip current directory references
            }
            std::path::Component::ParentDir => {
                // Go up one directory
                normalized.pop();
            }
            _ => {
                normalized.push(component);
            }
        }
    }
    
    normalized.to_string_lossy().to_string()
}

// ===== DIRECTORY OPERATIONS =====

/// Get current working directory
pub fn current_dir() -> OvieResult<String, String> {
    match env::current_dir() {
        Ok(dir) => ok(dir.to_string_lossy().to_string()),
        Err(e) => err(format!("Failed to get current directory: {}", e)),
    }
}

/// Set current working directory
pub fn set_current_dir(path: &str) -> OvieResult<(), String> {
    match env::set_current_dir(path) {
        Ok(()) => ok(()),
        Err(e) => err(format!("Failed to set current directory: {}", e)),
    }
}

/// Get home directory
pub fn home_dir() -> OvieOption<String> {
    match dirs::home_dir() {
        Some(dir) => some(dir.to_string_lossy().to_string()),
        None => none(),
    }
}

/// Get temporary directory
pub fn temp_dir() -> String {
    env::temp_dir().to_string_lossy().to_string()
}

// ===== COMMAND LINE ARGUMENTS =====

/// Get command line arguments
pub fn args() -> OvieVec<String> {
    let mut arguments = OvieVec::new();
    for arg in env::args() {
        arguments.push(arg);
    }
    arguments
}

/// Get program name (first argument)
pub fn program_name() -> String {
    env::args().next().unwrap_or_default()
}

/// Get command line arguments without program name
pub fn args_without_program() -> OvieVec<String> {
    let mut arguments = OvieVec::new();
    for arg in env::args().skip(1) {
        arguments.push(arg);
    }
    arguments
}

// ===== PROCESS CONTROL =====

/// Exit the current process with code
pub fn exit(code: i32) -> ! {
    std::process::exit(code);
}

/// Exit the current process successfully
pub fn exit_success() -> ! {
    exit(0);
}

/// Exit the current process with error
pub fn exit_failure() -> ! {
    exit(1);
}

// ===== COMMON ENVIRONMENT VARIABLES =====

/// Get PATH environment variable as list
pub fn path() -> OvieVec<String> {
    let system_info = OvieSystemInfo::current();
    let separator = system_info.path_list_separator();
    let path_var = var_or("PATH", "");
    
    if path_var.is_empty() {
        OvieVec::new()
    } else {
        let mut paths = OvieVec::new();
        for path in path_var.split(&separator) {
            paths.push(path.to_string());
        }
        paths
    }
}

/// Get HOME directory from environment
pub fn home() -> OvieOption<String> {
    let system_info = OvieSystemInfo::current();
    
    if system_info.is_windows() {
        if let OvieOption::Some(userprofile) = var("USERPROFILE") {
            return some(userprofile);
        }
        
        if let (OvieOption::Some(homedrive), OvieOption::Some(homepath)) = (var("HOMEDRIVE"), var("HOMEPATH")) {
            return some(format!("{}{}", homedrive, homepath));
        }
    } else {
        if let OvieOption::Some(home_var) = var("HOME") {
            return some(home_var);
        }
    }
    
    // Fallback to system call
    home_dir()
}

/// Get USER or USERNAME
pub fn user() -> OvieOption<String> {
    if let OvieOption::Some(user_var) = var("USER") {
        return some(user_var);
    }
    
    if let OvieOption::Some(username_var) = var("USERNAME") {
        return some(username_var);
    }
    
    none()
}

/// Get SHELL environment variable
pub fn shell() -> OvieOption<String> {
    var("SHELL")
}

/// Get EDITOR environment variable
pub fn editor() -> OvieOption<String> {
    if let OvieOption::Some(editor_var) = var("EDITOR") {
        return some(editor_var);
    }
    
    if let OvieOption::Some(visual_var) = var("VISUAL") {
        return some(visual_var);
    }
    
    none()
}

// ===== SECURITY UTILITIES =====

/// Check if running with elevated privileges
pub fn is_elevated() -> bool {
    is_process_elevated()
}

/// Get effective user ID (Unix-like systems)
pub fn effective_user_id() -> OvieOption<u32> {
    let system_info = OvieSystemInfo::current();
    
    if system_info.is_unix_like() {
        some(get_effective_user_id())
    } else {
        none()
    }
}

/// Get real user ID (Unix-like systems)
pub fn real_user_id() -> OvieOption<u32> {
    let system_info = OvieSystemInfo::current();
    
    if system_info.is_unix_like() {
        some(get_real_user_id())
    } else {
        none()
    }
}

/// Check if running as root (Unix-like systems)
pub fn is_root() -> bool {
    match effective_user_id() {
        OvieOption::Some(euid) => euid == 0,
        OvieOption::None => false,
    }
}

// ===== INTERNAL UTILITY FUNCTIONS =====

fn is_case_sensitive_os() -> bool {
    !cfg!(windows)
}

fn get_os_name() -> String {
    env::consts::OS.to_string()
}

fn get_os_version() -> String {
    // This is a simplified version - in a real implementation,
    // you'd query the actual OS version
    "unknown".to_string()
}

fn get_architecture() -> String {
    env::consts::ARCH.to_string()
}

fn get_hostname() -> String {
    hostname::get()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

fn get_username() -> String {
    whoami::username()
}

fn get_home_directory() -> String {
    match dirs::home_dir() {
        Some(dir) => dir.to_string_lossy().to_string(),
        None => String::new(),
    }
}

fn get_current_directory() -> String {
    match env::current_dir() {
        Ok(dir) => dir.to_string_lossy().to_string(),
        Err(_) => String::new(),
    }
}

fn get_temp_directory() -> String {
    env::temp_dir().to_string_lossy().to_string()
}

fn get_process_id() -> u32 {
    std::process::id()
}

fn get_parent_process_id() -> u32 {
    // This is platform-specific and would need proper implementation
    0
}

fn get_executable_path() -> String {
    match env::current_exe() {
        Ok(path) => path.to_string_lossy().to_string(),
        Err(_) => String::new(),
    }
}

fn is_process_running(_pid: u32) -> bool {
    // This would need platform-specific implementation
    true
}

fn get_process_exit_code(_pid: u32) -> i32 {
    // This would need platform-specific implementation
    0
}

fn is_process_elevated() -> bool {
    // This would need platform-specific implementation
    false
}

fn get_effective_user_id() -> u32 {
    #[cfg(unix)]
    {
        unsafe { libc::geteuid() }
    }
    #[cfg(not(unix))]
    {
        0
    }
}

fn get_real_user_id() -> u32 {
    #[cfg(unix)]
    {
        unsafe { libc::getuid() }
    }
    #[cfg(not(unix))]
    {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_creation() {
        let env = OvieEnvironment::new();
        assert!(env.is_empty());
        assert_eq!(env.len(), 0);
        
        let system_env = OvieEnvironment::from_system();
        // System environment should have at least some variables
        assert!(!system_env.is_empty());
    }
    
    #[test]
    fn test_environment_operations() {
        let mut env = OvieEnvironment::new();
        
        // Test set and get
        env.set("TEST_VAR", "test_value");
        assert_eq!(env.get("TEST_VAR"), some("test_value".to_string()));
        assert!(env.contains("TEST_VAR"));
        
        // Test get_or
        assert_eq!(env.get_or("TEST_VAR", "default"), "test_value");
        assert_eq!(env.get_or("NONEXISTENT", "default"), "default");
        
        // Test remove
        assert_eq!(env.remove("TEST_VAR"), some("test_value".to_string()));
        assert_eq!(env.get("TEST_VAR"), none());
        assert!(!env.contains("TEST_VAR"));
    }
    
    #[test]
    fn test_system_info() {
        let system_info = OvieSystemInfo::current();
        
        // Basic checks
        assert!(!system_info.os_name.is_empty());
        assert!(!system_info.architecture.is_empty());
        
        // OS detection
        let is_windows = cfg!(windows);
        let is_unix = cfg!(unix);
        
        assert_eq!(system_info.is_windows(), is_windows);
        assert_eq!(system_info.is_unix_like(), is_unix);
        
        // Path separators
        if system_info.is_windows() {
            assert_eq!(system_info.path_separator(), "\\");
            assert_eq!(system_info.path_list_separator(), ";");
            assert_eq!(system_info.line_ending(), "\r\n");
        } else {
            assert_eq!(system_info.path_separator(), "/");
            assert_eq!(system_info.path_list_separator(), ":");
            assert_eq!(system_info.line_ending(), "\n");
        }
    }
    
    #[test]
    fn test_process_info() {
        let process_info = OvieProcessInfo::current();
        
        // Basic checks
        assert!(process_info.pid > 0);
        assert!(!process_info.executable_path.is_empty());
        assert!(!process_info.arguments.is_empty());
        assert!(process_info.is_running());
        
        // Test by_id with current process
        let same_process = OvieProcessInfo::by_id(process_info.pid);
        assert!(same_process.is_some());
        
        // Test by_id with different process (should return None for security)
        let other_process = OvieProcessInfo::by_id(99999);
        assert!(other_process.is_none());
    }
    
    #[test]
    fn test_path_utilities() {
        // Test join_path
        let components = vec!["home".to_string(), "user".to_string(), "documents".to_string()];
        let joined = join_path(&components);
        assert!(joined.contains("home"));
        assert!(joined.contains("user"));
        assert!(joined.contains("documents"));
        
        // Test is_absolute_path
        #[cfg(windows)]
        {
            assert!(is_absolute_path("C:\\Windows"));
            assert!(!is_absolute_path("relative\\path"));
        }
        #[cfg(unix)]
        {
            assert!(is_absolute_path("/usr/bin"));
            assert!(!is_absolute_path("relative/path"));
        }
    }
    
    #[test]
    fn test_directory_operations() {
        // Test current_dir
        let current = current_dir();
        assert!(current.is_ok());
        
        // Test temp_dir
        let temp = temp_dir();
        assert!(!temp.is_empty());
    }
    
    #[test]
    fn test_command_line_args() {
        let args = args();
        assert!(!args.is_empty());
        
        let program = program_name();
        assert!(!program.is_empty());
        
        let args_without = args_without_program();
        assert_eq!(args_without.len(), args.len() - 1);
    }
    
    #[test]
    fn test_environment_variables() {
        // Test setting and getting environment variables
        set_var("OVIE_TEST_VAR", "test_value").unwrap();
        assert_eq!(var("OVIE_TEST_VAR"), some("test_value".to_string()));
        
        // Test var_or
        assert_eq!(var_or("OVIE_TEST_VAR", "default"), "test_value");
        assert_eq!(var_or("NONEXISTENT_VAR", "default"), "default");
        
        // Clean up
        remove_var("OVIE_TEST_VAR").unwrap();
        assert_eq!(var("OVIE_TEST_VAR"), none());
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
    }
    
    #[test]
    fn test_deterministic_behavior() {
        // Test that same operations produce same results
        for _ in 0..10 {
            let system_info1 = OvieSystemInfo::current();
            let system_info2 = OvieSystemInfo::current();
            
            assert_eq!(system_info1.os_name, system_info2.os_name);
            assert_eq!(system_info1.architecture, system_info2.architecture);
            
            let args1 = args();
            let args2 = args();
            assert_eq!(args1.len(), args2.len());
        }
    }
}