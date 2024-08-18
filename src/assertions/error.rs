use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Write},
};

use crate::{assertions::ContextFrame, styles};

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
#[derive(Debug)]
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

fn write_frame(f: &mut Formatter, frame: &ContextFrame, comment: &str) -> std::fmt::Result {
    writeln!(f, "  {}:{comment}", frame.assertion_name)?;
    for (key, value) in &frame.annotations {
        writeln!(f, "    {}", styles::dimmed(&format_args!("{key}: {value}")))?;
    }
    writeln!(f)?;
    Ok(())
}

impl Display for AssertionError {
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

        // Write frames
        writeln!(f, "steps:")?;
        let mut idx = 0;
        let mut pages = Vec::new();
        let frames = self.cx.visited.iter().chain(self.cx.recovered.iter());
        for frame in frames {
            let mut comment_parts = Vec::new();

            // Additional pages
            if !frame.pages.is_empty() {
                // Track pages for later
                let mut related_pages = String::new();
                for page in &frame.pages {
                    let page_idx = pages.len() + 1;
                    if related_pages.is_empty() {
                        write!(related_pages, "{page_idx}")?;
                    } else {
                        write!(related_pages, ", {page_idx}")?;
                    }

                    pages.push(page);
                }

                // Write references to the comment
                comment_parts.push(styles::reference(&format!("[{related_pages}]")).to_string());
            }

            // Error message
            if idx == self.cx.visited.len() - 1 {
                comment_parts.push(styles::error(&self.message).to_string());
            }

            // Write frame
            let comment = if comment_parts.is_empty() {
                String::new()
            } else {
                format!(" {}", comment_parts.join(" "))
            };
            write_frame(f, frame, &comment)?;
            idx += 1;
        }

        // Write non-visited frames
        for frame in &self.cx.remaining[self.cx.recovered.len()..] {
            writeln!(f, "  {frame}: {}", styles::dimmed(&"(not visited)"))?;
            writeln!(f)?;
            idx += 1;
        }

        // Write context pages
        for (idx, (title, page)) in pages.into_iter().enumerate() {
            writeln!(
                f,
                "----- {title} {} -----",
                styles::reference(&format_args!("[{}]", idx + 1))
            )?;
            writeln!(f, "{page}")?;
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Error for AssertionError {}
