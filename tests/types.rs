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

#[test]
fn null_and_undefined_map_to_none() {
    assert!(is_null(&Option::<i32>::None));
    assert!(is_undefined(&Option::<i32>::None));
    assert!(!is_null(&Some(1)));
    assert!(!is_undefined(&Some(1)));
}

#[test]
fn same_type_matches_concrete_types() {
    assert!(is_same_type(&42i32, &7i32));
    assert!(!is_same_type(&42i32, &7u32));
    assert!(is_same_type(&f64::NAN, &f64::NAN));
    assert!(!is_same_type(&f64::NAN, &1.0f64));
}
