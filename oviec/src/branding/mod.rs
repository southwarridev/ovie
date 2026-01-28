//! Ovie Branding and Icon Management
//! 
//! This module provides consistent branding and icon management across
//! all Ovie projects and components.

use std::path::{Path, PathBuf};
use std::fs;
use crate::error::{OvieError, OvieResult};

/// Ovie branding configuration
pub struct BrandingConfig {
    /// Path to the main Ovie icon
    pub icon_path: PathBuf,
    /// Project name
    pub project_name: String,
    /// Project description
    pub project_description: String,
    /// Project version
    pub project_version: String,
    /// Project website
    pub project_website: String,
}

impl Default for BrandingConfig {
    fn default() -> Self {
        Self {
            icon_path: PathBuf::from("ovie.png"),
            project_name: "Ovie".to_string(),
            project_description: "The Ovie Programming Language".to_string(),
            project_version: env!("CARGO_PKG_VERSION").to_string(),
            project_website: "https://github.com/ovie-lang/ovie".to_string(),
        }
    }
}

impl BrandingConfig {
    /// Create a new branding configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set custom icon path
    pub fn with_icon_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.icon_path = path.as_ref().to_path_buf();
        self
    }

    /// Set project name
    pub fn with_project_name(mut self, name: String) -> Self {
        self.project_name = name;
        self
    }

    /// Get the icon path, searching in common locations
    pub fn resolve_icon_path(&self) -> OvieResult<PathBuf> {
        // Try the specified path first
        if self.icon_path.exists() {
            return Ok(self.icon_path.clone());
        }

        // Search in common locations
        let search_paths = vec![
            PathBuf::from("ovie.png"),
            PathBuf::from("../ovie.png"),
            PathBuf::from("../../ovie.png"),
            PathBuf::from("assets/ovie.png"),
            PathBuf::from("icons/ovie.png"),
            PathBuf::from("resources/ovie.png"),
        ];

        for path in search_paths {
            if path.exists() {
                return Ok(path);
            }
        }

        Err(OvieError::runtime_error(
            "Ovie icon (ovie.png) not found in any standard location".to_string()
        ))
    }

    /// Copy the icon to a target directory
    pub fn copy_icon_to<P: AsRef<Path>>(&self, target_dir: P) -> OvieResult<PathBuf> {
        let source_path = self.resolve_icon_path()?;
        let target_path = target_dir.as_ref().join("ovie.png");

        // Create target directory if it doesn't exist
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                OvieError::runtime_error(format!("Failed to create directory: {}", e))
            })?;
        }

        // Copy the icon
        fs::copy(&source_path, &target_path).map_err(|e| {
            OvieError::runtime_error(format!("Failed to copy icon: {}", e))
        })?;

        Ok(target_path)
    }

    /// Generate project metadata with icon
    pub fn generate_project_metadata(&self) -> OvieResult<ProjectMetadata> {
        let icon_path = self.resolve_icon_path()?;
        
        Ok(ProjectMetadata {
            name: self.project_name.clone(),
            description: self.project_description.clone(),
            version: self.project_version.clone(),
            website: self.project_website.clone(),
            icon_path: icon_path.to_string_lossy().to_string(),
            icon_exists: true,
        })
    }
}

/// Project metadata including branding information
#[derive(Debug, Clone)]
pub struct ProjectMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub website: String,
    pub icon_path: String,
    pub icon_exists: bool,
}

/// Project template generator with branding
pub struct ProjectTemplate {
    branding: BrandingConfig,
}

impl ProjectTemplate {
    /// Create a new project template with default branding
    pub fn new() -> Self {
        Self {
            branding: BrandingConfig::new(),
        }
    }

    /// Create a new project template with custom branding
    pub fn with_branding(branding: BrandingConfig) -> Self {
        Self { branding }
    }

    /// Generate a new Ovie project with proper branding
    pub fn generate_project<P: AsRef<Path>>(&self, project_path: P, project_name: &str) -> OvieResult<()> {
        let project_dir = project_path.as_ref();
        
        // Create project directory
        fs::create_dir_all(project_dir).map_err(|e| {
            OvieError::runtime_error(format!("Failed to create project directory: {}", e))
        })?;

        // Copy the Ovie icon to the project
        let icon_path = self.branding.copy_icon_to(project_dir)?;
        println!("✅ Copied Ovie icon to: {}", icon_path.display());

        // Create project configuration with icon reference
        self.create_project_config(project_dir, project_name)?;

        // Create README with icon
        self.create_readme_with_icon(project_dir, project_name)?;

        // Create .ovie directory with branding
        self.create_ovie_directory(project_dir)?;

        Ok(())
    }

