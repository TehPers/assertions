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

#[cfg(test)]
mod tests {
    use std::{iter::repeat, sync::mpsc::channel, thread::spawn, time::Duration};

    use test_case::test_case;

    use crate::prelude::*;

    fn with_timeout<F>(t: Duration, f: F) -> bool
    where
        F: FnOnce() + Send + 'static,
    {
        let (done_tx, done_rx) = channel();
        let _run = spawn(move || {
            f();
            let _ = done_tx.send(());
        });

        let output = done_rx.recv_timeout(t);
        output.is_ok()
    }

    #[test_case(false, || expect!(repeat(0), all, to_equal(0)); "all infinite")]
    #[test_case(true, || expect!(repeat(0), not, all, to_equal(1)); "all short-circuit")]
    #[test_case(false, || expect!(repeat(0), any, to_equal(1)); "any infinite")]
    #[test_case(true, || expect!(repeat(0), any, to_equal(0)); "any short-circuit")]
    fn short_circuit(should_pass: bool, f: fn()) {
        let success = with_timeout(Duration::from_secs(1), f);
        expect!(success, to_equal(should_pass));
    }
}

#[cfg(all(test, feature = "futures"))]
mod async_tests {
    use std::{
        future::{ready, Future},
        iter::repeat,
        sync::mpsc::channel,
        time::Duration,
    };

    use test_case::test_case;
    use tokio::spawn;

    use crate::prelude::*;

    fn with_timeout<F>(t: Duration, f: F) -> bool
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let (done_tx, done_rx) = channel();
        let _run = spawn(async move {
            f.await;
            let _ = done_tx.send(());
        });

        let output = done_rx.recv_timeout(t);
        output.is_ok()
    }

    #[test_case(
        false,
        // Need to wrap these expectations because even constructing them is
        // an infinite loop due to the iterator being collected into a
        // FuturesUnordered
        async {
            expect!(repeat(ready(0)), all, when_ready, to_equal(0)).await
        };
        "all infinite"
    )]
    #[test_case(
        true,
        async {
            expect!(repeat(ready(0)), not, all, when_ready, to_equal(1)).await
        } => ignore["not implemented yet"];
        "all short-circuit"
    )]
    #[test_case(
        false,
        async {
            expect!(repeat(ready(0)), any, when_ready, to_equal(1)).await
        };
        "any infinite"
    )]
    #[test_case(
        true,
        async {
            expect!(repeat(ready(0)), any, when_ready, to_equal(0)).await
        } => ignore["not implemented yet"];
        "any short-circuit"
    )]
    #[tokio::test]
    async fn short_circuit<F>(should_pass: bool, f: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let success = with_timeout(Duration::from_secs(1), f);
        expect!(success, to_equal(should_pass));
    }

    /// Ensures that assertions that use non-Clone opaque features can still be
    /// executed with the merging modifiers. This means the assertion executed
    /// after the merging modifier must be Clone even if the subject passed into
    /// the assertion is not.
    #[tokio::test]
    async fn opaque_futures() {
        async fn get_cat_url(id: u32) -> String {
            format!("cats/{id}.png")
        }

        expect!(
            [get_cat_url(1), get_cat_url(2)],
            all,
            when_ready,
            to_contain_substr(".png")
        )
        .await;
    }
}
