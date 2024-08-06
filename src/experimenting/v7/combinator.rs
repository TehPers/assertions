use super::{AllCombinator, Assertion, NotCombinator};

/// A type which builds an assertion to execute on a value.
pub trait AssertionCombinator: Sized {
    /// The input passed to the *next assertion* in the chain.
    ///
    /// This combinator's associated assertion may receive a different type. For
    /// example, this combinator may accept some (known) iterable value, like an
    /// `[i32; 4]`, and pass an [`i32`] to the next assertion in the chain. This
    /// type would then be [`i32`].
    type NextInput;

    /// The associated assertion. This is what gets built when this combinator
    /// is applied to another assertion.
    // This type cannot be a GAT since each combinator may have different type
    // bounds on the assertion. For example, `NotAssertion<Next>` may require
    // that `Next` returns a `Result<(), AssertionFailure>` to implement the
    // `Assertion` trait.
    //
    // In those cases, the bound would either need to be specified in the `impl`
    // block for the combinator, or on the `impl` block for the assertion. If
    // this associated type is constrained such that it must be an assertion,
    // then the combinator's `impl` block must enforce the constraint as well.
    type Assertion<Next>
    where
        Next: Assertion<Self::NextInput>;

    /// Builds an assertion using this combinator.
    fn build<Next>(self, assertion: Next) -> Self::Assertion<Next>
    where
        Next: Assertion<Self::NextInput>;

    // Since we don't know yet what assertion will be passed into the
    // `NotCombinator` type, we can't specify the `Next` parameter to constrain
    // this method such that it can only be called when that particular impl's
    // bounds are satisfied. We also don't yet know what the return value will
    // be for the final assertion because it hasn't been fully built yet.
    //
    // There needs to be a way to constrain the assertions chained onto this so
    // that they satisfy the bounds on the `NotCombinator`'s assertion.
    fn not(self) -> NotCombinator<Self> {
        NotCombinator::new(self)
    }

    fn all(self) -> AllCombinator<Self>
    where
        Self::NextInput: IntoIterator,
    {
        AllCombinator::new(self)
    }
}
