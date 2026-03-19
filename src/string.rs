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

/// Returns `true` if `s` is uppercase.
///
/// # Examples
///
/// ```
/// use is_rs::string::is_upper_case;
/// assert!(is_upper_case("HELLO"));
/// assert!(is_upper_case("HELLO 123"));
/// assert!(!is_upper_case("Hello"));
/// assert!(is_upper_case(""));
/// ```
pub fn is_upper_case(s: &str) -> bool {
    s == s.to_uppercase()
}

/// Returns `true` if `s` is lowercase.
///
/// # Examples
///
/// ```
/// use is_rs::string::is_lower_case;
/// assert!(is_lower_case("hello"));
/// assert!(is_lower_case("hello 123"));
/// assert!(!is_lower_case("Hello"));
/// assert!(is_lower_case(""));
/// ```
pub fn is_lower_case(s: &str) -> bool {
    s == s.to_lowercase()
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

/// Returns `true` if every non-empty word in `s` starts with an uppercase letter.
///
/// # Examples
///
/// ```
/// use is_rs::string::is_capitalized;
/// assert!(is_capitalized("Hello"));
/// assert!(is_capitalized("Hello World"));
/// assert!(!is_capitalized("hello"));
/// assert!(is_capitalized(""));
/// ```
pub fn is_capitalized(s: &str) -> bool {
    s.split(' ')
        .filter(|word| !word.is_empty())
        .all(|word| word.chars().next().is_some_and(char::is_uppercase))
}

/// Returns `true` if `s` is a palindrome after removing non-alphanumeric
/// characters and normalizing case.
///
/// # Examples
///
/// ```
/// use is_rs::string::is_palindrome;
/// assert!(is_palindrome("racecar"));
/// assert!(is_palindrome("A man, a plan, a canal: Panama"));
/// assert!(is_palindrome(""));
/// assert!(!is_palindrome("hello"));
/// ```
pub fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect();
    let rev: Vec<char> = chars.iter().rev().copied().collect();
    chars == rev
}
