//! Ovie Standard Library - File System Module Runtime Implementation
//! 
//! This module provides the runtime implementation of the std::fs module
//! as specified in std/fs/mod.ov. All operations are offline-first and
//! designed to work entirely with local files.

use crate::error::OvieError;
use crate::stdlib::{OvieResult, OvieOption, OvieVec, ok, err, some, none};
use std::fs;
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

// ===== FILE SYSTEM TYPES =====

/// File handle for reading and writing
#[derive(Debug)]
pub struct OvieFile {
    path: String,
    file: Option<fs::File>,
    mode: OvieFileMode,
}

/// File opening modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OvieFileMode {
    Read,
    Write,
    Append,
    ReadWrite,
}

/// File metadata
#[derive(Debug, Clone)]
pub struct OvieMetadata {
    pub size: u64,
    pub is_file: bool,
    pub is_directory: bool,
    pub created: u64,    // Unix timestamp
    pub modified: u64,   // Unix timestamp
    pub accessed: u64,   // Unix timestamp
    pub permissions: OviePermissions,
}

/// File permissions
#[derive(Debug, Clone)]
pub struct OviePermissions {
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
}

/// Directory entry
#[derive(Debug, Clone)]
pub struct OvieDirEntry {
    pub name: String,
    pub path: String,
    pub metadata: OvieMetadata,
}

// ===== FILE OPERATIONS =====

/// Open a file for reading
pub fn open(path: String) -> OvieResult<OvieFile, String> {
    open_with_mode(path, OvieFileMode::Read)
}

/// Open a file with specific mode
pub fn open_with_mode(path: String, mode: OvieFileMode) -> OvieResult<OvieFile, String> {
    // Validate path (no network schemes allowed)
    if is_network_path(&path) {
        return err("Network paths not allowed in offline-first file system".to_string());
    }
    
    // Normalize path to prevent directory traversal attacks
    let normalized_path = match normalize_path(&path) {
        Ok(p) => p,
        Err(e) => return err(format!("Invalid file path: {}", e)),
    };
    
    // Open file using standard library
    let file_result = match mode {
        OvieFileMode::Read => fs::File::open(&normalized_path),
        OvieFileMode::Write => fs::File::create(&normalized_path),
        OvieFileMode::Append => fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&normalized_path),
        OvieFileMode::ReadWrite => fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&normalized_path),
    };
    
    match file_result {
        Ok(file) => {
            let ovie_file = OvieFile {
                path: normalized_path,
                file: Some(file),
                mode,
            };
            ok(ovie_file)
        }
        Err(e) => err(format!("Failed to open file: {}", e)),
    }
}

/// Create a new file
pub fn create(path: String) -> OvieResult<OvieFile, String> {
    open_with_mode(path, OvieFileMode::Write)
}

/// Read entire file contents as string
pub fn read_to_string(path: String) -> OvieResult<String, String> {
    // Validate path
    if is_network_path(&path) {
        return err("Network paths not allowed in offline-first file system".to_string());
    }
    
    let normalized_path = match normalize_path(&path) {
        Ok(p) => p,
        Err(e) => return err(format!("Invalid file path: {}", e)),
    };
    
    match fs::read_to_string(&normalized_path) {
        Ok(content) => ok(content),
        Err(e) => err(format!("Failed to read file: {}", e)),
    }
}

/// Read entire file contents as bytes
pub fn read_to_bytes(path: String) -> OvieResult<OvieVec<u8>, String> {
    // Validate path
    if is_network_path(&path) {
        return err("Network paths not allowed in offline-first file system".to_string());
    }
    
    let normalized_path = match normalize_path(&path) {
        Ok(p) => p,
        Err(e) => return err(format!("Invalid file path: {}", e)),
    };
    
    match fs::read(&normalized_path) {
        Ok(bytes) => {
            let mut ovie_vec = OvieVec::new();
            for byte in bytes {
                ovie_vec.push(byte);
            }
            ok(ovie_vec)
        }
        Err(e) => err(format!("Failed to read file: {}", e)),
    }
}

