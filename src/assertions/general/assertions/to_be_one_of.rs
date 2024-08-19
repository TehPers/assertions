use crate::{
    assertions::{Assertion, AssertionContext},
    metadata::Annotated,
    AssertionOutput,
};

/// Asserts that the subject is equal to one of the items in an iterator.
#[derive(Clone, Debug)]
pub struct ToBeOneOf<I> {
    items: Annotated<I>,
}

impl<I> ToBeOneOf<I> {
    #[inline]
    pub(crate) fn new(items: Annotated<I>) -> Self {
        Self { items }
    }
}

impl<I, T> Assertion<T> for ToBeOneOf<I>
where
    I: IntoIterator,
    T: PartialEq<I::Item>,
{
    type Output = AssertionOutput;

    fn execute(self, mut cx: AssertionContext, subject: T) -> Self::Output {
        cx.annotate("items", &self.items);

        for item in self.items.into_inner() {
            if subject == item {
                return cx.pass();
            }
        }

        cx.fail("not found")
    }
}
