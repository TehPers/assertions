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
            to_be_less_than_or_equal_to, to_equal, to_satisfy, to_satisfy_all, to_satisfy_any,
        },
        iterators::{all, any, count, nth},
        options::{to_be_none, to_be_some, to_be_some_and},
        results::{to_be_err, to_be_err_and, to_be_ok, to_be_ok_and},
        strings::{as_debug, as_display, to_contain_substr},
    },
    expect, try_expect,
};

#[cfg(feature = "futures")]
pub use crate::assertions::futures::when_ready;

#[cfg(feature = "regex")]
pub use crate::assertions::strings::to_match_regex;
