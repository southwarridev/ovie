//! Ovie Standard Library - Math Module
//! 
//! Deterministic mathematical operations with overflow handling.
//! All operations are designed to be reproducible across platforms.

use crate::stdlib::core::{OvieResult, ok, err};
use std::f64;

// ===== MATHEMATICAL CONSTANTS =====

/// Mathematical constants with deterministic precision
pub const PI: f64 = 3.141592653589793;
pub const E: f64 = 2.718281828459045;
pub const TAU: f64 = 6.283185307179586; // 2 * PI
pub const SQRT_2: f64 = 1.4142135623730951;
pub const SQRT_3: f64 = 1.7320508075688772;
pub const LN_2: f64 = 0.6931471805599453;
pub const LN_10: f64 = 2.302585092994046;
pub const LOG2_E: f64 = 1.4426950408889634;
pub const LOG10_E: f64 = 0.4342944819032518;

/// Floating-point limits
pub const INFINITY: f64 = f64::INFINITY;
pub const NEG_INFINITY: f64 = f64::NEG_INFINITY;
pub const NAN: f64 = f64::NAN;
pub const EPSILON: f64 = f64::EPSILON;

/// Integer limits (64-bit)
pub const MAX_INT: i64 = i64::MAX;
pub const MIN_INT: i64 = i64::MIN;

// ===== BASIC ARITHMETIC WITH OVERFLOW CHECKING =====

/// Checked addition with overflow detection
pub fn checked_add(a: f64, b: f64) -> OvieResult<f64, String> {
    // Check for integer overflow if both are integers
    if is_integer(a) && is_integer(b) {
        let a_int = a as i64;
        let b_int = b as i64;
        
        if a_int > 0 && b_int > MAX_INT - a_int {
            return err("Integer overflow in addition".to_string());
        }
        if a_int < 0 && b_int < MIN_INT - a_int {
            return err("Integer underflow in addition".to_string());
        }
    }
    
    let result = a + b;
    
    // Check for floating-point overflow
    if result.is_infinite() && !a.is_infinite() && !b.is_infinite() {
        return err("Floating-point overflow in addition".to_string());
    }
    
    ok(result)
}

/// Checked subtraction with overflow detection
pub fn checked_sub(a: f64, b: f64) -> OvieResult<f64, String> {
    // Check for integer overflow if both are integers
    if is_integer(a) && is_integer(b) {
        let a_int = a as i64;
        let b_int = b as i64;
        
        if b_int > 0 && a_int < MIN_INT + b_int {
            return err("Integer underflow in subtraction".to_string());
        }
        if b_int < 0 && a_int > MAX_INT + b_int {
            return err("Integer overflow in subtraction".to_string());
        }
    }
    
    let result = a - b;
    
    // Check for floating-point overflow
    if result.is_infinite() && !a.is_infinite() && !b.is_infinite() {
        return err("Floating-point overflow in subtraction".to_string());
    }
    
    ok(result)
}

/// Checked multiplication with overflow detection
pub fn checked_mul(a: f64, b: f64) -> OvieResult<f64, String> {
    // Check for integer overflow if both are integers
    if is_integer(a) && is_integer(b) {
        let a_int = a as i64;
        let b_int = b as i64;
        
        if a_int != 0 && b_int != 0 {
            if a_int > 0 && b_int > 0 && a_int > MAX_INT / b_int {
                return err("Integer overflow in multiplication".to_string());
            }
            if a_int > 0 && b_int < 0 && b_int < MIN_INT / a_int {
                return err("Integer underflow in multiplication".to_string());
            }
            if a_int < 0 && b_int > 0 && a_int < MIN_INT / b_int {
                return err("Integer underflow in multiplication".to_string());
            }
            if a_int < 0 && b_int < 0 && a_int < MAX_INT / b_int {
                return err("Integer overflow in multiplication".to_string());
            }
        }
    }
    
    let result = a * b;
    
    // Check for floating-point overflow
    if result.is_infinite() && !a.is_infinite() && !b.is_infinite() {
        return err("Floating-point overflow in multiplication".to_string());
    }
    
    ok(result)
}

