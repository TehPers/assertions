mod sealed {
    pub trait Sealed {
        type T;
        type E;

        type OutT;
        type OutE;

        fn ok(self) -> Option<Self::OutT>;
        fn err(self) -> Option<Self::OutE>;
    }

    impl<T, E> Sealed for Result<T, E> {
        type T = T;
        type E = E;

        type OutT = T;
        type OutE = E;

        #[inline]
        fn ok(self) -> Option<Self::OutT> {
            self.ok()
        }

        #[inline]
        fn err(self) -> Option<Self::OutE> {
            self.err()
        }
    }

    impl<'a, T, E> Sealed for &'a Result<T, E> {
        type T = T;
        type E = E;

        type OutT = &'a T;
        type OutE = &'a E;

        #[inline]
        fn ok(self) -> Option<Self::OutT> {
            self.as_ref().ok()
        }

        #[inline]
        fn err(self) -> Option<Self::OutE> {
            self.as_ref().err()
        }
    }

    impl<'a, T, E> Sealed for &'a mut Result<T, E> {
        type T = T;
        type E = E;

        type OutT = &'a mut T;
        type OutE = &'a mut E;

        #[inline]
        fn ok(self) -> Option<Self::OutT> {
            self.as_mut().ok()
        }

        #[inline]
        fn err(self) -> Option<Self::OutE> {
            self.as_mut().err()
        }
    }
}

/// Helper trait for mapping [`Result<T, E>`] and its references to its
/// component values and types.
///
/// This is implemented for:
/// - `Result<T, E>`
/// - `&Result<T, E>`
/// - `&mut Result<T, E>`
pub trait Resultish: sealed::Sealed {}

impl<R> Resultish for R where R: sealed::Sealed {}
