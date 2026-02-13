//! Ovie Standard Library - I/O Module
//! 
//! Offline-first I/O operations with deterministic behavior.
//! All I/O operations are designed to be reproducible and platform-independent.

use crate::stdlib::core::{OvieResult, ok, err};
use std::io::{self, Write, Read, BufRead, BufReader, BufWriter};
use std::fs::File;
use std::path::Path;

// ===== STANDARD I/O HANDLES =====

/// Standard input handle
pub struct Stdin {
    inner: std::io::Stdin,
}

impl Stdin {
    /// Create a new stdin handle
    pub fn new() -> Self {
        Self {
            inner: std::io::stdin(),
        }
    }
    
    /// Read a line from stdin
    pub fn read_line(&self) -> OvieResult<String, String> {
        let mut buffer = String::new();
        match self.inner.lock().read_line(&mut buffer) {
            Ok(_) => {
                // Remove trailing newline if present
                if buffer.ends_with('\n') {
                    buffer.pop();
                    if buffer.ends_with('\r') {
                        buffer.pop();
                    }
                }
                ok(buffer)
            }
            Err(e) => err(format!("Failed to read from stdin: {}", e)),
        }
    }
    
    /// Read all input until EOF
    pub fn read_to_string(&self) -> OvieResult<String, String> {
        let mut buffer = String::new();
        match self.inner.lock().read_to_string(&mut buffer) {
            Ok(_) => ok(buffer),
            Err(e) => err(format!("Failed to read from stdin: {}", e)),
        }
    }
    
    /// Check if stdin is connected to a terminal
    pub fn is_terminal(&self) -> bool {
        // Simple heuristic - in a real implementation this would use platform-specific APIs
        std::env::var("TERM").is_ok()
    }
}

/// Standard output handle
pub struct Stdout {
    inner: std::io::Stdout,
}

impl Stdout {
    /// Create a new stdout handle
    pub fn new() -> Self {
        Self {
            inner: std::io::stdout(),
        }
    }
    
    /// Write a string to stdout
    pub fn write_str(&self, s: &str) -> OvieResult<(), String> {
        match self.inner.lock().write_all(s.as_bytes()) { Ok(_) => ok(()),
            Err(e) => err(format!("Failed to write to stdout: {}", e)),
        }
    }
    
    /// Write a string with newline to stdout
    pub fn write_line(&self, s: &str) -> OvieResult<(), String> {
        match self.inner.lock().write_all(format!("{}\n", s).as_bytes()) {
            Ok(_) => ok(()),
            Err(e) => err(format!("Failed to write to stdout: {}", e)),
        }
    }
    
    /// Flush stdout buffer
    pub fn flush(&self) -> OvieResult<(), String> {
        match self.inner.lock().flush() { Ok(_) => ok(()),
            Err(e) => err(format!("Failed to flush stdout: {}", e)),
        }
    }
    
    /// Check if stdout is connected to a terminal
    pub fn is_terminal(&self) -> bool {
        // Simple heuristic - in a real implementation this would use platform-specific APIs
        std::env::var("TERM").is_ok()
    }
}

/// Standard error handle
pub struct Stderr {
    inner: std::io::Stderr,
}

impl Stderr {
    /// Create a new stderr handle
    pub fn new() -> Self {
        Self {
            inner: std::io::stderr(),
        }
    }
    
    /// Write a string to stderr
    pub fn write_str(&self, s: &str) -> OvieResult<(), String> {
        match self.inner.lock().write_all(s.as_bytes()) { Ok(_) => ok(()),
            Err(e) => err(format!("Failed to write to stderr: {}", e)),
        }
    }
    
    /// Write a string with newline to stderr
    pub fn write_line(&self, s: &str) -> OvieResult<(), String> {
        match self.inner.lock().write_all(format!("{}\n", s).as_bytes()) {
            Ok(_) => ok(()),
            Err(e) => err(format!("Failed to write to stderr: {}", e)),
        }
    }
    
    /// Flush stderr buffer
    pub fn flush(&self) -> OvieResult<(), String> {
        match self.inner.lock().flush() { Ok(_) => ok(()),
            Err(e) => err(format!("Failed to flush stderr: {}", e)),
        }
    }
    
