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
    assert!(!is_existy::<i32>(&None));
}

#[test]
fn test_is_truthy_positive() {
    assert!(is_truthy(true));
}

#[test]
fn test_is_truthy_negative() {
    assert!(!is_truthy(false));
}

#[test]
fn test_is_falsy_positive() {
    assert!(is_falsy(false));
}

#[test]
fn test_is_falsy_negative() {
    assert!(!is_falsy(true));
}

#[test]
fn test_is_space_positive() {
    assert!(is_space("   "));
    assert!(is_space("\t\n\r"));
}

#[test]
fn test_is_space_negative() {
    assert!(!is_space("hello"));
    assert!(!is_space("  a  "));
    // empty string is not "space"
    assert!(!is_space(""));
}
