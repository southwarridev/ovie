//! Ovie Standard Library - Time Module Runtime Implementation
//! 
//! This module provides the runtime implementation of the std::time module
//! as specified in std/time/mod.ov. All operations are offline-first and
//! designed to work deterministically across platforms.

use crate::stdlib::{OvieResult, OvieOption, ok, err, some, none};
use std::time::{SystemTime, UNIX_EPOCH, Duration as StdDuration, Instant};
use std::thread;

// ===== TIME TYPES =====

/// Represents a point in time as seconds since Unix epoch
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct OvieTime {
    pub seconds: u64,
    pub nanoseconds: u32,
}

/// Represents a duration between two time points
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct OvieDuration {
    pub seconds: u64,
    pub nanoseconds: u32,
}

/// Represents an instant for measuring elapsed time
#[derive(Debug, Clone)]
pub struct OvieInstant {
    inner: Instant,
}

/// Date components
#[derive(Debug, Clone, PartialEq)]
pub struct OvieDate {
    pub year: i32,
    pub month: u8,   // 1-12
    pub day: u8,     // 1-31
}

/// Time components
#[derive(Debug, Clone, PartialEq)]
pub struct OvieTimeOfDay {
    pub hour: u8,    // 0-23
    pub minute: u8,  // 0-59
    pub second: u8,  // 0-59
    pub nanosecond: u32, // 0-999,999,999
}

/// Combined date and time
#[derive(Debug, Clone, PartialEq)]
pub struct OvieDateTime {
    pub date: OvieDate,
    pub time: OvieTimeOfDay,
}

// ===== TIME CONSTANTS =====

pub const SECONDS_PER_MINUTE: u64 = 60;
pub const SECONDS_PER_HOUR: u64 = 3600;
pub const SECONDS_PER_DAY: u64 = 86400;
pub const NANOSECONDS_PER_SECOND: u32 = 1_000_000_000;
pub const NANOSECONDS_PER_MILLISECOND: u32 = 1_000_000;
pub const NANOSECONDS_PER_MICROSECOND: u32 = 1_000;

// ===== TIME CREATION =====

/// Get the current time
pub fn now() -> OvieTime {
    let system_time = SystemTime::now();
    let duration = system_time.duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| StdDuration::from_secs(0));
    
    OvieTime {
        seconds: duration.as_secs(),
        nanoseconds: duration.subsec_nanos(),
    }
}

/// Create a time from Unix timestamp
pub fn from_unix_timestamp(seconds: u64) -> OvieTime {
    OvieTime {
        seconds,
        nanoseconds: 0,
    }
}

/// Create a time from Unix timestamp with nanoseconds
pub fn from_unix_timestamp_nanos(seconds: u64, nanoseconds: u32) -> OvieResult<OvieTime, String> {
    if nanoseconds >= NANOSECONDS_PER_SECOND {
        return err("Nanoseconds must be less than 1,000,000,000".to_string());
    }
    
    ok(OvieTime {
        seconds,
        nanoseconds,
    })
}

/// Create a time from date and time components
pub fn from_date_time(date: OvieDate, time: OvieTimeOfDay) -> OvieResult<OvieTime, String> {
    // Validate date components
    if !is_valid_date(&date) {
        return err("Invalid date".to_string());
    }
    
    // Validate time components
    if !is_valid_time(&time) {
        return err("Invalid time".to_string());
    }
    
    // Calculate Unix timestamp
    let days_since_epoch = match days_since_unix_epoch(&date) {
        OvieResult::Ok(days) => days,
        OvieResult::Err(e) => return err(e),
    };
    let seconds_from_date = days_since_epoch * SECONDS_PER_DAY;
    let seconds_from_time = (time.hour as u64) * SECONDS_PER_HOUR +
                           (time.minute as u64) * SECONDS_PER_MINUTE +
                           (time.second as u64);
    
    let total_seconds = seconds_from_date + seconds_from_time;
    
    ok(OvieTime {
        seconds: total_seconds,
        nanoseconds: time.nanosecond,
    })
}

// ===== DURATION CREATION =====

