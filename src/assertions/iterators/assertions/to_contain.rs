use crate::{
    assertions::{Assertion, AssertionContext},
    metadata::Annotated,
    AssertionOutput,
};

/// Asserts that the subject contains an expected element.
#[derive(Clone, Debug)]
pub struct ToContain<U> {
    expected: Annotated<U>,
}

impl<U> ToContain<U> {
    #[inline]
    pub(crate) fn new(expected: Annotated<U>) -> Self {
        Self { expected }
    }
}

impl<U, T> Assertion<T> for ToContain<U>
where
    T: IntoIterator<Item: PartialEq<U>>,
{
    type Output = AssertionOutput;

    fn execute(self, mut cx: AssertionContext, subject: T) -> Self::Output {
        cx.annotate("expected", &self.expected);
        cx.pass_if(
            subject
                .into_iter()
                .any(|item| &item == self.expected.inner()),
            "value not found",
        )
    }
}
