use crate::metadata::{Annotated, AnnotatedKind, SourceLoc};

use super::AssertionError;

/// Context that is passed through an assertion to track the full execution flow
/// that occurred.
///
/// This stores information needed to provide meaningful error messages on
/// failures. This type is used to generate success and failure values that are
/// returned from assertions, and to annotate steps within the execution flow
/// to provide additional context to failures.
#[derive(Clone, Debug)]
pub struct AssertionContext {
    source_loc: &'static SourceLoc,
    visited: Vec<ContextFrame>,
    remaining: &'static [&'static str],
}

impl AssertionContext {
    #[doc(hidden)]
    pub fn __new(source_loc: &'static SourceLoc, frames: &'static [&'static str]) -> Self {
        Self {
            source_loc,
            remaining: frames,
            visited: vec![],
        }
    }

    /// Adds an annotation to this frame.
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
    /// [`Debug`](core::fmt::Debug) representation.
    ///
    /// Note that function parameters to modifier functions and assertion
    /// functions (the functions the user actually calls) *almost always* have a
    /// meaningful string representation. Those values should generally be
    /// recorded using [`annotate()`](Self::annotate()) instead.
    #[inline]
    pub fn try_annotate<T>(&mut self, key: &'static str, value: &Annotated<T>) {
        match value.kind() {
            AnnotatedKind::Debug => self.annotate(key, value.as_str()),
            _ => {}
        }
    }

    /// Creates a new success value.
    #[inline]
    pub fn pass(&self) {
        // TODO: track success path? somehow recover frames when inverting a
        // success into a fail?
    }

    // TODO: recover missing frames from an error that was recovered from
    // pub fn recover(&mut self, error: AssertionError) {}
    // probably need another field for the recovered frames to allow context to
    // continue to be propagated. on propagation - lose the recovered frames so
    // they don't get mixed in with a different execution path and confuse the
    // user

    /// Creates a new error with the given error message. Context is attached
    /// to the error based on the context that was provided through the
    /// [`annotate()`](Self::annotate()) function.
    ///
    /// The full assertion chain and any context associated with the current
    /// execution path through that chain (like the index of the item within a
    /// parent list) is also recorded onto the error to aid with debugging.
    #[inline]
    pub fn fail(&self, message: impl ToString) -> AssertionError {
        AssertionError::new(
            message.to_string(),
            self.source_loc,
            self.visited.clone(),
            self.remaining,
        )
    }

    /// Creates a child context from this assertion context, finalizing this
    /// frame's annotations.
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
        self
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ContextFrame {
    pub assertion_name: &'static str,
    pub annotations: Vec<(&'static str, String)>,
}
