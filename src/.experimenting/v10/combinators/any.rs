use std::fmt::{Display, Formatter};

use crate::{AssertionFailure, AssertionResult};

use super::AssertionCombinator;

/// Wraps another [`AssertionCombinator`] and ensures the assertion succeeds for
/// some inner value.
#[derive(Clone, Debug)]
pub struct AnyCombinator<Inner> {
    inner: Inner,
}

impl<Inner> AnyCombinator<Inner> {
    /// Creates a new instance of this combinator, wrapping the inner
    /// combinator.
    #[inline]
    pub fn new(inner: Inner) -> Self {
        Self { inner }
    }
}

impl<Inner> AssertionCombinator for AnyCombinator<Inner>
where
    Inner: AssertionCombinator,
    Inner::Target: IntoIterator,
{
    type Target = <Inner::Target as IntoIterator>::Item;
    type Result = Inner::Result;

    fn execute<F>(self, mut f: F) -> Self::Result
    where
        F: FnMut(Self::Target) -> AssertionResult,
    {
        self.inner.execute(|values| {
            let mut error = AssertionFailure::builder().build("TODO");
            for value in values {
                match f(value) {
                    Ok(()) => return Ok(()),
                    Err(e) => error = e,
                }
            }

            Err(error)
        })
    }
}

// impl<Next, Target> Assertion<Target> for AnyAssertion<Next>
// where
//     Target: IntoIterator,
//     Next: Assertion<Target::Item, Output = AssertionResult> + Clone,
// {
//     type Output = AssertionResult;

//     fn assert(self, target: Target) -> Self::Output {
//         let mut error = AssertionFailure::builder().build("TODO");
//         for value in target {
//             match self.next.clone().assert(value) {
//                 Ok(()) => return Ok(()),
//                 Err(e) => error = e,
//             }
//         }

//         Err(error)
//     }
// }
