//! Integration tests for the std::time module implementation
//! 
//! These tests verify that the time module works correctly and follows
//! the offline-first design principles with deterministic behavior.

use oviec::stdlib::time::{
    now, from_unix_timestamp, from_unix_timestamp_nanos, from_date_time,
    duration_from_seconds, duration_from_millis, duration_from_micros,
    duration_from_nanos, duration_from_hms,
    instant_now, elapsed_since, duration_between,
    sleep, sleep_seconds, sleep_millis,
    is_valid_date, is_valid_time, is_leap_year, days_in_month,
    OvieTime, OvieDuration, OvieInstant, OvieDate, OvieTimeOfDay, OvieDateTime,
    SECONDS_PER_MINUTE, SECONDS_PER_HOUR, SECONDS_PER_DAY,
    NANOSECONDS_PER_SECOND, NANOSECONDS_PER_MILLISECOND, NANOSECONDS_PER_MICROSECOND,
};
use std::thread;
use std::time::Duration as StdDuration;

#[test]
fn test_time_creation() {
    // Test Unix timestamp creation
    let time = from_unix_timestamp(1609459200); // 2021-01-01 00:00:00 UTC
    assert_eq!(time.seconds, 1609459200);
    assert_eq!(time.nanoseconds, 0);
    
    // Test Unix timestamp with nanoseconds
    let time_with_nanos = from_unix_timestamp_nanos(1609459200, 123456789).unwrap();
    assert_eq!(time_with_nanos.seconds, 1609459200);
    assert_eq!(time_with_nanos.nanoseconds, 123456789);
    
    // Test invalid nanoseconds
    assert!(from_unix_timestamp_nanos(1609459200, NANOSECONDS_PER_SECOND).is_err());
}

#[test]
fn test_duration_creation() {
    // Test duration from seconds
    let dur = duration_from_seconds(60);
    assert_eq!(dur.seconds, 60);
    assert_eq!(dur.nanoseconds, 0);
    
    // Test duration from milliseconds
    let dur_millis = duration_from_millis(1500);
    assert_eq!(dur_millis.seconds, 1);
    assert_eq!(dur_millis.nanoseconds, 500_000_000);
    
    // Test duration from microseconds
    let dur_micros = duration_from_micros(1_500_000);
    assert_eq!(dur_micros.seconds, 1);
    assert_eq!(dur_micros.nanoseconds, 500_000_000);
    
    // Test duration from nanoseconds
    let dur_nanos = duration_from_nanos(1_500_000_000);
    assert_eq!(dur_nanos.seconds, 1);
    assert_eq!(dur_nanos.nanoseconds, 500_000_000);
    
    // Test duration from hours, minutes, seconds
    let dur_hms = duration_from_hms(1, 30, 45).unwrap();
    assert_eq!(dur_hms.seconds, 5445); // 1*3600 + 30*60 + 45
    
    // Test invalid HMS values
    assert!(duration_from_hms(1, 60, 0).is_err()); // Invalid minutes
    assert!(duration_from_hms(1, 0, 60).is_err()); // Invalid seconds
}

#[test]
fn test_time_arithmetic() {
    let time = from_unix_timestamp(1000);
    let duration = duration_from_seconds(500);
    
    // Test addition
    let later = time.add_duration(duration).unwrap();
    assert_eq!(later.seconds, 1500);
    
    // Test subtraction
    let earlier = later.sub_duration(duration).unwrap();
    assert_eq!(earlier.seconds, 1000);
    
    // Test duration between times
    let diff = later.duration_since(&earlier).unwrap();
    assert_eq!(diff.seconds, 500);
    
    // Test error cases
    assert!(time.sub_duration(duration_from_seconds(2000)).is_err()); // Underflow
    assert!(earlier.duration_since(&later).is_err()); // Later time
}

