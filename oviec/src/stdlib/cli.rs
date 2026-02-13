//! Ovie Standard Library - CLI Module Runtime Implementation
//! 
//! Command-line interface utilities for building CLI applications.
//! This module provides the runtime implementation of the std::cli module
//! specified in std/cli/mod.ov.

use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use crate::stdlib::core::{OvieResult, OvieOption, OvieVec, ok, err, some, none};
use std::result::Result;

// ===== CLI TYPES =====

/// Command-line application
#[derive(Debug, Clone)]
pub struct OvieApp {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub commands: Vec<OvieCommand>,
    pub global_flags: Vec<OvieFlag>,
    pub global_options: Vec<OvieCliOption>,
    pub help_template: String,
}

/// Command within an application
#[derive(Debug, Clone)]
pub struct OvieCommand {
    pub name: String,
    pub description: String,
    pub aliases: Vec<String>,
    pub flags: Vec<OvieFlag>,
    pub options: Vec<OvieCliOption>,
    pub arguments: Vec<OvieArgument>,
    pub subcommands: Vec<OvieCommand>,
    pub handler: fn(OvieCommandContext) -> OvieResult<(), String>,
}

/// Command-line flag (boolean option)
#[derive(Debug, Clone)]
pub struct OvieFlag {
    pub name: String,
    pub short: String,
    pub description: String,
    pub default_value: bool,
    pub required: bool,
}

/// Command-line option (key-value pair)
#[derive(Debug, Clone)]
pub struct OvieCliOption {
    pub name: String,
    pub short: String,
    pub description: String,
    pub default_value: String,
    pub required: bool,
    pub takes_value: bool,
    pub possible_values: Vec<String>,
}

/// Positional argument
#[derive(Debug, Clone)]
pub struct OvieArgument {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub multiple: bool,
    pub index: usize,
}

/// Parsed command-line arguments
#[derive(Debug, Clone)]
pub struct OvieArgs {
    pub command_path: Vec<String>,
    pub flags: HashMap<String, bool>,
    pub options: HashMap<String, String>,
    pub arguments: Vec<String>,
    pub raw_args: Vec<String>,
}

/// Context passed to command handlers
#[derive(Debug, Clone)]
pub struct OvieCommandContext {
    pub app: OvieApp,
    pub command: OvieCommand,
    pub args: OvieArgs,
}

/// CLI parsing error
#[derive(Debug, Clone)]
pub enum OvieCliError {
    UnknownCommand(String),
    UnknownFlag(String),
    UnknownOption(String),
    MissingRequiredFlag(String),
    MissingRequiredOption(String),
    MissingRequiredArgument(String),
    InvalidOptionValue(String, String),
    TooManyArguments,
    TooFewArguments,
    HelpRequested,
    VersionRequested,
}

// ===== APP BUILDER =====

impl OvieApp {
    /// Create a new CLI application
    pub fn new(name: String) -> Self {
        Self {
            name,
            version: "1.0.0".to_string(),
            description: String::new(),
            author: String::new(),
            commands: Vec::new(),
            global_flags: Vec::new(),
            global_options: Vec::new(),
            help_template: default_help_template(),
        }
    }
    
    /// Set application version
    pub fn version(mut self, version: String) -> Self {
        self.version = version;
        self
    }
    
    /// Set application description
    pub fn description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
    
    /// Set application author
    pub fn author(mut self, author: String) -> Self {
        self.author = author;
        self
    }
    
    /// Add a command to the application
    pub fn command(mut self, command: OvieCommand) -> Self {
        self.commands.push(command);
        self
    }
    
    /// Add a global flag
    pub fn flag(mut self, flag: OvieFlag) -> Self {
        self.global_flags.push(flag);
        self
    }
    
    /// Add a global option
    pub fn option(mut self, option: OvieCliOption) -> Self {
        self.global_options.push(option);
        self
    }
    
    /// Set custom help template
    pub fn help_template(mut self, template: String) -> Self {
        self.help_template = template;
        self
    }
}

// ===== COMMAND BUILDER =====

impl OvieCommand {
    /// Create a new command
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: String::new(),
            aliases: Vec::new(),
            flags: Vec::new(),
            options: Vec::new(),
            arguments: Vec::new(),
            subcommands: Vec::new(),
            handler: default_handler,
        }
    }
    
    /// Set command description
    pub fn description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
    
    /// Add command alias
    pub fn alias(mut self, alias: String) -> Self {
        self.aliases.push(alias);
        self
    }
    
    /// Add a flag to the command
    pub fn flag(mut self, flag: OvieFlag) -> Self {
        self.flags.push(flag);
        self
    }
    
    /// Add an option to the command
    pub fn option(mut self, option: OvieCliOption) -> Self {
        self.options.push(option);
        self
    }
    
    /// Add an argument to the command
    pub fn argument(mut self, argument: OvieArgument) -> Self {
        self.arguments.push(argument);
        self
    }
    
    /// Add a subcommand
    pub fn subcommand(mut self, subcommand: OvieCommand) -> Self {
        self.subcommands.push(subcommand);
        self
    }
    
    /// Set command handler
    pub fn handler(mut self, handler: fn(OvieCommandContext) -> OvieResult<(), String>) -> Self {
        self.handler = handler;
        self
    }
}

// ===== FLAG BUILDER =====

impl OvieFlag {
    /// Create a new flag
    pub fn new(name: String) -> Self {
        Self {
            name,
            short: String::new(),
            description: String::new(),
            default_value: false,
            required: false,
        }
    }
    
    /// Set short flag
    pub fn short(mut self, short: String) -> Self {
        self.short = short;
        self
    }
    
    /// Set flag description
    pub fn description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
    
    /// Set default value
    pub fn default_value(mut self, default: bool) -> Self {
        self.default_value = default;
        self
    }
    
    /// Make flag required
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
}

// ===== OPTION BUILDER =====

impl OvieCliOption {
    /// Create a new option
    pub fn new(name: String) -> Self {
        Self {
            name,
            short: String::new(),
            description: String::new(),
            default_value: String::new(),
            required: false,
            takes_value: true,
            possible_values: Vec::new(),
        }
    }
    
    /// Set short option
    pub fn short(mut self, short: String) -> Self {
        self.short = short;
        self
    }
    
    /// Set option description
    pub fn description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
    
    /// Set default value
    pub fn default_value(mut self, default: String) -> Self {
        self.default_value = default;
        self
    }
    
    /// Make option required
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    
    /// Set whether option takes a value
    pub fn takes_value(mut self, takes_value: bool) -> Self {
        self.takes_value = takes_value;
        self
    }
    
