use std::fmt::Display;

use crate::Assertable;

/// Wraps an [`Assertable`] and performs an assertion on the number of elements
/// in the target.
#[derive(Clone, Debug)]
pub struct CountCombinator<Inner> {
    inner: Inner,
}

impl<Inner> CountCombinator<Inner> {
    /// Creates a new combinator which wraps an inner [`Assertable`].
    #[inline]
    pub fn new(inner: Inner) -> Self {
        Self { inner }
    }
}

impl<Inner> Assertable for CountCombinator<Inner>
where
    Inner: Assertable,
    <Inner as Assertable>::Target: IntoIterator,
{
    type Target = usize;
    type Result = Inner::Result;

    fn to_satisfy<F>(self, expectation: impl Display, mut f: F) -> Self::Result
    where
        F: FnMut(Self::Target) -> bool,
    {
        self.inner.to_satisfy(
            format_args!("the length satisfies: {expectation}"),
            |values| f(values.into_iter().count()),
        )
    }
}