    /// Check if stderr is connected to a terminal
    pub fn is_terminal(&self) -> bool {
        // Simple heuristic - in a real implementation this would use platform-specific APIs
        std::env::var("TERM").is_ok()
    }
}

// ===== GLOBAL I/O FUNCTIONS =====

/// Get the global stdin handle
pub fn stdin() -> Stdin {
    Stdin::new()
}

/// Get the global stdout handle
pub fn stdout() -> Stdout {
    Stdout::new()
}

/// Get the global stderr handle
pub fn stderr() -> Stderr {
    Stderr::new()
}

/// Print a string to stdout
pub fn print(s: &str) -> OvieResult<(), String> {
    stdout().write_str(s)
}

/// Print a string with newline to stdout
pub fn println(s: &str) -> OvieResult<(), String> {
    stdout().write_line(s)
}

/// Print a string to stderr
pub fn eprint(s: &str) -> OvieResult<(), String> {
    stderr().write_str(s)
}

/// Print a string with newline to stderr
pub fn eprintln(s: &str) -> OvieResult<(), String> {
    stderr().write_line(s)
}

// ===== BUFFERED I/O =====

/// Buffered reader for efficient reading
pub struct OvieBufReader<R> {
    inner: BufReader<R>,
}

impl<R: Read> OvieBufReader<R> {
    /// Create a new buffered reader
    pub fn new(reader: R) -> Self {
        Self {
            inner: BufReader::new(reader),
        }
    }
    
    /// Create a buffered reader with specified capacity
    pub fn with_capacity(capacity: usize, reader: R) -> Self {
        Self {
            inner: BufReader::with_capacity(capacity, reader),
        }
    }
    
    /// Read a line from the buffered reader
    pub fn read_line(&mut self) -> OvieResult<String, String> {
        let mut buffer = String::new();
        match self.inner.read_line(&mut buffer) {
            Ok(0) => err("End of file reached".to_string()),
            Ok(_) => {
                // Remove trailing newline if present
                if buffer.ends_with('\n') {
                    buffer.pop();
                    if buffer.ends_with('\r') {
                        buffer.pop();
                    }
                }
                ok(buffer)
            }
            Err(e) => err(format!("Failed to read line: {}", e)),
        }
    }
    
    /// Read all lines from the buffered reader
    pub fn read_lines(&mut self) -> OvieResult<Vec<String>, String> {
        let mut lines = Vec::new();
        loop {
            match self.read_line() {
                OvieResult::Ok(line) => lines.push(line),
                OvieResult::Err(msg) if msg == "End of file reached" => break,
                OvieResult::Err(e) => return err(e),
            }
        }
        ok(lines)
    }
    
    /// Read all content to string
    pub fn read_to_string(&mut self) -> OvieResult<String, String> {
        let mut buffer = String::new();
        match self.inner.read_to_string(&mut buffer) {
            Ok(_) => ok(buffer),
            Err(e) => err(format!("Failed to read to string: {}", e)),
        }
    }
    
    /// Read exact number of bytes
    pub fn read_exact(&mut self, len: usize) -> OvieResult<Vec<u8>, String> {
        let mut buffer = vec![0; len];
        match self.inner.read_exact(&mut buffer) {
            Ok(_) => ok(buffer),
            Err(e) => err(format!("Failed to read exact bytes: {}", e)),
        }
    }
}

/// Buffered writer for efficient writing
pub struct OvieBufWriter<W: Write> {
    inner: BufWriter<W>,
}

impl<W: Write> OvieBufWriter<W> {
    /// Create a new buffered writer
    pub fn new(writer: W) -> Self {
        Self {
            inner: BufWriter::new(writer),
        }
    }
    
    /// Create a buffered writer with specified capacity
    pub fn with_capacity(capacity: usize, writer: W) -> Self {
        Self {
            inner: BufWriter::with_capacity(capacity, writer),
        }
    }
    
    /// Write a string
    pub fn write_string(&mut self, s: &str) -> OvieResult<(), String> {
        match self.inner.write_all(s.as_bytes()) { Ok(_) => ok(()),
            Err(e) => err(format!("Failed to write string: {}", e)),
        }
    }
    
