// Distribution Package Builder
// Implements Task 13.1: Create package structure generator, binary/asset copying, and archive creation

use crate::error::{OvieError, OvieResult};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Write};

/// Platform target for distribution
#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    WindowsX64,
    LinuxX64,
    MacOSArm64,
    MacOSX64,
}

impl Platform {
    /// Get the platform name for package naming
    pub fn name(&self) -> &str {
        match self {
            Platform::WindowsX64 => "windows-x64",
            Platform::LinuxX64 => "linux-x64",
            Platform::MacOSArm64 => "macos-arm64",
            Platform::MacOSX64 => "macos-x64",
        }
    }

    /// Get the binary extension for this platform
    pub fn binary_extension(&self) -> &str {
        match self {
            Platform::WindowsX64 => ".exe",
            _ => "",
        }
    }

    /// Get the archive extension for this platform
    pub fn archive_extension(&self) -> &str {
        match self {
            Platform::WindowsX64 => ".zip",
            _ => ".tar.gz",
        }
    }
}

/// Distribution package structure
#[derive(Debug, Clone)]
pub struct PackageStructure {
    /// Root directory name
    pub root_dir: String,
    /// Binary files to include
    pub binaries: Vec<String>,
    /// Documentation files to include
    pub docs: Vec<String>,
    /// Example files to include
    pub examples: Vec<String>,
    /// Standard library files to include
    pub stdlib: Vec<String>,
    /// License and legal files
    pub legal: Vec<String>,
    /// Installation scripts
    pub install_scripts: Vec<String>,
}

impl PackageStructure {
    /// Create a new package structure for a version
    pub fn new(version: &str, platform: &Platform) -> Self {
        Self {
            root_dir: format!("ovie-{}-{}", version, platform.name()),
            binaries: vec![
                format!("oviec{}", platform.binary_extension()),
                format!("ovie{}", platform.binary_extension()),
            ],
            docs: vec![
                "README.md".to_string(),
                "docs/".to_string(),
            ],
            examples: vec![
                "examples/".to_string(),
            ],
            stdlib: vec![
                "std/".to_string(),
            ],
            legal: vec![
                "LICENSE".to_string(),
            ],
            install_scripts: match platform {
                Platform::WindowsX64 => vec!["install.bat".to_string()],
                _ => vec!["install.sh".to_string()],
            },
        }
    }
}

/// Distribution package builder
pub struct DistributionBuilder {
    /// Version being built
    version: String,
    /// Target platform
    platform: Platform,
    /// Source directory (workspace root)
    source_dir: PathBuf,
    /// Output directory for packages
    output_dir: PathBuf,
    /// Package structure
    structure: PackageStructure,
}

impl DistributionBuilder {
    /// Create a new distribution builder
    pub fn new(version: String, platform: Platform, source_dir: PathBuf, output_dir: PathBuf) -> Self {
        let structure = PackageStructure::new(&version, &platform);
        
        Self {
            version,
            platform,
            source_dir,
            output_dir,
            structure,
        }
    }

    /// Build the complete distribution package
    pub fn build(&self) -> OvieResult<PathBuf> {
        println!("Building distribution package for {} v{}", self.platform.name(), self.version);
        
        // Create package directory
        let package_dir = self.output_dir.join(&self.structure.root_dir);
        self.create_package_structure(&package_dir)?;
        
        // Copy binaries
        self.copy_binaries(&package_dir)?;
        
        // Copy assets (docs, examples, stdlib)
        self.copy_assets(&package_dir)?;
        
        // Copy legal files
        self.copy_legal_files(&package_dir)?;
        
        // Copy installation scripts
        self.copy_install_scripts(&package_dir)?;
        
        // Create archive
        let archive_path = self.create_archive(&package_dir)?;
        
        println!("✓ Package created: {}", archive_path.display());
        
        Ok(archive_path)
    }

    /// Create the package directory structure
    fn create_package_structure(&self, package_dir: &Path) -> OvieResult<()> {
        println!("  Creating package structure...");
        
        // Create root directory
        fs::create_dir_all(package_dir)
            .map_err(|e| OvieError::io_error(format!("Failed to create package directory: {}", e)))?;
        
        // Create subdirectories
        fs::create_dir_all(package_dir.join("docs"))
            .map_err(|e| OvieError::io_error(format!("Failed to create docs directory: {}", e)))?;
        
        fs::create_dir_all(package_dir.join("examples"))
            .map_err(|e| OvieError::io_error(format!("Failed to create examples directory: {}", e)))?;
        
        fs::create_dir_all(package_dir.join("std"))
            .map_err(|e| OvieError::io_error(format!("Failed to create std directory: {}", e)))?;
        
        Ok(())
    }

