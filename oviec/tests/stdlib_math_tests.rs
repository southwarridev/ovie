// Ovie Standard Library - Math Module Tests
// Comprehensive tests for std::math module functionality
// Tests mathematical constants, arithmetic operations, and functions

use std::process::Command;
use std::fs;

#[cfg(test)]
mod math_tests {
    use super::*;

    // Helper function to compile and run Ovie code
    fn run_ovie_code(code: &str) -> Result<String, String> {
        let test_file = "test_temp.ov";
        fs::write(test_file, code).map_err(|e| format!("Failed to write test file: {}", e))?;
        
        let output = Command::new("./oviec")
            .arg("run")
            .arg(test_file)
            .output()
            .map_err(|e| format!("Failed to execute oviec: {}", e))?;
        
        let _ = fs::remove_file(test_file);
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    // Helper function to check if two floats are approximately equal
    fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() < epsilon
    }

    // ===== MATHEMATICAL CONSTANTS TESTS =====

    #[test]
    fn test_mathematical_constants() {
        let code = r#"
            use std::math::{PI, E, TAU, SQRT_2, SQRT_3, LN_2, LN_10};
            
            fn main() {
                // Test PI
                print("PI:");
                print(PI.to_string());
                print("\n");
                
                // Test E
                print("E:");
                print(E.to_string());
                print("\n");
                
                // Test TAU (2*PI)
                print("TAU:");
                print(TAU.to_string());
                print("\n");
                
                // Test SQRT_2
                print("SQRT_2:");
                print(SQRT_2.to_string());
                print("\n");
                
                // Test SQRT_3
                print("SQRT_3:");
                print(SQRT_3.to_string());
                print("\n");
            }
        "#;
        
        let output = run_ovie_code(code).unwrap();
        let lines: Vec<&str> = output.trim().split('\n').collect();
        
        // Verify constants are deterministic and have expected values
        assert!(lines.len() >= 10);
        
        // Check PI (approximately 3.14159...)
        let pi_line = lines.iter().find(|&&line| line.starts_with("PI:")).unwrap();
        let pi_value: f64 = pi_line[3..].parse().unwrap();
        assert!(approx_equal(pi_value, std::f64::consts::PI, 1e-10));
        
        // Check E (approximately 2.71828...)
        let e_line = lines.iter().find(|&&line| line.starts_with("E:")).unwrap();
        let e_value: f64 = e_line[2..].parse().unwrap();
        assert!(approx_equal(e_value, std::f64::consts::E, 1e-10));
    }

    #[test]
    fn test_floating_point_limits() {
        let code = r#"
            use std::math::{INFINITY, NEG_INFINITY, NAN, EPSILON, is_infinite, is_nan};
            
            fn main() {
                // Test infinity
                if is_infinite(INFINITY) {
                    print("INF_PASS");
                } else {
                    print("INF_FAIL");
                }
                print("\n");
                
                // Test negative infinity
                if is_infinite(NEG_INFINITY) {
                    print("NEG_INF_PASS");
                } else {
                    print("NEG_INF_FAIL");
                }
                print("\n");
                
                // Test NaN
                if is_nan(NAN) {
                    print("NAN_PASS");
                } else {
                    print("NAN_FAIL");
                }
                print("\n");
                
                // Test epsilon is small positive number
                if EPSILON > 0.0 && EPSILON < 1e-10 {
                    print("EPSILON_PASS");
                } else {
                    print("EPSILON_FAIL");
                }
            }
        "#;
        
        let output = run_ovie_code(code).unwrap();
        assert!(output.contains("INF_PASS"));
        assert!(output.contains("NEG_INF_PASS"));
        assert!(output.contains("NAN_PASS"));
        assert!(output.contains("EPSILON_PASS"));
    }

