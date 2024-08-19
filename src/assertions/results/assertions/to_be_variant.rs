use crate::{
    assertions::{results::Resultish, Assertion, AssertionContext},
    AssertionOutput,
};

/// Asserts that the subject is a specific [`Result`] variant.
#[derive(Clone, Debug)]
pub struct ToBeResultVariant {
    expected: ResultVariant,
}

impl ToBeResultVariant {
    #[inline]
    pub(crate) fn new(expected: ResultVariant) -> Self {
        Self { expected }
    }
}

impl<R> Assertion<R> for ToBeResultVariant
where
    R: Resultish,
{
    type Output = AssertionOutput;

    #[inline]
    fn execute(self, cx: AssertionContext, subject: R) -> Self::Output {
        match self.expected {
            ResultVariant::Ok => cx.pass_if(subject.ok().is_some(), "received Err"),
            ResultVariant::Err => cx.pass_if(subject.err().is_some(), "received Ok"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum ResultVariant {
    Ok,
    Err,
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn ok_refs_work() {
        let mut result: Result<i32, ()> = Ok(1);
        expect!(&result, to_be_ok);
        expect!(&mut result, to_be_ok);
        expect!(result, to_be_ok);

        let mut result: Result<(), i32> = Err(1);
        expect!(&result, not, to_be_ok);
        expect!(&mut result, not, to_be_ok);
        expect!(result, not, to_be_ok);
    }

    #[test]
    fn err_refs_work() {
        let mut result: Result<(), i32> = Err(1);
        expect!(&result, to_be_err);
        expect!(&mut result, to_be_err);
        expect!(result, to_be_err);

        let mut result: Result<i32, ()> = Ok(1);
        expect!(&result, not, to_be_err);
        expect!(&mut result, not, to_be_err);
        expect!(result, not, to_be_err);
    }
}
