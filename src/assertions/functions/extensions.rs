use std::panic::UnwindSafe;

use crate::{
    annotated,
    assertions::{
        functions::{ApplyFnOnce, ApplyFnOnceUnwindSafe, ToPanic, WhenCalledModifier},
        AssertionBuilder,
    },
    metadata::Annotated,
};

/// Assertions and modifiers for functions that accept no arguments.
pub trait SimpleFunctionAssertions<F, O, M>
where
    F: FnOnce() -> O,
{
    /// Executes an assertion on the value returned by this function.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(|| 1, when_called, to_equal(1));
    /// ```
    fn when_called(self) -> AssertionBuilder<O, WhenCalledModifier<M>>;

    /// Asserts that the subject panics when called.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(|| panic!(), to_panic);
    /// ```
    ///
    /// This assertion fails if the subject does not panic.
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(|| {}, to_panic);
    /// ```
    #[inline]
    fn to_panic(&self) -> ToPanic
    where
        F: UnwindSafe,
    {
        ToPanic::new(annotated!(()))
    }
}

impl<F, O, M> SimpleFunctionAssertions<F, O, M> for AssertionBuilder<F, M>
where
    F: FnOnce() -> O,
{
    #[inline]
    fn when_called(self) -> AssertionBuilder<O, WhenCalledModifier<M>> {
        AssertionBuilder::modify(self, |prev| WhenCalledModifier::new(prev, annotated!(())))
    }
}

/// Assertions and modifiers for functions of arity up to 12.
pub trait FunctionAssertions<F, I, M>
where
    F: ApplyFnOnce<I>,
{
    /// Executes an assertion on the value returned by this function when called
    /// with certain arguments.
    ///
    /// Arguments for a N-arity function must be passed as a N-tuple:
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// use std::{cmp::max, str::from_utf8};
    /// expect!(max, when_called_with((1, 2)), to_equal(2));
    /// expect!(from_utf8, when_called_with(([].as_slice(),)), to_be_ok);
    /// ```
    ///
    /// Note that functions that take 1 argument are passed it via `(arg,)` (a
    /// 1-tuple).
    fn when_called_with(
        self,
        args: Annotated<I>,
    ) -> AssertionBuilder<F::Output, WhenCalledModifier<M, I>>;

    /// Asserts that the subject panics when called with certain arguments.
    ///
    /// Arguments for a N-arity function must be passed as a N-tuple:
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(|a: i32, b: i32| panic!("{a}{b}"), to_panic_with((1, 2)));
    /// expect!(|msg: &str| panic!("{msg}"), to_panic_with(("foo",)));
    /// ```
    ///
    /// Note that functions that take 1 argument are passed it via `(arg,)` (a
    /// 1-tuple).
    ///
    /// This assertion fails if the subject does not panic.
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!(|n: i32| n, to_panic_with((1,)));
    /// ```
    #[inline]
    #[allow(private_bounds)]
    fn to_panic_with(&self, args: Annotated<I>) -> ToPanic<I>
    where
        F: ApplyFnOnceUnwindSafe<I>,
    {
        ToPanic::new(args)
    }
}

impl<F, I, M> FunctionAssertions<F, I, M> for AssertionBuilder<F, M>
where
    F: ApplyFnOnce<I>,
{
    fn when_called_with(
        self,
        args: Annotated<I>,
    ) -> AssertionBuilder<F::Output, WhenCalledModifier<M, I>> {
        AssertionBuilder::modify(self, move |prev| WhenCalledModifier::new(prev, args))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn identity() {
        expect!(|x| x, when_called_with((1,)), to_equal(1));
        expect!(|x| x, when_called_with(("foo",)), to_equal("foo"));
    }
}
