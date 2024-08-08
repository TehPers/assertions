use std::{fmt::Debug, marker::PhantomData};

use super::AssertionContext;

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
    /// This is generally a recursive function.
    ///
    /// TODO
    fn apply(self, next: A) -> Self::Output;
}

/// Indicates the type of subject being passed to the next modifier/assertion in
/// the chain.
///
/// This is necessary to help the compiler infer the type of the value being
/// passed from a modifier to the next modifier/assertion.
///
/// To create an instance of it, use:
///
/// ```
/// use expecters::assertions::key;
/// let key = key::<i32>();
/// # let _ = key;
/// ```
pub struct SubjectKey<T>(PhantomData<fn(T)>);

impl<T> Clone for SubjectKey<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for SubjectKey<T> {}

impl<T> Debug for SubjectKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SubjectKey").field(&self.0).finish()
    }
}

/// Creates a new key for a subject. Modifiers must return a key on creation to
/// indicate the type of subject the next modifier/assertion is expected to
/// receive.
#[must_use]
pub const fn key<T>() -> SubjectKey<T> {
    SubjectKey(PhantomData)
}
