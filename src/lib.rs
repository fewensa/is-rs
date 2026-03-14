//! # is-rs
//!
//! A Rust port of [is.js](https://is.js.org/) — a micro check library.
//!
//! Provides predicate functions for arithmetic, string, presence, array, object,
//! regular-expression, time, and type checks.
//!
//! ## Usage
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! is-rs = "0.1"
//! ```
//!
//! Then import the module you need:
//!
//! ```
//! use is_rs::arithmetic::is_even;
//! assert!(is_even(4));
//! ```

pub mod arithmetic;
pub mod array;
pub mod object;
pub mod presence;
pub mod regexp;
pub mod string;
pub mod time;
pub mod types;
