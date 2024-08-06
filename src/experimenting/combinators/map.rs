use std::fmt::Display;

use crate::Assertable;

/// Wraps an [`Assertable`] and applies the assertion to a target derived from
/// the inner expectation's target. In other words, this maps the target to a
/// new value, then applys the assertion to the new value.
#[derive(Clone, Debug)]
#[must_use = "a combinator does nothing without an assertion"]
pub struct MapCombinator<Inner, M> {
    inner: Inner,
    map: M,
}

impl<Inner, M> MapCombinator<Inner, M> {
    /// Creates a new combinator which wraps an inner [`Assertable`].
    #[inline]
    pub fn new(inner: Inner, map: M) -> Self {
        Self { inner, map }
    }
}

impl<Inner, M, T> Assertable for MapCombinator<Inner, M>
where
    Inner: Assertable,
    M: FnMut(Inner::Target) -> T,
{
    type Target = T;
    type Result = Inner::Result;

    fn to_satisfy<F>(mut self, expectation: impl Display, mut f: F) -> Self::Result
    where
        F: FnMut(Self::Target) -> bool,
    {
        self.inner.to_satisfy(
            format_args!("for the mapped value, {expectation}"),
            |value| f((self.map)(value)),
        )
    }
}
