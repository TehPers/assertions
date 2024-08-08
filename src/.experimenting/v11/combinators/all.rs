use crate::{assertions::Assertion, AssertionResult};

use super::AssertionCombinator;

#[derive(Clone, Debug)]
pub struct AllCombinator<Inner> {
    inner: Inner,
}

impl<Inner> AllCombinator<Inner> {
    simple_ctor!(new(inner: Inner));
}

impl<Inner, A> AssertionCombinator<A> for AllCombinator<Inner>
where
    Inner: AssertionCombinator<AllAssertion<A>>,
    Inner::NextTarget: IntoIterator,
    A: Assertion<<Inner::NextTarget as IntoIterator>::Item>,
{
    type NextTarget = <Inner::NextTarget as IntoIterator>::Item;
    type Output = Inner::Output;

    #[inline]
    fn apply(self, assertion: A) -> Self::Output {
        self.inner.apply(AllAssertion::new(assertion))
    }
}

#[derive(Clone, Debug)]
pub struct AllAssertion<Inner> {
    inner: Inner,
}

impl<Inner> AllAssertion<Inner> {
    simple_ctor!(new(inner: Inner));
}

impl<Inner, T> Assertion<T> for AllAssertion<Inner>
where
    T: IntoIterator,
    Inner: Assertion<T::Item>,
{
    #[inline]
    fn execute(&mut self, target: T) -> AssertionResult {
        target
            .into_iter()
            .map(|value| self.inner.execute(value))
            .collect()
    }
}
