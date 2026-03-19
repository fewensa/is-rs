//! Object-style API mirroring `is.X / is.not.X / is.all.X / is.any.X` from is.js.
//!
//! Use the global [`IS`](crate::IS) constant as the entry point:
//!
//! ```
//! use is_rs::IS;
//!
//! // Direct check
//! assert!(IS.even(4));
//!
//! // Negated
//! assert!(IS.not().even(3));
//!
//! // All items in a slice satisfy the predicate
//! assert!(IS.all().even(&[2, 4, 6]));
//!
//! // Any item in a slice satisfies the predicate
//! assert!(IS.any().odd(&[2, 3, 4]));
//! ```

use std::collections::HashMap;

use chrono::{DateTime, Local, NaiveDate, Utc, Weekday};

use crate::{arithmetic, arithmetic::Num, array, object, presence, regexp, string, time, types};

// ---------------------------------------------------------------------------
// Core structs
// ---------------------------------------------------------------------------

/// The primary check struct. Obtain via the global [`IS`](crate::IS) constant.
///
/// Call [`Is::not()`] to get [`Not`], [`Is::all()`] to get [`All`],
/// or [`Is::any()`] to get [`Any`].
#[derive(Clone, Copy, Debug, Default)]
pub struct Is;

/// Negated checks — returns `true` when the underlying predicate returns `false`.
///
/// Obtained via [`Is::not()`].
#[derive(Clone, Copy, Debug, Default)]
pub struct Not;

/// All-of checks — returns `true` when **all** elements in a slice satisfy the predicate.
///
/// Obtained via [`Is::all()`].
#[derive(Clone, Copy, Debug, Default)]
pub struct All;

/// Any-of checks — returns `true` when **at least one** element in a slice satisfies the predicate.
///
/// Obtained via [`Is::any()`].
#[derive(Clone, Copy, Debug, Default)]
pub struct Any;

// ---------------------------------------------------------------------------
// Is — access to Not / All / Any
// ---------------------------------------------------------------------------

impl Is {
    /// Returns a [`Not`] handle for negated checks.
    ///
    /// ```
    /// use is_rs::IS;
    /// assert!(IS.not().even(3));
    /// ```
    #[allow(clippy::should_implement_trait)]
    #[inline]
    pub fn not(self) -> Not {
        Not
    }

    /// Returns an [`All`] handle for all-of checks over a slice.
    ///
    /// ```
    /// use is_rs::IS;
    /// assert!(IS.all().even(&[2, 4, 6]));
    /// ```
    #[inline]
    pub fn all(self) -> All {
        All
    }

    /// Returns an [`Any`] handle for any-of checks over a slice.
    ///
    /// ```
    /// use is_rs::IS;
    /// assert!(IS.any().odd(&[1, 2, 4]));
    /// ```
    #[inline]
    pub fn any(self) -> Any {
        Any
    }
}

// ---------------------------------------------------------------------------
// Macro helpers
// ---------------------------------------------------------------------------

/// Generate `Is`, `Not`, `All`, `Any` methods for a single-argument predicate.
///
/// `$fn` is a path to a function that takes the argument by value.
/// `All` and `Any` receive a `&[$arg_ty]` slice.
macro_rules! impl_check {
    ($method:ident, $arg_ty:ty, $fn:path) => {
        impl Is {
            pub fn $method(&self, v: $arg_ty) -> bool {
                $fn(v)
            }
        }
        impl Not {
            pub fn $method(&self, v: $arg_ty) -> bool {
                !$fn(v)
            }
        }
        impl All {
            pub fn $method(&self, values: &[$arg_ty]) -> bool {
                values.iter().all(|v| $fn(*v))
            }
        }
        impl Any {
            pub fn $method(&self, values: &[$arg_ty]) -> bool {
                values.iter().any(|v| $fn(*v))
            }
        }
    };
    // Variant for types that need to be passed by reference inside the closure
    // (e.g. &str — Copy but the closure gives &&str)
    (ref $method:ident, $arg_ty:ty, $fn:path) => {
        impl Is {
            pub fn $method(&self, v: $arg_ty) -> bool {
                $fn(v)
            }
        }
        impl Not {
            pub fn $method(&self, v: $arg_ty) -> bool {
                !$fn(v)
            }
        }
        impl All {
            pub fn $method(&self, values: &[$arg_ty]) -> bool {
                values.iter().all(|v| $fn(v))
            }
        }
        impl Any {
            pub fn $method(&self, values: &[$arg_ty]) -> bool {
                values.iter().any(|v| $fn(v))
            }
        }
    };
    // Variant for single-argument functions that take any `Num` type.
    // `Is`, `Not`, `All`, and `Any` all accept any `N: Num` so integer / float /
    // `&str` literals work without explicit casts at the call site.
    (num $method:ident, $fn:path) => {
        impl Is {
            pub fn $method<N: Num>(&self, v: N) -> bool {
                $fn(v)
            }
        }
        impl Not {
            pub fn $method<N: Num>(&self, v: N) -> bool {
                !$fn(v)
            }
        }
        impl All {
            pub fn $method<N: Num>(&self, values: &[N]) -> bool {
                values.iter().all(|&v| $fn(v))
            }
        }
        impl Any {
            pub fn $method<N: Num>(&self, values: &[N]) -> bool {
                values.iter().any(|&v| $fn(v))
            }
        }
    };
}

