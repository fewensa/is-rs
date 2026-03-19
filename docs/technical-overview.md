# Technical Overview

## Goals

`is-rs` ports the core `is.js` predicate model into Rust while keeping the API
small and predictable:

- module-level predicates such as `arithmetic::is_even`
- object-style predicates through the global `IS` constant
- negated, all-of, and any-of dispatch via `IS.not()`, `IS.all()`, and `IS.any()`

The crate focuses on checks that have a meaningful Rust equivalent. Browser and
DOM environment checks from `is.js` are intentionally left out.

## Module layout

- `src/arithmetic.rs`: numeric predicates and the `Num` abstraction used by the
  arithmetic surface
- `src/array.rs`: slice membership and sortedness checks
- `src/object.rs`: `HashMap`-based property helpers
- `src/presence.rs`: emptiness, existy, truthy, falsy, and whitespace checks
- `src/regexp.rs`: regex-backed format validation ported from `is.js`
- `src/string.rs`: string predicates such as case, capitalization, and palindrome
- `src/time.rs`: calendar and relative-window checks using `chrono`
- `src/types.rs`: runtime type-oriented predicates such as `nan`, `number`,
  `null`, `undefined`, and `same_type`
- `src/is.rs`: the object-style facade that wires module predicates into
  `IS`, `IS.not()`, `IS.all()`, and `IS.any()`

## API model

The functional API is the source of truth. The object-style API is a thin
adapter layer that forwards into the module-level functions.

`src/is.rs` uses a small macro (`impl_check!`) to generate the repetitive
single-argument methods for:

- direct predicates on `IS`
- negated predicates on `IS.not()`
- homogenous slice aggregation on `IS.all()`
- homogenous slice aggregation on `IS.any()`

Multi-argument predicates such as `equal`, `above`, `within`, `includes`, and
`same_type` are implemented manually because they do not fit the single-argument
macro shape.

## Parity decisions

### Arithmetic

Arithmetic predicates accept any `Num` implementation. In addition to Rust
numeric primitives, `&str` is supported and parsed as `f64`.

Recent parity tightening:

- `even` and `odd` now reject fractional, `NaN`, and infinite inputs instead of
  truncating them
- `decimal` and `integer` now require finite numeric values

This keeps the ergonomic string support that existed in `is-rs`, while aligning
the predicate semantics more closely with `is.js`.

### Presence

`is.js` presence checks are more value-oriented than Rust’s type system usually
encourages, so `src/presence.rs` uses traits:

- `Empty`
- `Existy`
- `Truthy`

These traits let the crate support `is.empty`, `is.existy`, `is.truthy`, and
`is.falsy` across several Rust types without introducing a boxed dynamic value
representation.

Current mappings:

- `Option::None` represents the Rust equivalent of JS nullish values
- numeric zero and `NaN` are falsy
- empty strings are falsy
- collections remain truthy, matching JS object/array behavior
- `is.space` mirrors `is.js` and checks for a single whitespace character

### Strings

String predicates now follow `is.js` semantics more closely:

- `upper_case` / `lower_case` accept the empty string
- `capitalized` checks every non-empty word
- `palindrome` ignores non-alphanumeric characters and normalizes case

### Time

`is.js` uses exclusive bounds for `inDateRange`; `is-rs` now matches that rule.

The crate still uses Rust-native time types instead of a JS `Date` object:

- `NaiveDate` for calendar-date checks
- `DateTime<Utc>` for relative past/future windows
- `DateTime<Local>` for daylight-saving detection

## Unsupported `is.js` areas

Some `is.js` predicates do not map cleanly to a Rust library:

- DOM checks such as `domNode` and `windowObject`
- browser and platform detection such as `chrome`, `ios`, `desktop`
- JS-only runtime concepts such as `arguments` and `thenable`

Where a meaningful Rust equivalent exists, the crate prefers an explicit Rust
type-driven API instead of emulating JavaScript runtime tags.

## Testing strategy

The test suite is split by module under `tests/` and covers:

- direct functional predicates
- the object-style `IS` facade
- edge cases for regex and time predicates
- doctests embedded next to the implementation

`cargo test` therefore validates both the runtime behavior and the documented
examples in one pass.