/// Write string to file
pub fn write_string(path: String, content: String) -> OvieResult<(), String> {
    // Validate path
    if is_network_path(&path) {
        return err("Network paths not allowed in offline-first file system".to_string());
    }
    
    let normalized_path = match normalize_path(&path) {
        Ok(p) => p,
        Err(e) => return err(format!("Invalid file path: {}", e)),
    };
    
    match fs::write(&normalized_path, content.as_bytes()) {
        Ok(()) => ok(()),
        Err(e) => err(format!("Failed to write file: {}", e)),
    }
}

/// Write bytes to file
pub fn write_bytes(path: String, content: OvieVec<u8>) -> OvieResult<(), String> {
    // Validate path
    if is_network_path(&path) {
        return err("Network paths not allowed in offline-first file system".to_string());
    }
    
    let normalized_path = match normalize_path(&path) {
        Ok(p) => p,
        Err(e) => return err(format!("Invalid file path: {}", e)),
    };
    
    // Convert OvieVec to Vec
    let mut bytes = Vec::new();
    for i in 0..content.len() {
        match content.get(i) {
            OvieOption::Some(byte) => bytes.push(byte),
            OvieOption::None => break,
        }
    }
    
    match fs::write(&normalized_path, &bytes) {
        Ok(()) => ok(()),
        Err(e) => err(format!("Failed to write file: {}", e)),
    }
}

/// Append string to file
pub fn append_string(path: String, content: String) -> OvieResult<(), String> {
    let mut file = match open_with_mode(path, OvieFileMode::Append) {
        OvieResult::Ok(f) => f,
        OvieResult::Err(e) => return err(e),
    };
    
    file.write_string(content)
}

// ===== FILE METHODS =====

impl OvieFile {
    /// Read entire file contents as string
    pub fn read_to_string(&mut self) -> OvieResult<String, String> {
        if self.file.is_none() {
            return err("File is not open".to_string());
        }
        
        if self.mode != OvieFileMode::Read && self.mode != OvieFileMode::ReadWrite {
            return err("File not opened for reading".to_string());
        }
        
        let mut content = String::new();
        match self.file.as_mut().unwrap().read_to_string(&mut content) {
            Ok(_) => ok(content),
            Err(e) => err(format!("Failed to read file: {}", e)),
        }
    }
    
    /// Read entire file contents as bytes
    pub fn read_to_bytes(&mut self) -> OvieResult<OvieVec<u8>, String> {
        if self.file.is_none() {
            return err("File is not open".to_string());
        }
        
        if self.mode != OvieFileMode::Read && self.mode != OvieFileMode::ReadWrite {
            return err("File not opened for reading".to_string());
        }
        
        let mut buffer = Vec::new();
        match self.file.as_mut().unwrap().read_to_end(&mut buffer) {
            Ok(_) => {
                let mut ovie_vec = OvieVec::new();
                for byte in buffer {
                    ovie_vec.push(byte);
                }
                ok(ovie_vec)
            }
            Err(e) => err(format!("Failed to read file: {}", e)),
        }
    }
    
    /// Read a line from the file
    pub fn read_line(&mut self) -> OvieResult<OvieOption<String>, String> {
        if self.file.is_none() {
            return err("File is not open".to_string());
        }
        
        if self.mode != OvieFileMode::Read && self.mode != OvieFileMode::ReadWrite {
            return err("File not opened for reading".to_string());
        }
        
        let mut reader = BufReader::new(self.file.as_mut().unwrap());
        let mut line = String::new();
        
        match reader.read_line(&mut line) {
            Ok(0) => ok(none()), // End of file
            Ok(_) => {
                // Remove trailing newline
                if line.ends_with('\n') {
                    line.pop();
                    if line.ends_with('\r') {
                        line.pop();
                    }
                }
                ok(some(line))
            }
            Err(e) => err(format!("Failed to read line: {}", e)),
        }
    }
    
