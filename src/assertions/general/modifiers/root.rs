use crate::{
    assertions::{key, Assertion, AssertionContext, AssertionModifier, SubjectKey},
    metadata::Annotated,
};

#[doc(hidden)]
#[inline]
pub fn __root<T>(cx: AssertionContext, subject: Annotated<T>) -> (Root<T>, SubjectKey<T>) {
    (Root { cx, subject }, key())
}

/// The root of an assertion.
#[derive(Clone, Debug)]
pub struct Root<T> {
    cx: AssertionContext,
    subject: Annotated<T>,
}

impl<T, A> AssertionModifier<A> for Root<T>
where
    A: Assertion<T>,
{
    type Output = A::Output;

    #[inline]
    fn apply(self, assertion: A) -> Self::Output {
        assertion.execute(self.cx, self.subject.into_inner())
    }
}
