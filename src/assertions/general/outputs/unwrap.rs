use crate::AssertionResult;

/// An assertion output that can be unwrapped.
///
/// Unwrapping the output causes it to panic as soon as possible. For
/// [`AssertionResult`]s, the value is converted into a [`Result`] and panics if
/// the result is an [`Err`], for example. Other output types may choose to
/// unwrap in a different manner (like unwrapping an inner output once it's
/// available in the case of asynchronous outputs).
pub trait UnwrappableOutput {
    /// The unwrapped output. This is generally either `()` or a wrapper around
    /// one (like a future).
    type Unwrapped;

    /// Unwraps this output.
    ///
    /// The purpose of this method is to panic as soon as possible if an
    /// assertion fails. Not all outputs will be unwrapped, but if they are,
    /// they should provide output to the user as soon as possible if the
    /// assertion failed.
    ///
    /// This is what the assertion returns when calling
    /// [`expect!`](crate::expect!).
    ///
    /// Implementers of this function should also attach `#[track_caller]` to
    /// the function that performs the unwrapping. For synchronous outputs, this
    /// function is usually the one that unwraps the value, but async outputs
    /// may choose to unwrap the value in a `poll` function, for example.
    fn unwrap(self) -> Self::Unwrapped;
}

impl UnwrappableOutput for AssertionResult {
    type Unwrapped = ();

    #[inline]
    #[track_caller]
    fn unwrap(self) -> Self::Unwrapped {
        if let Err(e) = self.into_result() {
            panic!("{e:?}")
        }
    }
}
