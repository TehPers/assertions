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
    iterators::{MergeStrategy, MergeableOutput},
    AssertionContext,
};

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
        strategy: MergeStrategy,
    }
}

impl<F> MergedOutputsFuture<F>
where
    F: Future<Output: MergeableOutput>,
{
    /// Creates a new merged outputs future using the given merge strategy.
    #[inline]
    pub fn new<I>(cx: AssertionContext, strategy: MergeStrategy, outputs: I) -> Self
    where
        I: IntoIterator<Item = F>,
    {
        Self {
            inner: FuturesUnordered::from_iter(outputs).collect(),
            cx: Some(cx),
            strategy,
        }
    }
}

impl<F> Future for MergedOutputsFuture<F>
where
    F: Future<Output: MergeableOutput>,
{
    type Output = <F::Output as MergeableOutput>::Merged;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let projected = self.project();
        let outputs = ready!(projected.inner.poll(cx));
        let cx = projected.cx.take().expect("poll after ready");
        Poll::Ready(MergeableOutput::merge(cx, *projected.strategy, outputs))
    }
}

impl<F> MergeableOutput for F
where
    F: Future<Output: MergeableOutput>,
{
    type Merged = MergedOutputsFuture<F>;

    #[inline]
    fn merge<I>(cx: AssertionContext, strategy: MergeStrategy, outputs: I) -> Self::Merged
    where
        I: IntoIterator<Item = Self>,
    {
        MergedOutputsFuture::new(cx, strategy, outputs)
    }
}
