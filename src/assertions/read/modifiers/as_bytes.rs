use std::io::Read;

use crate::assertions::{
    general::IntoInitializableOutput, Assertion, AssertionContext, AssertionContextBuilder,
    AssertionModifier,
};

/// Reads a subject into a buffer.
#[derive(Clone, Debug)]
pub struct WhenReadAsBytesModifier<M> {
    prev: M,
}

impl<M> WhenReadAsBytesModifier<M> {
    #[inline]
    pub(crate) fn new(prev: M) -> Self {
        WhenReadAsBytesModifier { prev }
    }
}

impl<M, A> AssertionModifier<A> for WhenReadAsBytesModifier<M>
where
    M: AssertionModifier<WhenReadAsBytesAssertion<A>>,
{
    type Output = M::Output;

    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
        self.prev.apply(cx, WhenReadAsBytesAssertion { next })
    }
}

/// Reads the subject into a buffer and executes the inner assertion on it.
#[derive(Clone, Debug)]
pub struct WhenReadAsBytesAssertion<A> {
    next: A,
}

impl<T, A> Assertion<T> for WhenReadAsBytesAssertion<A>
where
    T: Read,
    A: Assertion<Vec<u8>, Output: IntoInitializableOutput>,
{
    type Output = <A::Output as IntoInitializableOutput>::Initialized;

    fn execute(self, mut cx: AssertionContext, subject: T) -> Self::Output {
        let bytes = match subject.bytes().collect::<Result<Vec<_>, _>>() {
            Ok(bytes) => bytes,
            Err(error) => {
                cx.annotate("error", &error);
                return cx.fail("failed to read");
            }
        };

        self.next.execute(cx, bytes).into_initialized()
    }
}
