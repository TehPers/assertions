use std::{
    fmt::{Display, Formatter},
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};

use pin_project_lite::pin_project;

use super::{Assertion, AssertionCombinator};

/// Wraps another [`AssertionCombinator`] and executes assertions on the output
/// of the inner value's future.
///
/// This causes assertions to be asynchronous. Assertion outputs from this
/// combinator will be wrapped in futures, meaning the assertions must be
/// `.await`ed.
#[derive(Clone, Debug)]
pub struct WhenReadyCombinator<Inner> {
    inner: Inner,
}

impl<Inner> WhenReadyCombinator<Inner> {
    /// Creates a new instance of this combinator, wrapping the inner
    /// combinator.
    #[inline]
    pub fn new(inner: Inner) -> Self {
        Self { inner }
    }
}

impl<Inner, Next> AssertionCombinator<Next> for WhenReadyCombinator<Inner>
where
    Inner: AssertionCombinator<WhenReadyAssertion<Next>>,
    Inner::NextTarget: Future,
{
    type Target = Inner::Target;
    type NextTarget = <Inner::NextTarget as Future>::Output;
    type Assertion = Inner::Assertion;

    #[inline]
    fn apply(self, next: Next) -> Self::Assertion {
        self.inner.apply(WhenReadyAssertion::new(next))
    }
}

/// Wraps another assertion and executes it on the output of the provided
/// future.
#[derive(Clone, Debug, Default)]
pub struct WhenReadyAssertion<Next> {
    next: Next,
}

impl<Next> WhenReadyAssertion<Next> {
    /// Creates a new instance of this assertion.
    #[inline]
    pub fn new(next: Next) -> Self {
        Self { next }
    }
}

impl<Next> Display for WhenReadyAssertion<Next>
where
    Next: Display,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "when the future is ready, {}", self.next)
    }
}

impl<Next, Target> Assertion<Target> for WhenReadyAssertion<Next>
where
    Target: Future,
    Next: Assertion<Target::Output>,
{
    type Output = WhenReadyOutput<Target, Next>;

    #[inline]
    fn assert(self, target: Target) -> Self::Output {
        WhenReadyOutput {
            target,
            next: Some(self.next),
        }
    }
}

pin_project! {
    /// An assertion output that will resolve to a success or an error
    /// eventually. This is usually created when performing an assertion on the
    /// output of a future.
    #[derive(Clone, Debug, Default)]
    #[must_use]
    pub struct WhenReadyOutput<Target, Next> {
        #[pin]
        target: Target,
        next: Option<Next>,
    }
}

impl<Target, Next> Future for WhenReadyOutput<Target, Next>
where
    Target: Future,
    Next: Assertion<Target::Output>,
{
    type Output = Next::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let projected = self.project();
        let target = ready!(projected.target.poll(cx));
        let next = projected.next.take().expect("poll after ready");
        Poll::Ready(next.assert(target))
    }
}
