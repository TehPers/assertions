use std::fmt::{Display, Formatter};

use super::{Assertion, Combinator};

/// Counts the number of elements in the input.
#[derive(Clone, Copy, Debug, Default)]
pub struct CountCombinator;

impl<Next> Combinator<Next> for CountCombinator {
    type Assertion = CountAssertion<Next>;

    fn build(self, next: Next) -> Self::Assertion {
        CountAssertion::new(next)
    }
}

/// Counts the number of elements in the input, then passes the count to the
/// next assertion.
#[derive(Clone, Copy, Debug, Default)]
pub struct CountAssertion<Next> {
    next: Next,
}

impl<Next> CountAssertion<Next> {
    /// Creates a new [`CountAssertion`].
    #[inline]
    pub fn new(next: Next) -> Self {
        Self { next }
    }
}

impl<Next> Display for CountAssertion<Next>
where
    Next: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "the length satisfies: {}", self.next)
    }
}

impl<Next, Input> Assertion<Input> for CountAssertion<Next>
where
    Input: IntoIterator,
    Next: Assertion<usize>,
{
    type Output = Next::Output;

    fn execute(self, input: Input) -> Self::Output {
        self.next.execute(input.into_iter().count())
    }
}
