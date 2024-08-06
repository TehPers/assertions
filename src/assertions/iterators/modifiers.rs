use crate::{assertions::AssertionContext, metadata::Annotated, AssertionResult};

use super::MergeableResult;

/// Executes an assertion on every value within the subject, and succeeds if and
/// only if none of the assertions fail.
///
/// ```
/// # use expecters::prelude::*;
/// expect!([1, 3, 5], all, to_be_less_than(10));
/// expect!([] as [i32; 0], all, to_equal(1));
/// ```
///
/// This method panics if any element does not satisfy the assertion:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!([1, 3, 5], all, to_equal(5));
/// ```
#[inline]
pub fn all<T, O>(
    cx: AssertionContext,
    subject: T,
    next: fn(AssertionContext, T::Item) -> O,
) -> O::Merged
where
    T: IntoIterator,
    O: MergeableResult,
{
    O::merge_all(
        cx.clone(),
        subject.into_iter().enumerate().map(|(idx, item)| {
            let mut cx = cx.clone();
            cx.annotate("index", idx);
            next(cx, item)
        }),
    )
}

/// Executes an assertion on every value within the subject, and succeeds if and
/// only if an assertion succeeds.
///
/// ```
/// # use expecters::prelude::*;
/// expect!([1, 3, 5], any, to_equal(5));
/// expect!([] as [i32; 0], not, any, to_equal(1));
/// ```
///
/// This method panics if any element does not satisfy the assertion:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!([1, 3, 5], any, to_equal(4));
/// ```
#[inline]
pub fn any<T, O>(
    cx: AssertionContext,
    subject: T,
    next: fn(AssertionContext, T::Item) -> O,
) -> O::Merged
where
    T: IntoIterator,
    O: MergeableResult,
{
    O::merge_any(
        cx.clone(),
        subject.into_iter().enumerate().map(|(idx, item)| {
            let mut cx = cx.clone();
            cx.annotate("index", idx);
            next(cx, item)
        }),
    )
}

/// Counts the length of the subject, and executes an assertion on the result.
///
/// This uses the [`Iterator::count`] method to determine the number of
/// elements in the target. If the target is an unbounded iterator, then
/// this method will loop indefinitely.
///
/// ```
/// # use expecters::prelude::*;
/// expect!([1, 2, 3], count, to_equal(3));
/// ```
///
/// This method panics if the number of elements does not satisfy the
/// assertion:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!([1, 2, 3], count, to_equal(4));
/// ```
#[inline]
pub fn count<T, O>(
    mut cx: AssertionContext,
    subject: T,
    next: fn(AssertionContext, usize) -> O,
) -> O
where
    T: IntoIterator,
{
    let count = subject.into_iter().count();
    cx.annotate("count", count);
    next(cx, count)
}

/// Applies an assertion to a specific element in the target. If the element
/// does not exist or does not satisfy the assertion, then the result is
/// treated as a failure. The index is zero-based.
///
/// ```
/// # use expecters::prelude::*;
/// expect!([1, 2, 3], nth(1), to_equal(2));
/// ```
///
/// This method panics if the element does not exist:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!([1, 2, 3], nth(3), to_equal(4));
/// ```
///
/// It also panics if the element does not satisfy the assertion:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!([1, 2, 3], nth(1), to_equal(1));
/// ```
#[inline]
pub fn nth<T>(
    idx: Annotated<usize>,
) -> impl FnOnce(AssertionContext, T, fn(AssertionContext, T::Item) -> AssertionResult) -> AssertionResult
where
    T: IntoIterator,
{
    move |mut cx, subject, next| {
        cx.annotate("index", idx.as_str());
        let item = subject
            .into_iter()
            .nth(idx.into_inner())
            .ok_or_else(|| cx.fail("index out of bounds"))?;
        next(cx, item)
    }
}
