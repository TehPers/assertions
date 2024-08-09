use std::{
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};

use pin_project_lite::pin_project;

use crate::assertions::{general::InvertibleOutput, AssertionContext};

pin_project! {
    /// Inverts an asynchronous output.
    #[derive(Clone, Debug)]
    pub struct InvertedOutputFuture<F> {
        #[pin]
        inner: F,
        cx: Option<AssertionContext>,
    }
}

impl<F> InvertedOutputFuture<F>
where
    F: Future<Output: InvertibleOutput>,
{
    /// Creates a new inverted output future.
    #[inline]
    pub fn new(cx: AssertionContext, inner: F) -> Self {
        Self {
            inner,
            cx: Some(cx),
        }
    }
}

impl<F> Future for InvertedOutputFuture<F>
where
    F: Future<Output: InvertibleOutput>,
{
    type Output = <F::Output as InvertibleOutput>::Inverted;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let projected = self.project();
        let output = ready!(projected.inner.poll(cx));
        let cx = projected.cx.take().expect("poll after ready");
        Poll::Ready(output.invert(cx))
    }
}

impl<F> InvertibleOutput for F
where
    F: Future<Output: InvertibleOutput>,
{
    type Inverted = InvertedOutputFuture<F>;

    #[inline]
    fn invert(self, cx: AssertionContext) -> Self::Inverted {
        InvertedOutputFuture::new(cx, self)
    }
}
