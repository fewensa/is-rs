/// Returns `true` if the string is empty.
///
/// # Examples
///
/// ```
/// use is_rs::presence::is_empty_str;
/// assert!(is_empty_str(""));
/// assert!(!is_empty_str("hello"));
/// ```
pub fn is_empty_str(s: &str) -> bool {
    s.is_empty()
}

/// Returns `true` if the slice is empty.
///
/// # Examples
///
/// ```
/// use is_rs::presence::is_empty_slice;
/// assert!(is_empty_slice::<i32>(&[]));
/// assert!(!is_empty_slice(&[1, 2, 3]));
/// ```
pub fn is_empty_slice<T>(v: &[T]) -> bool {
    v.is_empty()
}

/// Returns `true` if the `Option` contains a value (`Some`).
///
/// # Examples
///
/// ```
/// use is_rs::presence::is_existy;
/// assert!(is_existy(&Some(42)));
/// assert!(!is_existy::<i32>(&None));
/// ```
pub fn is_existy<T>(v: &Option<T>) -> bool {
    v.is_some()
}

/// Returns `true` if the boolean is `true`.
///
/// # Examples
///
/// ```
/// use is_rs::presence::is_truthy;
/// assert!(is_truthy(true));
/// assert!(!is_truthy(false));
/// ```
pub fn is_truthy(v: bool) -> bool {
    v
}

/// Returns `true` if the boolean is `false`.
///
/// # Examples
///
/// ```
/// use is_rs::presence::is_falsy;
/// assert!(is_falsy(false));
/// assert!(!is_falsy(true));
/// ```
pub fn is_falsy(v: bool) -> bool {
    !v
}

/// Returns `true` if the string consists entirely of whitespace characters.
///
/// An empty string returns `false` (no whitespace to speak of).
///
/// # Examples
///
/// ```
/// use is_rs::presence::is_space;
/// assert!(is_space("   "));
/// assert!(is_space("\t\n"));
/// assert!(!is_space(""));
/// assert!(!is_space("  a  "));
/// ```
pub fn is_space(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_whitespace())
}
