use crate::assertions::AssertionBuilder;

use super::{OptionVariant, Optionish, SomeAndModifier, ToBeOptionVariantAssertion};

/// Assertions and modifiers for [`Option`]s.
pub trait OptionAssertions<T, M>
where
    T: Optionish,
{
    /// Asserts that the subject holds a value, then continues the assertion with
    /// the contained value.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(Some(1), to_be_some_and, to_equal(1));
    /// ```
    ///
    /// The assertion fails if the option is [`None`]:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(None::<i32>, to_be_some_and, to_equal(2));
    /// ```
    fn to_be_some_and(self) -> AssertionBuilder<T::OutT, SomeAndModifier<M>>;

    /// Asserts that the subject holds a value.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(Some(1), to_be_some);
    /// ```
    ///
    /// The assertion fails if the subject does not hold a value:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(None::<i32>, to_be_some);
    /// ```
    #[inline]
    #[must_use]
    fn to_be_some(&self) -> ToBeOptionVariantAssertion {
        ToBeOptionVariantAssertion::new(OptionVariant::Some)
    }

    /// Asserts that the subject does not hold a value.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(None::<i32>, to_be_none);
    /// ```
    ///
    /// The assertion fails if the subject holds a value:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(Some(1), to_be_none);
    /// ```
    #[inline]
    #[must_use]
    fn to_be_none(&self) -> ToBeOptionVariantAssertion {
        ToBeOptionVariantAssertion::new(OptionVariant::None)
    }
}

impl<T, M> OptionAssertions<T, M> for AssertionBuilder<T, M>
where
    T: Optionish,
{
    #[inline]
    fn to_be_some_and(self) -> AssertionBuilder<T::OutT, SomeAndModifier<M>> {
        AssertionBuilder::modify(self, SomeAndModifier::new)
    }
}
