/// Returns `true` if `item` is present in `arr`.
///
/// # Examples
///
/// ```
/// use is_rs::array::in_array;
/// assert!(in_array(&3, &[1, 2, 3, 4]));
/// assert!(!in_array(&5, &[1, 2, 3, 4]));
/// ```
pub fn in_array<T: PartialEq>(item: &T, arr: &[T]) -> bool {
    arr.contains(item)
}

/// Returns `true` if `arr` is sorted in non-decreasing order.
///
/// An empty or single-element slice is considered sorted.
///
/// # Examples
///
/// ```
/// use is_rs::array::is_sorted;
/// assert!(is_sorted(&[1, 2, 3, 4]));
/// assert!(is_sorted::<i32>(&[]));
/// assert!(!is_sorted(&[3, 1, 2]));
/// ```
pub fn is_sorted<T: PartialOrd>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_array_positive() {
        assert!(in_array(&3, &[1, 2, 3, 4]));
        assert!(in_array(&"hello", &["hello", "world"]));
    }

    #[test]
    fn test_in_array_negative() {
        assert!(!in_array(&5, &[1, 2, 3, 4]));
        assert!(!in_array(&"foo", &["hello", "world"]));
    }

    #[test]
    fn test_is_sorted_positive() {
        assert!(is_sorted(&[1, 2, 3, 4, 5]));
        assert!(is_sorted(&[1, 1, 2, 3]));
        assert!(is_sorted::<i32>(&[]));
        assert!(is_sorted(&[42]));
    }

    #[test]
    fn test_is_sorted_negative() {
        assert!(!is_sorted(&[3, 1, 2]));
        assert!(!is_sorted(&[5, 4, 3, 2, 1]));
    }
}
