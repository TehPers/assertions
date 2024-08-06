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

use super::{FinalizedOutputFuture, InvertedOutputFuture, MergedOutputsFuture};

pin_project! {
    /// A [`Future`] which executes an assertion when its subject is ready.
    #[derive(Clone, Debug)]
    pub struct WhenReadyFuture<T, O>
    where
        T: Future,
    {
        #[pin]
        subject: T,
        cx: Option<AssertionContext>,
        next: fn(AssertionContext, T::Output) -> O,
    }
}

impl<T, O> WhenReadyFuture<T, O>
where
    T: Future,
{
    /// Creates a new instance of this future.
    #[inline]
    pub(crate) fn new(
        cx: AssertionContext,
        subject: T,
        next: fn(AssertionContext, T::Output) -> O,
    ) -> Self {
        Self {
            subject,
            cx: Some(cx),
            next,
        }
    }
}

impl<T, O> Future for WhenReadyFuture<T, O>
where
    T: Future,
{
    type Output = O;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let projected = self.project();
        let input = ready!(projected.subject.poll(cx));
        let cx = projected.cx.take().expect("poll after ready");
        Poll::Ready((projected.next)(cx, input))
    }
}

impl<T, O> InvertibleResult for WhenReadyFuture<T, O>
where
    T: Future,
    O: InvertibleResult,
{
    type Inverted = InvertedOutputFuture<Self>;

    #[inline]
    fn invert(self, cx: AssertionContext) -> Self::Inverted {
        InvertedOutputFuture::new(cx, self)
    }
}

impl<T, O> MergeableResult for WhenReadyFuture<T, O>
where
    T: Future,
    O: MergeableResult,
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

impl<T, O> FinalizableResult for WhenReadyFuture<T, O>
where
    T: Future,
    O: FinalizableResult,
{
    type Finalized = FinalizedOutputFuture<Self>;

    #[inline]
    fn finalize(self) -> Self::Finalized {
        FinalizedOutputFuture::new(self)
    }
}
