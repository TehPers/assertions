use std::{
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};

use pin_project_lite::pin_project;

use crate::assertions::general::FinalizableResult;

pin_project! {
    /// Finalizes an asynchronous output.
    #[derive(Clone, Debug)]
    pub struct FinalizedOutputFuture<F> {
        #[pin]
        inner: F,
    }
}

impl<F> FinalizedOutputFuture<F>
where
    F: Future,
    F::Output: FinalizableResult,
{
    /// Create a new finalized output future.
    #[inline]
    pub fn new(inner: F) -> Self {
        Self { inner }
    }
}

impl<F> Future for FinalizedOutputFuture<F>
where
    F: Future,
    F::Output: FinalizableResult,
{
    type Output = <F::Output as FinalizableResult>::Finalized;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let projected = self.project();
        let output = ready!(projected.inner.poll(cx));
        Poll::Ready(output.finalize())
    }
}
