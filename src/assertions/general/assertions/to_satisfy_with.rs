use crate::{
    assertions::{Assertion, AssertionContext, AssertionError},
    metadata::Annotated,
    AssertionOutput,
};

/// Asserts that the subject satisfies a series of assertions.
#[derive(Clone, Debug)]
pub struct ToSatisfyWithAssertion<F> {
    predicate: Annotated<F>,
}

impl<F> ToSatisfyWithAssertion<F> {
    #[inline]
    pub(crate) fn new(predicate: Annotated<F>) -> Self {
        Self { predicate }
    }
}

impl<F, T> Assertion<T> for ToSatisfyWithAssertion<F>
where
    F: FnOnce(T) -> Result<(), AssertionError>,
{
    type Output = AssertionOutput;

    #[inline]
    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        // TODO: allow error context to be "added" to cx so failure messages
        // show the full execution path and not just the child path
        let result = (self.predicate.into_inner())(subject);
        cx.pass_if(result.is_ok(), "inner assertions failed")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn vacuous() {
        expect!(1, to_satisfy_with(|_| Ok(())));
    }
}
