/// Returns `true` if `s` contains the substring `sub`.
pub fn includes(s: &str, sub: &str) -> bool {
    s.contains(sub)
}

/// Returns `true` if `s` is non-empty and every alphabetic character is uppercase.
pub fn is_upper_case(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| !c.is_alphabetic() || c.is_uppercase())
}

/// Returns `true` if `s` is non-empty and every alphabetic character is lowercase.
pub fn is_lower_case(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| !c.is_alphabetic() || c.is_lowercase())
}

/// Returns `true` if `s` starts with the substring `sub`.
pub fn starts_with(s: &str, sub: &str) -> bool {
    s.starts_with(sub)
}

/// Returns `true` if `s` ends with the substring `sub`.
pub fn ends_with(s: &str, sub: &str) -> bool {
    s.ends_with(sub)
}

/// Returns `true` if `s` is non-empty and its first character is uppercase.
pub fn is_capitalized(s: &str) -> bool {
    s.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
}

/// Returns `true` if `s` reads the same forwards and backwards (case-sensitive).
pub fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let rev: Vec<char> = chars.iter().copied().rev().collect();
    chars == rev
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(!is_upper_case(""));
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
        assert!(!is_lower_case(""));
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
        assert!(!is_capitalized("hello"));
        assert!(!is_capitalized(""));
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("madam"));
        assert!(is_palindrome("")); // empty string is palindrome
        assert!(is_palindrome("a"));
        assert!(!is_palindrome("hello"));
        assert!(!is_palindrome("Racecar")); // case-sensitive
    }
}
