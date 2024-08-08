use crate::{
    assertions::{Assertion, AssertionContext},
    metadata::Annotated,
    AssertionResult,
};

/// Asserts that the subject is equal to the given value.
///
/// ```
/// # use expecters::prelude::*;
/// expect!(1, to_equal(1));
/// ```
///
/// The assertion fails if the subject is not equal to the given value:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!(1, to_equal(2));
/// ```
#[inline]
pub fn to_equal<U>(expected: Annotated<U>) -> ToEqualAssertion<U> {
    ToEqualAssertion { expected }
}

/// Assertion for [`to_equal`].
#[derive(Clone, Debug)]
pub struct ToEqualAssertion<U> {
    expected: Annotated<U>,
}

impl<T, U> Assertion<T> for ToEqualAssertion<U>
where
    T: PartialEq<U>,
{
    type Output = AssertionResult;

    #[inline]
    fn execute(self, mut cx: AssertionContext, value: T) -> Self::Output {
        cx.annotate("expected", &self.expected);
        cx.pass_if(value == self.expected.into_inner(), "values not equal")
    }
}