    #[test]
    fn test_integer_limits() {
        let code = r#"
            use std::math::{MAX_INT, MIN_INT};
            
            fn main() {
                // Test that MAX_INT is positive and large
                if MAX_INT > 1000000000 {
                    print("MAX_INT_PASS");
                } else {
                    print("MAX_INT_FAIL");
                }
                print("\n");
                
                // Test that MIN_INT is negative and large in magnitude
                if MIN_INT < -1000000000 {
                    print("MIN_INT_PASS");
                } else {
                    print("MIN_INT_FAIL");
                }
                print("\n");
                
                // Test that they are opposites (approximately)
                if MAX_INT == -MIN_INT - 1 {
                    print("LIMITS_SYMMETRIC_PASS");
                } else {
                    print("LIMITS_SYMMETRIC_FAIL");
                }
            }
        "#;
        
        let output = run_ovie_code(code).unwrap();
        assert!(output.contains("MAX_INT_PASS"));
        assert!(output.contains("MIN_INT_PASS"));
        assert!(output.contains("LIMITS_SYMMETRIC_PASS"));
    }

    // ===== CHECKED ARITHMETIC TESTS =====

    #[test]
    fn test_checked_add() {
        let code = r#"
            use std::math::checked_add;
            
            fn main() {
                // Test normal addition
                let result1 = checked_add(5, 3);
                if result1.is_ok() && result1.unwrap() == 8 {
                    print("ADD_NORMAL_PASS");
                } else {
                    print("ADD_NORMAL_FAIL");
                }
                print("\n");
                
                // Test zero addition
                let result2 = checked_add(42, 0);
                if result2.is_ok() && result2.unwrap() == 42 {
                    print("ADD_ZERO_PASS");
                } else {
                    print("ADD_ZERO_FAIL");
                }
                print("\n");
                
                // Test negative addition
                let result3 = checked_add(-10, 5);
                if result3.is_ok() && result3.unwrap() == -5 {
                    print("ADD_NEGATIVE_PASS");
                } else {
                    print("ADD_NEGATIVE_FAIL");
                }
            }
        "#;
        
        let output = run_ovie_code(code).unwrap();
        assert!(output.contains("ADD_NORMAL_PASS"));
        assert!(output.contains("ADD_ZERO_PASS"));
        assert!(output.contains("ADD_NEGATIVE_PASS"));
    }

    #[test]
    fn test_checked_sub() {
        let code = r#"
            use std::math::checked_sub;
            
            fn main() {
                // Test normal subtraction
                let result1 = checked_sub(10, 3);
                if result1.is_ok() && result1.unwrap() == 7 {
                    print("SUB_NORMAL_PASS");
                } else {
                    print("SUB_NORMAL_FAIL");
                }
                print("\n");
                
                // Test zero subtraction
                let result2 = checked_sub(42, 0);
                if result2.is_ok() && result2.unwrap() == 42 {
                    print("SUB_ZERO_PASS");
                } else {
                    print("SUB_ZERO_FAIL");
                }
                print("\n");
                
                // Test negative result
                let result3 = checked_sub(5, 10);
                if result3.is_ok() && result3.unwrap() == -5 {
                    print("SUB_NEGATIVE_PASS");
                } else {
                    print("SUB_NEGATIVE_FAIL");
                }
            }
        "#;
        
        let output = run_ovie_code(code).unwrap();
        assert!(output.contains("SUB_NORMAL_PASS"));
        assert!(output.contains("SUB_ZERO_PASS"));
        assert!(output.contains("SUB_NEGATIVE_PASS"));
    }

    #[test]
    fn test_checked_mul() {
        let code = r#"
            use std::math::checked_mul;
            
            fn main() {
                // Test normal multiplication
                let result1 = checked_mul(6, 7);
                if result1.is_ok() && result1.unwrap() == 42 {
                    print("MUL_NORMAL_PASS");
                } else {
                    print("MUL_NORMAL_FAIL");
                }
                print("\n");
                
                // Test zero multiplication
                let result2 = checked_mul(42, 0);
                if result2.is_ok() && result2.unwrap() == 0 {
                    print("MUL_ZERO_PASS");
                } else {
                    print("MUL_ZERO_FAIL");
                }
                print("\n");
                
                // Test negative multiplication
                let result3 = checked_mul(-6, 7);
                if result3.is_ok() && result3.unwrap() == -42 {
                    print("MUL_NEGATIVE_PASS");
                } else {
                    print("MUL_NEGATIVE_FAIL");
                }
            }
        "#;
        
        let output = run_ovie_code(code).unwrap();
        assert!(output.contains("MUL_NORMAL_PASS"));
        assert!(output.contains("MUL_ZERO_PASS"));
        assert!(output.contains("MUL_NEGATIVE_PASS"));
    }

