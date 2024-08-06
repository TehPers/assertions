//! This module contains commonly used exports from this crate.
//!
//! To keep your imports simple, rather than importing these members
//! individually, you can write:
//!
//! ```
//! # #[allow(unused_imports)]
//! use expecters::prelude::*;
//! ```
//!
//! While not necessary, it is recommended to glob import this module in any
//! test modules that use this crate.

pub use crate::{
    assertions::{
        general::{
            map, not, to_be_greater_than, to_be_greater_than_or_equal_to, to_be_less_than,
            to_be_less_than_or_equal_to, to_equal,
        },
        iterators::{all, any, count, nth},
    },
    expect,
};

#[cfg(feature = "futures")]
pub use crate::assertions::futures::when_ready;
