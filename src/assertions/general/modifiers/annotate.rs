use crate::{
    assertions::{key, Assertion, AssertionContext, AssertionModifier, SubjectKey},
    metadata::{Annotated, AnnotatedKind},
};

#[doc(hidden)]
pub fn __annotate<T, M>(
    prev: M,
    _: SubjectKey<T>,
    annotate: fn(T) -> Annotated<T>,
) -> (AnnotateModifier<T, M>, SubjectKey<T>) {
    (AnnotateModifier { prev, annotate }, key())
}

/// Annotates and records input values, and updates the [`AssertionContext`]
/// after modifiers are applied. When using the [`expect!`](crate::expect!)
/// macro, this is applied automatically before every modifier and the final
/// assertion in the chain.
#[derive(Clone, Debug)]
pub struct AnnotateModifier<T, M> {
    prev: M,
    annotate: fn(T) -> Annotated<T>,
}

impl<T, M, A> AssertionModifier<A> for AnnotateModifier<T, M>
where
    M: AssertionModifier<AnnotateAssertion<A, T>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, assertion: A) -> Self::Output {
        self.prev.apply(AnnotateAssertion {
            next: assertion,
            annotate: self.annotate,
        })
    }
}

/// Assertion for [`AnnotateModifier`]. See the docs for the modifier for more
/// information.
#[derive(Clone, Debug)]
pub struct AnnotateAssertion<A, T> {
    next: A,
    annotate: fn(T) -> Annotated<T>,
}

impl<A, T> Assertion<T> for AnnotateAssertion<A, T>
where
    A: Assertion<T>,
{
    type Output = A::Output;

    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        let mut next_cx = cx.next();
        let annotated = (self.annotate)(subject);
        next_cx.annotate(
            "received",
            match annotated.kind() {
                AnnotatedKind::Debug => annotated.as_str(),
                AnnotatedKind::Stringify => "? (no debug representation)",
            },
        );

        self.next.execute(next_cx, annotated.into_inner())
    }
}
