use std::fmt::Display;

use crate::Assertable;

/// Wraps an [`Assertable`] and applies the assertion to the inner value
/// contained within the target. If the target is [`Err`], then the assertion
/// fails instead.
#[derive(Clone, Debug)]
pub struct OkCombinator<Inner> {
    inner: Inner,
}

impl<Inner> OkCombinator<Inner> {
    /// Creates a new combinator which wraps an inner [`Assertable`].
    #[inline]
    pub fn new(inner: Inner) -> Self {
        Self { inner }
    }
}

impl<Inner, T, E> Assertable for OkCombinator<Inner>
where
    Inner: Assertable<Target = Result<T, E>>,
{
    type Target = T;
    type Result = Inner::Result;

    fn to_satisfy<F>(self, expectation: impl Display, mut f: F) -> Self::Result
    where
        F: FnMut(Self::Target) -> bool,
    {
        self.inner.to_satisfy(
            format_args!("value is `Ok`, and inner value satisfies: {expectation}"),
            |value| value.is_ok_and(&mut f),
        )
    }
}
