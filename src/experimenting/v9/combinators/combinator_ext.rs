use crate::{assertions::SimpleAssertion, AssertionFailure};

use super::{AllCombinator, AnyCombinator, AssertionCombinator, NotCombinator};

pub trait AssertionCombinatorExt<Next>: AssertionCombinator<Next> + Sized {
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

    // /// Applies a mapping function to the target before applying the assertion.
    // /// This is useful when the target is a complex type and the assertion
    // /// should be applied to a specific field or property.
    // ///
    // /// Since strings (both [`str`] and [`String`]) can't be directly iterated,
    // /// this method can be used to map a string to an iterator using the
    // /// [`str::chars`] method, [`str::bytes`] method, or any other method that
    // /// returns an iterator. This allows any combinators or assertions that
    // /// work with iterators to be used with strings as well.
    // ///
    // /// ```
    // /// # use expecters::prelude::*;
    // /// expect!("abcd").map(str::chars).any().to_equal('b');
    // /// // Ignoring the error message, the above code is equivalent to:
    // /// expect!("abcd".chars()).any().to_equal('b');
    // /// ```
    // ///
    // /// This method panics if the mapped target does not satisfy the assertion:
    // ///
    // /// ```should_panic
    // /// # use expecters::prelude::*;
    // /// expect!("abcd").map(str::chars).any().to_equal('e');
    // /// ```
    // fn map<T, F>(self, map: F) -> MapCombinator<Self, F>
    // where
    //     F: FnMut(Self::Target) -> T,
    // {
    //     MapCombinator::new(self, map)
    // }

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
        Self::NextTarget: IntoIterator,
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
        Self::NextTarget: IntoIterator,
    {
        AnyCombinator::new(self)
    }

    // fn all(self) -> AllCombinator<Self>
    // where
    //     AllCombinator<Self>: AssertionCombinator<SimpleAssertion<fn(i32) -> AssertionResult>>,
    // {
    //     AllCombinator::new(self)
    // }
    // fn any(self) -> AnyCombinator<Self>
    // where
    //     AnyCombinator<Self>:
    //         AssertionCombinator<SimpleAssertion<fn(Self::Target) -> AssertionResult>>,
    // {
    //     AnyCombinator::new(self)
    // }

    // fn to_equal<U>(self, other: U) -> impl AssertionOutput
    // where
    //     Self::Target: PartialEq<U>,
    // {
    //     self.apply("the values to be equal", move |value| {
    //         (value == other)
    //             .then_some(())
    //             .ok_or_else(|| AssertionFailure::builder().build("the values to be equal"))
    //     })
    // }
}

impl<T, Next> AssertionCombinatorExt<Next> for T where T: AssertionCombinator<Next> {}

pub trait AssertionCombinatorAssertionsExt<Target>:
    AssertionCombinator<SimpleAssertion<Target>, NextTarget = Target> + Sized
{
    fn to_equal<U>(self, other: U)
    where
        Target: PartialEq<U>,
    {
        let assertion = SimpleAssertion::new("the values are equal", move |target| {
            if target == other {
                Ok(())
            } else {
                Err(AssertionFailure::builder().build("TODO"))
            }
        });
        let assertion = self.apply(assertion);
    }
}

impl<C, Target> AssertionCombinatorAssertionsExt<Target> for C where
    C: AssertionCombinator<SimpleAssertion<Target>, NextTarget = Target>
{
}
