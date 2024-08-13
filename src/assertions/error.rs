use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use crate::assertions::ContextFrame;

use super::AssertionContext;

/// The foundational assertion output. Most assertions either output this type
/// directly, or output a type that wraps this type in some form.
///
/// Unlike a traditional [`Result`], this type includes additional context about
/// the execution path that led to a success or a failure. It can be converted
/// into a normal [`Result`] with [`into_result`](AssertionOutput::into_result).
///
/// Note that not all assertions return this as their output (like asynchronous
/// assertions), but it is the preferred foundational output type for
/// assertions. It should be possible to eventually get a value of this type
/// from the output of an assertion by performing some commonly understood (or
/// clearly documented) set of operations on that output (like `.await`ing the
/// output).
#[derive(Clone, Debug)]
#[must_use]
pub struct AssertionOutput {
    cx: AssertionContext,
    error: Option<String>,
}

impl AssertionOutput {
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
#[must_use]
pub struct AssertionError {
    cx: Box<AssertionContext>,
    message: String,
}

impl AssertionError {
    #[inline]
    pub(crate) fn new(cx: AssertionContext, message: String) -> Self {
        Self {
            cx: Box::new(cx),
            message,
        }
    }
}

#[cfg(feature = "colors")]
mod styles {
    use std::fmt::Display;

    use owo_colors::{OwoColorize, Stream};

    #[inline]
    pub fn dimmed(s: &impl Display) -> impl Display + '_ {
        s.if_supports_color(Stream::Stderr, |s| s.dimmed())
    }

    #[inline]
    pub fn bright_red(s: &impl Display) -> impl Display + '_ {
        s.if_supports_color(Stream::Stderr, |s| s.bright_red())
    }
}

#[cfg(not(feature = "colors"))]
mod styles {
    #[inline]
    pub fn dimmed<T>(s: &T) -> &T {
        s
    }

    #[inline]
    pub fn bright_red<T>(s: &T) -> &T {
        s
    }
}

fn write_frame(f: &mut Formatter, frame: &ContextFrame, comment: &str) -> std::fmt::Result {
    writeln!(f, "  {}:{comment}", frame.assertion_name)?;
    for (key, value) in &frame.annotations {
        writeln!(f, "    {}", styles::dimmed(&format_args!("{key}: {value}")))?;
    }
    writeln!(f)?;
    Ok(())
}

impl Debug for AssertionError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "assertion failed:")?;
        writeln!(
            f,
            "  {}",
            styles::dimmed(&format_args!("at: {}", self.cx.source_loc)),
        )?;
        writeln!(
            f,
            "  {}",
            styles::dimmed(&format_args!("subject: {}", self.cx.subject)),
        )?;
        writeln!(f)?;

        // Write visited frames
        writeln!(f, "steps:")?;
        let mut idx = 0;
        for frame in &self.cx.visited {
            let comment = if idx == self.cx.visited.len() - 1 {
                format!(" {}", styles::bright_red(&self.message))
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
            writeln!(f, "  {frame}: {}", styles::dimmed(&"(not visited)"))?;
            idx += 1;
        }

        Ok(())
    }
}

impl Display for AssertionError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for AssertionError {}
