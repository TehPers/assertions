use std::{
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};

use pin_project_lite::pin_project;

use crate::assertions::{
    general::{FinalizableResult, InvertibleResult},
    iterators::MergeableResult,
    AssertionContext,
};

use super::{FinalizedOutputFuture, MergedOutputsFuture};

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
    F: Future,
    F::Output: InvertibleResult,
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
    F: Future,
    F::Output: InvertibleResult,
{
    type Output = <F::Output as InvertibleResult>::Inverted;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let projected = self.project();
        let output = ready!(projected.inner.poll(cx));
        let cx = projected.cx.take().expect("poll after ready");
        Poll::Ready(output.invert(cx))
    }
}

impl<F> InvertibleResult for InvertedOutputFuture<F> {
    type Inverted = F;

    #[inline]
    fn invert(self, _cx: AssertionContext) -> Self::Inverted {
        // Undo the inversion to preserve context
        self.inner
    }
}

impl<F> MergeableResult for InvertedOutputFuture<F>
where
    F: Future,
    F::Output: InvertibleResult,
    <F::Output as InvertibleResult>::Inverted: MergeableResult,
{
    type Merged = MergedOutputsFuture<Self>;

    #[inline]
    fn merge_all<I>(cx: AssertionContext, results: I) -> Self::Merged
    where
        I: IntoIterator<Item = Self>,
    {
        MergedOutputsFuture::all(cx, results)
    }

    #[inline]
    fn merge_any<I>(cx: AssertionContext, results: I) -> Self::Merged
    where
        I: IntoIterator<Item = Self>,
    {
        MergedOutputsFuture::any(cx, results)
    }
}

impl<F> FinalizableResult for InvertedOutputFuture<F>
where
    F: Future,
    F::Output: InvertibleResult,
    <F::Output as InvertibleResult>::Inverted: FinalizableResult,
{
    type Finalized = FinalizedOutputFuture<Self>;

    #[inline]
    fn finalize(self) -> Self::Finalized {
        FinalizedOutputFuture::new(self)
    }
}
