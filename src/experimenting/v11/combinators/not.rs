use crate::{assertions::Assertion, AssertionFailure, AssertionResult};

use super::AssertionCombinator;

#[derive(Clone, Debug)]
pub struct NotCombinator<Inner> {
    inner: Inner,
}

impl<Inner> NotCombinator<Inner> {
    simple_ctor!(new(inner: Inner));
}

impl<Inner, A> AssertionCombinator<A> for NotCombinator<Inner>
where
    Inner: AssertionCombinator<NotAssertion<A>>,
    A: Assertion<Inner::NextTarget>,
{
    type NextTarget = Inner::NextTarget;
    type Output = Inner::Output;

    #[inline]
    fn apply(self, assertion: A) -> Self::Output {
        self.inner.apply(NotAssertion::new(assertion))
    }
}

#[derive(Clone, Debug)]
pub struct NotAssertion<Inner> {
    inner: Inner,
}

impl<Inner> NotAssertion<Inner> {
    simple_ctor!(new(inner: Inner));
}

impl<Inner, T> Assertion<T> for NotAssertion<Inner>
where
    Inner: Assertion<T>,
{
    #[inline]
    fn execute(&mut self, target: T) -> AssertionResult {
        match self.inner.execute(target) {
            Ok(_) => Err(AssertionFailure::builder().build("TODO")),
            Err(_) => Ok(()),
        }
    }
}
