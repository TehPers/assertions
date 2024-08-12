use crate::assertions::{
    general::IntoInitializableOutput, options::Optionish, Assertion, AssertionContext,
    AssertionContextBuilder, AssertionModifier,
};

/// Maps the subject to its inner value.
#[derive(Clone, Debug)]
pub struct SomeAndModifier<M> {
    prev: M,
}

impl<M> SomeAndModifier<M> {
    #[inline]
    pub(crate) fn new(prev: M) -> Self {
        Self { prev }
    }
}

impl<M, A> AssertionModifier<A> for SomeAndModifier<M>
where
    M: AssertionModifier<SomeAndAssertion<A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
        self.prev.apply(cx, SomeAndAssertion { next })
    }
}

/// Executes the inner assertion on the subject's inner value.
#[derive(Clone, Debug)]
pub struct SomeAndAssertion<A> {
    next: A,
}

impl<A, O> Assertion<O> for SomeAndAssertion<A>
where
    A: Assertion<O::OutT, Output: IntoInitializableOutput>,
    O: Optionish,
{
    type Output = <A::Output as IntoInitializableOutput>::Initialized;

    #[inline]
    fn execute(self, cx: AssertionContext, subject: O) -> Self::Output {
        let Some(subject) = subject.some() else {
            return cx.fail("received None");
        };
        self.next.execute(cx, subject).into_initialized()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn refs_work() {
        let mut option: Option<i32> = Some(1);
        expect!(&option, to_be_some_and, to_satisfy(|&n| n == 1));
        expect!(&mut option, to_be_some_and, to_satisfy(|&mut n| n == 1));
        expect!(option, to_be_some_and, to_equal(1));

        let mut option: Option<i32> = None;
        expect!(&option, not, to_be_some_and, to_satisfy(|_| true));
        expect!(&mut option, not, to_be_some_and, to_satisfy(|_| true));
        expect!(option, not, to_be_some_and, to_satisfy(|_| true));
    }

    #[cfg(feature = "futures")]
    #[tokio::test]
    async fn nested_async_works() {
        use std::future::ready;

        let result = Some(ready(1));
        expect!(result, to_be_some_and, when_ready, to_equal(1)).await;
    }
}
