use std::collections::{BTreeMap, HashMap};

/// Trait for values that can participate in `is.empty`.
pub trait Empty {
    fn is_empty(&self) -> bool;
}

/// Trait for values that can participate in `is.existy`.
pub trait Existy {
    fn is_existy(&self) -> bool;
}

/// Trait for values that can participate in `is.truthy` / `is.falsy`.
pub trait Truthy {
    fn is_truthy(&self) -> bool;
}

impl<T: Empty + ?Sized> Empty for &T {
    fn is_empty(&self) -> bool {
        (*self).is_empty()
    }
}

impl<T: Existy + ?Sized> Existy for &T {
    fn is_existy(&self) -> bool {
        (*self).is_existy()
    }
}

impl<T: Truthy + ?Sized> Truthy for &T {
    fn is_truthy(&self) -> bool {
        (*self).is_truthy()
    }
}

macro_rules! impl_always_existy {
    ($($t:ty),+ $(,)?) => {
        $(
            impl Existy for $t {
                fn is_existy(&self) -> bool {
                    true
                }
            }
        )+
    };
}

macro_rules! impl_numeric_truthy {
    ($($t:ty),+ $(,)?) => {
        $(
            impl Truthy for $t {
                fn is_truthy(&self) -> bool {
                    *self != 0 as $t
                }
            }
        )+
    };
}

impl Empty for str {
    fn is_empty(&self) -> bool {
        str::is_empty(self)
    }
}

impl Empty for String {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> Empty for [T] {
    fn is_empty(&self) -> bool {
        <[T]>::is_empty(self)
    }
}

impl<T, const N: usize> Empty for [T; N] {
    fn is_empty(&self) -> bool {
        N == 0
    }
}

impl<T> Empty for Vec<T> {
    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }
}

impl<K, V, S> Empty for HashMap<K, V, S> {
    fn is_empty(&self) -> bool {
        HashMap::is_empty(self)
    }
}

impl<K, V> Empty for BTreeMap<K, V> {
    fn is_empty(&self) -> bool {
        BTreeMap::is_empty(self)
    }
}

impl<T> Existy for Option<T> {
    fn is_existy(&self) -> bool {
        self.is_some()
    }
}

impl Truthy for bool {
    fn is_truthy(&self) -> bool {
        *self
    }
}

impl Truthy for str {
    fn is_truthy(&self) -> bool {
        !self.is_empty()
    }
}

impl Truthy for String {
    fn is_truthy(&self) -> bool {
        !self.is_empty()
    }
}

impl Truthy for f32 {
    fn is_truthy(&self) -> bool {
        self.is_finite() && *self != 0.0
    }
}

impl Truthy for f64 {
    fn is_truthy(&self) -> bool {
        self.is_finite() && *self != 0.0
    }
}

impl<T> Truthy for Option<T> {
    fn is_truthy(&self) -> bool {
        self.is_some()
    }
}

impl<T> Truthy for [T] {
    fn is_truthy(&self) -> bool {
        true
    }
}

impl<T, const N: usize> Truthy for [T; N] {
    fn is_truthy(&self) -> bool {
        true
    }
}

impl<T> Truthy for Vec<T> {
    fn is_truthy(&self) -> bool {
        true
    }
}

impl<K, V, S> Truthy for HashMap<K, V, S> {
    fn is_truthy(&self) -> bool {
        true
    }
}

impl<K, V> Truthy for BTreeMap<K, V> {
    fn is_truthy(&self) -> bool {
        true
    }
}

impl_numeric_truthy!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);
impl_always_existy!(
    str, String, bool, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64
);

impl<T> Existy for [T] {
    fn is_existy(&self) -> bool {
        true
    }
}

impl<T, const N: usize> Existy for [T; N] {
    fn is_existy(&self) -> bool {
        true
    }
}

impl<T> Existy for Vec<T> {
    fn is_existy(&self) -> bool {
        true
    }
}

impl<K, V, S> Existy for HashMap<K, V, S> {
    fn is_existy(&self) -> bool {
        true
    }
}

impl<K, V> Existy for BTreeMap<K, V> {
    fn is_existy(&self) -> bool {
        true
    }
}

/// Returns `true` if the value is empty.
///
/// Supports strings, slices, vectors, arrays, and map types.
///
/// # Examples
///
/// ```
/// use is_rs::presence::is_empty;
/// use std::collections::HashMap;
///
/// assert!(is_empty(""));
/// let empty: &[i32] = &[];
/// assert!(is_empty(empty));
///
/// let map: HashMap<&str, i32> = HashMap::new();
/// assert!(is_empty(&map));
/// ```
pub fn is_empty<T: Empty>(value: T) -> bool {
    value.is_empty()
}

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
    is_empty(s)
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
    is_empty(v)
}

/// Returns `true` if the map has no entries.
pub fn is_empty_map<K, V, S>(map: &HashMap<K, V, S>) -> bool {
    is_empty(map)
}

/// Returns `true` if the value is not nullish.
///
/// `Option::None` maps to JavaScript's `null` / `undefined`; all other supported
/// Rust values are existy.
///
/// # Examples
///
/// ```
/// use is_rs::presence::is_existy;
/// assert!(is_existy(&Some(42)));
/// assert!(!is_existy(&Option::<i32>::None));
/// assert!(is_existy("hello"));
/// ```
pub fn is_existy<T: Existy>(v: T) -> bool {
    v.is_existy()
}

/// Returns `true` if the value is truthy using JavaScript-style semantics.
///
/// Supported falsy values are `false`, `Option::None`, numeric zero, `NaN`, and
/// empty strings. Collections remain truthy, matching is.js object/array rules.
///
/// # Examples
///
/// ```
/// use is_rs::presence::is_truthy;
/// assert!(is_truthy(true));
/// assert!(is_truthy(1));
/// assert!(!is_truthy(0));
/// assert!(!is_truthy(""));
/// assert!(!is_truthy(&Option::<i32>::None));
/// ```
pub fn is_truthy<T: Truthy>(v: T) -> bool {
    v.is_truthy()
}

/// Returns `true` if the value is falsy using JavaScript-style semantics.
///
/// # Examples
///
/// ```
/// use is_rs::presence::is_falsy;
/// assert!(is_falsy(false));
/// assert!(is_falsy(0));
/// assert!(!is_falsy(1));
/// ```
pub fn is_falsy<T: Truthy>(v: T) -> bool {
    !is_truthy(v)
}

/// Returns `true` if the string contains exactly one whitespace character.
///
/// This mirrors `is.js` `is.space`, which is a character check rather than a
/// "string contains only whitespace" check.
///
/// # Examples
///
/// ```
/// use is_rs::presence::is_space;
/// assert!(is_space(" "));
/// assert!(is_space("\t"));
/// assert!(!is_space(""));
/// assert!(!is_space("  "));
/// assert!(!is_space("a"));
/// ```
pub fn is_space(s: &str) -> bool {
    let mut chars = s.chars();
    matches!(chars.next(), Some(ch) if ch.is_whitespace()) && chars.next().is_none()
}
