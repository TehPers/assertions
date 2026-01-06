use crate::assertions::{Assertion, AssertionContext, AssertionContextBuilder, AssertionModifier};

/// Calls the subject.
#[derive(Clone, Debug)]
pub struct WhenCalledModifier<M> {
    prev: M,
}

impl<M> WhenCalledModifier<M> {
    #[inline]
    pub(crate) fn new(prev: M) -> Self {
        Self { prev }
    }
}

impl<M, A> AssertionModifier<A> for WhenCalledModifier<M>
where
    M: AssertionModifier<WhenCalledAssertion<A>>,
{
    type Output = M::Output;

    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
        self.prev.apply(cx, WhenCalledAssertion { next })
    }
}

/// Calls the subject and executes the inner assertion on the returned value.
#[derive(Clone, Debug)]
pub struct WhenCalledAssertion<A> {
    next: A,
}

impl<A, F, O> Assertion<F> for WhenCalledAssertion<A>
where
    A: Assertion<O>,
    F: FnOnce() -> O,
{
    type Output = A::Output;

    fn execute(self, cx: AssertionContext, subject: F) -> Self::Output {
        let subject = (subject)();
        self.next.execute(cx, subject)
    }
}
