use std::io::Read;

use crate::assertions::AssertionBuilder;

use super::WhenReadAsBytesModifier;

/// Modifiers for types that implement [`Read`].
pub trait ReadExtensions<T, M>
where
    T: Read,
{
    /// Reads the subject into a buffer, then executes the assertion on it.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// use std::io::Cursor;
    /// expect!(
    ///     Cursor::new("Hello, world!"),
    ///     when_read,
    ///     as_utf8,
    ///     to_equal("Hello, world!"),
    /// );
    /// ```
    ///
    /// The assertion fails if reading the subject fails:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// use std::io::{Error, ErrorKind, Read};
    ///
    /// struct MyReader;
    ///
    /// impl Read for MyReader {
    ///     fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
    ///         Err(Error::new(ErrorKind::Other, "always fail"))
    ///     }
    /// }
    ///
    /// expect!(
    ///     MyReader,
    ///     when_read,
    ///     count,
    ///     to_be_greater_than_or_equal_to(0),
    /// );
    /// ```
    fn when_read(self) -> AssertionBuilder<Vec<u8>, WhenReadAsBytesModifier<M>>;
}

impl<T, M> ReadExtensions<T, M> for AssertionBuilder<T, M>
where
    T: Read,
{
    #[inline]
    fn when_read(self) -> AssertionBuilder<Vec<u8>, WhenReadAsBytesModifier<M>> {
        AssertionBuilder::modify(self, WhenReadAsBytesModifier::new)
    }
}
