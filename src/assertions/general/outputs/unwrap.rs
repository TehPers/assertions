use crate::{assertions::AssertionError, AssertionOutput};

/// An assertion output that can be unwrapped.
///
/// Unwrapping the output causes it to panic as soon as possible. For
/// [`AssertionOutput`]s, the value is converted into a [`Result`] and panics if
/// the result is an [`Err`], for example. Other output types may choose to
/// unwrap in a different manner (like unwrapping an inner output once it's
/// available in the case of asynchronous outputs).
pub trait UnwrappableOutput {
    /// The unwrapped output. This is generally either `()` or a wrapper around
    /// one (like a future).
    type Unwrapped;

    /// The output representing an attempt at unwrapping. This is generally a
    /// [`Result<(), AssertionError>`] or a wrapper around one (like a future).
    type TryUnwrapped;

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

    /// Tries to unwrap this output.
    ///
    /// This is similar to [`unwrap`](UnwrappableOutput::unwrap), but instead of
    /// panicking on failure, it instead returns an [`Err`] containing the
    /// error. On success, returns an [`Ok`] instead.
    ///
    /// The actual return value from this function may be a [`Result`], or may
    /// be another type that eventually becomes a [`Result`] through some series
    /// of well-documented operations. For example, for an asynchronous
    /// assertion, a future may be returned instead that eventually outputs a
    /// [`Result`].
    fn try_unwrap(self) -> Self::TryUnwrapped;
}

impl UnwrappableOutput for AssertionOutput {
    type Unwrapped = ();
    type TryUnwrapped = Result<(), AssertionError>;

    #[inline]
    #[track_caller]
    fn unwrap(self) -> Self::Unwrapped {
        if let Err(e) = self.into_result() {
            panic!("{e:?}")
        }
    }

    #[inline]
    fn try_unwrap(self) -> Self::TryUnwrapped {
        self.into_result()
    }
}
