use std::{
    fmt::{Display, Formatter},
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};

use pin_project_lite::pin_project;

use crate::{AssertionOutput, AssertionResult};

use super::{Assertion, AssertionCombinator};

/// Wraps another [`AssertionCombinator`] and executes assertions on the output
/// of the inner value's future.
///
/// This causes assertions to be asynchronous. Assertion outputs from this
/// combinator will be wrapped in futures, meaning the assertions must be
/// `.await`ed.
#[derive(Clone, Debug)]
pub struct WhenReadyCombinator<Inner> {
    inner: Inner,
}

impl<Inner> WhenReadyCombinator<Inner> {
    /// Creates a new instance of this combinator, wrapping the inner
    /// combinator.
    #[inline]
    pub fn new(inner: Inner) -> Self {
        Self { inner }
    }
}

// impl<Inner> AssertionCombinator for WhenReadyCombinator<Inner>
// where
//     Inner: AssertionCombinator,
//     Inner::Target: Future,
// {
//     type Target = <Inner::Target as Future>::Output;

//     type Assertion<R, F> = WhenReadyOutput<Inner::Target, Box<dyn FnMut(<Inner::Target as Future>::Output) -> R>>
//     where
//         R: AssertionOutput,
//         F: FnMut(Self::Target) -> R;

//     fn to_satisfy<R, F>(self, expectation: impl Display, assert: F) -> Self::Assertion<R, F>
//     where
//         R: AssertionOutput,
//         F: FnMut(Self::Target) -> R,
//     {
//         // Since the assertion is passed to potentially multiple futures, we
//         // need to wrap it in an Arc<Mutex<T>> to ensure that only one of those
//         // futures actually executes the assertion at a time.
//         let assert = Arc::new(Mutex::new(assert));

//         self.inner.to_satisfy(
//             format!("when the future is ready, {expectation}"),
//             move |fut| {
//                 let assert = assert.clone();
//                 WhenReadyOutput::evaluated(
//                     fut,
//                     Box::new(move |value| {
//                         let mut assert = assert.lock().unwrap();
//                         assert(value)
//                     })
//                         as Box<dyn FnMut(<Inner::Target as Future>::Output) -> R>,
//                 )
//             },
//         )
//     }
// }

impl<Inner, Next> AssertionCombinator<Next> for WhenReadyCombinator<Inner>
where
    Inner: AssertionCombinator<WhenReadyAssertion<Next>>,
    Inner::Target: Future,
{
    type Target = Inner::Target;

    type Assertion = Inner::Assertion;

    fn apply(self, next: Next) -> Self::Assertion {
        self.inner.apply(WhenReadyAssertion::new(next))
    }
}

pub struct WhenReadyAssertion<Next> {
    next: Next,
}

impl<Next> WhenReadyAssertion<Next> {
    pub fn new(next: Next) -> Self {
        WhenReadyAssertion { next }
    }
}

impl<Next> Display for WhenReadyAssertion<Next>
where
    Next: Display,
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "when the future is ready, {}", self.next)
    }
}

impl<Next, Target> Assertion<Target> for WhenReadyAssertion<Next>
where
    Target: Future,
    Next: Assertion<Target::Output>,
{
    type Output = WhenReadyOutput<Target, Next>;

    fn assert(self, target: Target) -> Self::Output {
        WhenReadyOutput::evaluated(target, self.next)
    }
}

pin_project! {
    /// An assertion output that will resolve to a success or an error
    /// eventually. This is usually created when performing an assertion on the
    /// output of a future.
    #[derive(Clone, Debug)]
    #[must_use]
    pub struct WhenReadyOutput<Fut, A> {
        // TODO: is the double-pin needed?
        #[pin]
        inner: WhenReadyOutputInner<Fut, A>
    }
}

impl<Fut, A> WhenReadyOutput<Fut, A> {
    #[inline]
    fn evaluated(fut: Fut, map: A) -> Self {
        Self {
            inner: WhenReadyOutputInner::Evaluated {
                fut,
                map: Some(map),
            },
        }
    }

    #[inline]
    fn forced(result: AssertionResult) -> Self {
        Self {
            inner: WhenReadyOutputInner::Forced {
                result: Some(result),
            },
        }
    }
}

pin_project! {
    #[project = WhenReadyOutputInnerProj]
    #[derive(Clone, Debug)]
    enum WhenReadyOutputInner<Fut, A> {
        Forced {
            result: Option<AssertionResult>,
        },
        Evaluated {
            #[pin]
            fut: Fut,
            map: Option<A>,
        },
    }
}

// impl<Fut, A, R> Future for WhenReadyOutput<Fut, A>
// where
//     Fut: Future,
//     A: FnOnce(Fut::Output) -> R,
//     R: AssertionOutput,
// {
//     type Output = R;

//     fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
//         let projected = self.project();
//         let projected = projected.inner.project();

