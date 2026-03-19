//! # is-rs
//!
//! A Rust port of [is.js](https://is.js.org/) — a micro check library.
//!
//! ## API
//!
//! Two usage styles are supported:
//!
//! ### Functional (module-level)
//!
//! ```
//! use is_rs::arithmetic::is_even;
//! assert!(is_even(4));
//! ```
//!
//! ### Object-style (`Is / Is::not() / Is::all() / Is::any()`)
//!
//! Mirrors the `is.X / is.not.X / is.all.X / is.any.X` pattern from is.js.
//!
//! ```
//! use is_rs::IS;
//!
//! assert!(IS.even(4));
//! assert!(IS.not().even(3));
//! assert!(IS.all().even(&[2, 4, 6]));
//! assert!(IS.any().odd(&[2, 3, 4]));
//! ```
//!
//! ## Unified numeric input (`Num` trait)
//!
//! All arithmetic functions accept any type that implements [`arithmetic::Num`]:
//! `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`,
//! `u128`, `usize`, `f32`, `f64`, and `&str` (parsed as `f64`).
//!
//! This means you can pass any numeric primitive — or a string representation —
//! without explicit casts:
//!
//! ```
//! use is_rs::arithmetic::{is_even, is_positive, is_above};
//!
//! assert!(is_even(4i32));
//! assert!(is_even(4u64));
//! assert!(is_positive(1i32));
//! assert!(is_above(5u8, 3u8));
//! assert!(is_even("4"));
//! assert!(is_positive("1.5"));
//! ```
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! is-rs = "0.1"
//! ```

pub mod arithmetic;
pub mod array;
pub mod is;
pub mod object;
pub mod presence;
pub mod regexp;
pub mod string;
pub mod time;
pub mod types;

pub use is::{All, Any, Is, Not};

/// Global `Is` instance — the primary entry point for the object-style API.
///
/// ```
/// use is_rs::IS;
/// assert!(IS.even(4));
/// assert!(IS.not().even(3));
/// ```
pub static IS: Is = Is;
