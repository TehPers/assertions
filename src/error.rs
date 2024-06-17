use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

/// An error that indicates an assertion failure.
///
/// This error is formatted to display information about both the failed
/// assertion and the original source of the expectation.
#[derive(Clone, Debug)]
pub struct AssertError {
    fields: Vec<(&'static str, Cow<'static, str>)>,
}

impl AssertError {
    /// Creates a new assertion error. Attach fields using the
    /// [`Self::with_field`] method.
    pub const fn new() -> Self {
        Self { fields: Vec::new() }
    }

    /// Attaches a custom field to the error. This will appear in the error when
    /// formatting it using its [`Display`] implementation.
    pub fn with_field(mut self, name: &'static str, value: impl Into<Cow<'static, str>>) -> Self {
        self.fields.push((name, value.into()));
        self
    }
}

impl Display for AssertError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "assertion failed.")?;
        for (name, value) in &self.fields {
            writeln!(f, "  {name}: {value}")?;
        }

        Ok(())
    }
}
