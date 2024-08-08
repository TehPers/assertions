//! Assertions and modifiers for tests that involve [`Result<T, E>`].

mod assertions;
mod modifiers;
mod resultish;

pub use assertions::*;
pub use modifiers::*;
pub use resultish::*;