    /// Set possible values
    pub fn possible_values(mut self, values: Vec<String>) -> Self {
        self.possible_values = values;
        self
    }
}

// ===== ARGUMENT BUILDER =====

impl OvieArgument {
    /// Create a new argument
    pub fn new(name: String, index: usize) -> Self {
        Self {
            name,
            description: String::new(),
            required: false,
            multiple: false,
            index,
        }
    }
    
    /// Set argument description
    pub fn description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
    
    /// Make argument required
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    
    /// Allow multiple values
    pub fn multiple(mut self) -> Self {
        self.multiple = true;
        self
    }
}

// ===== ARGS HELPER METHODS =====

impl OvieArgs {
    /// Check if flag is present
    pub fn has_flag(&self, name: &str) -> bool {
        self.flags.get(name).copied().unwrap_or(false)
    }
    
    /// Get option value
    pub fn get_option(&self, name: &str) -> OvieOption<String> {
        if let Some(value) = self.options.get(name) {
            some(value.clone())
        } else {
            none()
        }
    }
    
    /// Get option value with default
    pub fn get_option_or(&self, name: &str, default: String) -> String {
        self.options.get(name).cloned().unwrap_or(default)
    }
    
    /// Get argument by index
    pub fn get_argument(&self, index: usize) -> OvieOption<String> {
        if index < self.arguments.len() {
            some(self.arguments[index].clone())
        } else {
            none()
        }
    }
    
    /// Get all arguments
    pub fn get_arguments(&self) -> Vec<String> {
        self.arguments.clone()
    }
}

// ===== UTILITY FUNCTIONS =====

/// Default command handler
pub fn default_handler(context: OvieCommandContext) -> OvieResult<(), String> {
    context.app.print_help();
    ok(())
}

/// Default help template
pub fn default_help_template() -> String {
    "{name} {version}\n{author}\n{description}\n\nUSAGE:\n    {name} [FLAGS] [OPTIONS] [SUBCOMMAND]\n\n{flags}{options}{commands}".to_string()
}

/// Convert CLI error to string
pub fn cli_error_to_string(error: &OvieCliError) -> String {
    match error {
        OvieCliError::UnknownCommand(cmd) => format!("Unknown command: {}", cmd),
        OvieCliError::UnknownFlag(flag) => format!("Unknown flag: {}", flag),
        OvieCliError::UnknownOption(option) => format!("Unknown option: {}", option),
        OvieCliError::MissingRequiredFlag(flag) => format!("Missing required flag: {}", flag),
        OvieCliError::MissingRequiredOption(option) => format!("Missing required option: {}", option),
        OvieCliError::MissingRequiredArgument(arg) => format!("Missing required argument: {}", arg),
        OvieCliError::InvalidOptionValue(option, value) => format!("Invalid value for option {}: {}", option, value),
        OvieCliError::TooManyArguments => "Too many arguments provided".to_string(),
        OvieCliError::TooFewArguments => "Too few arguments provided".to_string(),
        OvieCliError::HelpRequested => "Help requested".to_string(),
        OvieCliError::VersionRequested => "Version requested".to_string(),
    }
}

// ===== HELPER FUNCTIONS =====

/// Find flag by name
pub fn find_flag<'a>(command: &'a OvieCommand, global_flags: &'a [OvieFlag], name: &str) -> std::option::Option<&'a OvieFlag> {
    command.flags.iter()
        .chain(global_flags.iter())
        .find(|flag| flag.name == name)
}

/// Find flag by short name
pub fn find_flag_by_short<'a>(command: &'a OvieCommand, global_flags: &'a [OvieFlag], short: &str) -> std::option::Option<&'a OvieFlag> {
    command.flags.iter()
        .chain(global_flags.iter())
        .find(|flag| flag.short == short)
}

/// Find option by name
pub fn find_option<'a>(command: &'a OvieCommand, global_options: &'a [OvieCliOption], name: &str) -> std::option::Option<&'a OvieCliOption> {
    command.options.iter()
        .chain(global_options.iter())
        .find(|option| option.name == name)
}

/// Find option by short name
pub fn find_option_by_short<'a>(command: &'a OvieCommand, global_options: &'a [OvieCliOption], short: &str) -> std::option::Option<&'a OvieCliOption> {
    command.options.iter()
        .chain(global_options.iter())
        .find(|option| option.short == short)
}

/// Check if arguments have multiple flag
pub fn has_multiple_arg(arguments: &[OvieArgument]) -> bool {
    arguments.iter().any(|arg| arg.multiple)
}

// ===== ARGUMENT PARSING FRAMEWORK =====

impl OvieApp {
    /// Parse command line arguments and run the application
    pub fn run(self, args: Vec<String>) -> OvieResult<(), String> {
        match self.parse_args(args) {
            Ok(context) => {
                // Execute the command
                match self.execute_command(context) {
                    crate::stdlib::core::OvieResult::Ok(()) => ok(()),
                    crate::stdlib::core::OvieResult::Err(error) => {
                        eprintln!("Error: {}", error);
                        err(error)
                    }
                }
            }
            Err(error) => {
                match error {
                    OvieCliError::HelpRequested => {
                        self.print_help();
                        ok(())
                    }
                    OvieCliError::VersionRequested => {
                        self.print_version();
                        ok(())
                    }
                    _ => {
                        eprintln!("Error: {}", cli_error_to_string(&error));
                        eprintln!();
                        self.print_help();
                        err(cli_error_to_string(&error))
                    }
                }
            }
        }
    }
    
