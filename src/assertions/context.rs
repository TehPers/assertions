use crate::metadata::{Annotated, AnnotatedKind, SourceLoc};

use super::AssertionResult;

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
/// paths, like when using [`all`](crate::prelude::all) or
/// [`any`](crate::prelude::any) to execute an assertion on several values.
/// Forked contexts do not affect each other, so adding an attribute to a forked
/// context or passing it into another assertion will not affect any of the
/// other contexts that were created.
#[derive(Clone, Debug)]
pub struct AssertionContext {
    pub(crate) subject: String,
    pub(crate) source_loc: &'static SourceLoc,
    pub(crate) visited: Vec<ContextFrame>,
    pub(crate) remaining: &'static [&'static str],
    pub(crate) recovered: Vec<ContextFrame>,
}

impl AssertionContext {
    #[doc(hidden)]
    #[must_use]
    pub fn __new(
        subject: String,
        source_loc: &'static SourceLoc,
        frames: &'static [&'static str],
    ) -> Self {
        Self {
            subject,
            source_loc,
            visited: vec![],
            remaining: frames,
            recovered: vec![],
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

    /// Adds an annotation to this frame if the provided annotated value has a
    /// [`Debug`] representation.
    ///
    /// Note that function parameters to modifiers and assertions *almost
    /// always* have a meaningful string representation. Those values should
    /// generally be recorded using [`annotate()`](Self::annotate()) instead.
    ///
    /// This method exists in case a value's stringified representation is not
    /// expected to be meaningful, and it is unknown whether that value
    /// implements [`Debug`]. For example, a value being passed from one
    /// modifier to the next is temporarily stored in a variable, which is then
    /// annotated. The name of the variable is not meaningful, so the annotated
    /// value only has a meaningful string representation if the value
    /// implements [`Debug`].
    ///
    /// [`Debug`]: core::fmt::Debug
    #[inline]
    pub fn try_annotate<T>(&mut self, key: &'static str, value: &Annotated<T>) {
        if value.kind() == AnnotatedKind::Debug {
            self.annotate(key, value.as_str());
        }
    }

    /// Creates a new success value.
    #[inline]
    pub fn pass(self) -> AssertionResult {
        AssertionResult::new(self, None)
    }

    /// Creates a new success value based on a condition. Otherwise, create a
    /// new failure value.
    ///
    /// This is a convenience function for turning a boolean into either a pass
    /// or a fail.
    #[inline]
    pub fn pass_if(self, pass: bool, failure_message: impl ToString) -> AssertionResult {
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
    pub fn fail(self, message: impl ToString) -> AssertionResult {
        AssertionResult::new(self, Some(message.to_string()))
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
        });
        self.remaining = remaining;

        // New execution path, so recovered frames aren't relevant anymore
        self.recovered.clear();

        self
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ContextFrame {
    pub assertion_name: &'static str,
    pub annotations: Vec<(&'static str, String)>,
}
