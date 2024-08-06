use std::fmt::{Display, Formatter};

use dyn_clone::{clone_trait_object, DynClone};

use crate::AssertionResult;

use super::Assertion;

/// A simple assertion consisting only of a closure and an expectation string.
#[derive(Clone, Debug)]
pub struct SimpleAssertion<F> {
    expectation: String,
    assert: F,
}

impl<F> SimpleAssertion<F> {
    /// Create a new simple assertion.
    #[inline]
    pub fn new(expectation: impl Display, assert: F) -> Self {
        Self {
            expectation: expectation.to_string(),
            assert,
        }
    }
}

impl<F> Display for SimpleAssertion<F> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.expectation.fmt(f)
    }
}

impl<F, Target, Output> Assertion<Target> for SimpleAssertion<F>
where
    F: FnOnce(Target) -> Output,
{
    type Output = Output;

    #[inline]
    fn assert(self, target: Target) -> Self::Output {
        (self.assert)(target)
    }
}

/// A simple, cloneable, boxable assertion function. This is used to create a
/// boxed assertion that can be trivially shared.
pub trait SimpleAssertionFn<Target>: DynClone {
    fn call(self, target: Target) -> AssertionResult;
}

clone_trait_object!(<Target> SimpleAssertionFn<Target>);

impl<Target, F> SimpleAssertionFn<Target> for F
where
    F: (FnOnce(Target) -> AssertionResult) + Clone,
{
    fn call(self, target: Target) -> AssertionResult {
        self(target)
    }
}