/// Create a duration from seconds
pub fn duration_from_seconds(seconds: u64) -> OvieDuration {
    OvieDuration {
        seconds,
        nanoseconds: 0,
    }
}

/// Create a duration from milliseconds
pub fn duration_from_millis(millis: u64) -> OvieDuration {
    let seconds = millis / 1000;
    let remaining_millis = millis % 1000;
    let nanoseconds = (remaining_millis as u32) * NANOSECONDS_PER_MILLISECOND;
    
    OvieDuration {
        seconds,
        nanoseconds,
    }
}

/// Create a duration from microseconds
pub fn duration_from_micros(micros: u64) -> OvieDuration {
    let seconds = micros / 1_000_000;
    let remaining_micros = micros % 1_000_000;
    let nanoseconds = (remaining_micros as u32) * NANOSECONDS_PER_MICROSECOND;
    
    OvieDuration {
        seconds,
        nanoseconds,
    }
}

/// Create a duration from nanoseconds
pub fn duration_from_nanos(nanos: u64) -> OvieDuration {
    let seconds = nanos / (NANOSECONDS_PER_SECOND as u64);
    let remaining_nanos = (nanos % (NANOSECONDS_PER_SECOND as u64)) as u32;
    
    OvieDuration {
        seconds,
        nanoseconds: remaining_nanos,
    }
}

/// Create a duration from hours, minutes, and seconds
pub fn duration_from_hms(hours: u64, minutes: u64, seconds: u64) -> OvieResult<OvieDuration, String> {
    if minutes >= 60 {
        return err("Minutes must be less than 60".to_string());
    }
    if seconds >= 60 {
        return err("Seconds must be less than 60".to_string());
    }
    
    let total_seconds = hours * SECONDS_PER_HOUR + minutes * SECONDS_PER_MINUTE + seconds;
    
    ok(OvieDuration {
        seconds: total_seconds,
        nanoseconds: 0,
    })
}

// ===== INSTANT OPERATIONS =====

/// Create an instant representing the current moment
pub fn instant_now() -> OvieInstant {
    OvieInstant {
        inner: Instant::now(),
    }
}

/// Get the elapsed time since an instant
pub fn elapsed_since(instant: &OvieInstant) -> OvieDuration {
    let elapsed = instant.inner.elapsed();
    OvieDuration {
        seconds: elapsed.as_secs(),
        nanoseconds: elapsed.subsec_nanos(),
    }
}

/// Get the duration between two instants
pub fn duration_between(earlier: &OvieInstant, later: &OvieInstant) -> OvieResult<OvieDuration, String> {
    if later.inner < earlier.inner {
        return err("Later instant must be after earlier instant".to_string());
    }
    
    let duration = later.inner.duration_since(earlier.inner);
    ok(OvieDuration {
        seconds: duration.as_secs(),
        nanoseconds: duration.subsec_nanos(),
    })
}

// ===== TIME ARITHMETIC =====

impl OvieTime {
    /// Add a duration to this time
    pub fn add_duration(&self, duration: OvieDuration) -> OvieResult<OvieTime, String> {
        let total_nanos = self.nanoseconds as u64 + duration.nanoseconds as u64;
        let extra_seconds = total_nanos / (NANOSECONDS_PER_SECOND as u64);
        let final_nanos = (total_nanos % (NANOSECONDS_PER_SECOND as u64)) as u32;
        
        let total_seconds = match self.seconds.checked_add(duration.seconds)
            .and_then(|s| s.checked_add(extra_seconds)) {
            Some(s) => s,
            None => return err("Time overflow".to_string()),
        };
        
        ok(OvieTime {
            seconds: total_seconds,
            nanoseconds: final_nanos,
        })
    }
    
