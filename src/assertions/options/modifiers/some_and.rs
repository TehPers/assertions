use std::marker::PhantomData;

use crate::{
    assertions::{
        key, options::Optionish, Assertion, AssertionContext, AssertionModifier, SubjectKey,
    },
    AssertionResult,
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
pub fn to_be_some_and<O, M>(
    prev: M,
    _: SubjectKey<O>,
) -> (SomeAndModifier<O, M>, SubjectKey<O::OutT>)
where
    O: Optionish,
{
    (
        SomeAndModifier {
            prev,
            marker: PhantomData,
        },
        key(),
    )
}

/// Modifier for [`to_be_some_and()`].
#[derive(Clone, Debug)]
pub struct SomeAndModifier<O, M> {
    prev: M,
    marker: PhantomData<fn(O)>,
}

impl<O, M, A> AssertionModifier<A> for SomeAndModifier<O, M>
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
    A: Assertion<O::OutT, Output = AssertionResult>,
    O: Optionish,
{
    type Output = AssertionResult;

    #[inline]
    fn execute(self, cx: AssertionContext, subject: O) -> Self::Output {
        let Some(subject) = subject.some() else {
            return cx.fail("subject is None");
        };
        self.next.execute(cx, subject)
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
}
