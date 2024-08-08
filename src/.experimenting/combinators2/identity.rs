use super::Combinator;

/// A combinator that does nothing.
///
/// This combinator is used to terminate a chain of combinators. It outputs the
/// assertion that was passed to it.
#[derive(Clone, Copy, Debug, Default)]
pub struct IdentityCombinator;

impl<Next> Combinator<Next> for IdentityCombinator {
    type Assertion = Next;

    fn build(self, next: Next) -> Self::Assertion {
        next
    }
}
