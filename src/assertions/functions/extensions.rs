use std::panic::UnwindSafe;

use crate::assertions::{
    functions::{ToPanic, WhenCalledModifier},
    AssertionBuilder,
};

/// Assertions and modifiers for functions that accept no arguments.
pub trait FunctionAssertions<F, O, M> {
    /// Executes an assertion on the value returned by this function.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(|| 1, when_called, to_equal(1));
    /// ```
    fn when_called(self) -> AssertionBuilder<O, WhenCalledModifier<M>>;

    /// Asserts that the subject panics when called.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(|| panic!(), to_panic);
    /// ```
    ///
    /// This assertion fails if the subject does not panic.
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(|| {}, to_panic);
    /// ```
    #[inline]
    fn to_panic(&self) -> ToPanic
    where
        F: UnwindSafe,
    {
        ToPanic::new()
    }
}

impl<F, O, M> FunctionAssertions<F, O, M> for AssertionBuilder<F, M>
where
    F: FnOnce() -> O,
{
    #[inline]
    fn when_called(self) -> AssertionBuilder<O, WhenCalledModifier<M>> {
        AssertionBuilder::modify(self, WhenCalledModifier::new)
    }
}
