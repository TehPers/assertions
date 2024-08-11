use crate::{
    assertions::{key, Assertion, AssertionContextBuilder, AssertionModifier, SubjectKey},
    metadata::Annotated,
};

#[doc(hidden)]
#[inline]
pub fn __root<T>(subject: Annotated<T>) -> (Root<T>, SubjectKey<T>) {
    (Root { subject }, key())
}

/// The root of an assertion.
#[derive(Clone, Debug)]
pub struct Root<T> {
    subject: Annotated<T>,
}

impl<T, A> AssertionModifier<A> for Root<T>
where
    A: Assertion<T>,
{
    type Output = A::Output;

    #[inline]
    fn apply(self, cx: AssertionContextBuilder, assertion: A) -> Self::Output {
        assertion.execute(cx.innerner, self.subject.into_inner())
    }
}
