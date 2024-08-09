use crate::assertions::{
    general::IntoInitializableOutput, key, results::Resultish, Assertion, AssertionContext,
    AssertionModifier, SubjectKey,
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
pub fn to_be_ok_and<R, M>(prev: M, _: SubjectKey<R>) -> (OkAndModifier<M>, SubjectKey<R::OutT>)
where
    R: Resultish,
{
    (OkAndModifier { prev }, key())
}

/// Modifier for [`to_be_ok_and()`].
#[derive(Clone, Debug)]
pub struct OkAndModifier<M> {
    prev: M,
}

impl<M, A> AssertionModifier<A> for OkAndModifier<M>
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
    A: Assertion<R::OutT, Output: IntoInitializableOutput>,
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
