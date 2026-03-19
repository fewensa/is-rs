use is_rs::arithmetic::*;

#[test]
fn test_is_equal() {
    assert!(is_equal(1, 1));
    assert!(is_equal("hello", "hello"));
    assert!(!is_equal(1, 2));
    assert!(!is_equal("a", "b"));
}

#[test]
fn test_is_even() {
    assert!(is_even(0));
    assert!(is_even(4));
    assert!(is_even(-2));
    assert!(!is_even(3));
    assert!(!is_even(-1));
    assert!(!is_even(4.5));
}

#[test]
fn test_is_odd() {
    assert!(is_odd(1));
    assert!(is_odd(-3));
    assert!(!is_odd(0));
    assert!(!is_odd(4));
    assert!(!is_odd(3.5));
}

#[test]
fn test_is_positive() {
    assert!(is_positive(1.0));
    assert!(is_positive(0.001));
    assert!(!is_positive(0.0));
    assert!(!is_positive(-1.0));
}

#[test]
fn test_is_negative() {
    assert!(is_negative(-1.0));
    assert!(is_negative(-0.001));
    assert!(!is_negative(0.0));
    assert!(!is_negative(1.0));
}

#[test]
fn test_is_above() {
    assert!(is_above(5.0, 3.0));
    assert!(!is_above(3.0, 5.0));
    assert!(!is_above(3.0, 3.0));
}

#[test]
fn test_is_under() {
    assert!(is_under(3.0, 5.0));
    assert!(!is_under(5.0, 3.0));
    assert!(!is_under(3.0, 3.0));
}

#[test]
fn test_is_within() {
    assert!(is_within(5.0, 3.0, 10.0));
    assert!(!is_within(3.0, 3.0, 10.0)); // exclusive lower bound
    assert!(!is_within(10.0, 3.0, 10.0)); // exclusive upper bound
    assert!(!is_within(1.0, 3.0, 10.0));
}

#[test]
fn test_is_decimal() {
    assert!(is_decimal(1.5));
    assert!(is_decimal(-0.1));
    assert!(!is_decimal(1.0));
    assert!(!is_decimal(0.0));
    assert!(!is_decimal(f64::NAN));
}

#[test]
fn test_is_integer() {
    assert!(is_integer(1.0));
    assert!(is_integer(0.0));
    assert!(is_integer(-4.0));
    assert!(!is_integer(1.5));
    assert!(!is_integer(-0.1));
    assert!(!is_integer(f64::NAN));
}

#[test]
fn test_is_finite() {
    assert!(is_finite(1.0));
    assert!(is_finite(0.0));
    assert!(!is_finite(f64::INFINITY));
    assert!(!is_finite(f64::NEG_INFINITY));
    assert!(!is_finite(f64::NAN));
}

#[test]
fn test_is_infinite() {
    assert!(is_infinite(f64::INFINITY));
    assert!(is_infinite(f64::NEG_INFINITY));
    assert!(!is_infinite(1.0));
    assert!(!is_infinite(f64::NAN));
}

// -----------------------------------------------------------------------
// Multi-type / Num trait tests — verifying the unified numeric input
// -----------------------------------------------------------------------

#[test]
fn test_even_multi_types() {
    // integer types
    assert!(is_even(4i32));
    assert!(is_even(4u8));
    assert!(is_even(4u64));
    assert!(is_even(4i8));
    assert!(!is_even(3i32));
    assert!(!is_even(3u16));

    // float (whole number)
    assert!(is_even(4.0f32));
    assert!(is_even(4.0f64));
    assert!(!is_even(3.0f64));

    // string
    assert!(is_even("4"));
    assert!(is_even("0"));
    assert!(!is_even("3"));
    assert!(!is_even("abc"));
    assert!(!is_even("4.5"));
}

#[test]
fn test_odd_multi_types() {
    assert!(is_odd(3i32));
    assert!(is_odd(3u32));
    assert!(is_odd(3.0f64));
    assert!(is_odd("3"));
    assert!(!is_odd("4"));
    assert!(!is_odd("abc"));
    assert!(!is_odd("3.5"));
}

#[test]
fn test_positive_multi_types() {
    assert!(is_positive(1i32));
    assert!(is_positive(1u64));
    assert!(is_positive(1.0f32));
    assert!(is_positive("1.5"));
    assert!(!is_positive(0i32));
    assert!(!is_positive(-1i64));
    assert!(!is_positive("0"));
    assert!(!is_positive("-1"));
    assert!(!is_positive("abc")); // NAN → not positive
}

#[test]
fn test_negative_multi_types() {
    assert!(is_negative(-1i32));
    assert!(is_negative(-1.0f32));
    assert!(is_negative("-2.5"));
    assert!(!is_negative(0i32));
    assert!(!is_negative(1u8));
}

#[test]
fn test_above_multi_types() {
    assert!(is_above(5i32, 3i32));
    assert!(is_above(5u64, 3u64));
    assert!(is_above(5.0f32, 3.0f32));
    assert!(is_above("5", "3"));
    assert!(!is_above(3i32, 5i32));
    assert!(!is_above(3i32, 3i32));
}

#[test]
fn test_under_multi_types() {
    assert!(is_under(3i32, 5i32));
    assert!(is_under(3.0f32, 5.0f32));
    assert!(is_under("3", "5"));
    assert!(!is_under(5i32, 3i32));
}

#[test]
fn test_within_multi_types() {
    assert!(is_within(5i32, 3i32, 10i32));
    assert!(is_within(5u8, 3u8, 10u8));
    assert!(is_within(5.0f32, 3.0f32, 10.0f32));
    assert!(is_within("5", "3", "10"));
    assert!(!is_within(3i32, 3i32, 10i32));
    assert!(!is_within(10i32, 3i32, 10i32));
}

#[test]
fn test_decimal_multi_types() {
    assert!(is_decimal(1.5f32));
    assert!(is_decimal(1.5f64));
    assert!(is_decimal("1.5"));
    assert!(!is_decimal(1i32));
    assert!(!is_decimal("1"));
    assert!(!is_decimal("1.0"));
    assert!(!is_decimal("abc"));
}

#[test]
fn test_integer_multi_types() {
    assert!(is_integer(1i32));
    assert!(is_integer(0u8));
    assert!(is_integer(1.0f64));
    assert!(is_integer("2"));
    assert!(!is_integer(1.5f64));
    assert!(!is_integer("1.5"));
    assert!(!is_integer("abc"));
}

#[test]
fn test_finite_multi_types() {
    assert!(is_finite(1i32));
    assert!(is_finite(0u8));
    assert!(is_finite(1.0f32));
    assert!(is_finite("42"));
    assert!(!is_finite(f64::INFINITY));
    assert!(!is_finite("abc")); // NAN → not finite
}

#[test]
fn test_infinite_multi_types() {
    assert!(is_infinite(f64::INFINITY));
    assert!(is_infinite(f64::NEG_INFINITY));
    assert!(!is_infinite(1i32));
    assert!(!is_infinite("42"));
}

#[test]
fn test_str_string_equivalence() {
    // &str and String::as_str() should give identical results
    let s = String::from("4");
    assert!(is_even(s.as_str()));
    assert!(is_positive(s.as_str()));
}
