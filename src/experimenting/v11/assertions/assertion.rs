use crate::AssertionResult;

pub trait Assertion<T> {
    // type Output;

    fn execute(&mut self, target: T) -> AssertionResult;
}

impl<T, F> Assertion<T> for F
where
    F: FnMut(T) -> AssertionResult,
{
    #[inline]
    fn execute(&mut self, target: T) -> AssertionResult {
        self(target)
    }
}
