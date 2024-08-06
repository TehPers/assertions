use crate::{assertions::AssertionContext, AssertionResult};

/// An assertion result that can be inverted.
///
/// An inverted result is swapped from a failure to a success, or from a success
/// to a failure.
pub trait InvertibleResult {
    /// The inverted result.
    type Inverted;

    /// Inverts the result.
    ///
    /// A success is converted to a failure, and a failure is converted to a
    /// success.
    ///
    /// ## Async
    ///
    /// If it is not yet known whether the result represents a success or
    /// failure, then a value is returned that inverts that result when it is
    /// known.
    fn invert(self, cx: AssertionContext) -> Self::Inverted;
}

impl InvertibleResult for AssertionResult {
    type Inverted = AssertionResult;

    #[inline]
    fn invert(self, cx: AssertionContext) -> Self::Inverted {
        match self {
            Ok(()) => Err(cx.fail("expected a failure, received a success")),
            Err(_) => Ok(cx.pass()),
        }
    }
}
