use std::fmt::Display;

/// An assertion which can pass or fail when provided an input. The input value
/// is represented by the `Target` type parameter.
///
/// The [`Display`] implementation of the assertion is used to describe the
/// assertion's expectation. The format of the expectation should be all
/// lowercase without any sentence terminating puctuation (like periods), and
/// should succinctly describe what the assertion is testing. For example, an
/// expectation could be "the given value is even". For an assertion that wraps
/// another assertion, the inner assertion's [`Display`] implementation can be
/// used, for example "when the future is ready, {inner}".
#[must_use = "assertions do nothing until 'assert' is called"]
pub trait Assertion<Target>: Display {
    /// The output from executing this assertion.
    type Output;

    /// Performs the assertion on a target value.
    fn assert(self, target: Target) -> Self::Output;
}
