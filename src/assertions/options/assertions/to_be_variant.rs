use crate::{
    assertions::{options::Optionish, Assertion, AssertionContext},
    AssertionOutput,
};

/// Asserts that the subject is a specific [`Option`] variant.
#[derive(Clone, Debug)]
pub struct ToBeOptionVariant {
    expected: OptionVariant,
}

impl ToBeOptionVariant {
    #[inline]
    pub(crate) fn new(expected: OptionVariant) -> Self {
        Self { expected }
    }
}

impl<O> Assertion<O> for ToBeOptionVariant
where
    O: Optionish,
{
    type Output = AssertionOutput;

    #[inline]
    fn execute(self, cx: AssertionContext, subject: O) -> Self::Output {
        match self.expected {
            OptionVariant::Some => cx.pass_if(subject.some().is_some(), "received None"),
            OptionVariant::None => cx.pass_if(subject.some().is_none(), "received Some"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum OptionVariant {
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
