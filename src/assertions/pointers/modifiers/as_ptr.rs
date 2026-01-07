use crate::assertions::{
    pointers::PointerLike, Assertion, AssertionContext, AssertionContextBuilder, AssertionModifier,
};

/// Converts the subject to a raw pointer (`*const T`).
#[derive(Clone, Debug)]
pub struct AsPtrModifier<M> {
    prev: M,
}

impl<M> AsPtrModifier<M> {
    #[inline]
    pub(crate) fn new(prev: M) -> Self {
        Self { prev }
    }
}

impl<M, A> AssertionModifier<A> for AsPtrModifier<M>
where
    M: AssertionModifier<AsPtrAssertion<A>>,
{
    type Output = M::Output;

    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
        self.prev.apply(cx, AsPtrAssertion { next })
    }
}

/// Converts the subject to a raw pointer and executes the inner assertion on
/// it.
#[derive(Clone, Debug)]
pub struct AsPtrAssertion<A> {
    next: A,
}

impl<A, T> Assertion<T> for AsPtrAssertion<A>
where
    A: Assertion<*const T::Target>,
    T: PointerLike,
{
    type Output = A::Output;

    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        let subject = T::as_ptr(&subject);
        self.next.execute(cx, subject)
    }
}
