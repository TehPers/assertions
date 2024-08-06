use either::Either;

use crate::{AssertionFailure, AssertionResult};

/// An output from performing an assertion. This represents either a success or
/// a failure, and should include additional information about the reason for
/// the failure.
///
/// Assertion outputs fundamentally parallel `Result<(), AssertionFailure>`,
/// though it's not always trivial to convert one to a result. For example,
/// while `Result<(), AssertionFailure>` *is* an [`AssertionOutput`] and is
/// trivially convertible to itself, a [`Future<Output = AssertionOutput>`]
/// cannot be converted to a result trivially and must be polled to completion
/// to get a result.
pub trait AssertionOutput {
    // TODO: docs
    type Mapped<F>: AssertionOutput
    where
        F: FnMut(AssertionResult) -> AssertionResult;

    type AndThen<O, F>: AssertionOutput
    where
        O: AssertionOutput,
        F: FnOnce() -> O;

    type OrElse<O, F>: AssertionOutput
    where
        O: AssertionOutput,
        F: FnOnce() -> O;

    type All<I>: AssertionOutput
    where
        I: IntoIterator<Item = Self>;

    type Any<I>: AssertionOutput
    where
        I: IntoIterator<Item = Self>;

    /// Creates a new output representing a success.
    ///
    /// This acts as a "default" success value for combinators that need it. For
    /// example, the `.all` combinator uses this to create a success when there
    /// are no values to execute its assertion on.
    fn new_success() -> Self;

    /// Creates a new output representing the given failure.
    ///
    /// This acts as a "default" failure value for combinators that need it. For
    /// example, the `.any` combinator uses this to create a failure when there
    /// are no values to execute its assertion on.
    fn new_failure(failure: AssertionFailure) -> Self;

    /// Maps this output to a new output.
    ///
    /// The function passed in is provided this output's value represented as a
    /// result. It is not guaranteed that this function will be called right
    /// away, and may be called in the future when needed instead.
    fn map<F>(self, f: F) -> Self::Mapped<F>
    where
        F: FnMut(AssertionResult) -> AssertionResult;

    /// Creates a new output that succeeds if and only if this and another
    /// output both succeed.
    ///
    /// This may choose to lazily evaluate `other`, but it is not guaranteed.
    /// This means that `other` may not actually be evaluated if this output
    /// already represents a failure (but this is not a guarantee).
    ///
    /// While it cannot be enforced at the type level, it is strongly encouraged
    /// for implementers to support the commutitive law. This means that whether
    /// this output `and` another output represents a success should be the same
    /// as whether the other output `and` this output represents a success.
    fn and_then<O, F>(self, other: F) -> Self::AndThen<O, F>
    where
        O: AssertionOutput,
        F: FnOnce() -> O;

    /// Creates a new output that succeeds if and only if either this or another
    /// output (or both) succeed.
    ///
    /// The remarks for [`and_then`](AssertionOutput::and_then) regarding lazy
    /// evaluation and commutitivity apply here as well. See that documentation
    /// for more information.
    fn or_else<O, F>(self, other: F) -> Self::OrElse<O, F>
    where
        O: AssertionOutput,
        F: FnOnce() -> O;

    /// Creates a new output that succeeds if and only if there are no outputs
    /// in the given iterator that represent a failure.
    ///
    /// Note that this means empty iterators will always result in a success.
    ///
    /// The order the outputs are checked for success is unspecified, and it's
    /// up to the implementer to decide what order they want to compare the
    /// outputs in. For example, suppose the iterator represented a sequence of
    /// 3 outputs. The implementer may decide to check if the second output
    /// represents a success before looking at the other outputs. Because the
    /// implementation may choose to short-circuit, if the second output
    /// represents a failure, this means there is a chance that the other
    /// outputs are not checked.
    ///
    /// In most cases, this does not matter. However, it may be relevant for
    /// asynchronous outputs. In this case, if one output is ready before the
    /// others and represents a failure, the implementation may choose to
    /// short-circuit the rest of the outputs and not complete those futures.
    /// However, this behavior is optional, and an implementer may also choose
    /// to evaluate all the futures before checking if any are a success or a
    /// failure.
    // TODO: might be useful to explore a more robust "aggregate output" trait
    //       for things like mixing required/optional outputs, min/max successes
    //       or failures, support for short-circuiting, etc.
    fn all<I>(outputs: I) -> Self::All<I>
    where
        I: IntoIterator<Item = Self>;

    /// Creates a new output that succeeds if and only if there exists an output
    /// that represents a success in the given iterator.
    ///
    /// Note that this means empty iterators will always result in a failure.
    ///
    /// The remarks for [`all`](AssertionOutput::all) regarding order
    /// specificity also apply here. See that documentation for more
    /// information.
    fn any<I>(outputs: I) -> Self::Any<I>
    where
        I: IntoIterator<Item = Self>;
}

impl AssertionOutput for Result<(), AssertionFailure> {
    type Mapped<F> = Self
        where
            F: FnMut(AssertionResult) -> AssertionResult;

    type AndThen<O, F> = Either<Self, O>
    where
        O: AssertionOutput,
        F: FnOnce() -> O;

    type OrElse<O, F> = Either<Self, O>
    where
        O: AssertionOutput,
        F: FnOnce() -> O;

    type All<I> = Self
    where
        I: IntoIterator<Item = Self>;

    type Any<I> = Self
    where
        I: IntoIterator<Item = Self>;

    fn new_success() -> Self {
        Ok(())
    }

    fn new_failure(failure: AssertionFailure) -> Self {
        Err(failure)
    }

    fn map<F>(self, mut f: F) -> Self::Mapped<F>
    where
        F: FnMut(Result<(), AssertionFailure>) -> Result<(), AssertionFailure>,
    {
        f(self)
    }

    fn and_then<O, F>(self, other: F) -> Self::AndThen<O, F>
    where
        O: AssertionOutput,
        F: FnOnce() -> O,
    {
        match self {
            Ok(()) => Either::Right(other()),
            failure => Either::Left(failure),
        }
    }

    fn or_else<O, F>(self, other: F) -> Self::OrElse<O, F>
    where
        O: AssertionOutput,
        F: FnOnce() -> O,
    {
        match self {
            Ok(()) => Either::Left(Ok(())),
            Err(_) => Either::Right(other()),
        }
    }

    fn all<I>(outputs: I) -> Self::All<I>
    where
        I: IntoIterator<Item = Self>,
    {
        outputs.into_iter().collect::<Self>()
    }

    fn any<I>(outputs: I) -> Self::Any<I>
    where
        I: IntoIterator<Item = Self>,
    {
        outputs
            .into_iter()
            .filter_map(Result::ok)
            .next()
            // TODO: build the failure correctly
            .ok_or_else(|| AssertionFailure::builder().build("no values in assertion"))
    }
}