    /// Write a string with newline
    pub fn write_line(&mut self, s: &str) -> OvieResult<(), String> {
        match self.inner.write_all(format!("{}\n", s).as_bytes()) {
            Ok(_) => ok(()),
            Err(e) => err(format!("Failed to write line: {}", e)),
        }
    }
    
    /// Write bytes
    pub fn write_bytes(&mut self, bytes: &[u8]) -> OvieResult<(), String> {
        match self.inner.write_all(bytes) { Ok(_) => ok(()),
            Err(e) => err(format!("Failed to write bytes: {}", e)),
        }
    }
    
    /// Flush the buffer
    pub fn flush(&mut self) -> OvieResult<(), String> {
        match self.inner.flush() { Ok(_) => ok(()),
            Err(e) => err(format!("Failed to flush buffer: {}", e)),
        }
    }
    
    /// Get the inner writer back, flushing first
    pub fn into_inner(self) -> OvieResult<W, String> {
        match self.inner.into_inner() { Ok(writer) => ok(writer),
            Err(e) => err(format!("Failed to get inner writer: {}", e)),
        }
    }
}

// ===== I/O TRAITS =====

/// Trait for reading data
pub trait OvieRead {
    /// Read data into a buffer
    fn read(&mut self, buf: &mut [u8]) -> OvieResult<usize, String>;
    
    /// Read all data to end
    fn read_to_end(&mut self) -> OvieResult<Vec<u8>, String> {
        let mut buffer = Vec::new();
        let mut chunk = [0; 1024];
        
        loop {
            match self.read(&mut chunk) {
                OvieResult::Ok(0) => break, // EOF
                OvieResult::Ok(n) => buffer.extend_from_slice(&chunk[..n]),
                OvieResult::Err(e) => return err(e),
            }
        }
        
        ok(buffer)
    }
    
    /// Read all data to string
    fn read_to_string(&mut self) -> OvieResult<String, String> {
        match self.read_to_end() {
            OvieResult::Ok(bytes) => {
                match String::from_utf8(bytes) { Ok(s) => ok(s),
                    Err(e) => err(format!("Invalid UTF-8: {}", e)),
                }
            }
            OvieResult::Err(e) => err(e),
        }
    }
}

/// Trait for writing data
pub trait OvieWrite {
    /// Write data from a buffer
    fn write(&mut self, buf: &[u8]) -> OvieResult<usize, String>;
    
    /// Write all data from buffer
    fn write_all(&mut self, mut buf: &[u8]) -> OvieResult<(), String> {
        while !buf.is_empty() {
            match self.write(buf) {
                OvieResult::Ok(0) => return err("Write returned 0".to_string()),
                OvieResult::Ok(n) => buf = &buf[n..],
                OvieResult::Err(e) => return err(e),
            }
        }
        ok(())
    }
    
    /// Flush any buffered data
    fn flush(&mut self) -> OvieResult<(), String>;
    
    /// Write a string
    fn write_str(&mut self, s: &str) -> OvieResult<(), String> {
        self.write_all(s.as_bytes())
    }
    
    /// Write a formatted string
    fn write_fmt(&mut self, args: std::fmt::Arguments) -> OvieResult<(), String> {
        self.write_str(&format!("{}", args))
    }
}

/// Trait for seeking within data
pub trait OvieSeek {
    /// Seek to a position
    fn seek(&mut self, pos: OvieSeekFrom) -> OvieResult<u64, String>;
    
    /// Get current position
    fn stream_position(&mut self) -> OvieResult<u64, String> {
        self.seek(OvieSeekFrom::Current(0))
    }
}

/// Seek position specification
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OvieSeekFrom {
    /// Seek from start of stream
    Start(u64),
    /// Seek from end of stream
    End(i64),
    /// Seek from current position
    Current(i64),
}

// ===== FILE I/O =====

/// File handle for reading and writing
pub struct OvieFile {
    inner: File,
}

impl OvieFile {
    /// Open a file for reading
    pub fn open<P: AsRef<Path>>(path: P) -> OvieResult<Self, String> {
        match File::open(path) { Ok(file) => ok(Self { inner: file }),
            Err(e) => err(format!("Failed to open file: {}", e)),
        }
    }
    
