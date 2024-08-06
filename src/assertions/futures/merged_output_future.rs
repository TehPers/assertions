use std::{
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};

use futures::{
    stream::{Collect, FuturesUnordered},
    StreamExt,
};
use pin_project_lite::pin_project;

use crate::assertions::{
    general::{FinalizableResult, InvertibleResult},
    iterators::MergeableResult,
    AssertionContext,
};

use super::{FinalizedOutputFuture, InvertedOutputFuture};

pin_project! {
    /// Merges many asynchronous outputs.
    #[derive(Debug)]
    pub struct MergedOutputsFuture<F>
    where
        F: Future,
    {
        #[pin]
        inner: Collect<FuturesUnordered<F>, Vec<F::Output>>,
        cx: Option<AssertionContext>,
        merge_kind: MergeKind,
    }
}

impl<F> MergedOutputsFuture<F>
where
    F: Future,
    F::Output: MergeableResult,
{
    /// Creates a new merged outputs future using
    /// [`merge_all`](MergeableResult::merge_all()).
    #[inline]
    pub fn all<I>(cx: AssertionContext, futs: I) -> Self
    where
        I: IntoIterator<Item = F>,
    {
        Self {
            inner: FuturesUnordered::from_iter(futs.into_iter()).collect(),
            cx: Some(cx),
            merge_kind: MergeKind::All,
        }
    }

    /// Creates a new merged outputs future using
    /// [`merge_any`](MergeableResult::merge_any()).
    #[inline]
    pub fn any<I>(cx: AssertionContext, futs: I) -> Self
    where
        I: IntoIterator<Item = F>,
    {
        Self {
            inner: FuturesUnordered::from_iter(futs.into_iter()).collect(),
            cx: Some(cx),
            merge_kind: MergeKind::Any,
        }
    }
}

impl<F> Future for MergedOutputsFuture<F>
where
    F: Future,
    F::Output: MergeableResult,
{
    type Output = <F::Output as MergeableResult>::Merged;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let projected = self.project();
        let outputs = ready!(projected.inner.poll(cx));
        let cx = projected.cx.take().expect("poll after ready");
        Poll::Ready(match projected.merge_kind {
            MergeKind::All => F::Output::merge_all(cx, outputs),
            MergeKind::Any => F::Output::merge_any(cx, outputs),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum MergeKind {
    All,
    Any,
}

impl<F> InvertibleResult for MergedOutputsFuture<F>
where
    F: Future,
    F::Output: MergeableResult,
    <F::Output as MergeableResult>::Merged: InvertibleResult,
{
    type Inverted = InvertedOutputFuture<Self>;

    #[inline]
    fn invert(self, cx: AssertionContext) -> Self::Inverted {
        InvertedOutputFuture::new(cx, self)
    }
}

impl<F> MergeableResult for MergedOutputsFuture<F>
where
    F: Future,
    F::Output: MergeableResult,
    <F::Output as MergeableResult>::Merged: MergeableResult,
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

impl<F> FinalizableResult for MergedOutputsFuture<F>
where
    F: Future,
    F::Output: MergeableResult,
    <F::Output as MergeableResult>::Merged: FinalizableResult,
{
    type Finalized = FinalizedOutputFuture<Self>;

    #[inline]
    fn finalize(self) -> Self::Finalized {
        FinalizedOutputFuture::new(self)
    }
}
