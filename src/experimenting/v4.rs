use std::{
    fmt::{self, Display, Formatter},
    future::{ready, Future},
    marker::PhantomData,
    pin::Pin,
    sync::Arc,
};

#[derive(Debug, Default)]
pub struct AssertionFailure {
    fields: Vec<(String, String)>,
}

/// A type which builds an assertion to execute on a value.
pub trait AssertionCombinator: Sized {
    type NextInput;

    type Assertion<A>
    where
        A: Assertion<Self::NextInput>;

    /// Builds an assertion using this combinator.
    fn build<A>(self, assertion: A) -> Self::Assertion<A>
    where
        A: Assertion<Self::NextInput>;

    fn not(self) -> NotCombinator<Self> {
        NotCombinator { prev: self }
    }

    fn all(self) -> AllCombinator<Self> {
        AllCombinator { prev: self }
    }
}

/// Performs a validation on a value. The [`Display`] implementation should
/// output the predicate this assertion expects to be true of the value.
pub trait Assertion<Input>: Display + Sized {
    type Output;

    /// Execute the assertion on a target value.
    fn assert(self, target: Input) -> Self::Output;
}

#[derive(Clone, Debug)]
pub struct SimpleAssertion<F> {
    expectation: Arc<str>,
    predicate: F,
}

impl<F> SimpleAssertion<F> {
    pub fn new(expectation: impl ToString, predicate: F) -> Self {
        Self {
            expectation: expectation.to_string().into(),
            predicate,
        }
    }
}

impl<F> Display for SimpleAssertion<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expectation)
    }
}

impl<F, I, O> Assertion<I> for SimpleAssertion<F>
where
    F: FnOnce(I) -> O,
{
    type Output = O;

    fn assert(self, target: I) -> Self::Output {
        (self.predicate)(target)
    }
}

/////////

#[derive(Clone, Debug)]
pub struct AssertionRoot<T> {
    target: T,
}

impl<T> AssertionCombinator for AssertionRoot<T> {
    type NextInput = T;

    type Assertion<A> = RootAssertion<T, A>
    where
        A: Assertion<Self::NextInput>;

    fn build<A>(self, assertion: A) -> Self::Assertion<A>
    where
        A: Assertion<Self::NextInput>,
    {
        RootAssertion {
            target: self.target,
            next: assertion,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RootAssertion<T, Next> {
    target: T,
    next: Next,
}

impl<T, Next> Display for RootAssertion<T, Next>
where
    Next: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.next)
    }
}

impl<T, Next> Assertion<()> for RootAssertion<T, Next>
where
    Next: Assertion<T>,
{
    type Output = Next::Output;

    fn assert(self, _target: ()) -> Self::Output {
        self.next.assert(self.target)
    }
}

#[derive(Clone, Debug)]
pub struct NotCombinator<Prev> {
    prev: Prev,
}

impl<Prev> AssertionCombinator for NotCombinator<Prev>
where
    Prev: AssertionCombinator,
{
    type NextInput = Prev::NextInput;

    type Assertion<A> = Prev::Assertion<NotAssertion<A>>
    where
        A: Assertion<Self::NextInput>;

    fn build<A>(self, assertion: A) -> Self::Assertion<A>
    where
        A: Assertion<Self::NextInput>,
    {
        NotAssertion(self.prev.build(assertion))
    }
}

fn a() {
    let assertion = NotCombinator {
        prev: AssertionRoot { target: 1 },
    }
    .build(SimpleAssertion::new("not zero", |value| value != 0));

    let _result = assertion.assert(());
}

#[derive(Clone, Debug)]
pub struct NotAssertion<Next>(Next);

impl<Next> Display for NotAssertion<Next>
where
    Next: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "the following to not be satisfied: {}", self.0)
    }
}

impl<Next, Input> Assertion<Input> for NotAssertion<Next>
where
    Next: Assertion<Input, Output = Result<(), AssertionFailure>>,
{
    type Output = Next::Output;

    fn assert(self, target: Input) -> Self::Output {
        match self.0.assert(target) {
            Ok(_) => Err(AssertionFailure::default()),
            Err(_) => todo!(),
        }
    }
}

pub struct AllCombinator<Prev> {
    prev: Prev,
}

impl<Prev> AssertionCombinator for AllCombinator<Prev>
where
    Prev: AssertionCombinator,
    Prev::NextInput: IntoIterator,
{
    type NextInput = <Prev::NextInput as IntoIterator>::Item;

    type Assertion<A> = AllAssertion<A>
    where
        A: Assertion<Self::NextInput>;

    fn build<A>(self, assertion: A) -> Self::Assertion<A>
    where
        A: Assertion<Self::NextInput>,
    {
        self.prev.build(AllAssertion { next: assertion })
    }
}

#[derive(Clone, Debug)]
pub struct AllAssertion<Next> {
    next: Next,
}

impl<I, Next> Display for AllAssertion<I, Next>
where
    Next: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl<I, Next> Assertion for AllAssertion<I, Next>
where
    I: IntoIterator,
    Next: Assertion<Input = I::Item> + Clone,
    Result<(), AssertionFailure>: FromIterator<Next::Output>,
{
    type Input = I;
    type Output = Result<(), AssertionFailure>;

    fn assert(self, target: Self::Input) -> Self::Output {
        target
            .into_iter()
            .map(|target| self.next.clone().assert(target))
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct MapAssertion<Next, I, O> {
    next: Next,
    f: fn(I) -> O,
}

impl<Next, I, O> Display for MapAssertion<Next, I, O>
where
    Next: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl<Next, I, O> Assertion for MapAssertion<Next, I, O>
where
    Next: Assertion<Input = O>,
{
    type Input = I;
    type Output = Next::Output;

    fn assert(self, target: Self::Input) -> Self::Output {
        self.next.assert((self.f)(target))
    }
}

#[derive(Clone, Debug)]
pub struct WhenReadyAssertion<I, Next> {
    next: Next,
    marker: PhantomData<fn(I)>,
}

impl<I, Next> Display for WhenReadyAssertion<I, Next>
where
    Next: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl<I, Next> Assertion for WhenReadyAssertion<I, Next>
where
    I: Future + Send + 'static,
    Next: Assertion<Input = I::Output> + Send + 'static,
{
    type Input = I;
    type Output = Pin<Box<dyn Future<Output = Next::Output> + Send>>;

    fn assert(self, target: Self::Input) -> Self::Output {
        Box::pin(async move { self.next.assert(target.await) })
    }
}

async fn foo() {
    let assertion = RootAssertion {
        target: ready([1, 2, 3]),
        next: WhenReadyAssertion {
            marker: PhantomData,
            next: AllAssertion {
                marker: PhantomData,
                next: SimpleAssertion::new("non-zero", |value| {
                    if value == 0 {
                        Err(AssertionFailure::default())
                    } else {
                        Ok(())
                    }
                }),
            },
        },
    };
    let result = assertion.assert(()).await;

    let assertion = RootAssertion {
        target: [1, 2, 3],
        next: AllAssertion {
            marker: PhantomData,
            next: SimpleAssertion::new("non-zero", |value| {
                if value == 0 {
                    Err(AssertionFailure::default())
                } else {
                    Ok(())
                }
            }),
        },
    };
    let result = assertion.assert(());

    let combinator = AllCombinator {
        marker: PhantomData,
        prev: AssertionRoot { target: [1, 2, 3] },
    };
    let assertion = combinator.build(SimpleAssertion::new("non-zero", |value| {
        if value == 0 {
            Err(AssertionFailure::default())
        } else {
            Ok(())
        }
    }));
    let result = assertion.assert(());
}