    /// Parse command line arguments
    pub fn parse_args(&self, args: Vec<String>) -> Result<OvieCommandContext, OvieCliError> {
        if args.is_empty() {
            return Err(OvieCliError::TooFewArguments);
        }
        
        let mut current_args = args.clone();
        let mut command_path = Vec::new();
        let mut current_command: std::option::Option<&OvieCommand> = std::option::Option::None;
        let mut parsed_flags = HashMap::new();
        let mut parsed_options = HashMap::new();
        let mut parsed_arguments = Vec::new();
        
        // Skip program name (first argument)
        if !current_args.is_empty() {
            current_args.remove(0);
        }
        
        // Check for global help/version flags
        for arg in &current_args {
            if arg == "--help" || arg == "-h" {
                return Err(OvieCliError::HelpRequested);
            }
            if arg == "--version" || arg == "-V" {
                return Err(OvieCliError::VersionRequested);
            }
        }
        
        // Parse command path
        let mut remaining_args = current_args;
        let mut commands = &self.commands;
        
        while !remaining_args.is_empty() {
            let first_arg = &remaining_args[0];
            
            // Check if it's a flag or option
            if first_arg.starts_with('-') {
                break;
            }
            
            // Look for matching command
            let mut found_command: std::option::Option<&OvieCommand> = std::option::Option::None;
            for command in commands {
                if command.name == *first_arg || command.aliases.contains(first_arg) {
                    found_command = std::option::Option::Some(command);
                    break;
                }
            }
            
            if let std::option::Option::Some(cmd) = found_command {
                command_path.push(first_arg.clone());
                current_command = std::option::Option::Some(cmd);
                commands = &cmd.subcommands;
                remaining_args.remove(0);
            } else {
                break;
            }
        }
        
        // If no command found, use default behavior
        let command = if let std::option::Option::Some(cmd) = current_command {
            cmd.clone()
        } else if !self.commands.is_empty() {
            let unknown_cmd = if !remaining_args.is_empty() {
                remaining_args[0].clone()
            } else {
                String::new()
            };
            return Err(OvieCliError::UnknownCommand(unknown_cmd));
        } else {
            // Create a default command for apps without subcommands
            OvieCommand {
                name: self.name.clone(),
                description: self.description.clone(),
                aliases: Vec::new(),
                flags: Vec::new(),
                options: Vec::new(),
                arguments: Vec::new(),
                subcommands: Vec::new(),
                handler: default_handler,
            }
        };
        
        // Parse flags and options
        let mut i = 0;
        while i < remaining_args.len() {
            let arg = &remaining_args[i];
            
            if arg.starts_with("--") {
                // Long flag or option
                let name = &arg[2..];
                
                if let Some(eq_pos) = name.find('=') {
                    // Option with value: --option=value
                    let option_name = &name[..eq_pos];
                    let option_value = &name[eq_pos + 1..];
                    
                    if find_option(&command, &self.global_options, option_name).is_none() {
                        return Err(OvieCliError::UnknownOption(option_name.to_string()));
                    }
                    
                    parsed_options.insert(option_name.to_string(), option_value.to_string());
                } else {
                    // Check if it's a flag or option
                    if let std::option::Option::Some(_flag) = find_flag(&command, &self.global_flags, name) {
                        parsed_flags.insert(name.to_string(), true);
                    } else if let std::option::Option::Some(opt) = find_option(&command, &self.global_options, name) {
                        if opt.takes_value {
                            // Next argument should be the value
                            if i + 1 >= remaining_args.len() {
                                return Err(OvieCliError::MissingRequiredOption(name.to_string()));
                            }
                            
                            i += 1;
                            let value = &remaining_args[i];
                            parsed_options.insert(name.to_string(), value.clone());
                        } else {
                            parsed_flags.insert(name.to_string(), true);
                        }
                    } else {
                        return Err(OvieCliError::UnknownOption(name.to_string()));
                    }
                }
            } else if arg.starts_with('-') && arg.len() > 1 {
                // Short flag(s) or option
                let chars = &arg[1..];
                
                for (j, ch) in chars.char_indices() {
                    let char_str = ch.to_string();
                    
                    if let std::option::Option::Some(flag) = find_flag_by_short(&command, &self.global_flags, &char_str) {
                        parsed_flags.insert(flag.name.clone(), true);
                    } else if let std::option::Option::Some(opt) = find_option_by_short(&command, &self.global_options, &char_str) {
                        if opt.takes_value {
                            // Rest of the argument or next argument is the value
                            if j + 1 < chars.len() {
                                let value = &chars[j + 1..];
                                parsed_options.insert(opt.name.clone(), value.to_string());
                                break;
                            } else if i + 1 < remaining_args.len() {
                                i += 1;
                                let value = &remaining_args[i];
                                parsed_options.insert(opt.name.clone(), value.clone());
                            } else {
                                return Err(OvieCliError::MissingRequiredOption(opt.name.clone()));
                            }
                        } else {
                            parsed_flags.insert(opt.name.clone(), true);
                        }
                    } else {
                        return Err(OvieCliError::UnknownFlag(char_str));
                    }
                }
            } else {
                // Positional argument
                parsed_arguments.push(arg.clone());
            }
            
            i += 1;
        }
        
        // Validate required flags and options
        for flag in &command.flags {
            if flag.required && !parsed_flags.contains_key(&flag.name) {
                return Err(OvieCliError::MissingRequiredFlag(flag.name.clone()));
            }
        }
        
        for flag in &self.global_flags {
            if flag.required && !parsed_flags.contains_key(&flag.name) {
                return Err(OvieCliError::MissingRequiredFlag(flag.name.clone()));
            }
        }
        
        for option in &command.options {
            if option.required && !parsed_options.contains_key(&option.name) {
                return Err(OvieCliError::MissingRequiredOption(option.name.clone()));
            }
        }
        
        for option in &self.global_options {
            if option.required && !parsed_options.contains_key(&option.name) {
                return Err(OvieCliError::MissingRequiredOption(option.name.clone()));
            }
        }
        
        // Validate arguments
        let required_args = command.arguments.iter().filter(|arg| arg.required).count();
        
        if parsed_arguments.len() < required_args {
            return Err(OvieCliError::TooFewArguments);
        }
        
        if parsed_arguments.len() > command.arguments.len() && !has_multiple_arg(&command.arguments) {
            return Err(OvieCliError::TooManyArguments);
        }
        
        // Create context
        let args = OvieArgs {
            command_path,
            flags: parsed_flags,
            options: parsed_options,
            arguments: parsed_arguments,
            raw_args: args,
        };
        
        let context = OvieCommandContext {
            app: self.clone(),
            command,
            args,
        };
        
        Ok(context)
    }
    
    /// Execute the parsed command
    pub fn execute_command(&self, context: OvieCommandContext) -> OvieResult<(), String> {
        (context.command.handler)(context)
    }
}
// ===== OPTION AND FLAG HANDLING =====

impl OvieApp {
    /// Add a global flag with builder pattern
    pub fn add_flag(&mut self, name: String, short: std::option::Option<String>, description: String, required: bool) -> &mut Self {
        let flag = OvieFlag {
            name,
            short: short.unwrap_or_default(),
            description,
            default_value: false,
            required,
        };
        self.global_flags.push(flag);
        self
    }
    
    /// Add a global option with builder pattern
    pub fn add_option(&mut self, name: String, short: std::option::Option<String>, description: String, 
                     default_value: std::option::Option<String>, required: bool, takes_value: bool) -> &mut Self {
        let option = OvieCliOption {
            name,
            short: short.unwrap_or_default(),
            description,
            default_value: default_value.unwrap_or_default(),
            required,
            takes_value,
            possible_values: Vec::new(),
        };
        self.global_options.push(option);
        self
    }
}