// ---------------------------------------------------------------------------
// Arithmetic
// ---------------------------------------------------------------------------

impl Is {
    /// Returns `true` if `a == b`.
    ///
    /// ```
    /// use is_rs::IS;
    /// assert!(IS.equal(1, 1));
    /// assert!(IS.not().equal(1, 2));
    /// ```
    pub fn equal<T: PartialEq>(&self, a: T, b: T) -> bool {
        arithmetic::is_equal(a, b)
    }

    /// Returns `true` if `n > min`.
    ///
    /// Accepts any [`Num`] type (`i32`, `f64`, `&str`, …).
    ///
    /// ```
    /// use is_rs::IS;
    /// assert!(IS.above(5.0, 3.0));
    /// assert!(IS.above(5i32, 3i32));
    /// assert!(IS.not().above(3.0, 3.0));
    /// ```
    pub fn above<N: Num>(&self, n: N, min: N) -> bool {
        arithmetic::is_above(n, min)
    }

    /// Returns `true` if `n < max`.
    ///
    /// Accepts any [`Num`] type.
    pub fn under<N: Num>(&self, n: N, max: N) -> bool {
        arithmetic::is_under(n, max)
    }

    /// Returns `true` if `min < n < max`.
    ///
    /// Accepts any [`Num`] type.
    pub fn within<N: Num>(&self, n: N, min: N, max: N) -> bool {
        arithmetic::is_within(n, min, max)
    }
}

impl Not {
    pub fn equal<T: PartialEq>(&self, a: T, b: T) -> bool {
        !arithmetic::is_equal(a, b)
    }

    pub fn above<N: Num>(&self, n: N, min: N) -> bool {
        !arithmetic::is_above(n, min)
    }

    pub fn under<N: Num>(&self, n: N, max: N) -> bool {
        !arithmetic::is_under(n, max)
    }

    pub fn within<N: Num>(&self, n: N, min: N, max: N) -> bool {
        !arithmetic::is_within(n, min, max)
    }
}

// Single-arg arithmetic — all four interfaces (Is/Not/All/Any).
// The object-style API uses f64 as the concrete type so that numeric literals
// and all Num types (via as f64) work without extra ceremony at the call site.
impl_check!(num even, arithmetic::is_even);
impl_check!(num odd, arithmetic::is_odd);
impl_check!(num positive, arithmetic::is_positive);
impl_check!(num negative, arithmetic::is_negative);
impl_check!(num decimal, arithmetic::is_decimal);
impl_check!(num integer, arithmetic::is_integer);
impl_check!(num finite, arithmetic::is_finite);
impl_check!(num infinite, arithmetic::is_infinite);

// ---------------------------------------------------------------------------
// Array
// ---------------------------------------------------------------------------

impl Is {
    /// Returns `true` if `arr` is sorted in non-decreasing order.
    ///
    /// ```
    /// use is_rs::IS;
    /// assert!(IS.sorted(&[1, 2, 3]));
    /// assert!(IS.not().sorted(&[3, 1, 2]));
    /// ```
    pub fn sorted<T: PartialOrd>(&self, arr: &[T]) -> bool {
        array::is_sorted(arr)
    }

    /// Returns `true` if `item` is in `arr`.
    ///
    /// ```
    /// use is_rs::IS;
    /// assert!(IS.in_array(&3, &[1, 2, 3]));
    /// assert!(IS.not().in_array(&5, &[1, 2, 3]));
    /// ```
    pub fn in_array<T: PartialEq>(&self, item: &T, arr: &[T]) -> bool {
        array::in_array(item, arr)
    }
}

impl Not {
    pub fn sorted<T: PartialOrd>(&self, arr: &[T]) -> bool {
        !array::is_sorted(arr)
    }