    #[test]
    fn test_checked_div() {
        let code = r#"
            use std::math::checked_div;
            
            fn main() {
                // Test normal division
                let result1 = checked_div(42, 6);
                if result1.is_ok() && result1.unwrap() == 7 {
                    print("DIV_NORMAL_PASS");
                } else {
                    print("DIV_NORMAL_FAIL");
                }
                print("\n");
                
                // Test division by zero (should fail)
                let result2 = checked_div(42, 0);
                if result2.is_err() {
                    print("DIV_ZERO_PASS");
                } else {
                    print("DIV_ZERO_FAIL");
                }
                print("\n");
                
                // Test negative division
                let result3 = checked_div(-42, 6);
                if result3.is_ok() && result3.unwrap() == -7 {
                    print("DIV_NEGATIVE_PASS");
                } else {
                    print("DIV_NEGATIVE_FAIL");
                }
            }
        "#;
        
        let output = run_ovie_code(code).unwrap();
        assert!(output.contains("DIV_NORMAL_PASS"));
        assert!(output.contains("DIV_ZERO_PASS"));
        assert!(output.contains("DIV_NEGATIVE_PASS"));
    }

    // ===== POWER AND ROOT FUNCTION TESTS =====

    #[test]
    fn test_pow_function() {
        let code = r#"
            use std::math::pow;
            
            fn main() {
                // Test basic power
                let result1 = pow(2, 3);
                if result1.is_ok() && result1.unwrap() == 8 {
                    print("POW_BASIC_PASS");
                } else {
                    print("POW_BASIC_FAIL");
                }
                print("\n");
                
                // Test power of 1
                let result2 = pow(42, 1);
                if result2.is_ok() && result2.unwrap() == 42 {
                    print("POW_ONE_PASS");
                } else {
                    print("POW_ONE_FAIL");
                }
                print("\n");
                
                // Test power of 0
                let result3 = pow(42, 0);
                if result3.is_ok() && result3.unwrap() == 1 {
                    print("POW_ZERO_PASS");
                } else {
                    print("POW_ZERO_FAIL");
                }
            }
        "#;
        
        let output = run_ovie_code(code).unwrap();
        assert!(output.contains("POW_BASIC_PASS"));
        assert!(output.contains("POW_ONE_PASS"));
        assert!(output.contains("POW_ZERO_PASS"));
    }

    #[test]
    fn test_sqrt_function() {
        let code = r#"
            use std::math::{sqrt, approx_eq};
            
            fn main() {
                // Test perfect square
                let result1 = sqrt(16);
                if result1.is_ok() && result1.unwrap() == 4 {
                    print("SQRT_PERFECT_PASS");
                } else {
                    print("SQRT_PERFECT_FAIL");
                }
                print("\n");
                
                // Test sqrt of 1
                let result2 = sqrt(1);
                if result2.is_ok() && result2.unwrap() == 1 {
                    print("SQRT_ONE_PASS");
                } else {
                    print("SQRT_ONE_FAIL");
                }
                print("\n");
                
                // Test sqrt of 0
                let result3 = sqrt(0);
                if result3.is_ok() && result3.unwrap() == 0 {
                    print("SQRT_ZERO_PASS");
                } else {
                    print("SQRT_ZERO_FAIL");
                }
                print("\n");
                
                // Test negative sqrt (should fail)
                let result4 = sqrt(-1);
                if result4.is_err() {
                    print("SQRT_NEGATIVE_PASS");
                } else {
                    print("SQRT_NEGATIVE_FAIL");
                }
            }
        "#;
        
        let output = run_ovie_code(code).unwrap();
        assert!(output.contains("SQRT_PERFECT_PASS"));
        assert!(output.contains("SQRT_ONE_PASS"));
        assert!(output.contains("SQRT_ZERO_PASS"));
        assert!(output.contains("SQRT_NEGATIVE_PASS"));
    }