impl OvieCommand {
    /// Add a flag with builder pattern
    pub fn add_flag(&mut self, name: String, short: std::option::Option<String>, description: String, required: bool) -> &mut Self {
        let flag = OvieFlag {
            name,
            short: short.unwrap_or_default(),
            description,
            default_value: false,
            required,
        };
        self.flags.push(flag);
        self
    }
    
    /// Add an option with builder pattern
    pub fn add_option(&mut self, name: String, short: std::option::Option<String>, description: String,
                     default_value: std::option::Option<String>, required: bool, takes_value: bool) -> &mut Self {
        let option = OvieCliOption {
            name,
            short: short.unwrap_or_default(),
            description,
            default_value: default_value.unwrap_or_default(),
            required,
            takes_value,
            possible_values: Vec::new(),
        };
        self.options.push(option);
        self
    }
}

/// Advanced flag and option validation
impl OvieApp {
    /// Validate option values against possible values
    pub fn validate_option_values(&self, args: &OvieArgs) -> Result<(), OvieCliError> {
        // Check global options
        for option in &self.global_options {
            if !option.possible_values.is_empty() {
                if let Some(value) = args.options.get(&option.name) {
                    if !option.possible_values.contains(value) {
                        return Err(OvieCliError::InvalidOptionValue(
                            option.name.clone(),
                            value.clone()
                        ));
                    }
                }
            }
        }
        Ok(())
    }
    
    /// Get flag value with default
    pub fn get_flag_value(&self, args: &OvieArgs, flag_name: &str) -> bool {
        if let Some(&value) = args.flags.get(flag_name) {
            value
        } else {
            // Check for default value in flag definition
            for flag in &self.global_flags {
                if flag.name == flag_name {
                    return flag.default_value;
                }
            }
            false
        }
    }
    
    /// Get option value with default
    pub fn get_option_value(&self, args: &OvieArgs, option_name: &str) -> String {
        if let Some(value) = args.options.get(option_name) {
            value.clone()
        } else {
            // Check for default value in option definition
            for option in &self.global_options {
                if option.name == option_name {
                    return option.default_value.clone();
                }
            }
            String::new()
        }
    }
}

impl OvieCommand {
    /// Validate option values against possible values for command
    pub fn validate_option_values(&self, args: &OvieArgs) -> Result<(), OvieCliError> {
        // Check command options
        for option in &self.options {
            if !option.possible_values.is_empty() {
                if let Some(value) = args.options.get(&option.name) {
                    if !option.possible_values.contains(value) {
                        return Err(OvieCliError::InvalidOptionValue(
                            option.name.clone(),
                            value.clone()
                        ));
                    }
                }
            }
        }
        Ok(())
    }
    
    /// Get flag value with default for command
    pub fn get_flag_value(&self, args: &OvieArgs, flag_name: &str) -> bool {
        if let Some(&value) = args.flags.get(flag_name) {
            value
        } else {
            // Check for default value in flag definition
            for flag in &self.flags {
                if flag.name == flag_name {
                    return flag.default_value;
                }
            }
            false
        }
    }
    
    /// Get option value with default for command
    pub fn get_option_value(&self, args: &OvieArgs, option_name: &str) -> String {
        if let Some(value) = args.options.get(option_name) {
            value.clone()
        } else {
            // Check for default value in option definition
            for option in &self.options {
                if option.name == option_name {
                    return option.default_value.clone();
                }
            }
            String::new()
        }
    }
}

/// Convenience functions for creating common flags and options
pub fn create_help_flag() -> OvieFlag {
    OvieFlag::new("help".to_string())
        .short("h".to_string())
        .description("Show help information".to_string())
}

pub fn create_version_flag() -> OvieFlag {
    OvieFlag::new("version".to_string())
        .short("V".to_string())
        .description("Show version information".to_string())
}

pub fn create_verbose_flag() -> OvieFlag {
    OvieFlag::new("verbose".to_string())
        .short("v".to_string())
        .description("Enable verbose output".to_string())
}

pub fn create_quiet_flag() -> OvieFlag {
    OvieFlag::new("quiet".to_string())
        .short("q".to_string())
        .description("Suppress output".to_string())
}

pub fn create_config_option() -> OvieCliOption {
    OvieCliOption::new("config".to_string())
        .short("c".to_string())
        .description("Configuration file path".to_string())
        .takes_value(true)
}

pub fn create_output_option() -> OvieCliOption {
    OvieCliOption::new("output".to_string())
        .short("o".to_string())
        .description("Output file path".to_string())
        .takes_value(true)
}

