use crate::assertions::{Assertion, AssertionContext, AssertionContextBuilder, AssertionModifier};

/// Converts the subject to its characters.
#[derive(Clone, Debug)]
pub struct CharsModifier<M> {
    prev: M,
}

impl<M> CharsModifier<M> {
    #[inline]
    pub(crate) fn new(prev: M) -> Self {
        CharsModifier { prev }
    }
}

impl<M, A> AssertionModifier<A> for CharsModifier<M>
where
    M: AssertionModifier<CharsAssertion<A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
        self.prev.apply(cx, CharsAssertion { next })
    }
}

/// Executes the inner assertion with the characters in the subject.
#[derive(Clone, Debug)]
pub struct CharsAssertion<A> {
    next: A,
}

impl<A, T> Assertion<T> for CharsAssertion<A>
where
    A: Assertion<Vec<char>>,
    T: AsRef<str>,
{
    type Output = A::Output;

    #[inline]
    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        self.next.execute(cx, subject.as_ref().chars().collect())
    }
}
