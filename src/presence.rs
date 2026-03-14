/// Returns `true` if the string is empty.
pub fn is_empty_str(s: &str) -> bool {
    s.is_empty()
}

/// Returns `true` if the slice is empty.
pub fn is_empty_slice<T>(v: &[T]) -> bool {
    v.is_empty()
}

/// Returns `true` if the `Option` contains a value (`Some`).
pub fn is_existy<T>(v: &Option<T>) -> bool {
    v.is_some()
}

/// Returns `true` if the boolean is `true`.
pub fn is_truthy(v: bool) -> bool {
    v
}

/// Returns `true` if the boolean is `false`.
pub fn is_falsy(v: bool) -> bool {
    !v
}

/// Returns `true` if the string consists entirely of whitespace characters.
///
/// An empty string returns `false` (no whitespace to speak of).
pub fn is_space(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_whitespace())
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
