use crate::assertions::{
    general::InvertibleOutput, Assertion, AssertionContext, AssertionContextBuilder,
    AssertionModifier,
};

/// Inverts an assertion.
#[derive(Clone, Debug)]
pub struct NotModifier<M> {
    prev: M,
}

impl<M> NotModifier<M> {
    #[inline]
    pub(crate) fn new(prev: M) -> Self {
        NotModifier { prev }
    }
}

impl<M, A> AssertionModifier<A> for NotModifier<M>
where
    M: AssertionModifier<NotAssertion<A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
        self.prev.apply(cx, NotAssertion { next })
    }
}

/// Inverts an inner assertion.
#[derive(Clone, Debug)]
pub struct NotAssertion<A> {
    next: A,
}

impl<A, T> Assertion<T> for NotAssertion<A>
where
    A: Assertion<T, Output: InvertibleOutput>,
{
    type Output = <A::Output as InvertibleOutput>::Inverted;

    #[inline]
    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        self.next.execute(cx.clone(), subject).invert(cx)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn preserves_context() {
        let res = try_expect!("blah", not, not, to_contain_substr("world"));
        expect!(
            res,
            to_be_err_and,
            as_display,
            to_contain_substr("\"world\"")
        );
    }
}
