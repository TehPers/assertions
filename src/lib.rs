//! Build complex, self-describing assertions by chaining together reusable
//! methods. Supports both synchronous and asynchronous assertions.
//!
//! ```sh
//! cargo add --dev expecters
//! ```
//!
//! ## Example
//!
//! ```
//! use expecters::prelude::*;
//!
#![cfg_attr(
    feature = "futures",
    doc = r#" # #[tokio::main(flavor = "current_thread")]"#,
    doc = " # async fn main() {"
)]
#![cfg_attr(not(feature = "futures"), doc = " # fn main() {")]
//! expect!(1, as_display, to_equal("1"));
//! expect!(1..=5, count, to_equal(5));
//! # #[cfg(feature = "futures")]
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
//! See the [`expect!`] macro's documentation for usage information.
//!
//! ## Built-in assertions
//!
//! There are plenty of built-in assertions to handle many different common
//! needs. Since modifiers can be combined with each other and assertions, most
//! cases using standard library types and traits should be covered.
//!
//! For creating custom assertions, see the [`assertions`] module.
//!
//! ### General
//!
//! These assertions apply to all types in general.
//!
//! | Assertion                          | Description          |
//! | ---------------------------------- | -------------------- |
//! | [`to_equal`]                       | `x == y`             |
//! | [`to_equal_approximately`]         | `\|x - y\| < d`      |
//! | [`to_be_greater_than`]             | `x > y`              |
//! | [`to_be_greater_than_or_equal_to`] | `x >= y`             |
//! | [`to_be_less_than`]                | `x < y`              |
//! | [`to_be_less_than_or_equal_to`]    | `x <= y`             |
//! | [`to_be_one_of`]                   | `x in [y1, y2, ...]` |
//! | [`to_satisfy`]                     | `f(x) == true`       |
//! | [`to_satisfy_with`]                | `f(x)` is Ok         |
//!
//! | Modifier   | Description    |
//! | ---------- | -------------- |
//! | [`not`]    | negates result |
//! | [`map`]    | maps subject   |
//!
//! [`to_equal`]: crate::assertions::general::GeneralAssertions::to_equal
//! [`to_equal_approximately`]: crate::assertions::general::GeneralAssertions::to_equal_approximately
//! [`to_be_greater_than`]: crate::assertions::general::GeneralAssertions::to_be_greater_than
//! [`to_be_greater_than_or_equal_to`]: crate::assertions::general::GeneralAssertions::to_be_greater_than_or_equal_to
//! [`to_be_less_than`]: crate::assertions::general::GeneralAssertions::to_be_less_than
//! [`to_be_less_than_or_equal_to`]: crate::assertions::general::GeneralAssertions::to_be_less_than_or_equal_to
//! [`to_be_one_of`]: crate::assertions::general::GeneralAssertions::to_be_one_of
//! [`to_satisfy`]: crate::assertions::general::GeneralAssertions::to_satisfy
//! [`to_satisfy_with`]: crate::assertions::general::GeneralAssertions::to_satisfy_with
//! [`not`]: crate::assertions::general::GeneralAssertions::not
//! [`map`]: crate::assertions::general::GeneralAssertions::map
//!
//! ### Options
//!
//! These assertions only apply to [`Option`]s.
//!
//! | Assertion      | Description |
//! | -------------- | ----------- |
//! | [`to_be_some`] | x is Some   |
//! | [`to_be_none`] | x is None   |
//!
//! | Modifier           | Description   |
//! | ------------------ | ------------- |
//! | [`to_be_some_and`] | extracts Some |
//!
//! [`to_be_some`]: crate::assertions::options::OptionAssertions::to_be_some
//! [`to_be_none`]: crate::assertions::options::OptionAssertions::to_be_none
//! [`to_be_some_and`]: crate::assertions::options::OptionAssertions::to_be_some_and
//!
//! ### Results
//!
//! These assertions only apply to [`Result`]s.
//!
//! | Assertion     | Description |
//! | ------------- | ----------- |
//! | [`to_be_ok`]  | x is Ok     |
//! | [`to_be_err`] | x is Err    |
//!
//! | Modifier          | Description  |
//! | ----------------- | ------------ |
//! | [`to_be_ok_and`]  | extracts Ok  |
//! | [`to_be_err_and`] | extracts Err |
//!
//! [`to_be_ok`]: crate::assertions::results::ResultAssertions::to_be_ok
//! [`to_be_err`]: crate::assertions::results::ResultAssertions::to_be_err
//! [`to_be_ok_and`]: crate::assertions::results::ResultAssertions::to_be_ok_and
//! [`to_be_err_and`]: crate::assertions::results::ResultAssertions::to_be_err_and
//!
//! ### Strings
//!
//! These assertions test strings and convert types into strings.
//!
//! | Assertion             | Description       | Requires feature |
//! | --------------------- | ----------------- | ---------------- |
//! | [`to_contain_substr`] | x contains y      |                  |
//! | [`to_start_with`]     | x starts with y   |                  |
//! | [`to_end_with`]       | x ends with y     |                  |
//! | [`to_match_regex`]    | x matches pattern | `regex`          |
//!
//! | Modifier       | Description                             |
//! | -------------- | --------------------------------------- |
//! | [`chars`]      | map subject to `char` sequence          |
//! | [`as_debug`]   | map subject to `Debug` representation   |
//! | [`as_display`] | map subject to `Display` representation |
//!
//! [`to_contain_substr`]: crate::assertions::strings::StringAssertions::to_contain_substr
//! [`to_start_with`]: crate::assertions::strings::StringAssertions::to_start_with
//! [`to_end_with`]: crate::assertions::strings::StringAssertions::to_end_with
//! [`to_match_regex`]: crate::assertions::strings::StringAssertions::to_match_regex
//! [`chars`]: crate::assertions::strings::StringAssertions::chars
//! [`as_debug`]: crate::assertions::strings::DebugAssertions::as_debug
//! [`as_display`]: crate::assertions::strings::DisplayAssertions::as_display
//!
//! ### Iterators
//!
//! These assertions apply to types implementing [`IntoIterator`].
//!
//! | Assertion              | Description                  |
//! | ---------------------- | ---------------------------- |
//! | [`to_contain`]         | x contains y                 |
//! | [`to_contain_exactly`] | x is sequentially equal to y |
//!
//! | Modifier    | Description                           |
//! | ----------- | ------------------------------------- |
//! | [`all`]     | each item satisfies assertion         |
//! | [`any`]     | at least one item satisfies assertion |
//! | [`count`]   | counts items                          |
//! | [`nth`]     | gets nth item                         |
//! | [`zip`]     | zips two iterators                    |
//! | [`as_utf8`] | parses as utf8                        |
//!
//! [`to_contain`]: crate::assertions::iterators::IteratorAssertions::to_contain
//! [`to_contain_exactly`]: crate::assertions::iterators::IteratorAssertions::to_contain_exactly
//! [`all`]: crate::assertions::iterators::IteratorAssertions::all
//! [`any`]: crate::assertions::iterators::IteratorAssertions::any
//! [`count`]: crate::assertions::iterators::IteratorAssertions::count
//! [`nth`]: crate::assertions::iterators::IteratorAssertions::nth
//! [`zip`]: crate::assertions::iterators::IteratorAssertions::zip
//! [`as_utf8`]: crate::assertions::iterators::IteratorAssertions::as_utf8
//!
//! ### Functions
//!
//! The assertions apply to [`FnOnce`] types (all functions).
//!
//! | Assertion         | Description        |
//! | ----------------- | ------------------ |
//! | [`to_panic`]      | `f()` panics       |
//! | [`to_panic_with`] | `f(..args)` panics |
//!
//! | Modifier             | Description       |
//! | -------------------- | ----------------- |
//! | [`when_called`]      | calls `f()`       |
//! | [`when_called_with`] | calls `f(..args)` |
//!
//! [`to_panic`]: crate::assertions::functions::SimpleFunctionAssertions::to_panic
//! [`to_panic_with`]: crate::assertions::functions::FunctionAssertions::to_panic_with
//! [`when_called`]: crate::assertions::functions::SimpleFunctionAssertions::when_called
//! [`when_called_with`]: crate::assertions::functions::FunctionAssertions::when_called_with
//!
//! ### Pointers
//!
//! These assertions and modifiers apply to pointer-like values, including
//! pointer types like `&T`, `Box<T>`, and `Arc<T>`.
//!
//! | Assertion       | Description     |
//! | --------------- | --------------- |
//! | [`to_be_null`]  | `x.is_null()`   |
//! | [`to_point_to`] | `ptr::eq(x, y)` |
//!
//! | Modifier   | Description           |
//! | ---------- | --------------------- |
//! | [`as_ptr`] | convert to `*const T` |
//!
//! [`to_be_null`]: crate::assertions::pointers::PointerAssertions::to_be_null
//! [`to_point_to`]: crate::assertions::pointers::PointerAssertions::to_point_to
//! [`as_ptr`]: crate::assertions::pointers::PointerAssertions::as_ptr
//!
//! ### Readers
//!
//! These assertions apply to types that can be read from.
//!
//! | Modifier            | Description                           | Requires feature |
//! | ------------------- | ------------------------------------- | ---------------- |
//! | [`when_read`]       | reads into byte buffer                |                  |
//! | [`when_read_async`] | asynchronously reads into byte buffer | `futures`        |
//!
//! [`when_read`]: crate::assertions::read::ReadAssertions::when_read
//! [`when_read_async`]: crate::assertions::async_read::AsyncReadAssertions::when_read_async
//!
//! ### Futures
//!
//! These assertions apply to types implementing [`IntoFuture`].
//!
//! | Modifier              | Description                          | Requires feature |
//! | --------------------- | ------------------------------------ | ---------------- |
//! | [`when_ready`]        | gets output                          | `futures`        |
//! | [`when_ready_before`] | gets output if it completes before y | `futures`        |
//! | [`when_ready_after`]  | gets output if it completes after y  | `futures`        |
//!
//! [`IntoFuture`]: std::future::IntoFuture
//! [`when_ready`]: crate::assertions::futures::FutureAssertions::when_ready
//! [`when_ready_before`]: crate::assertions::futures::FutureAssertions::when_ready_before
//! [`when_ready_after`]: crate::assertions::futures::FutureAssertions::when_ready_after
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
//! - `diff`*: Enables generating diffs for some assertions.

// This can't be in Cargo.toml or it'll require tests to also have docs
#![warn(missing_docs)]

pub mod assertions;
pub mod metadata;
pub mod prelude;
#[doc(hidden)]
pub mod specialization;

mod diff;
mod macros;
mod styles;

pub use assertions::AssertionOutput;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme_tests {
    // Runs doc tests on the readme
}
