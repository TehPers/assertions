use std::fmt::Display;

use crate::Assertable;

/// Wraps an [`Assertable`] and negates the expectation. The overall assertion
/// succeeds if and only if the chained assertion fails.
#[derive(Clone, Debug)]
pub struct NotCombinator<Inner> {
    inner: Inner,
}

impl<Inner> NotCombinator<Inner> {
    /// Creates a new combinator which wraps an inner [`Assertable`].
    #[inline]
    pub fn new(inner: Inner) -> Self {
        Self { inner }
    }
}

impl<Inner> Assertable for NotCombinator<Inner>
where
    Inner: Assertable,
{
    type Target = Inner::Target;
    type Result = Inner::Result;

    fn to_satisfy<F>(self, expectation: impl Display, mut f: F) -> Self::Result
    where
        F: FnMut(Self::Target) -> bool,
    {
        self.inner.to_satisfy(
            format_args!("the following is not satisfied: {expectation}"),
            |value| !f(value),
        )
    }
}
