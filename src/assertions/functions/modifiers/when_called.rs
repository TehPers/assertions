use crate::{
    assertions::{
        functions::ApplyFnOnce, Assertion, AssertionContext, AssertionContextBuilder,
        AssertionModifier,
    },
    metadata::Annotated,
};

/// Calls the subject.
#[derive(Clone, Debug)]
pub struct WhenCalledModifier<M, I = ()> {
    prev: M,
    args: Annotated<I>,
}

impl<M, I> WhenCalledModifier<M, I> {
    #[inline]
    pub(crate) fn new(prev: M, args: Annotated<I>) -> Self {
        Self { prev, args }
    }
}

impl<M, I, A> AssertionModifier<A> for WhenCalledModifier<M, I>
where
    M: AssertionModifier<WhenCalledAssertion<A, I>>,
{
    type Output = M::Output;

    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
        self.prev.apply(
            cx,
            WhenCalledAssertion {
                next,
                args: self.args,
            },
        )
    }
}

/// Calls the subject and executes the inner assertion on the returned value.
#[derive(Clone, Debug)]
pub struct WhenCalledAssertion<A, I = ()> {
    next: A,
    args: Annotated<I>,
}

impl<A, I, F> Assertion<F> for WhenCalledAssertion<A, I>
where
    A: Assertion<F::Output>,
    F: ApplyFnOnce<I>,
{
    type Output = A::Output;

    fn execute(self, mut cx: AssertionContext, subject: F) -> Self::Output {
        if !F::EMPTY_ARGS {
            cx.annotate("args", &self.args);
        }

        let subject = subject.apply_once(self.args.into_inner())();
        self.next.execute(cx, subject)
    }
}
