use crate::{
    assertions::{Assertion, AssertionContext, AssertionContextBuilder, AssertionModifier},
    metadata::Annotated,
};

/// Maps the subject to a new value.
#[derive(Clone, Debug)]
pub struct MapModifier<M, F> {
    prev: M,
    map: Annotated<F>,
}

impl<M, F> MapModifier<M, F> {
    #[inline]
    pub(crate) fn new(prev: M, map: Annotated<F>) -> Self {
        Self { prev, map }
    }
}

impl<M, F, A> AssertionModifier<A> for MapModifier<M, F>
where
    M: AssertionModifier<MapAssertion<A, F>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
        self.prev.apply(
            cx,
            MapAssertion {
                next,
                map: self.map,
            },
        )
    }
}

/// Maps the subject to a new value and executes an inner assertion on it.
#[derive(Clone, Debug)]
pub struct MapAssertion<A, F> {
    next: A,
    map: Annotated<F>,
}

impl<A, T, U, F> Assertion<T> for MapAssertion<A, F>
where
    A: Assertion<U>,
    F: FnOnce(T) -> U,
{
    type Output = A::Output;

    #[inline]
    fn execute(self, mut cx: AssertionContext, subject: T) -> Self::Output {
        cx.annotate("function", &self.map);

        let map = self.map.into_inner();
        self.next.execute(cx, map(subject))
    }
}
