use std::{
    fmt::{self, Display, Formatter},
    future::{ready, Future},
    marker::PhantomData,
    pin::Pin,
    sync::Arc,
};

use crate::combinators2::Combinator;

#[derive(Debug, Default)]
pub struct AssertionFailure {
    fields: Vec<(String, String)>,
}

/// A type which builds an assertion to execute on a value.
pub trait AssertionCombinator<A>: Sized
where
    A: Assertion,
{
    // /// The type of value this combinator expects assertions to be executed on.
    // /// This constrains the type of assertions that can be executed on this
    // /// combinator to just those which can be executed on types this combinator
    // /// can provide to them.
    // // Keep this as an associated type - it helps with error messages and
    // // intellisense!
    // // This flows "forwards" - it's the type that gets passed to the next item
    // // in the chain, whether that's a combinator or assertion
    // type Target;

    type NextInput;

    /// The transformed assertion, given an assertion as input. This is the type
    /// returned when transforming an assertion with this combinator, and is
    /// itself a type of assertion.
    // This flows "backwards" - it constrains the kinds of combinators this
    // combinator can be applied to, since `TransformedAssertion<A>` is itself
    // only an assertion in certain circumstances. The target of the transformed
    // assertion may be different than `Self::Target` because, for example, this
    // assertion may take an iterator as input, but pass an item in that
    // iterator to the next item in the chain.
    type Transformed: Assertion;

    /// Builds an assertion using this combinator.
    fn build(self, assertion: A) -> Self::Transformed;

    fn not(self) -> NotCombinator<Self>
    where
        NotCombinator<Self>: Combinator<Self::Transformed>,
    {
        NotCombinator { prev: self }
    }

    fn all(self) -> AllCombinator<Self> {
        AllCombinator { prev: self }
    }

    fn to_satisfy<F>(self, f: F) -> Self::Transformed
    where
        Self: Combinator<SimpleAssertion<<Self::Transformed as Assertion>::Input>>,
        Self::Transformed: Assertion<Output = Result<(), AssertionFailure>>,

        F: FnOnce(
            <Self::Transformed as Assertion>::Input,
        ) -> <Self::Transformed as Assertion>::Output,
    {
        self.build(SimpleAssertion::new("a", f))
    }
}

/// Performs a validation on a value. The [`Display`] implementation should
/// output the predicate this assertion expects to be true of the value.
pub trait Assertion: Display + Sized {
    type Input;
    type Output;

    /// Execute the assertion on a target value.
    fn assert(self, target: Self::Input) -> Self::Output;

    fn not(self) -> NotAssertion<Self>
    where
        NotAssertion<Self>: Assertion,
    {
        NotAssertion(self)
    }
}

#[derive(Clone, Debug)]
pub struct SimpleAssertion<T> {
    expectation: Arc<str>,
    predicate: fn(T) -> Result<(), AssertionFailure>,
}

impl<T> SimpleAssertion<T> {
    pub fn new(
        expectation: impl ToString,
        predicate: fn(T) -> Result<(), AssertionFailure>,
    ) -> Self {
        Self {
            expectation: expectation.to_string().into(),
            predicate,
        }
    }
}

impl<T> Display for SimpleAssertion<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expectation)
    }
}

impl<T> Assertion for SimpleAssertion<T> {
    type Input = T;
    type Output = Result<(), AssertionFailure>;

    fn assert(self, target: Self::Input) -> Result<(), AssertionFailure> {
        (self.predicate)(target)
    }
}

/////////

#[derive(Clone, Debug)]
pub struct AssertionRoot<T> {
    target: T,
}

impl<T, A> AssertionCombinator<A> for AssertionRoot<T>
where
    A: Assertion<Input = T>,
{
    type NextInput = T;
    type Transformed = RootAssertion<T, A>;

    fn build(self, assertion: A) -> Self::Transformed {
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

impl<T, Next> Assertion for RootAssertion<T, Next>
where
    Next: Assertion<Input = T>,
{
    type Input = ();

    type Output = Next::Output;

    fn assert(self, _target: Self::Input) -> Self::Output {
        self.next.assert(self.target)
    }
}

#[derive(Clone, Debug)]
pub struct NotCombinator<Prev> {
    prev: Prev,
}

impl<Prev, A> AssertionCombinator<A> for NotCombinator<Prev>
where
    A: Assertion<Output = Result<(), AssertionFailure>>,
    Prev: AssertionCombinator<NotAssertion<A>>,
{
    type NextInput = Prev::NextInput;
    type Transformed = Prev::Transformed;

    fn build(self, assertion: A) -> Self::Transformed {
        self.prev.build(NotAssertion(assertion))
    }
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

impl<Next> Assertion for NotAssertion<Next>
where
    Next: Assertion<Output = Result<(), AssertionFailure>>,
{
    type Input = Next::Input;
    type Output = Next::Output;

    fn assert(self, target: Self::Input) -> Result<(), AssertionFailure> {
        match self.0.assert(target) {
            Ok(_) => Err(AssertionFailure::default()),
            Err(_) => todo!(),
        }
    }
}

pub struct AllCombinator<Prev> {
    prev: Prev,
}

impl<Prev, A> AssertionCombinator<A> for AllCombinator<Prev>
where
    A: Assertion,
{
    type NextInput = <Prev::NextInput as IntoIterator>::Item;
    type Transformed = AllAssertion<Prev::Transformed, A>;

    fn build(self, assertion: A) -> Self::Transformed {
        AllAssertion {
            next: assertion,
            marker: PhantomData,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AllAssertion<I, Next> {
    marker: PhantomData<fn(I)>,
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

    let combinator = AssertionRoot { target: [1, 2, 3] }
        .not()
        .to_satisfy(|values: [i32; 3]| values.is_empty());

    AllCombinator {
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