    /// Read all lines from the file
    pub fn read_lines(&mut self) -> OvieResult<OvieVec<String>, String> {
        let mut lines = OvieVec::new();
        
        loop {
            match self.read_line() {
                OvieResult::Ok(line_option) => {
                    if line_option.is_none() {
                        break; // End of file
                    }
                    lines.push(line_option.unwrap());
                }
                OvieResult::Err(e) => return err(e),
            }
        }
        
        ok(lines)
    }
    
    /// Write string to file
    pub fn write_string(&mut self, content: String) -> OvieResult<(), String> {
        if self.file.is_none() {
            return err("File is not open".to_string());
        }
        
        if self.mode == OvieFileMode::Read {
            return err("File not opened for writing".to_string());
        }
        
        match self.file.as_mut().unwrap().write_all(content.as_bytes()) {
            Ok(()) => ok(()),
            Err(e) => err(format!("Failed to write to file: {}", e)),
        }
    }
    
    /// Write bytes to file
    pub fn write_bytes(&mut self, content: OvieVec<u8>) -> OvieResult<(), String> {
        if self.file.is_none() {
            return err("File is not open".to_string());
        }
        
        if self.mode == OvieFileMode::Read {
            return err("File not opened for writing".to_string());
        }
        
        // Convert OvieVec to Vec
        let mut bytes = Vec::new();
        for i in 0..content.len() {
            match content.get(i) {
                OvieOption::Some(byte) => bytes.push(byte),
                OvieOption::None => break,
            }
        }
        
        match self.file.as_mut().unwrap().write_all(&bytes) {
            Ok(()) => ok(()),
            Err(e) => err(format!("Failed to write to file: {}", e)),
        }
    }
    
    /// Write a line to the file
    pub fn write_line(&mut self, line: String) -> OvieResult<(), String> {
        self.write_string(format!("{}\n", line))
    }
    
    /// Flush file buffers
    pub fn flush(&mut self) -> OvieResult<(), String> {
        if self.file.is_none() {
            return err("File is not open".to_string());
        }
        
        match self.file.as_mut().unwrap().flush() {
            Ok(()) => ok(()),
            Err(e) => err(format!("Failed to flush file: {}", e)),
        }
    }
    
    /// Close the file
    pub fn close(&mut self) -> OvieResult<(), String> {
        if self.file.is_none() {
            return ok(()); // Already closed
        }
        
        // Flush before closing
        match self.flush() {
            OvieResult::Err(e) => return err(e),
            OvieResult::Ok(()) => {},
        }
        
        self.file = None;
        ok(())
    }
    
    /// Get file metadata
    pub fn metadata(&self) -> OvieResult<OvieMetadata, String> {
        get_metadata(&self.path)
    }
    
    /// Get file size
    pub fn size(&self) -> OvieResult<u64, String> {
        match self.metadata() {
            OvieResult::Ok(metadata) => ok(metadata.size),
            OvieResult::Err(e) => err(e),
        }
    }
}

// ===== DIRECTORY OPERATIONS =====

/// Create a directory
pub fn create_dir(path: String) -> OvieResult<(), String> {
    if is_network_path(&path) {
        return err("Network paths not allowed in offline-first file system".to_string());
    }
    
    let normalized_path = match normalize_path(&path) {
        Ok(p) => p,
        Err(e) => return err(format!("Invalid directory path: {}", e)),
    };
    
    match fs::create_dir(&normalized_path) {
        Ok(()) => ok(()),
        Err(e) => err(format!("Failed to create directory: {}", e)),
    }
}

/// Create a directory and all parent directories
pub fn create_dir_all(path: String) -> OvieResult<(), String> {
    if is_network_path(&path) {
        return err("Network paths not allowed in offline-first file system".to_string());
    }
    
    let normalized_path = match normalize_path(&path) {
        Ok(p) => p,
        Err(e) => return err(format!("Invalid directory path: {}", e)),
    };
    
    match fs::create_dir_all(&normalized_path) {
        Ok(()) => ok(()),
        Err(e) => err(format!("Failed to create directory: {}", e)),
    }
}

