use std::{future::Future, marker::PhantomData};

use crate::assertions::{
    futures::WhenReadyFuture, key, Assertion, AssertionContext, AssertionModifier, SubjectKey,
};

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
/// expect!(
///     ready(ready(1)),
///     when_ready, // outer future
///     when_ready, // inner future
///     to_equal(1)
/// )
/// .await
/// .await;
/// # })
/// ```
#[inline]
pub fn when_ready<T, M>(
    prev: M,
    _: SubjectKey<T>,
) -> (WhenReadyModifier<T, M>, SubjectKey<T::Output>)
where
    T: Future,
{
    (
        WhenReadyModifier {
            prev,
            marker: PhantomData,
        },
        key(),
    )
}

/// Modifier for [`when_ready()`].
#[derive(Clone, Debug)]
pub struct WhenReadyModifier<T, M> {
    prev: M,
    marker: PhantomData<fn(T)>,
}

impl<T, M, A> AssertionModifier<A> for WhenReadyModifier<T, M>
where
    M: AssertionModifier<WhenReadyAssertion<A>>,
{
    type Output = M::Output;

    fn apply(self, next: A) -> Self::Output {
        self.prev.apply(WhenReadyAssertion { next })
    }
}

/// Assertion for [`when_ready()`].
#[derive(Clone, Debug)]
pub struct WhenReadyAssertion<A> {
    next: A,
}

impl<A, T> Assertion<T> for WhenReadyAssertion<A>
where
    T: Future,
    A: Assertion<T::Output>,
{
    type Output = WhenReadyFuture<T, A>;

    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        WhenReadyFuture::new(cx, subject, self.next)
    }
}
