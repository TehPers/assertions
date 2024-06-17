use std::fmt::Display;

use crate::combinators::{
    AllCombinator, AnyCombinator, AtPath, CountCombinator, ErrCombinator, MapCombinator,
    NotCombinator, NthCombinator, OkCombinator, SomeCombinator, Traversal, WhenCalledCombinator,
};

/// A type that defines behavior for assertions.
///
/// See the methods on this trait for a list of built-in assertions and
/// combinators.
pub trait Assertable: Sized {
    /// The type of the target of the assertion.
    type Target;

    /// The result of an assertion. Normally, assertions are performed right
    /// away, so this type is `()`. However, in some cases, the result of an
    /// assertion might not be immediately known (e.g., when the assertion is
    /// on the result of a `Future`). In those cases, a value is returned
    /// instead which can be used to perform the assertion.
    type Result;

    /// Asserts that the target matches the given predicate. If the predicate
    /// is not satisfied, this method panics with a message that includes the
    /// given expectation.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(1).to_satisfy("value is odd", |n| n % 2 == 1);
    /// ```
    ///
    /// This method panics if the target does not satisfy the predicate:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(2).to_satisfy("value is odd", |n| n % 2 == 1);
    /// ```
    ///
    /// This method is the foundation for all other assertions. It is used to
    /// build more complex assertions by composing a complex expectation message
    /// and predicate function. If creating a new combinator, this method should
    /// be implemented to provide the basic functionality.
    fn to_satisfy<F>(self, expectation: impl Display, f: F) -> Self::Result
    where
        F: FnMut(Self::Target) -> bool;

    // COMBINATORS

    /// Negates an assertion. If the assertion is satisfied, then the result
    /// is treated as a failure, and if the assertion is not satisfied, then
    /// the result is treated as a success.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(1).not().to_equal(2);
    /// ```
    ///
    /// This method panics if the assertion is satisfied:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(1).not().to_equal(1);
    /// ```
    fn not(self) -> NotCombinator<Self> {
        NotCombinator::new(self)
    }

    /// Applies a mapping function to the target before applying the assertion.
    /// This is useful when the target is a complex type and the assertion
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
    /// expect!("abcd").map(str::chars).any().to_equal('b');
    /// // Ignoring the error message, the above code is equivalent to:
    /// expect!("abcd".chars()).any().to_equal('b');
    /// ```
    ///
    /// This method panics if the mapped target does not satisfy the assertion:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!("abcd").map(str::chars).any().to_equal('e');
    /// ```
    fn map<T, F>(self, map: F) -> MapCombinator<Self, F>
    where
        F: FnMut(Self::Target) -> T,
    {
        MapCombinator::new(self, map)
    }

    /// Applies an assertion to each element in the target. If any element does
    /// not satisfy the assertion, then the result is treated as a failure.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!([1, 3, 5]).all().to_be_less_than(10);
    /// ```
    ///
    /// This method panics if any element does not satisfy the assertion:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!([1, 3, 5]).all().to_equal(5);
    /// ```
    fn all(self) -> AllCombinator<Self>
    where
        Self::Target: IntoIterator,
    {
        AllCombinator::new(self)
    }

    /// Applies an assertion to each element in the target. If every element
    /// does not satisfy the assertion, then the result is treated as a failure.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!([1, 3, 5]).any().to_equal(5);
    /// ```
    ///
    /// This method panics if every element does not satisfy the assertion:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!([1, 3, 5]).any().to_equal(4);
    /// ```
    fn any(self) -> AnyCombinator<Self>
    where
        Self::Target: IntoIterator,
    {
        AnyCombinator::new(self)
    }

    /// Applies an assertion to the number of elements in the target. If the
    /// number of elements does not satisfy the assertion, then the result is
    /// treated as a failure.
    ///
    /// This uses the [`Iterator::count`] method to determine the number of
    /// elements in the target. If the target is an unbounded iterator, then
    /// this method will loop indefinitely.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!([1, 2, 3]).count().to_equal(3);
    /// ```
    ///
    /// This method panics if the number of elements does not satisfy the
    /// assertion:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!([1, 2, 3]).count().to_equal(4);
    /// ```
    fn count(self) -> CountCombinator<Self>
    where
        Self::Target: IntoIterator,
    {
        CountCombinator::new(self)
    }

