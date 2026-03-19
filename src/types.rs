use std::any::{Any, TypeId};

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

/// Returns `true` if the `Option` is `None` (Rust equivalent of is.js `is.null`).
///
/// # Examples
///
/// ```
/// use is_rs::types::is_null;
/// assert!(is_null(&Option::<i32>::None));
/// assert!(!is_null(&Some(1)));
/// ```
pub fn is_null<T>(value: &Option<T>) -> bool {
    value.is_none()
}

/// Returns `true` if the `Option` is `None` (Rust equivalent of is.js `is.undefined`).
///
/// Rust does not distinguish `null` from `undefined`, so both checks map to
/// `Option::None`.
///
/// # Examples
///
/// ```
/// use is_rs::types::is_undefined;
/// assert!(is_undefined(&Option::<i32>::None));
/// assert!(!is_undefined(&Some(1)));
/// ```
pub fn is_undefined<T>(value: &Option<T>) -> bool {
    value.is_none()
}

/// Returns `true` if both values have the same concrete type.
///
/// This mirrors `is.js` `sameType`, including the special rule that `NaN` only
/// matches another `NaN`.
///
/// # Examples
///
/// ```
/// use is_rs::types::is_same_type;
/// assert!(is_same_type(&42i32, &7i32));
/// assert!(!is_same_type(&42i32, &7u32));
/// assert!(is_same_type(&f64::NAN, &f64::NAN));
/// assert!(!is_same_type(&f64::NAN, &1.0f64));
/// ```
pub fn is_same_type<T: Any, U: Any>(value: &T, other: &U) -> bool {
    if TypeId::of::<T>() != TypeId::of::<U>() {
        return false;
    }

    let left = value as &dyn Any;
    let right = other as &dyn Any;

    if let (Some(a), Some(b)) = (left.downcast_ref::<f32>(), right.downcast_ref::<f32>()) {
        return !(a.is_nan() ^ b.is_nan());
    }

    if let (Some(a), Some(b)) = (left.downcast_ref::<f64>(), right.downcast_ref::<f64>()) {
        return !(a.is_nan() ^ b.is_nan());
    }

    true
}
