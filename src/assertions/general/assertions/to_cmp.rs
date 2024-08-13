use std::cmp::Ordering;

use crate::{
    assertions::{Assertion, AssertionContext},
    metadata::Annotated,
    AssertionOutput,
};

/// A general-purpose assertion for comparing the ordering between two values.
#[derive(Clone, Debug)]
pub struct ToCmpAssertion<U> {
    boundary: Annotated<U>,
    ordering: Ordering,
    allow_eq: bool,
    cmp_message: &'static str,
}

impl<U> ToCmpAssertion<U> {
    #[inline]
    pub(crate) fn new(
        boundary: Annotated<U>,
        ordering: Ordering,
        allow_eq: bool,
        cmp_message: &'static str,
    ) -> Self {
        Self {
            boundary,
            ordering,
            allow_eq,
            cmp_message,
        }
    }
}

impl<T, U> Assertion<T> for ToCmpAssertion<U>
where
    T: PartialOrd<U>,
{
    type Output = AssertionOutput;

    fn execute(self, mut cx: AssertionContext, subject: T) -> Self::Output {
        cx.annotate("boundary", &self.boundary);
        cx.annotate(
            "actual ordering",
            match subject.partial_cmp(self.boundary.inner()) {
                Some(Ordering::Less) => "subject < boundary",
                Some(Ordering::Equal) => "subject == boundary",
                Some(Ordering::Greater) => "subject > boundary",
                None => "none",
            },
        );

        // Use a match here to call the specialized comparison functions in case
        // those functions were overridden for a type
        let boundary = self.boundary.into_inner();
        let success = match (self.ordering, self.allow_eq) {
            (Ordering::Less, true) => subject <= boundary,
            (Ordering::Less, false) => subject < boundary,
            (Ordering::Greater, true) => subject >= boundary,
            (Ordering::Greater, false) => subject > boundary,
            (Ordering::Equal, _) => return cx.fail("use to_equal instead"),
        };
        cx.pass_if(success, format_args!("not {} boundary", self.cmp_message))
    }
}
