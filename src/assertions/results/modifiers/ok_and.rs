use crate::assertions::{
    general::IntoInitializableOutput, results::Resultish, Assertion, AssertionContext,
    AssertionContextBuilder, AssertionModifier,
};

/// Maps the subject to its [`Ok`] value.
#[derive(Clone, Debug)]
pub struct OkAndModifier<M> {
    prev: M,
}

impl<M> OkAndModifier<M> {
    #[inline]
    pub(crate) fn new(prev: M) -> Self {
        Self { prev }
    }
}

impl<M, A> AssertionModifier<A> for OkAndModifier<M>
where
    M: AssertionModifier<OkAndAssertion<A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
        self.prev.apply(cx, OkAndAssertion { next })
    }
}

/// Executes the inner assertion on the subject's [`Ok`] value.
#[derive(Clone, Debug)]
pub struct OkAndAssertion<A> {
    next: A,
}

impl<A, R> Assertion<R> for OkAndAssertion<A>
where
    A: Assertion<R::Inner, Output: IntoInitializableOutput>,
    R: Resultish,
{
    type Output = <A::Output as IntoInitializableOutput>::Initialized;

    #[inline]
    fn execute(self, cx: AssertionContext, subject: R) -> Self::Output {
        let Some(subject) = subject.ok() else {
            return cx.fail("received Err");
        };
        self.next.execute(cx, subject).into_initialized()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn refs_work() {
        let mut result: Result<i32, ()> = Ok(1);
        expect!(&result, to_be_ok_and, to_satisfy(|&n| n == 1));
        expect!(&mut result, to_be_ok_and, to_satisfy(|&mut n| n == 1));
        expect!(result, to_be_ok_and, to_equal(1));

        let mut result: Result<i32, ()> = Err(());
        expect!(&result, not, to_be_ok_and, to_satisfy(|_| true));
        expect!(&mut result, not, to_be_ok_and, to_satisfy(|_| true));
        expect!(result, not, to_be_ok_and, to_satisfy(|_| true));
    }
}

#[cfg(all(test, feature = "futures"))]
mod async_tests {
    use std::future::ready;

    use crate::prelude::*;

    #[tokio::test]
    async fn nested_async_works() {
        let result: Result<_, ()> = Ok(ready(1));
        expect!(result, to_be_ok_and, when_ready, to_equal(1)).await;
    }
}
