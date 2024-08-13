use crate::{
    assertions::{Assertion, AssertionContext},
    metadata::Annotated,
    AssertionOutput,
};

/// Asserts that the subject is equal to an expected value.
#[derive(Clone, Debug)]
pub struct ToEqualAssertion<U> {
    expected: Annotated<U>,
}

impl<U> ToEqualAssertion<U> {
    #[inline]
    pub(crate) fn new(expected: Annotated<U>) -> Self {
        Self { expected }
    }
}

impl<T, U> Assertion<T> for ToEqualAssertion<U>
where
    T: PartialEq<U>,
{
    type Output = AssertionOutput;

    #[inline]
    fn execute(self, mut cx: AssertionContext, value: T) -> Self::Output {
        cx.annotate("expected", &self.expected);
        cx.pass_if(value == self.expected.into_inner(), "values not equal")
    }
}
