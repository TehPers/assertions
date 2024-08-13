mod sealed {
    pub trait Sealed {
        type T;
        type OutT;

        fn some(self) -> Option<Self::OutT>;
    }

    impl<T> Sealed for Option<T> {
        type T = T;
        type OutT = T;

        #[inline]
        fn some(self) -> Option<Self::OutT> {
            self
        }
    }

    impl<'a, T> Sealed for &'a Option<T> {
        type T = T;
        type OutT = &'a T;

        #[inline]
        fn some(self) -> Option<Self::OutT> {
            self.as_ref()
        }
    }

    impl<'a, T> Sealed for &'a mut Option<T> {
        type T = T;
        type OutT = &'a mut T;

        #[inline]
        fn some(self) -> Option<Self::OutT> {
            self.as_mut()
        }
    }
}

/// Helper trait for mapping [`Option<T>`] and its references to its inner value
/// and type.
///
/// This is implemented for:
/// - `Option<T>`
/// - `&Option<T>`
/// - `&mut Option<T>`
pub trait Optionish: sealed::Sealed {}

impl<R> Optionish for R where R: sealed::Sealed {}
