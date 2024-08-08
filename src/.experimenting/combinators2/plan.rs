use super::Combinator;

/// Chains two combinators together.
///
/// This is used to control the order that an input is passed through a chain of
/// combinators. The `Outer` combinator is executed first, then the output is
/// passed to the `Inner` combinator. The output of the `Inner` combinator is
/// returned by the plan.
pub struct Plan<Outer, Inner> {
    outer: Outer,
    inner: Inner,
}

impl<Outer, Inner> Plan<Outer, Inner> {
    /// Creates a new [`Plan`] combinator.
    pub fn new(prev: Outer, next: Inner) -> Self {
        Self {
            outer: prev,
            inner: next,
        }
    }
}

impl<Outer, Next1, Next2> Combinator<Next2> for Plan<Outer, Next1>
where
    Outer: Combinator<Next1::Assertion>,
    Next1: Combinator<Next2>,
{
    type Assertion = Outer::Assertion;

    fn build(self, next: Next2) -> Self::Assertion {
        self.outer.build(self.inner.build(next))
    }
}
