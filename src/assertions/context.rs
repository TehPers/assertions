use std::borrow::Cow;

use crate::metadata::SourceLoc;

use super::general::InitializableOutput;

/// Context that is passed through an assertion to track the full execution flow
/// that occurred.
///
/// This stores information needed to provide meaningful error messages on
/// failures. This type is used to generate success and failure values that are
/// returned from assertions, and to annotate steps within the execution flow
/// to provide additional context to failures.
///
/// Assertion contexts can be cloned to indicate a fork in an execution path.
/// Cloning the context allows the context to be passed down several execution
/// paths, like when using [`all`] or [`any`] to execute an assertion on several
/// values.
///
/// Forked contexts do not affect each other, so adding an attribute to a forked
/// context or passing it into another assertion will not affect any of the
/// other contexts that were created.
///
/// [`all`]: crate::prelude::IteratorAssertions::all
/// [`any`]: crate::prelude::IteratorAssertions::any
#[derive(Clone, Debug)]
pub struct AssertionContext {
    pub(crate) subject: String,
    pub(crate) source_loc: SourceLoc,
    pub(crate) visited: Vec<ContextFrame>,
    pub(crate) remaining: &'static [&'static str],
    pub(crate) recovered: Vec<ContextFrame>,
}

impl AssertionContext {
    #[doc(hidden)]
    #[must_use]
    pub fn __new(
        subject: String,
        source_loc: SourceLoc,
        frames: &'static [&'static str],
    ) -> AssertionContextBuilder {
        AssertionContextBuilder {
            inner: Self {
                subject,
                source_loc,
                visited: vec![],
                remaining: frames,
                recovered: vec![],
            },
        }
    }

    /// Adds an annotation to this frame. The annotation is added to failure
    /// messages to help the user understand what happened on the execution path
    /// that triggered the failure.
    ///
    /// ```
    /// use expecters::{
    ///     assertions::AssertionContext,
    ///     metadata::Annotated
    /// };
    ///
    /// fn execute_to_equal<T>(
    ///     mut cx: AssertionContext,
    ///     expected: Annotated<T>
    /// ) {
    ///     // this appears as 'expected: foo' in failures
    ///     cx.annotate("my other annotation", "foo");
    ///
    ///     // this appears as 'expected: <value>' in failures. note that
    ///     // annotated values always implement ToString and require no
    ///     // additional type bounds on T
    ///     cx.annotate("expected", &expected);
    /// }
    /// ```
    #[allow(clippy::needless_pass_by_value, clippy::missing_panics_doc)]
    pub fn annotate(&mut self, key: &'static str, value: impl ToString) {
        // self.next() must be called at least once before annotations can be
        // added, otherwise there will be no current frame
        self.visited
            .last_mut()
            .expect("no visited frames (this is a bug)")
            .annotations
            .push((key, value.to_string()));
    }

    /// Adds a page with additional details to this frame. This page appears in
    /// failure messages.
    ///
    /// Pages can provide detailed information about a particular failure. They
    /// are intended to be used where a short message is insufficient, and more
    /// control over the format of the output is desired.
    ///
    /// For example, a page can provide a diff between an expected value and the
    /// received subject.
    #[allow(clippy::needless_pass_by_value, clippy::missing_panics_doc)]
    pub fn add_page(&mut self, title: impl Into<Cow<'static, str>>, page: impl ToString) {
        self.visited
            .last_mut()
            .expect("no visited frames (this is a bug)")
            .pages
            .push((title.into(), page.to_string()));
    }

    /// Creates a new success value.
    #[inline]
    #[must_use]
    pub fn pass<O>(self) -> O
    where
        O: InitializableOutput,
    {
        O::pass(self)
    }

    /// Creates a new success value based on a condition. Otherwise, create a
    /// new failure value.
    ///
    /// This is a convenience function for turning a boolean into either a pass
    /// or a fail.
    #[inline]
    pub fn pass_if<O>(self, pass: bool, failure_message: impl ToString) -> O
    where
        O: InitializableOutput,
    {
        if pass {
            self.pass()
        } else {
            self.fail(failure_message)
        }
    }

    /// Creates a new error with the given error message. Context is attached
    /// to the error based on the context that was provided through the
    /// [`annotate()`](Self::annotate()) function.
    ///
    /// The full assertion chain and any context associated with the current
    /// execution path through that chain (like the index of the item within a
    /// parent list) is also recorded onto the error to aid with debugging.
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn fail<O>(self, message: impl ToString) -> O
    where
        O: InitializableOutput,
    {
        O::fail(self, message.to_string())
    }

    /// Gets the source location for the assertion. This is the file, line,
    /// column, and module name where the [`expect!`] macro was called.
    ///
    /// This will be included automatically in any failure messages, but can be
    /// useful for uniquely identifying an assertion if needed (within a single
    /// test run). If the source code changes though, then it's possible for
    /// this value to change as well. For example, an extra newline added before
    /// the call to [`expect!`] would change where this value points to.
    ///
    /// [`expect!`]: crate::expect!
    #[inline]
    #[must_use]
    pub fn source_location(&self) -> SourceLoc {
        self.source_loc
    }

    /// Recovers missing frames from another context.
    ///
    /// The recovered frames are used to provide additional information on what
    /// happened during an unsuccessful execution path, especially where part of
    /// that execution path was successful but became unsuccessful by an earlier
    /// modifier.
    pub(crate) fn recover(&mut self, mut other: AssertionContext) {
        self.recovered = other
            .visited
            .drain(self.visited.len()..)
            .chain(other.recovered)
            .collect();
    }

    /// Creates a child context from this assertion context. This indicates a
    /// step through an execution path.
    pub(crate) fn next(mut self) -> AssertionContext {
        let (next, remaining) = self
            .remaining
            .split_first()
            .expect("no more context (this is a bug)");
        self.visited.push(ContextFrame {
            assertion_name: next,
            annotations: vec![],
            pages: vec![],
        });
        self.remaining = remaining;

        // New execution path, so recovered frames aren't relevant anymore
        self.recovered.clear();

        self
    }
}

/// Prepares an [`AssertionContext`] for use within an assertion.
///
/// This is passed up through the chain of
/// [`AssertionModifier`](crate::assertions::AssertionModifier)s before the
/// context is built and passed back down through the constructed assertions.
#[derive(Clone, Debug)]
pub struct AssertionContextBuilder {
    pub(crate) inner: AssertionContext,
}

#[derive(Clone, Debug)]
pub(crate) struct ContextFrame {
    pub assertion_name: &'static str,
    pub annotations: Vec<(&'static str, String)>,
    pub pages: Vec<(Cow<'static, str>, String)>,
}