    #[test]
    fn test_cbrt_function() {
        let code = r#"
            use std::math::{cbrt, approx_eq};
            
            fn main() {
                // Test perfect cube
                let result1 = cbrt(27);
                if approx_eq(result1, 3.0, 1e-10) {
                    print("CBRT_PERFECT_PASS");
                } else {
                    print("CBRT_PERFECT_FAIL");
                }
                print("\n");
                
                // Test cbrt of 1
                let result2 = cbrt(1);
                if approx_eq(result2, 1.0, 1e-10) {
                    print("CBRT_ONE_PASS");
                } else {
                    print("CBRT_ONE_FAIL");
                }
                print("\n");
                
                // Test cbrt of 0
                let result3 = cbrt(0);
                if result3 == 0 {
                    print("CBRT_ZERO_PASS");
                } else {
                    print("CBRT_ZERO_FAIL");
                }
                print("\n");
                
                // Test negative cbrt
                let result4 = cbrt(-8);
                if approx_eq(result4, -2.0, 1e-10) {
                    print("CBRT_NEGATIVE_PASS");
                } else {
                    print("CBRT_NEGATIVE_FAIL");
                }
            }
        "#;
        
        let output = run_ovie_code(code).unwrap();
        assert!(output.contains("CBRT_PERFECT_PASS"));
        assert!(output.contains("CBRT_ONE_PASS"));
        assert!(output.contains("CBRT_ZERO_PASS"));
        assert!(output.contains("CBRT_NEGATIVE_PASS"));
    }

    // ===== TRIGONOMETRIC FUNCTION TESTS =====

    #[test]
    fn test_sin_function() {
        let code = r#"
            use std::math::{sin, PI, approx_eq};
            
            fn main() {
                // Test sin(0)
                let result1 = sin(0);
                if approx_eq(result1, 0.0, 1e-10) {
                    print("SIN_ZERO_PASS");
                } else {
                    print("SIN_ZERO_FAIL");
                }
                print("\n");
                
                // Test sin(PI/2) ≈ 1
                let result2 = sin(PI / 2);
                if approx_eq(result2, 1.0, 1e-10) {
                    print("SIN_PI_HALF_PASS");
                } else {
                    print("SIN_PI_HALF_FAIL");
                }
                print("\n");
                
                // Test sin(PI) ≈ 0
                let result3 = sin(PI);
                if approx_eq(result3, 0.0, 1e-10) {
                    print("SIN_PI_PASS");
                } else {
                    print("SIN_PI_FAIL");
                }
            }
        "#;
        
        let output = run_ovie_code(code).unwrap();
        assert!(output.contains("SIN_ZERO_PASS"));
        assert!(output.contains("SIN_PI_HALF_PASS"));
        assert!(output.contains("SIN_PI_PASS"));
    }

    #[test]
    fn test_cos_function() {
        let code = r#"
            use std::math::{cos, PI, approx_eq};
            
            fn main() {
                // Test cos(0) = 1
                let result1 = cos(0);
                if approx_eq(result1, 1.0, 1e-10) {
                    print("COS_ZERO_PASS");
                } else {
                    print("COS_ZERO_FAIL");
                }
                print("\n");
                
                // Test cos(PI/2) ≈ 0
                let result2 = cos(PI / 2);
                if approx_eq(result2, 0.0, 1e-10) {
                    print("COS_PI_HALF_PASS");
                } else {
                    print("COS_PI_HALF_FAIL");
                }
                print("\n");
                
                // Test cos(PI) ≈ -1
                let result3 = cos(PI);
                if approx_eq(result3, -1.0, 1e-10) {
                    print("COS_PI_PASS");
                } else {
                    print("COS_PI_FAIL");
                }
            }
        "#;
        
        let output = run_ovie_code(code).unwrap();
        assert!(output.contains("COS_ZERO_PASS"));
        assert!(output.contains("COS_PI_HALF_PASS"));
        assert!(output.contains("COS_PI_PASS"));
    }