    /// Applies an assertion to a specific element in the target. If the element
    /// does not exist or does not satisfy the assertion, then the result is
    /// treated as a failure. The index is zero-based.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!([1, 2, 3]).nth(1).to_equal(2);
    /// ```
    ///
    /// This method panics if the element does not exist:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!([1, 2, 3]).nth(3).to_equal(4);
    /// ```
    ///
    /// It also panics if the element does not satisfy the assertion:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!([1, 2, 3]).nth(1).to_equal(1);
    /// ```
    fn nth(self, n: usize) -> NthCombinator<Self>
    where
        Self::Target: IntoIterator,
    {
        NthCombinator::new(self, n)
    }

    /// Applies an assertion to the inner value of an [`Option<T>`]. If the
    /// option is [`None`], then the result is treated as a failure. Otherwise,
    /// the assertion is applied to the inner value.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(Some(1i32)).to_be_some_and().to_equal(1);
    /// ```
    ///
    /// This method panics if the option is [`None`]:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(None::<i32>).to_be_some_and().to_equal(2);
    /// ```
    ///
    /// It also panics if the inner value does not satisfy the assertion:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(Some(1i32)).to_be_some_and().to_equal(2);
    /// ```
    fn to_be_some_and<T>(self) -> SomeCombinator<Self>
    where
        Self: Assertable<Target = Option<T>>,
    {
        SomeCombinator::new(self)
    }

    /// Applies an assertion to the inner value of a [`Result<T, E>`]. If the
    /// result is [`Err`], then the result is treated as a failure. Otherwise,
    /// the assertion is applied to the inner value.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// let result: Result<i32, &str> = Ok(1);
    /// expect!(result).to_be_ok_and().to_equal(1);
    /// ```
    ///
    /// This method panics if the result is [`Err`]:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// let result: Result<i32, &str> = Err("error");
    /// expect!(result).to_be_ok_and().to_equal(1);
    /// ```
    ///
    /// It also panics if the inner value does not satisfy the assertion:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// let result: Result<i32, &str> = Ok(1);
    /// expect!(result).to_be_ok_and().to_equal(2);
    /// ```
    fn to_be_ok_and<T, E>(self) -> OkCombinator<Self>
    where
        Self: Assertable<Target = Result<T, E>>,
    {
        OkCombinator::new(self)
    }

    /// Applies an assertion to the error value of a [`Result<T, E>`]. If the
    /// result is [`Ok`], then the result is treated as a failure. Otherwise,
    /// the assertion is applied to the error value.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// let result: Result<i32, &str> = Err("error");
    /// expect!(result).to_be_err_and().to_equal("error");
    /// ```
    ///
    /// This method panics if the result is [`Ok`]:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// let result: Result<i32, &str> = Ok(1);
    /// expect!(result).to_be_err_and().to_equal("error");
    /// ```
    ///
    /// It also panics if the error value does not satisfy the assertion:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// let result: Result<i32, &str> = Err("error");
    /// expect!(result).to_be_err_and().to_equal("another error");
    /// ```
    fn to_be_err_and<T, E>(self) -> ErrCombinator<Self>
    where
        Self: Assertable<Target = Result<T, E>>,
    {
        ErrCombinator::new(self)
    }

    /// Applies an assertion to the return value of a function. This is
    /// equivalent to calling
    /// [`.when_called_with(())`](Assertable::when_called_with).
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(|| 1).when_called().to_equal(1);
    /// ```
    ///
    /// This method panics if the return value does not satisfy the assertion:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(|| 1).when_called().to_equal(2);
    /// ```
    fn when_called<R>(self) -> WhenCalledCombinator<Self, ()>
    where
        Self::Target: FnOnce() -> R,
    {
        WhenCalledCombinator::new(self, ())
    }

