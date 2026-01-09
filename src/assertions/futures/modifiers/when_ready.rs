use std::future::IntoFuture;

use crate::assertions::{
    futures::WhenReadyFuture, Assertion, AssertionContext, AssertionContextBuilder,
    AssertionModifier,
};

/// Executes as assertion when the subject is ready.
#[derive(Clone, Debug)]
pub struct WhenReadyModifier<M> {
    prev: M,
}

impl<M> WhenReadyModifier<M> {
    #[inline]
    pub(crate) fn new(prev: M) -> Self {
        Self { prev }
    }
}

impl<M, A> AssertionModifier<A> for WhenReadyModifier<M>
where
    M: AssertionModifier<WhenReadyAssertion<A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
        self.prev.apply(cx, WhenReadyAssertion { next })
    }
}

/// Executes the inner assertion when the subject is ready.
#[derive(Clone, Debug)]
pub struct WhenReadyAssertion<A> {
    next: A,
}

impl<A, T> Assertion<T> for WhenReadyAssertion<A>
where
    T: IntoFuture,
    A: Assertion<T::Output>,
{
    type Output = WhenReadyFuture<T::IntoFuture, A>;

    #[inline]
    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        WhenReadyFuture::new(cx, subject.into_future(), self.next)
    }
}
