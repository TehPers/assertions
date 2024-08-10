use crate::{
    assertions::{results::Resultish, Assertion, AssertionContext},
    AssertionOutput,
};

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
pub fn to_be_ok() -> ToBeResultVariantAssertion {
    ToBeResultVariantAssertion {
        expected: Variant::Ok,
    }
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
pub fn to_be_err() -> ToBeResultVariantAssertion {
    ToBeResultVariantAssertion {
        expected: Variant::Err,
    }
}

/// Assertion for [`to_be_ok()`] and [`to_be_err()`].
#[derive(Clone, Debug)]
pub struct ToBeResultVariantAssertion {
    expected: Variant,
}

impl<R> Assertion<R> for ToBeResultVariantAssertion
where
    R: Resultish,
{
    type Output = AssertionOutput;

    fn execute(self, mut cx: AssertionContext, subject: R) -> Self::Output {
        cx.annotate("expected", format_args!("{:?}", self.expected));

        match self.expected {
            Variant::Ok => cx.pass_if(subject.ok().is_some(), "received Err"),
            Variant::Err => cx.pass_if(subject.err().is_some(), "received Ok"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Variant {
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
