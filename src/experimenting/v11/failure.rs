use std::{
    error::Error,
    fmt::{Display, Formatter},
};

pub type AssertionResult<T = ()> = Result<T, AssertionFailure>;

/// An error that indicates an assertion failure.
///
/// This error is formatted to display information about both the failed
/// assertion and the original source of the expectation.
#[derive(Clone, Debug)]
pub struct AssertionFailure {
    fields: Vec<(&'static str, String)>,
}

impl AssertionFailure {
    /// Creates a builder for a new failure.
    pub fn builder() -> AssertionFailureBuilder {
        AssertionFailureBuilder::default()
    }
}

impl Display for AssertionFailure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "assertion failed.")?;
        for (name, value) in &self.fields {
            writeln!(f, "  {name}: {value}")?;
        }

        Ok(())
    }
}

impl Error for AssertionFailure {}

/// A builder for a failure.
#[derive(Clone, Debug)]
pub struct AssertionFailureBuilder {
    fields: Vec<(&'static str, String)>,
}

impl Default for AssertionFailureBuilder {
    fn default() -> Self {
        Self {
            fields: vec![("expected", String::new())],
        }
    }
}

impl AssertionFailureBuilder {
    /// Attaches a custom field to the error. This will appear in the error when
    /// formatting it using its [`Display`] implementation.
    pub fn with_field(mut self, name: &'static str, value: impl Display) -> Self {
        self.fields.push((name, value.to_string()));
        self
    }

    /// Builds the error with the given expectation.
    pub fn build(mut self, expectation: impl Display) -> AssertionFailure {
        self.fields[0].1 = expectation.to_string();
        AssertionFailure {
            fields: self.fields,
        }
    }
}
