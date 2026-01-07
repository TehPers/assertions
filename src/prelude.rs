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
        functions::{FunctionAssertions, SimpleFunctionAssertions},
        general::GeneralAssertions,
        iterators::IteratorAssertions,
        options::OptionAssertions,
        pointers::PointerAssertions,
        read::ReadExtensions,
        results::ResultAssertions,
        strings::{DebugAssertions, DisplayAssertions, StringAssertions},
    },
    expect, try_expect,
};

#[cfg(feature = "futures")]
pub use crate::assertions::{async_read::AsyncReadAssertions, futures::FutureAssertions};
