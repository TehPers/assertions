use std::fmt::{Display, Formatter};

use crate::{assertions::Assertion, combinators::AssertionCombinator, AssertionResult, SourceLoc};

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
            $crate::source_loc!(),
            ::core::stringify!($e),
        )
    };
}

/// The root of an expectation. Other expectations are built on top of this.
#[derive(Clone, Debug)]
pub struct ExpectationRoot<Target> {
    target: Target,
    source_info: SourceLoc,
    target_source: &'static str,
}

impl<Target> ExpectationRoot<Target> {
    /// Creates a new [`ExpectationRoot`] which wraps a target value.
    ///
    /// This method is not intended to be used directly. Instead, use the
    /// [`expect!`] macro to create an expectation.
    #[inline]
    pub fn new(target: Target, source_info: SourceLoc, target_source: &'static str) -> Self {
        Self {
            target,
            source_info,
            target_source,
        }
    }
}

impl<Target, A> AssertionCombinator<A> for ExpectationRoot<Target>
where
    A: Assertion<Target>,
{
    type NextTarget = Target;
    type Output = AssertionResult;

    fn apply(self, mut assertion: A) -> Self::Output {
        assertion.execute(self.target)
    }
}

#[cfg(test)]
mod tests {
    use crate::{combinators::*, prelude::*};

    #[test]
    fn foo() {
        // let combinator = ExpectationRoot::new([1, 2, 3], todo!(), "a");
        // let _a = expect!([1, 2, 3]).not().all();

        let _a = expect!([1, 2, 3]).not().apply(|_n| Ok(()));

        let _a = NotCombinator::new(AllCombinator::new(expect!([1, 2, 3])));
        let result = _a.apply(|_n| Ok(()));

        // let output = expect!(1).to_equal(1);
    }
}
