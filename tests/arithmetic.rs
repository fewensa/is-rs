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
}

#[test]
fn test_is_odd() {
    assert!(is_odd(1));
    assert!(is_odd(-3));
    assert!(!is_odd(0));
    assert!(!is_odd(4));
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
}

#[test]
fn test_is_integer() {
    assert!(is_integer(1.0));
    assert!(is_integer(0.0));
    assert!(is_integer(-4.0));
    assert!(!is_integer(1.5));
    assert!(!is_integer(-0.1));
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
