/// Returns `true` if `s` contains the substring `sub`.
///
/// # Examples
///
/// ```
/// use is_rs::string::includes;
/// assert!(includes("hello world", "world"));
/// assert!(!includes("hello world", "xyz"));
/// ```
pub fn includes(s: &str, sub: &str) -> bool {
    s.contains(sub)
}

/// Returns `true` if `s` is non-empty and every alphabetic character is uppercase.
///
/// Non-alphabetic characters (digits, spaces, punctuation) are ignored.
///
/// # Examples
///
/// ```
/// use is_rs::string::is_upper_case;
/// assert!(is_upper_case("HELLO"));
/// assert!(is_upper_case("HELLO 123"));
/// assert!(!is_upper_case("Hello"));
/// assert!(!is_upper_case(""));
/// ```
pub fn is_upper_case(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| !c.is_alphabetic() || c.is_uppercase())
}

/// Returns `true` if `s` is non-empty and every alphabetic character is lowercase.
///
/// Non-alphabetic characters (digits, spaces, punctuation) are ignored.
///
/// # Examples
///
/// ```
/// use is_rs::string::is_lower_case;
/// assert!(is_lower_case("hello"));
/// assert!(is_lower_case("hello 123"));
/// assert!(!is_lower_case("Hello"));
/// assert!(!is_lower_case(""));
/// ```
pub fn is_lower_case(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| !c.is_alphabetic() || c.is_lowercase())
}

/// Returns `true` if `s` starts with the substring `sub`.
///
/// # Examples
///
/// ```
/// use is_rs::string::starts_with;
/// assert!(starts_with("hello world", "hello"));
/// assert!(!starts_with("hello world", "world"));
/// ```
pub fn starts_with(s: &str, sub: &str) -> bool {
    s.starts_with(sub)
}

/// Returns `true` if `s` ends with the substring `sub`.
///
/// # Examples
///
/// ```
/// use is_rs::string::ends_with;
/// assert!(ends_with("hello world", "world"));
/// assert!(!ends_with("hello world", "hello"));
/// ```
pub fn ends_with(s: &str, sub: &str) -> bool {
    s.ends_with(sub)
}

/// Returns `true` if `s` is non-empty and its first character is uppercase.
///
/// # Examples
///
/// ```
/// use is_rs::string::is_capitalized;
/// assert!(is_capitalized("Hello"));
/// assert!(!is_capitalized("hello"));
/// assert!(!is_capitalized(""));
/// ```
pub fn is_capitalized(s: &str) -> bool {
    s.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
}

/// Returns `true` if `s` reads the same forwards and backwards (case-sensitive).
///
/// An empty string is considered a palindrome.
///
/// # Examples
///
/// ```
/// use is_rs::string::is_palindrome;
/// assert!(is_palindrome("racecar"));
/// assert!(is_palindrome(""));
/// assert!(!is_palindrome("hello"));
/// ```
pub fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let rev: Vec<char> = chars.iter().copied().rev().collect();
    chars == rev
}
