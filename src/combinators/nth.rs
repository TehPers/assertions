use std::fmt::Display;

use crate::Assertable;

/// Wraps an [`Assertable`] and performs an assertion on a specific element in
/// the target.
#[derive(Clone, Debug)]
pub struct NthCombinator<Inner> {
    inner: Inner,
    n: usize,
}

impl<Inner> NthCombinator<Inner> {
    /// Creates a new combinator which wraps an inner [`Assertable`].
    #[inline]
    pub fn new(inner: Inner, n: usize) -> Self {
        Self { inner, n }
    }
}

impl<Inner> Assertable for NthCombinator<Inner>
where
    Inner: Assertable,
    <Inner as Assertable>::Target: IntoIterator,
{
    type Target = <Inner::Target as IntoIterator>::Item;
    type Result = Inner::Result;

    fn to_satisfy<F>(self, expectation: impl Display, mut f: F) -> Self::Result
    where
        F: FnMut(Self::Target) -> bool,
    {
        self.inner.to_satisfy(
            format_args!("element {} exists and satisfies: {}", self.n, expectation),
            |values| values.into_iter().nth(self.n).is_some_and(&mut f),
        )
    }
}
