use std::sync::Arc;

use regex::Regex;

use crate::{
    assertions::{Assertion, AssertionContext},
    metadata::Annotated,
    AssertionOutput,
};

/// Asserts that the subject matches the given regular expression.
///
/// ```
/// # use expecters::prelude::*;
/// expect!("12345", to_match_regex(r"\d+"));
/// ```
///
/// The assertion fails if the subject does not match the pattern:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!("abcde", to_match_regex(r"\d+"));
/// ```
pub fn to_match_regex<P>(pattern: Annotated<P>) -> ToMatchRegexAssertion
where
    P: AsRef<str>,
{
    let pattern = pattern.inner().as_ref();
    let regex = Regex::new(pattern.as_ref()).expect("invalid regex");
    ToMatchRegexAssertion {
        regex: Arc::new(regex),
    }
}

/// Assertion for [`to_match_regex()`].
#[derive(Clone, Debug)]
pub struct ToMatchRegexAssertion {
    regex: Arc<Regex>,
}

impl<T> Assertion<T> for ToMatchRegexAssertion
where
    T: AsRef<str>,
{
    type Output = AssertionOutput;

    fn execute(self, mut cx: AssertionContext, subject: T) -> Self::Output {
        cx.annotate("pattern", &self.regex.as_str());

        cx.pass_if(
            self.regex.is_match(subject.as_ref()),
            "didn't match pattern",
        )
    }
}
