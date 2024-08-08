use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::assertions::{key, Assertion, AssertionContext, AssertionModifier, SubjectKey};

/// Converts a value to its [`Display`] representation.
///
/// ```
/// # use expecters::prelude::*;
/// expect!(1, as_display, to_equal("1"));
/// ```
#[inline]
pub fn as_display<T, M>(prev: M, _: SubjectKey<T>) -> (AsDisplayModifier<T, M>, SubjectKey<String>)
where
    T: Display,
{
    (
        AsDisplayModifier {
            prev,
            marker: PhantomData,
        },
        key(),
    )
}

/// Modifier for [`as_display()`].
#[derive(Clone, Debug)]
pub struct AsDisplayModifier<T, M> {
    prev: M,
    marker: PhantomData<fn(T)>,
}

impl<T, M, A> AssertionModifier<A> for AsDisplayModifier<T, M>
where
    M: AssertionModifier<AsDisplayAssertion<A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, next: A) -> Self::Output {
        self.prev.apply(AsDisplayAssertion { next })
    }
}

/// Assertion for [`as_display()`].
#[derive(Clone, Debug)]
pub struct AsDisplayAssertion<A> {
    next: A,
}

impl<A, T> Assertion<T> for AsDisplayAssertion<A>
where
    A: Assertion<String>,
    T: Display,
{
    type Output = A::Output;

    #[inline]
    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        self.next.execute(cx, subject.to_string())
    }
}
