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
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! is-rs = "0.1"
//! ```

pub mod arithmetic;
pub mod array;
pub mod object;
pub mod presence;
pub mod regexp;
pub mod string;
pub mod time;
pub mod types;
pub mod is;

pub use is::{Is, Not, All, Any};

/// Global `Is` instance — the primary entry point for the object-style API.
///
/// ```
/// use is_rs::IS;
/// assert!(IS.even(4));
/// assert!(IS.not().even(3));
/// ```
pub static IS: Is = Is;
