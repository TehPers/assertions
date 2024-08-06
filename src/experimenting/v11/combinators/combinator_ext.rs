use crate::assertions::Assertion;

use super::{AllCombinator, AssertionCombinator, NotCombinator};

pub trait AssertionCombinatorExt<A>: AssertionCombinator<A> + Sized
where
    A: Assertion<Self::NextTarget>,
{
    #[inline]
    fn not(self) -> NotCombinator<Self> {
        NotCombinator::new(self)
    }

    #[inline]
    fn all(self) -> AllCombinator<Self>
    where
        Self::NextTarget: IntoIterator,
    {
        AllCombinator::new(self)
    }
}

impl<C, A> AssertionCombinatorExt<A> for C
where
    C: AssertionCombinator<A>,
    A: Assertion<C::NextTarget>,
{
}
