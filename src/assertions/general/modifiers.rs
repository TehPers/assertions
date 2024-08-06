use crate::{assertions::AssertionContext, metadata::Annotated};

use super::InvertibleResult;

/// Inverts the result of an assertion.
///
/// If (and only if) the assertion is satisfied, then the result is treated as
/// a failure.
///
/// ```
/// # use expecters::prelude::*;
/// expect!(1, not, to_equal(2));
/// ```
///
/// This method panics if the assertion is satisfied:
///
/// ```should_panic
/// # use expecters::prelude::*;
/// expect!(1, not, to_equal(1));
/// ```
#[inline]
pub fn not<T, O>(
    cx: AssertionContext,
    subject: T,
    next: fn(AssertionContext, T) -> O,
) -> O::Inverted
where
    O: InvertibleResult,
{
    let output = next(cx.clone(), subject);
    output.invert(cx)
}

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
pub fn map<T, U, O, F>(
    mut f: Annotated<F>,
) -> impl FnOnce(AssertionContext, T, fn(AssertionContext, U) -> O) -> O
where
    F: FnMut(T) -> U,
{
    move |mut cx, subject, next| {
        cx.annotate("function", &f);
        let subject = (f.inner_mut())(subject);
        next(cx, subject)
    }
}
