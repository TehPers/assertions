use std::fmt::{Display, Formatter};

use crate::AssertionResult;

use super::{Assertion, AssertionCombinator};

/// Wraps another [`AssertionCombinator`] and ensures the assertion succeeds for
/// every inner value.
#[derive(Clone, Debug)]
pub struct AllCombinator<Inner> {
    inner: Inner,
}

impl<Inner> AllCombinator<Inner> {
    /// Creates a new instance of this combinator, wrapping the inner
    /// combinator.
    #[inline]
    pub fn new(inner: Inner) -> Self {
        Self { inner }
    }
}

impl<Inner, Next> AssertionCombinator<Next> for AllCombinator<Inner>
where
    Inner: AssertionCombinator<AllAssertion<Next>>,
    Inner::NextTarget: IntoIterator,
{
    type Target = Inner::Target;
    type NextTarget = <Inner::NextTarget as IntoIterator>::Item;
    type Assertion = Inner::Assertion;

    #[inline]
    fn apply(self, next: Next) -> Self::Assertion {
        self.inner.apply(AllAssertion::new(next))
    }
}

/// Wraps another assertion and ensures that the inner assertion does not fail
/// for any value in the provided target.
#[derive(Clone, Debug, Default)]
pub struct AllAssertion<Next> {
    next: Next,
}

impl<Next> AllAssertion<Next> {
    /// Creates a new instance of this assertion.
    #[inline]
    pub fn new(next: Next) -> Self {
        Self { next }
    }
}

impl<Next> Display for AllAssertion<Next>
where
    Next: Display,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "for each item, {}", self.next)
    }
}

impl<Next, Target> Assertion<Target> for AllAssertion<Next>
where
    Target: IntoIterator,
    Next: Assertion<Target::Item, Output = AssertionResult> + Clone,
{
    type Output = AssertionResult;

    fn assert(self, target: Target) -> Self::Output {
        target
            .into_iter()
            .map(move |target| self.next.clone().assert(target))
            .collect()
    }
}

// TODO: make an "AllOutput"-style trait so this doesn't need to have the output
// bound, which would work similar to the previous `AssertionOutput` trait but
// only for the `.all()` combinator
