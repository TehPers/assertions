use std::future::Future;

use crate::{
    assertions::{
        futures::{CompletionOrder, CompletionOrderFuture},
        Assertion, AssertionContext, AssertionContextBuilder, AssertionModifier,
    },
    metadata::Annotated,
};

/// Executes an assertion when the subject completes before or after another
/// future.
#[derive(Clone, Debug)]
pub struct CompletionOrderModifier<Fut, M> {
    prev: M,
    fut: Annotated<Fut>,
    order: CompletionOrder,
}

impl<Fut, M> CompletionOrderModifier<Fut, M> {
    #[inline]
    pub(crate) fn new(prev: M, fut: Annotated<Fut>, order: CompletionOrder) -> Self {
        Self { prev, fut, order }
    }
}

impl<Fut, M, A> AssertionModifier<A> for CompletionOrderModifier<Fut, M>
where
    M: AssertionModifier<CompletionOrderAssertion<Fut, A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
        self.prev.apply(
            cx,
            CompletionOrderAssertion {
                next,
                fut: self.fut,
                order: self.order,
            },
        )
    }
}

/// Executes the inner assertion when the subject completes before or after
/// another future.
#[derive(Clone, Debug)]
pub struct CompletionOrderAssertion<Fut, A> {
    next: A,
    fut: Annotated<Fut>,
    order: CompletionOrder,
}

impl<Fut, A, T> Assertion<T> for CompletionOrderAssertion<Fut, A>
where
    Fut: Future,
    A: Assertion<T::Output>,
    T: Future,
{
    type Output = CompletionOrderFuture<Fut, T, A>;

    #[inline]
    fn execute(self, mut cx: AssertionContext, subject: T) -> Self::Output {
        cx.annotate("other", &self.fut);
        CompletionOrderFuture::new(cx, subject, self.fut.into_inner(), self.next, self.order)
    }
}
