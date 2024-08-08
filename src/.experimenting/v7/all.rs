use std::fmt::{self, Display, Formatter};

use super::{Assertion, AssertionCombinator, AssertionError};

pub struct AllCombinator<Prev> {
    prev: Prev,
}

impl<Prev> AllCombinator<Prev> {
    /// Creates a new [`AllCombinator`].
    #[inline]
    pub fn new(prev: Prev) -> Self {
        Self { prev }
    }
}

impl<Prev> AssertionCombinator for AllCombinator<Prev>
where
    Prev: AssertionCombinator,
    Prev::NextInput: IntoIterator,
    // Result<(), AssertionError>: FromIterator<Next::Output>,
    Result<(), AssertionError>: FromIterator<<Next as Assertion<<<Prev as AssertionCombinator>::NextInput as IntoIterator>::Item>>::Output>
{
    type NextInput = <Prev::NextInput as IntoIterator>::Item;
    type Assertion<Next> = Prev::Assertion<AllAssertion<Next>>
    where
        Next: Assertion<Self::NextInput>;

    fn build<Next>(self, assertion: Next) -> Self::Assertion<Next>
    where
        Next: Assertion<Self::NextInput>,
    {
        self.prev.build(AllAssertion::new(assertion))
    }
}

#[derive(Clone, Debug)]
pub struct AllAssertion<Next> {
    next: Next,
}

impl<Next> AllAssertion<Next> {
    /// Creates a new [`AllAssertion`].
    #[inline]
    pub fn new(next: Next) -> Self {
        Self { next }
    }
}

impl<Next> Display for AllAssertion<Next>
where
    Next: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "for each inner value, {}", self.next)
    }
}

impl<Next, Input> Assertion<Input> for AllAssertion<Next>
where
    Input: IntoIterator,
    Next: Assertion<Input::Item>,
    Result<(), AssertionError>: FromIterator<Next::Output>,
{
    type Output = Result<(), AssertionError>;

    fn assert(&mut self, target: Input) -> Self::Output {
        target
            .into_iter()
            .map(|item| self.next.assert(item))
            .collect()
    }
}
