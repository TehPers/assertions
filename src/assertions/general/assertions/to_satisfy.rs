use crate::{
    assertions::{Assertion, AssertionContext},
    metadata::Annotated,
    AssertionResult,
};

/// Asserts that the subject matches the given predicate.
///
/// ```
/// # use expecters::prelude::*;
/// expect!(1, to_satisfy(|n| n % 2 == 1));
/// ```
///
/// The assertion fails if the subject does not satisfy the predicate:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!(2, to_satisfy(|n| n % 2 == 1));
/// ```
///
/// Since the predicate that is passed into this function will be included in
/// the failure message if the assertion fails, it is recommended to keep the
/// predicate short and simple to keep failure message readable. If a more
/// complex predicate is needed, it's possible to define a separate function and
/// pass that function in as an argument instead:
///
/// ```
/// # use expecters::prelude::*;
/// fn is_odd(n: i32) -> bool {
///     n % 2 == 1
/// }
///
/// expect!(1, to_satisfy(is_odd));
/// ```
#[inline]
pub fn to_satisfy<F>(predicate: Annotated<F>) -> ToSatisfyAssertion<F> {
    ToSatisfyAssertion { predicate }
}

/// Assertion for [`to_satisfy()`].
#[derive(Clone, Debug)]
pub struct ToSatisfyAssertion<F> {
    predicate: Annotated<F>,
}

impl<F, T> Assertion<T> for ToSatisfyAssertion<F>
where
    F: FnOnce(T) -> bool,
{
    type Output = AssertionResult;

    fn execute(self, mut cx: AssertionContext, subject: T) -> Self::Output {
        cx.annotate("predicate", &self.predicate);
        cx.pass_if(
            (self.predicate.into_inner())(subject),
            "subject did not satisfy predicate",
        )
    }
}
