use std::fmt::{Display, Formatter};

use crate::{AssertionFailure, AssertionResult};

use super::{Assertion, AssertionCombinator};

/// Wraps another [`AssertionCombinator`] and negates the output. The overall
/// assertion succeeds if and only if the chained assertion fails.
#[derive(Clone, Debug)]
pub struct NotCombinator<Inner> {
    inner: Inner,
}

impl<Inner> NotCombinator<Inner> {
    /// Creates a new instance of this combinator, wrapping the inner
    /// combinator.
    #[inline]
    pub fn new(inner: Inner) -> Self {
        Self { inner }
    }
}

impl<Inner, Next> AssertionCombinator<Next> for NotCombinator<Inner>
where
    Inner: AssertionCombinator<NotAssertion<Next>>,
    Next: Assertion<Inner::Target>,
{
    type Target = Inner::Target;
    type NextTarget = Inner::Target;
    type Assertion = Inner::Assertion;

    fn apply(self, next: Next) -> Self::Assertion {
        self.inner.apply(NotAssertion::new(next))
    }
}

/// Wraps another assertion and inverts the result, passing if and only if the
/// inner assertion failed and failing if it passes.
#[derive(Clone, Debug, Default)]
pub struct NotAssertion<Next> {
    next: Next,
}

impl<Next> NotAssertion<Next> {
    /// Creates a new instance of this assertion.
    #[inline]
    pub fn new(next: Next) -> Self {
        NotAssertion { next }
    }
}

impl<Next> Display for NotAssertion<Next>
where
    Next: Display,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "the following is not satisfied: {}", self.next)
    }
}

impl<Target, Next> Assertion<Target> for NotAssertion<Next>
where
    Next: Assertion<Target, Output = AssertionResult>,
{
    type Output = Next::Output;

    fn assert(self, target: Target) -> Self::Output {
        let result = self.next.assert(target);
        match result {
            Ok(_) => Err(AssertionFailure::builder().build("TODO")),
            Err(_) => Ok(()),
        }
    }
}
