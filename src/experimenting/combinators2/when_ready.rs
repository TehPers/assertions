use std::{
    fmt::{Display, Formatter},
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};

use pin_project_lite::pin_project;

use super::{Assertion, Combinator};

/// Performs the assertion when the input future is ready.
#[derive(Clone, Copy, Debug, Default)]
pub struct WhenReadyCombinator;

impl<Next> Combinator<Next> for WhenReadyCombinator {
    type Assertion = WhenReadyAssertion<Next>;

    fn build(self, next: Next) -> Self::Assertion {
        WhenReadyAssertion::new(next)
    }
}

/// Waits for the input to be ready, then passes it to the next assertion.
#[derive(Clone, Copy, Debug, Default)]
pub struct WhenReadyAssertion<Next> {
    next: Next,
}

impl<Next> WhenReadyAssertion<Next> {
    /// Creates a new [`WhenReadyAssertion`].
    #[inline]
    pub fn new(next: Next) -> Self {
        Self { next }
    }
}

impl<Next> Display for WhenReadyAssertion<Next>
where
    Next: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "when ready, {}", self.next)
    }
}

impl<Next, Input> Assertion<Input> for WhenReadyAssertion<Next>
where
    Input: Future,
    Next: Assertion<Input::Output>,
{
    type Output = WhenReadyFuture<Input, Next>;

    fn execute(self, input: Input) -> Self::Output {
        WhenReadyFuture {
            input,
            next: Some(self.next),
        }
    }
}

pin_project! {
    /// A future that performs an assertion when it is ready.
    pub struct WhenReadyFuture<Input, Next> {
        #[pin]
        input: Input,
        next: Option<Next>,
    }
}

impl<Input, Next> Future for WhenReadyFuture<Input, Next>
where
    Input: Future,
    Next: Assertion<Input::Output>,
{
    type Output = Next::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let projected = self.project();
        let res = ready!(projected.input.poll(cx));
        let next = projected.next.take().expect("polled after ready");
        Poll::Ready(next.execute(res))
    }
}
