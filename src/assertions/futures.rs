//! Modifiers used for asynchronous tests.
//!
//! This module contains types used primarily for testing asynchronous code. The
//! assertions created from the modifiers in this module are generally
//! asynchronous and need to be `.await`ed in order for them to execute.
//!
//! This module also contains types that can be useful for writing your own
//! asynchronous assertions and modifiers, if needed.
//!
//! ```
//! # use expecters::prelude::*;
//! use std::future::ready;
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() {
//! expect!(ready(1), when_ready, to_equal(1)).await;
//! # }
//! ```

mod extensions;
mod modifiers;
mod outputs;

pub use extensions::*;
pub use modifiers::*;
pub use outputs::*;
