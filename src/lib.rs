#![forbid(unsafe_code)]

pub mod assertions;
pub mod metadata;
pub mod prelude;
#[doc(hidden)]
pub mod specialization;

mod macros;

// pub use expect::*;
pub use assertions::AssertionResult;