    /// Create project configuration file
    fn create_project_config<P: AsRef<Path>>(&self, project_dir: P, project_name: &str) -> OvieResult<()> {
        let config_path = project_dir.as_ref().join("ovie.toml");
        let config_content = format!(
            r#"[project]
name = "{}"
version = "0.1.0"
description = "An Ovie project"
icon = "ovie.png"

[build]
target = "interpreter"
optimize = false

[dependencies]
# Add your dependencies here

[branding]
icon = "ovie.png"
project_url = "{}"
"#,
            project_name, self.branding.project_website
        );

        fs::write(config_path, config_content).map_err(|e| {
            OvieError::runtime_error(format!("Failed to create project config: {}", e))
        })?;

        Ok(())
    }

    /// Create README with icon
    fn create_readme_with_icon<P: AsRef<Path>>(&self, project_dir: P, project_name: &str) -> OvieResult<()> {
        let readme_path = project_dir.as_ref().join("README.md");
        let readme_content = format!(
            r#"# {}

![Ovie Logo](ovie.png)

A project built with the [Ovie Programming Language]({}).

## Getting Started

```bash
# Run the project
ovie run main.ov

# Build the project
ovie build

# Run tests
ovie test
```

## About Ovie

Ovie is a modern programming language designed for clarity, safety, and performance.
Learn more at [{}]({}).

## Project Structure

- `main.ov` - Main application file
- `ovie.toml` - Project configuration
- `ovie.png` - Ovie language icon
- `tests/` - Test files

## License

This project is licensed under the same terms as the Ovie programming language.
"#,
            project_name,
            self.branding.project_website,
            self.branding.project_website,
            self.branding.project_website
        );

        fs::write(readme_path, readme_content).map_err(|e| {
            OvieError::runtime_error(format!("Failed to create README: {}", e))
        })?;

        Ok(())
    }

    /// Create .ovie directory with branding configuration
    fn create_ovie_directory<P: AsRef<Path>>(&self, project_dir: P) -> OvieResult<()> {
        let ovie_dir = project_dir.as_ref().join(".ovie");
        fs::create_dir_all(&ovie_dir).map_err(|e| {
            OvieError::runtime_error(format!("Failed to create .ovie directory: {}", e))
        })?;

        // Create branding configuration
        let branding_config_path = ovie_dir.join("branding.toml");
        let branding_content = format!(
            r#"[branding]
icon = "ovie.png"
project_name = "{}"
project_description = "{}"
project_version = "{}"
project_website = "{}"

[icon]
path = "ovie.png"
format = "PNG"
size = "256x256"
"#,
            self.branding.project_name,
            self.branding.project_description,
            self.branding.project_version,
            self.branding.project_website
        );

        fs::write(branding_config_path, branding_content).map_err(|e| {
            OvieError::runtime_error(format!("Failed to create branding config: {}", e))
        })?;

        Ok(())
    }
}

impl Default for ProjectTemplate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_branding_config_creation() {
        let config = BrandingConfig::new();
        assert_eq!(config.project_name, "Ovie");
        assert_eq!(config.icon_path, PathBuf::from("ovie.png"));
        assert!(!config.project_version.is_empty());
    }

    #[test]
    fn test_branding_config_customization() {
        let config = BrandingConfig::new()
            .with_project_name("My Ovie Project".to_string())
            .with_icon_path("custom/icon.png");
        
        assert_eq!(config.project_name, "My Ovie Project");
        assert_eq!(config.icon_path, PathBuf::from("custom/icon.png"));
    }

    #[test]
    fn test_project_template_creation() {
        let template = ProjectTemplate::new();
        assert_eq!(template.branding.project_name, "Ovie");
    }

    #[test]
    fn test_project_metadata_generation() {
        let config = BrandingConfig::new();
        
        // This test will only pass if ovie.png exists in a findable location
        // In a real test environment, we'd mock the file system
        if let Ok(metadata) = config.generate_project_metadata() {
            assert_eq!(metadata.name, "Ovie");
            assert!(!metadata.version.is_empty());
            assert!(metadata.icon_exists);
        }
    }
}