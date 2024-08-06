use std::{
    error::Error,
    fmt::{Display, Formatter},
};

/// An error that indicates an assertion failure.
///
/// This error is formatted to display information about both the failed
/// assertion and the original source of the expectation.
#[derive(Clone, Debug)]
pub struct AssertionError {
    fields: Vec<(&'static str, String)>,
}

impl AssertionError {
    /// Creates a builder for a new [`AssertionError`].
    pub fn builder() -> AssertionErrorBuilder {
        AssertionErrorBuilder::default()
    }
}

impl Display for AssertionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "assertion failed.")?;
        for (name, value) in &self.fields {
            writeln!(f, "  {name}: {value}")?;
        }

        Ok(())
    }
}

impl Error for AssertionError {}

/// A builder for an [`AssertionError`].
#[derive(Clone, Debug)]
pub struct AssertionErrorBuilder {
    fields: Vec<(&'static str, String)>,
}

impl Default for AssertionErrorBuilder {
    fn default() -> Self {
        Self {
            fields: vec![("expected", String::new())],
        }
    }
}

impl AssertionErrorBuilder {
    /// Attaches a custom field to the error. This will appear in the error when
    /// formatting it using its [`Display`] implementation.
    pub fn with_field(mut self, name: &'static str, value: impl Display) -> Self {
        self.fields.push((name, value.to_string()));
        self
    }

    /// Builds the error with the given expectation.
    pub fn build(mut self, expectation: impl Display) -> AssertionError {
        self.fields[0].1 = expectation.to_string();
        AssertionError {
            fields: self.fields,
        }
    }
}