#[test]
fn test_duration_arithmetic() {
    let dur1 = duration_from_seconds(100);
    let dur2 = duration_from_seconds(50);
    
    // Test addition
    let sum = dur1.add(&dur2).unwrap();
    assert_eq!(sum.seconds, 150);
    
    // Test subtraction
    let diff = sum.sub(&dur2).unwrap();
    assert_eq!(diff.seconds, 100);
    
    // Test multiplication
    let doubled = dur1.mul(2).unwrap();
    assert_eq!(doubled.seconds, 200);
    
    // Test division
    let halved = doubled.div(2).unwrap();
    assert_eq!(halved.seconds, 100);
    
    // Test division by zero
    assert!(dur1.div(0).is_err());
    
    // Test subtraction underflow
    assert!(dur2.sub(&dur1).is_err());
}

#[test]
fn test_duration_conversions() {
    let dur = duration_from_hms(1, 30, 45).unwrap(); // 5445 seconds
    
    // Test as_seconds_f64
    let seconds_f64 = dur.as_seconds_f64();
    assert_eq!(seconds_f64, 5445.0);
    
    // Test as_millis
    let millis = dur.as_millis();
    assert_eq!(millis, 5445000);
    
    // Test as_micros
    let micros = dur.as_micros();
    assert_eq!(micros, 5445000000);
    
    // Test as_nanos
    let nanos = dur.as_nanos();
    assert_eq!(nanos, 5445000000000);
    
    // Test is_zero
    assert!(!dur.is_zero());
    assert!(duration_from_seconds(0).is_zero());
}

#[test]
fn test_instant_operations() {
    let instant1 = instant_now();
    thread::sleep(StdDuration::from_millis(10));
    let instant2 = instant_now();
    
    // Test elapsed time
    let elapsed = elapsed_since(&instant1);
    assert!(elapsed.as_millis() >= 10);
    
    // Test duration between instants
    let between = duration_between(&instant1, &instant2).unwrap();
    assert!(between.as_millis() >= 10);
    
    // Test error case (reversed order)
    assert!(duration_between(&instant2, &instant1).is_err());
}

#[test]
fn test_sleep_operations() {
    let start = instant_now();
    
    // Test sleep with duration
    let sleep_duration = duration_from_millis(50);
    assert!(sleep(sleep_duration).is_ok());
    
    let elapsed = elapsed_since(&start);
    assert!(elapsed.as_millis() >= 50);
    
    // Test sleep_millis
    let start2 = instant_now();
    assert!(sleep_millis(50).is_ok());
    let elapsed2 = elapsed_since(&start2);
    assert!(elapsed2.as_millis() >= 50);
}

#[test]
fn test_date_validation() {
    // Test valid dates
    let valid_date = OvieDate { year: 2021, month: 2, day: 28 };
    assert!(is_valid_date(&valid_date));
    
    let leap_year_date = OvieDate { year: 2020, month: 2, day: 29 };
    assert!(is_valid_date(&leap_year_date));
    
    // Test invalid dates
    let invalid_month = OvieDate { year: 2021, month: 13, day: 1 };
    assert!(!is_valid_date(&invalid_month));
    
    let invalid_day = OvieDate { year: 2021, month: 2, day: 29 }; // Not a leap year
    assert!(!is_valid_date(&invalid_day));
    
    let zero_day = OvieDate { year: 2021, month: 1, day: 0 };
    assert!(!is_valid_date(&zero_day));
}

#[test]
fn test_time_validation() {
    // Test valid time
    let valid_time = OvieTimeOfDay {
        hour: 12,
        minute: 30,
        second: 45,
        nanosecond: 123456789,
    };
    assert!(is_valid_time(&valid_time));
    
    // Test invalid times
    let invalid_hour = OvieTimeOfDay {
        hour: 24,
        minute: 0,
        second: 0,
        nanosecond: 0,
    };
    assert!(!is_valid_time(&invalid_hour));
    
    let invalid_minute = OvieTimeOfDay {
        hour: 12,
        minute: 60,
        second: 0,
        nanosecond: 0,
    };
    assert!(!is_valid_time(&invalid_minute));
    
    let invalid_nanosecond = OvieTimeOfDay {
        hour: 12,
        minute: 30,
        second: 45,
        nanosecond: NANOSECONDS_PER_SECOND,
    };
    assert!(!is_valid_time(&invalid_nanosecond));
}

