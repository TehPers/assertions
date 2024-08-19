use crate::assertions::AssertionBuilder;

use super::{ErrAndModifier, OkAndModifier, ResultVariant, Resultish, ToBeResultVariant};

/// Assertions and modifiers for [`Result`]s.
pub trait ResultAssertions<T, M>
where
    T: Resultish,
{
    /// Asserts that the target holds a success, then continues the assertion with
    /// the contained value.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// let mut subject: Result<i32, &str> = Ok(1);
    /// expect!(subject, to_be_ok_and, to_equal(1));
    /// ```
    ///
    /// The assertion fails if the result is [`Err`]:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// let subject: Result<i32, &str> = Err("error");
    /// expect!(subject, to_be_ok_and, to_equal(1));
    /// ```
    fn to_be_ok_and(self) -> AssertionBuilder<T::OutT, OkAndModifier<M>>;

    /// Asserts that the target holds an error, then continues the assertion with
    /// the contained value.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// let result: Result<i32, &str> = Err("error");
    /// expect!(result, to_be_err_and, to_equal("error"));
    /// ```
    ///
    /// The assertion fails if the result is [`Ok`]:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// let result: Result<i32, &str> = Ok(1);
    /// expect!(result, to_be_err_and, to_equal("error"));
    /// ```
    fn to_be_err_and(self) -> AssertionBuilder<T::OutE, ErrAndModifier<M>>;

    /// Asserts that the target holds a success.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// let result: Result<i32, &str> = Ok(1);
    /// expect!(result, to_be_ok);
    /// ```
    ///
    /// The assertion fails if the subject does not hold a success:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// let result: Result<i32, &str> = Err("error");
    /// expect!(result, to_be_ok);
    /// ```
    #[inline]
    #[must_use]
    fn to_be_ok(&self) -> ToBeResultVariant {
        ToBeResultVariant::new(ResultVariant::Ok)
    }

    /// Asserts that the subject holds an error.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// let result: Result<i32, &str> = Err("error");
    /// expect!(result, to_be_err);
    /// ```
    ///
    /// The assertion fails if the subject does not hold an error:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// let result: Result<i32, &str> = Ok(1);
    /// expect!(result, to_be_err);
    /// ```
    #[inline]
    #[must_use]
    fn to_be_err(&self) -> ToBeResultVariant {
        ToBeResultVariant::new(ResultVariant::Err)
    }
}

impl<T, M> ResultAssertions<T, M> for AssertionBuilder<T, M>
where
    T: Resultish,
{
    #[inline]
    fn to_be_ok_and(self) -> AssertionBuilder<T::OutT, OkAndModifier<M>> {
        AssertionBuilder::modify(self, OkAndModifier::new)
    }

    #[inline]
    fn to_be_err_and(self) -> AssertionBuilder<T::OutE, ErrAndModifier<M>> {
        AssertionBuilder::modify(self, ErrAndModifier::new)
    }
}
