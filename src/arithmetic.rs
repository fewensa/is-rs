/// A unified numeric input type that can be constructed from any primitive number type
/// or parsed from a string.
///
/// This trait is the single entry point for all arithmetic checks in `is-rs`.
/// It lets callers pass `i32`, `i64`, `u32`, `u64`, `f32`, `f64`, or even `&str` /
/// `String` without explicit casts, while keeping a clean internal representation.
///
/// # Parsing from strings
///
/// When constructed from a `&str` or `String`, the value is parsed as `f64`.
/// If parsing fails the internal value is `f64::NAN`, which propagates naturally
/// through all downstream checks (e.g. `is_finite("abc")` → `false`).
///
/// # Examples
///
/// ```
/// use is_rs::arithmetic::{is_even, is_positive, is_above};
///
/// // integer types
/// assert!(is_even(4i32));
/// assert!(is_even(4u64));
/// assert!(is_even(4i64));
///
/// // float types
/// assert!(is_positive(1.0f32));
/// assert!(is_above(5u32, 3u32));
///
/// // strings
/// assert!(is_even("4"));
/// assert!(is_positive("1.5"));
/// assert!(!is_positive("abc")); // parse failure → NAN → false
/// ```
pub trait Num: Copy {
    /// Return the value as `f64` for comparisons that need fractional precision.
    fn as_f64(self) -> f64;
    /// Return the value rounded to `i64` for integer-parity checks.
    fn as_i64(self) -> i64 {
        self.as_f64() as i64
    }
}

macro_rules! impl_num_int {
    ($($t:ty),+) => {
        $(
            impl Num for $t {
                #[inline]
                fn as_f64(self) -> f64 { self as f64 }
                #[inline]
                fn as_i64(self) -> i64 { self as i64 }
            }
        )+
    };
}

macro_rules! impl_num_float {
    ($($t:ty),+) => {
        $(
            impl Num for $t {
                #[inline]
                fn as_f64(self) -> f64 { self as f64 }
            }
        )+
    };
}

impl_num_int!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
impl_num_float!(f32, f64);

impl Num for &str {
    #[inline]
    fn as_f64(self) -> f64 {
        self.trim().parse::<f64>().unwrap_or(f64::NAN)
    }
}

// `String` is not `Copy`, so callers should pass `s.as_str()` or `&s` instead.

#[inline]
fn finite_value<N: Num>(n: N) -> Option<f64> {
    let value = n.as_f64();
    value.is_finite().then_some(value)
}

#[inline]
fn finite_integer_value<N: Num>(n: N) -> Option<f64> {
    let value = finite_value(n)?;
    (value.fract() == 0.0).then_some(value)
}

/// Returns `true` if `a == b`.
///
/// # Examples
///
/// ```
/// use is_rs::arithmetic::is_equal;
/// assert!(is_equal(1, 1));
/// assert!(!is_equal(1, 2));
/// ```
pub fn is_equal<T: PartialEq>(a: T, b: T) -> bool {
    a == b
}

/// Returns `true` if `n` is even.
///
/// Accepts any type that implements [`Num`]: `i32`, `i64`, `u32`, `f64`, `&str`, etc.
///
/// # Examples
///
/// ```
/// use is_rs::arithmetic::is_even;
/// assert!(is_even(4i32));
/// assert!(is_even(4u64));
/// assert!(is_even(4.0f64));
/// assert!(is_even("4"));
/// assert!(!is_even(3i32));
/// ```
pub fn is_even<N: Num>(n: N) -> bool {
    finite_integer_value(n).is_some_and(|value| value.rem_euclid(2.0) == 0.0)
}

/// Returns `true` if `n` is odd.
///
/// Accepts any type that implements [`Num`]: `i32`, `i64`, `u32`, `f64`, `&str`, etc.
///
/// # Examples
///
/// ```
/// use is_rs::arithmetic::is_odd;
/// assert!(is_odd(3i32));
/// assert!(is_odd(3u8));
/// assert!(is_odd("3"));
/// assert!(!is_odd(4i32));
/// ```
pub fn is_odd<N: Num>(n: N) -> bool {
    finite_integer_value(n).is_some_and(|value| value.rem_euclid(2.0) == 1.0)
}

/// Returns `true` if `n > 0`.
///
/// Accepts any type that implements [`Num`]: `i32`, `i64`, `u32`, `f32`, `f64`, `&str`, etc.
///
/// # Examples
///
/// ```
/// use is_rs::arithmetic::is_positive;
/// assert!(is_positive(1i32));
/// assert!(is_positive(1.0f32));
/// assert!(is_positive("1.5"));
/// assert!(!is_positive(0i32));
/// assert!(!is_positive(-1i64));
/// ```
pub fn is_positive<N: Num>(n: N) -> bool {
    n.as_f64() > 0.0
}