    /// Subtract a duration from this time
    pub fn sub_duration(&self, duration: OvieDuration) -> OvieResult<OvieTime, String> {
        let total_duration_nanos = duration.seconds as u128 * (NANOSECONDS_PER_SECOND as u128) + duration.nanoseconds as u128;
        let self_total_nanos = self.seconds as u128 * (NANOSECONDS_PER_SECOND as u128) + self.nanoseconds as u128;
        
        if total_duration_nanos > self_total_nanos {
            return err("Cannot subtract duration larger than time".to_string());
        }
        
        let result_nanos = self_total_nanos - total_duration_nanos;
        let result_seconds = result_nanos / (NANOSECONDS_PER_SECOND as u128);
        let result_nano_part = (result_nanos % (NANOSECONDS_PER_SECOND as u128)) as u32;
        
        if result_seconds > u64::MAX as u128 {
            return err("Time underflow".to_string());
        }
        
        ok(OvieTime {
            seconds: result_seconds as u64,
            nanoseconds: result_nano_part,
        })
    }
    
    /// Get the duration between this time and another time
    pub fn duration_since(&self, other: &OvieTime) -> OvieResult<OvieDuration, String> {
        if self < other {
            return err("Cannot get duration to a later time".to_string());
        }
        
        let self_total_nanos = self.seconds as u128 * (NANOSECONDS_PER_SECOND as u128) + self.nanoseconds as u128;
        let other_total_nanos = other.seconds as u128 * (NANOSECONDS_PER_SECOND as u128) + other.nanoseconds as u128;
        
        let diff_nanos = self_total_nanos - other_total_nanos;
        let diff_seconds = diff_nanos / (NANOSECONDS_PER_SECOND as u128);
        let diff_nano_part = (diff_nanos % (NANOSECONDS_PER_SECOND as u128)) as u32;
        
        if diff_seconds > u64::MAX as u128 {
            return err("Duration overflow".to_string());
        }
        
        ok(OvieDuration {
            seconds: diff_seconds as u64,
            nanoseconds: diff_nano_part,
        })
    }
    
    /// Convert to Unix timestamp
    pub fn to_unix_timestamp(&self) -> u64 {
        self.seconds
    }
    
    /// Convert to Unix timestamp with nanoseconds
    pub fn to_unix_timestamp_nanos(&self) -> (u64, u32) {
        (self.seconds, self.nanoseconds)
    }
    
    /// Convert to date and time components
    pub fn to_date_time(&self) -> OvieDateTime {
        let date = unix_timestamp_to_date(self.seconds);
        let seconds_in_day = self.seconds % SECONDS_PER_DAY;
        
        let hour = (seconds_in_day / SECONDS_PER_HOUR) as u8;
        let minute = ((seconds_in_day % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE) as u8;
        let second = (seconds_in_day % SECONDS_PER_MINUTE) as u8;
        
        let time = OvieTimeOfDay {
            hour,
            minute,
            second,
            nanosecond: self.nanoseconds,
        };
        
        OvieDateTime { date, time }
    }
}

// ===== DURATION ARITHMETIC =====

impl OvieDuration {
    /// Add two durations
    pub fn add(&self, other: &OvieDuration) -> OvieResult<OvieDuration, String> {
        let total_nanos = self.nanoseconds as u64 + other.nanoseconds as u64;
        let extra_seconds = total_nanos / (NANOSECONDS_PER_SECOND as u64);
        let final_nanos = (total_nanos % (NANOSECONDS_PER_SECOND as u64)) as u32;
        
        let total_seconds = match self.seconds.checked_add(other.seconds)
            .and_then(|s| s.checked_add(extra_seconds)) {
            Some(s) => s,
            None => return err("Duration overflow".to_string()),
        };
        
        ok(OvieDuration {
            seconds: total_seconds,
            nanoseconds: final_nanos,
        })
    }
    
    /// Subtract a duration from this duration
    pub fn sub(&self, other: &OvieDuration) -> OvieResult<OvieDuration, String> {
        let self_total_nanos = self.seconds as u128 * (NANOSECONDS_PER_SECOND as u128) + self.nanoseconds as u128;
        let other_total_nanos = other.seconds as u128 * (NANOSECONDS_PER_SECOND as u128) + other.nanoseconds as u128;
        
        if other_total_nanos > self_total_nanos {
            return err("Cannot subtract larger duration from smaller duration".to_string());
        }
        
        let result_nanos = self_total_nanos - other_total_nanos;
        let result_seconds = result_nanos / (NANOSECONDS_PER_SECOND as u128);
        let result_nano_part = (result_nanos % (NANOSECONDS_PER_SECOND as u128)) as u32;
        
        ok(OvieDuration {
            seconds: result_seconds as u64,
            nanoseconds: result_nano_part,
        })
    }
    
