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
//! ## Creating an assertion
//!
//! The signature for assertions is simple. An assertion function, like
//! [`to_be_some`](crate::prelude::to_be_some), is a regular function that
//! returns a value implementing the [`Assertion<T>`] trait. It acts as a
//! constructor for that type. For example, calling `to_be_some()` returns an
//! instance of the [`ToBeOptionVariantAssertion`] type configured to check if
//! the input it receives is of the [`Some`] variant.
//!
//! To create your own assertion function, first create the type that represents
//! the assertion, then create the function that produces the type. For example,
//! to create an assertion that passes if it receives a `0`:
//!
//! ```
//! use expecters::{
//!     assertions::{Assertion, AssertionContext},
//!     metadata::Annotated,
//!     prelude::*,
//!     AssertionResult,
//! };
//!
//! // Input parameters are automatically annotated, so we need to wrap them
//! // with `Annotated<T>`
//! pub fn to_be_zero(annotation: Annotated<String>) -> ToBeZeroAssertion {
//!     ToBeZeroAssertion(annotation)
//! }
//!
//! #[derive(Clone, Debug)]
//! pub struct ToBeZeroAssertion(Annotated<String>);
//!
//! impl Assertion<i32> for ToBeZeroAssertion {
//!     // What does this assertion return when it's executed? Sometimes
//!     // assertions want to return other output types, like if they need to
//!     // run asynchronously and have to return a future instead.
//!     type Output = AssertionResult;
//!
//!     fn execute(self, mut cx: AssertionContext, value: i32) -> Self::Output {
//!         cx.annotate("my annotation", "this appears in failure messages");
//!         cx.annotate("input parameter", &self.0);
//!         cx.pass_if(value == 0, "was not zero")
//!     }
//! }
//!
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
//! function for the modifier. The function should take at minimum two required
//! inputs (but it may specify additional inputs), and should return a pair
//! containing the constructed modifier and a subject key.
//!
//! ```
//! use expecters::{
//!     assertions::{
//!         Assertion,
//!         AssertionContext,
//!         AssertionModifier,
//!         SubjectKey,
//!         key,
//!     },
//!     metadata::Annotated,
//!     prelude::*,
//! };
//!
//! // The first two parameters are required, but you may specify any number of
//! // additional parameters to your modifier:
//! pub fn divided_by<M>(
//!     prev: M, // the modifier we're wrapping
//!     _: SubjectKey<f32>, // the type we're expecting to receive in this step
//!     divisor: Annotated<f32>, // our own custom parameter
//! ) -> (
//!     DividedByModifier<M>, // our constructed modifier type, wrapping M
//!     SubjectKey<f32>, // the type we're passing to the next step
//! ) {
//!     (DividedByModifier(prev, divisor), key())
//! }
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
//!     fn apply(self, next: A) -> Self::Output {
//!         self.0.apply(DividedByAssertion(next, self.1))
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
//! expect!(4.0, divided_by(2.0), to_be_less_than(2.1));
//! ```
//!
//! Similar to assertions, modifiers that take no arguments can be used in the
//! [`expect!`] macro without an argument list. [`not`] is an example of this,
//! and common usage of it is without parentheses, though parentheses are still
//! allowed.
//!
//! [`ToBeOptionVariantAssertion`]: options::ToBeOptionVariantAssertion
//! [`expect!`]: crate::expect!
//! [`not`]: crate::prelude::not
//! [`to_equal`]: crate::prelude::to_equal

// pub mod functions;
#[cfg(feature = "futures")]
pub mod futures;
pub mod general;
pub mod iterators;
pub mod options;
pub mod results;
pub mod strings;

mod assertion;
mod context;
mod error;

pub use assertion::*;
pub use context::*;
pub use error::*;
