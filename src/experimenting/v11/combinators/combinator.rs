use crate::assertions::Assertion;

pub trait AssertionCombinator<A>
where
    A: Assertion<Self::NextTarget>,
{
    type NextTarget;
    type Output;

    fn apply(self, assertion: A) -> Self::Output;
}