/// Remove a directory (must be empty)
pub fn remove_dir(path: String) -> OvieResult<(), String> {
    if is_network_path(&path) {
        return err("Network paths not allowed in offline-first file system".to_string());
    }
    
    let normalized_path = match normalize_path(&path) {
        Ok(p) => p,
        Err(e) => return err(format!("Invalid directory path: {}", e)),
    };
    
    match fs::remove_dir(&normalized_path) {
        Ok(()) => ok(()),
        Err(e) => err(format!("Failed to remove directory: {}", e)),
    }
}

/// Remove a directory and all its contents
pub fn remove_dir_all(path: String) -> OvieResult<(), String> {
    if is_network_path(&path) {
        return err("Network paths not allowed in offline-first file system".to_string());
    }
    
    let normalized_path = match normalize_path(&path) {
        Ok(p) => p,
        Err(e) => return err(format!("Invalid directory path: {}", e)),
    };
    
    match fs::remove_dir_all(&normalized_path) {
        Ok(()) => ok(()),
        Err(e) => err(format!("Failed to remove directory: {}", e)),
    }
}

/// Read directory contents
pub fn read_dir(path: String) -> OvieResult<OvieVec<OvieDirEntry>, String> {
    if is_network_path(&path) {
        return err("Network paths not allowed in offline-first file system".to_string());
    }
    
    let normalized_path = match normalize_path(&path) {
        Ok(p) => p,
        Err(e) => return err(format!("Invalid directory path: {}", e)),
    };
    
    let entries = match fs::read_dir(&normalized_path) {
        Ok(entries) => entries,
        Err(e) => return err(format!("Failed to read directory: {}", e)),
    };
    
    let mut ovie_entries = OvieVec::new();
    
    for entry in entries {
        match entry {
            Ok(entry) => {
                let entry_path = entry.path();
                let entry_path_str = entry_path.to_string_lossy().to_string();
                let name = entry.file_name().to_string_lossy().to_string();
                
                let metadata = match get_metadata(&entry_path_str) {
                    OvieResult::Ok(meta) => meta,
                    OvieResult::Err(_) => continue, // Skip entries we can't read metadata for
                };
                
                let dir_entry = OvieDirEntry {
                    name,
                    path: entry_path_str,
                    metadata,
                };
                
                ovie_entries.push(dir_entry);
            }
            Err(_) => continue, // Skip entries we can't read
        }
    }
    
    ok(ovie_entries)
}

// ===== PATH OPERATIONS =====

/// Check if a path exists
pub fn exists(path: String) -> bool {
    if is_network_path(&path) {
        return false; // Network paths not allowed
    }
    
    let normalized_path = match normalize_path(&path) {
        Ok(p) => p,
        Err(_) => return false, // Invalid path
    };
    
    Path::new(&normalized_path).exists()
}

/// Check if a path is a file
pub fn is_file(path: String) -> bool {
    match get_metadata(&path) {
        OvieResult::Ok(metadata) => metadata.is_file,
        OvieResult::Err(_) => false,
    }
}

/// Check if a path is a directory
pub fn is_dir(path: String) -> bool {
    match get_metadata(&path) {
        OvieResult::Ok(metadata) => metadata.is_directory,
        OvieResult::Err(_) => false,
    }
}

/// Get file metadata
pub fn get_metadata(path: &str) -> OvieResult<OvieMetadata, String> {
    if is_network_path(path) {
        return err("Network paths not allowed in offline-first file system".to_string());
    }
    
    let normalized_path = match normalize_path(path) {
        Ok(p) => p,
        Err(e) => return err(format!("Invalid file path: {}", e)),
    };
    
    let metadata = match fs::metadata(&normalized_path) {
        Ok(meta) => meta,
        Err(e) => return err(format!("Failed to get metadata: {}", e)),
    };
    
    // Convert system times to Unix timestamps
    let created = metadata.created()
        .unwrap_or(UNIX_EPOCH)
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    let modified = metadata.modified()
        .unwrap_or(UNIX_EPOCH)
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    let accessed = metadata.accessed()
        .unwrap_or(UNIX_EPOCH)
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    // Get permissions
    let permissions = metadata.permissions();
    let ovie_permissions = OviePermissions {
        readable: !permissions.readonly(),
        writable: !permissions.readonly(),
        executable: false, // Platform-specific, simplified for now
    };
    
    let ovie_metadata = OvieMetadata {
        size: metadata.len(),
        is_file: metadata.is_file(),
        is_directory: metadata.is_dir(),
        created,
        modified,
        accessed,
        permissions: ovie_permissions,
    };
    
    ok(ovie_metadata)
}

