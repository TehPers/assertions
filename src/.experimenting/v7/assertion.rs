use std::{
    fmt::{self, Display, Formatter},
    sync::Arc,
};

use super::AssertionError;

/// Performs a validation on a value. The [`Display`] implementation should
/// output the predicate this assertion expects to be true of the value.
pub trait Assertion<Input>: Display + Sized {
    /// The output of this assertion.
    type Output: AssertionOutput;

    /// Execute the assertion on a target value.
    fn assert(&mut self, target: Input) -> Self::Output;
}

/// An output from executing an assertion.
///
/// This generally represents some type that internally contains some
/// representation that can be converted to/from a
/// [`Result<(), AssertionFailure>`]. Since not all assertions can return a
/// [`Result`] directly, this allows those assertions to return something more
/// suitable for that assertion.
///
/// For example, the [`WhenReadyCombinator`] returns a [`Future`] which can be
/// `.await`ed into a [`Result`].
pub trait AssertionOutput {
    /// The inverted result type. This is returned by
    /// [`map`](AssertionOutput::map).
    type Mapped<F>: AssertionOutput
    where
        F: FnOnce(Result<(), AssertionError>) -> Result<(), AssertionError>;

    /// Maps the result represented by this output to a new result.
    ///
    /// This enables combinators to transform the result of the assertion
    /// without necessarily needing to know the type of the assertion. For
    /// example, the [`NotCombinator`] uses this method to transform a success
    /// into a failure and a failure into a success.
    fn map<F>(self, f: F) -> Self::Mapped<F>
    where
        F: FnOnce(Result<(), AssertionError>) -> Result<(), AssertionError>;
}

impl AssertionOutput for Result<(), AssertionError> {
    type Mapped<F> = Self
    where
        F: FnOnce(Result<(), AssertionError>) -> Result<(), AssertionError>;

    fn map<F>(self, f: F) -> Self::Mapped<F>
    where
        F: FnOnce(Result<(), AssertionError>) -> Result<(), AssertionError>,
    {
        f(self)
    }
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
    F: FnMut(I) -> O,
    O: AssertionOutput,
{
    type Output = O;

    fn assert(&mut self, target: I) -> Self::Output {
        (self.predicate)(target)
    }
}
