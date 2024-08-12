use crate::{
    assertions::{Assertion, AssertionContext},
    metadata::Annotated,
    AssertionOutput,
};

/// Asserts that the subject contains the given substring.
#[derive(Clone, Debug)]
pub struct ToContainSubstr<P> {
    pattern: Annotated<P>,
}

impl<P> ToContainSubstr<P> {
    #[inline]
    pub(crate) fn new(pattern: Annotated<P>) -> Self {
        Self { pattern }
    }
}

impl<P, T> Assertion<T> for ToContainSubstr<P>
where
    P: AsRef<str>,
    T: AsRef<str>,
{
    type Output = AssertionOutput;

    fn execute(self, mut cx: AssertionContext, subject: T) -> Self::Output {
        let pattern = self.pattern.inner().as_ref();
        cx.annotate("expected", format_args!("{pattern:?}"));
        cx.pass_if(subject.as_ref().contains(pattern), "substring not found")
    }
}
