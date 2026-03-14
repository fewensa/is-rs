# is-rs

A Rust port of [is.js](https://github.com/arasatasaygin/is.js) — a micro check library for common type, value, and format predicates.

## Installation

```toml
[dependencies]
is-rs = "0.1"
```

## API overview

Two equivalent styles are available. Pick whichever fits your codebase.

### Object-style API — `IS` (mirrors `is.X / is.not.X / is.all.X / is.any.X`)

```rust
use is_rs::IS;

IS.even(4);                          // direct check
IS.not().even(3);                    // negated
IS.all().even(&[2, 4, 6]);           // all items pass
IS.any().odd(&[2, 3, 4]);            // at least one passes
```

### Module-level API

```rust
use is_rs::arithmetic::is_even;
use is_rs::regexp::is_email;

is_even(4);
is_email("user@example.com");
```

---

## Arithmetic

Mirrors `is.even`, `is.odd`, `is.positive`, `is.negative`, `is.above`, `is.under`, `is.within`, `is.decimal`, `is.integer`, `is.finite`, `is.infinite`, `is.equal`.

```rust
use is_rs::IS;
use is_rs::arithmetic::*;

// is.even / is.odd
assert!(IS.even(4));
assert!(IS.odd(3));
assert!(IS.not().even(3));
assert!(IS.all().even(&[2, 4, 6]));
assert!(IS.any().odd(&[2, 3, 4]));

// is.positive / is.negative
assert!(IS.positive(1.0));
assert!(IS.negative(-1.0));
assert!(IS.all().positive(&[1.0, 2.0, 3.0]));

// is.above / is.under / is.within
assert!(IS.above(5.0, 3.0));        // n > min
assert!(IS.under(3.0, 5.0));        // n < max
assert!(IS.within(3.0, 1.0, 5.0)); // min < n < max
assert!(IS.not().above(1.0, 5.0));

// is.decimal / is.integer
assert!(IS.decimal(1.5));
assert!(IS.integer(2.0));

// is.finite / is.infinite
assert!(IS.finite(1.0));
assert!(IS.infinite(f64::INFINITY));

// is.equal
assert!(IS.equal(3, 3));
assert!(IS.not().equal(1, 2));

// Module-level equivalents
assert!(is_even(4));
assert!(is_positive(1.0));
assert!(is_above(5.0, 3.0));
assert!(is_within(3.0, 1.0, 5.0));
assert!(is_decimal(1.5));
assert!(is_finite(1.0));
```

---

## Array

Mirrors `is.inArray`, `is.sorted`.

```rust
use is_rs::IS;
use is_rs::array::*;

// is.inArray
assert!(IS.in_array(&3, &[1, 2, 3]));
assert!(IS.not().in_array(&5, &[1, 2, 3]));

// is.sorted
assert!(IS.sorted(&[1, 2, 3]));
assert!(IS.not().sorted(&[3, 1, 2]));

// Module-level
assert!(in_array(&3, &[1, 2, 3]));
assert!(is_sorted(&[1, 2, 3]));
```

---

## Object

Mirrors `is.propertyCount`, `is.propertyDefined`.

```rust
use std::collections::HashMap;
use is_rs::IS;
use is_rs::object::*;

let mut m: HashMap<&str, i32> = HashMap::new();
m.insert("name", 1);
m.insert("age",  2);

// is.propertyCount
assert!(IS.property_count(&m, 2));
assert!(IS.not().property_count(&m, 99));

// is.propertyDefined
assert!(IS.property_defined(&m, &"name"));
assert!(IS.not().property_defined(&m, &"missing"));

// Module-level
assert!(property_count(&m, 2));
assert!(property_defined(&m, &"name"));
```

---

## Presence

Mirrors `is.empty`, `is.existy`, `is.truthy`, `is.falsy`, `is.space`.

```rust
use is_rs::IS;
use is_rs::presence::*;

// is.empty — two variants: string and slice
assert!(IS.empty_str(""));
assert!(IS.not().empty_str("hi"));
assert!(IS.all().empty_str(&["", ""]));

assert!(IS.empty_slice::<i32>(&[]));
assert!(IS.not().empty_slice(&[1]));

// is.existy (Option)
assert!(IS.existy_opt(&Some(42)));
assert!(IS.not().existy_opt::<i32>(&None));

// is.truthy / is.falsy
assert!(is_truthy(true));
assert!(is_falsy(false));

// is.space
assert!(IS.space("   "));
assert!(IS.not().space("hi"));
assert!(IS.all().space(&["  ", "\t"]));

// Module-level
assert!(is_empty_str(""));
assert!(is_empty_slice::<i32>(&[]));
assert!(is_existy(&Some(1)));
assert!(is_space("  "));
```

---

## Regexp

Mirrors `is.url`, `is.email`, `is.creditCard`, `is.alphaNumeric`, `is.timeString`, `is.dateString`, `is.usZipCode`, `is.caPostalCode`, `is.ukPostCode`, `is.nanpPhone`, `is.eppPhone`, `is.socialSecurityNumber`, `is.affirmative`, `is.hexadecimal`, `is.hexColor`, `is.ip`, `is.ipv4`, `is.ipv6`.

```rust
use is_rs::IS;
use is_rs::regexp::*;

// is.url — accepts http/https/ftp and scheme-less; rejects private IPs
assert!(IS.url("https://example.com"));
assert!(IS.url("example.com"));
assert!(IS.not().url("not a url"));
assert!(IS.all().url(&["https://a.com", "b.org"]));

// is.email
assert!(IS.email("user@example.com"));
assert!(IS.email("customer/department=shipping@example.com"));
assert!(IS.not().email("bad"));

// is.creditCard
assert!(is_credit_card("4111111111111111")); // Visa test number

// is.alphaNumeric
assert!(is_alpha_numeric("abc123"));
assert!(!is_alpha_numeric("abc 123"));

// is.timeString (HH:MM:SS)
assert!(is_time_string("13:45:00"));
assert!(!is_time_string("25:00:00"));

// is.dateString (M/D/YYYY or M-D-YYYY, consistent separator)
assert!(is_date_string("12/25/2023"));
assert!(is_date_string("12-25-2023"));
assert!(!is_date_string("12/25-2023")); // mixed separators rejected

// is.usZipCode
assert!(is_us_zip_code("90210"));
assert!(is_us_zip_code("90210-1234"));

// is.caPostalCode
assert!(is_ca_postal_code("K1A 0B1"));
assert!(!is_ca_postal_code("D1A 0B1")); // D is invalid in FSA

// is.ukPostCode
assert!(is_uk_post_code("SW1A 1AA"));

// is.nanpPhone (North American Number Plan)
assert!(is_nanp_phone("(201) 555-0123"));

// is.eppPhone (Extensible Provisioning Protocol)
assert!(is_epp_phone("+1.2015550123"));

// is.socialSecurityNumber — with or without hyphens
assert!(is_social_security_number("123-45-6789"));
assert!(is_social_security_number("123456789"));

// is.affirmative
assert!(is_affirmative("yes"));
assert!(is_affirmative("true"));
assert!(is_affirmative("1"));

// is.hexadecimal — with or without 0x prefix
assert!(is_hexadecimal("deadbeef"));
assert!(is_hexadecimal("0xdeadbeef"));

// is.hexColor — with or without #
assert!(is_hex_color("#ff0000"));
assert!(is_hex_color("ff0000"));
assert!(is_hex_color("#fff"));

// is.ip / is.ipv4 / is.ipv6
assert!(IS.ip("192.168.1.1"));
assert!(IS.ipv4("192.168.1.1"));
assert!(IS.ipv6("::1"));
assert!(IS.not().ipv4("::1"));
assert!(IS.all().ipv4(&["1.2.3.4", "5.6.7.8"]));
```

---

## String

Mirrors `is.upperCase`, `is.lowerCase`, `is.capitalized`, `is.palindrome`, `is.include`, `is.startWith`, `is.endWith`.

```rust
use is_rs::IS;
use is_rs::string::*;

// is.upperCase / is.lowerCase
assert!(IS.upper_case("HELLO"));
assert!(IS.lower_case("hello"));
assert!(IS.not().upper_case("Hello"));
assert!(IS.all().upper_case(&["ABC", "XYZ"]));

// is.capitalized
assert!(IS.capitalized("Hello"));
assert!(IS.not().capitalized("hello"));

// is.palindrome
assert!(IS.palindrome("racecar"));
assert!(IS.not().palindrome("hello"));

// is.include
assert!(IS.includes("hello world", "world"));
assert!(IS.not().includes("hello", "xyz"));

// is.startWith / is.endWith
assert!(IS.starts_with("foobar", "foo"));
assert!(IS.ends_with("foobar", "bar"));
assert!(IS.not().starts_with("foobar", "baz"));

// Module-level
assert!(is_upper_case("HELLO"));
assert!(is_palindrome("racecar"));
assert!(includes("foobar", "oba"));
assert!(starts_with("foobar", "foo"));
assert!(ends_with("foobar", "bar"));
```

---

## Time

Mirrors `is.today`, `is.yesterday`, `is.tomorrow`, `is.past`, `is.future`, `is.day`, `is.month`, `is.year`, `is.leapYear`, `is.weekday`, `is.weekend`, `is.inDateRange`, `is.inLastWeek`, `is.inLastMonth`, `is.inLastYear`, `is.inNextWeek`, `is.inNextMonth`, `is.inNextYear`, `is.quarterOfYear`, `is.dayLightSavingTime`.

```rust
use chrono::{Duration, Local, NaiveDate, TimeZone, Utc, Weekday};
use is_rs::IS;
use is_rs::time::*;

let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap(); // Saturday

// is.day / is.month / is.year
assert!(IS.day(&date, Weekday::Sat));
assert!(IS.month(&date, 6));
assert!(IS.year(&date, 2024));
assert!(IS.not().day(&date, Weekday::Mon));

// is.leapYear
assert!(IS.leap_year(2000));
assert!(IS.not().leap_year(1900));
assert!(IS.all().leap_year(&[2000, 2004, 2008]));

// is.weekday / is.weekend
let mon = NaiveDate::from_ymd_opt(2024, 6, 17).unwrap();
assert!(IS.weekday(&mon));
assert!(IS.weekend(&date));
assert!(IS.all().weekday(&[&mon]));

// is.past / is.future
let past   = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
let future = Utc::now() + Duration::days(365);
assert!(IS.past(&past));
assert!(IS.future(&future));
assert!(IS.any().future(&[&past, &future]));

// is.quarterOfYear (1–4)
assert!(IS.quarter_of_year(&date, 2));

// is.inDateRange
let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
let end   = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
assert!(IS.in_date_range(&date, &start, &end));

// is.dayLightSavingTime
let now_local = Local::now();
// result depends on your timezone
let _ = IS.daylight_saving_time(&now_local);

// Relative window helpers
let recent = Utc::now() - Duration::days(3);
assert!(IS.in_last_week(&recent));
assert!(IS.in_last_month(&recent));
assert!(IS.in_last_year(&recent));

// Module-level
assert!(is_leap_year(2024));
assert!(is_weekday(&mon));
assert!(is_weekend(&date));
assert!(quarter_of_year(&date, 2));
```

---

## Types

Mirrors `is.nan`, `is.number`, `is.integer` (type-level), `is.char`.

> **Note:** Rust's type system makes most of is.js's type checks (`is.string`, `is.boolean`,
> `is.object`, etc.) redundant — the compiler enforces those at compile time. The checks
> below are the runtime-meaningful subset.

```rust
use is_rs::IS;
use is_rs::types::*;

// is.nan
assert!(IS.nan(f64::NAN));
assert!(IS.not().nan(1.0));
assert!(IS.all().nan(&[f64::NAN, f64::NAN]));

// is.number (finite, non-NaN f64)
assert!(IS.number(3.14));
assert!(IS.not().number(f64::NAN));

// is.integer (whole-number f64)
assert!(IS.number(42.0));
assert!(IS.not().number(1.5));

// is.char (single Unicode scalar as &str)
assert!(IS.char("a"));
assert!(IS.char("€"));
assert!(IS.not().char("ab"));
assert!(IS.all().char(&["a", "b", "c"]));

// Module-level
assert!(is_nan(f64::NAN));
assert!(is_number(3.14));
assert!(is_integer(42.0));
assert!(is_char("a"));
```

---

## is.js compatibility

| is.js function | is-rs equivalent | Notes |
|---|---|---|
| `is.equal(a, b)` | `IS.equal(a, b)` | Generic `PartialEq` |
| `is.even(n)` | `IS.even(n)` | `i64` |
| `is.odd(n)` | `IS.odd(n)` | `i64` |
| `is.positive(n)` | `IS.positive(n)` | `f64` |
| `is.negative(n)` | `IS.negative(n)` | `f64` |
| `is.above(n, min)` | `IS.above(n, min)` | `f64` |
| `is.under(n, max)` | `IS.under(n, max)` | `f64` |
| `is.within(n, min, max)` | `IS.within(n, min, max)` | `f64` |
| `is.decimal(n)` | `IS.decimal(n)` | `f64` |
| `is.integer(n)` | `IS.integer(n)` / `types::is_integer` | Two variants: arithmetic (f64) and types (f64) |
| `is.finite(n)` | `IS.finite(n)` | `f64` |
| `is.infinite(n)` | `IS.infinite(n)` | `f64` |
| `is.nan(v)` | `IS.nan(v)` | `f64` |
| `is.number(v)` | `IS.number(v)` | `f64`, finite & non-NaN |
| `is.inArray(v, arr)` | `IS.in_array(&v, arr)` | Generic `PartialEq` |
| `is.sorted(arr)` | `IS.sorted(arr)` | Generic `PartialOrd` slice |
| `is.propertyCount(obj, n)` | `IS.property_count(&map, n)` | `HashMap` |
| `is.propertyDefined(obj, k)` | `IS.property_defined(&map, &k)` | `HashMap` |
| `is.empty(v)` | `IS.empty_str(s)` / `IS.empty_slice(s)` | Split by type |
| `is.existy(v)` | `IS.existy_opt(&opt)` | `Option<T>` |
| `is.truthy(v)` | `presence::is_truthy(b)` | `bool` |
| `is.falsy(v)` | `presence::is_falsy(b)` | `bool` |
| `is.space(s)` | `IS.space(s)` | `&str` |
| `is.url(s)` | `IS.url(s)` | Scheme optional, private IPs rejected |
| `is.email(s)` | `IS.email(s)` | Full RFC-style regex |
| `is.creditCard(s)` | `is_credit_card(s)` | |
| `is.alphaNumeric(s)` | `is_alpha_numeric(s)` | |
| `is.timeString(s)` | `is_time_string(s)` | `HH:MM:SS` |
| `is.dateString(s)` | `is_date_string(s)` | `M/D/YY[YY]` or `M-D-YY[YY]` |
| `is.usZipCode(s)` | `is_us_zip_code(s)` | |
| `is.caPostalCode(s)` | `is_ca_postal_code(s)` | |
| `is.ukPostCode(s)` | `is_uk_post_code(s)` | |
| `is.nanpPhone(s)` | `is_nanp_phone(s)` | |
| `is.eppPhone(s)` | `is_epp_phone(s)` | |
| `is.socialSecurityNumber(s)` | `is_social_security_number(s)` | Hyphenated or bare |
| `is.affirmative(s)` | `is_affirmative(s)` | |
| `is.hexadecimal(s)` | `is_hexadecimal(s)` | `0x` prefix accepted |
| `is.hexColor(s)` | `is_hex_color(s)` | `#` optional |
| `is.ip(s)` | `IS.ip(s)` | |
| `is.ipv4(s)` | `IS.ipv4(s)` | |
| `is.ipv6(s)` | `IS.ipv6(s)` | |
| `is.upperCase(s)` | `IS.upper_case(s)` | |
| `is.lowerCase(s)` | `IS.lower_case(s)` | |
| `is.capitalized(s)` | `IS.capitalized(s)` | |
| `is.palindrome(s)` | `IS.palindrome(s)` | |
| `is.include(s, sub)` | `IS.includes(s, sub)` | |
| `is.startWith(s, sub)` | `IS.starts_with(s, sub)` | |
| `is.endWith(s, sub)` | `IS.ends_with(s, sub)` | |
| `is.char(s)` | `IS.char(s)` | `&str` (single codepoint) |
| `is.today(dt)` | `IS.today(&date)` | `NaiveDate` |
| `is.yesterday(dt)` | `IS.yesterday(&date)` | `NaiveDate` |
| `is.tomorrow(dt)` | `IS.tomorrow(&date)` | `NaiveDate` |
| `is.past(dt)` | `IS.past(&dt)` | `DateTime<Utc>` |
| `is.future(dt)` | `IS.future(&dt)` | `DateTime<Utc>` |
| `is.day(dt, day)` | `IS.day(&date, Weekday::Mon)` | `NaiveDate` + `chrono::Weekday` |
| `is.month(dt, m)` | `IS.month(&date, 6)` | `u32` 1–12 |
| `is.year(dt, y)` | `IS.year(&date, 2024)` | `i32` |
| `is.leapYear(y)` | `IS.leap_year(y)` | `i64` |
| `is.weekday(dt)` | `IS.weekday(&date)` | |
| `is.weekend(dt)` | `IS.weekend(&date)` | |
| `is.inDateRange(dt, s, e)` | `IS.in_date_range(&dt, &s, &e)` | `[start, end)` |
| `is.inLastWeek(dt)` | `IS.in_last_week(&dt)` | `DateTime<Utc>` |
| `is.inLastMonth(dt)` | `IS.in_last_month(&dt)` | |
| `is.inLastYear(dt)` | `IS.in_last_year(&dt)` | |
| `is.inNextWeek(dt)` | `IS.in_next_week(&dt)` | |
| `is.inNextMonth(dt)` | `IS.in_next_month(&dt)` | |
| `is.inNextYear(dt)` | `IS.in_next_year(&dt)` | |
| `is.quarterOfYear(dt, q)` | `IS.quarter_of_year(&date, q)` | `u8` 1–4 |
| `is.dayLightSavingTime(dt)` | `IS.daylight_saving_time(&dt)` | `DateTime<Local>` |
| `is.not.X(...)` | `IS.not().X(...)` | |
| `is.all.X(arr)` | `IS.all().X(&[...])` | Slice of values |
| `is.any.X(arr)` | `IS.any().X(&[...])` | Slice of values |
| `is.string(v)` | — | Compile-time in Rust |
| `is.boolean(v)` | — | Compile-time in Rust |
| `is.object(v)` | — | Compile-time in Rust |
| `is.array(v)` | — | Compile-time in Rust |
| `is.date(v)` | — | Compile-time in Rust |
| `is.error(v)` | — | Compile-time in Rust |
| `is.function(v)` | — | Compile-time in Rust |
| `is.undefined(v)` | — | Compile-time in Rust (`Option`) |
| `is.null(v)` | — | Compile-time in Rust (`Option`) |
| `is.sameType(a, b)` | — | Compile-time in Rust |
| `is.arguments(v)` | — | No equivalent concept |
| `is.regexp(v)` | — | Compile-time in Rust |
| `is.json(v)` | — | Not yet implemented |
| `is.domNode(v)` | — | Not applicable (no DOM) |
| `is.windowObject(v)` | — | Not applicable |
| `is.thenable(v)` | — | Not applicable (Rust async differs) |
| `is.online()` | — | Not yet implemented |
| `is.mobile/tablet/desktop/...` | — | Not applicable (environment checks) |
| `is.mac/windows/linux/ios/android/...` | — | Not applicable |
| `is.chrome/firefox/safari/...` | — | Not applicable |

## License

MIT
