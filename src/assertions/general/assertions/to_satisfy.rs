use crate::{
    assertions::{Assertion, AssertionContext},
    metadata::Annotated,
    AssertionOutput,
};

/// Asserts that the subject satisfies a predicate.
#[derive(Clone, Debug)]
pub struct ToSatisfyAssertion<F> {
    predicate: Annotated<F>,
}

impl<F> ToSatisfyAssertion<F> {
    #[inline]
    pub(crate) fn new(predicate: Annotated<F>) -> Self {
        Self { predicate }
    }
}

impl<F, T> Assertion<T> for ToSatisfyAssertion<F>
where
    F: FnOnce(T) -> bool,
{
    type Output = AssertionOutput;

    fn execute(self, mut cx: AssertionContext, subject: T) -> Self::Output {
        cx.annotate("predicate", &self.predicate);
        cx.pass_if(
            (self.predicate.into_inner())(subject),
            "did not satisfy predicate",
        )
    }
}
