use crate::{
    assertions::{Assertion, AssertionContextBuilder, AssertionModifier},
    metadata::Annotated,
};

/// The root of an assertion.
#[derive(Clone, Debug)]
pub struct Root<T> {
    subject: Annotated<T>,
}

impl<T> Root<T> {
    #[inline]
    pub(crate) fn new(subject: Annotated<T>) -> Self {
        Self { subject }
    }
}

impl<T, A> AssertionModifier<A> for Root<T>
where
    A: Assertion<T>,
{
    type Output = A::Output;

    #[inline]
    fn apply(self, cx: AssertionContextBuilder, assertion: A) -> Self::Output {
        assertion.execute(cx.inner, self.subject.into_inner())
    }
}
