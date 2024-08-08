use std::{fmt::Debug, marker::PhantomData};

use crate::assertions::{key, Assertion, AssertionContext, AssertionModifier, SubjectKey};

/// Converts a value to its [`Debug`] representation.
///
/// ```
/// # use expecters::prelude::*;
/// expect!("hello", as_debug, to_equal(r#""hello""#));
/// ```
#[inline]
pub fn as_debug<T, M>(prev: M, _: SubjectKey<T>) -> (AsDebugModifier<T, M>, SubjectKey<String>)
where
    T: Debug,
{
    (
        AsDebugModifier {
            prev,
            marker: PhantomData,
        },
        key(),
    )
}

/// Modifier for [`as_debug()`].
#[derive(Clone, Debug)]
pub struct AsDebugModifier<T, M> {
    prev: M,
    marker: PhantomData<fn(T)>,
}

impl<T, M, A> AssertionModifier<A> for AsDebugModifier<T, M>
where
    M: AssertionModifier<AsDebugAssertion<A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, next: A) -> Self::Output {
        self.prev.apply(AsDebugAssertion { next })
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