//         match projected {
//             WhenReadyOutputInnerProj::Forced { result } => {
//                 let result = result.take().expect("poll after ready");
//                 Poll::Ready(match result {
//                     Ok(()) => R::new_success(),
//                     Err(failure) => R::new_failure(failure),
//                 })
//             }
//             WhenReadyOutputInnerProj::Evaluated { fut, assert } => {
//                 let output = ready!(fut.poll(cx));
//                 let assert = assert.take().expect("polled after ready");
//                 Poll::Ready(assert(output))
//             }
//         }
//     }
// }

impl<Fut, M, R> Future for WhenReadyOutput<Fut, M>
where
    Fut: Future,
    M: FnOnce(Fut::Output) -> R,
    R: AssertionOutput,
{
    type Output = R;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let projected = self.project();
        let projected = projected.inner.project();

        match projected {
            WhenReadyOutputInnerProj::Forced { result } => {
                let result = result.take().expect("poll after ready");
                Poll::Ready(match result {
                    Ok(()) => Self::Output::new_success(),
                    Err(failure) => Self::Output::new_failure(failure),
                })
            }
            WhenReadyOutputInnerProj::Evaluated { fut, map } => {
                let output = ready!(fut.poll(cx));
                let map = map.take().expect("polled after ready");
                Poll::Ready(map(output))
            }
        }
    }
}

// impl<Fut, A> AssertionOutput for WhenReadyOutput<Fut, A>
// where
//     Fut: Future,
//     A: Assertion<Fut::Output>,
// {
//     type Mapped<F> = WhenReadyOutput<
//         Self,
//         SimpleAssertion<
//             A::Output,
//             <A::Output as AssertionOutput>::Mapped<F>
//         >
//     >
//     where
//         F: FnMut(AssertionResult) -> AssertionResult;

//     type AndThen<O, F>
//     where
//         O: AssertionOutput,
//         F: FnOnce() -> O;

//     type OrElse<O, F>
//     where
//         O: AssertionOutput,
//         F: FnOnce() -> O;

//     type All<I>
//     where
//         I: IntoIterator<Item = Self>;

//     type Any<I>
//     where
//         I: IntoIterator<Item = Self>;

//     #[inline]
//     fn new_success() -> Self {
//         Self::forced(Ok(()))
//     }

//     #[inline]
//     fn new_failure(failure: AssertionFailure) -> Self {
//         Self::forced(Err(failure))
//     }

//     #[inline]
//     fn map<F>(self, f: F) -> Self::Mapped<F>
//     where
//         F: FnMut(AssertionResult) -> AssertionResult,
//     {
//         WhenReadyOutput::evaluated(
//             self,
//             SimpleAssertion::new("", move |output: A::Output| output.map(f)),
//         )
//     }

//     fn and_then<O, F>(self, other: F) -> Self::AndThen<O, F>
//     where
//         O: AssertionOutput,
//         F: FnOnce() -> O,
//     {
//         todo!()
//     }

//     fn or_else<O, F>(self, other: F) -> Self::OrElse<O, F>
//     where
//         O: AssertionOutput,
//         F: FnOnce() -> O,
//     {
//         todo!()
//     }

//     fn all<I>(outputs: I) -> Self::All<I>
//     where
//         I: IntoIterator<Item = Self>,
//     {
//         todo!()
//     }

//     fn any<I>(outputs: I) -> Self::Any<I>
//     where
//         I: IntoIterator<Item = Self>,
//     {
//         todo!()
//     }

//     // #[inline]
//     // fn map<F>(self, f: F) -> impl AssertionOutput
//     // where
//     //     F: FnMut(AssertionResult) -> AssertionResult,
//     // {
//     //     WhenReadyOutput::evaluated(self, move |output: R| output.map(f))
//     // }

//     // #[inline]
//     // fn and_then<O, F>(self, other: F) -> impl AssertionOutput
//     // where
//     //     O: AssertionOutput,
//     //     F: FnOnce() -> O,
//     // {
//     //     WhenReadyOutput::evaluated(self, move |output: <Self as Future>::Output| {
//     //         output.and_then(other)
//     //     })
//     // }

//     // #[inline]
//     // fn or_else<O, F>(self, other: F) -> impl AssertionOutput
//     // where
//     //     O: AssertionOutput,
//     //     F: FnOnce() -> O,
//     // {
//     //     WhenReadyOutput::evaluated(self, move |output: <Self as Future>::Output| {
//     //         output.or_else(other)
//     //     })
//     // }

//     // #[inline]
//     // fn all(outputs: impl IntoIterator<Item = Self>) -> impl AssertionOutput {
//     //     // It would be nice to support short-circuiting here, but we need all
//     //     // the outputs ready at once to pass them to `R::all`
//     //     let stream: FuturesUnordered<_> = outputs.into_iter().collect();
//     //     WhenReadyOutput::evaluated(stream.collect::<Vec<_>>(), R::all)
//     // }

//     // #[inline]
//     // fn any(outputs: impl IntoIterator<Item = Self>) -> impl AssertionOutput {
//     //     // It would be nice to support short-circuiting here, but we need all
//     //     // the outputs ready at once to pass them to `R::any`
//     //     let stream: FuturesUnordered<_> = outputs.into_iter().collect();
//     //     WhenReadyOutput::evaluated(stream.collect::<Vec<_>>(), R::any)
//     // }
// }
