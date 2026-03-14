/// Integration tests for the object-style `IS.*` API.
///
/// Covers `Is` (direct), `Not` (negated), `All` (all-of slice), `Any` (any-of slice)
/// across all categories: arithmetic, array, object, presence, regexp, string, time, types.
use std::collections::HashMap;

use chrono::{Duration, NaiveDate, TimeZone, Utc};
use is_rs::IS;

// ---------------------------------------------------------------------------
// Arithmetic — Is
// ---------------------------------------------------------------------------

#[test]
fn is_even_odd() {
    assert!(IS.even(4));
    assert!(!IS.even(3));
    assert!(IS.odd(3));
    assert!(!IS.odd(4));
}

#[test]
fn is_positive_negative() {
    assert!(IS.positive(1.0));
    assert!(!IS.positive(-1.0));
    assert!(IS.negative(-1.0));
    assert!(!IS.negative(1.0));
}

#[test]
fn is_decimal_integer() {
    assert!(IS.decimal(1.5));
    assert!(!IS.decimal(2.0));
    assert!(IS.integer(2.0));
    assert!(!IS.integer(2.5));
}

#[test]
fn is_finite_infinite() {
    assert!(IS.finite(1.0));
    assert!(!IS.finite(f64::INFINITY));
    assert!(IS.infinite(f64::INFINITY));
    assert!(!IS.infinite(1.0));
}

#[test]
fn is_equal_above_under_within() {
    assert!(IS.equal(3, 3));
    assert!(!IS.equal(3, 4));
    assert!(IS.above(5.0, 3.0));
    assert!(!IS.above(3.0, 3.0));
    assert!(IS.under(2.0, 5.0));
    assert!(!IS.under(5.0, 5.0));
    assert!(IS.within(3.0, 1.0, 5.0));
    assert!(!IS.within(5.0, 1.0, 5.0));
}

// ---------------------------------------------------------------------------
// Arithmetic — Not
// ---------------------------------------------------------------------------

#[test]
fn not_even_odd() {
    assert!(IS.not().even(3));
    assert!(!IS.not().even(4));
    assert!(IS.not().odd(4));
}

#[test]
fn not_equal_above() {
    assert!(IS.not().equal(1, 2));
    assert!(!IS.not().equal(2, 2));
    assert!(IS.not().above(3.0, 5.0));
}

// ---------------------------------------------------------------------------
// Arithmetic — All / Any
// ---------------------------------------------------------------------------

#[test]
fn all_even() {
    assert!(IS.all().even(&[2, 4, 6]));
    assert!(!IS.all().even(&[2, 3, 6]));
}

#[test]
fn any_odd() {
    assert!(IS.any().odd(&[2, 3, 4]));
    assert!(!IS.any().odd(&[2, 4, 6]));
}

#[test]
fn all_positive() {
    assert!(IS.all().positive(&[1.0, 2.0, 3.0]));
    assert!(!IS.all().positive(&[1.0, -1.0]));
}

#[test]
fn any_negative() {
    assert!(IS.any().negative(&[1.0, -1.0]));
    assert!(!IS.any().negative(&[1.0, 2.0]));
}

// ---------------------------------------------------------------------------
// Array
// ---------------------------------------------------------------------------

#[test]
fn is_sorted() {
    assert!(IS.sorted(&[1, 2, 3]));
    assert!(!IS.sorted(&[3, 1, 2]));
    assert!(IS.not().sorted(&[3, 1, 2]));
}

#[test]
fn in_array() {
    assert!(IS.in_array(&3, &[1, 2, 3]));
    assert!(!IS.in_array(&5, &[1, 2, 3]));
    assert!(IS.not().in_array(&5, &[1, 2, 3]));
}

// ---------------------------------------------------------------------------
// Object (HashMap)
// ---------------------------------------------------------------------------

#[test]
fn property_count_and_defined() {
    let mut m: HashMap<&str, i32> = HashMap::new();
    m.insert("a", 1);
    m.insert("b", 2);

    assert!(IS.property_count(&m, 2));
    assert!(!IS.property_count(&m, 3));
    assert!(IS.not().property_count(&m, 99));

    assert!(IS.property_defined(&m, &"a"));
    assert!(!IS.property_defined(&m, &"z"));
    assert!(IS.not().property_defined(&m, &"z"));
}

// ---------------------------------------------------------------------------
// Presence
// ---------------------------------------------------------------------------

#[test]
fn is_empty_str_and_space() {
    assert!(IS.empty_str(""));
    assert!(!IS.empty_str("hi"));
    assert!(IS.not().empty_str("hi"));

    assert!(IS.space("   "));
    assert!(!IS.space("hi"));

    // All / Any for &str predicates
    assert!(IS.all().empty_str(&["", ""]));
    assert!(!IS.all().empty_str(&["", "x"]));
    assert!(IS.any().empty_str(&["x", ""]));
    assert!(!IS.any().empty_str(&["a", "b"]));
}

#[test]
fn is_empty_slice_and_existy_opt() {
    let empty: &[i32] = &[];
    assert!(IS.empty_slice::<i32>(empty));
    assert!(!IS.empty_slice(&[1, 2]));
    assert!(IS.not().empty_slice(&[1]));

    assert!(IS.existy_opt(&Some(42)));
    assert!(!IS.existy_opt::<i32>(&None));
    assert!(IS.not().existy_opt::<i32>(&None));
}

