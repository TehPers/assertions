use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use pin_project_lite::pin_project;

use crate::{
    assertions::{Assertion, AssertionContext},
    AssertionOutput,
};

pin_project! {
    /// A [`Future`] that checks the completion order of two inner futures, then
    /// executes an inner assertion if the ordering constraint is satisfied.
    ///
    /// Created by both [`when_ready_before`](crate::prelude::when_ready_before)
    /// and [`when_ready_after`](crate::prelude::when_ready_after).
    #[derive(Clone, Debug)]
    #[must_use]
    pub struct CompletionOrderFuture<Fut, T, A> {
        #[pin]
        subject: T,
        #[pin]
        fut: Fut,
        fut_done: bool,
        next: Option<(AssertionContext, A)>,
        order: CompletionOrder,
    }
}

impl<Fut, T, A> CompletionOrderFuture<Fut, T, A> {
    pub(crate) fn new(
        cx: AssertionContext,
        subject: T,
        fut: Fut,
        next: A,
        order: CompletionOrder,
    ) -> Self {
        Self {
            subject,
            fut,
            fut_done: false,
            next: Some((cx, next)),
            order,
        }
    }
}

impl<Fut, T, A> Future for CompletionOrderFuture<Fut, T, A>
where
    Fut: Future,
    T: Future,
    A: Assertion<T::Output, Output = AssertionOutput>,
{
    type Output = AssertionOutput;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let projected = self.project();

        // Update whether we finished the non-subject future
        *projected.fut_done =
            *projected.fut_done || matches!(projected.fut.poll(cx), Poll::Ready(_));

        // Get the success/error for the assertion
        #[allow(clippy::match_same_arms)]
        let result = match (
            projected.order,
            *projected.fut_done,
            projected.subject.poll(cx),
        ) {
            // Neither future is done
            (_, false, Poll::Pending) => return Poll::Pending,

            // Check if subject completed first (succeed on ties)
            (CompletionOrder::Before, _, Poll::Ready(subject)) => Ok(subject),
            (CompletionOrder::Before, true, Poll::Pending) => Err("did not complete before"),

            // Check if subject completed last (succeed on ties)
            (CompletionOrder::After, true, Poll::Ready(subject)) => Ok(subject),
            (CompletionOrder::After, true, Poll::Pending) => return Poll::Pending, // need output
            (CompletionOrder::After, false, Poll::Ready(_)) => Err("completed before"),
        };

        // Call next assertion (if success)
        let (cx, next) = projected.next.take().expect("poll after ready");
        Poll::Ready(match result {
            Ok(subject) => next.execute(cx, subject),
            Err(error) => cx.fail(error),
        })
    }
}

/// The order that the futures are expected to complete in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum CompletionOrder {
    /// Subject completes before the provided future.
    Before,

    /// Subject completes after the provided future.
    After,
}
