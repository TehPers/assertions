mod sealed {
    pub trait Sealed {
        type Inner;

        fn some(self) -> Option<Self::Inner>;
    }

    impl<T> Sealed for Option<T> {
        type Inner = T;

        #[inline]
        fn some(self) -> Option<Self::Inner> {
            self
        }
    }

    impl<'a, T> Sealed for &'a Option<T> {
        type Inner = &'a T;

        #[inline]
        fn some(self) -> Option<Self::Inner> {
            self.as_ref()
        }
    }

    impl<'a, T> Sealed for &'a mut Option<T> {
        type Inner = &'a mut T;

        #[inline]
        fn some(self) -> Option<Self::Inner> {
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