    pub fn in_array<T: PartialEq>(&self, item: &T, arr: &[T]) -> bool {
        !array::in_array(item, arr)
    }
}

// ---------------------------------------------------------------------------
// Object (HashMap)
// ---------------------------------------------------------------------------

impl Is {
    /// Returns `true` if the map has exactly `count` entries.
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use is_rs::IS;
    /// let mut m = HashMap::new();
    /// m.insert("a", 1);
    /// assert!(IS.property_count(&m, 1));
    /// assert!(IS.not().property_count(&m, 2));
    /// ```
    pub fn property_count<K, V>(&self, map: &HashMap<K, V>, count: usize) -> bool {
        object::property_count(map, count)
    }

    /// Returns `true` if `key` is present in the map.
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use is_rs::IS;
    /// let mut m = HashMap::new();
    /// m.insert("name", "Alice");
    /// assert!(IS.property_defined(&m, &"name"));
    /// assert!(IS.not().property_defined(&m, &"age"));
    /// ```
    pub fn property_defined<K: Eq + std::hash::Hash, V>(
        &self,
        map: &HashMap<K, V>,
        key: &K,
    ) -> bool {
        object::property_defined(map, key)
    }
}

impl Not {
    pub fn property_count<K, V>(&self, map: &HashMap<K, V>, count: usize) -> bool {
        !object::property_count(map, count)
    }

    pub fn property_defined<K: Eq + std::hash::Hash, V>(
        &self,
        map: &HashMap<K, V>,
        key: &K,
    ) -> bool {
        !object::property_defined(map, key)
    }
}

// ---------------------------------------------------------------------------
// Presence
// ---------------------------------------------------------------------------

impl_check!(ref empty_str, &str, presence::is_empty_str);
impl_check!(ref space, &str, presence::is_space);

impl Is {
    /// Returns `true` if the value is empty.
    pub fn empty<T: presence::Empty>(&self, v: T) -> bool {
        presence::is_empty(v)
    }

    /// Returns `true` if the slice is empty.
    ///
    /// ```
    /// use is_rs::IS;
    /// assert!(IS.empty_slice::<i32>(&[]));
    /// assert!(IS.not().empty_slice(&[1, 2]));
    /// ```
    pub fn empty_slice<T>(&self, v: &[T]) -> bool {
        presence::is_empty_slice(v)
    }

    /// Returns `true` if the `Option` contains a value.
    ///
    /// ```
    /// use is_rs::IS;
    /// assert!(IS.existy_opt(&Some(42)));
    /// assert!(IS.not().existy_opt::<i32>(&None));
    /// ```
    pub fn existy_opt<T>(&self, v: &Option<T>) -> bool {
        presence::is_existy(v)
    }

    /// Returns `true` if the value is not nullish.
    pub fn existy<T: presence::Existy>(&self, v: T) -> bool {
        presence::is_existy(v)
    }

    /// Returns `true` if the value is truthy.
    pub fn truthy<T: presence::Truthy>(&self, v: T) -> bool {
        presence::is_truthy(v)
    }

    /// Returns `true` if the value is falsy.
    pub fn falsy<T: presence::Truthy>(&self, v: T) -> bool {
        presence::is_falsy(v)
    }
}

impl Not {
    pub fn empty<T: presence::Empty>(&self, v: T) -> bool {
        !presence::is_empty(v)
    }

    pub fn empty_slice<T>(&self, v: &[T]) -> bool {
        !presence::is_empty_slice(v)
    }

    pub fn existy_opt<T>(&self, v: &Option<T>) -> bool {
        !presence::is_existy(v)
    }

    pub fn existy<T: presence::Existy>(&self, v: T) -> bool {
        !presence::is_existy(v)
    }

    pub fn truthy<T: presence::Truthy>(&self, v: T) -> bool {
        !presence::is_truthy(v)
    }

    pub fn falsy<T: presence::Truthy>(&self, v: T) -> bool {
        !presence::is_falsy(v)
    }
}

impl All {
    pub fn empty<T: presence::Empty>(&self, values: &[T]) -> bool {
        values.iter().all(presence::is_empty)
    }

    pub fn existy<T: presence::Existy>(&self, values: &[T]) -> bool {
        values.iter().all(presence::is_existy)
    }

    pub fn truthy<T: presence::Truthy>(&self, values: &[T]) -> bool {
        values.iter().all(presence::is_truthy)
    }

    pub fn falsy<T: presence::Truthy>(&self, values: &[T]) -> bool {
        values.iter().all(presence::is_falsy)
    }
}

