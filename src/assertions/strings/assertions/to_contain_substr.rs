use crate::{
    assertions::{Assertion, AssertionContext},
    metadata::Annotated,
    AssertionOutput,
};

/// Asserts that the subject contains the given substring.
///
/// ```
/// # use expecters::prelude::*;
/// expect!("Hello, world!", to_contain_substr("world"));
/// ```
///
/// The assertion fails if the subject does not contain the substring:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// // not case-insensitive
/// expect!("Hello, world!", to_contain_substr("WORLD"));
/// ```
#[inline]
pub fn to_contain_substr<P>(pattern: Annotated<P>) -> ToContainSubstr<P>
where
    P: AsRef<str>,
{
    ToContainSubstr { pattern }
}

/// Assertion for [`to_contain_substr()`].
#[derive(Clone, Debug)]
pub struct ToContainSubstr<P> {
    pattern: Annotated<P>,
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
