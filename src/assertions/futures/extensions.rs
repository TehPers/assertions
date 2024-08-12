use std::future::Future;

use crate::{assertions::AssertionBuilder, metadata::Annotated};

use super::{CompletionOrder, CompletionOrderModifier, WhenReadyModifier};

/// Assertions and modifiers for [Future]s.
pub trait FutureAssertions<T, M>
where
    T: Future,
{
    /// Executes an assertion on the output of a future.
    ///
    /// When the subject is ready, the assertion is executed on the output of the
    /// subject. This makes the assertion asynchronous, so it must be awaited or
    /// passed to an executor in order for it to actually perform the assertion.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// use std::future::ready;
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// expect!(ready(1), when_ready, to_equal(1)).await;
    /// # }
    /// ```
    ///
    /// Note that this can be chained multiple times if needed, but each level of
    /// nesting requires an additional `.await`:
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// use std::future::ready;
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// expect!(
    ///     ready(ready(1)),
    ///     when_ready, // outer future
    ///     when_ready, // inner future
    ///     to_equal(1)
    /// )
    /// .await
    /// .await;
    /// # }
    /// ```
    fn when_ready(self) -> AssertionBuilder<T::Output, WhenReadyModifier<M>>;

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
    fn when_ready_before<Fut>(
        self,
        other: Annotated<Fut>,
    ) -> AssertionBuilder<T::Output, CompletionOrderModifier<Fut, M>>
    where
        Fut: Future;

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
    fn when_ready_after<Fut>(
        self,
        other: Annotated<Fut>,
    ) -> AssertionBuilder<T::Output, CompletionOrderModifier<Fut, M>>
    where
        Fut: Future;
}

impl<T, M> FutureAssertions<T, M> for AssertionBuilder<T, M>
where
    T: Future,
{
    #[inline]
    fn when_ready(self) -> AssertionBuilder<T::Output, WhenReadyModifier<M>> {
        AssertionBuilder::modify(self, WhenReadyModifier::new)
    }

    #[inline]
    fn when_ready_before<Fut>(
        self,
        other: Annotated<Fut>,
    ) -> AssertionBuilder<T::Output, CompletionOrderModifier<Fut, M>>
    where
        Fut: Future,
    {
        AssertionBuilder::modify(self, move |prev| {
            CompletionOrderModifier::new(prev, other, CompletionOrder::Before)
        })
    }

    #[inline]
    fn when_ready_after<Fut>(
        self,
        other: Annotated<Fut>,
    ) -> AssertionBuilder<T::Output, CompletionOrderModifier<Fut, M>>
    where
        Fut: Future,
    {
        AssertionBuilder::modify(self, move |prev| {
            CompletionOrderModifier::new(prev, other, CompletionOrder::After)
        })
    }
}
