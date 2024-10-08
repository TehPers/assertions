use crate::{assertions::AssertionContext, AssertionOutput};

/// A type of assertion output that can be collected from an iterator and merged
/// into a single output.
///
/// This is the core of how modifiers like [`all`] and [`any`] work. Outputs
/// that implement this trait can be collected from an iterator into a new
/// output following one of two [merge strategies](MergeStrategy):
///
/// - [`All`](MergeStrategy::All): the merged output succeeds if none of the
///   original outputs were failures.
/// - [`Any`](MergeStrategy::Any): the merged output succeeds if at least one of
///   the original outputs was a success.
///
/// Note that these are carefully worded to include definitions for empty
/// iterators. An empty iterator represents either a success (for `All`) or a
/// failure (for `Any`) depending on your merge strategy.
///
/// [`all`]: crate::prelude::IteratorAssertions::all
/// [`any`]: crate::prelude::IteratorAssertions::any
pub trait MergeableOutput {
    /// The type of the merged output.
    type Merged;

    /// Merges an iterator of assertion outputs into a single output.
    ///
    /// This method may choose to short-circuit, but it is not guaranteed. For
    /// example, while iterators of [`AssertionOutput`]s can be short-circuited
    /// since their success/failure status is already known, iterators over
    /// futures are unable to do the same since the status is not yet known.
    fn merge<I>(cx: AssertionContext, strategy: MergeStrategy, outputs: I) -> Self::Merged
    where
        I: IntoIterator<Item = Self>;
}

impl MergeableOutput for AssertionOutput {
    type Merged = AssertionOutput;

    #[inline]
    fn merge<I>(cx: AssertionContext, strategy: MergeStrategy, outputs: I) -> Self::Merged
    where
        I: IntoIterator<Item = Self>,
    {
        let mut result = cx.pass_if(strategy == MergeStrategy::All, "no outputs");
        for output in outputs {
            match (strategy, output.is_pass()) {
                (MergeStrategy::Any, true) | (MergeStrategy::All, false) => return output,
                _ => result = output,
            }
        }

        result
    }
}

/// A strategy for merging outputs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MergeStrategy {
    /// Merged output represents a success if and only if none of the original
    /// outputs represented a failure.
    ///
    /// On failure, the failure represents one or more of the original failures.
    All,

    /// Merged output represents a success if and only if at least one of the
    /// original outputs represented a success.
    ///
    /// On success, the success represents one or more of the original
    /// successes.
    Any,
}
