use std::future::Future;

use crate::{
    assertions::{
        futures::{CompletionOrder, CompletionOrderFuture},
        key, Assertion, AssertionContext, AssertionModifier, SubjectKey,
    },
    metadata::Annotated,
};

/// Executes an assertion on the output of a future, but only if it does not
/// complete after another future.
///
/// If the subject completes before or at the same time as the given future,
/// then the rest of the assertion is executed on its output. Otherwise, the
/// assertion fails.
///
/// ```
/// # use expecters::prelude::*;
/// use std::future::{pending, ready};
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// expect!(ready(1), when_ready_before(pending::<()>()), to_equal(1)).await;
/// expect!(ready(1), when_ready_before(ready(())), to_equal(1)).await;
/// # }
/// ```
///
/// The assertion fails if the provided future completes first:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// use std::future::{pending, ready};
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// expect!(pending::<()>(), when_ready_before(ready(1)), to_equal(())).await;
/// # }
/// ```
pub fn when_ready_before<Fut, M, T>(
    fut: Annotated<Fut>,
) -> impl FnOnce(M, SubjectKey<T>) -> (CompletionOrderModifier<Fut, M>, SubjectKey<T::Output>)
where
    Fut: Future,
    T: Future,
{
    move |prev, _| {
        (
            CompletionOrderModifier {
                prev,
                fut,
                order: CompletionOrder::Before,
            },
            key(),
        )
    }
}

/// Executes an assertion on the output of a future, but only if it does not
/// complete before another future.
///
/// If the subject completes after or at the same time as the given future, then
/// the rest of the assertion is executed on its output. Otherwise, the
/// assertion fails.
///
/// ```
/// # use expecters::prelude::*;
/// use std::{future::ready, time::Duration};
/// use tokio::time::sleep;
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// let fut = async {
///     sleep(Duration::from_secs(1)).await;
///     1
/// };
/// expect!(fut, when_ready_after(ready(())), to_equal(1)).await;
/// expect!(ready(1), when_ready_after(ready(())), to_equal(1)).await;
/// # }
/// ```
///
/// The assertion fails if the provided future completes first:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// use std::future::{pending, ready};
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// expect!(ready(1), when_ready_after(pending::<()>()), to_equal(1)).await;
/// # }
/// ```
pub fn when_ready_after<Fut, M, T>(
    fut: Annotated<Fut>,
) -> impl FnOnce(M, SubjectKey<T>) -> (CompletionOrderModifier<Fut, M>, SubjectKey<T::Output>)
where
    Fut: Future,
    T: Future,
{
    move |prev, _| {
        (
            CompletionOrderModifier {
                prev,
                fut,
                order: CompletionOrder::After,
            },
            key(),
        )
    }
}

/// Modifier for [`when_ready_before()`] and [`when_ready_after()`].
#[derive(Clone, Debug)]
pub struct CompletionOrderModifier<Fut, M> {
    prev: M,
    fut: Annotated<Fut>,
    order: CompletionOrder,
}

impl<Fut, M, A> AssertionModifier<A> for CompletionOrderModifier<Fut, M>
where
    M: AssertionModifier<CompletionOrderAssertion<Fut, A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, next: A) -> Self::Output {
        self.prev.apply(CompletionOrderAssertion {
            next,
            fut: self.fut,
            order: self.order,
        })
    }
}

/// Assertion for [`when_ready_before()`] and [`when_ready_after()`].
#[derive(Clone, Debug)]
pub struct CompletionOrderAssertion<Fut, A> {
    next: A,
    fut: Annotated<Fut>,
    order: CompletionOrder,
}

impl<Fut, A, T> Assertion<T> for CompletionOrderAssertion<Fut, A>
where
    Fut: Future,
    A: Assertion<T::Output>,
    T: Future,
{
    type Output = CompletionOrderFuture<Fut, T, A>;

    #[inline]
    fn execute(self, mut cx: AssertionContext, subject: T) -> Self::Output {
        cx.annotate("other", &self.fut);
        CompletionOrderFuture::new(cx, subject, self.fut.into_inner(), self.next, self.order)
    }
}