/// Returns `true` if `n < 0`.
///
/// Accepts any type that implements [`Num`].
///
/// # Examples
///
/// ```
/// use is_rs::arithmetic::is_negative;
/// assert!(is_negative(-1i32));
/// assert!(is_negative("-1.5"));
/// assert!(!is_negative(0i32));
/// assert!(!is_negative(1i64));
/// ```
pub fn is_negative<N: Num>(n: N) -> bool {
    n.as_f64() < 0.0
}

/// Returns `true` if `n > min`.
///
/// Accepts any type that implements [`Num`].
///
/// # Examples
///
/// ```
/// use is_rs::arithmetic::is_above;
/// assert!(is_above(5i32, 3i32));
/// assert!(is_above(5.0f64, 3.0f64));
/// assert!(is_above("5", "3"));
/// assert!(!is_above(3i32, 3i32));
/// ```
pub fn is_above<N: Num>(n: N, min: N) -> bool {
    n.as_f64() > min.as_f64()
}

/// Returns `true` if `n < max`.
///
/// Accepts any type that implements [`Num`].
///
/// # Examples
///
/// ```
/// use is_rs::arithmetic::is_under;
/// assert!(is_under(3i32, 5i32));
/// assert!(is_under("3", "5"));
/// assert!(!is_under(5i32, 5i32));
/// ```
pub fn is_under<N: Num>(n: N, max: N) -> bool {
    n.as_f64() < max.as_f64()
}

/// Returns `true` if `min < n < max` (exclusive on both ends).
///
/// Accepts any type that implements [`Num`].
///
/// # Examples
///
/// ```
/// use is_rs::arithmetic::is_within;
/// assert!(is_within(5i32, 3i32, 10i32));
/// assert!(is_within("5", "3", "10"));
/// assert!(!is_within(3i32, 3i32, 10i32));
/// assert!(!is_within(10i32, 3i32, 10i32));
/// ```
pub fn is_within<N: Num>(n: N, min: N, max: N) -> bool {
    let v = n.as_f64();
    v > min.as_f64() && v < max.as_f64()
}

/// Returns `true` if `n` has a fractional component (i.e. is not a whole number).
///
/// Accepts any type that implements [`Num`].
///
/// # Examples
///
/// ```
/// use is_rs::arithmetic::is_decimal;
/// assert!(is_decimal(1.5f64));
/// assert!(is_decimal("1.5"));
/// assert!(!is_decimal(1i32));
/// assert!(!is_decimal("1.0"));
/// ```
pub fn is_decimal<N: Num>(n: N) -> bool {
    finite_value(n).is_some_and(|value| value.fract() != 0.0)
}

/// Returns `true` if `n` has no fractional component.
///
/// Accepts any type that implements [`Num`].
///
/// # Examples
///
/// ```
/// use is_rs::arithmetic::is_integer;
/// assert!(is_integer(1i32));
/// assert!(is_integer(1.0f64));
/// assert!(is_integer("2"));
/// assert!(!is_integer(1.5f64));
/// assert!(!is_integer("1.5"));
/// ```
pub fn is_integer<N: Num>(n: N) -> bool {
    finite_integer_value(n).is_some()
}

/// Returns `true` if `n` is finite (not NaN and not infinite).
///
/// Accepts any type that implements [`Num`].
///
/// # Examples
///
/// ```
/// use is_rs::arithmetic::is_finite;
/// assert!(is_finite(1i32));
/// assert!(is_finite(1.0f64));
/// assert!(!is_finite(f64::INFINITY));
/// assert!(!is_finite(f64::NAN));
/// ```
pub fn is_finite<N: Num>(n: N) -> bool {
    n.as_f64().is_finite()
}

/// Returns `true` if `n` is positive or negative infinity.
///
/// Accepts any type that implements [`Num`].
///
/// # Examples
///
/// ```
/// use is_rs::arithmetic::is_infinite;
/// assert!(is_infinite(f64::INFINITY));
/// assert!(is_infinite(f64::NEG_INFINITY));
/// assert!(!is_infinite(1i32));
/// assert!(!is_infinite(f64::NAN));
/// ```
pub fn is_infinite<N: Num>(n: N) -> bool {
    n.as_f64().is_infinite()
}