impl Any {
    pub fn empty<T: presence::Empty>(&self, values: &[T]) -> bool {
        values.iter().any(presence::is_empty)
    }

    pub fn existy<T: presence::Existy>(&self, values: &[T]) -> bool {
        values.iter().any(presence::is_existy)
    }

    pub fn truthy<T: presence::Truthy>(&self, values: &[T]) -> bool {
        values.iter().any(presence::is_truthy)
    }

    pub fn falsy<T: presence::Truthy>(&self, values: &[T]) -> bool {
        values.iter().any(presence::is_falsy)
    }
}

// ---------------------------------------------------------------------------
// Regexp
// ---------------------------------------------------------------------------

impl_check!(ref url, &str, regexp::is_url);
impl_check!(ref email, &str, regexp::is_email);
impl_check!(ref credit_card, &str, regexp::is_credit_card);
impl_check!(ref alpha_numeric, &str, regexp::is_alpha_numeric);
impl_check!(ref time_string, &str, regexp::is_time_string);
impl_check!(ref date_string, &str, regexp::is_date_string);
impl_check!(ref us_zip_code, &str, regexp::is_us_zip_code);
impl_check!(ref ca_postal_code, &str, regexp::is_ca_postal_code);
impl_check!(ref uk_post_code, &str, regexp::is_uk_post_code);
impl_check!(ref nanp_phone, &str, regexp::is_nanp_phone);
impl_check!(ref epp_phone, &str, regexp::is_epp_phone);
impl_check!(
    ref social_security_number,
    &str,
    regexp::is_social_security_number
);
impl_check!(ref affirmative, &str, regexp::is_affirmative);
impl_check!(ref hexadecimal, &str, regexp::is_hexadecimal);
impl_check!(ref hex_color, &str, regexp::is_hex_color);
impl_check!(ref ip, &str, regexp::is_ip);
impl_check!(ref ipv4, &str, regexp::is_ipv4);
impl_check!(ref ipv6, &str, regexp::is_ipv6);

// ---------------------------------------------------------------------------
// String
// ---------------------------------------------------------------------------

impl_check!(ref upper_case, &str, string::is_upper_case);
impl_check!(ref lower_case, &str, string::is_lower_case);
impl_check!(ref capitalized, &str, string::is_capitalized);
impl_check!(ref palindrome, &str, string::is_palindrome);

impl Is {
    /// Returns `true` if `s` contains `sub`.
    ///
    /// ```
    /// use is_rs::IS;
    /// assert!(IS.includes("hello world", "world"));
    /// assert!(IS.not().includes("hello", "xyz"));
    /// ```
    pub fn includes(&self, s: &str, sub: &str) -> bool {
        string::includes(s, sub)
    }

    /// Returns `true` if `s` starts with `sub`.
    pub fn starts_with(&self, s: &str, sub: &str) -> bool {
        string::starts_with(s, sub)
    }

    /// Returns `true` if `s` ends with `sub`.
    pub fn ends_with(&self, s: &str, sub: &str) -> bool {
        string::ends_with(s, sub)
    }
}

impl Not {
    pub fn includes(&self, s: &str, sub: &str) -> bool {
        !string::includes(s, sub)
    }

    pub fn starts_with(&self, s: &str, sub: &str) -> bool {
        !string::starts_with(s, sub)
    }

    pub fn ends_with(&self, s: &str, sub: &str) -> bool {
        !string::ends_with(s, sub)
    }
}

// ---------------------------------------------------------------------------
// Time
// ---------------------------------------------------------------------------

impl_check!(ref today, &NaiveDate, time::is_today);
impl_check!(ref yesterday, &NaiveDate, time::is_yesterday);
impl_check!(ref tomorrow, &NaiveDate, time::is_tomorrow);
impl_check!(ref weekday, &NaiveDate, time::is_weekday);
impl_check!(ref weekend, &NaiveDate, time::is_weekend);
impl_check!(ref past, &DateTime<Utc>, time::is_past);
impl_check!(ref future, &DateTime<Utc>, time::is_future);
impl_check!(ref in_last_week, &DateTime<Utc>, time::in_last_week);
impl_check!(ref in_last_month, &DateTime<Utc>, time::in_last_month);
impl_check!(ref in_last_year, &DateTime<Utc>, time::in_last_year);
impl_check!(ref in_next_week, &DateTime<Utc>, time::in_next_week);
impl_check!(ref in_next_month, &DateTime<Utc>, time::in_next_month);
impl_check!(ref in_next_year, &DateTime<Utc>, time::in_next_year);

