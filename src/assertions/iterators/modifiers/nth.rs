use crate::{
    assertions::{
        general::IntoInitializableOutput, Assertion, AssertionContext, AssertionContextBuilder,
        AssertionModifier,
    },
    metadata::Annotated,
};

/// Selects an element out of the subject.
#[derive(Clone, Debug)]
pub struct NthModifier<M> {
    prev: M,
    index: Annotated<usize>,
}

impl<M> NthModifier<M> {
    #[inline]
    pub(crate) fn new(prev: M, index: Annotated<usize>) -> Self {
        Self { prev, index }
    }
}

impl<M, A> AssertionModifier<A> for NthModifier<M>
where
    M: AssertionModifier<NthAssertion<A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
        self.prev.apply(
            cx,
            NthAssertion {
                next,
                index: self.index,
            },
        )
    }
}

/// Selects an element out of the subject and executes the inner assertion on
/// it.
#[derive(Clone, Debug)]
pub struct NthAssertion<A> {
    next: A,
    index: Annotated<usize>,
}

impl<A, T> Assertion<T> for NthAssertion<A>
where
    A: Assertion<T::Item, Output: IntoInitializableOutput>,
    T: IntoIterator,
{
    type Output = <A::Output as IntoInitializableOutput>::Initialized;

    #[inline]
    fn execute(self, mut cx: AssertionContext, subject: T) -> Self::Output {
        cx.annotate("index", self.index);

        let index = self.index.into_inner();
        let Some(subject) = subject.into_iter().nth(index) else {
            return cx.fail("index out of bounds");
        };
        self.next.execute(cx, subject).into_initialized()
    }
}

#[cfg(all(test, feature = "futures"))]
mod async_tests {
    use std::future::ready;

    use crate::prelude::*;

    #[tokio::test]
    async fn nested_async_works() {
        expect!([ready(1)], nth(0), when_ready, to_equal(1)).await;
    }
}
