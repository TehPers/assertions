//! Assertions and modifiers for tests that involve [`Result<T, E>`].

mod assertions;
mod extensions;
mod modifiers;
mod resultish;

pub use assertions::*;
pub use extensions::*;
pub use modifiers::*;
pub use resultish::*;
