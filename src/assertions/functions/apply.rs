use std::panic::UnwindSafe;

mod private {
    pub trait Sealed<I> {}
}

/// A function which can have arguments applied to it.
pub trait ApplyFnOnce<I>: private::Sealed<I> {
    /// The output from this function.
    type Output;

    /// Whether the args list is empty.
    const EMPTY_ARGS: bool;

    /// Apply arguments to the function, turning it into a `FnOnce() -> O`.
    fn apply_once(self, args: I) -> impl FnOnce() -> Self::Output;
}

/// A function which can have arguments applied to it to produce an unwind-safe
/// function.
///
/// This is a hack until TAIT is stabilized.
pub(crate) trait ApplyFnOnceUnwindSafe<I>: ApplyFnOnce<I> {
    /// Apply arguments to the function, turning it into a `FnOnce() -> O`.
    fn apply_once_unwind(self, args: I) -> impl FnOnce() -> Self::Output + UnwindSafe;
}

macro_rules! impl_apply {
    (@shift $T0:ident,) => {};
    (@shift $T0:ident, $($Tn:ident,)*) => {
        impl_apply!($($Tn),*);
    };
    ($($Tn:ident),*) => {
        impl<F, O $(, $Tn)*> private::Sealed<($($Tn,)*)> for F
        where
            F: FnOnce($($Tn),*) -> O,
        {}

        impl<F, O $(, $Tn)*> ApplyFnOnce<($($Tn,)*)> for F
        where
            F: FnOnce($($Tn),*) -> O,
        {
            type Output = O;

            const EMPTY_ARGS: bool = false;

            #[allow(non_snake_case)]
            fn apply_once(self, ($($Tn,)*): ($($Tn,)*)) -> impl FnOnce() -> Self::Output {
                move || (self)($($Tn,)*)
            }
        }

        impl<F, O $(, $Tn)*> ApplyFnOnceUnwindSafe<($($Tn,)*)> for F
        where
            F: FnOnce($($Tn),*) -> O + UnwindSafe,
            $($Tn: UnwindSafe,)*
        {
            #[allow(non_snake_case)]
            fn apply_once_unwind(self, ($($Tn,)*): ($($Tn,)*)) -> impl FnOnce() -> Self::Output + UnwindSafe {
                move || (self)($($Tn,)*)
            }
        }

        impl_apply!(@shift $($Tn,)*);
    };
}

impl_apply!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);

impl<F, O> private::Sealed<()> for F where F: FnOnce() -> O {}

impl<F, O> ApplyFnOnce<()> for F
where
    F: FnOnce() -> O,
{
    type Output = O;

    const EMPTY_ARGS: bool = true;

    fn apply_once(self, (): ()) -> impl FnOnce() -> Self::Output {
        self
    }
}

impl<F, O> ApplyFnOnceUnwindSafe<()> for F
where
    F: FnOnce() -> O + UnwindSafe,
{
    fn apply_once_unwind(self, (): ()) -> impl FnOnce() -> Self::Output + UnwindSafe {
        self
    }
}
