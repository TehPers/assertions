use std::fmt::{Debug, Display};

use crate::{assertions::AssertionBuilder, metadata::Annotated};

use super::{AsDebugModifier, AsDisplayModifier, ToContainSubstr};

/// Assertions and modifiers for [`String`]s.
pub trait StringAssertions<T, M>
where
    T: AsRef<str>,
{
    /// Asserts that the subject contains the given substring.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!("Hello, world!", to_contain_substr("world"));
    /// ```
    ///
    /// The assertion fails if the subject does not contain the substring:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// // not case-insensitive
    /// expect!("Hello, world!", to_contain_substr("WORLD"));
    /// ```
    #[inline]
    #[must_use]
    fn to_contain_substr<P>(&self, pattern: Annotated<P>) -> ToContainSubstr<P>
    where
        P: AsRef<str>,
    {
        ToContainSubstr::new(pattern)
    }

    /// Asserts that the subject matches the given regular expression.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!("12345", to_match_regex(r"\d+"));
    /// ```
    ///
    /// The assertion fails if the subject does not match the pattern:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!("abcde", to_match_regex(r"\d+"));
    /// ```
    ///
    /// ## Panics
    ///
    /// This panics immediately, without executing the assertion, if the provided
    /// pattern is an invalid regular expression.
    #[inline]
    #[must_use]
    #[cfg(feature = "regex")]
    fn to_match_regex<P>(&self, pattern: Annotated<P>) -> super::ToMatchRegexAssertion
    where
        P: AsRef<str>,
    {
        super::ToMatchRegexAssertion::new(pattern.inner().as_ref())
    }
}

impl<T, M> StringAssertions<T, M> for AssertionBuilder<T, M> where T: AsRef<str> {}

/// Assertions and modifiers for types with a [`Debug`] representation.
pub trait DebugAssertions<T, M>
where
    T: Debug,
{
    /// Converts a value to its [`Debug`] representation.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!("hello", as_debug, to_equal(r#""hello""#));
    /// ```
    #[allow(clippy::wrong_self_convention)]
    fn as_debug(self) -> AssertionBuilder<String, AsDebugModifier<M>>
    where
        T: Debug;
}

impl<T, M> DebugAssertions<T, M> for AssertionBuilder<T, M>
where
    T: Debug,
{
    #[inline]
    fn as_debug(self) -> AssertionBuilder<String, AsDebugModifier<M>> {
        AssertionBuilder::modify(self, AsDebugModifier::new)
    }
}

/// Assertions and modifiers for types with a [`Display`] representation.
pub trait DisplayAssertions<T, M>
where
    T: Display,
{
    /// Converts a value to its [`Display`] representation.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(1, as_display, to_equal("1"));
    /// ```
    #[allow(clippy::wrong_self_convention)]
    fn as_display(self) -> AssertionBuilder<String, AsDisplayModifier<M>>
    where
        T: Display;
}

impl<T, M> DisplayAssertions<T, M> for AssertionBuilder<T, M>
where
    T: Display,
{
    #[inline]
    fn as_display(self) -> AssertionBuilder<String, AsDisplayModifier<M>> {
        AssertionBuilder::modify(self, AsDisplayModifier::new)
    }
}