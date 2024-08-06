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

impl<Prev, Next> AssertionCombinator<Next> for AllCombinator<Prev>
where
    Prev: AssertionCombinator<AllAssertion<Next>>,
    Prev::NextInput: IntoIterator,
    AllAssertion<Next>: Assertion<Prev::NextInput>,
{
    type NextInput = <Prev::NextInput as IntoIterator>::Item;
    type Assertion = Prev::Assertion;

    fn build(self, assertion: Next) -> Self::Assertion {
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
    Next: Assertion<Input::Item> + Clone,
    Result<(), AssertionError>: FromIterator<Next::Output>,
{
    type Output = Result<(), AssertionError>;

    fn assert(self, target: Input) -> Self::Output {
        target
            .into_iter()
            .map(|item| self.next.clone().assert(item))
            .collect()
    }
}
