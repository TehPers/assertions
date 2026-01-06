use crate::{
    assertions::{pointers::PointerLike, Assertion, AssertionContext},
    AssertionOutput,
};

/// Asserts that the subject is the null pointer.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ToBeNull {}

impl ToBeNull {
    #[inline]
    pub(crate) fn new() -> Self {
        ToBeNull {}
    }
}

impl<T> Assertion<T> for ToBeNull
where
    T: PointerLike,
{
    type Output = AssertionOutput;

    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        cx.pass_if(
            PointerLike::as_ptr(&subject).is_null(),
            "pointer is not null",
        )
    }
}