#[test]
fn test_leap_year() {
    // Test leap years
    assert!(is_leap_year(2000)); // Divisible by 400
    assert!(is_leap_year(2004)); // Divisible by 4, not by 100
    assert!(is_leap_year(2020)); // Divisible by 4, not by 100
    
    // Test non-leap years
    assert!(!is_leap_year(1900)); // Divisible by 100, not by 400
    assert!(!is_leap_year(2001)); // Not divisible by 4
    assert!(!is_leap_year(2100)); // Divisible by 100, not by 400
}

#[test]
fn test_days_in_month() {
    // Test regular months
    assert_eq!(days_in_month(2021, 1).unwrap(), 31); // January
    assert_eq!(days_in_month(2021, 4).unwrap(), 30); // April
    assert_eq!(days_in_month(2021, 2).unwrap(), 28); // February (non-leap year)
    assert_eq!(days_in_month(2020, 2).unwrap(), 29); // February (leap year)
    
    // Test invalid month
    assert!(days_in_month(2021, 0).is_err());
    assert!(days_in_month(2021, 13).is_err());
}

#[test]
fn test_date_time_conversion() {
    let date = OvieDate { year: 2021, month: 1, day: 1 };
    let time = OvieTimeOfDay {
        hour: 12,
        minute: 30,
        second: 45,
        nanosecond: 123456789,
    };
    
    // Test conversion from date/time to OvieTime
    let ovie_time = from_date_time(date.clone(), time.clone()).unwrap();
    
    // Test conversion back to date/time
    let converted_datetime = ovie_time.to_date_time();
    
    assert_eq!(converted_datetime.date, date);
    assert_eq!(converted_datetime.time, time);
}

#[test]
fn test_time_formatting() {
    let time = from_unix_timestamp(1609459200); // 2021-01-01 00:00:00 UTC
    
    // Test ISO 8601 formatting
    let iso_string = time.to_iso8601();
    assert!(iso_string.starts_with("2021-01-01T00:00:00"));
    assert!(iso_string.ends_with("Z"));
    
    // Test timestamp string formatting
    let timestamp_string = time.to_timestamp_string();
    assert_eq!(timestamp_string, "1609459200.000000000");
}

#[test]
fn test_duration_formatting() {
    // Test various duration formats
    let zero_dur = duration_from_seconds(0);
    assert_eq!(zero_dur.to_string(), "0s");
    
    let seconds_dur = duration_from_seconds(45);
    assert_eq!(seconds_dur.to_string(), "45s");
    
    let minutes_dur = duration_from_seconds(120);
    assert_eq!(minutes_dur.to_string(), "2m");
    
    let mixed_dur = duration_from_seconds(125);
    assert_eq!(mixed_dur.to_string(), "2m5s");
    
    let hours_dur = duration_from_seconds(3600);
    assert_eq!(hours_dur.to_string(), "1h");
    
    let complex_dur = duration_from_hms(1, 30, 45).unwrap();
    assert_eq!(complex_dur.to_string(), "1h30m45s");
    
    let millis_dur = duration_from_millis(500);
    assert_eq!(millis_dur.to_string(), "500ms");
}

#[test]
fn test_time_constants() {
    assert_eq!(SECONDS_PER_MINUTE, 60);
    assert_eq!(SECONDS_PER_HOUR, 3600);
    assert_eq!(SECONDS_PER_DAY, 86400);
    assert_eq!(NANOSECONDS_PER_SECOND, 1_000_000_000);
    assert_eq!(NANOSECONDS_PER_MILLISECOND, 1_000_000);
    assert_eq!(NANOSECONDS_PER_MICROSECOND, 1_000);
}