    #[test]
    fn test_tan_function() {
        let code = r#"
            use std::math::{tan, PI, approx_eq};
            
            fn main() {
                // Test tan(0) = 0
                let result1 = tan(0);
                if result1.is_ok() && approx_eq(result1.unwrap(), 0.0, 1e-10) {
                    print("TAN_ZERO_PASS");
                } else {
                    print("TAN_ZERO_FAIL");
                }
                print("\n");
                
                // Test tan(PI) ≈ 0
                let result2 = tan(PI);
                if result2.is_ok() && approx_eq(result2.unwrap(), 0.0, 1e-10) {
                    print("TAN_PI_PASS");
                } else {
                    print("TAN_PI_FAIL");
                }
                print("\n");
                
                // Test tan(PI/2) should fail (undefined)
                let result3 = tan(PI / 2);
                if result3.is_err() {
                    print("TAN_PI_HALF_PASS");
                } else {
                    print("TAN_PI_HALF_FAIL");
                }
            }
        "#;
        
        let output = run_ovie_code(code).unwrap();
        assert!(output.contains("TAN_ZERO_PASS"));
        assert!(output.contains("TAN_PI_PASS"));
        assert!(output.contains("TAN_PI_HALF_PASS"));
    }

    // ===== UTILITY FUNCTION TESTS =====

    #[test]
    fn test_abs_function() {
        let code = r#"
            use std::math::abs;
            
            fn main() {
                // Test positive number
                let result1 = abs(42);
                if result1 == 42 {
                    print("ABS_POSITIVE_PASS");
                } else {
                    print("ABS_POSITIVE_FAIL");
                }
                print("\n");
                
                // Test negative number
                let result2 = abs(-42);
                if result2 == 42 {
                    print("ABS_NEGATIVE_PASS");
                } else {
                    print("ABS_NEGATIVE_FAIL");
                }
                print("\n");
                
                // Test zero
                let result3 = abs(0);
                if result3 == 0 {
                    print("ABS_ZERO_PASS");
                } else {
                    print("ABS_ZERO_FAIL");
                }
            }
        "#;
        
        let output = run_ovie_code(code).unwrap();
        assert!(output.contains("ABS_POSITIVE_PASS"));
        assert!(output.contains("ABS_NEGATIVE_PASS"));
        assert!(output.contains("ABS_ZERO_PASS"));
    }

    #[test]
    fn test_sign_function() {
        let code = r#"
            use std::math::sign;
            
            fn main() {
                // Test positive number
                let result1 = sign(42);
                if result1 == 1 {
                    print("SIGN_POSITIVE_PASS");
                } else {
                    print("SIGN_POSITIVE_FAIL");
                }
                print("\n");
                
                // Test negative number
                let result2 = sign(-42);
                if result2 == -1 {
                    print("SIGN_NEGATIVE_PASS");
                } else {
                    print("SIGN_NEGATIVE_FAIL");
                }
                print("\n");
                
                // Test zero
                let result3 = sign(0);
                if result3 == 0 {
                    print("SIGN_ZERO_PASS");
                } else {
                    print("SIGN_ZERO_FAIL");
                }
            }
        "#;
        
        let output = run_ovie_code(code).unwrap();
        assert!(output.contains("SIGN_POSITIVE_PASS"));
        assert!(output.contains("SIGN_NEGATIVE_PASS"));
        assert!(output.contains("SIGN_ZERO_PASS"));
    }

