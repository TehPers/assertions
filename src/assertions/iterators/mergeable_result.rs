use crate::{assertions::AssertionContext, AssertionResult};

/// A type of result that can be collected from an iterator into a merged result
/// value.
///
/// This is the core of how modifiers like [`all`] and [`any`] work. Results
/// that implement this trait can be collected from an iterator into a new
/// result following one of two strategies:
///
/// - `all`: the merged result succeeds if none of the original results were
///   failures.
/// - `any`: the merged result succeeds if at least one of the original results
///   was a failure.
///
/// [`all`]: crate::prelude::all
/// [`any`]: crate::prelude::any
pub trait MergeableResult {
    /// The type of the merged result.
    type Merged;

    /// Merges an iterator of results. The output represents a success if and
    /// only if none of the constituent results failed.
    fn merge_all<I>(cx: AssertionContext, results: I) -> Self::Merged
    where
        I: IntoIterator<Item = Self>;

    /// Merges an iterator of results. The output represents a success if and
    /// only if at least one of the constituent results succeeded.
    fn merge_any<I>(cx: AssertionContext, results: I) -> Self::Merged
    where
        I: IntoIterator<Item = Self>;
}

impl MergeableResult for AssertionResult {
    type Merged = AssertionResult;

    #[inline]
    fn merge_all<I>(_cx: AssertionContext, results: I) -> Self::Merged
    where
        I: IntoIterator<Item = Self>,
    {
        results.into_iter().collect()
    }

    fn merge_any<I>(cx: AssertionContext, results: I) -> Self::Merged
    where
        I: IntoIterator<Item = Self>,
    {
        let mut error = cx.fail("no results");
        for result in results {
            match result {
                Ok(()) => return Ok(()),
                Err(e) => error = e,
            }
        }

        Err(error)
    }
}
