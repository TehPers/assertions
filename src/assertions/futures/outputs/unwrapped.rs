use std::{
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};

use pin_project_lite::pin_project;

use crate::assertions::general::UnwrappableOutput;

pin_project! {
    /// Unwraps an asynchronous output.
    #[derive(Clone, Debug)]
    pub struct UnwrappedOutputFuture<F> {
        #[pin]
        inner: F,
    }
}

impl<F> UnwrappedOutputFuture<F>
where
    F: Future<Output: UnwrappableOutput>,
{
    /// Create a new finalized output future.
    #[inline]
    pub fn new(inner: F) -> Self {
        Self { inner }
    }
}

impl<F> Future for UnwrappedOutputFuture<F>
where
    F: Future<Output: UnwrappableOutput>,
{
    type Output = <F::Output as UnwrappableOutput>::Unwrapped;

    #[inline]
    #[track_caller]
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let projected = self.project();
        let output = ready!(projected.inner.poll(cx));
        Poll::Ready(output.unwrap())
    }
}

impl<F> UnwrappableOutput for F
where
    F: Future<Output: UnwrappableOutput>,
{
    type Unwrapped = UnwrappedOutputFuture<F>;

    #[inline]
    fn unwrap(self) -> Self::Unwrapped {
        UnwrappedOutputFuture::new(self)
    }
}
