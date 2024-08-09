use crate::{assertions::AssertionContext, AssertionResult};

/// An assertion output that can be directly constructed from an
/// [`AssertionContext`].
///
/// Some modifiers need to directly initialize an instance of their output type.
/// For example, fallible modifiers like
/// [`to_be_some_and`](crate::prelude::to_be_some_and) can fail without
/// continuing the rest of the assertion, and those modifiers need a way to
/// construct the failure for their output type. Output types that implement
/// this trait can be constructed directly, so those modifiers are able to fail
/// the assertion early without continuing execution.
pub trait InitializableOutput {
    // /// The initialized output type. This may differ from `Self` if it cannot be
    // /// constructed directly, but can be wrapped by another type that also
    // /// supports direct construction (which is often the case for asynchronous
    // /// outputs).
    // type Initialized;

    /// Constructs an output that represents a success.
    fn pass(cx: AssertionContext) -> Self;

    /// Constructs an output that represents a failure with a given message.
    fn fail(cx: AssertionContext, message: String) -> Self;

    // /// Converts this output into an instance of the initialized output type.
    // /// This is important to ensure that an existing instance of this output can
    // /// be converted to the success/failure types this output can produce.
    // fn into_initializable(self) -> Self::Initialized;
}

impl InitializableOutput for AssertionResult {
    #[inline]
    fn pass(cx: AssertionContext) -> Self {
        AssertionResult::new(cx, None)
    }

    #[inline]
    fn fail(cx: AssertionContext, message: String) -> Self {
        AssertionResult::new(cx, Some(message))
    }
}

/// An output type that can be converted into an
/// [initializable output type][initializable].
///
/// [initializable]: InitializableOutput
pub trait IntoInitializableOutput {
    /// The initialized output type.
    ///
    /// This may differ from `Self` if it cannot be constructed directly, but
    /// can be wrapped by another type that also supports direct construction
    /// (which is often the case for asynchronous outputs).
    type Initialized: InitializableOutput;

    /// Converts this output into an instance of the initialized output type.
    ///
    /// This is important to ensure that an existing instance of this output can
    /// be converted to the success/failure types this output can produce.
    fn into_initialized(self) -> Self::Initialized;
}

impl IntoInitializableOutput for AssertionResult {
    type Initialized = Self;

    #[inline]
    fn into_initialized(self) -> Self::Initialized {
        self
    }
}