/// Copy a file
pub fn copy_file(from: String, to: String) -> OvieResult<(), String> {
    if is_network_path(&from) || is_network_path(&to) {
        return err("Network paths not allowed in offline-first file system".to_string());
    }
    
    let from_normalized = match normalize_path(&from) {
        Ok(p) => p,
        Err(e) => return err(format!("Invalid source path: {}", e)),
    };
    
    let to_normalized = match normalize_path(&to) {
        Ok(p) => p,
        Err(e) => return err(format!("Invalid destination path: {}", e)),
    };
    
    match fs::copy(&from_normalized, &to_normalized) {
        Ok(_) => ok(()),
        Err(e) => err(format!("Failed to copy file: {}", e)),
    }
}

/// Move/rename a file
pub fn rename_file(from: String, to: String) -> OvieResult<(), String> {
    if is_network_path(&from) || is_network_path(&to) {
        return err("Network paths not allowed in offline-first file system".to_string());
    }
    
    let from_normalized = match normalize_path(&from) {
        Ok(p) => p,
        Err(e) => return err(format!("Invalid source path: {}", e)),
    };
    
    let to_normalized = match normalize_path(&to) {
        Ok(p) => p,
        Err(e) => return err(format!("Invalid destination path: {}", e)),
    };
    
    match fs::rename(&from_normalized, &to_normalized) {
        Ok(()) => ok(()),
        Err(e) => err(format!("Failed to rename file: {}", e)),
    }
}

/// Remove a file
pub fn remove_file(path: String) -> OvieResult<(), String> {
    if is_network_path(&path) {
        return err("Network paths not allowed in offline-first file system".to_string());
    }
    
    let normalized_path = match normalize_path(&path) {
        Ok(p) => p,
        Err(e) => return err(format!("Invalid file path: {}", e)),
    };
    
    match fs::remove_file(&normalized_path) {
        Ok(()) => ok(()),
        Err(e) => err(format!("Failed to remove file: {}", e)),
    }
}

// ===== PATH UTILITIES =====

/// Join path components
pub fn join_path(base: String, component: String) -> String {
    let path = Path::new(&base).join(&component);
    path.to_string_lossy().to_string()
}

/// Get parent directory of a path
pub fn parent_path(path: String) -> OvieOption<String> {
    let path_obj = Path::new(&path);
    match path_obj.parent() {
        Some(parent) => some(parent.to_string_lossy().to_string()),
        None => none(),
    }
}

/// Get filename from a path
pub fn filename(path: String) -> OvieOption<String> {
    let path_obj = Path::new(&path);
    match path_obj.file_name() {
        Some(name) => some(name.to_string_lossy().to_string()),
        None => none(),
    }
}

/// Get file extension
pub fn extension(path: String) -> OvieOption<String> {
    let path_obj = Path::new(&path);
    match path_obj.extension() {
        Some(ext) => some(ext.to_string_lossy().to_string()),
        None => none(),
    }
}

// ===== SECURITY AND VALIDATION =====

/// Check if a path is a network path (forbidden)
pub fn is_network_path(path: &str) -> bool {
    let lower_path = path.to_lowercase();
    
    // Check for network schemes
    lower_path.starts_with("http://") ||
    lower_path.starts_with("https://") ||
    lower_path.starts_with("ftp://") ||
    lower_path.starts_with("sftp://") ||
    lower_path.starts_with("file://") ||
    lower_path.starts_with("\\\\") // UNC paths
}

