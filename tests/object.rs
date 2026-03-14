use is_rs::object::*;
use std::collections::HashMap;

fn make_map() -> HashMap<&'static str, i32> {
    let mut m = HashMap::new();
    m.insert("a", 1);
    m.insert("b", 2);
    m
}

#[test]
fn test_property_count_positive() {
    let m = make_map();
    assert!(property_count(&m, 2));
}

#[test]
fn test_property_count_negative() {
    let m = make_map();
    assert!(!property_count(&m, 3));
    assert!(!property_count(&m, 0));
}

#[test]
fn test_property_defined_positive() {
    let m = make_map();
    assert!(property_defined(&m, &"a"));
    assert!(property_defined(&m, &"b"));
}

#[test]
fn test_property_defined_negative() {
    let m = make_map();
    assert!(!property_defined(&m, &"c"));
    assert!(!property_defined(&m, &"z"));
}