    /// Multiply duration by a scalar
    pub fn mul(&self, scalar: u64) -> OvieResult<OvieDuration, String> {
        let total_nanos = self.seconds as u128 * (NANOSECONDS_PER_SECOND as u128) + self.nanoseconds as u128;
        let result_nanos = match total_nanos.checked_mul(scalar as u128) {
            Some(n) => n,
            None => return err("Duration overflow".to_string()),
        };
        
        let result_seconds = result_nanos / (NANOSECONDS_PER_SECOND as u128);
        let result_nano_part = (result_nanos % (NANOSECONDS_PER_SECOND as u128)) as u32;
        
        if result_seconds > u64::MAX as u128 {
            return err("Duration overflow".to_string());
        }
        
        ok(OvieDuration {
            seconds: result_seconds as u64,
            nanoseconds: result_nano_part,
        })
    }
    
    /// Divide duration by a scalar
    pub fn div(&self, scalar: u64) -> OvieResult<OvieDuration, String> {
        if scalar == 0 {
            return err("Cannot divide by zero".to_string());
        }
        
        let total_nanos = self.seconds as u128 * (NANOSECONDS_PER_SECOND as u128) + self.nanoseconds as u128;
        let result_nanos = total_nanos / (scalar as u128);
        
        let result_seconds = result_nanos / (NANOSECONDS_PER_SECOND as u128);
        let result_nano_part = (result_nanos % (NANOSECONDS_PER_SECOND as u128)) as u32;
        
        ok(OvieDuration {
            seconds: result_seconds as u64,
            nanoseconds: result_nano_part,
        })
    }
    
    /// Convert to total seconds (as floating point)
    pub fn as_seconds_f64(&self) -> f64 {
        self.seconds as f64 + (self.nanoseconds as f64) / (NANOSECONDS_PER_SECOND as f64)
    }
    
    /// Convert to total milliseconds
    pub fn as_millis(&self) -> u64 {
        self.seconds * 1000 + (self.nanoseconds / NANOSECONDS_PER_MILLISECOND) as u64
    }
    
    /// Convert to total microseconds
    pub fn as_micros(&self) -> u64 {
        self.seconds * 1_000_000 + (self.nanoseconds / NANOSECONDS_PER_MICROSECOND) as u64
    }
    
    /// Convert to total nanoseconds
    pub fn as_nanos(&self) -> u128 {
        self.seconds as u128 * (NANOSECONDS_PER_SECOND as u128) + self.nanoseconds as u128
    }
    
    /// Check if duration is zero
    pub fn is_zero(&self) -> bool {
        self.seconds == 0 && self.nanoseconds == 0
    }
}

// ===== SLEEP OPERATIONS =====

/// Sleep for a specified duration
pub fn sleep(duration: OvieDuration) -> OvieResult<(), String> {
    let std_duration = StdDuration::new(duration.seconds, duration.nanoseconds);
    thread::sleep(std_duration);
    ok(())
}

/// Sleep for a specified number of seconds
pub fn sleep_seconds(seconds: u64) -> OvieResult<(), String> {
    sleep(duration_from_seconds(seconds))
}

/// Sleep for a specified number of milliseconds
pub fn sleep_millis(millis: u64) -> OvieResult<(), String> {
    sleep(duration_from_millis(millis))
}

// ===== DATE/TIME UTILITIES =====

/// Check if a date is valid
pub fn is_valid_date(date: &OvieDate) -> bool {
    if date.month < 1 || date.month > 12 {
        return false;
    }
    
    if date.day < 1 {
        return false;
    }
    
    let days_in_month = match date.month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(date.year) {
                29
            } else {
                28
            }
        }
        _ => return false,
    };
    
    date.day <= days_in_month
}

/// Check if a time is valid
pub fn is_valid_time(time: &OvieTimeOfDay) -> bool {
    time.hour < 24 && 
    time.minute < 60 && 
    time.second < 60 && 
    time.nanosecond < NANOSECONDS_PER_SECOND
}

