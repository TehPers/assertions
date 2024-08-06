/// Performs an assertion.
///
/// This macro is used to perform an assertion on a subject value. It's intended
/// to be used to build assertions piece-by-piece to perform more complex
/// assertions on a subject value.
///
/// Note that the "subject" of an assertion is the value the assertion is being
/// executed on. For example, if an assertion is checking whether a value is
/// greater than zero, then the subject of the assertion is the value that is
/// being checked.
///
/// ```
/// # use expecters::prelude::*;
/// let subject = 1;
/// expect!(subject, to_be_greater_than(0));
/// ```
///
/// ## Syntax
///
/// This macro is called similar to how a function is called. For example:
///
/// ```
/// # use expecters::prelude::*;
/// expect!(1, not, to_equal(0));
/// ```
///
/// Breaking this down, the macro accepts arguments in the format
/// `expect!(subject, modifiers..., assertion)`. The subject may be any value
/// that you want to execute an assertion on (and is moved/copied into the
/// assertion - make sure to borrow the value if needed). The final argument
/// must be a fully built assertion.
///
/// Both the modifiers and the final assertion must be either identifiers or
/// simple function calls in the format `<ident>(params...)`. This is because
/// the parameters to function calls will be annotated. This means that **the
/// following syntax is invalid**, as paths are not supported:
///
/// ```compile_fail
/// # use expecters::prelude::*;
/// expect!(1, not, expecters::prelude::to_equal(0));
/// ```
///
/// To fix this, either import [`to_equal`] directly, or alias it:
///
/// ```
/// # #[allow(unused_import)]
/// # use expecters::prelude::*;
/// use expecters::prelude::to_equal as expect_to_equal;
/// expect!(1, not, expect_to_equal(0));
/// ```
///
/// Note that aliasing a modifier or assertion will change its name in the error
/// message it generates as well. Error messages produced by the above assertion
/// will refer to the final parameter as `expect_to_equal` instead of `to_equal`
/// because of the alias.
///
/// Modifiers are special assertion builders that are used to modify a later
/// assertion either by transforming the input to that assertion (like [`map`]),
/// transforming the output from the assertion (like [`not`]), or even calling
/// the assertion multiple times (like [`all`]). In practice, a modifier may be
/// used to modify an assertion in any way it wants, and should generate a new
/// assertion from it.
///
/// Each modifier passed into this macro will be called with the assertion to
/// modify. For example, in the above code snippet, the [`not`] modifier is a
/// function that the macro calls, passing in the later assertion. It is
/// functionally being transformed to `not(to_equal(0))` (although it is not
/// receiving this exact input - more on this below). When chaining multiple
/// modifiers, they are functionally composed together. For example:
///
/// ```
/// # use expecters::prelude::*;
/// expect!([1, 2, 3], not, all, to_equal(2));
/// ```
///
/// In this assertion, the [`all`] modifier is functionally being called as
/// `all(to_equal(2))`, and the [`not`] modifier is functionally being called
/// with the assertion returned by *that* function call (since [`all`] returns
/// an assertion). In other words, the final assertion is essentially the result
/// of calling `not(all(to_equal(2)))`.
///
/// In practice, modifiers are slightly more complicated to this. Modifiers and
/// assertions are called lazily on-demand, and each of the intermediate
/// assertions and the final assertion are wrapped to record additional data
/// about the values being passed between assertions.
///
/// ## Async assertions
///
/// > *Note: requires crate feature `futures`.*
///
/// Async assertions function similar to sync assertions.
///
/// TODO - maybe doc this in the `futures` module so it's cfg-locked and can use
/// doc tests
///
/// ## Annotations
///
/// Values passed as parameters to a modifier or the final assertion are
/// annotated. Values passed into assertions (and from modifiers to other
/// assertions) are *transparently* annotated.
///
/// An annotated value is a value with an additional string representation
/// attached to it. This string representation is generated either from the
/// value's [`Debug`] representation or from the [stringified] source code
/// itself (if no [`Debug`] implementation is available).
///
/// Above, it was noted that applying, for example, the [`not`] modifier to an
/// assertion `a` was *functionally* equivalent to calling `not(a)`. In
/// implementation, [`not`] does not actually receive the assertion `a`, but
/// instead receives a special annotated assertion which wraps `a`.
///
/// This annotated assertion is a hidden modifier that annotates the value that
/// it receives. This means that when calling `expect!(1, not, to_equal(2))`,
/// the value being sent from [`not`] to [`to_equal`] is automatically annotated
/// by this macro. Additionally, the `2` parameter to [`to_equal`] is
/// automatically annotated by this macro, so the [`to_equal`] function is
/// actually not receiving an [`i32`], but an annotated version of it.
///
/// In other words, if the hidden modifier's name is `annotate` and there
/// existed a constructor `Annotated(T)` to construct an annotated value, then
/// the assertion being called could be simplistically represented as
/// `annotate(not(annotate(to_equal(Annotated(2)))))`. Note that the parameter
/// to [`to_equal`] is also annotated, as would any parameters to any modifiers
/// in the chain (if there existed any which accepted parameters).
///
/// This macro must perform the annotation itself to avoid adding additional
/// bounds to assertions. This is because this macro performs autoref
/// specialization to extract the string representation of the value. Without
/// this, the [`to_equal`] assertion would need to have an additional [`Debug`]
/// constraint on the values that it receives to be able to display those values
/// in case of an assertion failure, meaning that assertion would not be as
/// useful for values that do not have a [`Debug`] representation.
///
/// One limitation of this approach is that values being passed from modifiers
/// to other assertions down the chain do not have a meaningful source
/// representation. If those values do not have a [`Debug`] implementation, then
/// the string representation of those values will not be meaningful. However,
/// assertions can see whether a meaningful string representation is available
/// before generating error messages, and this approach removes the burden on
/// assertions (and users) to constrain their inputs to values that can be
/// meaningfully represented as a string.
///
/// Note that there will not always be a meaningful string representation of a
/// value. For values defined directly in source code (like `2` in the example
/// above), a source representation of the value can be used to provide some
/// context on where the value came from. However, for intermediate values (like
/// the value sent from [`not`] to [`to_equal`]), there may not be a meaningful
/// source representation of the value, as the annotated value would simply
/// represent an internal variable of the macro. A best-effort attempt will be
/// made to preserve as much useful information as possible to provide
/// informative error messages.
///
/// [`Annotated<T>`]: crate::metadata::Annotated
/// [`AnnotatedAssertion<A, T>`]: crate::assertions::AnnotatedAssertion
/// [`Debug`]: core::fmt::Debug
/// [`all`]: crate::prelude::all
/// [`map`]: crate::prelude::map
/// [`not`]: crate::prelude::not
/// [`to_equal`]: crate::prelude::to_equal
/// [stringified]: core::stringify
#[macro_export]
macro_rules! expect {
    ($($tokens:tt)*) => {
        $crate::__expect_inner!($($tokens)*)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __expect_inner {
    // Entrypoint
    (
        $subject:expr,
        $($assertions:tt)*
    ) => {{
        let cx = $crate::assertions::AssertionContext::__new(
            $crate::source_loc!(),
            $crate::__expect_inner!(
                @build_ctx_frames,
                [],
                $($assertions)*
            ),
        );
        let result = $crate::__expect_inner!(
            @build_assertion,
            cx,
            $subject,
            $($assertions)*
        );
        $crate::assertions::general::FinalizableResult::finalize(result)
    }};

    // Build context frame names (from modifier/assertion names)
    (
        // Recursive case
        @build_ctx_frames,
        [$($frames:expr),*],
        $frame_name:ident $(($($_:tt)*))?,
        $($assertions:tt)*
    ) => {
        $crate::__expect_inner!(
            @build_ctx_frames,
            [$($frames,)* ::core::stringify!($frame_name)],
            $($assertions)*
        )
    };
    (
        // Base case
        @build_ctx_frames,
        [$($frames:expr),*],
        $frame_name:ident $(($($_:tt)*))?
    ) => {{
        const FRAMES: &'static [&'static str] = &[
            $($frames,)*
            ::core::stringify!($frame_name),
        ];
        FRAMES
    }};

    // Build assertion (chain modifiers and final assertion)
    (
        // Recursive case (with params)
        @build_assertion,
        $cx:expr,
        $subject:expr,
        $assertion:ident($($param:expr),* $(,)?),
        $($rest:tt)*
    ) => {
        $crate::__expect_inner!(
            @annotate_assertion,
            $cx,
            $subject,
            |cx, subject| {
                let assertion = $assertion($($crate::annotated!($param),)*);
                assertion(cx, subject, |cx, no_debug_impl| {
                    $crate::__expect_inner!(
                        @build_assertion,
                        cx,
                        no_debug_impl,
                        $($rest)*
                    )
                })
            }
        )
    };
    (
        // Recursive case (without params)
        @build_assertion,
        $cx:expr,
        $subject:expr,
        $assertion:ident,
        $($rest:tt)*
    ) => {
        $crate::__expect_inner!(
            @annotate_assertion,
            $cx,
            $subject,
            |cx, subject| {
                $assertion(cx, subject, |cx, no_debug_impl| {
                    $crate::__expect_inner!(
                        @build_assertion,
                        cx,
                        no_debug_impl,
                        $($rest)*
                    )
                })
            }
        )
    };
    (
        // Base case (with params)
        @build_assertion,
        $cx:expr,
        $subject:expr,
        $assertion:ident($($param:expr),* $(,)?) $(,)?
    ) => {
        $crate::__expect_inner!(
            @annotate_assertion,
            $cx,
            $subject,
            |cx, subject| {
                let assertion = $assertion($($crate::annotated!($param),)*);
                assertion(cx, subject)
            }
        )
    };
    (
        // Base case (without params)
        @build_assertion,
        $cx:expr,
        $subject:expr,
        $assertion:ident $(,)?
    ) => {
        $crate::__expect_inner!(
            @annotate_assertion,
            $cx,
            $subject,
            |cx, subject| {
                let assertion = $assertion;
                assertion(cx, subject)
            }
        )
    };

    // Wrap assertion and annotate intermediate value
    (
        @annotate_assertion,
        $cx:expr,
        $subject:expr,
        |$cx_param:pat_param, $subject_param:pat_param| $assertion:expr
    ) => {
        $crate::assertions::__annotate_assertion(
            $cx,
            $crate::annotated!($subject),
            |$cx_param, $subject_param| $assertion,
        )
    };
}

#[cfg(test)]
mod tests {
    use core::future::ready;
    use std::marker::PhantomData;

    use crate::{
        assertions::{
            general::{FinalizableResult, InvertibleResult},
            Assertion, AssertionContext, AssertionModifier,
        },
        metadata::Annotated,
        prelude::*,
        AssertionResult,
    };

    #[derive(PartialEq)]
    struct NotDebug<T>(T);

    #[tokio::test]
    async fn test_debugging() {
        debugging().await;
    }

    async fn debugging() {
        expect!(
            [NotDebug(1), NotDebug(2), NotDebug(3)],
            all,
            map(|x: NotDebug<_>| x.0),
            not,
            to_be_less_than(3)
        );

        expect!([ready(1), ready(2)], all, when_ready, to_be_less_than(2)).await;
    }

    // async fn debugging2() {
    //     // expect!(1, not, to_equal(0));

    //     let x = 1;
    //     expect!(1, not, to_equal(x));

    //     {
    //         const SOURCE_LOC: crate::metadata::SourceLoc = crate::source_loc!();
    //         let cx = crate::assertions::AssertionContext::__new(&SOURCE_LOC, {
    //             const FRAMES: &'static [&'static str] = &[("not"), "to_equal"];
    //             FRAMES
    //         });
    //         let result =
    //             crate::assertions::__annotate_assertion(cx, crate::annotated!(1), |cx, subject| {
    //                 not(cx, subject, |cx, no_debug_impl| {
    //                     crate::assertions::__annotate_assertion(
    //                         cx,
    //                         crate::annotated!(no_debug_impl),
    //                         |cx, subject| {
    //                             let assertion = to_equal(crate::annotated!(0));
    //                             assertion(cx, subject)
    //                         },
    //                     )
    //                 })
    //             });
    //         crate::assertions::general::FinalizableResult::finalize(result)
    //     }

    //     /*
    //     {
    //         let subject = crate::annotated!(1);
    //         let assertion = __annotate_assertion_begin(
    //             &subject,
    //             |x| crate::annotated!(x),
    //         );
    //         let assertion = assertion.apply(
    //             not().apply(
    //                 __annotate_assertion2(
    //                 to_equal(0)
    //             )
    //         );
    //     }
    //     */
    // }

    #[test]
    fn debugging3() {
        let _res = {
            let cx = AssertionContext::__new(
                crate::source_loc!(),
                &["not2", "not2", "not2", "to_equal2"],
            );

            // TODO
            let chain = Root(1);
            let chain = annotate_input(|x| crate::annotated!(x))(chain);
            let chain = not2(chain);
            let chain = annotate_input(|x| crate::annotated!(x))(chain);
            let chain = not2(chain);
            let chain = annotate_input(|x| crate::annotated!(x))(chain);
            let chain = not2(chain);
            let chain = annotate_input(|x| crate::annotated!(x))(chain);
            let assert = to_equal2(1);
            let res = chain.apply(cx, assert);

            res.finalize()
        };
    }

    struct Root<T>(T);

    impl<T, A> AssertionModifier<A> for Root<T>
    where
        A: Assertion<T>,
    {
        type Output = A::Output;

        fn apply(self, cx: AssertionContext, assertion: A) -> Self::Output {
            assertion.execute(cx, self.0)
        }
    }

    fn annotate_input<T, M>(
        annotate: fn(T) -> Annotated<T>,
    ) -> impl FnOnce(M) -> AnnotateModifier<T, M> {
        move |prev| AnnotateModifier { prev, annotate }
    }

    #[derive(Clone, Debug)]
    struct AnnotateModifier<T, M> {
        prev: M,
        annotate: fn(T) -> Annotated<T>,
    }

    impl<T, M, A> AssertionModifier<A> for AnnotateModifier<T, M>
    where
        M: AssertionModifier<AnnotateAssertion<A, T>>,
    {
        type Output = M::Output;

        fn apply(self, cx: AssertionContext, assertion: A) -> Self::Output {
            self.prev.apply(
                cx,
                AnnotateAssertion {
                    next: assertion,
                    annotate: self.annotate,
                },
            )
        }
    }

    #[derive(Clone, Debug)]
    struct AnnotateAssertion<A, T> {
        next: A,
        annotate: fn(T) -> Annotated<T>,
    }

    impl<A, T> Assertion<T> for AnnotateAssertion<A, T>
    where
        A: Assertion<T>,
    {
        type Output = A::Output;

        fn execute(self, cx: AssertionContext, value: T) -> Self::Output {
            self.next.execute(cx.next(), value)
        }
    }

    #[inline]
    fn not2<T, M>(prev: M) -> Not2Modifier<T, M> {
        Not2Modifier(prev, PhantomData)
    }

    #[derive(Clone, Debug)]
    struct Not2Modifier<T, M>(M, PhantomData<fn(T)>);

    impl<T, M, A> AssertionModifier<A> for Not2Modifier<T, M>
    where
        A: Assertion<T>,
        A::Output: InvertibleResult,
        M: AssertionModifier<Not2Assertion<A>>,
    {
        type Output = M::Output;

        #[inline]
        fn apply(self, cx: AssertionContext, assertion: A) -> Self::Output {
            self.0.apply(cx, Not2Assertion(assertion))
        }
    }

    #[derive(Clone, Debug)]
    struct Not2Assertion<A>(A);

    impl<A, T> Assertion<T> for Not2Assertion<A>
    where
        A: Assertion<T>,
        A::Output: InvertibleResult,
    {
        type Output = <A::Output as InvertibleResult>::Inverted;

        #[inline]
        fn execute(self, cx: AssertionContext, value: T) -> Self::Output {
            self.0.execute(cx.clone(), value).invert(cx)
        }
    }

    fn to_equal2<U>(value: U) -> ToEqual2Assertion<U> {
        ToEqual2Assertion(value)
    }

    #[derive(Clone, Debug)]
    struct ToEqual2Assertion<U>(U);

    impl<T, U> Assertion<T> for ToEqual2Assertion<U>
    where
        T: PartialEq<U>,
    {
        type Output = AssertionResult;

        fn execute(self, cx: AssertionContext, value: T) -> Self::Output {
            if value == self.0 {
                Ok(())
            } else {
                Err(cx.fail("not equal"))
            }
        }
    }
}
