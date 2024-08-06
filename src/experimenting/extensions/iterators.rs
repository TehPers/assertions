use std::fmt::{Display, Formatter};

use crate::{
    combinators2::{Assertion, AssertionFn, Combinator, CountCombinator, Plan},
    AssertionRoot,
};

pub trait IteratorAssertions<T, P> {
    fn count(self) -> AssertionRoot<T, Plan<P, CountCombinator>>;
}

impl<T, P> IteratorAssertions<T, P> for AssertionRoot<T, P>
where
    P: Combinator<IdentityAssertion>,
    // P::Assertion: Assertion<I>,
    // <P::Assertion as Assertion2>::Output: IntoIterator,
{
    fn count(self) -> AssertionRoot<T, Plan<P, CountCombinator>> {
        self.chain(CountCombinator)
    }
}

struct IdentityAssertion;

impl Display for IdentityAssertion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "always succeeds")
    }
}

impl<Input> Assertion<Input> for IdentityAssertion {
    type Output = Input;

    fn execute(self, input: Input) -> Self::Output {
        input
    }
}

fn foo() {
    use crate::expect2;

    let success = expect2!([1, 2, 3])
        .count()
        .execute(AssertionFn::new("value is not 0", |value| value != 0));

    let success = expect2!(1)
        .count()
        .execute(AssertionFn::new("value is not 0", |value| value != 0));

    let success = expect2!(1)
        .count()
        .count()
        .execute(AssertionFn::new("value is not 0", |value| value != 0));
}
