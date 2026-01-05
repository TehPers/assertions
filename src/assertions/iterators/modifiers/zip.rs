use std::iter::Zip;

use crate::{
    assertions::{
        general::IntoInitializableOutput, Assertion, AssertionContext, AssertionContextBuilder,
        AssertionModifier,
    },
    metadata::Annotated,
};

/// Zips two iterators together.
///
/// See [`Iterator::zip()`] for details on zipping iterators.
#[derive(Clone, Debug)]
pub struct ZipModifier<I, M> {
    prev: M,
    other: Annotated<I>,
}

impl<I, M> ZipModifier<I, M> {
    #[inline]
    pub(crate) fn new(prev: M, other: Annotated<I>) -> Self {
        Self { prev, other }
    }
}

impl<I, M, A> AssertionModifier<A> for ZipModifier<I, M>
where
    M: AssertionModifier<ZipAssertion<I, A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
        self.prev.apply(
            cx,
            ZipAssertion {
                next,
                other: self.other,
            },
        )
    }
}

/// Zips two iterators together and executes the inner assertion on the result.
#[derive(Clone, Debug)]
pub struct ZipAssertion<I, A> {
    next: A,
    other: Annotated<I>,
}

impl<I, A, T> Assertion<T> for ZipAssertion<I, A>
where
    I: IntoIterator,
    T: IntoIterator,
    A: Assertion<Zip<T::IntoIter, I::IntoIter>, Output: IntoInitializableOutput>,
{
    type Output = <A::Output as IntoInitializableOutput>::Initialized;

    #[inline]
    fn execute(self, mut cx: AssertionContext, subject: T) -> Self::Output {
        cx.annotate("other", &self.other);

        let subject = subject.into_iter().zip(self.other.into_inner());
        self.next.execute(cx, subject).into_initialized()
    }
}
