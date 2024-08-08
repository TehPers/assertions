//! TODO

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

pub use assertions::AssertionResult;
