//! Build composable assertions with a functional API.
//!
//! ```
//! use expecters::prelude::*;
//! expect!([1, 2, 3]).all().not().to_equal(0);
//! ```
//!
//! This crate provides a set of combinators and assertions that can be used to
//! build complex assertions in a functional manner. The combinators are
//! designed to be chained together to form a pipeline that is applied to the
//! target value.
//!
//! The following built-in combinators are supported:
//!
//! - [`not`](Assertable::not): Invert the result of the chained assertion.
//! - [`map`](Assertable::map): Transform the target value before applying the
//!   chained assertion.
//! - [`all`](Assertable::all): Assert that all elements of an iterator satisfy
//!   the chained assertion.
//! - [`any`](Assertable::any): Assert that any element of an iterator satisfies
//!   the chained assertion.
//! - [`count`](Assertable::count): Assert that the number of elements in an
//!   iterator satisfies the chained assertion.
//! - [`nth`](Assertable::nth): Assert that a specific element in an iterator
//!   satisfies the chained assertion.
//! - [`to_be_some_and`](Assertable::to_be_some_and): Assert that the target
//!   value is `Some` and that the inner value satisfies the chained assertion.
//! - [`to_be_ok_and`](Assertable::to_be_ok_and): Assert that the target value
//!   is `Ok` and that the inner value satisfies the chained assertion.
//! - [`to_be_err_and`](Assertable::to_be_err_and): Assert that the target value
//!   is `Err` and that the inner value satisfies the chained assertion.
//! - [`when_called`](Assertable::when_called): Assert that the target function
//!   returns a value that satisfies the chained assertion.
//! - [`when_called_with`](Assertable::when_called_with): Assert that the target
//!   function returns a value that satisfies the chained assertion when called
//!   with the given arguments.
//!
//! These combinators can be chained together as needed. For example:
//!
//! ```
//! # use expecters::prelude::*;
//! expect!(i32::checked_add)
//!     .when_called_with((1, 2))
//!     .to_be_some_and()
//!     .to_equal(3);
//! expect!(i32::checked_add).when_called_with((i32::MAX, 1)).to_be_none();
//! ```
//!
//! In addition to these combinators, a set of built-in assertions are provided
//! that can be used to form the final assertion. For a full list of assertions,
//! see the [`Assertable`] trait.
//!
//! If you need the error from the assertion, you can use the [`as_result`]
//! method at the start of the chain to convert the assertion to a result:
//!
//! ```
//! # use expecters::prelude::*;
//! let result = expect!(42).as_result().to_be_less_than(10);
//! expect!(result).to_be_err();
//! ```
//!
//! Note that this crate does not support any kind of mocking or test harness
//! features. It is only intended to be used for writing assertions in tests.
//! Other crates, such as [`mockall`] and [`test-case`], can be used in
//! conjunction with this crate to enhance testing capabilities.
//!
//! [`as_result`]: ExpectationRoot::as_result
//! [`mockall`]: https://crates.io/crates/mockall
//! [`test-case`]: https://crates.io/crates/test-case

pub mod combinators;
pub mod combinators2;

mod extensions;

mod assertions;
mod error;
// mod expect;
mod root;
// mod v3;
// mod v4;
mod v5;
// pub mod v6;
pub mod v7;
mod v8;

pub use assertions::*;
pub use error::*;
// pub use expect::*;
pub use root::*;

/// Commonly used types and traits. Import this module to get everything you
/// need to start writing expectations.
pub mod prelude {
    // TODO: don't accidentally re-export the expect module
    pub use crate::{expect2, extensions::*, path, Assertable};
}

#[doc(hidden)]
pub mod specialization;
