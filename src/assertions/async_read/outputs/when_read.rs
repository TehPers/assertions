use std::{
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};

use futures::AsyncRead;
use pin_project_lite::pin_project;

use crate::assertions::{general::IntoInitializableOutput, Assertion, AssertionContext};

pin_project! {
    /// Asynchronously reads a subject and executes an assertion on it.
    #[derive(Clone, Debug)]
    pub struct WhenReadAsyncFuture<T, A> {
        #[pin]
        subject: T,
        buffer: Vec<u8>,
        result: Vec<u8>,
        next: Option<(AssertionContext, A)>
    }
}

impl<T, A> WhenReadAsyncFuture<T, A> {
    #[inline]
    pub(crate) fn new(cx: AssertionContext, subject: T, next: A) -> Self {
        WhenReadAsyncFuture {
            subject,
            buffer: vec![0; 32],
            result: Vec::new(),
            next: Some((cx, next)),
        }
    }
}

impl<T, A> Future for WhenReadAsyncFuture<T, A>
where
    T: AsyncRead,
    A: Assertion<Vec<u8>, Output: IntoInitializableOutput>,
{
    type Output = <A::Output as IntoInitializableOutput>::Initialized;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut projected = self.project();

        // Read the subject
        loop {
            let result = ready!(projected.subject.as_mut().poll_read(cx, projected.buffer));
            match result {
                Ok(0) => break,
                Ok(n) => {
                    projected.result.extend(&projected.buffer[..n]);

                    // Check if we can grow the buffer for the next read
                    if n == projected.buffer.len() {
                        projected.buffer.reserve(32);
                        projected.buffer.resize(projected.buffer.capacity(), 0);
                    }
                }
                Err(error) => {
                    let (mut cx, _) = projected.next.take().expect("poll after ready");
                    cx.annotate("error", error);
                    return Poll::Ready(cx.fail("failed to read"));
                }
            };
        }

        let (mut cx, next) = projected.next.take().expect("poll after ready");
        cx.annotate("read bytes", projected.result.len());
        Poll::Ready(
            next.execute(cx, std::mem::take(projected.result))
                .into_initialized(),
        )
    }
}

#[cfg(test)]
mod tests {
    use futures::io::Cursor;

    use crate::prelude::*;

    #[tokio::test]
    async fn long_data() {
        let subject = "Hello, world! ".repeat(100);
        expect!(
            Cursor::new(subject.clone()),
            when_read_async,
            as_utf8,
            to_equal(subject),
        )
        .await;
    }
}