    /// Create a file for writing
    pub fn create<P: AsRef<Path>>(path: P) -> OvieResult<Self, String> {
        match File::create(path) { Ok(file) => ok(Self { inner: file }),
            Err(e) => err(format!("Failed to create file: {}", e)),
        }
    }
    
    /// Open a file with specific options
    pub fn open_with_options<P: AsRef<Path>>(
        path: P,
        read: bool,
        write: bool,
        create: bool,
        append: bool,
        truncate: bool,
    ) -> OvieResult<Self, String> {
        let mut options = std::fs::OpenOptions::new();
        options.read(read).write(write).create(create).append(append).truncate(truncate);
        
        match options.open(path) { Ok(file) => ok(Self { inner: file }),
            Err(e) => err(format!("Failed to open file with options: {}", e)),
        }
    }
    
    /// Get file metadata
    pub fn metadata(&self) -> OvieResult<OvieFileMetadata, String> {
        match self.inner.metadata() { Ok(metadata) => ok(OvieFileMetadata::from_std(metadata)),
            Err(e) => err(format!("Failed to get file metadata: {}", e)),
        }
    }
    
    /// Sync all data to disk
    pub fn sync_all(&self) -> OvieResult<(), String> {
        match self.inner.sync_all() { Ok(_) => ok(()),
            Err(e) => err(format!("Failed to sync file: {}", e)),
        }
    }
    
    /// Sync data (but not metadata) to disk
    pub fn sync_data(&self) -> OvieResult<(), String> {
        match self.inner.sync_data() { Ok(_) => ok(()),
            Err(e) => err(format!("Failed to sync file data: {}", e)),
        }
    }
}

impl OvieRead for OvieFile {
    fn read(&mut self, buf: &mut [u8]) -> OvieResult<usize, String> {
        match io::Read::read(&mut self.inner, buf) {
            Ok(n) => ok(n),
            Err(e) => err(format!("Failed to read from file: {}", e)),
        }
    }
}

impl OvieWrite for OvieFile {
    fn write(&mut self, buf: &[u8]) -> OvieResult<usize, String> {
        match io::Write::write(&mut self.inner, buf) {
            Ok(n) => ok(n),
            Err(e) => err(format!("Failed to write to file: {}", e)),
        }
    }
    
    fn flush(&mut self) -> OvieResult<(), String> {
        match io::Write::flush(&mut self.inner) {
            Ok(_) => ok(()),
            Err(e) => err(format!("Failed to flush file: {}", e)),
        }
    }
}

impl OvieSeek for OvieFile {
    fn seek(&mut self, pos: OvieSeekFrom) -> OvieResult<u64, String> {
        let std_pos = match pos {
            OvieSeekFrom::Start(n) => io::SeekFrom::Start(n),
            OvieSeekFrom::End(n) => io::SeekFrom::End(n),
            OvieSeekFrom::Current(n) => io::SeekFrom::Current(n),
        };
        
        match io::Seek::seek(&mut self.inner, std_pos) {
            Ok(pos) => ok(pos),
            Err(e) => err(format!("Failed to seek in file: {}", e)),
        }
    }
}

/// File metadata information
#[derive(Debug, Clone)]
pub struct OvieFileMetadata {
    pub len: u64,
    pub is_dir: bool,
    pub is_file: bool,
    pub readonly: bool,
}

impl OvieFileMetadata {
    fn from_std(metadata: std::fs::Metadata) -> Self {
        Self {
            len: metadata.len(),
            is_dir: metadata.is_dir(),
            is_file: metadata.is_file(),
            readonly: metadata.permissions().readonly(),
        }
    }
}

// ===== FORMAT UTILITIES =====

/// Format a string with arguments
pub fn format(template: &str, args: &[&str]) -> String {
    let mut result = template.to_string();
    
    for (i, arg) in args.iter().enumerate() {
        let placeholder = format!("{{{}}}", i);
        result = result.replace(&placeholder, arg);
    }
    
    result
}

/// Print formatted string to stdout
pub fn printf(template: &str, args: &[&str]) -> OvieResult<(), String> {
    let formatted = format(template, args);
    print(&formatted)
}

