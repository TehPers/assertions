use std::cmp::Ordering;

use crate::{
    assertions::{AssertionBuilder, AssertionError},
    metadata::Annotated,
};

use super::{
    Float, MapModifier, NotModifier, ToBeOneOfAssertion, ToCmpAssertion, ToEqualApproxAssertion,
    ToEqualAssertion, ToSatisfyAssertion, ToSatisfyWithAssertion,
};

/// General-purpose assertions and modifiers.
pub trait GeneralAssertions<T, M> {
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
    /// The assertion fails if the assertion is satisfied:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(1, not, to_equal(1));
    /// ```
    fn not(self) -> AssertionBuilder<T, NotModifier<M>>;

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
    /// The assertion fails if the mapped subject does not satisfy the assertion:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!("abcd", map(str::chars), any, to_equal('e'));
    /// ```
    ///
    /// ## Type inference
    ///
    /// The Rust compiler can sometimes have trouble inferring the type of the value
    /// being mapped. This is due to how the [`expect!`] macro is implemented. The
    /// macro wraps the mapping function passed to this modifier to annotate it, but
    /// in the process needs to know what the exact type of the closure is and can
    /// sometimes struggle to infer it.
    ///
    /// If type inference is an issue, provide the specific type in the closure. For
    /// example, this fails to compile:
    ///
    /// ```compile_fail
    /// # use expecters::prelude::*;
    /// struct MyStruct<T>(T);
    /// expect!(MyStruct(1), map(|n| n.0), to_equal(1));
    /// ```
    ///
    /// Providing a specific type (through a pattern or by specifying the exact
    /// type) solves this:
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// struct MyStruct<T>(T);
    /// expect!(MyStruct(1), map(|n: MyStruct<i32>| n.0), to_equal(1));
    /// expect!(MyStruct(1), map(|MyStruct(n)| n), to_equal(1));
    /// ```
    ///
    /// [`expect!`]: crate::expect!
    fn map<U, F>(self, f: Annotated<F>) -> AssertionBuilder<U, MapModifier<M, F>>;

    /// Asserts that the subject matches the given predicate.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(1, to_satisfy(|n| n % 2 == 1));
    /// ```
    ///
    /// The assertion fails if the subject does not satisfy the predicate:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(2, to_satisfy(|n| n % 2 == 1));
    /// ```
    ///
    /// Since the predicate that is passed into this function will be included in
    /// the failure message if the assertion fails, it is recommended to keep the
    /// predicate short and simple to keep failure message readable. If a more
    /// complex predicate is needed, it's possible to define a separate function and
    /// pass that function in as an argument instead:
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// fn is_odd(n: i32) -> bool {
    ///     n % 2 == 1
    /// }
    ///
    /// expect!(1, to_satisfy(is_odd));
    /// ```
    #[inline]
    fn to_satisfy<F>(&self, predicate: Annotated<F>) -> ToSatisfyAssertion<F>
    where
        F: FnOnce(T) -> bool,
    {
        ToSatisfyAssertion::new(predicate)
    }

    /// Asserts that the subject matches a series of inner assertions. This
    /// "forks" the assertion, allowing an intermediate value to have several
    /// different assertions applied to it.
    ///
    /// This assertion expects a function to be provided to it which performs
    /// some inner assertions on the value, returning a
    /// [`Result<(), AssertionError>`] to indicate whether the assertion should
    /// pass or fail.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(
    ///     [1, 2, 3],
    ///     count,
    ///     to_satisfy_with(|value| {
    ///         try_expect!(value, to_be_greater_than(0))?;
    ///         try_expect!(value, to_be_less_than(4))?;
    ///         Ok(())
    ///     }),
    /// );
    /// ```
    ///
    /// The assertion fails if any of the results were failures:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(
    ///     [1, 2, 3],
    ///     count,
    ///     to_satisfy_with(|value| {
    ///         try_expect!(value, to_be_greater_than(3))?;
    ///         Ok(())
    ///     }),
    /// );
    /// ```
    ///
    /// This does **not** work if passed an async function:
    ///
    /// ```compile_fail
    /// # use expecters::prelude::*;
    /// expect!(
    ///     [ready(1), ready(2), ready(3)],
    ///     all,
    ///     to_satisfy_with(|value| async move {
    ///         try_expect!(value, when_ready, to_be_greater_than(0)).await?;
    ///         Ok(())
    ///     })
    /// )
    /// ```
    // TODO: make an async version
    #[inline]
    fn to_satisfy_with<F>(&self, predicate: Annotated<F>) -> ToSatisfyWithAssertion<F>
    where
        F: FnOnce(T) -> Result<(), AssertionError>,
    {
        ToSatisfyWithAssertion::new(predicate)
    }

