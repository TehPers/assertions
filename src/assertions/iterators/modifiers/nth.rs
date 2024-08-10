use crate::{
    assertions::{
        general::IntoInitializableOutput, key, Assertion, AssertionContext, AssertionModifier,
        SubjectKey,
    },
    metadata::Annotated,
};

/// Applies an assertion to a specific element in the target. If the element
/// does not exist or does not satisfy the assertion, then the result is
/// treated as a failure. The index is zero-based.
///
/// ```
/// # use expecters::prelude::*;
/// expect!([1, 2, 3], nth(1), to_equal(2));
/// ```
///
/// The assertion fails if the element does not exist:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!([1, 2, 3], nth(3), to_equal(4));
/// ```
///
/// It also fails if the element does not satisfy the assertion:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!([1, 2, 3], nth(1), to_equal(1));
/// ```
#[inline]
pub fn nth<T, M>(
    prev: M,
    _: SubjectKey<T>,
    index: Annotated<usize>,
) -> (NthModifier<M>, SubjectKey<T::Item>)
where
    T: IntoIterator,
{
    (NthModifier { prev, index }, key())
}

/// Modifier for [`nth()`].
#[derive(Clone, Debug)]
pub struct NthModifier<M> {
    prev: M,
    index: Annotated<usize>,
}

impl<M, A> AssertionModifier<A> for NthModifier<M>
where
    M: AssertionModifier<NthAssertion<A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, next: A) -> Self::Output {
        self.prev.apply(NthAssertion {
            next,
            index: self.index,
        })
    }
}

/// Assertion for [`nth()`].
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
        cx.annotate("index", &self.index);

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
