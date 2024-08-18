use crate::{
    assertions::{Assertion, AssertionContext},
    diff::fmt_diff,
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
    fn execute_annotated(self, mut cx: AssertionContext, subject: Annotated<T>) -> Self::Output
    where
        Self: Sized,
    {
        // Only diff if the values are different
        cx.annotate("expected", &self.expected);
        if subject.inner() == self.expected.inner() {
            return cx.pass();
        }

        // Get string representations of values
        let (subject_repr, expected_repr) = if let Some((subject, expected)) =
            subject.as_display().zip(self.expected.as_display())
        {
            (subject.to_string(), expected.to_string())
        } else if let Some((subject, expected)) = subject.as_debug().zip(self.expected.as_debug()) {
            (format!("{subject:#?}"), format!("{expected:#?}"))
        } else {
            return cx.fail("values not equal");
        };

        // Perform the diff
        if let Some(diff) = fmt_diff(&expected_repr, &subject_repr) {
            cx.add_page("diff", diff);
        }

        cx.fail("values not equal")
    }

    #[inline]
    fn execute(self, mut cx: AssertionContext, subject: T) -> Self::Output {
        cx.annotate("expected", &self.expected);
        cx.pass_if(subject == self.expected.into_inner(), "values not equal")
    }
}
