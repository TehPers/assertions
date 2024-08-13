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
    #[must_use]
    pub struct UnwrappedOutputFuture<F> {
        #[pin]
        inner: F,
    }
}

impl<F> UnwrappedOutputFuture<F>
where
    F: Future<Output: UnwrappableOutput>,
{
    /// Creates a new instance of this future.
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

pin_project! {
    /// Tries to unwrap an asynchronous output.
    #[derive(Clone, Debug)]
    #[must_use]
    pub struct TryUnwrappedOutputFuture<F> {
        #[pin]
        inner: F,
    }
}

impl<F> TryUnwrappedOutputFuture<F>
where
    F: Future<Output: UnwrappableOutput>,
{
    /// Creates a new instance of this future.
    #[inline]
    pub fn new(inner: F) -> Self {
        Self { inner }
    }
}

impl<F> Future for TryUnwrappedOutputFuture<F>
where
    F: Future<Output: UnwrappableOutput>,
{
    type Output = <F::Output as UnwrappableOutput>::TryUnwrapped;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let projected = self.project();
        let output = ready!(projected.inner.poll(cx));
        Poll::Ready(output.try_unwrap())
    }
}

impl<F> UnwrappableOutput for F
where
    F: Future<Output: UnwrappableOutput>,
{
    type Unwrapped = UnwrappedOutputFuture<F>;
    type TryUnwrapped = TryUnwrappedOutputFuture<F>;

    #[inline]
    fn unwrap(self) -> Self::Unwrapped {
        UnwrappedOutputFuture::new(self)
    }

    #[inline]
    fn try_unwrap(self) -> Self::TryUnwrapped {
        TryUnwrappedOutputFuture::new(self)
    }
}
