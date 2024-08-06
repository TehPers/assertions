use super::AssertionContext;

/// TODO
pub trait Assertion<T> {
    /// The output type from executing this assertion.
    type Output;

    /// Executes this assertion on a given subject.
    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output;
}

/// Modifies an assertion.
///
/// TODO
pub trait AssertionModifier<A> {
    /// The output type from executing this modifier on an assertion.
    type Output;

    /// Applies this modifier to a given assertion, then executes the assertion.
    ///
    /// This is generally a recursive function.
    ///
    /// TODO
    fn apply(self, cx: AssertionContext, assertion: A) -> Self::Output;
}
