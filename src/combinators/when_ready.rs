use std::{
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};

use pin_project_lite::pin_project;

use crate::Assertable;

/// Wraps an [`Assertable`] and performs an expectation on the result of the
/// target future when it is ready.
#[derive(Clone, Debug)]
pub struct WhenReadyExpectation<Inner> {
    inner: Inner,
}

impl<Inner> WhenReadyExpectation<Inner> {
    /// Creates a new [`CountExpectation`] which wraps an inner [`Assertable`].
    #[inline]
    pub fn new(inner: Inner) -> Self {
        Self { inner }
    }
}

impl<Inner> Assertable for WhenReadyExpectation<Inner>
where
    Inner: Assertable,
    <Inner as Assertable>::Target: Future,
{
    type Target = <Inner::Target as Future>::Output;
    type Result = WhenReadyExpectationFuture<Self::Target, Inner>;

    fn to_satisfy<F>(self, expectation: &str, f: F) -> Self::Result
    where
        F: FnMut(Self::Target) -> bool,
    {
        WhenReadyExpectationFuture {
            future: todo!(),
            expectation: expectation.to_string(),
            inner: self.inner,
            predicate: Box::new(f),
        }
    }
}

pin_project! {
    /// A future that performs an assertion when it is ready.
    pub struct WhenReadyExpectationFuture<Fut, Inner>
    where
        Fut: Future,
    {
        #[pin]
        future: Fut,
        expectation: String,
        inner: Inner,
        predicate: Box<dyn FnMut(Fut::Output) -> bool>,
    }
}

impl<Fut, Inner> Future for WhenReadyExpectationFuture<Fut, Inner>
where
    Fut: Future,
    Inner: Assertable,
{
    type Output = Inner::Result;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let projected = self.project();
        let output = ready!(projected.future.poll(cx));
        Poll::Ready(
            projected
                .inner
                .to_satisfy(&projected.expectation, projected.predicate),
        )
    }
}
