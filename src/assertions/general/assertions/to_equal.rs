use crate::{
    assertions::{Assertion, AssertionContext},
    diff::fmt_diff,
    metadata::Annotated,
    AssertionOutput,
};

/// Asserts that the subject is equal to an expected value.
#[derive(Clone, Debug)]
pub struct ToEqual<U> {
    expected: Annotated<U>,
}

impl<U> ToEqual<U> {
    #[inline]
    pub(crate) fn new(expected: Annotated<U>) -> Self {
        Self { expected }
    }
}

impl<T, U> Assertion<T> for ToEqual<U>
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

        // Skip the diff if the representations aren't multiline to avoid
        // cluttering the output
        if subject_repr.contains('\n') || expected_repr.contains('\n') {
            // Perform the diff
            if let Some(diff) = fmt_diff(&expected_repr, &subject_repr) {
                cx.add_page("diff", diff);
            }
        }

        cx.fail("values not equal")
    }

    #[inline]
    fn execute(self, mut cx: AssertionContext, subject: T) -> Self::Output {
        cx.annotate("expected", &self.expected);
        cx.pass_if(subject == self.expected.into_inner(), "values not equal")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn no_diff() {
        // Don't show diffs for short values
        expect!(
            try_expect!(1, to_equal(2)),
            to_be_err_and,
            as_display,
            not,
            to_contain_substr("diff"),
        );
    }

    #[test]
    fn do_diff() {
        // Show diffs for longer values
        expect!(
            try_expect!("abc\ndef", to_equal("abc\ndeg")),
            to_be_err_and,
            as_display,
            to_contain_substr("diff"),
        );
    }
}
