use crate::combinators2::{Assertion, Combinator, IdentityCombinator, Plan};

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
macro_rules! expect2 {
    ($e:expr) => {
        // TODO: specialize for types that impl `Display` and `Debug`
        $crate::AssertionRoot::new(
            $e,
            $crate::AssertionMetadata::new(
                file!(),
                line!(),
                column!(),
                stringify!($e),
            ),
        )
    };
}

/// The root of an assertion. Combinators are chained together to build an
/// assertion plan, and the plan is executed with a final assertion.
#[derive(Clone, Debug)]
#[must_use = "this type does nothing until an assertion is executed"]
pub struct AssertionRoot<T, P> {
    target: T,
    plan: P,
    metadata: AssertionMetadata,
}

impl<T> AssertionRoot<T, IdentityCombinator> {
    /// Creates a new [`AssertionRoot`] which wraps a target value.
    ///
    /// This method is not intended to be used directly. Instead, use the
    /// [`expect!`] macro to create an expectation.
    #[inline]
    pub fn new(target: T, metadata: AssertionMetadata) -> Self {
        Self {
            target,
            plan: IdentityCombinator,
            metadata,
        }
    }
}

impl<T, P> AssertionRoot<T, P> {
    /// Chains a combinator onto the end of the assertion plan.
    ///
    /// This is the core of how combinators are built. Combinators are used to
    /// transform the target value before executing a final assertion. This
    /// method allows them to be chained together in a functional manner to
    /// transform the target value in a complex way.
    pub fn chain<Next>(self, combinator: Next) -> AssertionRoot<T, Plan<P, Next>> {
        AssertionRoot {
            target: self.target,
            plan: Plan::new(self.plan, combinator),
            metadata: self.metadata,
        }
    }

    /// Executes the assertion plan by providing the final assertion.
    ///
    /// This is the core of how assertions are executed. The plan is built up
    /// using combinators, and then the final assertion is executed on the
    /// transformed target value.
    ///
    /// Normally, you want to call one of the assertion extension methods
    /// directly rather than calling this method.
    pub fn execute<A>(
        self,
        assertion: A,
    ) -> <<P as Combinator<A>>::Assertion as Assertion<T>>::Output
    where
        P: Combinator<A>,
        P::Assertion: Assertion<T>,
    {
        self.plan.build(assertion).execute(self.target)
    }
}

/// Metadata about an assertion that's being executed. This includes information
/// about where the assertion was created and the original target of the
/// assertion.
#[derive(Clone, Debug)]
pub struct AssertionMetadata {
    pub(crate) file: &'static str,
    pub(crate) line: u32,
    pub(crate) column: u32,
    pub(crate) target_source: &'static str,
}

impl AssertionMetadata {
    // Signature may change at any time!
    #[doc(hidden)]
    pub fn new(file: &'static str, line: u32, column: u32, target_source: &'static str) -> Self {
        Self {
            file,
            line,
            column,
            target_source,
        }
    }
}
