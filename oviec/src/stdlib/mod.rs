//! Ovie Standard Library Runtime Implementation
//! 
//! This module provides the runtime implementations of all standard library
//! modules that are specified in the .ov files in the std/ directory.

pub mod core;
pub mod math;
pub mod io;
pub mod fs;
pub mod time;
pub mod env;
pub mod cli;
pub mod test;
pub mod log;

// Re-export core types for easy access
pub use self::core::{
    OvieResult, OvieOption, OvieVec, OvieHashMap, OvieRc, OvieBox,
    ok, err, some, none,
    ovie_panic, ovie_assert, ovie_assert_eq, ovie_assert_ne,
    identity, swap, min, max, clamp, deterministic_hash,
    OvieVecIterator, OvieHashMapIterator, OvieHashMapKeysIterator, OvieHashMapValuesIterator,
};

// Re-export math functions for easy access
pub use self::math::{
    // Constants
    PI, E, TAU, SQRT_2, SQRT_3, LN_2, LN_10, LOG2_E, LOG10_E,
    INFINITY, NEG_INFINITY, NAN, EPSILON, MAX_INT, MIN_INT,
    
    // Checked arithmetic
    checked_add, checked_sub, checked_mul, checked_div, checked_mod,
    
    // Power and root functions
    pow, integer_pow, ovie_sqrt, cbrt,
    
    // Utility functions
    ovie_abs, sign, ovie_floor, ovie_ceil, ovie_round, truncate, fract,
    
    // Classification functions
    is_integer, is_finite, is_infinite, is_nan, is_normal, approx_eq,
    ovie_min, ovie_max, ovie_clamp,
    
    // Exponential and logarithmic functions
    ovie_exp, ovie_ln, log10, log2, log,
    
    // Special functions
    factorial, gcd, lcm,
};

// Re-export io functions for easy access
pub use self::io::{
    // Standard I/O handles
    Stdin, Stdout, Stderr, stdin, stdout, stderr,
    
    // Global I/O functions
    print, println, eprint, eprintln,
    
    // Buffered I/O
    OvieBufReader, OvieBufWriter,
    
    // I/O traits
    OvieRead, OvieWrite, OvieSeek, OvieSeekFrom,
    
    // File I/O
    OvieFileMetadata,
    
    // Format utilities
    format, printf, printfln, eprintf, eprintfln,
};

// Re-export fs functions for easy access
pub use self::fs::{
    // File operations
    open, open_with_mode, create, read_to_string, read_to_bytes,
    write_string, write_bytes, append_string,
    
    // Directory operations
    create_dir, create_dir_all, remove_dir, remove_dir_all, read_dir,
    
    // Path operations
    exists, is_file, is_dir, get_metadata, copy_file, rename_file, remove_file,
    
    // Path utilities
    join_path, parent_path, filename, extension,
    
    // Types
    OvieFile, OvieFileMode, OvieMetadata, OviePermissions, OvieDirEntry,
    
    // Security functions
    is_network_path, normalize_path,
};

// Re-export time functions for easy access
pub use self::time::{
    // Time creation
    now, from_unix_timestamp, from_unix_timestamp_nanos, from_date_time,
    
    // Duration creation
    duration_from_seconds, duration_from_millis, duration_from_micros,
    duration_from_nanos, duration_from_hms,
    
    // Instant operations
    instant_now, elapsed_since, duration_between,
    
    // Sleep operations
    sleep, sleep_seconds, sleep_millis,
    
    // Date/time utilities
    is_valid_date, is_valid_time, is_leap_year, days_in_month,
    
    // Types
    OvieTime, OvieDuration, OvieInstant, OvieDate, OvieTimeOfDay, OvieDateTime,
    
    // Constants
    SECONDS_PER_MINUTE, SECONDS_PER_HOUR, SECONDS_PER_DAY,
    NANOSECONDS_PER_SECOND, NANOSECONDS_PER_MILLISECOND, NANOSECONDS_PER_MICROSECOND,
};

// Re-export env functions for easy access
pub use self::env::{
    // Environment types
    OvieEnvironment, OvieSystemInfo, OvieProcessInfo,
    
    // Global environment functions
    var, var_or, set_var, remove_var, vars,
    
    // Directory operations
    current_dir, set_current_dir, home_dir, temp_dir,
    
    // Command line arguments
    args, program_name, args_without_program,
    
    // Process control
    exit, exit_success, exit_failure,
    
    // Path utilities
    split_path, absolute_path, is_absolute_path,
    
    // Common environment variables
    path, home, user, shell, editor,
    
    // Security utilities
    is_elevated, effective_user_id, real_user_id, is_root,
};

// Re-export cli functions for easy access
pub use self::cli::{
    // CLI types
    App, Command, Flag, Option, Argument, Args, CommandContext, CliError, CliErrorContext, ProgressBar,
    
    // Utility functions
    default_handler, default_help_template, cli_error_to_string,
    find_flag, find_flag_by_short, find_option, find_option_by_short, has_multiple_arg,
    
    // Common flags and options
    create_help_flag, create_version_flag, create_verbose_flag, create_quiet_flag,
    create_config_option, create_output_option,
    
    // Validation functions
    validate_flag_name, validate_short_flag, validate_option_name, check_conflicts,
    generate_suggestions, is_valid_version, is_valid_command_name, is_valid_argument_name,
    
    // Security and parsing utilities
    sanitize_input, validate_file_path, parse_integer_arg, parse_float_arg, parse_bool_arg,
    
    // Interactive utilities
    prompt, confirm, select,
    
    // Help formatting
    format_help_section, wrap_text,
};

// Re-export test functions for easy access
pub use self::test::{
    // Test result types
    TestResult, TestCase, TestStats, TestConfig, TestContext, TestRegistry,
    TestExecution, TestSuiteResult, TestRunner, TestReportFormatter,
    
    // Test function type
    TestFunction,
    
    // Registry functions
    init_test_registry, get_test_registry, register_test,
    
    // Test case creation helpers
    test_case, test_case_with_description, test_case_should_panic, 
    test_case_ignore, test_case_with_timeout, test_case_with_tags,
    
    // Assertion functions
    assert, assert_eq, assert_ne, assert_true, assert_false,
    assert_some, assert_none, assert_some_eq,
    assert_ok, assert_err, assert_ok_eq, assert_err_eq,
    assert_approx_eq, assert_in_range,
    assert_contains, assert_not_contains, assert_empty, assert_not_empty, assert_length,
    assert_str_contains, assert_str_starts_with, assert_str_ends_with, assert_str_matches,
    combine_results, pass, fail, skip, pass_if, fail_if,
    
    // Property-based testing
    Generator, PropertyConfig, PropertyResult, SimpleRng,
    IntRangeGenerator, FloatRangeGenerator, BoolGenerator, StringGenerator, VecGenerator,
    check_property, property, property_with_config,
    int_range, float_range, bools, strings, strings_with_length, vecs, vecs_with_length,
    
    // Test runner functions
    run_tests, run_tests_with_config, run_tests_and_exit, run_tests_with_config_and_exit,
};

// Re-export log functions for easy access
pub use self::log::{
    // Core logging types
    LogLevel, LogRecord, LogConfig, LogDestination, Logger,
    
    // Global logging functions
    init_logger, trace, debug, info, warn, error, fatal,
    is_enabled, set_level, get_level, flush,
    
    // Utility functions
    console_destination, file_destination, testing_destination,
};