/// Flag and option validation utilities
pub fn validate_flag_name(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

pub fn validate_short_flag(short: &str) -> bool {
    short.len() == 1 && short.chars().next().unwrap().is_alphanumeric()
}

pub fn validate_option_name(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

/// Check for conflicting flags and options
pub fn check_conflicts(flags: &[OvieFlag], options: &[OvieCliOption]) -> Result<(), String> {
    let mut names = std::collections::HashSet::new();
    let mut shorts = std::collections::HashSet::new();
    
    // Check flag conflicts
    for flag in flags {
        if !names.insert(&flag.name) {
            return Err(format!("Duplicate flag name: {}", flag.name));
        }
        if !flag.short.is_empty() && !shorts.insert(&flag.short) {
            return Err(format!("Duplicate short flag: {}", flag.short));
        }
    }
    
    // Check option conflicts
    for option in options {
        if !names.insert(&option.name) {
            return Err(format!("Duplicate option name: {}", option.name));
        }
        if !option.short.is_empty() && !shorts.insert(&option.short) {
            return Err(format!("Duplicate short option: {}", option.short));
        }
    }
    
    Ok(())
}
// ===== HELP GENERATION =====

impl OvieApp {
    /// Print application help
    pub fn print_help(&self) {
        let help_text = self.generate_help();
        println!("{}", help_text);
    }
    
    /// Generate help text
    pub fn generate_help(&self) -> String {
        let mut help_text = self.help_template.clone();
        
        // Replace template variables
        help_text = help_text.replace("{name}", &self.name);
        help_text = help_text.replace("{version}", &self.version);
        help_text = help_text.replace("{description}", &self.description);
        help_text = help_text.replace("{author}", &self.author);
        
        // Generate usage section
        let usage = self.generate_usage();
        help_text = help_text.replace("{usage}", &usage);
        
        // Generate commands section
        let commands_text = if !self.commands.is_empty() {
            self.generate_commands_help()
        } else {
            String::new()
        };
        help_text = help_text.replace("{commands}", &commands_text);
        
        // Generate flags section
        let flags_text = self.generate_flags_help();
        help_text = help_text.replace("{flags}", &flags_text);
        
        // Generate options section
        let options_text = self.generate_options_help();
        help_text = help_text.replace("{options}", &options_text);
        
        help_text
    }
    
    /// Generate usage line
    pub fn generate_usage(&self) -> String {
        let mut usage = format!("{}", self.name);
        
        // Add global flags
        if !self.global_flags.is_empty() {
            usage.push_str(" [FLAGS]");
        }
        
        // Add global options
        if !self.global_options.is_empty() {
            usage.push_str(" [OPTIONS]");
        }
        
        // Add subcommands
        if !self.commands.is_empty() {
            usage.push_str(" [SUBCOMMAND]");
        }
        
        usage
    }
    
    /// Generate commands help section
    pub fn generate_commands_help(&self) -> String {
        if self.commands.is_empty() {
            return String::new();
        }
        
        let mut commands_text = String::from("COMMANDS:\n");
        
        // Calculate max width for alignment
        let max_width = self.commands.iter()
            .map(|cmd| cmd.name.len())
            .max()
            .unwrap_or(0);
        
        for command in &self.commands {
            commands_text.push_str(&format!("    {:<width$}", command.name, width = max_width));
            if !command.description.is_empty() {
                commands_text.push_str(&format!("    {}", command.description));
            }
            commands_text.push('\n');
        }
        
        commands_text.push('\n');
        commands_text
    }
    
    /// Generate flags help section
    pub fn generate_flags_help(&self) -> String {
        if self.global_flags.is_empty() {
            return String::new();
        }
        
        let mut flags_text = String::from("FLAGS:\n");
        
        // Calculate max width for alignment
        let max_width = self.global_flags.iter()
            .map(|flag| {
                let mut width = flag.name.len() + 2; // for "--"
                if !flag.short.is_empty() {
                    width += flag.short.len() + 4; // for "-x, "
                }
                width
            })
            .max()
            .unwrap_or(0);
        
        for flag in &self.global_flags {
            let mut flag_str = String::from("    ");
            
            if !flag.short.is_empty() {
                flag_str.push_str(&format!("-{}, ", flag.short));
            }
            flag_str.push_str(&format!("--{}", flag.name));
            
            flags_text.push_str(&format!("{:<width$}", flag_str, width = max_width + 4));
            
            if !flag.description.is_empty() {
                flags_text.push_str(&format!("    {}", flag.description));
            }
            
            if flag.required {
                flags_text.push_str(" [required]");
            }
            
            flags_text.push('\n');
        }
        
        flags_text.push('\n');
        flags_text
    }
    
    /// Generate options help section
    pub fn generate_options_help(&self) -> String {
        if self.global_options.is_empty() {
            return String::new();
        }
        
        let mut options_text = String::from("OPTIONS:\n");
        
        // Calculate max width for alignment
        let max_width = self.global_options.iter()
            .map(|option| {
                let mut width = option.name.len() + 2; // for "--"
                if !option.short.is_empty() {
                    width += option.short.len() + 4; // for "-x, "
                }
                if option.takes_value {
                    width += 8; // for " <value>"
                }
                width
            })
            .max()
            .unwrap_or(0);
        
        for option in &self.global_options {
            let mut option_str = String::from("    ");
            
            if !option.short.is_empty() {
                option_str.push_str(&format!("-{}, ", option.short));
            }
            option_str.push_str(&format!("--{}", option.name));
            
            if option.takes_value {
                option_str.push_str(" <value>");
            }
            
            options_text.push_str(&format!("{:<width$}", option_str, width = max_width + 4));
            
            if !option.description.is_empty() {
                options_text.push_str(&format!("    {}", option.description));
            }
            
            if !option.default_value.is_empty() {
                options_text.push_str(&format!(" [default: {}]", option.default_value));
            }
            
            if option.required {
                options_text.push_str(" [required]");
            }
            
            if !option.possible_values.is_empty() {
                options_text.push_str(&format!(" [possible values: {}]", option.possible_values.join(", ")));
            }
            
            options_text.push('\n');
        }
        
        options_text.push('\n');
        options_text
    }
    
    /// Print application version
    pub fn print_version(&self) {
        println!("{} {}", self.name, self.version);
    }
}

impl OvieCommand {
    /// Print command help
    pub fn print_help(&self, app_name: &str) {
        let help_text = self.generate_help(app_name);
        println!("{}", help_text);
    }
    
    /// Generate command help text
    pub fn generate_help(&self, app_name: &str) -> String {
        let mut help_text = String::new();
        
        // Command name and description
        help_text.push_str(&format!("{} {}\n", app_name, self.name));
        if !self.description.is_empty() {
            help_text.push_str(&format!("{}\n", self.description));
        }
        help_text.push('\n');
        
        // Usage
        help_text.push_str("USAGE:\n");
        help_text.push_str(&format!("    {} {}", app_name, self.generate_usage()));
        help_text.push_str("\n\n");
        
        // Arguments
        if !self.arguments.is_empty() {
            help_text.push_str(&self.generate_arguments_help());
        }
        
        // Flags
        if !self.flags.is_empty() {
            help_text.push_str(&self.generate_command_flags_help());
        }
        
        // Options
        if !self.options.is_empty() {
            help_text.push_str(&self.generate_command_options_help());
        }
        
        // Subcommands
        if !self.subcommands.is_empty() {
            help_text.push_str(&self.generate_subcommands_help());
        }
        
        help_text
    }
    
    /// Generate command usage line
    pub fn generate_usage(&self) -> String {
        let mut usage = self.name.clone();
        
        // Add flags
        if !self.flags.is_empty() {
            usage.push_str(" [FLAGS]");
        }
        
        // Add options
        if !self.options.is_empty() {
            usage.push_str(" [OPTIONS]");
        }
        
        // Add arguments
        for arg in &self.arguments {
            if arg.required {
                usage.push_str(&format!(" <{}>", arg.name));
            } else {
                usage.push_str(&format!(" [{}]", arg.name));
            }
            
            if arg.multiple {
                usage.push_str("...");
            }
        }
        
        // Add subcommands
        if !self.subcommands.is_empty() {
            usage.push_str(" [SUBCOMMAND]");
        }
        
        usage
    }
    
    /// Generate arguments help section
    pub fn generate_arguments_help(&self) -> String {
        if self.arguments.is_empty() {
            return String::new();
        }
        
        let mut args_text = String::from("ARGS:\n");
        
        // Calculate max width for alignment
        let max_width = self.arguments.iter()
            .map(|arg| arg.name.len())
            .max()
            .unwrap_or(0);
        
        for arg in &self.arguments {
            args_text.push_str(&format!("    {:<width$}", arg.name, width = max_width));
            if !arg.description.is_empty() {
                args_text.push_str(&format!("    {}", arg.description));
            }
            
            if arg.required {
                args_text.push_str(" [required]");
            }
            
            if arg.multiple {
                args_text.push_str(" [multiple]");
            }
            
            args_text.push('\n');
        }
        
        args_text.push('\n');
        args_text
    }
    
    /// Generate command flags help section
    pub fn generate_command_flags_help(&self) -> String {
        if self.flags.is_empty() {
            return String::new();
        }
        
        let mut flags_text = String::from("FLAGS:\n");
        
        // Calculate max width for alignment
        let max_width = self.flags.iter()
            .map(|flag| {
                let mut width = flag.name.len() + 2; // for "--"
                if !flag.short.is_empty() {
                    width += flag.short.len() + 4; // for "-x, "
                }
                width
            })
            .max()
            .unwrap_or(0);
        
        for flag in &self.flags {
            let mut flag_str = String::from("    ");
            
            if !flag.short.is_empty() {
                flag_str.push_str(&format!("-{}, ", flag.short));
            }
            flag_str.push_str(&format!("--{}", flag.name));
            
            flags_text.push_str(&format!("{:<width$}", flag_str, width = max_width + 4));
            
            if !flag.description.is_empty() {
                flags_text.push_str(&format!("    {}", flag.description));
            }
            
            if flag.required {
                flags_text.push_str(" [required]");
            }
            
            flags_text.push('\n');
        }
        
        flags_text.push('\n');
        flags_text
    }
    
    /// Generate command options help section
    pub fn generate_command_options_help(&self) -> String {
        if self.options.is_empty() {
            return String::new();
        }
        
        let mut options_text = String::from("OPTIONS:\n");
        
        // Calculate max width for alignment
        let max_width = self.options.iter()
            .map(|option| {
                let mut width = option.name.len() + 2; // for "--"
                if !option.short.is_empty() {
                    width += option.short.len() + 4; // for "-x, "
                }
                if option.takes_value {
                    width += 8; // for " <value>"
                }
                width
            })
            .max()
            .unwrap_or(0);
        
        for option in &self.options {
            let mut option_str = String::from("    ");
            
            if !option.short.is_empty() {
                option_str.push_str(&format!("-{}, ", option.short));
            }
            option_str.push_str(&format!("--{}", option.name));
            
            if option.takes_value {
                option_str.push_str(" <value>");
            }
            
            options_text.push_str(&format!("{:<width$}", option_str, width = max_width + 4));
            
            if !option.description.is_empty() {
                options_text.push_str(&format!("    {}", option.description));
            }
            
            if !option.default_value.is_empty() {
                options_text.push_str(&format!(" [default: {}]", option.default_value));
            }
            
            if option.required {
                options_text.push_str(" [required]");
            }
            
            if !option.possible_values.is_empty() {
                options_text.push_str(&format!(" [possible values: {}]", option.possible_values.join(", ")));
            }
            
            options_text.push('\n');
        }
        
        options_text.push('\n');
        options_text
    }
    
    /// Generate subcommands help section
    pub fn generate_subcommands_help(&self) -> String {
        if self.subcommands.is_empty() {
            return String::new();
        }
        
        let mut subcommands_text = String::from("SUBCOMMANDS:\n");
        
        // Calculate max width for alignment
        let max_width = self.subcommands.iter()
            .map(|cmd| cmd.name.len())
            .max()
            .unwrap_or(0);
        
        for subcommand in &self.subcommands {
            subcommands_text.push_str(&format!("    {:<width$}", subcommand.name, width = max_width));
            if !subcommand.description.is_empty() {
                subcommands_text.push_str(&format!("    {}", subcommand.description));
            }
            subcommands_text.push('\n');
        }
        
        subcommands_text.push('\n');
        subcommands_text
    }
}

/// Help formatting utilities
pub fn format_help_section(title: &str, items: &[String]) -> String {
    if items.is_empty() {
        return String::new();
    }
    
    let mut section = format!("{}:\n", title);
    for item in items {
        section.push_str(&format!("    {}\n", item));
    }
    section.push('\n');
    section
}

pub fn wrap_text(text: &str, width: usize) -> String {
    if text.len() <= width {
        return text.to_string();
    }
    
    let mut result = String::new();
    let mut current_line = String::new();
    
    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > width {
            if !current_line.is_empty() {
                result.push_str(&current_line);
                result.push('\n');
                current_line.clear();
            }
        }
        
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }
    
    if !current_line.is_empty() {
        result.push_str(&current_line);
    }
    
    result
}
// ===== VALIDATION AND ERROR HANDLING =====

/// Comprehensive validation for CLI applications
impl OvieApp {
    /// Validate the entire application configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate app name
        if self.name.is_empty() {
            return Err("Application name cannot be empty".to_string());
        }
        
        // Validate version format (basic semver check)
        if !is_valid_version(&self.version) {
            return Err(format!("Invalid version format: {}", self.version));
        }
        
        // Check for conflicts in global flags and options
        check_conflicts(&self.global_flags, &self.global_options)?;
        
        // Validate each command
        for command in &self.commands {
            command.validate()?;
        }
        
        // Check for duplicate command names
        let mut command_names = std::collections::HashSet::new();
        for command in &self.commands {
            if !command_names.insert(&command.name) {
                return Err(format!("Duplicate command name: {}", command.name));
            }
            
            // Check aliases don't conflict with other command names
            for alias in &command.aliases {
                if !command_names.insert(alias) {
                    return Err(format!("Command alias '{}' conflicts with existing command", alias));
                }
            }
        }
        
        Ok(())
    }
    
    /// Validate arguments against command requirements
    pub fn validate_parsed_args(&self, context: &OvieCommandContext) -> Result<(), OvieCliError> {
        // Validate option values
        self.validate_option_values(&context.args)?;
        context.command.validate_option_values(&context.args)?;
        
        // Validate required arguments are present
        let required_arg_count = context.command.arguments.iter()
            .filter(|arg| arg.required)
            .count();
        
        if context.args.arguments.len() < required_arg_count {
            let missing_args: Vec<String> = context.command.arguments.iter()
                .take(required_arg_count)
                .skip(context.args.arguments.len())
                .map(|arg| arg.name.clone())
                .collect();
            
            return Err(OvieCliError::MissingRequiredArgument(
                missing_args.join(", ")
            ));
        }
        
        // Validate argument count doesn't exceed maximum
        let max_args = if has_multiple_arg(&context.command.arguments) {
            usize::MAX
        } else {
            context.command.arguments.len()
        };
        
        if context.args.arguments.len() > max_args {
            return Err(OvieCliError::TooManyArguments);
        }
        
        Ok(())
    }
}

