use std::panic::{catch_unwind, UnwindSafe};

use crate::{
    assertions::{Assertion, AssertionContext},
    AssertionOutput,
};

/// Asserts that the subject function panics when called.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ToPanic {}

impl ToPanic {
    #[inline]
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<F, O> Assertion<F> for ToPanic
where
    F: FnOnce() -> O + UnwindSafe,
{
    type Output = AssertionOutput;

    #[inline]
    fn execute(self, cx: AssertionContext, subject: F) -> Self::Output {
        let result = catch_unwind(subject);
        cx.pass_if(result.is_err(), "did not panic")
    }
}
