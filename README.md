# is-rs

A Rust port of [is.js](https://github.com/arasatasaygin/is.js) — a micro check library for common type, value, and format predicates.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
is-rs = "0.1"
```

## Modules

| Module | Description |
|--------|-------------|
| `arithmetic` | Numeric checks: even/odd, positive/negative, integer, decimal, within range, etc. |
| `array` | Array checks: membership and sort-order. |
| `object` | Map checks: property existence and count. |
| `presence` | Emptiness, truthiness, and whitespace checks. |
| `regexp` | Format checks: email, URL, IP, phone, SSN, credit card, postal codes, etc. |
| `string` | String checks: case, palindrome, prefix/suffix, inclusion. |
| `time` | Date/time checks: today, past/future, weekday, leap year, DST, quarters, etc. |
| `types` | Type-level checks: number, integer, NaN, char. |

## Usage

### Object-style API (`IS`)

`is-rs` ships a global `IS` constant that mirrors is.js's `is.X / is.not.X / is.all.X / is.any.X` pattern:

```rust
use is_rs::IS;

// Direct check
assert!(IS.even(4));
assert!(IS.email("user@example.com"));

// Negated — IS.not().X(...)
assert!(IS.not().even(3));
assert!(IS.not().email("bad"));

// All items in a slice satisfy the predicate — IS.all().X(&[...])
assert!(IS.all().even(&[2, 4, 6]));
assert!(IS.all().email(&["a@b.com", "c@d.org"]));

// Any item in a slice satisfies the predicate — IS.any().X(&[...])
assert!(IS.any().odd(&[2, 3, 4]));
assert!(IS.any().url(&["bad", "https://ok.com"]));
```

Multi-argument predicates (`equal`, `above`, `under`, `within`, `in_array`, etc.) support
`IS.X(...)` and `IS.not().X(...)` but not `all`/`any` (since the slice semantics are ambiguous):

```rust
use is_rs::IS;

assert!(IS.equal(3, 3));
assert!(IS.not().equal(1, 2));
assert!(IS.above(5.0, 3.0));
assert!(IS.not().within(99.0, 1.0, 10.0));
```

### Module-level API

```rust
use is_rs::arithmetic::is_even;
use is_rs::regexp::is_email;
use is_rs::time::is_leap_year;

fn main() {
    assert!(is_even(4));
    assert!(is_email("user@example.com"));
    assert!(is_leap_year(2024));
}
```

### Arithmetic

```rust
use is_rs::arithmetic::*;

assert!(is_positive(1.0));
assert!(is_negative(-1.0));
assert!(is_integer(42.0));
assert!(is_decimal(3.14));
assert!(is_even(2.0));
assert!(is_odd(3.0));
assert!(is_within(5.0, 1.0, 10.0));
assert!(is_above(5.0, 3.0));
assert!(is_under(3.0, 5.0));
```

### Presence

```rust
use is_rs::presence::*;

assert!(is_empty_str(""));
assert!(is_empty_slice::<i32>(&[]));
assert!(is_truthy(1.0));
assert!(is_falsy(0.0));
assert!(is_space("   "));
```

### String

```rust
use is_rs::string::*;

assert!(is_upper_case("HELLO"));
assert!(is_lower_case("hello"));
assert!(is_capitalized("Hello"));
assert!(is_palindrome("racecar"));
assert!(starts_with("foobar", "foo"));
assert!(ends_with("foobar", "bar"));
assert!(includes("foobar", "oba"));
```

### Regexp

```rust
use is_rs::regexp::*;

assert!(is_email("user@example.com"));
assert!(is_url("https://example.com"));
assert!(is_ipv4("192.168.1.1"));
assert!(is_ipv6("::1"));
assert!(is_nanp_phone("(201) 555-0123"));
assert!(is_us_zip_code("90210"));
assert!(is_hex_color("#ff0000"));
assert!(is_social_security_number("123-45-6789"));
```

### Time

```rust
use chrono::{NaiveDate, Utc, TimeDelta};
use is_rs::time::*;

let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
assert!(is_month(&date, 6));
assert!(is_year(&date, 2024));
assert!(is_weekday(&date));          // Saturday? use is_weekend
assert!(is_leap_year(2024));
assert!(quarter_of_year(&date, 2));

let past = Utc::now() - TimeDelta::seconds(5);
assert!(is_past(&past));
```

### Types

```rust
use is_rs::types::*;

assert!(is_number(42.0));
assert!(is_integer(42.0));
assert!(is_nan(f64::NAN));
assert!(is_char('x'));
```

## License

MIT
