use std::fmt::Display;

use crate::Assertable;

/// Wraps an [`Assertable`] and applies the assertion to the inner function's
/// return value when called with the given arguments.
pub struct WhenCalledCombinator<Inner, Args> {
    inner: Inner,
    args: Args,
}

impl<Inner, Args> WhenCalledCombinator<Inner, Args> {
    /// Creates a new combinator which wraps an inner [`Assertable`].
    #[inline]
    pub fn new(inner: Inner, args: Args) -> Self {
        Self { inner, args }
    }
}

macro_rules! impl_when_called_combinator {
    ($($arg:ident),*) => {
        impl<Inner, R, $($arg),*> Assertable for WhenCalledCombinator<Inner, ($($arg,)*)>
        where
            Inner: Assertable,
            Inner::Target: FnOnce($($arg),*) -> R,
            ($($arg,)*): Clone,
        {
            type Target = R;
            type Result = Inner::Result;

            fn to_satisfy<F>(self, expectation: impl Display, mut f: F) -> Self::Result
            where
                F: FnMut(Self::Target) -> bool,
            {
                self.inner
                    .to_satisfy(format_args!("when called, {expectation}"), |value| {
                        #[allow(non_snake_case)]
                        let ($($arg,)*) = self.args.clone();
                        let result = value($($arg),*);
                        f(result)
                    })
            }
        }
    };
}

impl_when_called_combinator!();
impl_when_called_combinator!(A1);
impl_when_called_combinator!(A1, A2);
impl_when_called_combinator!(A1, A2, A3);
impl_when_called_combinator!(A1, A2, A3, A4);
impl_when_called_combinator!(A1, A2, A3, A4, A5);
impl_when_called_combinator!(A1, A2, A3, A4, A5, A6);
impl_when_called_combinator!(A1, A2, A3, A4, A5, A6, A7);
impl_when_called_combinator!(A1, A2, A3, A4, A5, A6, A7, A8);
impl_when_called_combinator!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_when_called_combinator!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);
impl_when_called_combinator!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11);
impl_when_called_combinator!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12);
