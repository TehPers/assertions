use crate::assertions::{Assertion, AssertionContext, AssertionContextBuilder, AssertionModifier};

/// Counts the number of items in a subject.
#[derive(Clone, Debug)]
pub struct CountModifier<M> {
    prev: M,
}

impl<M> CountModifier<M> {
    #[inline]
    pub(crate) fn new(prev: M) -> CountModifier<M> {
        CountModifier { prev }
    }
}

impl<M, A> AssertionModifier<A> for CountModifier<M>
where
    M: AssertionModifier<CountAssertion<A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
        self.prev.apply(cx, CountAssertion { next })
    }
}

/// Executes the inner assertion on the number of items in the subject.
#[derive(Clone, Debug)]
pub struct CountAssertion<A> {
    next: A,
}

impl<A, T> Assertion<T> for CountAssertion<A>
where
    A: Assertion<usize>,
    T: IntoIterator,
{
    type Output = A::Output;

    #[inline]
    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        self.next.execute(cx, subject.into_iter().count())
    }
}
