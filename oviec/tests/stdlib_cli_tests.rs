//! Tests for the Ovie Standard Library CLI Module
//! 
//! This module tests the CLI framework implementation to ensure it correctly
//! handles argument parsing, flag/option processing, help generation, and error handling.

use oviec::stdlib::cli::*;
use oviec::stdlib::core::{ok, err, some, none};

#[cfg(test)]
mod cli_tests {
    use super::*;

    // ===== BASIC APP CREATION TESTS =====

    #[test]
    fn test_app_creation() {
        let app = OvieApp::new("test-app".to_string())
            .version("1.0.0".to_string())
            .description("A test application".to_string())
            .author("Test Author".to_string());
        
        assert_eq!(app.name, "test-app");
        assert_eq!(app.version, "1.0.0");
        assert_eq!(app.description, "A test application");
        assert_eq!(app.author, "Test Author");
        assert!(app.commands.is_empty());
        assert!(app.global_flags.is_empty());
        assert!(app.global_options.is_empty());
    }

    #[test]
    fn test_command_creation() {
        let command = OvieCommand::new("test-cmd".to_string())
            .description("A test command".to_string())
            .alias("tc".to_string());
        
        assert_eq!(command.name, "test-cmd");
        assert_eq!(command.description, "A test command");
        assert_eq!(command.aliases, vec!["tc"]);
        assert!(command.flags.is_empty());
        assert!(command.options.is_empty());
        assert!(command.arguments.is_empty());
        assert!(command.subcommands.is_empty());
    }

    #[test]
    fn test_flag_creation() {
        let flag = OvieFlag::new("verbose".to_string())
            .short("v".to_string())
            .description("Enable verbose output".to_string())
            .required();
        
        assert_eq!(flag.name, "verbose");
        assert_eq!(flag.short, "v");
        assert_eq!(flag.description, "Enable verbose output");
        assert_eq!(flag.default_value, false);
        assert_eq!(flag.required, true);
    }

    #[test]
    fn test_option_creation() {
        let option = OvieCliOption::new("config".to_string())
            .short("c".to_string())
            .description("Configuration file".to_string())
            .default_value("config.toml".to_string())
            .required()
            .possible_values(vec!["config.toml".to_string(), "config.json".to_string()]);
        
        assert_eq!(option.name, "config");
        assert_eq!(option.short, "c");
        assert_eq!(option.description, "Configuration file");
        assert_eq!(option.default_value, "config.toml");
        assert_eq!(option.required, true);
        assert_eq!(option.takes_value, true);
        assert_eq!(option.possible_values, vec!["config.toml", "config.json"]);
    }

    #[test]
    fn test_argument_creation() {
        let arg = OvieArgument::new("input".to_string(), 0)
            .description("Input file".to_string())
            .required()
            .multiple();
        
        assert_eq!(arg.name, "input");
        assert_eq!(arg.description, "Input file");
        assert_eq!(arg.required, true);
        assert_eq!(arg.multiple, true);
        assert_eq!(arg.index, 0);
    }

    // ===== ARGUMENT PARSING TESTS =====

    #[test]
    fn test_parse_simple_command() {
        let app = OvieApp::new("test".to_string());
        let args = vec!["test".to_string()];
        
        let result = app.parse_args(args);
        assert!(result.is_ok());
        
        let context = result.unwrap();
        assert_eq!(context.app.name, "test");
        assert!(context.args.command_path.is_empty());
        assert!(context.args.flags.is_empty());
        assert!(context.args.options.is_empty());
        assert!(context.args.arguments.is_empty());
    }