/// Check if a year is a leap year
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Get the number of days in a month
pub fn days_in_month(year: i32, month: u8) -> OvieResult<u8, String> {
    if month < 1 || month > 12 {
        return err("Month must be between 1 and 12".to_string());
    }
    
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => unreachable!(),
    };
    
    ok(days)
}

// ===== INTERNAL UTILITIES =====

/// Calculate days since Unix epoch (1970-01-01)
fn days_since_unix_epoch(date: &OvieDate) -> OvieResult<u64, String> {
    if date.year < 1970 {
        return err("Year must be 1970 or later".to_string());
    }
    
    let mut days = 0u64;
    
    // Add days for complete years
    for year in 1970..date.year {
        days += if is_leap_year(year) { 366 } else { 365 };
    }
    
    // Add days for complete months in the current year
    for month in 1..date.month {
        days += match days_in_month(date.year, month) {
            OvieResult::Ok(d) => d as u64,
            OvieResult::Err(e) => return err(e),
        };
    }
    
    // Add days in the current month (minus 1 because we count from day 1)
    days += (date.day - 1) as u64;
    
    ok(days)
}

/// Convert Unix timestamp to date
fn unix_timestamp_to_date(timestamp: u64) -> OvieDate {
    let mut days = timestamp / SECONDS_PER_DAY;
    let mut year = 1970;
    
    // Find the year
    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if days < days_in_year {
            break;
        }
        days -= days_in_year;
        year += 1;
    }
    
    // Find the month
    let mut month = 1;
    loop {
        let days_in_current_month = match days_in_month(year, month) {
            OvieResult::Ok(d) => d as u64,
            OvieResult::Err(_) => break, // Should not happen
        };
        
        if days < days_in_current_month {
            break;
        }
        days -= days_in_current_month;
        month += 1;
        
        if month > 12 {
            break; // Should not happen
        }
    }
    
    let day = (days + 1) as u8; // Add 1 because days are 1-indexed
    
    OvieDate { year, month, day }
}

// ===== FORMATTING =====

impl OvieTime {
    /// Format time as ISO 8601 string (YYYY-MM-DDTHH:MM:SS.sssZ)
    pub fn to_iso8601(&self) -> String {
        let datetime = self.to_date_time();
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
            datetime.date.year,
            datetime.date.month,
            datetime.date.day,
            datetime.time.hour,
            datetime.time.minute,
            datetime.time.second,
            datetime.time.nanosecond / NANOSECONDS_PER_MILLISECOND
        )
    }
    
    /// Format time as Unix timestamp string
    pub fn to_timestamp_string(&self) -> String {
        format!("{}.{:09}", self.seconds, self.nanoseconds)
    }
}