    /// Asserts that the subject is equal to the given value.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(1, to_equal(1));
    /// ```
    ///
    /// The assertion fails if the subject is not equal to the given value:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(1, to_equal(2));
    /// ```
    #[inline]
    fn to_equal<U>(&self, expected: Annotated<U>) -> ToEqualAssertion<U>
    where
        T: PartialEq<U>,
    {
        ToEqualAssertion::new(expected)
    }

    /// Asserts that the subject is within a specified range of another value.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(0.9, to_equal_approximately(1.0, 0.2));
    /// ```
    ///
    /// The assertion fails if the subject is out of range:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(0.7, to_equal_approximately(1.0, 0.2));
    /// ```
    #[inline]
    fn to_equal_approximately(
        &self,
        expected: Annotated<T>,
        max_delta: Annotated<T>,
    ) -> ToEqualApproxAssertion<T>
    where
        T: Float,
    {
        ToEqualApproxAssertion::new(expected, max_delta)
    }

    /// Asserts that the subject is less than the given value.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(1, to_be_less_than(2));
    /// ```
    ///
    /// The assertion fails if the subject is not less than the given value:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(2, to_be_less_than(1));
    /// ```
    #[inline]
    fn to_be_less_than<U>(&self, boundary: Annotated<U>) -> ToCmpAssertion<U>
    where
        T: PartialOrd<U>,
    {
        ToCmpAssertion::new(boundary, Ordering::Less, false, "less than")
    }

    /// Asserts that the subject is less than or equal to the given value.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(1, to_be_less_than_or_equal_to(1));
    /// expect!(1, to_be_less_than_or_equal_to(2));
    /// ```
    ///
    /// The assertion fails if the subject is greater less the given value:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(2, to_be_less_than_or_equal_to(1));
    /// ```
    #[inline]
    fn to_be_less_than_or_equal_to<U>(&self, boundary: Annotated<U>) -> ToCmpAssertion<U>
    where
        T: PartialOrd<U>,
    {
        ToCmpAssertion::new(boundary, Ordering::Less, true, "less than or equal to")
    }

    /// Asserts that the subject is greater than the given value.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(2, to_be_greater_than(1));
    /// ```
    ///
    /// The assertion fails if the subject is not greater than the given value:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(1, to_be_greater_than(2));
    /// ```
    #[inline]
    fn to_be_greater_than<U>(&self, boundary: Annotated<U>) -> ToCmpAssertion<U>
    where
        T: PartialOrd<U>,
    {
        ToCmpAssertion::new(boundary, Ordering::Greater, false, "greater than")
    }

    /// Asserts that the subject is greater than or equal to the given value.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(1, to_be_greater_than_or_equal_to(1));
    /// expect!(1, to_be_greater_than_or_equal_to(0));
    /// ```
    ///
    /// The assertion fails if the subject is less than than the given value:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(1, to_be_greater_than_or_equal_to(2));
    /// ```
    #[inline]
    fn to_be_greater_than_or_equal_to<U>(&self, boundary: Annotated<U>) -> ToCmpAssertion<U>
    where
        T: PartialOrd<U>,
    {
        ToCmpAssertion::new(
            boundary,
            Ordering::Greater,
            true,
            "greater than or equal to",
        )
    }

    /// Asserts that the subject is equal to an item in the given sequence.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(1, to_be_one_of([1, 2, 3]));
    /// ```
    ///
    /// The assertion fails if the subject is not in the sequence or if the
    /// sequence is empty:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(1, to_be_one_of([2, 3, 4]));
    /// ```
    #[inline]
    fn to_be_one_of<I>(&self, items: Annotated<I>) -> ToBeOneOfAssertion<I>
    where
        I: IntoIterator,
        T: PartialEq<I::Item>,
    {
        ToBeOneOfAssertion::new(items)
    }
}

impl<T, M> GeneralAssertions<T, M> for AssertionBuilder<T, M> {
    #[inline]
    fn not(self) -> AssertionBuilder<T, NotModifier<M>> {
        AssertionBuilder::modify(self, NotModifier::new)
    }

    #[inline]
    fn map<U, F>(self, f: Annotated<F>) -> AssertionBuilder<U, MapModifier<M, F>> {
        AssertionBuilder::modify(self, move |prev| MapModifier::new(prev, f))
    }
}
