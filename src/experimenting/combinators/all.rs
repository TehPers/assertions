use std::{
    fmt::Display,
    future::{ready, Future},
    pin::Pin,
};

use crate::Assertable;

/// Wraps an [`Assertable`] and applies the assertion to each element in the
/// target. If there exists an element that fails the chained assertion, then
/// then the whole assertion fails.
///
/// This is similar to [`AnyCombinator`](crate::combinators::AnyCombinator),
/// but every element needs to satisfy the expectation.
#[derive(Clone, Debug)]
pub struct AllCombinator<Inner> {
    inner: Inner,
}

impl<Inner> AllCombinator<Inner> {
    /// Creates a new combinator which wraps an inner [`Assertable`].
    #[inline]
    pub fn new(inner: Inner) -> Self {
        Self { inner }
    }
}

impl<Inner> Assertable for AllCombinator<Inner>
where
    Inner: Assertable,
    Inner::Target: IntoIterator,
{
    type Target = <Inner::Target as IntoIterator>::Item;
    type Result = Inner::Result;

    fn to_satisfy<F>(self, expectation: impl Display, mut f: F) -> Self::Result
    where
        F: FnMut(Self::Target) -> bool,
    {
        self.inner.to_satisfy(
            format_args!("for each inner value, {expectation}"),
            |values| values.into_iter().all(|value| f(value)),
        )
    }
}

////////////

pub trait Assertion2<Input> {
    type Output;

    /// Executes this assertion. This takes an input, performs some kind of
    /// transformation on it, then produces a new output.
    fn execute(self, input: Input) -> Self::Output;
}

impl<F, I, O> Assertion2<I> for F
where
    F: FnOnce(I) -> O,
{
    type Output = O;

    /// Executes this assertion. This takes an input, performs some kind of
    /// transformation on it, then produces a new output.
    fn execute(self, input: I) -> Self::Output {
        self(input)
    }
}

pub trait Combinator2<A> {
    type Output;

    /// Applies this combinator, passing an input into the given assertion and
    /// returning the transformed output.
    fn apply(self, assertion: A) -> Self::Output;
}

struct Count<Prev> {
    prev: Prev,
}

impl<Prev, A> Combinator2<A> for Count<Prev>
where
    Prev: Combinator2<CountAssertion<A>>,
{
    type Output = Prev::Output;

    fn apply(self, assertion: A) -> Self::Output {
        self.prev.apply(CountAssertion { next: assertion })
    }
}

struct CountAssertion<Next> {
    next: Next,
}

impl<Next, Input> Assertion2<Input> for CountAssertion<Next>
where
    Input: IntoIterator,
    Next: Assertion2<usize>,
{
    type Output = Next::Output;

    fn execute(self, input: Input) -> Self::Output {
        self.next.execute(input.into_iter().count())
    }
}

struct WhenReady<Prev> {
    prev: Prev,
}

impl<Prev, A> Combinator2<A> for WhenReady<Prev>
where
    Prev: Combinator2<WhenReadyAssertion<A>>,
{
    type Output = Prev::Output;

    fn apply(self, assertion: A) -> Self::Output {
        self.prev.apply(WhenReadyAssertion { next: assertion })
    }
}

struct WhenReadyAssertion<Next> {
    next: Next,
}

impl<Next, Input> Assertion2<Input> for WhenReadyAssertion<Next>
where
    Input: Future + Send + 'static,
    Next: Assertion2<<Input as Future>::Output> + Send + 'static,
{
    type Output = Pin<
        Box<dyn Future<Output = <Next as Assertion2<<Input as Future>::Output>>::Output> + Send>,
    >;

    fn execute(self, input: Input) -> Self::Output {
        Box::pin(async move {
            let input = input.await;
            self.next.execute(input)
        })
    }
}

struct Root<T> {
    target: T,
}

impl<T, A> Combinator2<A> for Root<T>
where
    A: Assertion2<T>,
{
    type Output = A::Output;

    fn apply(self, assertion: A) -> Self::Output {
        assertion.execute(self.target)
    }
}

async fn foo() {
    let root = Root { target: [1, 2, 3] };
    let combinator = Count { prev: root };
    let _result = combinator.apply(|count| count > 0);

    let assertion = WhenReadyAssertion {
        next: CountAssertion {
            next: |count| count > 0,
        },
    };
    let _result = assertion.execute(ready([1, 2, 3])).await;

    let root = Root {
        target: ready([1i32, 2, 3]),
    };
    let combinator = Count {
        prev: WhenReady { prev: root },
    };
    let _result = combinator.apply(|len| len > 0).await;
}