/// Checked division with zero-division detection
pub fn checked_div(a: f64, b: f64) -> OvieResult<f64, String> {
    if b == 0.0 {
        return err("Division by zero".to_string());
    }
    
    // Check for integer division overflow (MIN_INT / -1)
    if is_integer(a) && is_integer(b) {
        let a_int = a as i64;
        let b_int = b as i64;
        
        if a_int == MIN_INT && b_int == -1 {
            return err("Integer overflow in division".to_string());
        }
    }
    
    let result = a / b;
    ok(result)
}

/// Checked modulo with zero-division detection
pub fn checked_mod(a: f64, b: f64) -> OvieResult<f64, String> {
    if b == 0.0 {
        return err("Modulo by zero".to_string());
    }
    
    // Check for integer modulo overflow (MIN_INT % -1)
    if is_integer(a) && is_integer(b) {
        let a_int = a as i64;
        let b_int = b as i64;
        
        if a_int == MIN_INT && b_int == -1 {
            return err("Integer overflow in modulo".to_string());
        }
    }
    
    let result = a % b;
    ok(result)
}

// ===== POWER AND ROOT FUNCTIONS =====

/// Power function with overflow checking
pub fn pow(base: f64, exponent: f64) -> OvieResult<f64, String> {
    // Handle special cases
    if base == 0.0 && exponent < 0.0 {
        return err("Division by zero in power function".to_string());
    }
    
    if base == 0.0 && exponent == 0.0 {
        return ok(1.0); // 0^0 = 1 by convention
    }
    
    if exponent == 0.0 {
        return ok(1.0);
    }
    
    if exponent == 1.0 {
        return ok(base);
    }
    
    // For integer exponents, use repeated multiplication
    if is_integer(exponent) && exponent >= 0.0 {
        return integer_pow(base, exponent as i64);
    }
    
    // For floating-point exponents, use logarithmic method
    if base <= 0.0 {
        return err("Cannot raise negative number to fractional power".to_string());
    }
    
    let ln_base = match ovie_ln(base) {
        crate::stdlib::core::OvieResult::Ok(val) => val,
        crate::stdlib::core::OvieResult::Err(e) => return err(e),
    };
    let result = ovie_exp(exponent * ln_base);
    
    if result.is_infinite() {
        return err("Overflow in power function".to_string());
    }
    
    ok(result)
}

/// Integer power using repeated multiplication
pub fn integer_pow(base: f64, exponent: i64) -> OvieResult<f64, String> {
    if exponent < 0 {
        return err("Negative exponent not supported in integer_pow".to_string());
    }
    
    let mut result = 1.0;
    let mut current_base = base;
    let mut current_exp = exponent;
    
    while current_exp > 0 {
        if current_exp % 2 == 1 {
            let mul_result = match checked_mul(result, current_base) {
                crate::stdlib::core::OvieResult::Ok(val) => val,
                crate::stdlib::core::OvieResult::Err(e) => return err(e),
            };
            result = mul_result;
        }
        
        let square_result = match checked_mul(current_base, current_base) {
            crate::stdlib::core::OvieResult::Ok(val) => val,
            crate::stdlib::core::OvieResult::Err(e) => return err(e),
        };
        current_base = square_result;
        current_exp /= 2;
    }
    
    ok(result)
}

/// Square root with error checking
pub fn ovie_sqrt(x: f64) -> OvieResult<f64, String> {
    if x < 0.0 {
        return err("Cannot take square root of negative number".to_string());
    }
    
    if x == 0.0 || x == 1.0 {
        return ok(x);
    }
    
    // Newton's method for square root
    let mut guess = x / 2.0;
    let epsilon = 1e-15;
    
    for _ in 0..100 { // Maximum iterations to ensure termination
        let new_guess = (guess + x / guess) / 2.0;
        
        if ovie_abs(new_guess - guess) < epsilon {
            return ok(new_guess);
        }
        
        guess = new_guess;
    }
    
    ok(guess)
}

/// Cube root
pub fn cbrt(x: f64) -> f64 {
    if x == 0.0 {
        return 0.0;
    }
    
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let abs_x = ovie_abs(x);
    
    // Newton's method for cube root
    let mut guess = abs_x / 3.0;
    let epsilon = 1e-15;
    
    for _ in 0..100 {
        let new_guess = (2.0 * guess + abs_x / (guess * guess)) / 3.0;
        
        if ovie_abs(new_guess - guess) < epsilon {
            return sign * new_guess;
        }
        
        guess = new_guess;
    }
    
    sign * guess
}

