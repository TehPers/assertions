use std::fmt::Display;

use crate::Assertable;

/// Wraps an [`Assertable`] and applies the assertion to the error value
/// contained within the target. If the target is [`Ok`], then the assertion
/// fails instead.
#[derive(Clone, Debug)]
pub struct ErrCombinator<Inner> {
    inner: Inner,
}

impl<Inner> ErrCombinator<Inner> {
    /// Creates a new combinator which wraps an inner [`Assertable`].
    #[inline]
    pub fn new(inner: Inner) -> Self {
        Self { inner }
    }
}

impl<Inner, T, E> Assertable for ErrCombinator<Inner>
where
    Inner: Assertable<Target = Result<T, E>>,
{
    type Target = E;
    type Result = Inner::Result;

    fn to_satisfy<F>(self, expectation: impl Display, mut f: F) -> Self::Result
    where
        F: FnMut(Self::Target) -> bool,
    {
        self.inner.to_satisfy(
            format_args!("value is `Err`, and inner value satisfies: {expectation}"),
            |value| value.is_err_and(&mut f),
        )
    }
}
