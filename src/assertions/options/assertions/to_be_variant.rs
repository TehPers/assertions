use crate::{
    assertions::{options::Optionish, Assertion, AssertionContext},
    AssertionOutput,
};

/// Asserts that the subject holds a value.
///
/// ```
/// # use expecters::prelude::*;
/// expect!(Some(1), to_be_some);
/// ```
///
/// The assertion fails if the subject does not hold a value:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!(None::<i32>, to_be_some);
/// ```
#[inline]
#[must_use]
pub fn to_be_some() -> ToBeOptionVariantAssertion {
    ToBeOptionVariantAssertion {
        expected: Variant::Some,
    }
}

/// Asserts that the subject does not hold a value.
///
/// ```
/// # use expecters::prelude::*;
/// expect!(None::<i32>, to_be_none);
/// ```
///
/// The assertion fails if the subject holds a value:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!(Some(1), to_be_none);
/// ```
#[inline]
#[must_use]
pub fn to_be_none() -> ToBeOptionVariantAssertion {
    ToBeOptionVariantAssertion {
        expected: Variant::None,
    }
}

/// Assertion for [`to_be_some()`] and [`to_be_none()`].
#[derive(Clone, Debug)]
pub struct ToBeOptionVariantAssertion {
    expected: Variant,
}

impl<O> Assertion<O> for ToBeOptionVariantAssertion
where
    O: Optionish,
{
    type Output = AssertionOutput;

    fn execute(self, mut cx: AssertionContext, subject: O) -> Self::Output {
        cx.annotate("expected", format_args!("{:?}", self.expected));

        match self.expected {
            Variant::Some => cx.pass_if(subject.some().is_some(), "received None"),
            Variant::None => cx.pass_if(subject.some().is_none(), "received Some"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Variant {
    Some,
    None,
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn some_refs_work() {
        let mut option: Option<i32> = Some(1);
        expect!(&option, to_be_some);
        expect!(&mut option, to_be_some);
        expect!(option, to_be_some);

        let mut option: Option<i32> = None;
        expect!(&option, not, to_be_some);
        expect!(&mut option, not, to_be_some);
        expect!(option, not, to_be_some);
    }

    #[test]
    fn none_refs_work() {
        let mut option: Option<i32> = None;
        expect!(&option, to_be_none);
        expect!(&mut option, to_be_none);
        expect!(option, to_be_none);

        let mut option: Option<i32> = Some(1);
        expect!(&option, not, to_be_none);
        expect!(&mut option, not, to_be_none);
        expect!(option, not, to_be_none);
    }
}