// ===== UTILITY FUNCTIONS =====

/// Absolute value
pub fn ovie_abs(x: f64) -> f64 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

/// Sign function
pub fn sign(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        0.0
    }
}

/// Floor function
pub fn ovie_floor(x: f64) -> f64 {
    if is_integer(x) {
        return x;
    }
    
    if x >= 0.0 {
        truncate(x)
    } else {
        truncate(x) - 1.0
    }
}

/// Ceiling function
pub fn ovie_ceil(x: f64) -> f64 {
    if is_integer(x) {
        return x;
    }
    
    if x >= 0.0 {
        truncate(x) + 1.0
    } else {
        truncate(x)
    }
}

/// Round to nearest integer
pub fn ovie_round(x: f64) -> f64 {
    if x >= 0.0 {
        ovie_floor(x + 0.5)
    } else {
        ovie_ceil(x - 0.5)
    }
}

/// Truncate to integer
pub fn truncate(x: f64) -> f64 {
    x.trunc()
}

/// Fractional part
pub fn fract(x: f64) -> f64 {
    x - truncate(x)
}

// ===== COMPARISON AND CLASSIFICATION =====

/// Check if a number is an integer
pub fn is_integer(x: f64) -> bool {
    x == truncate(x)
}

/// Check if a number is finite
pub fn is_finite(x: f64) -> bool {
    x.is_finite()
}

/// Check if a number is infinite
pub fn is_infinite(x: f64) -> bool {
    x.is_infinite()
}

/// Check if a number is NaN
pub fn is_nan(x: f64) -> bool {
    x.is_nan()
}

/// Check if a number is normal (not zero, infinite, or NaN)
pub fn is_normal(x: f64) -> bool {
    x.is_normal()
}

/// Compare floating-point numbers with epsilon
pub fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
    ovie_abs(a - b) < epsilon
}

/// Minimum of two values
pub fn ovie_min(a: f64, b: f64) -> f64 {
    if a <= b { a } else { b }
}

/// Maximum of two values
pub fn ovie_max(a: f64, b: f64) -> f64 {
    if a >= b { a } else { b }
}

/// Clamp value between min and max
pub fn ovie_clamp(value: f64, min_val: f64, max_val: f64) -> f64 {
    if value < min_val {
        min_val
    } else if value > max_val {
        max_val
    } else {
        value
    }
}

// ===== EXPONENTIAL AND LOGARITHMIC FUNCTIONS =====

/// Natural exponential function
pub fn ovie_exp(x: f64) -> f64 {
    if x == 0.0 {
        return 1.0;
    }
    
    if x > 700.0 { // Prevent overflow
        return INFINITY;
    }
    
    if x < -700.0 { // Prevent underflow
        return 0.0;
    }
    
    // Taylor series: e^x = 1 + x + x²/2! + x³/3! + ...
    let mut result = 1.0;
    let mut term = 1.0;
    
    for n in 1..50 {
        term = term * x / (n as f64);
        result = result + term;
        
        if ovie_abs(term) < EPSILON {
            break;
        }
    }
    
    result
}

/// Natural logarithm
pub fn ovie_ln(x: f64) -> OvieResult<f64, String> {
    if x <= 0.0 {
        return err("Natural logarithm domain error: input must be positive".to_string());
    }
    
    if x == 1.0 {
        return ok(0.0);
    }
    
    if x == E {
        return ok(1.0);
    }
    
    // Use Newton's method: ln(x) = y where e^y = x
    let mut guess = if x > 1.0 { x / E } else { x - 1.0 };
    let epsilon = 1e-15;
    
    for _ in 0..100 {
        let exp_guess = ovie_exp(guess);
        let new_guess = guess - (exp_guess - x) / exp_guess;
        
        if ovie_abs(new_guess - guess) < epsilon {
            return ok(new_guess);
        }
        
        guess = new_guess;
    }
    
    ok(guess)
}

/// Base-10 logarithm
pub fn log10(x: f64) -> OvieResult<f64, String> {
    let ln_result = match ovie_ln(x) {
        crate::stdlib::core::OvieResult::Ok(val) => val,
        crate::stdlib::core::OvieResult::Err(e) => return err(e),
    };
    ok(ln_result / LN_10)
}

