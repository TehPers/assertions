use std::marker::PhantomData;

use crate::{
    assertions::{
        key, results::Resultish, Assertion, AssertionContext, AssertionModifier, SubjectKey,
    },
    AssertionResult,
};

/// Asserts that the target holds a success, then continues the assertion with
/// the contained value.
///
/// ```
/// # use expecters::prelude::*;
/// let mut subject: Result<i32, &str> = Ok(1);
/// expect!(subject, to_be_ok_and, to_equal(1));
/// ```
///
/// The assertion fails if the result is [`Err`]:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// let subject: Result<i32, &str> = Err("error");
/// expect!(subject, to_be_ok_and, to_equal(1));
/// ```
#[inline]
pub fn to_be_ok_and<R, M>(prev: M, _: SubjectKey<R>) -> (OkAndModifier<R, M>, SubjectKey<R::OutT>)
where
    R: Resultish,
{
    (
        OkAndModifier {
            prev,
            marker: PhantomData,
        },
        key(),
    )
}

/// Modifier for [`to_be_ok_and()`].
#[derive(Clone, Debug)]
pub struct OkAndModifier<R, M> {
    prev: M,
    marker: PhantomData<fn(R)>,
}

impl<R, M, A> AssertionModifier<A> for OkAndModifier<R, M>
where
    M: AssertionModifier<OkAndAssertion<A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, next: A) -> Self::Output {
        self.prev.apply(OkAndAssertion { next })
    }
}

/// Assertion for [`to_be_ok_and()`].
#[derive(Clone, Debug)]
pub struct OkAndAssertion<A> {
    next: A,
}

impl<A, R> Assertion<R> for OkAndAssertion<A>
where
    A: Assertion<R::OutT, Output = AssertionResult>,
    R: Resultish,
{
    type Output = AssertionResult;

    #[inline]
    fn execute(self, cx: AssertionContext, subject: R) -> Self::Output {
        let Some(subject) = subject.ok() else {
            return cx.fail("subject is Err");
        };
        self.next.execute(cx, subject)
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
