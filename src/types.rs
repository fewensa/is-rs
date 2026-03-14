/// 检查字符串是否恰好包含一个 Unicode 标量值（is.js: is.char）。
pub fn is_char(s: &str) -> bool {
    let mut chars = s.chars();

    chars.next().is_some() && chars.next().is_none()
}

/// 检查 f64 是否为 NaN（is.js: is.nan）。
pub fn is_nan(v: f64) -> bool {
    v.is_nan()
}

/// 检查 f64 是否为有限数字（is.js: is.number）。
pub fn is_number(v: f64) -> bool {
    v.is_finite()
}

/// 检查 f64 是否为有限整数（is.js: is.integer）。
pub fn is_integer(v: f64) -> bool {
    v.is_finite() && v.fract() == 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_accepts_single_unicode_scalar() {
        assert!(is_char("a"));
        assert!(is_char("你"));
        assert!(!is_char(""));
        assert!(!is_char("ab"));
    }

    #[test]
    fn nan_only_matches_nan_values() {
        assert!(is_nan(f64::NAN));
        assert!(!is_nan(1.0));
    }

    #[test]
    fn number_requires_finite_values() {
        assert!(is_number(42.5));
        assert!(!is_number(f64::NAN));
        assert!(!is_number(f64::INFINITY));
    }

    #[test]
    fn integer_requires_finite_whole_numbers() {
        assert!(is_integer(42.0));
        assert!(is_integer(-0.0));
        assert!(!is_integer(42.5));
        assert!(!is_integer(f64::NEG_INFINITY));
    }
}
