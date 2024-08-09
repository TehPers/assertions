use crate::assertions::{
    iterators::{MergeStrategy, MergeableOutput},
    key, Assertion, AssertionContext, AssertionModifier, SubjectKey,
};

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
/// Requires that the rest of the assertion is [`Clone`]. For example, comparing
/// each item to a non-cloneable value will not compile:
///
/// ```compile_fail
/// # use expecters::prelude::*;
/// struct NotClone(i32);
/// expect!([NotClone(0)], all, map(|NotClone(x)| x), to_equal(0));
/// ```
#[inline]
pub fn all<T, M>(prev: M, _: SubjectKey<T>) -> (MergeModifier<M>, SubjectKey<T::Item>)
where
    T: IntoIterator,
{
    (
        MergeModifier {
            prev,
            strategy: MergeStrategy::All,
        },
        key(),
    )
}

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
#[inline]
pub fn any<T, M>(prev: M, _: SubjectKey<T>) -> (MergeModifier<M>, SubjectKey<T::Item>)
where
    T: IntoIterator,
{
    (
        MergeModifier {
            prev,
            strategy: MergeStrategy::Any,
        },
        key(),
    )
}

/// Modifier for [`all()`] and [`any()`].
#[derive(Clone, Debug)]
pub struct MergeModifier<M> {
    prev: M,
    strategy: MergeStrategy,
}

impl<M, A> AssertionModifier<A> for MergeModifier<M>
where
    M: AssertionModifier<MergeAssertion<A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, next: A) -> Self::Output {
        self.prev.apply(MergeAssertion {
            next,
            strategy: self.strategy,
        })
    }
}

/// Assertion for [`all()`] and [`any()`].
#[derive(Clone, Debug)]
pub struct MergeAssertion<A> {
    next: A,
    strategy: MergeStrategy,
}

impl<A, T> Assertion<T> for MergeAssertion<A>
where
    A: Assertion<T::Item, Output: MergeableOutput> + Clone,
    T: IntoIterator,
{
    type Output = <A::Output as MergeableOutput>::Merged;

    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        let outputs = subject.into_iter().enumerate().map({
            // Clone the context so it can be moved into the closure (we need it
            // again later to merge the outputs)
            let cx = cx.clone();

            move |(idx, item)| {
                // Create a new context for this execution path
                let mut cx = cx.clone();
                cx.annotate("index", idx);

                // Call the next assertion
                self.next.clone().execute(cx, item)
            }
        });

        // Merge the outputs
        MergeableOutput::merge(cx, self.strategy, outputs)
    }
}
