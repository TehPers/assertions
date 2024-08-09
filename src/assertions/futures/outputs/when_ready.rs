use std::{
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};

use pin_project_lite::pin_project;

use crate::assertions::{Assertion, AssertionContext};

pin_project! {
    /// A [`Future`] which executes an assertion when its subject is ready.
    ///
    /// Created by [`when_ready`](crate::prelude::when_ready).
    #[derive(Clone, Debug)]
    pub struct WhenReadyFuture<T, A>
    where
        T: Future,
    {
        #[pin]
        subject: T,
        next: Option<(AssertionContext, A)>,
    }
}

impl<T, A> WhenReadyFuture<T, A>
where
    T: Future,
    A: Assertion<T::Output>,
{
    /// Creates a new instance of this future.
    #[inline]
    pub(crate) fn new(cx: AssertionContext, subject: T, next: A) -> Self {
        Self {
            subject,
            next: Some((cx, next)),
        }
    }
}

impl<T, A> Future for WhenReadyFuture<T, A>
where
    T: Future,
    A: Assertion<T::Output>,
{
    type Output = A::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let projected = self.project();
        let input = ready!(projected.subject.poll(cx));
        let (cx, next) = projected.next.take().expect("poll after ready");
        Poll::Ready(next.execute(cx, input))
    }
}
