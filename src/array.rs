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
