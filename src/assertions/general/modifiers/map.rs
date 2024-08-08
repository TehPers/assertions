use std::marker::PhantomData;

use crate::{
    assertions::{key, Assertion, AssertionContext, AssertionModifier, SubjectKey},
    metadata::Annotated,
};

/// Applies a mapping function to the subject before executing an assertion.
/// This is useful when the subject is a complex type and the assertion
/// should be applied to a specific field or property.
///
/// Since strings (both [`str`] and [`String`]) can't be directly iterated,
/// this method can be used to map a string to an iterator using the
/// [`str::chars`] method, [`str::bytes`] method, or any other method that
/// returns an iterator. This allows any combinators or assertions that
/// work with iterators to be used with strings as well.
///
/// ```
/// # use expecters::prelude::*;
/// expect!("abcd", map(str::chars), any, to_equal('b'));
/// // Ignoring the error message, the above code is equivalent to:
/// expect!("abcd".chars(), any, to_equal('b'));
/// ```
///
/// This method panics if the mapped target does not satisfy the assertion:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!("abcd", map(str::chars), any, to_equal('e'));
/// ```
#[inline]
pub fn map<M, T, U, F>(
    f: Annotated<F>,
) -> impl FnOnce(M, SubjectKey<T>) -> (MapModifier<M, T, U, F>, SubjectKey<U>)
where
    F: FnOnce(T) -> U,
{
    move |prev, _| {
        (
            MapModifier {
                prev,
                map: f,
                marker: PhantomData,
            },
            key(),
        )
    }
}

/// Modifier for [`map()`].
#[derive(Clone, Debug)]
pub struct MapModifier<M, T, U, F> {
    prev: M,
    map: Annotated<F>,
    marker: PhantomData<fn(T) -> U>,
}

impl<M, T, U, F, A> AssertionModifier<A> for MapModifier<M, T, U, F>
where
    M: AssertionModifier<MapAssertion<A, T, U, F>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, next: A) -> Self::Output {
        self.prev.apply(MapAssertion {
            next,
            map: self.map,
            marker: PhantomData,
        })
    }
}

/// Assertion for [`map()`].
#[derive(Clone, Debug)]
pub struct MapAssertion<A, T, U, F> {
    next: A,
    map: Annotated<F>,
    marker: PhantomData<fn(T) -> U>,
}

impl<A, T, U, F> Assertion<T> for MapAssertion<A, T, U, F>
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
