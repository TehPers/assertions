use std::fmt::Display;

use crate::Assertable;

/// Wraps an [`Assertable`] and applies the assertion to each element in the
/// target. If there does not exist an element that satisfies the chained
/// assertion, then the whole assertion fails.
///
/// This is similar to [`AllCombinator`](crate::combinators::AllCombinator),
/// but only one element needs to satisfy the expectation.
#[derive(Clone, Debug)]
pub struct AnyCombinator<Inner> {
    inner: Inner,
}

impl<Inner> AnyCombinator<Inner> {
    /// Creates a new combinator which wraps an inner [`Assertable`].
    #[inline]
    pub fn new(inner: Inner) -> Self {
        Self { inner }
    }
}

impl<Inner> Assertable for AnyCombinator<Inner>
where
    Inner: Assertable,
    Inner::Target: IntoIterator,
{
    type Target = <Inner::Target as IntoIterator>::Item;
    type Result = Inner::Result;

    fn to_satisfy<F>(self, expectation: impl Display, mut f: F) -> Self::Result
    where
        F: FnMut(Self::Target) -> bool,
    {
        self.inner.to_satisfy(
            format_args!("for some inner value, {expectation}"),
            |values| values.into_iter().any(|value| f(value)),
        )
    }
}
