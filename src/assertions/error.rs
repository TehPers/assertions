use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use owo_colors::{OwoColorize, Stream, SupportsColorsDisplay};

use crate::assertions::ContextFrame;

use super::AssertionContext;

/// The foundational assertion output. Most assertions either output this type
/// directly, or output a type that wraps this type in some form.
///
/// Unlike a traditional [`Result`], this type includes additional context about
/// the execution path that led to a success or a failure. It can be converted
/// into a normal [`Result`] with [`into_result`](AssertionResult::into_result).
///
/// Note that not all assertions return this as their output (like asynchronous
/// assertions), but it is the preferred foundational output type for
/// assertions. It should be possible to eventually get a value of this type
/// from the output of an assertion by performing some commonly understood (or
/// clearly documented) set of operations on that output (like `.await`ing the
/// output).
#[derive(Clone, Debug)]
#[must_use]
pub struct AssertionResult {
    cx: AssertionContext,
    error: Option<String>,
}

impl AssertionResult {
    #[inline]
    pub(crate) fn new(cx: AssertionContext, error: Option<String>) -> Self {
        Self { cx, error }
    }

    /// Gets whether this output indicates a success.
    #[inline]
    #[must_use]
    pub fn is_pass(&self) -> bool {
        self.error.is_none()
    }

    /// Sets the state of this output to a pass. This overrides the context of
    /// the result.
    #[inline]
    pub(crate) fn set_pass(&mut self, mut new_cx: AssertionContext) {
        self.error = None;

        // Swap the context, but recover missing frames from the new context
        std::mem::swap(&mut self.cx, &mut new_cx);
        self.cx.recover(new_cx);
    }

    /// Sets the state of this output to a failure with the given message.
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub(crate) fn set_fail(&mut self, mut new_cx: AssertionContext, message: impl ToString) {
        self.error = Some(message.to_string());

        // Swap the context, but recover missing frames from the new context
        std::mem::swap(&mut self.cx, &mut new_cx);
        self.cx.recover(new_cx);
    }

    /// Converts this output into a [`Result`].
    #[inline]
    pub fn into_result(self) -> Result<(), AssertionError> {
        match self.error {
            Some(message) => Err(AssertionError::new(self.cx, message)),
            None => Ok(()),
        }
    }
}

/// An error that can occur during an assertion.
pub struct AssertionError {
    cx: AssertionContext,
    message: String,
}

impl AssertionError {
    pub(crate) fn new(cx: AssertionContext, message: String) -> Self {
        Self { cx, message }
    }
}

#[cfg(feature = "colors")]
fn colored<'a, T, U, F>(val: &'a T, apply: F) -> SupportsColorsDisplay<'a, T, U, F>
where
    T: OwoColorize,
    F: Fn(&'a T) -> U,
{
    val.if_supports_color(Stream::Stderr, apply)
}

impl Debug for AssertionError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        // TODO: keep colors?
        writeln!(f, "assertion failed:")?;
        writeln!(
            f,
            "  {}",
            colored(&format_args!("at: {}", self.cx.source_loc), |text| text
                .dimmed())
        )?;
        writeln!(
            f,
            "  {}",
            colored(&format_args!("subject: {}", self.cx.subject), |text| text
                .dimmed())
        )?;
        writeln!(f)?;

        fn write_frame(f: &mut Formatter, frame: &ContextFrame, comment: &str) -> std::fmt::Result {
            writeln!(f, "{} {}:{comment}", " ", frame.assertion_name)?;
            for (key, value) in &frame.annotations {
                #[cfg(feature = "colors")]
                writeln!(
                    f,
                    "    {}",
                    colored(&format_args!("{key}: {value}"), |text| text.dimmed())
                )?;
                #[cfg(not(feature = "colors"))]
                writeln!(f, "    {key}: {value}")?;
            }
            writeln!(f)?;
            Ok(())
        }

        // Write visited frames
        writeln!(f, "steps:")?;
        let mut idx = 0;
        for frame in self.cx.visited.iter() {
            let comment = if idx == self.cx.visited.len() - 1 {
                format!(" {}", colored(&self.message, |text| text.bright_red()))
            } else {
                String::new()
            };
            write_frame(f, frame, &comment)?;
            idx += 1;
        }

        // Write recovered frames
        for frame in &self.cx.recovered {
            write_frame(f, frame, "")?;
            idx += 1;
        }

        // Write non-visited frames
        for frame in &self.cx.remaining[self.cx.recovered.len()..] {
            writeln!(
                f,
                "  {frame}: {}",
                colored(&"(not visited)", |text| text.dimmed())
            )?;
            idx += 1;
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
