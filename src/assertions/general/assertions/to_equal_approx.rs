use crate::{
    assertions::{Assertion, AssertionContext},
    metadata::Annotated,
    AssertionOutput,
};

/// Asserts that the subject is approximately equal to an expected value.
#[derive(Clone, Debug)]
pub struct ToEqualApprox<T> {
    expected: Annotated<T>,
    max_delta: Annotated<T>,
}

impl<T> ToEqualApprox<T> {
    #[inline]
    pub(crate) fn new(expected: Annotated<T>, max_delta: Annotated<T>) -> Self {
        Self {
            expected,
            max_delta,
        }
    }
}

impl Assertion<f32> for ToEqualApprox<f32> {
    type Output = AssertionOutput;

    fn execute(self, mut cx: AssertionContext, subject: f32) -> Self::Output {
        let expected = self.expected.into_inner();
        let max_delta = self.max_delta.into_inner();
        let range = (expected - max_delta)..=(expected + max_delta);

        cx.annotate("expected", format_args!("{range:?}"));
        cx.pass_if(range.contains(&subject), "out of expected range")
    }
}

impl Assertion<f64> for ToEqualApprox<f64> {
    type Output = AssertionOutput;

    fn execute(self, mut cx: AssertionContext, subject: f64) -> Self::Output {
        let expected = self.expected.into_inner();
        let max_delta = self.max_delta.into_inner();
        let range = (expected - max_delta)..=(expected + max_delta);

        cx.annotate("expected", format_args!("{range:?}"));
        cx.pass_if(range.contains(&subject), "out of expected range")
    }
}

#[doc(hidden)]
pub trait Float {}

impl Float for f32 {}
impl Float for f64 {}
