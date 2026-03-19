use is_rs::presence::*;

#[test]
fn test_is_empty_str_positive() {
    assert!(is_empty_str(""));
}

#[test]
fn test_is_empty_str_negative() {
    assert!(!is_empty_str("hello"));
}

#[test]
fn test_is_empty_slice_positive() {
    let v: &[i32] = &[];
    assert!(is_empty_slice(v));
}

#[test]
fn test_is_empty_slice_negative() {
    assert!(!is_empty_slice(&[1, 2, 3]));
}

#[test]
fn test_is_existy_positive() {
    assert!(is_existy(&Some(42)));
}

#[test]
fn test_is_existy_negative() {
    assert!(!is_existy(&Option::<i32>::None));
}

#[test]
fn test_is_existy_non_option_values() {
    assert!(is_existy("hello"));
    assert!(is_existy(0));
}

#[test]
fn test_is_truthy_positive() {
    assert!(is_truthy(true));
    assert!(is_truthy(1));
    assert!(is_truthy("hello"));
}

#[test]
fn test_is_truthy_negative() {
    assert!(!is_truthy(false));
    assert!(!is_truthy(0));
    assert!(!is_truthy(""));
    assert!(!is_truthy(f64::NAN));
}

#[test]
fn test_is_falsy_positive() {
    assert!(is_falsy(false));
    assert!(is_falsy(0));
    assert!(is_falsy(""));
}

#[test]
fn test_is_falsy_negative() {
    assert!(!is_falsy(true));
    assert!(!is_falsy(1));
}

#[test]
fn test_is_space_positive() {
    assert!(is_space(" "));
    assert!(is_space("\t"));
}

#[test]
fn test_is_space_negative() {
    assert!(!is_space("hello"));
    assert!(!is_space("  "));
    assert!(!is_space("\t\n"));
    assert!(!is_space(""));
}

#[test]
fn test_is_empty_generic() {
    use std::collections::HashMap;

    let empty: &[i32] = &[];
    assert!(is_empty(""));
    assert!(is_empty(empty));

    let map: HashMap<&str, i32> = HashMap::new();
    assert!(is_empty(&map));
}
