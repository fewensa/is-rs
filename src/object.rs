use std::collections::HashMap;

/// Returns `true` if the map has exactly `count` entries.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use is_rs::object::property_count;
/// let mut m = HashMap::new();
/// m.insert("a", 1);
/// m.insert("b", 2);
/// assert!(property_count(&m, 2));
/// assert!(!property_count(&m, 3));
/// ```
pub fn property_count<K, V>(map: &HashMap<K, V>, count: usize) -> bool {
    map.len() == count
}

/// Returns `true` if `key` is present in the map.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use is_rs::object::property_defined;
/// let mut m = HashMap::new();
/// m.insert("name", "Alice");
/// assert!(property_defined(&m, &"name"));
/// assert!(!property_defined(&m, &"age"));
/// ```
pub fn property_defined<K: Eq + std::hash::Hash, V>(map: &HashMap<K, V>, key: &K) -> bool {
    map.contains_key(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_map() -> HashMap<&'static str, i32> {
        let mut m = HashMap::new();
        m.insert("a", 1);
        m.insert("b", 2);
        m
    }

    #[test]
    fn test_property_count_positive() {
        let m = make_map();
        assert!(property_count(&m, 2));
    }

    #[test]
    fn test_property_count_negative() {
        let m = make_map();
        assert!(!property_count(&m, 3));
        assert!(!property_count(&m, 0));
    }

    #[test]
    fn test_property_defined_positive() {
        let m = make_map();
        assert!(property_defined(&m, &"a"));
        assert!(property_defined(&m, &"b"));
    }

    #[test]
    fn test_property_defined_negative() {
        let m = make_map();
        assert!(!property_defined(&m, &"c"));
        assert!(!property_defined(&m, &"z"));
    }
}
