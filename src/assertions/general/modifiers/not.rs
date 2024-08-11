use crate::assertions::{
    general::InvertibleOutput, key, Assertion, AssertionContext, AssertionModifier, SubjectKey,
};

/// Inverts the result of an assertion.
///
/// If (and only if) the assertion is satisfied, then the result is treated as
/// a failure.
///
/// ```
/// # use expecters::prelude::*;
/// expect!(1, not, to_equal(2));
/// ```
///
/// This method panics if the assertion is satisfied:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!(1, not, to_equal(1));
/// ```
#[inline]
pub fn not<T, M>(prev: M, _: SubjectKey<T>) -> (NotModifier<M>, SubjectKey<T>) {
    (NotModifier { prev }, key())
}

/// Modifier for [`not()`].
#[derive(Clone, Debug)]
pub struct NotModifier<M> {
    prev: M,
}

impl<M, A> AssertionModifier<A> for NotModifier<M>
where
    M: AssertionModifier<NotAssertion<A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, next: A) -> Self::Output {
        self.prev.apply(NotAssertion { next })
    }
}

/// Assertion for [`not()`].
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
            as_debug,
            to_contain_substr(r#""world""#)
        );
    }
}