    /// Copy binary files to the package
    fn copy_binaries(&self, package_dir: &Path) -> OvieResult<()> {
        println!("  Copying binaries...");
        
        // Determine binary source directory based on platform
        let binary_source = self.source_dir.join("target").join("release");
        
        for binary_name in &self.structure.binaries {
            let source_path = binary_source.join(binary_name);
            let dest_path = package_dir.join(binary_name);
            
            if source_path.exists() {
                fs::copy(&source_path, &dest_path)
                    .map_err(|e| OvieError::io_error(format!("Failed to copy {}: {}", binary_name, e)))?;
                
                // Set executable permissions on Unix platforms
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = fs::metadata(&dest_path)
                        .map_err(|e| OvieError::io_error(format!("Failed to get permissions: {}", e)))?
                        .permissions();
                    perms.set_mode(0o755);
                    fs::set_permissions(&dest_path, perms)
                        .map_err(|e| OvieError::io_error(format!("Failed to set permissions: {}", e)))?;
                }
                
                println!("    ✓ {}", binary_name);
            } else {
                println!("    ⚠ {} not found (skipping)", binary_name);
            }
        }
        
        Ok(())
    }

    /// Copy asset files (docs, examples, stdlib) to the package
    fn copy_assets(&self, package_dir: &Path) -> OvieResult<()> {
        println!("  Copying assets...");
        
        // Copy documentation
        self.copy_directory_recursive(
            &self.source_dir.join("docs"),
            &package_dir.join("docs"),
        )?;
        println!("    ✓ Documentation");
        
        // Copy examples
        self.copy_directory_recursive(
            &self.source_dir.join("examples"),
            &package_dir.join("examples"),
        )?;
        println!("    ✓ Examples");
        
        // Copy standard library
        self.copy_directory_recursive(
            &self.source_dir.join("std"),
            &package_dir.join("std"),
        )?;
        println!("    ✓ Standard library");
        
        Ok(())
    }

    /// Copy legal files to the package
    fn copy_legal_files(&self, package_dir: &Path) -> OvieResult<()> {
        println!("  Copying legal files...");
        
        for legal_file in &self.structure.legal {
            let source_path = self.source_dir.join(legal_file);
            let dest_path = package_dir.join(legal_file);
            
            if source_path.exists() {
                fs::copy(&source_path, &dest_path)
                    .map_err(|e| OvieError::io_error(format!("Failed to copy {}: {}", legal_file, e)))?;
                println!("    ✓ {}", legal_file);
            }
        }
        
        // Copy README
        let readme_source = self.source_dir.join("README.md");
        let readme_dest = package_dir.join("README.md");
        if readme_source.exists() {
            fs::copy(&readme_source, &readme_dest)
                .map_err(|e| OvieError::io_error(format!("Failed to copy README: {}", e)))?;
            println!("    ✓ README.md");
        }
        
        Ok(())
    }

    /// Copy installation scripts to the package
    fn copy_install_scripts(&self, package_dir: &Path) -> OvieResult<()> {
        println!("  Copying installation scripts...");
        
        for script in &self.structure.install_scripts {
            let source_path = self.source_dir.join(script);
            let dest_path = package_dir.join(script);
            
            if source_path.exists() {
                fs::copy(&source_path, &dest_path)
                    .map_err(|e| OvieError::io_error(format!("Failed to copy {}: {}", script, e)))?;
                
                // Set executable permissions on Unix platforms
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = fs::metadata(&dest_path)
                        .map_err(|e| OvieError::io_error(format!("Failed to get permissions: {}", e)))?
                        .permissions();
                    perms.set_mode(0o755);
                    fs::set_permissions(&dest_path, perms)
                        .map_err(|e| OvieError::io_error(format!("Failed to set permissions: {}", e)))?;
                }
                
                println!("    ✓ {}", script);
            }
        }
        
        Ok(())
    }

    /// Recursively copy a directory
    fn copy_directory_recursive(&self, source: &Path, dest: &Path) -> OvieResult<()> {
        if !source.exists() {
            return Ok(()); // Skip if source doesn't exist
        }
        
        fs::create_dir_all(dest)
            .map_err(|e| OvieError::io_error(format!("Failed to create directory {}: {}", dest.display(), e)))?;
        
        for entry in fs::read_dir(source)
            .map_err(|e| OvieError::io_error(format!("Failed to read directory {}: {}", source.display(), e)))? 
        {
            let entry = entry
                .map_err(|e| OvieError::io_error(format!("Failed to read entry: {}", e)))?;
            let path = entry.path();
            let file_name = entry.file_name();
            let dest_path = dest.join(&file_name);
            
            if path.is_dir() {
                self.copy_directory_recursive(&path, &dest_path)?;
            } else {
                fs::copy(&path, &dest_path)
                    .map_err(|e| OvieError::io_error(format!("Failed to copy file {}: {}", path.display(), e)))?;
            }
        }
        
        Ok(())
    }

    /// Create an archive from the package directory
    fn create_archive(&self, package_dir: &Path) -> OvieResult<PathBuf> {
        println!("  Creating archive...");
        
        let archive_name = format!("{}{}", self.structure.root_dir, self.platform.archive_extension());
        let archive_path = self.output_dir.join(&archive_name);
        
        match self.platform {
            Platform::WindowsX64 => {
                self.create_zip_archive(package_dir, &archive_path)?;
            }
            _ => {
                self.create_tar_gz_archive(package_dir, &archive_path)?;
            }
        }
        
        Ok(archive_path)
    }

    /// Create a ZIP archive (for Windows)
    fn create_zip_archive(&self, source_dir: &Path, archive_path: &Path) -> OvieResult<()> {
        use std::io::Write;
        
        // Create ZIP file
        let file = fs::File::create(archive_path)
            .map_err(|e| OvieError::io_error(format!("Failed to create ZIP file: {}", e)))?;
        
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);
        
        // Walk the source directory and add all files
        self.add_directory_to_zip(&mut zip, source_dir, source_dir, options)?;
        
        zip.finish()
            .map_err(|e| OvieError::io_error(format!("Failed to finalize ZIP: {}", e)))?;
        
        println!("    ✓ Created ZIP archive: {}", archive_path.display());
        Ok(())
    }

    /// Recursively add directory contents to ZIP archive
    fn add_directory_to_zip<W: Write + io::Seek>(
        &self,
        zip: &mut zip::ZipWriter<W>,
        base_dir: &Path,
        current_dir: &Path,
        options: zip::write::FileOptions,
    ) -> OvieResult<()> {
        for entry in fs::read_dir(current_dir)
            .map_err(|e| OvieError::io_error(format!("Failed to read directory: {}", e)))?
        {
            let entry = entry
                .map_err(|e| OvieError::io_error(format!("Failed to read entry: {}", e)))?;
            let path = entry.path();
            let name = path.strip_prefix(base_dir.parent().unwrap_or(base_dir))
                .unwrap_or(&path);
            
            if path.is_file() {
                zip.start_file(name.to_string_lossy().to_string(), options)
                    .map_err(|e| OvieError::io_error(format!("Failed to start ZIP file entry: {}", e)))?;
                
                let contents = fs::read(&path)
                    .map_err(|e| OvieError::io_error(format!("Failed to read file: {}", e)))?;
                
                zip.write_all(&contents)
                    .map_err(|e| OvieError::io_error(format!("Failed to write to ZIP: {}", e)))?;
            } else if path.is_dir() {
                zip.add_directory(name.to_string_lossy().to_string(), options)
                    .map_err(|e| OvieError::io_error(format!("Failed to add directory to ZIP: {}", e)))?;
                
                self.add_directory_to_zip(zip, base_dir, &path, options)?;
            }
        }
        
        Ok(())
    }

    /// Create a tar.gz archive (for Unix platforms)
    fn create_tar_gz_archive(&self, source_dir: &Path, archive_path: &Path) -> OvieResult<()> {
        use flate2::Compression;
        use flate2::write::GzEncoder;
        
        // Create tar.gz file
        let tar_gz_file = fs::File::create(archive_path)
            .map_err(|e| OvieError::io_error(format!("Failed to create tar.gz file: {}", e)))?;
        
        let enc = GzEncoder::new(tar_gz_file, Compression::default());
        let mut tar = tar::Builder::new(enc);
        
        // Add the entire directory to the tar archive
        let dir_name = source_dir.file_name()
            .ok_or_else(|| OvieError::runtime_error("Invalid source directory name".to_string()))?;
        
        tar.append_dir_all(dir_name, source_dir)
            .map_err(|e| OvieError::io_error(format!("Failed to add directory to tar: {}", e)))?;
        
        tar.finish()
            .map_err(|e| OvieError::io_error(format!("Failed to finalize tar: {}", e)))?;
        
        println!("    ✓ Created tar.gz archive: {}", archive_path.display());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_platform_names() {
        assert_eq!(Platform::WindowsX64.name(), "windows-x64");
        assert_eq!(Platform::LinuxX64.name(), "linux-x64");
        assert_eq!(Platform::MacOSArm64.name(), "macos-arm64");
        assert_eq!(Platform::MacOSX64.name(), "macos-x64");
    }

    #[test]
    fn test_platform_extensions() {
        assert_eq!(Platform::WindowsX64.binary_extension(), ".exe");
        assert_eq!(Platform::LinuxX64.binary_extension(), "");
        assert_eq!(Platform::WindowsX64.archive_extension(), ".zip");
        assert_eq!(Platform::LinuxX64.archive_extension(), ".tar.gz");
    }

    #[test]
    fn test_package_structure_creation() {
        let structure = PackageStructure::new("2.2.0", &Platform::WindowsX64);
        
        assert_eq!(structure.root_dir, "ovie-2.2.0-windows-x64");
        assert!(structure.binaries.contains(&"oviec.exe".to_string()));
        assert!(structure.binaries.contains(&"ovie.exe".to_string()));
        assert!(structure.docs.contains(&"README.md".to_string()));
    }

    #[test]
    fn test_distribution_builder_creation() {
        let source_dir = env::current_dir().unwrap();
        let output_dir = source_dir.join("target").join("dist");
        
        let builder = DistributionBuilder::new(
            "2.2.0".to_string(),
            Platform::LinuxX64,
            source_dir.clone(),
            output_dir.clone(),
        );
        
        assert_eq!(builder.version, "2.2.0");
        assert_eq!(builder.platform, Platform::LinuxX64);
        assert_eq!(builder.source_dir, source_dir);
        assert_eq!(builder.output_dir, output_dir);
    }
}