    #[test]
    fn test_floor_ceil_round() {
        let code = r#"
            use std::math::{floor, ceil, round};
            
            fn main() {
                // Test floor
                let floor1 = floor(3.7);
                let floor2 = floor(-3.7);
                if floor1 == 3 && floor2 == -4 {
                    print("FLOOR_PASS");
                } else {
                    print("FLOOR_FAIL");
                }
                print("\n");
                
                // Test ceil
                let ceil1 = ceil(3.2);
                let ceil2 = ceil(-3.2);
                if ceil1 == 4 && ceil2 == -3 {
                    print("CEIL_PASS");
                } else {
                    print("CEIL_FAIL");
                }
                print("\n");
                
                // Test round
                let round1 = round(3.4);
                let round2 = round(3.6);
                let round3 = round(-3.4);
                let round4 = round(-3.6);
                if round1 == 3 && round2 == 4 && round3 == -3 && round4 == -4 {
                    print("ROUND_PASS");
                } else {
                    print("ROUND_FAIL");
                }
            }
        "#;
        
        let output = run_ovie_code(code).unwrap();
        assert!(output.contains("FLOOR_PASS"));
        assert!(output.contains("CEIL_PASS"));
        assert!(output.contains("ROUND_PASS"));
    }

    // ===== CLASSIFICATION FUNCTION TESTS =====

    #[test]
    fn test_classification_functions() {
        let code = r#"
            use std::math::{is_finite, is_infinite, is_nan, is_normal, INFINITY, NAN};
            
            fn main() {
                // Test is_finite
                if is_finite(42.0) && !is_finite(INFINITY) && !is_finite(NAN) {
                    print("IS_FINITE_PASS");
                } else {
                    print("IS_FINITE_FAIL");
                }
                print("\n");
                
                // Test is_infinite
                if is_infinite(INFINITY) && !is_infinite(42.0) && !is_infinite(NAN) {
                    print("IS_INFINITE_PASS");
                } else {
                    print("IS_INFINITE_FAIL");
                }
                print("\n");
                
                // Test is_nan
                if is_nan(NAN) && !is_nan(42.0) && !is_nan(INFINITY) {
                    print("IS_NAN_PASS");
                } else {
                    print("IS_NAN_FAIL");
                }
                print("\n");
                
                // Test is_normal
                if is_normal(42.0) && !is_normal(0.0) && !is_normal(INFINITY) && !is_normal(NAN) {
                    print("IS_NORMAL_PASS");
                } else {
                    print("IS_NORMAL_FAIL");
                }
            }
        "#;
        
        let output = run_ovie_code(code).unwrap();
        assert!(output.contains("IS_FINITE_PASS"));
        assert!(output.contains("IS_INFINITE_PASS"));
        assert!(output.contains("IS_NAN_PASS"));
        assert!(output.contains("IS_NORMAL_PASS"));
    }

    // ===== PROPERTY-BASED TESTS =====

