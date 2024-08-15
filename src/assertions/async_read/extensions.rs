use futures::AsyncRead;

use crate::assertions::AssertionBuilder;

use super::WhenReadAsyncModifier;

/// Modifiers for types that implement [`futures::AsyncRead`].
pub trait AsyncReadAssertions<T, M>
where
    T: AsyncRead,
{
    /// Asynchronously reads the subject into a buffer, then executes the
    /// assertion on it.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// use futures::io::Cursor;
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// expect!(
    ///     Cursor::new("Hello, world!"),
    ///     when_read_async,
    ///     as_utf8,
    ///     to_equal("Hello, world!"),
    /// )
    /// .await;
    /// # }
    /// ```
    ///
    /// The assertion fails if reading the subject fails:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// use std::{
    ///     pin::Pin,
    ///     task::{Context, Poll},
    /// };
    ///
    /// use futures::io::{Error, ErrorKind, AsyncRead};
    ///
    /// struct MyReader;
    ///
    /// impl AsyncRead for MyReader {
    ///     fn poll_read(
    ///         self: Pin<&mut Self>,
    ///         _cx: &mut Context,
    ///         _buf: &mut [u8],
    ///     ) -> Poll<std::io::Result<usize>> {
    ///         Poll::Ready(Err(Error::new(ErrorKind::Other, "always fail")))
    ///     }
    /// }
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// expect!(
    ///     MyReader,
    ///     when_read_async,
    ///     count,
    ///     to_be_greater_than_or_equal_to(0),
    /// )
    /// .await;
    /// # }
    /// ```
    fn when_read_async(self) -> AssertionBuilder<Vec<u8>, WhenReadAsyncModifier<M>>;
}

impl<T, M> AsyncReadAssertions<T, M> for AssertionBuilder<T, M>
where
    T: AsyncRead,
{
    #[inline]
    fn when_read_async(self) -> AssertionBuilder<Vec<u8>, WhenReadAsyncModifier<M>> {
        AssertionBuilder::modify(self, WhenReadAsyncModifier::new)
    }
}
