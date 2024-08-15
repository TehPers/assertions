use futures::AsyncRead;

use crate::assertions::{
    async_read::WhenReadAsyncFuture, general::IntoInitializableOutput, Assertion, AssertionContext,
    AssertionContextBuilder, AssertionModifier,
};

/// Reads a subject into a buffer asynchronously.
#[derive(Clone, Debug)]
pub struct WhenReadAsyncModifier<M> {
    prev: M,
}

impl<M> WhenReadAsyncModifier<M> {
    #[inline]
    pub(crate) fn new(prev: M) -> Self {
        Self { prev }
    }
}

impl<M, A> AssertionModifier<A> for WhenReadAsyncModifier<M>
where
    M: AssertionModifier<WhenReadAsyncAssertion<A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
        self.prev.apply(cx, WhenReadAsyncAssertion { next })
    }
}

/// Reads the subject into a buffer asynchronously and executes the inner
/// assertion on it.
#[derive(Clone, Debug)]
pub struct WhenReadAsyncAssertion<A> {
    next: A,
}

impl<A, T> Assertion<T> for WhenReadAsyncAssertion<A>
where
    A: Assertion<Vec<u8>, Output: IntoInitializableOutput>,
    T: AsyncRead,
{
    type Output = WhenReadAsyncFuture<T, A>;

    #[inline]
    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        WhenReadAsyncFuture::new(cx, subject, self.next)
    }
}
