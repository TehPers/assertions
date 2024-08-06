use std::fmt::{Display, Formatter};

use crate::{AssertionFailure, AssertionResult};

use super::{Assertion, AssertionCombinator};

/// Wraps another [`AssertionCombinator`] and ensures the assertion succeeds for
/// some inner value.
#[derive(Clone, Debug)]
pub struct AnyCombinator<Inner> {
    inner: Inner,
}

impl<Inner> AnyCombinator<Inner> {
    /// Creates a new instance of this combinator, wrapping the inner
    /// combinator.
    #[inline]
    pub fn new(inner: Inner) -> Self {
        Self { inner }
    }
}

impl<Inner, Next> AssertionCombinator<Next> for AnyCombinator<Inner>
where
    Inner: AssertionCombinator<AnyAssertion<Next>>,
    Inner::NextTarget: IntoIterator,
{
    type Target = Inner::Target;
    type NextTarget = <Inner::NextTarget as IntoIterator>::Item;
    type Assertion = Inner::Assertion;

    fn apply(self, next: Next) -> Self::Assertion {
        self.inner.apply(AnyAssertion::new(next))
    }
}

/// Wraps another assertion and ensures that the inner assertion passes for at
/// least one value in the provided target.
#[derive(Clone, Debug, Default)]
pub struct AnyAssertion<Next> {
    next: Next,
}

impl<Next> AnyAssertion<Next> {
    /// Creates a new instance of this assertion.
    #[inline]
    pub fn new(next: Next) -> Self {
        AnyAssertion { next }
    }
}

impl<Next> Display for AnyAssertion<Next>
where
    Next: Display,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "for each item, {}", self.next)
    }
}

impl<Next, Target> Assertion<Target> for AnyAssertion<Next>
where
    Target: IntoIterator,
    Next: Assertion<Target::Item, Output = AssertionResult> + Clone,
{
    type Output = AssertionResult;

    fn assert(self, target: Target) -> Self::Output {
        let mut error = AssertionFailure::builder().build("TODO");
        for value in target {
            match self.next.clone().assert(value) {
                Ok(()) => return Ok(()),
                Err(e) => error = e,
            }
        }

        Err(error)
    }
}