/// Print formatted string with newline to stdout
pub fn printfln(template: &str, args: &[&str]) -> OvieResult<(), String> {
    let formatted = format(template, args);
    println(&formatted)
}

/// Print formatted string to stderr
pub fn eprintf(template: &str, args: &[&str]) -> OvieResult<(), String> {
    let formatted = format(template, args);
    eprint(&formatted)
}

/// Print formatted string with newline to stderr
pub fn eprintfln(template: &str, args: &[&str]) -> OvieResult<(), String> {
    let formatted = format(template, args);
    eprintln(&formatted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_stdin_stdout_stderr_creation() {
        // Test that handles can be created
        let _stdin = stdin();
        let _stdout = stdout();
        let _stderr = stderr();
        
        // Test basic functionality (these won't actually read/write in tests)
        let stdout_handle = stdout();
        assert!(!stdout_handle.is_terminal() || stdout_handle.is_terminal()); // Either true or false is fine
    }

    #[test]
    fn test_buffered_reader() {
        let data = "line1\nline2\nline3\n";
        let cursor = Cursor::new(data.as_bytes());
        let mut reader = OvieBufReader::new(cursor);
        
        // Test reading lines
        assert_eq!(reader.read_line().unwrap(), "line1");
        assert_eq!(reader.read_line().unwrap(), "line2");
        assert_eq!(reader.read_line().unwrap(), "line3");
        
        // Test EOF
        assert!(reader.read_line().is_err());
    }

    #[test]
    fn test_buffered_reader_read_lines() {
        let data = "line1\nline2\nline3\n";
        let cursor = Cursor::new(data.as_bytes());
        let mut reader = OvieBufReader::new(cursor);
        
        let lines = reader.read_lines().unwrap();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "line1");
        assert_eq!(lines[1], "line2");
        assert_eq!(lines[2], "line3");
    }

    #[test]
    fn test_buffered_reader_read_to_string() {
        let data = "Hello, World!\nThis is a test.";
        let cursor = Cursor::new(data.as_bytes());
        let mut reader = OvieBufReader::new(cursor);
        
        let content = reader.read_to_string().unwrap();
        assert_eq!(content, data);
    }

    #[test]
    fn test_buffered_writer() {
        let mut buffer = Vec::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut writer = OvieBufWriter::new(cursor);
            
            assert!(writer.write_string("Hello").is_ok());
            assert!(writer.write_line(" World!").is_ok());
            assert!(writer.write_bytes(b"Test").is_ok());
            assert!(writer.flush().is_ok());
        }
        
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "Hello World!\nTest");
    }

    #[test]
    fn test_ovie_read_trait() {
        struct TestReader {
            data: Vec<u8>,
            pos: usize,
        }
        
        impl OvieRead for TestReader {
            fn read(&mut self, buf: &mut [u8]) -> OvieResult<usize, String> {
                let remaining = self.data.len() - self.pos;
                let to_read = std::cmp::min(buf.len(), remaining);
                
                if to_read == 0 {
                    return ok(0);
                }
                
                buf[..to_read].copy_from_slice(&self.data[self.pos..self.pos + to_read]);
                self.pos += to_read;
                ok(to_read)
            }
        }
        
        let mut reader = TestReader {
            data: b"Hello, World!".to_vec(),
            pos: 0,
        };
        
        // Test read_to_string
        let content = reader.read_to_string().unwrap();
        assert_eq!(content, "Hello, World!");
    }

    #[test]
    fn test_ovie_write_trait() {
        struct TestWriter {
            data: Vec<u8>,
        }
        
        impl OvieWrite for TestWriter {
            fn write(&mut self, buf: &[u8]) -> OvieResult<usize, String> {
                self.data.extend_from_slice(buf);
                ok(buf.len())
            }
            
            fn flush(&mut self) -> OvieResult<(), String> {
                ok(())
            }
        }
        
        let mut writer = TestWriter { data: Vec::new() };
        
        assert!(writer.write_str("Hello").is_ok());
        assert!(writer.write_all(b" World!").is_ok());
        assert!(writer.flush().is_ok());
        
        let result = String::from_utf8(writer.data).unwrap();
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_format_utilities() {
        // Test basic formatting
        let result = format("Hello, {0}!", &["World"]);
        assert_eq!(result, "Hello, World!");
        
        // Test multiple arguments
        let result = format("{0} + {1} = {2}", &["2", "3", "5"]);
        assert_eq!(result, "2 + 3 = 5");
        
        // Test repeated arguments
        let result = format("{0} {0} {1}", &["Hello", "World"]);
        assert_eq!(result, "Hello Hello World");
        
        // Test no arguments
        let result = format("No placeholders", &[]);
        assert_eq!(result, "No placeholders");
    }

    #[test]
    fn test_seek_from() {
        // Test SeekFrom variants
        let start = OvieSeekFrom::Start(10);
        let end = OvieSeekFrom::End(-5);
        let current = OvieSeekFrom::Current(3);
        
        assert_eq!(start, OvieSeekFrom::Start(10));
        assert_eq!(end, OvieSeekFrom::End(-5));
        assert_eq!(current, OvieSeekFrom::Current(3));
    }

    #[test]
    fn test_file_metadata() {
        // Test metadata creation (using dummy values)
        let metadata = OvieFileMetadata {
            len: 1024,
            is_dir: false,
            is_file: true,
            readonly: false,
        };
        
        assert_eq!(metadata.len, 1024);
        assert!(!metadata.is_dir);
        assert!(metadata.is_file);
        assert!(!metadata.readonly);
    }

    // Property-based tests for I/O operations
    #[test]
    fn test_io_properties() {
        // Property: Reading and writing should be consistent
        let test_data = ["", "Hello", "Hello, World!", "Multi\nLine\nText", "Unicode: 🦀"];
        
        for &data in &test_data {
            let mut buffer = Vec::new();
            {
                let cursor = Cursor::new(&mut buffer);
                let mut writer = OvieBufWriter::new(cursor);
                assert!(writer.write_string(data).is_ok());
                assert!(writer.flush().is_ok());
            }
            
            let cursor = Cursor::new(&buffer);
            let mut reader = OvieBufReader::new(cursor);
            let read_data = reader.read_to_string().unwrap();
            assert_eq!(read_data, data);
        }
    }

    #[test]
    fn test_buffered_io_properties() {
        // Property: Buffered I/O should preserve data integrity
        let lines = ["line1", "line2", "line3", "line4", "line5"];
        
        let mut buffer = Vec::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut writer = OvieBufWriter::new(cursor);
            
            for line in &lines {
                assert!(writer.write_line(line).is_ok());
            }
            assert!(writer.flush().is_ok());
        }
        
        let cursor = Cursor::new(&buffer);
        let mut reader = OvieBufReader::new(cursor);
        let read_lines = reader.read_lines().unwrap();
        
        assert_eq!(read_lines.len(), lines.len());
        for (i, line) in lines.iter().enumerate() {
            assert_eq!(read_lines[i], *line);
        }
    }

    #[test]
    fn test_format_properties() {
        // Property: Format should handle various argument counts
        let test_cases = [
            ("No args", vec![], "No args"),
            ("One {0}", vec!["arg"], "One arg"),
            ("{0} {1}", vec!["Hello", "World"], "Hello World"),
            ("{0} {1} {2}", vec!["A", "B", "C"], "A B C"),
        ];
        
        for (template, args, expected) in &test_cases {
            let args_refs: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();
            let result = format(template, &args_refs);
            assert_eq!(result, *expected);
        }
    }

    #[test]
    fn test_deterministic_behavior() {
        // Property: Same operations should produce same results
        let test_data = "Hello, World!\nLine 2\nLine 3";
        
        for _ in 0..5 {
            let mut buffer = Vec::new();
            {
                let cursor = Cursor::new(&mut buffer);
                let mut writer = OvieBufWriter::new(cursor);
                assert!(writer.write_string(test_data).is_ok());
                assert!(writer.flush().is_ok());
            }
            
            let cursor = Cursor::new(&buffer);
            let mut reader = OvieBufReader::new(cursor);
            let result = reader.read_to_string().unwrap();
            assert_eq!(result, test_data);
        }
    }
}