use crate::{assertions::AssertionBuilder, metadata::Annotated};

use super::{CountModifier, MergeModifier, MergeStrategy, NthModifier, ToContainAssertion};

/// Assertions and modifiers for [Iterator]s.
pub trait IteratorAssertions<T, M>
where
    T: IntoIterator,
{
    /// Executes an assertion on every value within the subject, and succeeds if and
    /// only if none of the assertions fail.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!([1, 3, 5], all, to_be_less_than(10));
    /// expect!([] as [i32; 0], all, to_equal(1));
    /// ```
    ///
    /// The assertion fails if any element does not satisfy the assertion:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!([1, 3, 5], all, to_equal(5));
    /// ```
    ///
    /// Requires that the rest of the assertion is [`Clone`]. The subject of the
    /// assertion doesn't need to be cloneable, but the rest of the assertion does.
    /// For example, this works fine:
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// #[derive(PartialEq)]
    /// struct NotClone(i32);
    /// expect!([NotClone(0)], all, to_satisfy(|x| x == NotClone(0)));
    /// ```
    ///
    /// This does not though since `to_equal` takes ownership of a non-cloneable
    /// value:
    ///
    /// ```compile_fail
    /// # use expecters::prelude::*;
    /// #[derive(PartialEq)]
    /// struct NotClone(i32);
    /// expect!([NotClone(0)], all, to_equal(NonClone(0)));
    /// ```
    fn all(self) -> AssertionBuilder<T::Item, MergeModifier<M>>;

    /// Executes an assertion on every value within the subject, and succeeds if and
    /// only if an assertion succeeds.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!([1, 3, 5], any, to_equal(5));
    /// expect!([] as [i32; 0], not, any, to_equal(1));
    /// ```
    ///
    /// The assertion fails if no element satisfies the assertion:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!([1, 3, 5], any, to_equal(4));
    /// ```
    ///
    /// Requires that the rest of the assertion is [`Clone`]. The subject of the
    /// assertion doesn't need to be cloneable, but the rest of the assertion does.
    /// For example, this works fine:
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// #[derive(PartialEq)]
    /// struct NotClone(i32);
    /// expect!([NotClone(0)], any, to_satisfy(|x| x == NotClone(0)));
    /// ```
    ///
    /// This does not though since `to_equal` takes ownership of a non-cloneable
    /// value:
    ///
    /// ```compile_fail
    /// # use expecters::prelude::*;
    /// #[derive(PartialEq)]
    /// struct NotClone(i32);
    /// expect!([NotClone(0)], any, to_equal(NonClone(0)));
    /// ```
    fn any(self) -> AssertionBuilder<T::Item, MergeModifier<M>>;

    /// Counts the length of the subject, and executes an assertion on the result.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!([1, 2, 3], count, to_equal(3));
    /// ```
    ///
    /// This uses the [`Iterator::count`] method to determine the number of elements
    /// in the subject. If the subject is an unbounded iterator, then the assertion
    /// will not complete (unless it panics for another reason). See the iterator
    /// method for more information.
    fn count(self) -> AssertionBuilder<usize, CountModifier<M>>;

    /// Applies an assertion to a specific element in the target. If the element
    /// does not exist or does not satisfy the assertion, then the result is
    /// treated as a failure. The index is zero-based.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!([1, 2, 3], nth(1), to_equal(2));
    /// ```
    ///
    /// The assertion fails if the element does not exist:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!([1, 2, 3], nth(3), to_equal(4));
    /// ```
    ///
    /// It also fails if the element does not satisfy the assertion:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!([1, 2, 3], nth(1), to_equal(1));
    /// ```
    fn nth(self, index: Annotated<usize>) -> AssertionBuilder<T::Item, NthModifier<M>>;

    /// Asserts that the subject contains an element.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!([1, 2, 3], to_contain(3));
    /// ```
    ///
    /// This assertion fails if the element is not found:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!([1, 2, 3], to_contain(4));
    /// ```
    #[inline]
    fn to_contain<U>(&self, item: Annotated<U>) -> ToContainAssertion<U>
    where
        T::Item: PartialEq<U>,
    {
        ToContainAssertion::new(item)
    }
}

impl<T, M> IteratorAssertions<T, M> for AssertionBuilder<T, M>
where
    T: IntoIterator,
{
    #[inline]
    fn all(self) -> AssertionBuilder<T::Item, MergeModifier<M>> {
        AssertionBuilder::modify(self, |prev| MergeModifier::new(prev, MergeStrategy::All))
    }

    #[inline]
    fn any(self) -> AssertionBuilder<T::Item, MergeModifier<M>> {
        AssertionBuilder::modify(self, |prev| MergeModifier::new(prev, MergeStrategy::Any))
    }

    #[inline]
    fn count(self) -> AssertionBuilder<usize, CountModifier<M>> {
        AssertionBuilder::modify(self, CountModifier::new)
    }

    #[inline]
    fn nth(self, index: Annotated<usize>) -> AssertionBuilder<T::Item, NthModifier<M>> {
        AssertionBuilder::modify(self, move |prev| NthModifier::new(prev, index))
    }
}