impl OvieCommand {
    /// Validate command configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate command name
        if self.name.is_empty() {
            return Err("Command name cannot be empty".to_string());
        }
        
        if !is_valid_command_name(&self.name) {
            return Err(format!("Invalid command name: {}", self.name));
        }
        
        // Validate aliases
        for alias in &self.aliases {
            if !is_valid_command_name(alias) {
                return Err(format!("Invalid command alias: {}", alias));
            }
        }
        
        // Check for conflicts in command flags and options
        check_conflicts(&self.flags, &self.options)?;
        
        // Validate arguments
        self.validate_arguments()?;
        
        // Validate subcommands
        for subcommand in &self.subcommands {
            subcommand.validate()?;
        }
        
        // Check for duplicate subcommand names
        let mut subcommand_names = std::collections::HashSet::new();
        for subcommand in &self.subcommands {
            if !subcommand_names.insert(&subcommand.name) {
                return Err(format!("Duplicate subcommand name: {}", subcommand.name));
            }
        }
        
        Ok(())
    }
    
    /// Validate argument configuration
    pub fn validate_arguments(&self) -> Result<(), String> {
        let mut required_found = false;
        let mut multiple_found = false;
        
        for (i, arg) in self.arguments.iter().enumerate() {
            // Validate argument name
            if arg.name.is_empty() {
                return Err(format!("Argument at index {} has empty name", i));
            }
            
            if !is_valid_argument_name(&arg.name) {
                return Err(format!("Invalid argument name: {}", arg.name));
            }
            
            // Check argument ordering rules
            if multiple_found && arg.required {
                return Err("Required arguments cannot come after multiple arguments".to_string());
            }
            
            if !arg.required && !required_found {
                required_found = true;
            }
            
            if arg.required && required_found {
                return Err("Required arguments must come before optional arguments".to_string());
            }
            
            if arg.multiple {
                if multiple_found {
                    return Err("Only one argument can accept multiple values".to_string());
                }
                multiple_found = true;
            }
            
            // Validate index
            if arg.index != i {
                return Err(format!("Argument index mismatch: expected {}, got {}", i, arg.index));
            }
        }
        
        Ok(())
    }
}

