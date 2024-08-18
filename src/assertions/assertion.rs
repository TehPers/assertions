use std::{
    fmt::{Debug, Formatter},
    marker::PhantomData,
};

use crate::metadata::Annotated;

use super::{general::Root, AssertionContext, AssertionContextBuilder};

/// Evaluates a subject and determines whether it satisfies a condition.
///
/// Assertions take a value, execute some logic to determine whether it
/// satisfies a particular condition, and returns either a success or a failure
/// based on whether the condition was satisfied. Assertions may also attach
/// additional context to indicate why it may have failed.
///
/// Modifiers create special assertions which may choose to evaluate a
/// condition, but don't always do so. Modifiers create assertions that wrap
/// other assertions and call. They usually either transform the subject,
/// transform the output, or both.
pub trait Assertion<T> {
    /// The output type from executing this assertion.
    type Output;

    /// Executes this assertion on a given subject.
    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output;

    /// Executes this assertion on an annotated subject.
    ///
    /// By default, this just calls [`execute`]. This can be overridden if
    /// needed for assertions that need access to the annotated subject.
    ///
    /// Assertions created by modifiers should call [`execute`] instead. This is
    /// called automatically when using [`expect!`] after modifiers are
    /// executed.
    ///
    /// [`execute`]: Assertion::execute
    /// [`expect!`]: crate::expect!
    #[inline]
    fn execute_annotated(self, cx: AssertionContext, subject: Annotated<T>) -> Self::Output
    where
        Self: Sized,
    {
        self.execute(cx, subject.into_inner())
    }
}

/// Modifies an assertion.
///
/// Modifiers wrap other modifiers, and transform an assertion before passing it
/// to their inner modifier to consume. The assertion that the modifier creates
/// usually either transforms the subject, transforms the output, or both.
pub trait AssertionModifier<A> {
    /// The output type from executing this modifier on an assertion.
    type Output;

    /// Applies this modifier to a given assertion, then executes the assertion.
    ///
    /// This is usually a recursive function that calls an inner modifier's
    /// `apply` function. Its purpose is to construct the assertion that will be
    /// executed, and to invert the flow so that the assertion subject flows
    /// from the [`Root`](crate::assertions::general::Root) through each of the
    /// modifiers in order before reaching the final assertion.
    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output;
}

/// Builds an assertion.
///
/// To apply a modifier to this assertion, see [`Self::modify`].
#[must_use]
pub struct AssertionBuilder<T, M> {
    modifier: M,
    marker: PhantomData<fn(T)>,
}

impl<T> AssertionBuilder<T, Root<T>> {
    #[doc(hidden)]
    pub fn __new(subject: Annotated<T>) -> Self {
        AssertionBuilder {
            modifier: Root::new(subject),
            marker: PhantomData,
        }
    }
}

impl<T, M> AssertionBuilder<T, M> {
    /// Applies a modifier to the assertion.
    ///
    /// This associated function does not take `self` to avoid appearing in
    /// completions when writing out an expectation. The completions only appear
    /// for functions that can be executed on the builder directly, for example
    /// `builder.not()`. Because this function doesn't take `self`, it is
    /// invalid to write `builder.modify(constructor)`, so it should not appear
    /// in the suggested completions for most users.
    #[inline]
    pub fn modify<T2, M2>(builder: Self, wrap: impl FnOnce(M) -> M2) -> AssertionBuilder<T2, M2> {
        AssertionBuilder {
            modifier: wrap(builder.modifier),
            marker: PhantomData,
        }
    }

    #[doc(hidden)]
    pub fn __apply<A>(builder: Self, cx: AssertionContextBuilder, next: A) -> M::Output
    where
        M: AssertionModifier<A>,
    {
        builder.modifier.apply(cx, next)
    }
}

impl<T, M> Clone for AssertionBuilder<T, M>
where
    M: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            modifier: self.modifier.clone(),
            marker: self.marker,
        }
    }
}

impl<T, M> Debug for AssertionBuilder<T, M>
where
    M: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_struct("AssertionBuilder")
            .field("modifier", &self.modifier)
            .field("marker", &self.marker)
            .finish()
    }
}
