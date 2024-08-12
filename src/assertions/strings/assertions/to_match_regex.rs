use std::sync::Arc;

use regex::Regex;

use crate::{
    assertions::{Assertion, AssertionContext},
    AssertionOutput,
};

/// Asserts that the subject matches a regular expression.
#[derive(Clone, Debug)]
pub struct ToMatchRegexAssertion {
    regex: Arc<Regex>,
}

impl ToMatchRegexAssertion {
    #[inline]
    pub(crate) fn new(pattern: &str) -> Self {
        let regex = Regex::new(pattern).expect("invalid regex");
        Self {
            regex: Arc::new(regex),
        }
    }
}

impl<T> Assertion<T> for ToMatchRegexAssertion
where
    T: AsRef<str>,
{
    type Output = AssertionOutput;

    fn execute(self, mut cx: AssertionContext, subject: T) -> Self::Output {
        cx.annotate("pattern", self.regex.as_str());
        cx.pass_if(
            self.regex.is_match(subject.as_ref()),
            "didn't match pattern",
        )
    }
}
