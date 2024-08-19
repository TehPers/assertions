use crate::{
    assertions::{Assertion, AssertionContext},
    metadata::Annotated,
    AssertionOutput,
};

/// Asserts that the subject is equal to the given sequence.
#[derive(Clone, Debug)]
pub struct ToContainExactly<I> {
    expected: Annotated<I>,
}

impl<I> ToContainExactly<I> {
    #[inline]
    pub(crate) fn new(expected: Annotated<I>) -> Self {
        Self { expected }
    }
}

impl<I, T> Assertion<T> for ToContainExactly<I>
where
    I: IntoIterator,
    T: IntoIterator<Item: PartialEq<I::Item>>,
{
    type Output = AssertionOutput;

    fn execute(self, mut cx: AssertionContext, subject: T) -> Self::Output {
        cx.annotate("expected", &self.expected);

        let mut subject = subject.into_iter();
        let mut expected = self.expected.into_inner().into_iter();
        let mut idx = 0;
        loop {
            let error = match (subject.next(), expected.next()) {
                (None, None) => return cx.pass(),
                (Some(left), Some(right)) if left == right => {
                    idx += 1;
                    continue;
                }
                (Some(_), Some(_)) => "values not equal",
                (Some(_), None) => "subject has too many elements",
                (None, Some(_)) => "subject has too few elements",
            };

            // Return a failure
            cx.annotate("index", idx);
            return cx.fail(error);
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::prelude::*;

    #[test_case([1, 2, 3], [1, 2, 4]; "elements not equal")]
    #[test_case([1, 2], [1, 2, 3]; "too short")]
    #[test_case([1, 2, 3, 4], [1, 2, 3]; "too long")]
    #[should_panic = "assertion failed"]
    fn failure_cases<A, B>(left: A, right: B)
    where
        A: IntoIterator<Item: PartialEq<B::Item>>,
        B: IntoIterator,
    {
        expect!(left, to_contain_exactly(right));
    }
}
