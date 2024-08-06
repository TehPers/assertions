use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use crate::metadata::SourceLoc;

use super::ContextFrame;

/// A simple assertion result.
///
/// Most assertions either output this type directly, or output a type that
/// wraps this type in some form.
///
/// Note that not all assertions return this as their output (like asynchronous
/// assertions), but it is the preferred foundational output type for
/// assertions, and it should be possible to eventually get a value of this type
/// from the output of an assertion by performing some commonly understood (or
/// clearly documented) set of operations on that output (like `.await`ing the
/// output).
pub type AssertionResult = Result<(), AssertionError>;

/// An error that can occur during an assertion.
pub struct AssertionError {
    message: String,
    source_loc: &'static SourceLoc,
    visited: Vec<ContextFrame>,
    remaining: &'static [&'static str],
}

impl AssertionError {
    pub(crate) fn new(
        message: String,
        source_loc: &'static SourceLoc,
        visited: Vec<ContextFrame>,
        remaining: &'static [&'static str],
    ) -> Self {
        Self {
            message,
            source_loc,
            visited,
            remaining,
        }
    }
}

impl Debug for AssertionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "assertion failed: {}", self.message)?;
        writeln!(f, "  at: {}", self.source_loc)?;

        // Write visited frames
        writeln!(f, "steps:")?;
        for frame in &self.visited {
            writeln!(f, "  {}:", frame.assertion_name)?;
            for (key, value) in &frame.annotations {
                writeln!(f, "    {}: {}", key, value)?;
            }
        }

        for frame in self.remaining {
            writeln!(f, "  {frame}: # not visited")?;
        }

        Ok(())
    }
}

impl Display for AssertionError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for AssertionError {}
