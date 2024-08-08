use either::Either;

use crate::{AssertionFailure, AssertionOutput};

macro_rules! map_either {
    ($value:expr, $pattern:pat => $result:expr) => {
        match $value {
            Either::Left($pattern) => Either::Left($result),
            Either::Right($pattern) => Either::Right($result),
        }
    };
}

impl<L, R> AssertionOutput for Either<L, R>
where
    L: AssertionOutput,
    R: AssertionOutput,
{
    type Mapped<F> = Either<L::Mapped<F>, R::Mapped<F>>
    where
        F: FnMut(crate::AssertionResult) -> crate::AssertionResult;

    #[inline]
    fn new_success() -> Self {
        Either::Left(L::new_success())
    }

    #[inline]
    fn new_failure(failure: AssertionFailure) -> Self {
        Either::Left(L::new_failure(failure))
    }

    #[inline]
    fn map<F>(self, f: F) -> Self::Mapped<F>
    where
        F: FnMut(Result<(), AssertionFailure>) -> Result<(), AssertionFailure>,
    {
        map_either!(self, inner => inner.map(f))
    }

    #[inline]
    fn and_then<O, F>(self, other: F) -> impl AssertionOutput
    where
        O: AssertionOutput,
        F: FnOnce() -> O,
    {
        map_either!(self, inner => inner.and_then(other))
    }

    #[inline]
    fn or_else<O, F>(self, other: F) -> impl AssertionOutput
    where
        O: AssertionOutput,
        F: FnOnce() -> O,
    {
        map_either!(self, inner => inner.or_else(other))
    }

    fn all(outputs: impl IntoIterator<Item = Self>) -> impl AssertionOutput {
        let mut lefts = Vec::new();
        let mut rights = Vec::new();
        for output in outputs {
            match output {
                Either::Left(inner) => lefts.push(inner),
                Either::Right(inner) => rights.push(inner),
            }
        }

        L::all(lefts).and_then(move || R::all(rights))
    }

    fn any(outputs: impl IntoIterator<Item = Self>) -> impl AssertionOutput {
        let mut lefts = Vec::new();
        let mut rights = Vec::new();
        for output in outputs {
            match output {
                Either::Left(inner) => lefts.push(inner),
                Either::Right(inner) => rights.push(inner),
            }
        }

        L::any(lefts).or_else(move || R::any(rights))
    }
}