/// Normalize a path to prevent directory traversal
pub fn normalize_path(path: &str) -> Result<String, String> {
    if path.is_empty() {
        return Err("Empty path".to_string());
    }
    
    // Check for dangerous patterns
    if path.contains("..") {
        return Err("Directory traversal not allowed".to_string());
    }
    
    if path.contains('~') {
        return Err("Home directory expansion not allowed".to_string());
    }
    
    // Use PathBuf for proper normalization
    let path_buf = PathBuf::from(path);
    let normalized = path_buf.to_string_lossy().to_string();
    
    // Convert backslashes to forward slashes for consistency on Windows
    let normalized = normalized.replace('\\', "/");
    
    // Remove duplicate slashes
    let mut result = normalized;
    while result.contains("//") {
        result = result.replace("//", "/");
    }
    
    // Remove trailing slash (except for root)
    if result.len() > 1 && result.ends_with('/') {
        result.pop();
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_file_operations() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt").to_string_lossy().to_string();
        
        // Test write_string
        let content = "Hello, Ovie!".to_string();
        let result = write_string(file_path.clone(), content.clone());
        assert!(result.is_ok());
        
        // Test read_to_string
        let read_result = read_to_string(file_path.clone());
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), content);
        
        // Test exists
        assert!(exists(file_path.clone()));
        
        // Test is_file
        assert!(is_file(file_path.clone()));
        assert!(!is_dir(file_path.clone()));
        
        // Test remove_file
        let remove_result = remove_file(file_path.clone());
        assert!(remove_result.is_ok());
        assert!(!exists(file_path));
    }
    
    #[test]
    fn test_directory_operations() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().join("test_dir").to_string_lossy().to_string();
        
        // Test create_dir
        let result = create_dir(dir_path.clone());
        assert!(result.is_ok());
        
        // Test exists and is_dir
        assert!(exists(dir_path.clone()));
        assert!(is_dir(dir_path.clone()));
        assert!(!is_file(dir_path.clone()));
        
        // Test remove_dir
        let remove_result = remove_dir(dir_path.clone());
        assert!(remove_result.is_ok());
        assert!(!exists(dir_path));
    }
    
    #[test]
    fn test_path_validation() {
        // Test network path detection
        assert!(is_network_path("http://example.com/file.txt"));
        assert!(is_network_path("https://example.com/file.txt"));
        assert!(is_network_path("ftp://example.com/file.txt"));
        assert!(is_network_path("\\\\server\\share\\file.txt"));
        assert!(!is_network_path("/local/file.txt"));
        assert!(!is_network_path("C:\\local\\file.txt"));
        
        // Test path normalization
        assert!(normalize_path("../etc/passwd").is_err());
        assert!(normalize_path("~/secret").is_err());
        assert!(normalize_path("").is_err());
        assert!(normalize_path("/valid/path").is_ok());
    }
    
    #[test]
    fn test_file_modes() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("mode_test.txt").to_string_lossy().to_string();
        
        // Create file with write mode
        let mut file = create(file_path.clone()).unwrap();
        let write_result = file.write_string("test content".to_string());
        assert!(write_result.is_ok());
        file.close().unwrap();
        
        // Open file with read mode
        let mut file = open(file_path.clone()).unwrap();
        let read_result = file.read_to_string();
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), "test content");
        file.close().unwrap();
    }
    
    #[test]
    fn test_path_utilities() {
        // Test join_path
        let joined = join_path("/home/user".to_string(), "documents".to_string());
        assert!(joined.contains("home") && joined.contains("user") && joined.contains("documents"));
        
        // Test filename
        let name = filename("/home/user/document.txt".to_string());
        assert!(name.is_some());
        assert_eq!(name.unwrap(), "document.txt");
        
        // Test extension
        let ext = extension("/home/user/document.txt".to_string());
        assert!(ext.is_some());
        assert_eq!(ext.unwrap(), "txt");
    }
}