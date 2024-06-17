use std::fmt::Display;

use crate::Assertable;

/// Wraps an [`Assertable`] and applies the assertion to the inner value
/// contained within the target. If the target is [`None`], then the
/// assertion fails instead.
#[derive(Clone, Debug)]
pub struct SomeCombinator<Inner> {
    inner: Inner,
}

impl<Inner> SomeCombinator<Inner> {
    /// Creates a new combinator which wraps an inner [`Assertable`].
    #[inline]
    pub fn new(inner: Inner) -> Self {
        Self { inner }
    }
}

impl<Inner, T> Assertable for SomeCombinator<Inner>
where
    Inner: Assertable<Target = Option<T>>,
{
    type Target = T;
    type Result = Inner::Result;

    fn to_satisfy<F>(self, expectation: impl Display, mut f: F) -> Self::Result
    where
        F: FnMut(Self::Target) -> bool,
    {
        self.inner.to_satisfy(
            format_args!("value is `Some`, and inner value satisfies: {expectation}"),
            |value| value.is_some_and(&mut f),
        )
    }
}
