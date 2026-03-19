use is_rs::string::*;

#[test]
fn test_includes() {
    assert!(includes("hello world", "world"));
    assert!(!includes("hello world", "xyz"));
}

#[test]
fn test_includes_empty_sub() {
    // empty sub is always included
    assert!(includes("hello", ""));
}

#[test]
fn test_is_upper_case() {
    assert!(is_upper_case("HELLO"));
    assert!(is_upper_case("HELLO 123")); // digits/spaces are ignored
    assert!(!is_upper_case("Hello"));
    assert!(!is_upper_case("hello"));
}

#[test]
fn test_is_upper_case_empty() {
    assert!(is_upper_case(""));
}

#[test]
fn test_is_lower_case() {
    assert!(is_lower_case("hello"));
    assert!(is_lower_case("hello 123")); // digits/spaces are ignored
    assert!(!is_lower_case("Hello"));
    assert!(!is_lower_case("HELLO"));
}

#[test]
fn test_is_lower_case_empty() {
    assert!(is_lower_case(""));
}

#[test]
fn test_starts_with() {
    assert!(starts_with("hello world", "hello"));
    assert!(!starts_with("hello world", "world"));
}

#[test]
fn test_starts_with_empty_sub() {
    assert!(starts_with("hello", ""));
}

#[test]
fn test_ends_with() {
    assert!(ends_with("hello world", "world"));
    assert!(!ends_with("hello world", "hello"));
}

#[test]
fn test_ends_with_empty_sub() {
    assert!(ends_with("hello", ""));
}

#[test]
fn test_is_capitalized() {
    assert!(is_capitalized("Hello"));
    assert!(is_capitalized("HELLO"));
    assert!(is_capitalized("Hello World"));
    assert!(!is_capitalized("hello"));
    assert!(!is_capitalized("Hello world"));
    assert!(is_capitalized(""));
}

#[test]
fn test_is_palindrome() {
    assert!(is_palindrome("racecar"));
    assert!(is_palindrome("madam"));
    assert!(is_palindrome("Racecar"));
    assert!(is_palindrome("A man, a plan, a canal: Panama"));
    assert!(is_palindrome("")); // empty string is palindrome
    assert!(is_palindrome("a"));
    assert!(!is_palindrome("hello"));
}
