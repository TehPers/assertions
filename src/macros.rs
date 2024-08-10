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
/// This macro is called like a function. For example:
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
/// Async assertions function similar to sync assertions, but need to be
/// `.await`ed. For more information, see the
/// [`futures`](crate::assertions::futures) module.
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
/// [`Debug`]: std::fmt::Debug
/// [`all`]: crate::prelude::all
/// [`map`]: crate::prelude::map
/// [`not`]: crate::prelude::not
/// [`to_equal`]: crate::prelude::to_equal
/// [stringified]: std::stringify
#[macro_export]
macro_rules! expect {
    ($($tokens:tt)*) => {
        $crate::assertions::general::UnwrappableOutput::unwrap(
            $crate::__expect_inner!($($tokens)*),
        )
    };
}

/// Same as [`expect!`], but returns the result itself rather than panicking on
/// failure.
///
/// More specifically, this does not finalize the output of the assertion. The
/// syntax is exactly the same as [`expect!`] (and async assertions should still
/// be `.await`ed as usual), but the output from it will be a result type that
/// can be inspected rather than panicking on failure.
///
/// ```
/// # use expecters::prelude::*;
/// let result = try_expect!(1, to_equal(2));
/// expect!(result.into_result(), to_be_err);
/// ```
///
/// See [`expect!`] for more information on how to use this macro.
#[macro_export]
macro_rules! try_expect {
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
        let subject = $crate::annotated!($subject);
        let cx = $crate::assertions::AssertionContext::__new(
            ::std::string::ToString::to_string(&subject),
            $crate::source_loc!(),
            $crate::__expect_inner!(
                @build_ctx_frames,
                [],
                $($assertions)*
            ),
        );
        let (root, _key) = $crate::assertions::general::__root(cx, subject);
        $crate::__expect_inner!(
            @build_assertion,
            root,
            _key,
            $($assertions)*
        )
    }};

    // Build context frame names (from modifier/assertion names)
    (
        // Base case
        @build_ctx_frames,
        [$($frames:expr),*],
        $frame_name:ident $(($($_:tt)*))?
        $(,)?
    ) => {{
        const FRAMES: &'static [&'static str] = &[
            $($frames,)*
            ::std::stringify!($frame_name),
        ];
        FRAMES
    }};
    (
        // Recursive case
        @build_ctx_frames,
        [$($frames:expr),*],
        $frame_name:ident $(($($_:tt)*))?,
        $($assertions:tt)*
    ) => {
        $crate::__expect_inner!(
            @build_ctx_frames,
            [$($frames,)* ::std::stringify!($frame_name)],
            $($assertions)*
        )
    };

    // Build assertion (chain modifiers and final assertion)
    (
        // Base case (with params)
        @build_assertion,
        $chain:expr,
        $key:expr,
        $assertion:ident($($param:expr),* $(,)?)
        $(,)?
    ) => {{
        let (chain, _key) = $crate::__expect_inner!(@annotate, $chain, $key);
        let assertion = $assertion($($crate::annotated!($param),)*);
        $crate::assertions::AssertionModifier::apply(
            chain,
            assertion,
        )
    }};
    (
        // Base case (without params)
        @build_assertion,
        $chain:expr,
        $key:expr,
        $assertion:ident
        $(,)?
    ) => {
        $crate::__expect_inner!(@build_assertion, $chain, $key, $assertion())
    };
    (
        // Recursive case (with params)
        @build_assertion,
        $chain:expr,
        $key:expr,
        $modifier:ident($($param:expr),* $(,)?),
        $($rest:tt)*
    ) => {{
        let modifier = $modifier(
            $($crate::annotated!($param),)*
        );
        $crate::__expect_inner!(
            @build_assertion,
            $chain,
            $key,
            modifier,
            $($rest)*
        )
    }};
    (
        // Recursive case (without params)
        @build_assertion,
        $chain:expr,
        $key:expr,
        $modifier:ident,
        $($rest:tt)*
    ) => {{
        let (chain, _key) = $crate::__expect_inner!(@annotate, $chain, $key);
        let (chain, _key) = $modifier(chain, _key);
        $crate::__expect_inner!(@build_assertion, chain, _key, $($rest)*)
    }};

    // Annotate the value being passed down the chain
    (@annotate, $chain:expr, $key:expr) => {
        $crate::assertions::general::__annotate(
            $key,
            $chain,
            |not_debug| $crate::annotated!(not_debug),
        )
    };
}

#[cfg(test)]
mod tests {
    use std::future::ready;

    use crate::prelude::*;

    #[derive(Clone, PartialEq)]
    struct NotDebug<T>(T);

    #[tokio::test]
    #[ignore]
    async fn test_debugging() {
        debugging().await;
    }

    async fn debugging() {
        // expect!(ready(1), when_ready, to_equal(2)).await;
        // expect!([1, 2, 3], count, not, to_equal(3));
        // expect!([1, 2, 3], any, to_equal(4));
        expect!([1, 2, 3], all, to_satisfy(|n| n % 2 == 1));

        expect!("blah", to_match_regex(r"\d+"));

        expect!(
            "Hello, world!",
            map(|s: &str| format!("{} {} {}", s.to_uppercase(), s.to_lowercase(), s)),
            to_contain_substr("not present in this text because this is a really long substring"),
        );

        expect!(
            ready([NotDebug(1), NotDebug(2), NotDebug(3)]),
            when_ready,
            any,
            not,
            map(|NotDebug(x)| x),
            to_be_less_than(4),
        )
        .await;

        // expect!(ready([1, 2, 3]), when_ready, nth(3), to_equal(3)).await;
        // expect!(ready(1), when_ready, to_satisfy(|n| n < 0)).await;

        // let res = try_expect!(ready(1), when_ready, to_satisfy(|n| n < 0)).await;
        // res.into_result().unwrap();

        // expect!([ready(1), ready(2)], all, when_ready, to_be_less_than(2)).await;
    }
}
