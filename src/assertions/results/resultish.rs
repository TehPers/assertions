mod sealed {
    pub trait Sealed {
        type Inner;
        type Error;

        fn ok(self) -> Option<Self::Inner>;
        fn err(self) -> Option<Self::Error>;
    }

    impl<T, E> Sealed for Result<T, E> {
        type Inner = T;
        type Error = E;

        #[inline]
        fn ok(self) -> Option<Self::Inner> {
            self.ok()
        }

        #[inline]
        fn err(self) -> Option<Self::Error> {
            self.err()
        }
    }

    impl<'a, T, E> Sealed for &'a Result<T, E> {
        type Inner = &'a T;
        type Error = &'a E;

        #[inline]
        fn ok(self) -> Option<Self::Inner> {
            self.as_ref().ok()
        }

        #[inline]
        fn err(self) -> Option<Self::Error> {
            self.as_ref().err()
        }
    }

    impl<'a, T, E> Sealed for &'a mut Result<T, E> {
        type Inner = &'a mut T;
        type Error = &'a mut E;

        #[inline]
        fn ok(self) -> Option<Self::Inner> {
            self.as_mut().ok()
        }

        #[inline]
        fn err(self) -> Option<Self::Error> {
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
