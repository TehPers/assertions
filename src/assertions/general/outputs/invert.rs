use std::future::Future;

use crate::{assertions::AssertionContext, AssertionResult};

/// An assertion result that can be inverted.
///
/// An inverted result is swapped from a failure to a success, or from a success
/// to a failure.
pub trait InvertibleOutput {
    /// The inverted result.
    type Inverted;

    /// Inverts the result.
    ///
    /// A success is converted to a failure, and a failure is converted to a
    /// success.
    ///
    /// If it is not yet known whether the result represents a success or
    /// failure, then a value is returned that inverts that result when it is
    /// known.
    fn invert(self, cx: AssertionContext) -> Self::Inverted;
}

impl InvertibleOutput for AssertionResult {
    type Inverted = Self;

    #[inline]
    fn invert(mut self, cx: AssertionContext) -> Self::Inverted {
        if self.is_pass() {
            self.set_fail(cx, "expected a failure, received a success");
        } else {
            self.set_pass(cx);
        }

        self
    }
}

#[cfg(feature = "futures")]
const _: () = {
    use crate::assertions::futures::InvertedOutputFuture;

    impl<F> InvertibleOutput for F
    where
        F: Future<Output: InvertibleOutput>,
    {
        type Inverted = InvertedOutputFuture<F>;

        #[inline]
        fn invert(self, cx: AssertionContext) -> Self::Inverted {
            InvertedOutputFuture::new(cx, self)
        }
    }
};