/// Base-2 logarithm
pub fn log2(x: f64) -> OvieResult<f64, String> {
    let ln_result = match ovie_ln(x) {
        crate::stdlib::core::OvieResult::Ok(val) => val,
        crate::stdlib::core::OvieResult::Err(e) => return err(e),
    };
    ok(ln_result / LN_2)
}

/// Logarithm with arbitrary base
pub fn log(x: f64, base: f64) -> OvieResult<f64, String> {
    if base <= 0.0 || base == 1.0 {
        return err("Logarithm base error: base must be positive and not equal to 1".to_string());
    }
    
    let ln_x = match ovie_ln(x) {
        crate::stdlib::core::OvieResult::Ok(val) => val,
        crate::stdlib::core::OvieResult::Err(e) => return err(e),
    };
    let ln_base = match ovie_ln(base) {
        crate::stdlib::core::OvieResult::Ok(val) => val,
        crate::stdlib::core::OvieResult::Err(e) => return err(e),
    };
    
    ok(ln_x / ln_base)
}

// ===== TRIGONOMETRIC FUNCTIONS =====

/// Sine function using Taylor series
pub fn ovie_sin(x: f64) -> f64 {
    // Normalize angle to [-2π, 2π] range
    let normalized_x = x % TAU;
    
    // Taylor series: sin(x) = x - x³/3! + x⁵/5! - x⁷/7! + ...
    let mut result = 0.0;
    let mut term = normalized_x;
    let x_squared = normalized_x * normalized_x;
    
    for n in 0..20 { // Sufficient iterations for good precision
        if n % 2 == 0 {
            result += term;
        } else {
            result -= term;
        }
        
        term *= x_squared / ((2 * n + 2) as f64 * (2 * n + 3) as f64);
        
        if ovie_abs(term) < EPSILON {
            break;
        }
    }
    
    result
}

/// Cosine function using Taylor series
pub fn ovie_cos(x: f64) -> f64 {
    // Normalize angle to [-2π, 2π] range
    let normalized_x = x % TAU;
    
    // Taylor series: cos(x) = 1 - x²/2! + x⁴/4! - x⁶/6! + ...
    let mut result = 1.0;
    let mut term = 1.0;
    let x_squared = normalized_x * normalized_x;
    
    for n in 1..20 { // Sufficient iterations for good precision
        term *= x_squared / ((2 * n - 1) as f64 * (2 * n) as f64);
        
        if n % 2 == 1 {
            result -= term;
        } else {
            result += term;
        }
        
        if ovie_abs(term) < EPSILON {
            break;
        }
    }
    
    result
}

/// Tangent function
pub fn ovie_tan(x: f64) -> f64 {
    let cos_x = ovie_cos(x);
    
    // Check for division by zero (cos(x) = 0 at π/2 + nπ)
    if ovie_abs(cos_x) < EPSILON {
        if ovie_sin(x) > 0.0 {
            return INFINITY;
        } else {
            return NEG_INFINITY;
        }
    }
    
    ovie_sin(x) / cos_x
}

/// Arcsine function with domain checking
pub fn ovie_asin(x: f64) -> OvieResult<f64, String> {
    if x < -1.0 || x > 1.0 {
        return err("Arcsine domain error: input must be in range [-1, 1]".to_string());
    }
    
    if x == -1.0 {
        return ok(-PI / 2.0);
    }
    
    if x == 1.0 {
        return ok(PI / 2.0);
    }
    
    if x == 0.0 {
        return ok(0.0);
    }
    
    // Use Newton's method: find y such that sin(y) = x
    let mut guess = x; // Initial guess
    let epsilon = 1e-15;
    
    for _ in 0..100 {
        let sin_guess = ovie_sin(guess);
        let cos_guess = ovie_cos(guess);
        
        if ovie_abs(cos_guess) < EPSILON {
            break; // Avoid division by zero
        }
        
        let new_guess = guess - (sin_guess - x) / cos_guess;
        
        if ovie_abs(new_guess - guess) < epsilon {
            return ok(new_guess);
        }
        
        guess = new_guess;
    }
    
    ok(guess)
}

