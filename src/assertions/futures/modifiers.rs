use std::future::Future;

use crate::assertions::AssertionContext;

use super::WhenReadyFuture;

/// Executes an assertion on the output of a future.
///
/// When the subject is ready, the assertion is executed on the output of the
/// subject. This makes the assertion asynchronous, so it must be awaited or
/// passed to an executor in order for it to actually perform the assertion.
///
/// ```
/// # use expecters::prelude::*;
/// use core::future::ready;
/// # futures::executor::block_on(async {
/// expect!(ready(1), when_ready, to_equal(1)).await;
/// # })
/// ```
///
/// Note that this can be chained multiple times if needed, but each level of
/// nesting requires an additional `.await`:
///
/// ```
/// # use expecters::prelude::*;
/// use core::future::ready;
/// # futures::executor::block_on(async {
/// expect!(ready(ready(1)), when_ready, when_ready, to_equal(1)).await.await;
/// # })
/// ```
#[inline]
pub fn when_ready<T, O>(
    cx: AssertionContext,
    subject: T,
    next: fn(AssertionContext, T::Output) -> O,
) -> WhenReadyFuture<T, O>
where
    T: Future,
{
    WhenReadyFuture::new(cx, subject, next)
}
