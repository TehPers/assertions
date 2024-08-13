use crate::{assertions::AssertionContext, AssertionOutput};

/// An assertion output that can be inverted.
///
/// An inverted output is swapped from a failure to a success, or from a success
/// to a failure.
pub trait InvertibleOutput {
    /// The inverted output.
    type Inverted;

    /// Inverts the output.
    ///
    /// A success is converted to a failure, and a failure is converted to a
    /// success.
    ///
    /// If it is not yet known whether the output represents a success or
    /// failure, then a value is returned that inverts that output when it is
    /// known.
    ///
    /// The context passed into this method should represent the point at which
    /// the output was inverted. For example, an output's internal context may
    /// represent an execution flow going through `expect!(1, not, to_equal(2))`
    /// and reaching the [`to_equal`] assertion, but the inversion would occur
    /// at [`not`].
    ///
    /// [`not`]: crate::prelude::GeneralAssertions::not
    /// [`to_equal`]: crate::prelude::GeneralAssertions::to_equal
    fn invert(self, cx: AssertionContext) -> Self::Inverted;
}

impl InvertibleOutput for AssertionOutput {
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
