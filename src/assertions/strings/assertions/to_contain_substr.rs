use crate::{
    assertions::{Assertion, AssertionContext},
    metadata::Annotated,
    AssertionOutput,
};

/// Asserts that the subject contains the given substring.
#[derive(Clone, Debug)]
pub struct ToContainSubstr<P> {
    pattern: Annotated<P>,
    location: ContainsLocation,
}

impl<P> ToContainSubstr<P> {
    #[inline]
    pub(crate) fn new(pattern: Annotated<P>, location: ContainsLocation) -> Self {
        Self { pattern, location }
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

        let subject = subject.as_ref();
        let found = match self.location {
            ContainsLocation::Anywhere => subject.contains(pattern),
            ContainsLocation::Start => subject.starts_with(pattern),
            ContainsLocation::End => subject.ends_with(pattern),
        };
        cx.pass_if(found, "substring not found")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum ContainsLocation {
    Anywhere,
    Start,
    End,
}