#[test]
fn test_deterministic_behavior() {
    // Test that same operations produce same results
    for _ in 0..10 {
        let time1 = from_unix_timestamp(1000);
        let time2 = from_unix_timestamp(1000);
        assert_eq!(time1, time2);
        
        let dur1 = duration_from_seconds(100);
        let dur2 = duration_from_seconds(100);
        assert_eq!(dur1, dur2);
        
        // Test arithmetic determinism
        let result1 = time1.add_duration(dur1).unwrap();
        let result2 = time2.add_duration(dur2).unwrap();
        assert_eq!(result1, result2);
    }
}

#[test]
fn test_cross_platform_consistency() {
    // Test that time operations work consistently across platforms
    let unix_timestamp = 1609459200; // 2021-01-01 00:00:00 UTC
    let time = from_unix_timestamp(unix_timestamp);
    
    // Test that conversion to date/time is consistent
    let datetime = time.to_date_time();
    assert_eq!(datetime.date.year, 2021);
    assert_eq!(datetime.date.month, 1);
    assert_eq!(datetime.date.day, 1);
    assert_eq!(datetime.time.hour, 0);
    assert_eq!(datetime.time.minute, 0);
    assert_eq!(datetime.time.second, 0);
    
    // Test that conversion back produces same timestamp
    let converted_time = from_date_time(datetime.date, datetime.time).unwrap();
    assert_eq!(converted_time.seconds, unix_timestamp);
}

#[test]
fn test_error_handling() {
    // Test various error conditions
    
    // Invalid date components
    let invalid_date = OvieDate { year: 2021, month: 13, day: 1 };
    let valid_time = OvieTimeOfDay { hour: 12, minute: 0, second: 0, nanosecond: 0 };
    assert!(from_date_time(invalid_date, valid_time).is_err());
    
    // Invalid time components
    let valid_date = OvieDate { year: 2021, month: 1, day: 1 };
    let invalid_time = OvieTimeOfDay { hour: 25, minute: 0, second: 0, nanosecond: 0 };
    assert!(from_date_time(valid_date, invalid_time).is_err());
    
    // Arithmetic overflow/underflow
    let time = from_unix_timestamp(100);
    let large_duration = duration_from_seconds(200);
    assert!(time.sub_duration(large_duration).is_err());
    
    // Division by zero
    let duration = duration_from_seconds(100);
    assert!(duration.div(0).is_err());
}

#[test]
fn test_nanosecond_precision() {
    // Test that nanosecond precision is maintained
    let time_with_nanos = from_unix_timestamp_nanos(1000, 123456789).unwrap();
    assert_eq!(time_with_nanos.nanoseconds, 123456789);
    
    let duration_with_nanos = duration_from_nanos(123456789);
    assert_eq!(duration_with_nanos.nanoseconds, 123456789);
    
    // Test arithmetic with nanoseconds
    let result = time_with_nanos.add_duration(duration_with_nanos).unwrap();
    assert_eq!(result.nanoseconds, 246913578); // 123456789 * 2
}

#[test]
fn test_boundary_conditions() {
    // Test edge cases and boundary conditions
    
    // Test year 1970 (Unix epoch)
    let epoch_date = OvieDate { year: 1970, month: 1, day: 1 };
    let epoch_time = OvieTimeOfDay { hour: 0, minute: 0, second: 0, nanosecond: 0 };
    let epoch_ovie_time = from_date_time(epoch_date, epoch_time).unwrap();
    assert_eq!(epoch_ovie_time.seconds, 0);
    
    // Test leap year boundary (February 29)
    let leap_date = OvieDate { year: 2020, month: 2, day: 29 };
    assert!(is_valid_date(&leap_date));
    
    let non_leap_date = OvieDate { year: 2021, month: 2, day: 29 };
    assert!(!is_valid_date(&non_leap_date));
    
    // Test maximum values
    let max_time = OvieTimeOfDay {
        hour: 23,
        minute: 59,
        second: 59,
        nanosecond: NANOSECONDS_PER_SECOND - 1,
    };
    assert!(is_valid_time(&max_time));
}