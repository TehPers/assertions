//! Common, general purpose assertions and modifiers.
//!
//! This module contains types, assertions, and modules that are used by many
//! different kinds of assertions. The exports from this module are likely to
//! be commonly used.
//!
//! The assertions and modifiers are re-exported in the crate's prelude, so glob
//! importing the prelude will import all the assertions and modifiers from this
//! module.

mod comparisons;
mod finalizable_result;
mod invertible_result;
mod modifiers;

pub use comparisons::*;
pub use finalizable_result::*;
pub use invertible_result::*;
pub use modifiers::*;
