use std::{
    fmt::Debug,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use pin_project_lite::pin_project;

use crate::assertions::{
    general::{InitializableOutput, IntoInitializableOutput},
    AssertionContext,
};

pin_project! {
    /// An asynchronous output that can be initialized on-demand.
    pub struct InitializedOutputFuture<F>
    where
        F: Future,
    {
        #[pin]
        inner: Inner<F>,
    }
}

impl<F> Clone for InitializedOutputFuture<F>
where
    F: Future<Output: Clone> + Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<F> Debug for InitializedOutputFuture<F>
where
    F: Future<Output: Debug> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InitializedOutputFuture")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<F> Future for InitializedOutputFuture<F>
where
    F: Future<Output: InitializableOutput>,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let projected = self.project().inner.project();
        match projected {
            InnerEnum::Pass { data } => {
                let cx = data.take().expect("poll after ready");
                Poll::Ready(F::Output::pass(cx))
            }
            InnerEnum::Fail { data } => {
                let (cx, message) = data.take().expect("poll after ready");
                Poll::Ready(F::Output::fail(cx, message))
            }
            InnerEnum::Wrap { inner } => inner.poll(cx),
        }
    }
}

pin_project! {
    #[project = InnerEnum]
    #[derive(Clone, Debug)]
    enum Inner<F>
    where
        F: Future,
    {
        Pass {
            data: Option<AssertionContext>,
        },
        Fail {
            data: Option<(AssertionContext, String)>,
        },
        Wrap {
            #[pin]
            inner: F,
        },
    }
}

impl<F> InitializableOutput for InitializedOutputFuture<F>
where
    F: Future<Output: InitializableOutput>,
{
    #[inline]
    fn pass(cx: AssertionContext) -> Self {
        Self {
            inner: Inner::Pass { data: Some(cx) },
        }
    }

    #[inline]
    fn fail(cx: AssertionContext, message: String) -> Self {
        Self {
            inner: Inner::Fail {
                data: Some((cx, message)),
            },
        }
    }
}

impl<F> IntoInitializableOutput for F
where
    F: Future<Output: InitializableOutput>,
{
    type Initialized = InitializedOutputFuture<Self>;

    #[inline]
    fn into_initialized(self) -> Self::Initialized {
        InitializedOutputFuture {
            inner: Inner::Wrap { inner: self },
        }
    }
}
