//! Build complex, self-describing assertions by chaining together reusable methods.
//! Supports both synchronous and asynchronous assertions.
//!
//! ## Example
//!
//! ```
//! use expecters::prelude::*;
//!
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() {
//! expect!(1, as_display, to_equal("1"));
//! expect!(1..=5, count, to_equal(5));
//! expect!(
//!     [get_cat_url(0), get_cat_url(5)],
//!     all,
//!     when_ready,
//!     to_contain_substr(".png"),
//! ).await;
//! # }
//!
//! async fn get_cat_url(id: u32) -> String {
//!     format!("cats/{id}.png")
//! }
//! ```
//!
//! If your test fails, knowing why it failed is important. Unlike many other
//! assertions libraries, failures don't generate long expectation strings.
//! Instead, your assertion is broken down into its steps, and information is
//! attached to those steps to help you see what went wrong:
//!
//! ```should_panic
//! # use expecters::prelude::*;
//! expect!([1, 2, 3], all, to_satisfy(|n| n % 2 == 1));
//! ```
//!
//! This produces an error like the following:
//!
//! ```text
//! assertion failed:
//!   at: src\lib.rs:42:8 [your_lib::tests]
//!   subject: [1, 2, 3]
//!
//! steps:
//!   all:
//!     received: [1, 2, 3]
//!     index: 1
//!
//!   to_satisfy: did not satisfy predicate
//!     received: 2
//!     predicate: |n| n % 2 == 1
//! ```
//!
//! See the [`expect!`] macro's documentation for usage information. For a full
//! list of modifiers and assertions, look at the [`prelude`] module.
//!
//! ## Crate features
//!
//! Many of the assertions require certain crate features to be enabled. Default
//! features are marked with an asterisk (*) and can be disabled with
//! `default-features = false`:
//!
//! - `futures`*: Enables async assertions.
//! - `regex`*: Enables assertions that use regular expressions. Uses
//!   [regex](https://crates.io/crates/regex) to execute them.
//! - `colors`*: Enables styled failure messages. Styled messages can always be
//!   disabled by setting `NO_COLOR`.
#![warn(
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    clippy::all,
    clippy::pedantic,
    clippy::style
)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

pub mod assertions;
pub mod metadata;
pub mod prelude;
#[doc(hidden)]
pub mod specialization;

mod macros;

pub use assertions::AssertionOutput;