/// Arccosine function with domain checking
pub fn ovie_acos(x: f64) -> OvieResult<f64, String> {
    if x < -1.0 || x > 1.0 {
        return err("Arccosine domain error: input must be in range [-1, 1]".to_string());
    }
    
    if x == -1.0 {
        return ok(PI);
    }
    
    if x == 1.0 {
        return ok(0.0);
    }
    
    if x == 0.0 {
        return ok(PI / 2.0);
    }
    
    // Use identity: acos(x) = π/2 - asin(x)
    let asin_result = match ovie_asin(x) {
        crate::stdlib::core::OvieResult::Ok(val) => val,
        crate::stdlib::core::OvieResult::Err(e) => return err(e),
    };
    
    ok(PI / 2.0 - asin_result)
}

/// Arctangent function
pub fn ovie_atan(x: f64) -> f64 {
    if x == 0.0 {
        return 0.0;
    }
    
    if x.is_infinite() {
        return if x > 0.0 { PI / 2.0 } else { -PI / 2.0 };
    }
    
    // Use Newton's method: find y such that tan(y) = x
    let mut guess = if ovie_abs(x) > 1.0 { 
        if x > 0.0 { PI / 4.0 } else { -PI / 4.0 }
    } else { 
        x 
    };
    
    let epsilon = 1e-15;
    
    for _ in 0..100 {
        let tan_guess = ovie_tan(guess);
        let cos_guess = ovie_cos(guess);
        let sec_squared = 1.0 / (cos_guess * cos_guess);
        
        if ovie_abs(sec_squared) < EPSILON {
            break; // Avoid division by zero
        }
        
        let new_guess = guess - (tan_guess - x) / sec_squared;
        
        if ovie_abs(new_guess - guess) < epsilon {
            return new_guess;
        }
        
        guess = new_guess;
    }
    
    guess
}

/// Two-argument arctangent function
pub fn ovie_atan2(y: f64, x: f64) -> f64 {
    if x == 0.0 && y == 0.0 {
        return 0.0; // Undefined, but return 0 by convention
    }
    
    if x > 0.0 {
        return ovie_atan(y / x);
    }
    
    if x < 0.0 {
        if y >= 0.0 {
            return ovie_atan(y / x) + PI;
        } else {
            return ovie_atan(y / x) - PI;
        }
    }
    
    // x == 0.0
    if y > 0.0 {
        PI / 2.0
    } else {
        -PI / 2.0
    }
}

// ===== SPECIAL FUNCTIONS =====

/// Factorial function
pub fn factorial(n: f64) -> OvieResult<f64, String> {
    if !is_integer(n) || n < 0.0 {
        return err("Factorial domain error: input must be a non-negative integer".to_string());
    }
    
    if n > 170.0 { // Prevent overflow
        return err("Factorial overflow: input too large".to_string());
    }
    
    let n_int = n as i64;
    
    if n_int == 0 || n_int == 1 {
        return ok(1.0);
    }
    
    let mut result = 1.0;
    for i in 2..=n_int {
        result *= i as f64;
    }
    
    ok(result)
}

/// Greatest common divisor
pub fn gcd(a: f64, b: f64) -> OvieResult<f64, String> {
    if !is_integer(a) || !is_integer(b) {
        return err("GCD domain error: inputs must be integers".to_string());
    }
    
    let mut x = ovie_abs(a) as i64;
    let mut y = ovie_abs(b) as i64;
    
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    
    ok(x as f64)
}

