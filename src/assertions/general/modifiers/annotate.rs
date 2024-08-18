use std::fmt::Debug;

use crate::{
    assertions::{
        Assertion, AssertionBuilder, AssertionContext, AssertionContextBuilder, AssertionModifier,
    },
    metadata::Annotated,
};

#[doc(hidden)]
pub fn __annotate<T, M>(
    builder: AssertionBuilder<T, M>,
    annotate: fn(T) -> Annotated<T>,
) -> AssertionBuilder<T, AnnotateModifier<T, M>> {
    AssertionBuilder::modify(builder, |prev| AnnotateModifier { prev, annotate })
}

/// Annotates and records input values, and updates the [`AssertionContext`]
/// after modifiers are applied. When using the [`expect!`](crate::expect!)
/// macro, this is applied automatically before every modifier and the final
/// assertion in the chain.
pub struct AnnotateModifier<T, M> {
    prev: M,
    annotate: fn(T) -> Annotated<T>,
}

impl<T, M> Clone for AnnotateModifier<T, M>
where
    M: Clone,
{
    fn clone(&self) -> Self {
        Self {
            prev: self.prev.clone(),
            annotate: self.annotate,
        }
    }
}

impl<T, M> Debug for AnnotateModifier<T, M>
where
    M: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnnotateModifier")
            .field("prev", &self.prev)
            .field("annotate", &self.annotate)
            .finish()
    }
}

impl<T, M, A> AssertionModifier<A> for AnnotateModifier<T, M>
where
    M: AssertionModifier<AnnotateAssertion<T, A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, cx: AssertionContextBuilder, assertion: A) -> Self::Output {
        self.prev.apply(
            cx,
            AnnotateAssertion {
                next: assertion,
                annotate: self.annotate,
            },
        )
    }
}

/// Assertion for [`AnnotateModifier`]. See the docs for the modifier for more
/// information.
pub struct AnnotateAssertion<T, A> {
    next: A,
    annotate: fn(T) -> Annotated<T>,
}

impl<T, A> Clone for AnnotateAssertion<T, A>
where
    A: Clone,
{
    fn clone(&self) -> Self {
        Self {
            next: self.next.clone(),
            annotate: self.annotate,
        }
    }
}

impl<T, A> Debug for AnnotateAssertion<T, A>
where
    A: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnnotateAssertion")
            .field("next", &self.next)
            .field("annotate", &self.annotate)
            .finish()
    }
}

impl<T, A> Assertion<T> for AnnotateAssertion<T, A>
where
    A: Assertion<T>,
{
    type Output = A::Output;

    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        let mut cx = cx.next();
        let subject = (self.annotate)(subject);

        // Track the received value in the context
        if let Some(debug) = subject.as_debug() {
            cx.annotate("received", format_args!("{debug:?}"));
        } else {
            cx.annotate("received", "? (no debug representation)");
        }

        self.next.execute_annotated(cx, subject)
    }

    #[inline]
    fn execute_annotated(self, _cx: AssertionContext, _subject: Annotated<T>) -> Self::Output
    where
        Self: Sized,
    {
        unreachable!("call execute() instead")
    }
}