/// Enhanced error handling and reporting
impl std::fmt::Display for OvieCliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", cli_error_to_string(self))
    }
}

impl std::error::Error for OvieCliError {}

/// Detailed error context for better debugging
#[derive(Debug, Clone)]
pub struct OvieCliErrorContext {
    pub error: OvieCliError,
    pub command_path: Vec<String>,
    pub raw_args: Vec<String>,
    pub suggestions: Vec<String>,
}

impl OvieCliErrorContext {
    pub fn new(error: OvieCliError, command_path: Vec<String>, raw_args: Vec<String>) -> Self {
        let suggestions = generate_suggestions(&error, &command_path, &raw_args);
        Self {
            error,
            command_path,
            raw_args,
            suggestions,
        }
    }
    
    pub fn print_detailed_error(&self) {
        eprintln!("Error: {}", cli_error_to_string(&self.error));
        
        if !self.command_path.is_empty() {
            eprintln!("Command path: {}", self.command_path.join(" "));
        }
        
        if !self.suggestions.is_empty() {
            eprintln!("\nSuggestions:");
            for suggestion in &self.suggestions {
                eprintln!("  - {}", suggestion);
            }
        }
    }
}

/// Generate helpful suggestions based on error type
pub fn generate_suggestions(error: &OvieCliError, _command_path: &[String], _raw_args: &[String]) -> Vec<String> {
    match error {
        OvieCliError::UnknownCommand(cmd) => {
            vec![
                format!("Did you mean to use a different command? Check available commands with --help"),
                format!("Unknown command: '{}'", cmd),
            ]
        }
        OvieCliError::UnknownFlag(flag) => {
            vec![
                format!("Unknown flag: '{}'", flag),
                "Use --help to see available flags".to_string(),
            ]
        }
        OvieCliError::UnknownOption(option) => {
            vec![
                format!("Unknown option: '{}'", option),
                "Use --help to see available options".to_string(),
            ]
        }
        OvieCliError::MissingRequiredFlag(flag) => {
            vec![
                format!("Add the required flag: --{}", flag),
            ]
        }
        OvieCliError::MissingRequiredOption(option) => {
            vec![
                format!("Add the required option: --{} <value>", option),
            ]
        }
        OvieCliError::MissingRequiredArgument(arg) => {
            vec![
                format!("Provide the required argument: {}", arg),
            ]
        }
        OvieCliError::InvalidOptionValue(option, value) => {
            vec![
                format!("Invalid value '{}' for option '{}'", value, option),
                "Check the help for valid values".to_string(),
            ]
        }
        OvieCliError::TooManyArguments => {
            vec![
                "Remove extra arguments".to_string(),
                "Use --help to see expected arguments".to_string(),
            ]
        }
        OvieCliError::TooFewArguments => {
            vec![
                "Provide all required arguments".to_string(),
                "Use --help to see required arguments".to_string(),
            ]
        }
        OvieCliError::HelpRequested | OvieCliError::VersionRequested => {
            vec![]
        }
    }
}

/// Validation utility functions
pub fn is_valid_version(version: &str) -> bool {
    // Basic semver validation (major.minor.patch)
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    
    parts.iter().all(|part| {
        part.chars().all(|c| c.is_ascii_digit()) && !part.is_empty()
    })
}

pub fn is_valid_command_name(name: &str) -> bool {
    !name.is_empty() && 
    name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') &&
    name.chars().next().unwrap().is_alphabetic()
}

pub fn is_valid_argument_name(name: &str) -> bool {
    !name.is_empty() && 
    name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') &&
    name.chars().next().unwrap().is_alphabetic()
}

/// Safe argument parsing with error recovery
impl OvieApp {
    /// Parse arguments with detailed error context
    pub fn parse_args_with_context(&self, args: Vec<String>) -> Result<OvieCommandContext, OvieCliErrorContext> {
        match self.parse_args(args.clone()) {
            Ok(context) => {
                // Additional validation
                if let Err(validation_error) = self.validate_parsed_args(&context) {
                    return Err(OvieCliErrorContext::new(
                        validation_error,
                        context.args.command_path.clone(),
                        args,
                    ));
                }
                Ok(context)
            }
            Err(error) => {
                Err(OvieCliErrorContext::new(
                    error,
                    Vec::new(),
                    args,
                ))
            }
        }
    }
    