    /// Applies an assertion to the return value of a function when called with
    /// the given arguments.
    ///
    /// Arguments must be passed as a tuple, including for functions that take
    /// no arguments or a single argument. For single-argument functions, the
    /// argument must be passed like `(arg,)` to produce a tuple.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(|a, b| a + b).when_called_with((1, 2)).to_equal(3);
    /// expect!(|n| n * 2).when_called_with((2,)).to_equal(4);
    /// ```
    ///
    /// This method panics if the return value does not satisfy the assertion:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(|a, b| a + b).when_called_with((1, 2)).to_equal(4);
    /// ```
    ///
    /// Up to 12 arguments are supported. If more arguments are needed, consider
    /// calling [`map`](Assertable::map) instead to transform the function into
    /// its return value:
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(|a, b| a + b).map(|f| f(1, 2)).to_equal(3);
    /// ```
    fn when_called_with<Args, R>(self, args: Args) -> WhenCalledCombinator<Self, Args>
    where
        WhenCalledCombinator<Self, Args>: Assertable<Target = R>,
    {
        WhenCalledCombinator::new(self, args)
    }

    /// Applies an assertion to a sub-path of the target value. This is useful
    /// when the target is a complex type and the assertion should be applied to
    /// a specific field or property.
    ///
    /// Unlike [`map`](Assertable::map), this method allows you to access deeply
    /// nested values, even through fallible layers (like values with type
    /// [`Option`] or [`Result`]), using a simple path syntax. The path is
    /// included with the generated error message to help identify the source of
    /// the assertion failure.
    ///
    /// To generate the path, and for more information on the syntax, see the
    /// [`path!`](crate::path) macro.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// struct Foo(i32);
    ///
    /// expect!(Foo(3)).at_path(path!(.0)).to_equal(3);
    /// ```
    ///
    /// This method panics if the sub-path cannot be navigated to due to
    /// fallible components:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// struct Foo(Option<i32>);
    ///
    /// expect!(Foo(None))
    ///     .at_path(path!(.0?))
    ///     .to_satisfy("always succeed", |_| true);
    /// ```
    fn at_path<T>(self, path: Traversal<Self::Target, T>) -> AtPath<Self, T> {
        AtPath::new(self, path)
    }

    // ASSERTIONS

    /// Asserts that the target is equal to the given value.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(1).to_equal(1);
    /// ```
    ///
    /// This method panics if the target is not equal to the given value:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(1).to_equal(2);
    /// ```
    #[inline]
    fn to_equal<T>(self, other: T) -> Self::Result
    where
        Self::Target: PartialEq<T>,
    {
        self.to_satisfy("value is equal to a provided value", move |t| t == other)
    }

    /// Asserts that the target is less than the given value.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(1).to_be_less_than(2);
    /// ```
    ///
    /// This method panics if the target is not less than the given value:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(2).to_be_less_than(1);
    /// ```
    #[inline]
    fn to_be_less_than<T>(self, other: T) -> Self::Result
    where
        Self::Target: PartialOrd<T>,
    {
        self.to_satisfy("value is less than the input", move |t| t < other)
    }

    /// Asserts that the target is less than or equal to the given value.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(1).to_be_less_than_or_equal_to(1);
    /// expect!(1).to_be_less_than_or_equal_to(2);
    /// ```
    ///
    /// This method panics if the target is greater less the given value:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(2).to_be_less_than_or_equal_to(1);
    /// ```
    #[inline]
    fn to_be_less_than_or_equal_to<T>(self, other: T) -> Self::Result
    where
        Self::Target: PartialOrd<T>,
    {
        self.to_satisfy("value is less than or equal to the input", move |t| {
            t <= other
        })
    }

    /// Asserts that the target is greater than the given value.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(2).to_be_greater_than(1);
    /// ```
    ///
    /// This method panics if the target is not greater than the given value:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(1).to_be_greater_than(2);
    /// ```
    #[inline]
    fn to_be_greater_than<T>(self, other: T) -> Self::Result
    where
        Self::Target: PartialOrd<T>,
    {
        self.to_satisfy("value is greater than the input", move |t| t > other)
    }

    /// Asserts that the target is greater than or equal to the given value.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(1).to_be_greater_than_or_equal_to(1);
    /// expect!(1).to_be_greater_than_or_equal_to(0);
    /// ```
    ///
    /// This method panics if the target is less than than the given value:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(1).to_be_greater_than_or_equal_to(2);
    /// ```
    #[inline]
    fn to_be_greater_than_or_equal_to<T>(self, other: T) -> Self::Result
    where
        Self::Target: PartialOrd<T>,
    {
        self.to_satisfy("value is greater than or equal to the input", move |t| {
            t >= other
        })
    }

