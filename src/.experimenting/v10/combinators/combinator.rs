// TODO: docs

use crate::AssertionResult;

/// Transforms an assertion into a more complex assertion.
///
/// Combinators are used to build more complex assertions out of less complex
/// ones. They function as a sort of "middleware" and are able to transform both
/// the input to an assertion and the output from one.
///
/// The type parameter represents the type of assertion this combinator can
/// wrap.
#[must_use = "combinators do nothing until they are applied"]
pub trait AssertionCombinator {
    /// The type of value passed to this combinator's assertion.
    ///
    /// For many combinators which wrap another combinator, this is simply
    /// `Inner::Target` (where `Inner` is the inner combinator).
    type Target;

    /// The result type from executing an assertion with this combinator.
    type Result;

    /// Wraps an assertion.
    ///
    /// This function is the foundation for how combinators work. This is used
    /// to create more complex assertions. The combinator works by wrapping the
    /// given assertion with its own assertion. For example, the assertion
    /// returned by this combinator can negate the output of the provided
    /// assertion, call the provided assertion multiple times (if it's `Clone`),
    /// or transform the output of the provided assertion in some other manner.
    ///
    /// The returned assertion still needs to be executed on a value. The type
    /// of input the returned assertion accepts is determined by the
    /// [`AssertionCombinator::Target`] property.
    ///
    /// This method also usually has the side effect of "inverting" the type.
    /// For example, calling `expect!(value).not().all()` will create an
    /// `AllCombinator<NotCombinator<AssertionRoot<T>>>` (where `T` is the type
    /// of the value), and applying the combinator will generate an instance of
    /// `RootAssertion<NotAssertion<AllAssertion<Next>>>`.
    fn execute<F>(self, f: F) -> Self::Result
    where
        F: FnMut(Self::Target) -> AssertionResult;
}

impl<Target> AssertionCombinator for ExpectationRoot<Target> {
    type Target = Target;
    type Result = AssertionResult;

    fn execute<F>(self, mut f: F) -> Self::Result
    where
        F: FnMut(Self::Target) -> AssertionResult,
    {
        f(self.target)
    }
}
