use crate::assertions::{
    general::IntoInitializableOutput, Assertion, AssertionContext, AssertionContextBuilder,
    AssertionModifier,
};

/// Reads a subject as UTF-8
#[derive(Clone, Debug)]
pub struct AsUtf8Modifier<M> {
    prev: M,
}

impl<M> AsUtf8Modifier<M> {
    #[inline]
    pub(crate) fn new(prev: M) -> Self {
        Self { prev }
    }
}

impl<M, A> AssertionModifier<A> for AsUtf8Modifier<M>
where
    M: AssertionModifier<AsUtf8Assertion<A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
        self.prev.apply(cx, AsUtf8Assertion { next })
    }
}

/// Reads the subject as UTF-8, then executes the inner assertion on it.
#[derive(Clone, Debug)]
pub struct AsUtf8Assertion<A> {
    next: A,
}

impl<A, T> Assertion<T> for AsUtf8Assertion<A>
where
    A: Assertion<String, Output: IntoInitializableOutput>,
    T: IntoIterator<Item = u8>,
{
    type Output = <A::Output as IntoInitializableOutput>::Initialized;

    fn execute(self, mut cx: AssertionContext, subject: T) -> Self::Output {
        let bytes = subject.into_iter().collect();
        let subject = match String::from_utf8(bytes) {
            Ok(subject) => subject,
            Err(error) => {
                cx.annotate("error", error);
                return cx.fail("invalid utf8");
            }
        };

        self.next.execute(cx, subject).into_initialized()
    }
}