    #[test]
    fn test_parse_help_flag() {
        let app = OvieApp::new("test".to_string());
        let args = vec!["test".to_string(), "--help".to_string()];
        
        let result = app.parse_args(args);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            OvieCliError::HelpRequested => {},
            _ => panic!("Expected HelpRequested error"),
        }
    }

    #[test]
    fn test_parse_version_flag() {
        let app = OvieApp::new("test".to_string());
        let args = vec!["test".to_string(), "--version".to_string()];
        
        let result = app.parse_args(args);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            OvieCliError::VersionRequested => {},
            _ => panic!("Expected VersionRequested error"),
        }
    }

    #[test]
    fn test_parse_long_flag() {
        let mut app = OvieApp::new("test".to_string());
        app.add_flag("verbose".to_string(), Some("v".to_string()), "Verbose output".to_string(), false);
        
        let args = vec!["test".to_string(), "--verbose".to_string()];
        
        let result = app.parse_args(args);
        assert!(result.is_ok());
        
        let context = result.unwrap();
        assert_eq!(context.args.has_flag("verbose"), true);
    }

    #[test]
    fn test_parse_short_flag() {
        let mut app = OvieApp::new("test".to_string());
        app.add_flag("verbose".to_string(), Some("v".to_string()), "Verbose output".to_string(), false);
        
        let args = vec!["test".to_string(), "-v".to_string()];
        
        let result = app.parse_args(args);
        assert!(result.is_ok());
        
        let context = result.unwrap();
        assert_eq!(context.args.has_flag("verbose"), true);
    }

    #[test]
    fn test_parse_long_option() {
        let mut app = OvieApp::new("test".to_string());
        app.add_option("config".to_string(), Some("c".to_string()), "Config file".to_string(), 
                      Some("default.toml".to_string()), false, true);
        
        let args = vec!["test".to_string(), "--config".to_string(), "custom.toml".to_string()];
        
        let result = app.parse_args(args);
        assert!(result.is_ok());
        
        let context = result.unwrap();
        match context.args.get_option("config") {
            crate::stdlib::core::OvieOption::Some(value) => assert_eq!(value, "custom.toml"),
            crate::stdlib::core::OvieOption::None => panic!("Expected config option to be set"),
        }
    }

    #[test]
    fn test_parse_short_option() {
        let mut app = OvieApp::new("test".to_string());
        app.add_option("config".to_string(), Some("c".to_string()), "Config file".to_string(), 
                      Some("default.toml".to_string()), false, true);
        
        let args = vec!["test".to_string(), "-c".to_string(), "custom.toml".to_string()];
        
        let result = app.parse_args(args);
        assert!(result.is_ok());
        
        let context = result.unwrap();
        match context.args.get_option("config") {
            crate::stdlib::core::OvieOption::Some(value) => assert_eq!(value, "custom.toml"),
            crate::stdlib::core::OvieOption::None => panic!("Expected config option to be set"),
        }
    }

    #[test]
    fn test_parse_option_with_equals() {
        let mut app = OvieApp::new("test".to_string());
        app.add_option("config".to_string(), Some("c".to_string()), "Config file".to_string(), 
                      Some("default.toml".to_string()), false, true);
        
        let args = vec!["test".to_string(), "--config=custom.toml".to_string()];
        
        let result = app.parse_args(args);
        assert!(result.is_ok());
        
        let context = result.unwrap();
        match context.args.get_option("config") {
            crate::stdlib::core::OvieOption::Some(value) => assert_eq!(value, "custom.toml"),
            crate::stdlib::core::OvieOption::None => panic!("Expected config option to be set"),
        }
    }

    #[test]
    fn test_parse_positional_arguments() {
        let app = OvieApp::new("test".to_string());
        let args = vec!["test".to_string(), "file1.txt".to_string(), "file2.txt".to_string()];
        
        let result = app.parse_args(args);
        assert!(result.is_ok());
        
        let context = result.unwrap();
        assert_eq!(context.args.arguments.len(), 2);
        assert_eq!(context.args.arguments[0], "file1.txt");
        assert_eq!(context.args.arguments[1], "file2.txt");
    }

    #[test]
    fn test_parse_subcommand() {
        let subcommand = OvieCommand::new("sub".to_string())
            .description("A subcommand".to_string());
        
        let app = OvieApp::new("test".to_string())
            .command(subcommand);
        
        let args = vec!["test".to_string(), "sub".to_string()];
        
        let result = app.parse_args(args);
        assert!(result.is_ok());
        
        let context = result.unwrap();
        assert_eq!(context.args.command_path, vec!["sub"]);
        assert_eq!(context.command.name, "sub");
    }

    // ===== ERROR HANDLING TESTS =====

    #[test]
    fn test_unknown_command_error() {
        let app = OvieApp::new("test".to_string())
            .command(OvieCommand::new("valid".to_string()));
        
        let args = vec!["test".to_string(), "invalid".to_string()];
        
        let result = app.parse_args(args);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            OvieCliError::UnknownCommand(cmd) => assert_eq!(cmd, "invalid"),
            _ => panic!("Expected UnknownCommand error"),
        }
    }

    #[test]
    fn test_unknown_flag_error() {
        let app = OvieApp::new("test".to_string());
        let args = vec!["test".to_string(), "--unknown".to_string()];
        
        let result = app.parse_args(args);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            OvieCliError::UnknownOption(flag) => assert_eq!(flag, "unknown"),
            _ => panic!("Expected UnknownOption error"),
        }
    }

    #[test]
    fn test_missing_required_flag_error() {
        let mut app = OvieApp::new("test".to_string());
        app.add_flag("required".to_string(), None, "Required flag".to_string(), true);
        
        let args = vec!["test".to_string()];
        
        let result = app.parse_args(args);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            OvieCliError::MissingRequiredFlag(flag) => assert_eq!(flag, "required"),
            _ => panic!("Expected MissingRequiredFlag error"),
        }
    }

    #[test]
    fn test_missing_required_option_error() {
        let mut app = OvieApp::new("test".to_string());
        app.add_option("required".to_string(), None, "Required option".to_string(), None, true, true);
        
        let args = vec!["test".to_string()];
        
        let result = app.parse_args(args);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            OvieCliError::MissingRequiredOption(option) => assert_eq!(option, "required"),
            _ => panic!("Expected MissingRequiredOption error"),
        }
    }

    // ===== VALIDATION TESTS =====

    #[test]
    fn test_app_validation_success() {
        let app = OvieApp::new("test-app".to_string())
            .version("1.0.0".to_string());
        
        let result = app.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_app_validation_empty_name() {
        let app = OvieApp::new("".to_string());
        
        let result = app.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Application name cannot be empty"));
    }

    #[test]
    fn test_app_validation_invalid_version() {
        let app = OvieApp::new("test".to_string())
            .version("invalid".to_string());
        
        let result = app.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid version format"));
    }

    #[test]
    fn test_command_validation_success() {
        let command = OvieCommand::new("test-cmd".to_string());
        
        let result = command.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_command_validation_empty_name() {
        let command = OvieCommand::new("".to_string());
        
        let result = command.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Command name cannot be empty"));
    }

    #[test]
    fn test_version_validation() {
        assert!(is_valid_version("1.0.0"));
        assert!(is_valid_version("10.20.30"));
        assert!(!is_valid_version("1.0"));
        assert!(!is_valid_version("1.0.0.0"));
        assert!(!is_valid_version("1.0.a"));
        assert!(!is_valid_version(""));
    }

    #[test]
    fn test_command_name_validation() {
        assert!(is_valid_command_name("test"));
        assert!(is_valid_command_name("test-cmd"));
        assert!(is_valid_command_name("test_cmd"));
        assert!(is_valid_command_name("test123"));
        assert!(!is_valid_command_name(""));
        assert!(!is_valid_command_name("123test"));
        assert!(!is_valid_command_name("test@cmd"));
    }

    #[test]
    fn test_flag_name_validation() {
        assert!(validate_flag_name("verbose"));
        assert!(validate_flag_name("verbose-mode"));
        assert!(validate_flag_name("verbose_mode"));
        assert!(!validate_flag_name(""));
        assert!(!validate_flag_name("verbose@mode"));
    }

    #[test]
    fn test_short_flag_validation() {
        assert!(validate_short_flag("v"));
        assert!(validate_short_flag("1"));
        assert!(!validate_short_flag(""));
        assert!(!validate_short_flag("vv"));
        assert!(!validate_short_flag("@"));
    }

    // ===== HELP GENERATION TESTS =====

    #[test]
    fn test_help_generation() {
        let app = OvieApp::new("test-app".to_string())
            .version("1.0.0".to_string())
            .description("A test application".to_string())
            .author("Test Author".to_string());
        
        let help = app.generate_help();
        assert!(help.contains("test-app"));
        assert!(help.contains("1.0.0"));
        assert!(help.contains("A test application"));
        assert!(help.contains("Test Author"));
    }

    #[test]
    fn test_usage_generation() {
        let mut app = OvieApp::new("test-app".to_string());
        app.add_flag("verbose".to_string(), Some("v".to_string()), "Verbose".to_string(), false);
        app.add_option("config".to_string(), Some("c".to_string()), "Config".to_string(), None, false, true);
        
        let usage = app.generate_usage();
        assert!(usage.contains("test-app"));
        assert!(usage.contains("[FLAGS]"));
        assert!(usage.contains("[OPTIONS]"));
    }

    #[test]
    fn test_flags_help_generation() {
        let mut app = OvieApp::new("test-app".to_string());
        app.add_flag("verbose".to_string(), Some("v".to_string()), "Enable verbose output".to_string(), false);
        app.add_flag("quiet".to_string(), Some("q".to_string()), "Suppress output".to_string(), true);
        
        let flags_help = app.generate_flags_help();
        assert!(flags_help.contains("FLAGS:"));
        assert!(flags_help.contains("-v, --verbose"));
        assert!(flags_help.contains("Enable verbose output"));
        assert!(flags_help.contains("-q, --quiet"));
        assert!(flags_help.contains("Suppress output"));
        assert!(flags_help.contains("[required]"));
    }

    #[test]
    fn test_options_help_generation() {
        let mut app = OvieApp::new("test-app".to_string());
        app.add_option("config".to_string(), Some("c".to_string()), "Configuration file".to_string(), 
                      Some("default.toml".to_string()), false, true);
        app.add_option("output".to_string(), Some("o".to_string()), "Output file".to_string(), 
                      None, true, true);
        
        let options_help = app.generate_options_help();
        assert!(options_help.contains("OPTIONS:"));
        assert!(options_help.contains("-c, --config <value>"));
        assert!(options_help.contains("Configuration file"));
        assert!(options_help.contains("[default: default.toml]"));
        assert!(options_help.contains("-o, --output <value>"));
        assert!(options_help.contains("Output file"));
        assert!(options_help.contains("[required]"));
    }

    // ===== UTILITY FUNCTION TESTS =====

    #[test]
    fn test_cli_error_to_string() {
        let error = OvieCliError::UnknownCommand("test".to_string());
        let error_str = cli_error_to_string(&error);
        assert_eq!(error_str, "Unknown command: test");
        
        let error = OvieCliError::MissingRequiredFlag("verbose".to_string());
        let error_str = cli_error_to_string(&error);
        assert_eq!(error_str, "Missing required flag: verbose");
    }

    #[test]
    fn test_find_flag() {
        let flag = OvieFlag::new("verbose".to_string()).short("v".to_string());
        let command = OvieCommand::new("test".to_string()).flag(flag.clone());
        let global_flags = vec![];
        
        let found = find_flag(&command, &global_flags, "verbose");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "verbose");
        
        let not_found = find_flag(&command, &global_flags, "nonexistent");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_find_flag_by_short() {
        let flag = OvieFlag::new("verbose".to_string()).short("v".to_string());
        let command = OvieCommand::new("test".to_string()).flag(flag.clone());
        let global_flags = vec![];
        
        let found = find_flag_by_short(&command, &global_flags, "v");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "verbose");
        
        let not_found = find_flag_by_short(&command, &global_flags, "x");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_sanitize_input() {
        assert_eq!(sanitize_input("hello world"), "hello world");
        assert_eq!(sanitize_input("test-file_name.txt"), "test-file_name.txt");
        assert_eq!(sanitize_input("test/path/file.txt"), "test/path/file.txt");
        assert_eq!(sanitize_input("test<>|&;"), "test");
        assert_eq!(sanitize_input("test\x00\x01\x02"), "test");
    }

    #[test]
    fn test_validate_file_path() {
        assert!(validate_file_path("test.txt").is_ok());
        assert!(validate_file_path("dir/test.txt").is_ok());
        assert!(validate_file_path("").is_err());
        assert!(validate_file_path("../test.txt").is_err());
        assert!(validate_file_path("/absolute/path").is_err());
        assert!(validate_file_path("C:\\absolute\\path").is_err());
    }

    #[test]
    fn test_parse_integer_arg() {
        assert_eq!(parse_integer_arg("42", "test").unwrap(), 42);
        assert_eq!(parse_integer_arg("-42", "test").unwrap(), -42);
        assert!(parse_integer_arg("not_a_number", "test").is_err());
        assert!(parse_integer_arg("42.5", "test").is_err());
    }

    #[test]
    fn test_parse_float_arg() {
        assert_eq!(parse_float_arg("42.5", "test").unwrap(), 42.5);
        assert_eq!(parse_float_arg("-42.5", "test").unwrap(), -42.5);
        assert_eq!(parse_float_arg("42", "test").unwrap(), 42.0);
        assert!(parse_float_arg("not_a_number", "test").is_err());
    }

    #[test]
    fn test_parse_bool_arg() {
        assert_eq!(parse_bool_arg("true", "test").unwrap(), true);
        assert_eq!(parse_bool_arg("false", "test").unwrap(), false);
        assert_eq!(parse_bool_arg("yes", "test").unwrap(), true);
        assert_eq!(parse_bool_arg("no", "test").unwrap(), false);
        assert_eq!(parse_bool_arg("1", "test").unwrap(), true);
        assert_eq!(parse_bool_arg("0", "test").unwrap(), false);
        assert_eq!(parse_bool_arg("on", "test").unwrap(), true);
        assert_eq!(parse_bool_arg("off", "test").unwrap(), false);
        assert!(parse_bool_arg("maybe", "test").is_err());
    }

    // ===== PROGRESS BAR TESTS =====

    #[test]
    fn test_progress_bar_creation() {
        let progress = OvieProgressBar::new(100)
            .message("Processing".to_string())
            .width(30);
        
        assert_eq!(progress.total, 100);
        assert_eq!(progress.current, 0);
        assert_eq!(progress.width, 30);
        assert_eq!(progress.message, "Processing");
    }

    #[test]
    fn test_progress_bar_update() {
        let mut progress = OvieProgressBar::new(100);
        
        progress.update(50);
        assert_eq!(progress.current, 50);
        
        progress.increment();
        assert_eq!(progress.current, 51);
        
        progress.finish();
        assert_eq!(progress.current, 100);
    }

    // ===== INTEGRATION TESTS =====

    #[test]
    fn test_complete_cli_workflow() {
        // Create a complex CLI application
        let verbose_flag = OvieFlag::new("verbose".to_string())
            .short("v".to_string())
            .description("Enable verbose output".to_string());
        
        let config_option = OvieCliOption::new("config".to_string())
            .short("c".to_string())
            .description("Configuration file".to_string())
            .default_value("config.toml".to_string())
            .takes_value(true);
        
        let input_arg = OvieArgument::new("input".to_string(), 0)
            .description("Input file".to_string())
            .required();
        
        let build_command = OvieCommand::new("build".to_string())
            .description("Build the project".to_string())
            .flag(verbose_flag)
            .option(config_option)
            .argument(input_arg);
        
        let app = OvieApp::new("myapp".to_string())
            .version("1.2.3".to_string())
            .description("My awesome application".to_string())
            .author("Test Author".to_string())
            .command(build_command);
        
        // Validate the application
        assert!(app.validate().is_ok());
        
        // Test parsing valid arguments
        let args = vec![
            "myapp".to_string(),
            "build".to_string(),
            "--verbose".to_string(),
            "--config".to_string(),
            "custom.toml".to_string(),
            "input.txt".to_string(),
        ];
        
        let result = app.parse_args(args);
        assert!(result.is_ok());
        
        let context = result.unwrap();
        assert_eq!(context.args.command_path, vec!["build"]);
        assert_eq!(context.command.name, "build");
        assert!(context.args.has_flag("verbose"));
        match context.args.get_option("config") {
            crate::stdlib::core::OvieOption::Some(value) => assert_eq!(value, "custom.toml"),
            crate::stdlib::core::OvieOption::None => panic!("Expected config option to be set"),
        }
        assert_eq!(context.args.arguments, vec!["input.txt"]);
        
        // Test help generation
        let help = app.generate_help();
        assert!(help.contains("myapp"));
        assert!(help.contains("1.2.3"));
        assert!(help.contains("My awesome application"));
        assert!(help.contains("Test Author"));
    }
}