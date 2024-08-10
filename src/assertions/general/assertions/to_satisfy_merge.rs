use crate::{
    assertions::{
        iterators::{MergeStrategy, MergeableOutput},
        Assertion, AssertionContext,
    },
    metadata::Annotated,
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
///     to_satisfy_all(|value| [
///         try_expect!(value, to_be_greater_than(0)),
///         try_expect!(value, to_be_less_than(4)),
///     ]),
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
///     to_satisfy_all(|value| [
///         try_expect!(value, to_be_greater_than(3)),
///     ]),
/// );
/// ```
#[inline]
#[must_use]
pub fn to_satisfy_all<F>(predicates: Annotated<F>) -> ToSatisfyMergeAssertion<F> {
    ToSatisfyMergeAssertion {
        predicates,
        strategy: MergeStrategy::All,
    }
}

/// Asserts that the subject matches any of the given predicates. This "forks"
/// the assertion, allowing an intermediate value to have several different
/// assertions applied to it.
///
/// ```
/// # use expecters::prelude::*;
/// expect!(
///     [1, 2, 3],
///     count,
///     to_satisfy_any(|value| [
///         try_expect!(value, to_be_greater_than(0)),
///         try_expect!(value, to_be_less_than(0)),
///     ]),
/// );
/// ```
///
/// The assertion fails if none of the results were successes:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!(
///     [1, 2, 3],
///     count,
///     to_satisfy_any(|value| [
///         try_expect!(value, to_be_greater_than(3)),
///     ]),
/// );
/// ```
#[inline]
#[must_use]
pub fn to_satisfy_any<F>(predicates: Annotated<F>) -> ToSatisfyMergeAssertion<F> {
    ToSatisfyMergeAssertion {
        predicates,
        strategy: MergeStrategy::Any,
    }
}

/// Assertion for [`to_satisfy_all()`] and [`to_satisfy_any()`].
#[derive(Clone, Debug)]
pub struct ToSatisfyMergeAssertion<F> {
    predicates: Annotated<F>,
    strategy: MergeStrategy,
}

impl<F, T, R> Assertion<T> for ToSatisfyMergeAssertion<F>
where
    F: FnOnce(T) -> R,
    R: IntoIterator<Item: MergeableOutput>,
{
    type Output = <R::Item as MergeableOutput>::Merged;

    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        // TODO: allow result contexts to be "added" to cx so failure messages
        // show the full execution path and not just the child path
        let outputs = (self.predicates.into_inner())(subject);
        MergeableOutput::merge(cx, self.strategy, outputs)
    }
}

#[cfg(test)]
mod tests {
    use crate::{prelude::*, AssertionOutput};

    #[test]
    fn vacuous() {
        expect!(1, to_satisfy_all(|_| -> [AssertionOutput; 0] { [] }));
        expect!(1, not, to_satisfy_any(|_| -> [AssertionOutput; 0] { [] }));
    }
}

#[cfg(all(test, feature = "futures"))]
mod async_tests {
    use std::future::ready;

    use crate::prelude::*;

    #[tokio::test]
    async fn test_async_all() {
        // Outer async
        expect!(
            ready([1, 2, 3]),
            when_ready,
            to_satisfy_all(|values| [try_expect!(values, count, to_equal(3))]),
        )
        .await;
        expect!(
            ready([1, 2, 3]),
            when_ready,
            not,
            to_satisfy_all(|values| [try_expect!(values, count, to_equal(4))]),
        )
        .await;

        // Nested async
        expect!(
            ready([1, 2, 3]),
            to_satisfy_all(|values| [try_expect!(values, when_ready, count, to_equal(3))]),
        )
        .await;
    }

    #[tokio::test]
    async fn test_async_any() {
        // Outer async
        expect!(
            ready([1, 2, 3]),
            when_ready,
            to_satisfy_any(|values| [try_expect!(values, count, to_equal(3))]),
        )
        .await;
        expect!(
            ready([1, 2, 3]),
            when_ready,
            not,
            to_satisfy_any(|values| [try_expect!(values, count, to_equal(4))]),
        )
        .await;

        // Nested async
        expect!(
            ready([1, 2, 3]),
            to_satisfy_any(|values| [try_expect!(values, when_ready, count, to_equal(3))]),
        )
        .await;
    }
}