// ---------------------------------------------------------------------------
// Regexp
// ---------------------------------------------------------------------------

#[test]
fn is_url_email() {
    assert!(IS.url("http://example.com"));
    assert!(!IS.url("not-a-url"));
    assert!(IS.not().url("not-a-url"));

    assert!(IS.email("user@example.com"));
    assert!(!IS.email("not-an-email"));

    // All
    assert!(IS.all().email(&["a@b.com", "c@d.org"]));
    assert!(!IS.all().email(&["a@b.com", "bad"]));
    // Any
    assert!(IS.any().url(&["not", "http://ok.com"]));
    assert!(!IS.any().url(&["bad", "also-bad"]));
}

#[test]
fn is_ipv4_ipv6() {
    assert!(IS.ipv4("192.168.1.1"));
    assert!(!IS.ipv4("::1"));
    assert!(IS.ipv6("::1"));
    assert!(!IS.ipv6("192.168.1.1"));
}

#[test]
fn is_hex_color_hexadecimal() {
    assert!(IS.hex_color("#fff"));
    assert!(IS.hex_color("#aabbcc"));
    assert!(IS.hex_color("aabbcc")); // no leading # is also accepted
    assert!(!IS.hex_color("#gg0000"));

    assert!(IS.hexadecimal("deadbeef"));
    assert!(!IS.hexadecimal("xyz"));
}

// ---------------------------------------------------------------------------
// String
// ---------------------------------------------------------------------------

#[test]
fn is_upper_lower_capitalized_palindrome() {
    assert!(IS.upper_case("HELLO"));
    assert!(!IS.upper_case("Hello"));
    assert!(IS.lower_case("hello"));
    assert!(IS.capitalized("Hello"));
    assert!(!IS.capitalized("hello"));
    assert!(IS.palindrome("racecar"));
    assert!(!IS.palindrome("hello"));
}

#[test]
fn includes_starts_ends_with() {
    assert!(IS.includes("hello world", "world"));
    assert!(!IS.includes("hello", "xyz"));
    assert!(IS.not().includes("hello", "xyz"));

    assert!(IS.starts_with("hello", "he"));
    assert!(IS.not().starts_with("hello", "lo"));

    assert!(IS.ends_with("hello", "lo"));
    assert!(IS.not().ends_with("hello", "he"));
}

#[test]
fn all_any_upper_case() {
    assert!(IS.all().upper_case(&["ABC", "XYZ"]));
    assert!(!IS.all().upper_case(&["ABC", "abc"]));
    assert!(IS.any().lower_case(&["ABC", "abc"]));
    assert!(!IS.any().lower_case(&["ABC", "DEF"]));
}

// ---------------------------------------------------------------------------
// Time
// ---------------------------------------------------------------------------

#[test]
fn is_past_future() {
    let past = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let future = Utc::now() + Duration::days(365);

    assert!(IS.past(&past));
    assert!(!IS.past(&future));
    assert!(IS.not().past(&future));

    assert!(IS.future(&future));
    assert!(!IS.future(&past));
    assert!(IS.not().future(&past));

    // All / Any (ref variant — slices of references)
    let past2 = Utc.with_ymd_and_hms(1999, 6, 1, 0, 0, 0).unwrap();
    assert!(IS.all().past(&[&past, &past2]));
    assert!(IS.any().future(&[&past, &future]));
    assert!(!IS.any().future(&[&past, &past2]));
}

#[test]
fn is_leap_year() {
    assert!(IS.leap_year(2000));
    assert!(!IS.leap_year(1900));
    assert!(IS.not().leap_year(2001));

    assert!(IS.all().leap_year(&[2000, 2004, 2008]));
    assert!(!IS.all().leap_year(&[2000, 2001]));
    assert!(IS.any().leap_year(&[2001, 2004]));
    assert!(!IS.any().leap_year(&[2001, 2003]));
}

#[test]
fn is_weekday_weekend() {
    let mon = NaiveDate::from_ymd_opt(2026, 3, 9).unwrap(); // Monday
    let sat = NaiveDate::from_ymd_opt(2026, 3, 14).unwrap(); // Saturday

    assert!(IS.weekday(&mon));
    assert!(!IS.weekday(&sat));
    assert!(IS.weekend(&sat));
    assert!(!IS.weekend(&mon));

    assert!(IS.all().weekday(&[&mon]));
    assert!(!IS.all().weekday(&[&mon, &sat]));
    assert!(IS.any().weekend(&[&mon, &sat]));
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[test]
fn is_nan_number_char() {
    assert!(IS.nan(f64::NAN));
    assert!(!IS.nan(1.0));
    assert!(IS.not().nan(1.0));

    assert!(IS.number(3.14));
    assert!(!IS.number(f64::NAN));

    assert!(IS.char("a"));
    assert!(!IS.char("ab"));

    // All / Any
    assert!(IS.all().nan(&[f64::NAN, f64::NAN]));
    assert!(!IS.all().nan(&[f64::NAN, 1.0]));
    assert!(IS.any().number(&[f64::NAN, 1.0]));
    assert!(!IS.any().char(&["ab", "cd"]));
}
