use std::fmt::Display;

use crate::assertions::{Assertion, AssertionContext, AssertionContextBuilder, AssertionModifier};

/// Extracts the [`Display`] representation of the subject.
#[derive(Clone, Debug)]
pub struct AsDisplayModifier<M> {
    prev: M,
}

impl<M> AsDisplayModifier<M> {
    #[inline]
    pub(crate) fn new(prev: M) -> Self {
        Self { prev }
    }
}

impl<M, A> AssertionModifier<A> for AsDisplayModifier<M>
where
    M: AssertionModifier<AsDisplayAssertion<A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
        self.prev.apply(cx, AsDisplayAssertion { next })
    }
}

/// Executes the inner assertion with the [`Display`] representation of the
/// subject.
#[derive(Clone, Debug)]
pub struct AsDisplayAssertion<A> {
    next: A,
}

impl<A, T> Assertion<T> for AsDisplayAssertion<A>
where
    A: Assertion<String>,
    T: Display,
{
    type Output = A::Output;

    #[inline]
    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        self.next.execute(cx, subject.to_string())
    }
}
