use is_rs::types::*;

#[test]
fn char_accepts_single_unicode_scalar() {
    assert!(is_char("a"));
    assert!(is_char("你"));
    assert!(!is_char(""));
    assert!(!is_char("ab"));
}

#[test]
fn nan_only_matches_nan_values() {
    assert!(is_nan(f64::NAN));
    assert!(!is_nan(1.0));
}

#[test]
fn number_requires_finite_values() {
    assert!(is_number(42.5));
    assert!(!is_number(f64::NAN));
    assert!(!is_number(f64::INFINITY));
}

#[test]
fn integer_requires_finite_whole_numbers() {
    assert!(is_integer(42.0));
    assert!(is_integer(-0.0));
    assert!(!is_integer(42.5));
    assert!(!is_integer(f64::NEG_INFINITY));
}
