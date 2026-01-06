//! Assertions and modifiers that are used with [`expect!`], as well as any
//! types used to drive them.
//!
//! When using the [`expect!`] macro, the overall assertion is built up by
//! chaining together modifiers and a final assertion. Modifiers can perform
//! additional checks or transform inputs/outputs to later modifiers/assertions.
//!
//! ```
//! # use expecters::prelude::*;
//! expect!([1, 2, 3], all, to_be_greater_than(0));
//! ```
//!
//! ## Available assertions
//!
//! Many assertions are made available by importing the prelude. To see what
//! assertions are exported by default, look at the [`prelude`](crate::prelude)
//! module's exports. The assertions are defined by traits that are exported by
//! that module.
//!
//! For example, general purpose assertions are found in [`GeneralAssertions`],
//! while assertions on option values are in [`OptionAssertions`].
//!
//! ## Creating an assertion
//!
//! The signature for assertions is simple. An assertion function, like
//! [`to_be_some`], is a function added by a trait to the [`AssertionBuilder`]
//! that returns a value implementing the [`Assertion<T>`] trait. It acts as a
//! constructor for that type. For example, calling `builder.to_be_some()`
//! returns an instance of the [`ToBeOptionVariantAssertion`] type configured to
//! check if the input it receives is of a particular variant of [`Option`].
//!
//! Note that the same type is returned by [`to_be_none`], and these types can
//! be reused if needed.
//!
//! To create your own assertion function, first create the type that represents
//! the assertion, then create the function that produces the type. For example,
//! to create an assertion that passes if it receives a `0`:
//!
//! ```
//! use expecters::{
//!     assertions::{Assertion, AssertionBuilder, AssertionContext},
//!     metadata::Annotated,
//!     prelude::*,
//!     AssertionOutput,
//! };
//!
//! // We need to create a struct for our assertion and define its behavior
//! #[derive(Clone, Debug)]
//! pub struct ToBeZero(Annotated<String>);
//!
//! impl Assertion<i32> for ToBeZero {
//!     // What does this assertion return when it's executed? Sometimes
//!     // assertions want to return other output types, like if they need to
//!     // run asynchronously and have to return a future instead.
//!     type Output = AssertionOutput;
//!
//!     fn execute(self, mut cx: AssertionContext, value: i32) -> Self::Output {
//!         // You can annotate the context with additional information
//!         cx.annotate("my note", "this appears in failure messages");
//!         cx.annotate("input parameter", &self.0);
//!
//!         // Then execute your assertion
//!         cx.pass_if(value == 0, "was not zero")
//!     }
//! }
//!
//! // Now we need to attach our assertion to the assertion builder. We attach
//! // it through a trait implementation on the builder itself:
//! trait MyAssertions {
//!     // Input parameters are automatically annotated, so we need to wrap them
//!     // with `Annotated<T>`
//!     fn to_be_zero(&self, note: Annotated<String>) -> ToBeZero {
//!         ToBeZero(note)
//!     }
//! }
//!
//! // By implementing only for `AssertionBuilder<i32, M>`, we constrain our
//! // assertion to only be allowed on assertions against `i32` values. This is
//! // consistent with our `Assertion<i32>` implementation above.
//! impl<M> MyAssertions for AssertionBuilder<i32, M> {}
//!
//! // Now we can use the assertion:
//! expect!(0, to_be_zero("hello, world!".to_string()));
//! // You can also use modifiers with your assertion:
//! expect!(1, not, to_be_zero("this assertion is negated".to_string()));
//! ```
//!
//! An assertion function that takes no parameters can be called without
//! parentheses when using the [`expect!`] macro. For example, if the assertion
//! function signature is `pub fn to_be_zero() -> ToBeZeroAssertion`, then the
//! assertion can be used like `expect!(0, to_be_zero)`.
//!
//! ## Creating a modifier
//!
//! Modifiers are special types that wrap assertions in their own assertion,
//! then pass their assertion up the chain to the previous modifier. When
//! working with modifiers, it's important to keep in mind the direction that
//! data flows when both building up the intermediate assertion, and executing
//! the assertion.
//!
//! In the code `expect!(1, not, to_equal(2))`, there is the explicit [`not`]
//! modifier, two implicit modifiers added by this crate to track values being
//! passed around and update the assertion context, and one special root
//! modifier that holds the original subject of the assertion. The order that
//! modifiers are being applied is:
//!
//! 1. The root modifier, which holds `1` and drives the assertion.
//! 2. A hidden modifier which annotates intermediate values and notifies the
//!    context that the next step in the assertion has begun.
//! 3. The [`not`] modifier, which negates the rest of the assertion.
//! 4. The hidden modifier from step 2.
//! 5. The [`to_equal`] assertion. This is not a modifier, and is the root
//!    assertion that all the modifiers are wrapping.
//!
//! The modifiers are constructed in the above order, going from steps 1 through
//! 4, and wrapping the previous modifiers to generate a deeply nested
//! "composite modifier" that represents all those steps. Afterwards, the
//! [`to_equal`] assertion is provided to the composite modifier, and that flows
//! in reverse order back from steps 4 through 1, getting wrapped in another
//! assertion on each step.
//!
//! To create your own modifier, you should create two types:
//! - One that represents the modifier itself (which gets constructed on the
//!   first pass, when we're going from the root modifier down to the assertion)
//! - One that represents the assertion the modifier creates when wrapping
//!   another assertion, which happens on the second pass when we're passing the
//!   assertion at the end of the chain back down to the root.
//!
//! To use the modifier with the [`expect!`] macro, you should also define a
//! function for the modifier in your trait. The function should take the
//! builder by value (`self`), and may define any number of additional inputs
//! that can be used to configure the modifier. It should return the modified
//! builder.
//!
//! ```
//! use expecters::{
//!     assertions::{
//!         Assertion,
//!         AssertionBuilder,
//!         AssertionContext,
//!         AssertionContextBuilder,
//!         AssertionModifier,
//!     },
//!     metadata::Annotated,
//!     prelude::*,
//! };
//!
//! // This wraps the modifier chain (first pass, going from root -> assertion)
//! #[derive(Clone, Debug)]
//! pub struct DividedByModifier<M>(M, Annotated<f32>);
//!
//! impl<M, A> AssertionModifier<A> for DividedByModifier<M>
//! where
//!     M: AssertionModifier<DividedByAssertion<A>>,
//! {
//!     type Output = M::Output; // the output at this step, usually M::Output
//!
//!     fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
//!         self.0.apply(cx, DividedByAssertion(next, self.1))
//!     }
//! }
//!
//! // This wraps the assertion chain (second pass, assertion -> root)
//! #[derive(Clone, Debug)]
//! pub struct DividedByAssertion<A>(A, Annotated<f32>);
//!
//! impl<A> Assertion<f32> for DividedByAssertion<A>
//! where
//!     A: Assertion<f32>
//! {
//!     type Output = A::Output; // output from this assertion
//!
//!     fn execute(self, mut cx: AssertionContext, subject: f32) -> Self::Output {
//!         cx.annotate("divisor", &self.1);
//!         self.0.execute(cx, subject / self.1.into_inner())
//!     }
//! }
//!
//! // Now we need to attach our modifier. We can reuse an existing trait if we
//! // want, if the input types are compatible. Note that we now take `self`
//! // instead of `&self` (unlike the `to_be_zero` assertion):
//! trait MyAssertions<M> {
//!     // We return an `AssertionBuilder<f32, ...>` here because we're passing
//!     // a `f32` value to whatever assertion we receive. If we were to convert
//!     // the input `f32` into a `String`, for example, then we'd instead want
//!     // to return `AssertionBuilder<String, ...>` here to ensure that only
//!     // string assertions can be applied to it.
//!     fn divided_by(
//!         self,
//!         divisor: Annotated<f32>,
//!     ) -> AssertionBuilder<f32, DividedByModifier<M>>;
//! }
//!
//! impl<M> MyAssertions<M> for AssertionBuilder<f32, M> {
//!     fn divided_by(
//!         self,
//!         divisor: Annotated<f32>,
//!     ) -> AssertionBuilder<f32, DividedByModifier<M>> {
//!         // We can't call `self.modify` because `modify` doesn't take `self`
//!         // as its first parameter. This is to make sure you don't
//!         // accidentally treat `modify` as an assertion when calling
//!         // `expect!`. Instead, we do `AssertionBuilder::modify` and pass the
//!         // builder as the first parameter to modify the assertion:
//!         AssertionBuilder::modify(
//!             self,
//!             // This constructs our modifier:
//!             move |prev| DividedByModifier(prev, divisor),
//!         )
//!     }
//! }
//!
//! // Now we can use our modifier
//! expect!(4.0, divided_by(2.0), to_be_less_than(2.1));
//! ```
//!
//! Similar to assertions, modifiers that take no arguments can be used in the
//! [`expect!`] macro without an argument list. [`not`] is an example of this,
//! and common usage of it is without parentheses, though parentheses are still
//! allowed.
//!
//! [`GeneralAssertions`]: crate::prelude::GeneralAssertions
//! [`OptionAssertions`]: crate::prelude::OptionAssertions
//! [`ToBeOptionVariantAssertion`]: options::ToBeOptionVariant
//! [`expect!`]: crate::expect!
//! [`not`]: crate::prelude::GeneralAssertions::not
//! [`to_be_none`]: crate::prelude::OptionAssertions::to_be_none
//! [`to_be_some`]: crate::prelude::OptionAssertions::to_be_some
//! [`to_equal`]: crate::prelude::GeneralAssertions::to_equal

#[cfg(feature = "futures")]
pub mod async_read;
pub mod functions;
#[cfg(feature = "futures")]
pub mod futures;
pub mod general;
pub mod iterators;
pub mod options;
pub mod read;
pub mod results;
pub mod strings;

mod assertion;
mod context;
mod error;

pub use assertion::*;
pub use context::*;
pub use error::*;
