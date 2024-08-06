use crate::{
    assertions::{AssertionContext, AssertionResult},
    metadata::Annotated,
};

/// Asserts that the subject is equal to the given value.
///
/// ```
/// # use expecters::prelude::*;
/// expect!(1, to_equal(1));
/// ```
///
/// This method panics if the target is not equal to the given value:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!(1, to_equal(2));
/// ```
#[inline]
pub fn to_equal<T, U>(expected: Annotated<U>) -> impl FnOnce(AssertionContext, T) -> AssertionResult
where
    T: PartialEq<U>,
{
    move |mut cx, subject| {
        cx.annotate("expected", expected.as_str());

        if subject == expected.into_inner() {
            Ok(())
        } else {
            Err(cx.fail("subject is not equal to value"))
        }
    }
}

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
pub fn to_be_less_than<T, U>(
    boundary: Annotated<U>,
) -> impl FnOnce(AssertionContext, T) -> AssertionResult
where
    T: PartialOrd<U>,
{
    move |mut cx, subject| {
        cx.annotate("boundary", &boundary);

        if subject < boundary.into_inner() {
            Ok(())
        } else {
            Err(cx.fail("subject is not less than value"))
        }
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
pub fn to_be_less_than_or_equal_to<T, U>(
    boundary: Annotated<U>,
) -> impl FnOnce(AssertionContext, T) -> AssertionResult
where
    T: PartialOrd<U>,
{
    move |mut cx, subject| {
        cx.annotate("boundary", &boundary);

        if subject <= boundary.into_inner() {
            Ok(())
        } else {
            Err(cx.fail("subject is not less than or equal to value"))
        }
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
pub fn to_be_greater_than<T, U>(
    boundary: Annotated<U>,
) -> impl FnOnce(AssertionContext, T) -> AssertionResult
where
    T: PartialOrd<U>,
{
    move |mut cx, subject| {
        cx.annotate("boundary", &boundary);

        if subject > boundary.into_inner() {
            Ok(())
        } else {
            Err(cx.fail("subject is not greater than value"))
        }
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
pub fn to_be_greater_than_or_equal_to<T, U>(
    boundary: Annotated<U>,
) -> impl FnOnce(AssertionContext, T) -> AssertionResult
where
    T: PartialOrd<U>,
{
    move |mut cx, subject| {
        cx.annotate("boundary", &boundary);

        if subject >= boundary.into_inner() {
            Ok(())
        } else {
            Err(cx.fail("subject is not greater than or equal to value"))
        }
    }
}
