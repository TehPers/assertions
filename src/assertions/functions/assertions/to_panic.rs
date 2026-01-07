use std::panic::{catch_unwind, UnwindSafe};

use crate::{
    assertions::{functions::ApplyFnOnceUnwindSafe, Assertion, AssertionContext},
    metadata::Annotated,
    AssertionOutput,
};

/// Asserts that the subject function panics when called.
#[derive(Clone, Debug)]
pub struct ToPanic<I = ()> {
    args: Annotated<I>,
}

impl<I> ToPanic<I> {
    #[inline]
    pub(crate) fn new(args: Annotated<I>) -> Self {
        Self { args }
    }
}

impl<F, I> Assertion<F> for ToPanic<I>
where
    F: ApplyFnOnceUnwindSafe<I> + UnwindSafe,
{
    type Output = AssertionOutput;

    #[inline]
    fn execute(self, mut cx: AssertionContext, subject: F) -> Self::Output {
        if !F::EMPTY_ARGS {
            cx.annotate("args", &self.args);
        }

        let result = catch_unwind(subject.apply_once_unwind(self.args.into_inner()));
        cx.pass_if(result.is_err(), "did not panic")
    }
}
