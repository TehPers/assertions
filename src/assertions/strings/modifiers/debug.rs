use std::fmt::Debug;

use crate::assertions::{
    key, Assertion, AssertionContext, AssertionContextBuilder, AssertionModifier, SubjectKey,
};

/// Converts a value to its [`Debug`] representation.
///
/// ```
/// # use expecters::prelude::*;
/// expect!("hello", as_debug, to_equal(r#""hello""#));
/// ```
#[inline]
pub fn as_debug<T, M>(prev: M, _: SubjectKey<T>) -> (AsDebugModifier<M>, SubjectKey<String>)
where
    T: Debug,
{
    (AsDebugModifier { prev }, key())
}

/// Modifier for [`as_debug()`].
#[derive(Clone, Debug)]
pub struct AsDebugModifier<M> {
    prev: M,
}

impl<M, A> AssertionModifier<A> for AsDebugModifier<M>
where
    M: AssertionModifier<AsDebugAssertion<A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
        self.prev.apply(cx, AsDebugAssertion { next })
    }
}

/// Assertion for [`as_debug()`].
#[derive(Clone, Debug)]
pub struct AsDebugAssertion<A> {
    next: A,
}

impl<A, T> Assertion<T> for AsDebugAssertion<A>
where
    A: Assertion<String>,
    T: Debug,
{
    type Output = A::Output;

    #[inline]
    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        self.next.execute(cx, format!("{subject:?}"))
    }
}