// leap_year: takes i32 in the underlying fn, exposed as i64 for consistency
impl Is {
    /// Returns `true` if `year` is a leap year.
    ///
    /// ```
    /// use is_rs::IS;
    /// assert!(IS.leap_year(2000));
    /// assert!(!IS.leap_year(1900));
    /// assert!(IS.not().leap_year(2001));
    /// ```
    pub fn leap_year(&self, year: i64) -> bool {
        time::is_leap_year(year as i32)
    }
}
impl Not {
    pub fn leap_year(&self, year: i64) -> bool {
        !time::is_leap_year(year as i32)
    }
}
impl All {
    pub fn leap_year(&self, years: &[i64]) -> bool {
        years.iter().all(|&y| time::is_leap_year(y as i32))
    }
}
impl Any {
    pub fn leap_year(&self, years: &[i64]) -> bool {
        years.iter().any(|&y| time::is_leap_year(y as i32))
    }
}

impl Is {
    /// Returns `true` if `dt` is the given weekday.
    pub fn day(&self, dt: &NaiveDate, day: Weekday) -> bool {
        time::is_day(dt, day)
    }

    /// Returns `true` if `dt` is in the given month (1 = January … 12 = December).
    pub fn month(&self, dt: &NaiveDate, month: u32) -> bool {
        time::is_month(dt, month)
    }

    /// Returns `true` if `dt` is in the given year.
    pub fn year(&self, dt: &NaiveDate, year: i32) -> bool {
        time::is_year(dt, year)
    }

    /// Returns `true` if `dt` falls in the given quarter (1–4).
    pub fn quarter_of_year(&self, dt: &NaiveDate, quarter: u8) -> bool {
        time::quarter_of_year(dt, quarter)
    }

    /// Returns `true` if `dt` is in daylight saving time.
    pub fn daylight_saving_time(&self, dt: &DateTime<Local>) -> bool {
        time::is_daylight_saving_time(dt)
    }

    /// Returns `true` if `dt` falls within `[start, end)`.
    pub fn in_date_range(&self, dt: &NaiveDate, start: &NaiveDate, end: &NaiveDate) -> bool {
        time::in_date_range(dt, start, end)
    }
}

impl Not {
    pub fn day(&self, dt: &NaiveDate, day: Weekday) -> bool {
        !time::is_day(dt, day)
    }

    pub fn month(&self, dt: &NaiveDate, month: u32) -> bool {
        !time::is_month(dt, month)
    }

    pub fn year(&self, dt: &NaiveDate, year: i32) -> bool {
        !time::is_year(dt, year)
    }

    pub fn quarter_of_year(&self, dt: &NaiveDate, quarter: u8) -> bool {
        !time::quarter_of_year(dt, quarter)
    }

    pub fn daylight_saving_time(&self, dt: &DateTime<Local>) -> bool {
        !time::is_daylight_saving_time(dt)
    }

    pub fn in_date_range(&self, dt: &NaiveDate, start: &NaiveDate, end: &NaiveDate) -> bool {
        !time::in_date_range(dt, start, end)
    }
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

impl_check!(ref char, &str, types::is_char);
impl_check!(nan, f64, types::is_nan);
impl_check!(number, f64, types::is_number);

impl Is {
    pub fn null<T>(&self, value: &Option<T>) -> bool {
        types::is_null(value)
    }

    pub fn undefined<T>(&self, value: &Option<T>) -> bool {
        types::is_undefined(value)
    }

    pub fn same_type<T: 'static, U: 'static>(&self, value: &T, other: &U) -> bool {
        types::is_same_type(value, other)
    }
}

impl Not {
    pub fn null<T>(&self, value: &Option<T>) -> bool {
        !types::is_null(value)
    }

    pub fn undefined<T>(&self, value: &Option<T>) -> bool {
        !types::is_undefined(value)
    }

    pub fn same_type<T: 'static, U: 'static>(&self, value: &T, other: &U) -> bool {
        !types::is_same_type(value, other)
    }
}

impl All {
    pub fn null<T>(&self, values: &[Option<T>]) -> bool {
        values.iter().all(types::is_null)
    }

    pub fn undefined<T>(&self, values: &[Option<T>]) -> bool {
        values.iter().all(types::is_undefined)
    }
}

impl Any {
    pub fn null<T>(&self, values: &[Option<T>]) -> bool {
        values.iter().any(types::is_null)
    }

    pub fn undefined<T>(&self, values: &[Option<T>]) -> bool {
        values.iter().any(types::is_undefined)
    }
}
