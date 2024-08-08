use crate::assertions::Assertion;

/// Transforms an assertion into a more complex assertion.
///
/// Combinators are used to build more complex assertions out of less complex
/// ones. They function as a sort of "middleware" and are able to transform both
/// the input to an assertion and the output from one.
///
/// The type parameter represents the type of assertion this combinator can
/// wrap.
#[must_use = "combinators do nothing until they are applied"]
pub trait AssertionCombinator<Next> {
    /// The type of value passed to this combinator's assertion.
    ///
    /// For many combinators which wrap another combinator, this is simply
    /// `Inner::Target` (where `Inner` is the inner combinator).
    type Target;

    /// The type of value passed to the assertion that is passed to this
    /// combinator. This is used to determine which combinators and assertions
    /// are valid to be chained to a combinator. For example, this is used to
    /// prevent code like `expect!(1).all()` from compiling and to provide
    /// better completions to users.
    ///
    /// This should be set to the type of value that the assertion built by this
    /// combinator will pass to its inner/next assertion. `expect!("hi")` will
    /// pass a `&str` to the chained assertion for example, despite its
    /// [`Target`](AssertionCombinator::Target) being `()`.
    type NextTarget;

    /// The type of assertion that this combinator wraps the given assertion
    /// with.
    ///
    /// For many combinators which wrap another combinator, this is simply
    /// `Inner::Assertion` (where `Inner` is the inner combinator). Note that
    /// this usually implies a bound on the implementation of
    /// `Inner: AssertionCombinator<MyAssertion<Next>>` if passing a custom
    /// assertion into the inner combinator (which is generally recommended).
    type Assertion: Assertion<Self::Target>;

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
    fn apply(self, next: Next) -> Self::Assertion;
}

// expect!(a).not().all()
// -> AllCombinator<NotCombinator<AssertionRoot<T>>>
// .apply(next)
// -> Root<NotAssertion<AllAssertion<Next>>>
// -> Next needs to be Clone, must be type param to ensure that and so
// -> the AllAssertion can clone it
// .assert(value)
// -> first apply .all(), then apply .not() to invert, then identity at root