/// Least common multiple
pub fn lcm(a: f64, b: f64) -> OvieResult<f64, String> {
    if a == 0.0 || b == 0.0 {
        return ok(0.0);
    }
    
    let gcd_result = match gcd(a, b) {
        crate::stdlib::core::OvieResult::Ok(val) => val,
        crate::stdlib::core::OvieResult::Err(e) => return err(e),
    };
    ok(ovie_abs(a * b) / gcd_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mathematical_constants() {
        // Test that constants are defined with expected precision
        assert!((PI - 3.141592653589793).abs() < 1e-15);
        assert!((E - 2.718281828459045).abs() < 1e-15);
        assert!((TAU - 2.0 * PI).abs() < 1e-15);
        assert!((SQRT_2 * SQRT_2 - 2.0).abs() < 1e-15);
        assert!((SQRT_3 * SQRT_3 - 3.0).abs() < 1e-15);
    }

    #[test]
    fn test_checked_arithmetic() {
        // Test normal operations
        assert_eq!(checked_add(2.0, 3.0).unwrap(), 5.0);
        assert_eq!(checked_sub(5.0, 3.0).unwrap(), 2.0);
        assert_eq!(checked_mul(2.0, 3.0).unwrap(), 6.0);
        assert_eq!(checked_div(6.0, 2.0).unwrap(), 3.0);
        
        // Test error conditions
        assert!(checked_div(1.0, 0.0).is_err());
        assert!(checked_mod(1.0, 0.0).is_err());
    }

    #[test]
    fn test_power_functions() {
        // Test basic power operations
        assert_eq!(pow(2.0, 3.0).unwrap(), 8.0);
        assert_eq!(pow(0.0, 0.0).unwrap(), 1.0);
        assert_eq!(pow(5.0, 0.0).unwrap(), 1.0);
        assert_eq!(pow(5.0, 1.0).unwrap(), 5.0);
        
        // Test integer power
        assert_eq!(integer_pow(2.0, 10).unwrap(), 1024.0);
        
        // Test square root
        assert_eq!(ovie_sqrt(4.0).unwrap(), 2.0);
        assert_eq!(ovie_sqrt(9.0).unwrap(), 3.0);
        assert!(ovie_sqrt(-1.0).is_err());
        
        // Test cube root
        assert!((cbrt(8.0) - 2.0).abs() < 1e-10);
        assert!((cbrt(-8.0) + 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_utility_functions() {
        // Test absolute value
        assert_eq!(ovie_abs(5.0), 5.0);
        assert_eq!(ovie_abs(-5.0), 5.0);
        assert_eq!(ovie_abs(0.0), 0.0);
        
        // Test sign function
        assert_eq!(sign(5.0), 1.0);
        assert_eq!(sign(-5.0), -1.0);
        assert_eq!(sign(0.0), 0.0);
        
        // Test floor, ceil, round
        assert_eq!(ovie_floor(3.7), 3.0);
        assert_eq!(ovie_floor(-3.7), -4.0);
        assert_eq!(ovie_ceil(3.2), 4.0);
        assert_eq!(ovie_ceil(-3.2), -3.0);
        assert_eq!(ovie_round(3.5), 4.0);
        assert_eq!(ovie_round(-3.5), -4.0);
        
        // Test min, max, clamp
        assert_eq!(ovie_min(3.0, 5.0), 3.0);
        assert_eq!(ovie_max(3.0, 5.0), 5.0);
        assert_eq!(ovie_clamp(7.0, 2.0, 5.0), 5.0);
        assert_eq!(ovie_clamp(1.0, 2.0, 5.0), 2.0);
        assert_eq!(ovie_clamp(3.0, 2.0, 5.0), 3.0);
    }

    #[test]
    fn test_classification_functions() {
        // Test integer check
        assert!(is_integer(5.0));
        assert!(!is_integer(5.5));
        
        // Test finite/infinite/nan checks
        assert!(is_finite(5.0));
        assert!(!is_finite(INFINITY));
        assert!(!is_finite(NAN));
        assert!(is_infinite(INFINITY));
        assert!(!is_infinite(5.0));
        assert!(is_nan(NAN));
        assert!(!is_nan(5.0));
        
        // Test approximate equality
        assert!(approx_eq(1.0, 1.0000001, 1e-6));
        assert!(!approx_eq(1.0, 1.1, 1e-6));
    }

    #[test]
    fn test_exponential_logarithmic() {
        // Test exponential
        assert!((ovie_exp(0.0) - 1.0).abs() < 1e-10);
        assert!((ovie_exp(1.0) - E).abs() < 1e-10);
        
        // Test natural logarithm
        assert!((ovie_ln(1.0).unwrap() - 0.0).abs() < 1e-10);
        assert!((ovie_ln(E).unwrap() - 1.0).abs() < 1e-10);
        assert!(ovie_ln(-1.0).is_err());
        assert!(ovie_ln(0.0).is_err());
        
        // Test other logarithms
        assert!((log10(10.0).unwrap() - 1.0).abs() < 1e-10);
        assert!((log2(8.0).unwrap() - 3.0).abs() < 1e-10);
        assert!((log(8.0, 2.0).unwrap() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_trigonometric_functions() {
        // Test basic trigonometric identities
        let angles = [0.0, PI / 6.0, PI / 4.0, PI / 3.0, PI / 2.0, PI];
        
        for &angle in &angles {
            let sin_val = ovie_sin(angle);
            let cos_val = ovie_cos(angle);
            
            // Test Pythagorean identity: sin²(x) + cos²(x) = 1
            let identity = sin_val * sin_val + cos_val * cos_val;
            assert!((identity - 1.0).abs() < 1e-10, "Pythagorean identity failed for angle {}", angle);
            
            // Test tan(x) = sin(x) / cos(x) when cos(x) ≠ 0
            if ovie_abs(cos_val) > 1e-10 {
                let tan_val = ovie_tan(angle);
                let expected_tan = sin_val / cos_val;
                assert!((tan_val - expected_tan).abs() < 1e-10, "Tangent identity failed for angle {}", angle);
            }
        }
        
        // Test specific known values
        assert!((ovie_sin(0.0) - 0.0).abs() < 1e-15);
        assert!((ovie_sin(PI / 2.0) - 1.0).abs() < 1e-10);
        assert!((ovie_cos(0.0) - 1.0).abs() < 1e-15);
        assert!((ovie_cos(PI / 2.0) - 0.0).abs() < 1e-10);
        
        // Test inverse functions
        let test_values = [-0.9, -0.5, 0.0, 0.5, 0.9];
        
        for &val in &test_values {
            // Test asin
            let asin_result = ovie_asin(val).unwrap();
            let sin_asin = ovie_sin(asin_result);
            assert!((sin_asin - val).abs() < 1e-10, "asin/sin identity failed for {}", val);
            
            // Test acos
            let acos_result = ovie_acos(val).unwrap();
            let cos_acos = ovie_cos(acos_result);
            assert!((cos_acos - val).abs() < 1e-10, "acos/cos identity failed for {}", val);
        }
        
        // Test atan
        let atan_test_values = [-10.0, -1.0, 0.0, 1.0, 10.0];
        for &val in &atan_test_values {
            let atan_result = ovie_atan(val);
            let tan_atan = ovie_tan(atan_result);
            assert!((tan_atan - val).abs() < 1e-10, "atan/tan identity failed for {}", val);
        }
        
        // Test atan2
        assert!((ovie_atan2(1.0, 1.0) - PI / 4.0).abs() < 1e-10);
        assert!((ovie_atan2(1.0, -1.0) - 3.0 * PI / 4.0).abs() < 1e-10);
        assert!((ovie_atan2(-1.0, -1.0) + 3.0 * PI / 4.0).abs() < 1e-10);
        assert!((ovie_atan2(-1.0, 1.0) + PI / 4.0).abs() < 1e-10);
        
        // Test domain errors
        assert!(ovie_asin(-1.1).is_err());
        assert!(ovie_asin(1.1).is_err());
        assert!(ovie_acos(-1.1).is_err());
        assert!(ovie_acos(1.1).is_err());
    }

    #[test]
    fn test_special_functions() {
        // Test factorial
        assert_eq!(factorial(0.0).unwrap(), 1.0);
        assert_eq!(factorial(1.0).unwrap(), 1.0);
        assert_eq!(factorial(5.0).unwrap(), 120.0);
        assert!(factorial(-1.0).is_err());
        assert!(factorial(5.5).is_err());
        
        // Test GCD and LCM
        assert_eq!(gcd(12.0, 8.0).unwrap(), 4.0);
        assert_eq!(lcm(12.0, 8.0).unwrap(), 24.0);
        assert!(gcd(5.5, 3.0).is_err());
    }

    #[test]
    fn test_deterministic_behavior() {
        // Test that same inputs always produce same outputs
        let test_values = [0.0, 1.0, -1.0, 3.14159, -2.71828, 100.0, -100.0];
        
        for &val in &test_values {
            // Test multiple calls produce identical results
            let result1 = ovie_abs(val);
            let result2 = ovie_abs(val);
            assert_eq!(result1, result2);
            
            if val > 0.0 {
                let sqrt1 = ovie_sqrt(val).unwrap();
                let sqrt2 = ovie_sqrt(val).unwrap();
                assert_eq!(sqrt1, sqrt2);
                
                let ln1 = ovie_ln(val).unwrap();
                let ln2 = ovie_ln(val).unwrap();
                assert_eq!(ln1, ln2);
            }
            
            let exp1 = ovie_exp(val);
            let exp2 = ovie_exp(val);
            assert_eq!(exp1, exp2);
        }
    }
}