use std::{
    fmt::{Display, Formatter},
    ops::ControlFlow,
};

use crate::AssertionErrorBuilder;

use super::{Assertion, Combinator};

#[derive(Clone, Copy, Debug, Default)]
pub struct AggregateCombinator<A> {
    aggregator: A,
}

impl<A> AggregateCombinator<A> {
    /// Creates a new [`AggregateCombinator`].
    pub fn new(aggregator: A) -> Self {
        Self { aggregator }
    }
}

impl<A, Next> Combinator<Next> for AggregateCombinator<A> {
    type Assertion = AggregateAssertion<A, Next>;

    fn build(self, next: Next) -> Self::Assertion {
        AggregateAssertion::new(self.aggregator, next)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AggregateAssertion<A, Next> {
    aggregator: A,
    next: Next,
}

impl<A, Next> AggregateAssertion<A, Next> {
    /// Creates a new [`AggregateAssertion`].
    pub fn new(aggregator: A, next: Next) -> Self {
        Self { aggregator, next }
    }
}

impl<A, Next> Display for AggregateAssertion<A, Next>
where
    A: Display,
    Next: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "for {}, {}", self.aggregator, self.next)
    }
}

impl<A, Next, Input> Assertion<Input> for AggregateAssertion<A, Next>
where
    Input: IntoIterator,
    Next: Assertion<Input::Item> + Clone,
    A: Aggregator<Next::Output>,
{
    type Output = A::Aggregate;

    fn execute(mut self, input: Input) -> Self::Output {
        let output = input
            .into_iter()
            .map(|input| self.next.clone().execute(input))
            .try_fold(self.aggregator.zero(), |aggregate, output| {
                self.aggregator.with_output(aggregate, output)
            });

        match output {
            ControlFlow::Continue(aggregate) => aggregate,
            ControlFlow::Break(aggregate) => aggregate,
        }
    }
}

pub trait Aggregator<Output>: Display {
    type Aggregate;

    fn zero(&mut self) -> Self::Aggregate;

    fn with_output(
        &mut self,
        aggregate: Self::Aggregate,
        next: Output,
    ) -> ControlFlow<Self::Aggregate, Self::Aggregate>;
}

/// Aggregate function that requires all assertions to be successful.
#[derive(Clone, Copy, Debug, Default)]
pub struct AllSucceed;

impl Display for AllSucceed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "each inner value")
    }
}

impl Aggregator<Result<(), AssertionErrorBuilder>> for AllSucceed {
    type Aggregate = Result<(), AssertionErrorBuilder>;

    /// The "zero value" for the aggregation. This is what the aggregation
    /// starts with before any outputs are processed. If there are no outputs
    /// to process, this is also the final result.
    fn zero(&mut self) -> Self::Aggregate {
        Ok(())
    }

    /// Combine the current aggregate with the next output. At any point, the
    /// aggregation can short-circuit and return the final result by returning
    /// [`ControlFlow::Break`].
    fn with_output(
        &mut self,
        _: Self::Aggregate,
        next: Result<(), AssertionErrorBuilder>,
    ) -> ControlFlow<Self::Aggregate, Self::Aggregate> {
        match next {
            Ok(_) => ControlFlow::Continue(Ok(())),
            Err(e) => ControlFlow::Break(Err(e)),
        }
    }
}

/// Aggregate function that requires any assertions to be successful.
#[derive(Clone, Copy, Debug, Default)]
pub struct AnySucceed;

impl Display for AnySucceed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "some inner value")
    }
}

impl Aggregator<Result<(), AssertionErrorBuilder>> for AnySucceed {
    type Aggregate = Result<(), AssertionErrorBuilder>;

    fn zero(&mut self) -> Self::Aggregate {
        Err(AssertionErrorBuilder::default().with_field("reason", "no assertion succeeded"))
    }

    fn with_output(
        &mut self,
        aggregate: Self::Aggregate,
        next: Result<(), AssertionErrorBuilder>,
    ) -> ControlFlow<Self::Aggregate, Self::Aggregate> {
        match next {
            Ok(_) => ControlFlow::Break(Ok(())),
            Err(_) => ControlFlow::Continue(aggregate),
        }
    }
}
