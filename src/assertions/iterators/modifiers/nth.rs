use std::marker::PhantomData;

use crate::{
    assertions::{key, Assertion, AssertionContext, AssertionModifier, SubjectKey},
    metadata::Annotated,
    AssertionResult,
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
    index: Annotated<usize>,
) -> impl FnOnce(M, SubjectKey<T>) -> (NthModifier<T, M>, SubjectKey<T::Item>)
where
    T: IntoIterator,
{
    move |prev, _| {
        (
            NthModifier {
                prev,
                index,
                marker: PhantomData,
            },
            key(),
        )
    }
}

/// Modifier for [`nth()`].
#[derive(Clone, Debug)]
pub struct NthModifier<T, M> {
    prev: M,
    index: Annotated<usize>,
    marker: PhantomData<fn(T)>,
}

impl<T, M, A> AssertionModifier<A> for NthModifier<T, M>
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
    A: Assertion<T::Item, Output = AssertionResult>,
    T: IntoIterator,
{
    type Output = AssertionResult;

    #[inline]
    fn execute(self, mut cx: AssertionContext, subject: T) -> Self::Output {
        cx.annotate("index", &self.index);

        let index = self.index.into_inner();
        let Some(subject) = subject.into_iter().nth(index) else {
            return cx.fail("index out of bounds");
        };
        self.next.execute(cx, subject)
    }
}