    /// Run with enhanced error handling
    pub fn run_with_error_context(self, args: Vec<String>) -> OvieResult<(), String> {
        match self.parse_args_with_context(args) {
            Ok(context) => {
                match self.execute_command(context) {
                    crate::stdlib::core::OvieResult::Ok(()) => ok(()),
                    crate::stdlib::core::OvieResult::Err(error) => {
                        eprintln!("Execution error: {}", error);
                        err(error)
                    }
                }
            }
            Err(error_context) => {
                match error_context.error {
                    OvieCliError::HelpRequested => {
                        self.print_help();
                        ok(())
                    }
                    OvieCliError::VersionRequested => {
                        self.print_version();
                        ok(())
                    }
                    _ => {
                        error_context.print_detailed_error();
                        err(cli_error_to_string(&error_context.error))
                    }
                }
            }
        }
    }
}

/// Input sanitization and security
pub fn sanitize_input(input: &str) -> String {
    // Remove potentially dangerous characters
    input.chars()
        .filter(|&c| c.is_alphanumeric() || " -_./".contains(c))
        .collect()
}

pub fn validate_file_path(path: &str) -> Result<(), String> {
    // Basic path validation
    if path.is_empty() {
        return Err("Path cannot be empty".to_string());
    }
    
    // Check for path traversal attempts
    if path.contains("..") {
        return Err("Path traversal not allowed".to_string());
    }
    
    // Check for absolute paths (may be restricted in some contexts)
    if path.starts_with('/') || (path.len() > 1 && path.chars().nth(1) == Some(':')) {
        return Err("Absolute paths not allowed".to_string());
    }
    
    Ok(())
}

/// Argument type conversion with validation
pub fn parse_integer_arg(value: &str, arg_name: &str) -> Result<i64, String> {
    value.parse::<i64>()
        .map_err(|_| format!("Invalid integer value for {}: {}", arg_name, value))
}

pub fn parse_float_arg(value: &str, arg_name: &str) -> Result<f64, String> {
    value.parse::<f64>()
        .map_err(|_| format!("Invalid float value for {}: {}", arg_name, value))
}

pub fn parse_bool_arg(value: &str, arg_name: &str) -> Result<bool, String> {
    match value.to_lowercase().as_str() {
        "true" | "yes" | "1" | "on" => Ok(true),
        "false" | "no" | "0" | "off" => Ok(false),
        _ => Err(format!("Invalid boolean value for {}: {} (expected true/false, yes/no, 1/0, on/off)", arg_name, value))
    }
}
// ===== INTERACTIVE CLI UTILITIES =====

/// Prompt user for input
pub fn prompt(message: &str) -> OvieResult<String, String> {
    print!("{}", message);
    if let Err(e) = io::stdout().flush() {
        return err(e.to_string());
    }
    
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => ok(input.trim().to_string()),
        Err(e) => err(e.to_string()),
    }
}

/// Prompt user for confirmation (y/n)
pub fn confirm(message: &str) -> bool {
    loop {
        match prompt(&format!("{} (y/n): ", message)) {
            crate::stdlib::core::OvieResult::Ok(response) => {
                let answer = response.to_lowercase();
                if answer == "y" || answer == "yes" {
                    return true;
                } else if answer == "n" || answer == "no" {
                    return false;
                } else {
                    println!("Please enter 'y' or 'n'");
                }
            }
            crate::stdlib::core::OvieResult::Err(_) => continue,
        }
    }
}

/// Select from multiple options
pub fn select(message: &str, options: &[String]) -> OvieResult<usize, String> {
    println!("{}", message);
    
    for (i, option) in options.iter().enumerate() {
        println!("  {}) {}", i + 1, option);
    }
    
    loop {
        match prompt(&format!("Select option (1-{}): ", options.len())) {
            crate::stdlib::core::OvieResult::Ok(input) => {
                if let Ok(choice) = input.parse::<usize>() {
                    if choice >= 1 && choice <= options.len() {
                        return ok(choice - 1);
                    }
                }
                println!("Please enter a number between 1 and {}", options.len());
            }
            crate::stdlib::core::OvieResult::Err(_) => continue,
        }
    }
}

/// Progress bar for long-running operations
#[derive(Debug, Clone)]
pub struct OvieProgressBar {
    pub total: usize,
    pub current: usize,
    pub width: usize,
    pub message: String,
}

impl OvieProgressBar {
    /// Create a new progress bar
    pub fn new(total: usize) -> Self {
        Self {
            total,
            current: 0,
            width: 50,
            message: String::new(),
        }
    }
    
    /// Set progress bar message
    pub fn message(mut self, message: String) -> Self {
        self.message = message;
        self
    }
    
    /// Set progress bar width
    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }
    
    /// Update progress
    pub fn update(&mut self, current: usize) {
        self.current = current;
        self.render();
    }
    
    /// Increment progress
    pub fn increment(&mut self) {
        self.current += 1;
        self.render();
    }
    
    /// Finish progress bar
    pub fn finish(&mut self) {
        self.current = self.total;
        self.render();
        println!();
    }
    
    /// Render progress bar
    pub fn render(&self) {
        let percentage = if self.total > 0 { 
            (self.current as f64 / self.total as f64) * 100.0 
        } else { 
            0.0 
        };
        
        let filled = if self.total > 0 {
            ((self.current as f64 / self.total as f64) * self.width as f64) as usize
        } else {
            0
        };
        
        let empty = self.width.saturating_sub(filled);
        
        let mut bar = String::from("[");
        bar.push_str(&"=".repeat(filled));
        bar.push_str(&" ".repeat(empty));
        bar.push(']');
        
        let output = format!("\r{} {:.0}% ({}/{})", 
                           bar, percentage, self.current, self.total);
        
        if !self.message.is_empty() {
            print!("\r{}: {}", self.message, output);
        } else {
            print!("{}", output);
        }
        
        io::stdout().flush().unwrap_or(());
    }
}

// ===== MODULE EXPORTS =====

// Export all public types and functions for use by the Ovie runtime
pub use self::{
    OvieApp as App,
    OvieCommand as Command,
    OvieFlag as Flag,
    OvieCliOption as Option,
    OvieArgument as Argument,
    OvieArgs as Args,
    OvieCommandContext as CommandContext,
    OvieCliError as CliError,
    OvieCliErrorContext as CliErrorContext,
    OvieProgressBar as ProgressBar,
};