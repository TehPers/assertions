use crate::assertions::{
    iterators::{MergeStrategy, MergeableOutput},
    Assertion, AssertionContext, AssertionContextBuilder, AssertionModifier,
};

/// Forks an assertion, executing it for each element of the subject.
#[derive(Clone, Debug)]
pub struct MergeModifier<M> {
    prev: M,
    strategy: MergeStrategy,
}

impl<M> MergeModifier<M> {
    #[inline]
    pub(crate) fn new(prev: M, strategy: MergeStrategy) -> Self {
        Self { prev, strategy }
    }
}

impl<M, A> AssertionModifier<A> for MergeModifier<M>
where
    M: AssertionModifier<MergeAssertion<A>>,
{
    type Output = M::Output;

    #[inline]
    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
        self.prev.apply(
            cx,
            MergeAssertion {
                next,
                strategy: self.strategy,
            },
        )
    }
}

/// Forks the inner assertion, executing it for each element of the subject.
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
            expect!(repeat(ready(0)), all, when_ready, to_equal(0)).await;
        };
        "all infinite"
    )]
    #[test_case(
        true,
        async {
            expect!(repeat(ready(0)), not, all, when_ready, to_equal(1)).await;
        } => ignore["not implemented yet"];
        "all short-circuit"
    )]
    #[test_case(
        false,
        async {
            expect!(repeat(ready(0)), any, when_ready, to_equal(1)).await;
        };
        "any infinite"
    )]
    #[test_case(
        true,
        async {
            expect!(repeat(ready(0)), any, when_ready, to_equal(0)).await;
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
        #[allow(clippy::unused_async)]
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
