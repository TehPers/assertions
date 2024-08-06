use std::fmt::{Display, Formatter};

use crate::combinators::{Assertion, AssertionCombinator};

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
pub struct ExpectationRoot<Target> {
    target: Target,
    source_info: SourceInfo,
    target_source: &'static str,
}

impl<Target> ExpectationRoot<Target> {
    /// Creates a new [`ExpectationRoot`] which wraps a target value.
    ///
    /// This method is not intended to be used directly. Instead, use the
    /// [`expect!`] macro to create an expectation.
    #[inline]
    pub fn new(target: Target, source_info: SourceInfo, target_source: &'static str) -> Self {
        Self {
            target,
            source_info,
            target_source,
        }
    }
}

impl<Target, Next> AssertionCombinator<Next> for ExpectationRoot<Target>
where
    Next: Assertion<Target>,
{
    type Target = ();
    type NextTarget = Target;
    type Assertion = RootAssertion<Target, Next>;

    #[inline]
    fn apply(self, next: Next) -> Self::Assertion {
        RootAssertion::new(self.target, next)
    }
}

/// The root-level assertion. This assertion wraps another assertion, passes
/// that assertion the target, and attaches additional context to the error if
/// the assertion fails. All assertions are normally eventually wrapped by this
/// type.
#[derive(Clone, Debug, Default)]
pub struct RootAssertion<Target, Next> {
    target: Target,
    next: Next,
}

impl<Target, Next> RootAssertion<Target, Next> {
    /// Creates a new instance of this assertion.
    #[inline]
    pub fn new(target: Target, next: Next) -> Self {
        Self { target, next }
    }
}

impl<Target, Next> Display for RootAssertion<Target, Next>
where
    Next: Display,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.next.fmt(f)
    }
}

impl<Target, Next> Assertion<()> for RootAssertion<Target, Next>
where
    Next: Assertion<Target>,
{
    type Output = Next::Output;

    #[inline]
    fn assert(self, (): ()) -> Self::Output {
        // TODO: attach source info to the error
        self.next.assert(self.target)
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

#[cfg(test)]
mod tests {
    use std::future::ready;

    use crate::{combinators::*, prelude::*, AssertionResult, ExpectationRoot};

    #[test]
    fn foo() {
        // let combinator = ExpectationRoot::new([1, 2, 3], todo!(), "a");
        let _ = expect!([1, 2, 3]).all().all();
        // let combinator = ExpectationRoot::new(ready([1, 2, 3]), todo!(), "a");
        // let combinator = WhenReadyCombinator::new(combinator);
        // let combinator = NotCombinator::new(combinator);
        let combinator = AllCombinator::new(combinator);
        let assertion = SimpleAssertion::new("always pass", |_value| AssertionResult::Ok(()));
        let assertion = combinator.apply(assertion);
        // let assertion = AssertionCombinator::apply(combinator, assertion);
        // AllAssertion::new(assertion);
        // let assertion = NotAssertion::new(AllAssertion::new(assertion));
        let _blah = assertion.assert(());
        // let assertion = combinator

        // let output = expect!(1).to_equal(1);
    }
}
