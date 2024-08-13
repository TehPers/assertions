use std::fmt::Debug;

use crate::assertions::{Assertion, AssertionContext, AssertionContextBuilder, AssertionModifier};

/// Extracts the [`Debug`] representation of the subject.
#[derive(Clone, Debug)]
pub struct AsDebugModifier<M> {
    prev: M,
}

impl<M> AsDebugModifier<M> {
    #[inline]
    pub(crate) fn new(prev: M) -> Self {
        Self { prev }
    }
}

impl<M, A> AssertionModifier<A> for AsDebugModifier<M>
where
    M: AssertionModifier<AsDebugAssertion<A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
        self.prev.apply(cx, AsDebugAssertion { next })
    }
}

/// Executes the inner assertion with the [`Debug`] representation of the
/// subject.
#[derive(Clone, Debug)]
pub struct AsDebugAssertion<A> {
    next: A,
}

impl<A, T> Assertion<T> for AsDebugAssertion<A>
where
    A: Assertion<String>,
    T: Debug,
{
    type Output = A::Output;

    #[inline]
    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        self.next.execute(cx, format!("{subject:?}"))
    }
}
