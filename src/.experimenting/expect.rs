use std::fmt::{Display, Formatter};

use crate::{Assertable, AssertionError};

/// Begins an assertion.
///
/// This macro is used to start an assertion. It's intended to be used in a
/// functional manner, chaining combinators together to form a complex assertion
/// that can be applied to the target value.
///
/// ```
/// # use expecters::prelude::*;
/// expect!(42).not().to_be_greater_than(100);
/// expect!([1, 2, 3, 4]).all().not().to_equal(0);
/// ```
///
/// When using this macro, source information is automatically captured based
/// on where the macro is used, and is included in the error message if the
/// assertion fails. The original target is also included to help with
/// debugging.
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!(10).to_be_less_than(5);
///
/// // The above line will panic with a message similar to the following:
/// // assertion failed.
/// //   expected: value is less than the input
/// //   at: src/main.rs:1:1
/// //   original target: 10
/// ```
///
/// For a list of built-in combinators and assertions, see the [`Assertable`]
/// trait.
#[macro_export]
macro_rules! expect {
    ($e:expr) => {
        // TODO: specialize for types that impl `Display` and `Debug`
        $crate::ExpectationRoot::new(
            $e,
            $crate::SourceInfo::new(
                file!(),
                line!(),
                column!(),
            ),
            stringify!($e),
        )
    };
}

/// The root of an expectation. Other expectations are built on top of this.
#[derive(Clone, Debug)]
pub struct ExpectationRoot<T> {
    target: T,
    source_info: SourceInfo,
    target_source: &'static str,
}

impl<T> ExpectationRoot<T> {
    /// Creates a new [`ExpectationRoot`] which wraps a target value.
    ///
    /// This method is not intended to be used directly. Instead, use the
    /// [`expect!`] macro to create an expectation.
    #[inline]
    pub fn new(target: T, source_info: SourceInfo, target_source: &'static str) -> Self {
        Self {
            target,
            source_info,
            target_source,
        }
    }

    /// Converts the expectation into a result. Rather than panicking, this
    /// causes the expectation to return an error on failure that can be handled
    /// by the caller.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// let result = expect!(42).as_result().to_equal(41);
    /// expect!(result).to_be_err();
    /// ```
    #[inline]
    pub fn as_result(self) -> TryExpectationRoot<T> {
        TryExpectationRoot {
            target: self.target,
            source_info: self.source_info,
            target_source: self.target_source,
        }
    }
}

impl<T> Assertable for ExpectationRoot<T> {
    type Target = T;
    type Result = ();

    #[inline]
    fn to_satisfy<F>(self, expectation: impl Display, f: F)
    where
        F: FnMut(Self::Target) -> bool,
    {
        if let Err(error) = self.as_result().to_satisfy(expectation, f) {
            panic!("{error}");
        }
    }
}

/// Similar to [`ExpectationRoot`], but returns a result from assertions instead
/// of panicking on failure.
pub struct TryExpectationRoot<T> {
    target: T,
    source_info: SourceInfo,
    target_source: &'static str,
}

impl<T> Assertable for TryExpectationRoot<T> {
    type Target = T;
    type Result = Result<(), AssertionError>;

    fn to_satisfy<F>(self, expectation: impl Display, mut f: F) -> Self::Result
    where
        F: FnMut(Self::Target) -> bool,
    {
        let satisfied = f(self.target);
        if satisfied {
            Ok(())
        } else {
            let error = AssertionError::new(expectation.to_string())
                .with_field("at", self.source_info.to_string())
                .with_field("original target", self.target_source);
            Err(error)
        }
    }
}

/// Information about the source of an expectation.
#[derive(Clone, Debug)]
pub struct SourceInfo {
    pub(crate) file: &'static str,
    pub(crate) line: u32,
    pub(crate) column: u32,
}

impl SourceInfo {
    #[doc(hidden)]
    pub const fn new(file: &'static str, line: u32, column: u32) -> Self {
        Self { file, line, column }
    }
}

impl Display for SourceInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}
