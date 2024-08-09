use crate::assertions::{key, Assertion, AssertionContext, AssertionModifier, SubjectKey};

/// Counts the length of the subject, and executes an assertion on the result.
///
/// ```
/// # use expecters::prelude::*;
/// expect!([1, 2, 3], count, to_equal(3));
/// ```
///
/// This uses the [`Iterator::count`] method to determine the number of elements
/// in the subject. If the subject is an unbounded iterator, then the assertion
/// will not complete (unless it panics for another reason). See the iterator
/// method for more information.
#[inline]
pub fn count<T, M>(prev: M, _: SubjectKey<T>) -> (CountModifier<M>, SubjectKey<usize>) {
    (CountModifier { prev }, key())
}

/// Modifier for [`count()`].
#[derive(Clone, Debug)]
pub struct CountModifier<M> {
    prev: M,
}

impl<M, A> AssertionModifier<A> for CountModifier<M>
where
    M: AssertionModifier<CountAssertion<A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, next: A) -> Self::Output {
        self.prev.apply(CountAssertion { next })
    }
}

/// Assertion for [`count()`].
#[derive(Clone, Debug)]
pub struct CountAssertion<A> {
    next: A,
}

impl<A, T> Assertion<T> for CountAssertion<A>
where
    A: Assertion<usize>,
    T: IntoIterator,
{
    type Output = A::Output;

    #[inline]
    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        self.next.execute(cx, subject.into_iter().count())
    }
}
