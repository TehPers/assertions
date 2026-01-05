use crate::{
    assertions::{Assertion, AssertionContext, AssertionError},
    metadata::Annotated,
    AssertionOutput,
};

/// Asserts that the subject satisfies a series of assertions.
#[derive(Clone, Debug)]
pub struct ToSatisfyWith<F> {
    predicate: Annotated<F>,
}

impl<F> ToSatisfyWith<F> {
    #[inline]
    pub(crate) fn new(predicate: Annotated<F>) -> Self {
        Self { predicate }
    }
}

impl<F, T> Assertion<T> for ToSatisfyWith<F>
where
    F: FnOnce(T) -> Result<(), AssertionError>,
{
    type Output = AssertionOutput;

    #[inline]
    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        // TODO: allow error context to be "added" to cx so failure messages
        // show the full execution path and not just the child path
        let result = (self.predicate.into_inner())(subject);
        match result {
            Ok(()) => cx.pass(),
            Err(error) => cx.fail_with(error, "inner assertions failed"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn vacuous() {
        expect!(1, to_satisfy_with(|_| Ok(())));
    }

    #[test]
    fn inner_failure() {
        expect!(
            try_expect!(1, to_satisfy_with(|n| try_expect!(n, to_equal(2)))),
            to_be_err_and,
            as_display,
            to_satisfy_with(|msg| {
                try_expect!(&msg, to_contain_substr("CAUSED BY"))?;
                try_expect!(&msg, to_contain_substr("to_equal"))?;
                try_expect!(&msg, to_contain_substr("values not equal"))?;
                Ok(())
            }),
        );
    }
}