    /// Asserts that the target is empty.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(Vec::<i32>::new()).to_be_empty();
    /// expect!("".chars()).to_be_empty();
    /// ```
    ///
    /// This method panics if the target is not empty:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!([1, 2, 3]).to_be_empty();
    /// ```
    #[inline]
    fn to_be_empty(self) -> Self::Result
    where
        Self::Target: IntoIterator,
    {
        self.to_satisfy("value is empty", |value| value.into_iter().next().is_none())
    }

    /// Asserts that the target holds a value.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(Some(1i32)).to_be_some();
    /// ```
    ///
    /// This method panics if the target does not hold a value:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(None::<i32>).to_be_some();
    /// ```
    #[inline]
    fn to_be_some<T>(self) -> Self::Result
    where
        Self: Assertable<Target = Option<T>>,
    {
        self.to_satisfy("value is `Some`", |value| value.is_some())
    }

    /// Asserts that the target does not hold a value.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(None::<i32>).to_be_none();
    /// ```
    ///
    /// This method panics if the target holds a value:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(Some(1i32)).to_be_none();
    /// ```
    #[inline]
    fn to_be_none<T>(self) -> Self::Result
    where
        Self: Assertable<Target = Option<T>>,
    {
        self.to_satisfy("value is `None`", |value| value.is_none())
    }

    /// Asserts that the target holds a success.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// let result: Result<i32, &str> = Ok(1);
    /// expect!(result).to_be_ok();
    /// ```
    ///
    /// This method panics if the target does not hold a success:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// let result: Result<i32, &str> = Err("error");
    /// expect!(result).to_be_ok();
    /// ```
    #[inline]
    fn to_be_ok<T, E>(self) -> Self::Result
    where
        Self: Assertable<Target = Result<T, E>>,
    {
        self.to_satisfy("value is `Ok`", |value| value.is_ok())
    }

    /// Asserts that the target holds an error.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// let result: Result<i32, &str> = Err("error");
    /// expect!(result).to_be_err();
    /// ```
    ///
    /// This method panics if the target does not hold an error:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// let result: Result<i32, &str> = Ok(1);
    /// expect!(result).to_be_err();
    /// ```
    #[inline]
    fn to_be_err<T, E>(self) -> Self::Result
    where
        Self: Assertable<Target = Result<T, E>>,
    {
        self.to_satisfy("value is `Err`", |value| value.is_err())
    }

    /// Asserts that the target is `true`.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(true).to_be_true();
    /// ```
    ///
    /// This method panics if the target is `false`:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(false).to_be_true();
    /// ```
    fn to_be_true(self) -> Self::Result
    where
        Self::Target: Into<bool>,
    {
        self.to_satisfy("value is `true`", |value| value.into())
    }

    /// Asserts that the target is `false`.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(false).to_be_false();
    /// ```
    ///
    /// This method panics if the target is `true`:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(true).to_be_false();
    /// ```
    fn to_be_false(self) -> Self::Result
    where
        Self::Target: Into<bool>,
    {
        self.to_satisfy("value is `false`", |value| !value.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::expect;

    use super::*;

    #[test]
    fn all_not() {
        expect!([1, 2, 3]).all().not().to_equal(4);
        expect!([1, 2, 3]).not().all().to_equal(3);
    }

    #[test]
    #[should_panic]
    fn all_not_fails() {
        expect!([1, 2, 3]).all().not().to_equal(3);
    }

    #[test]
    #[should_panic]
    fn not_all_fails() {
        expect!([1, 2, 3]).not().to_be_empty();
        expect!([1, 2, 3]).not().all().to_be_less_than(4);
    }

    #[test]
    fn any_not() {
        expect!([1, 2, 3]).any().not().to_equal(4);
        expect!([1, 2, 3]).not().any().to_equal(4);
    }

    #[test]
    fn many_args_called_with() {
        fn sum(a: i32, b: i32, c: i32) -> i32 {
            a + b + c
        }
        expect!(sum).when_called_with((1, 2, 3)).to_equal(6);
    }
}
