use std::fmt::Debug;

/// An assertion result that can be finalized.
///
/// Finalizing the result "unwraps" the result. For actual [`Result<T, E>`]
/// results, this is literally the act of [unwrapping](Result::unwrap()) the
/// result, but other result types may choose to finalize in a different manner
/// (like unwrapping the result once it's available in the case of asynchronous
/// results).
///
/// The purpose of finalizing the result is to panic as soon as possible if an
/// assertion fails. Not all results will be finalized, but if they are
/// finalized, they should provide output to the user as soon as possible if the
/// assertion failed.
pub trait FinalizableResult {
    type Finalized;

    /// Finalizes this result.
    fn finalize(self) -> Self::Finalized;
}

impl<T, E> FinalizableResult for Result<T, E>
where
    E: Debug,
{
    type Finalized = T;

    fn finalize(self) -> Self::Finalized {
        match self {
            Ok(t) => t,
            Err(e) => panic!("{e:?}"),
        }
    }
}