    #[test]
    fn test_arithmetic_properties() {
        // Test that arithmetic operations are deterministic
        for i in 1..10 {
            let code = format!(r#"
                use std::math::{{checked_add, checked_mul}};
                
                fn main() {{
                    let add_result = checked_add({}, 10);
                    let mul_result = checked_mul({}, 2);
                    
                    if add_result.is_ok() && mul_result.is_ok() {{
                        let sum = add_result.unwrap() + mul_result.unwrap();
                        print(sum.to_string());
                    }} else {{
                        print("ERROR");
                    }}
                }}
            "#, i, i);
            
            let output1 = run_ovie_code(&code).unwrap();
            let output2 = run_ovie_code(&code).unwrap();
            
            assert_eq!(output1, output2, "Arithmetic operations should be deterministic");
            
            let expected = i + 10 + i * 2;
            assert_eq!(output1.trim(), expected.to_string());
        }
    }

    #[test]
    fn test_trigonometric_properties() {
        // Test that trigonometric functions are deterministic
        let test_values = vec![0.0, 0.5, 1.0, 1.5];
        
        for value in test_values {
            let code = format!(r#"
                use std::math::{{sin, cos, approx_eq}};
                
                fn main() {{
                    let sin_val = sin({});
                    let cos_val = cos({});
                    
                    // Test Pythagorean identity: sin²(x) + cos²(x) = 1
                    let identity = sin_val * sin_val + cos_val * cos_val;
                    
                    if approx_eq(identity, 1.0, 1e-10) {{
                        print("IDENTITY_PASS");
                    }} else {{
                        print("IDENTITY_FAIL");
                    }}
                }}
            "#, value, value);
            
            let output1 = run_ovie_code(&code).unwrap();
            let output2 = run_ovie_code(&code).unwrap();
            
            assert_eq!(output1, output2, "Trigonometric functions should be deterministic");
            assert_eq!(output1.trim(), "IDENTITY_PASS");
        }
    }

    #[test]
    fn test_power_properties() {
        // Test power function properties
        let code = r#"
            use std::math::{pow, approx_eq};
            
            fn main() {
                // Test a^0 = 1
                let pow_zero = pow(42, 0);
                if pow_zero.is_err() || !approx_eq(pow_zero.unwrap(), 1.0, 1e-10) {
                    print("POW_ZERO_FAIL");
                    return;
                }
                
                // Test a^1 = a
                let pow_one = pow(42, 1);
                if pow_one.is_err() || !approx_eq(pow_one.unwrap(), 42.0, 1e-10) {
                    print("POW_ONE_FAIL");
                    return;
                }
                
                // Test (a^m)^n = a^(m*n)
                let base = 2.0;
                let pow_2_3 = pow(base, 3);
                let pow_8_2 = pow(8.0, 2);
                let pow_2_6 = pow(base, 6);
                
                if pow_2_3.is_err() || pow_8_2.is_err() || pow_2_6.is_err() {
                    print("POW_CALC_FAIL");
                    return;
                }
                
                if approx_eq(pow_8_2.unwrap(), pow_2_6.unwrap(), 1e-10) {
                    print("POW_PROPERTIES_PASS");
                } else {
                    print("POW_PROPERTIES_FAIL");
                }
            }
        "#;
        
        let output = run_ovie_code(code).unwrap();
        assert_eq!(output.trim(), "POW_PROPERTIES_PASS");
    }

    // ===== EDGE CASE TESTS =====

    #[test]
    fn test_edge_cases() {
        let code = r#"
            use std::math::{checked_div, sqrt, pow, factorial};
            
            fn main() {
                // Test division by zero
                let div_zero = checked_div(1, 0);
                if div_zero.is_ok() {
                    print("DIV_ZERO_EDGE_FAIL");
                    return;
                }
                
                // Test sqrt of negative
                let sqrt_neg = sqrt(-1);
                if sqrt_neg.is_ok() {
                    print("SQRT_NEG_EDGE_FAIL");
                    return;
                }
                
                // Test 0^0 (should be 1 by convention)
                let zero_pow_zero = pow(0, 0);
                if zero_pow_zero.is_err() || zero_pow_zero.unwrap() != 1 {
                    print("ZERO_POW_ZERO_EDGE_FAIL");
                    return;
                }
                
                // Test factorial of 0 (should be 1)
                let fact_zero = factorial(0);
                if fact_zero.is_err() || fact_zero.unwrap() != 1 {
                    print("FACT_ZERO_EDGE_FAIL");
                    return;
                }
                
                print("EDGE_CASES_PASS");
            }
        "#;
        
        let output = run_ovie_code(code).unwrap();
        assert_eq!(output.trim(), "EDGE_CASES_PASS");
    }

    // ===== DETERMINISM TESTS =====

    #[test]
    fn test_deterministic_behavior() {
        // Test that all math operations produce identical results across multiple runs
        let code = r#"
            use std::math::{sin, cos, sqrt, pow, PI, E};
            
            fn main() {
                // Combine multiple operations
                let sin_pi = sin(PI);
                let cos_e = cos(E);
                let sqrt_2 = sqrt(2);
                let pow_result = pow(2, 10);
                
                if sqrt_2.is_ok() && pow_result.is_ok() {
                    let combined = sin_pi + cos_e + sqrt_2.unwrap() + pow_result.unwrap();
                    print(combined.to_string());
                } else {
                    print("ERROR");
                }
            }
        "#;
        
        // Run the same code multiple times
        let mut results = Vec::new();
        for _ in 0..5 {
            let output = run_ovie_code(code).unwrap();
            results.push(output.trim().to_string());
        }
        
        // All results should be identical
        for i in 1..results.len() {
            assert_eq!(results[0], results[i], "Math operations should be deterministic across runs");
        }
    }
}