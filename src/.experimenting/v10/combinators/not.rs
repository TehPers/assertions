use crate::{AssertionFailure, AssertionResult};

use super::AssertionCombinator;

/// Negates the output of an assertion. The overall assertion succeeds if and
/// only if the chained assertion fails.
#[derive(Clone, Debug)]
pub struct NotCombinator<Inner> {
    inner: Inner,
}

impl<Inner> NotCombinator<Inner> {
    /// Creates a new instance of this combinator wrapping the inner combinator.
    #[inline]
    pub fn new(inner: Inner) -> Self {
        Self { inner }
    }
}

impl<Inner> AssertionCombinator for NotCombinator<Inner>
where
    Inner: AssertionCombinator,
{
    type Target = Inner::Target;
    type Result = Inner::Result;

    #[inline]
    fn execute<F>(self, mut f: F) -> Self::Result
    where
        F: FnMut(Self::Target) -> AssertionResult,
    {
        self.inner.execute(|value| match f(value) {
            Ok(_) => Err(AssertionFailure::builder().build("TODO")),
            Err(_) => Ok(()),
        })
    }
}
