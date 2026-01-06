use crate::{
    assertions::{pointers::PointerLike, Assertion, AssertionContext},
    metadata::Annotated,
    AssertionOutput,
};

/// Asserts that the subject points to the same location as another pointer.
#[derive(Clone, Debug)]
pub struct ToPointTo<U> {
    other: Annotated<U>,
}

impl<U> ToPointTo<U> {
    #[inline]
    pub(crate) fn new(other: Annotated<U>) -> Self {
        Self { other }
    }
}

impl<T, U> Assertion<T> for ToPointTo<U>
where
    T: PointerLike,
    U: PointerLike<Target = T::Target>,
{
    type Output = AssertionOutput;

    fn execute(self, mut cx: AssertionContext, subject: T) -> Self::Output {
        cx.annotate("other", &self.other);
        cx.pass_if(
            std::ptr::eq(
                PointerLike::as_ptr(&subject),
                PointerLike::as_ptr(self.other.inner()),
            ),
            "pointers are inequal",
        )
    }
}
