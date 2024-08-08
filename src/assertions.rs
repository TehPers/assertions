//! Assertions and modifiers that are used with [`expect!`], as well as any
//! types used to drive them.
//!
//! [`expect!`]: crate::expect!

// pub mod functions;
#[cfg(feature = "futures")]
pub mod futures;
pub mod general;
pub mod iterators;
pub mod options;
pub mod results;
pub mod strings;

mod assertion;
mod context;
mod error;

pub use assertion::*;
pub use context::*;
pub use error::*;
