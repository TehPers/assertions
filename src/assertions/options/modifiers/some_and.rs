use crate::assertions::{
    general::IntoInitializableOutput, key, options::Optionish, Assertion, AssertionContext,
    AssertionModifier, SubjectKey,
};

/// Asserts that the subject holds a value, then continues the assertion with
/// the contained value.
///
/// ```
/// # use expecters::prelude::*;
/// expect!(Some(1), to_be_some_and, to_equal(1));
/// ```
///
/// The assertion fails if the option is [`None`]:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!(None::<i32>, to_be_some_and, to_equal(2));
/// ```
#[inline]
pub fn to_be_some_and<O, M>(prev: M, _: SubjectKey<O>) -> (SomeAndModifier<M>, SubjectKey<O::OutT>)
where
    O: Optionish,
{
    (SomeAndModifier { prev }, key())
}

/// Modifier for [`to_be_some_and()`].
#[derive(Clone, Debug)]
pub struct SomeAndModifier<M> {
    prev: M,
}

impl<M, A> AssertionModifier<A> for SomeAndModifier<M>
where
    M: AssertionModifier<SomeAndAssertion<A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, next: A) -> Self::Output {
        self.prev.apply(SomeAndAssertion { next })
    }
}

/// Assertion for [`to_be_some_and()`].
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
