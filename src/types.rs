/// Returns `true` if `s` contains exactly one Unicode scalar value (is.js: `is.char`).
///
/// # Examples
///
/// ```
/// use is_rs::types::is_char;
/// assert!(is_char("a"));
/// assert!(is_char("你"));
/// assert!(!is_char(""));
/// assert!(!is_char("ab"));
/// ```
pub fn is_char(s: &str) -> bool {
    let mut chars = s.chars();

    chars.next().is_some() && chars.next().is_none()
}

/// Returns `true` if `v` is NaN (is.js: `is.nan`).
///
/// # Examples
///
/// ```
/// use is_rs::types::is_nan;
/// assert!(is_nan(f64::NAN));
/// assert!(!is_nan(1.0));
/// ```
pub fn is_nan(v: f64) -> bool {
    v.is_nan()
}

/// Returns `true` if `v` is a finite number, i.e. not NaN and not infinite (is.js: `is.number`).
///
/// # Examples
///
/// ```
/// use is_rs::types::is_number;
/// assert!(is_number(42.5));
/// assert!(!is_number(f64::NAN));
/// assert!(!is_number(f64::INFINITY));
/// ```
pub fn is_number(v: f64) -> bool {
    v.is_finite()
}

/// Returns `true` if `v` is a finite whole number (is.js: `is.integer`).
///
/// # Examples
///
/// ```
/// use is_rs::types::is_integer;
/// assert!(is_integer(42.0));
/// assert!(!is_integer(42.5));
/// assert!(!is_integer(f64::NEG_INFINITY));
/// ```
pub fn is_integer(v: f64) -> bool {
    v.is_finite() && v.fract() == 0.0
}
