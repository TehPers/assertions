use std::fmt::{Display, Formatter};

/// A transformer for an assertion.
///
/// Combinators are used to transform the target value while executing an
/// assertion. This can range from something as simple as getting the length of
/// the input value to executing future assertions asynchronously once the
/// target value is ready. It can even execute future assertions multiple times
/// and aggregate the results.
///
/// These are the core of how the input value is transformed, how more complex
/// assertions can be built, and how the final result is produced.
pub trait Combinator<Next> {
    /// The assertion that is produced by this combinator when it is used to
    /// transform another assertion.
    type Assertion;

    /// Builds the assertion that is produced by this combinator.
    ///
    /// This method is used to transform the next assertion in the chain in some
    /// manner dependent on the functionality of the combinator. For example,
    /// the assertion returned by this method may transform the input passed to
    /// the next assertion or even execute it multiple times.
    fn build(self, next: Next) -> Self::Assertion;
}

/// An assertion that can be executed on an input value.
///
/// The assertion produces a result often indicative of whether the assertion
/// passed or failed. While in most cases the result is literally a [`Result`],
/// it can sometimes also be other types, such as a future that resolves to a
/// result.
///
/// ## Display
///
/// Assertions must implement [`Display`]. This allows the expectation message
/// to be communicated to the user in a human-readable format. The output format
/// should be a short description of what is expected of the input. For example,
/// the output format for an assertion that checks whether a value is true could
/// be: `"the value is true"`.
///
/// Combinators will want to ensure the expectation message includes the next
/// assertion's message as well, meaning the expectation message will often be
/// created by adding some text to the next assertion's message. For example, a
/// combinator that maps an input iterator to the number of elements in the
/// iterator could have an output format of: `"the length satisfies: {next}"`,
/// where `{next}` indicates the next assertion's expectation.
pub trait Assertion<Input>: Display {
    /// The output produced by the assertion. This is usually either a
    /// [`Result<(), AssertError>`] or a future which resolves to one.
    type Output;

    /// Executes the assertion on the input value.
    fn execute(self, input: Input) -> Self::Output;
}

/// A simple assertion constructed using a callback function.
#[derive(Clone, Debug)]
pub struct AssertionFn<F> {
    expectation: String,
    f: F,
}

impl<F> AssertionFn<F> {
    /// Creates a new [`AssertionFn`].
    #[inline]
    pub fn new(expectation: impl Display, f: F) -> Self {
        Self {
            expectation: expectation.to_string(),
            f,
        }
    }
}

impl<F> Display for AssertionFn<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expectation)
    }
}

impl<F, I, O> Assertion<I> for AssertionFn<F>
where
    F: FnOnce(I) -> O,
{
    type Output = O;

    fn execute(self, input: I) -> Self::Output {
        (self.f)(input)
    }
}