impl OvieDuration {
    /// Format duration as human-readable string
    pub fn to_string(&self) -> String {
        if self.seconds == 0 {
            if self.nanoseconds == 0 {
                "0s".to_string()
            } else if self.nanoseconds % NANOSECONDS_PER_MILLISECOND == 0 {
                format!("{}ms", self.nanoseconds / NANOSECONDS_PER_MILLISECOND)
            } else if self.nanoseconds % NANOSECONDS_PER_MICROSECOND == 0 {
                format!("{}μs", self.nanoseconds / NANOSECONDS_PER_MICROSECOND)
            } else {
                format!("{}ns", self.nanoseconds)
            }
        } else if self.seconds < 60 {
            if self.nanoseconds == 0 {
                format!("{}s", self.seconds)
            } else {
                format!("{}.{:03}s", self.seconds, self.nanoseconds / NANOSECONDS_PER_MILLISECOND)
            }
        } else if self.seconds < 3600 {
            let minutes = self.seconds / 60;
            let seconds = self.seconds % 60;
            if seconds == 0 && self.nanoseconds == 0 {
                format!("{}m", minutes)
            } else {
                format!("{}m{}s", minutes, seconds)
            }
        } else {
            let hours = self.seconds / 3600;
            let minutes = (self.seconds % 3600) / 60;
            let seconds = self.seconds % 60;
            
            if minutes == 0 && seconds == 0 && self.nanoseconds == 0 {
                format!("{}h", hours)
            } else if seconds == 0 && self.nanoseconds == 0 {
                format!("{}h{}m", hours, minutes)
            } else {
                format!("{}h{}m{}s", hours, minutes, seconds)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_creation() {
        let time = from_unix_timestamp(1609459200); // 2021-01-01 00:00:00 UTC
        assert_eq!(time.seconds, 1609459200);
        assert_eq!(time.nanoseconds, 0);
        
        let time_with_nanos = from_unix_timestamp_nanos(1609459200, 123456789).unwrap();
        assert_eq!(time_with_nanos.seconds, 1609459200);
        assert_eq!(time_with_nanos.nanoseconds, 123456789);
    }
    
    #[test]
    fn test_duration_creation() {
        let dur = duration_from_seconds(60);
        assert_eq!(dur.seconds, 60);
        assert_eq!(dur.nanoseconds, 0);
        
        let dur_millis = duration_from_millis(1500);
        assert_eq!(dur_millis.seconds, 1);
        assert_eq!(dur_millis.nanoseconds, 500_000_000);
        
        let dur_hms = duration_from_hms(1, 30, 45).unwrap();
        assert_eq!(dur_hms.seconds, 5445); // 1*3600 + 30*60 + 45
    }
    
    #[test]
    fn test_time_arithmetic() {
        let time = from_unix_timestamp(1000);
        let duration = duration_from_seconds(500);
        
        let later = time.add_duration(duration).unwrap();
        assert_eq!(later.seconds, 1500);
        
        let earlier = later.sub_duration(duration).unwrap();
        assert_eq!(earlier.seconds, 1000);
        
        let diff = later.duration_since(&earlier).unwrap();
        assert_eq!(diff.seconds, 500);
    }
    
    #[test]
    fn test_duration_arithmetic() {
        let dur1 = duration_from_seconds(100);
        let dur2 = duration_from_seconds(50);
        
        let sum = dur1.add(&dur2).unwrap();
        assert_eq!(sum.seconds, 150);
        
        let diff = sum.sub(&dur2).unwrap();
        assert_eq!(diff.seconds, 100);
        
        let doubled = dur1.mul(2).unwrap();
        assert_eq!(doubled.seconds, 200);
        
        let halved = doubled.div(2).unwrap();
        assert_eq!(halved.seconds, 100);
    }
    
    #[test]
    fn test_date_validation() {
        let valid_date = OvieDate { year: 2021, month: 2, day: 28 };
        assert!(is_valid_date(&valid_date));
        
        let invalid_date = OvieDate { year: 2021, month: 2, day: 29 };
        assert!(!is_valid_date(&invalid_date));
        
        let leap_year_date = OvieDate { year: 2020, month: 2, day: 29 };
        assert!(is_valid_date(&leap_year_date));
    }
    
    #[test]
    fn test_leap_year() {
        assert!(is_leap_year(2000)); // Divisible by 400
        assert!(is_leap_year(2004)); // Divisible by 4, not by 100
        assert!(!is_leap_year(1900)); // Divisible by 100, not by 400
        assert!(!is_leap_year(2001)); // Not divisible by 4
    }
    
    #[test]
    fn test_date_time_conversion() {
        let date = OvieDate { year: 2021, month: 1, day: 1 };
        let time = OvieTimeOfDay { hour: 12, minute: 30, second: 45, nanosecond: 123456789 };
        
        let ovie_time = from_date_time(date.clone(), time.clone()).unwrap();
        let converted_datetime = ovie_time.to_date_time();
        
        assert_eq!(converted_datetime.date, date);
        assert_eq!(converted_datetime.time, time);
    }
    
    #[test]
    fn test_formatting() {
        let time = from_unix_timestamp(1609459200); // 2021-01-01 00:00:00 UTC
        let iso_string = time.to_iso8601();
        assert!(iso_string.starts_with("2021-01-01T00:00:00"));
        
        let duration = duration_from_hms(1, 30, 45).unwrap();
        let duration_string = duration.to_string();
        assert_eq!(duration_string, "1h30m45s");
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
        }
    }
}