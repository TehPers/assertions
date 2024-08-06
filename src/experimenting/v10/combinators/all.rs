use crate::AssertionResult;

use super::AssertionCombinator;

/// Wraps another [`AssertionCombinator`] and ensures the assertion succeeds for
/// every inner value.
#[derive(Clone, Debug)]
pub struct AllCombinator<Inner> {
    inner: Inner,
}

impl<Inner> AllCombinator<Inner> {
    /// Creates a new instance of this combinator.
    #[inline]
    pub fn new(inner: Inner) -> Self {
        Self { inner }
    }
}

impl<Inner> AssertionCombinator for AllCombinator<Inner>
where
    Inner: AssertionCombinator,
    Inner::Target: IntoIterator,
{
    type Target = <Inner::Target as IntoIterator>::Item;
    type Result = Inner::Result;

    #[inline]
    fn execute<F>(self, mut f: F) -> Self::Result
    where
        F: FnMut(Self::Target) -> AssertionResult,
    {
        self.inner
            .execute(|values| values.into_iter().map(&mut f).collect())
    }
}
