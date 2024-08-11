use crate::{
    assertions::{Assertion, AssertionContext, AssertionError},
    metadata::Annotated,
    AssertionOutput,
};

/// Asserts that the subject matches all of the given predicates. This "forks"
/// the assertion, allowing an intermediate value to have several different
/// assertions applied to it.
///
/// ```
/// # use expecters::prelude::*;
/// expect!(
///     [1, 2, 3],
///     count,
///     to_satisfy_with(|value| {
///         try_expect!(value, to_be_greater_than(0))?;
///         try_expect!(value, to_be_less_than(4))?;
///         Ok(())
///     }),
/// );
/// ```
///
/// The assertion fails if any of the results were failures:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!(
///     [1, 2, 3],
///     count,
///     to_satisfy_with(|value| {
///         try_expect!(value, to_be_greater_than(3))?;
///         Ok(())
///     }),
/// );
/// ```
///
/// This does not work with nested async assertions.
// TODO: make an async version
#[inline]
#[must_use]
pub fn to_satisfy_with<F>(predicate: Annotated<F>) -> ToSatisfyMergeAssertion<F> {
    ToSatisfyMergeAssertion { predicate }
}

/// Assertion for [`to_satisfy_with()`].
#[derive(Clone, Debug)]
pub struct ToSatisfyMergeAssertion<F> {
    predicate: Annotated<F>,
}

impl<F, T> Assertion<T> for ToSatisfyMergeAssertion<F>
where
    F: FnOnce(T) -> Result<(), AssertionError>,
{
    type Output = AssertionOutput;

    #[inline]
    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        // TODO: allow error context to be "added" to cx so failure messages
        // show the full execution path and not just the child path
        let result = (self.predicate.into_inner())(subject);
        cx.pass_if(result.is_ok(), "inner assertions failed")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn vacuous() {
        expect!(1, to_satisfy_with(|_| Ok(())));
    }
}
