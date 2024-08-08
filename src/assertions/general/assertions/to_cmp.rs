use std::cmp::Ordering;

use crate::{
    assertions::{Assertion, AssertionContext},
    metadata::Annotated,
    AssertionResult,
};

/// Asserts that the target is less than the given value.
///
/// ```
/// # use expecters::prelude::*;
/// expect!(1, to_be_less_than(2));
/// ```
///
/// This method panics if the target is not less than the given value:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!(2, to_be_less_than(1));
/// ```
#[inline]
pub fn to_be_less_than<U>(boundary: Annotated<U>) -> ToCmpAssertion<U> {
    ToCmpAssertion {
        boundary,
        ordering: Ordering::Less,
        allow_eq: false,
        cmp_message: "less than",
    }
}

/// Asserts that the target is less than or equal to the given value.
///
/// ```
/// # use expecters::prelude::*;
/// expect!(1, to_be_less_than_or_equal_to(1));
/// expect!(1, to_be_less_than_or_equal_to(2));
/// ```
///
/// This method panics if the target is greater less the given value:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!(2, to_be_less_than_or_equal_to(1));
/// ```
#[inline]
pub fn to_be_less_than_or_equal_to<U>(boundary: Annotated<U>) -> ToCmpAssertion<U> {
    ToCmpAssertion {
        boundary,
        ordering: Ordering::Less,
        allow_eq: true,
        cmp_message: "less than or equal to",
    }
}

/// Asserts that the target is greater than the given value.
///
/// ```
/// # use expecters::prelude::*;
/// expect!(2, to_be_greater_than(1));
/// ```
///
/// This method panics if the target is not greater than the given value:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!(1, to_be_greater_than(2));
/// ```
#[inline]
pub fn to_be_greater_than<U>(boundary: Annotated<U>) -> ToCmpAssertion<U> {
    ToCmpAssertion {
        boundary,
        ordering: Ordering::Greater,
        allow_eq: false,
        cmp_message: "greater than",
    }
}

/// Asserts that the target is greater than or equal to the given value.
///
/// ```
/// # use expecters::prelude::*;
/// expect!(1, to_be_greater_than_or_equal_to(1));
/// expect!(1, to_be_greater_than_or_equal_to(0));
/// ```
///
/// This method panics if the target is less than than the given value:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!(1, to_be_greater_than_or_equal_to(2));
/// ```
#[inline]
pub fn to_be_greater_than_or_equal_to<U>(boundary: Annotated<U>) -> ToCmpAssertion<U> {
    ToCmpAssertion {
        boundary,
        ordering: Ordering::Greater,
        allow_eq: true,
        cmp_message: "greater than or equal to",
    }
}

/// A general-purpose assertion for comparing the ordering between two values.
#[derive(Clone, Debug)]
pub struct ToCmpAssertion<U> {
    boundary: Annotated<U>,
    ordering: Ordering,
    allow_eq: bool,
    cmp_message: &'static str,
}

impl<T, U> Assertion<T> for ToCmpAssertion<U>
where
    T: PartialOrd<U>,
{
    type Output = AssertionResult;

    fn execute(self, mut cx: AssertionContext, subject: T) -> Self::Output {
        cx.annotate("boundary", &self.boundary);
        cx.annotate(
            "ordering",
            format_args!("{:?}", subject.partial_cmp(self.boundary.inner())),
        );
        cx.annotate("expected ordering", self.cmp_message);

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
        cx.pass_if(
            success,
            format_args!("subject not {} boundary", self.cmp_message),
        )
    }
}
