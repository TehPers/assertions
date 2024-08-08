use std::fmt::{self, Display, Formatter};

use super::{Assertion, AssertionCombinator, AssertionError, AssertionOutput};

/// Inverts the result of an assertion.
#[derive(Clone, Debug)]
pub struct NotCombinator<Prev> {
    prev: Prev,
}

impl<Prev> NotCombinator<Prev> {
    /// Creates a new [`NotCombinator`].
    #[inline]
    pub fn new(prev: Prev) -> Self {
        Self { prev }
    }
}

impl<Prev> AssertionCombinator for NotCombinator<Prev>
where
    Prev: AssertionCombinator,
{
    type NextInput = Prev::NextInput;
    type Assertion<Next> = Prev::Assertion<NotAssertion<Next>>
    where
        Next: Assertion<Self::NextInput>;

    fn build<Next>(self, assertion: Next) -> Self::Assertion<Next>
    where
        Next: Assertion<Self::NextInput>,
    {
        self.prev.build(NotAssertion::new(assertion))
    }
}

/// Inverts the result of the next assertion.
#[derive(Clone, Debug)]
pub struct NotAssertion<Next> {
    next: Next,
}

impl<Next> NotAssertion<Next> {
    /// Creates a new [`NotAssertion`].
    #[inline]
    pub fn new(next: Next) -> Self {
        NotAssertion { next }
    }
}

impl<Next> Display for NotAssertion<Next>
where
    Next: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "the following to not be satisfied: {}", self.next)
    }
}

impl<Next, Input> Assertion<Input> for NotAssertion<Next>
where
    Next: Assertion<Input>,
{
    type Output = <Next::Output as AssertionOutput>::Mapped<
        fn(Result<(), AssertionError>) -> Result<(), AssertionError>,
    >;

    fn assert(&mut self, target: Input) -> Self::Output {
        self.next.assert(target).map(|result| match result {
            Ok(_) => Err(AssertionError::default()),
            Err(_) => Ok(()),
        })
    }
}
