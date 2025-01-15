//! Future for the `FuturesExt::or` method.

use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

/// Future for the `FuturesExt::or` method.
#[pin_project]
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct OrFuture<F1, F2> {
    #[pin]
    fut1: F1,
    #[pin]
    fut2: F2,
}

impl<F1, F2> OrFuture<F1, F2> {
    pub(crate) fn new(fut1: F1, fut2: F2) -> Self {
        Self { fut1, fut2 }
    }
}

impl<F1, F2, Output> Future for OrFuture<F1, F2>
where
    F1: Future<Output = Output>,
    F2: Future<Output = Output>,
{
    type Output = Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        if let Poll::Ready(result) = this.fut1.poll(cx) {
            return Poll::Ready(result)
        }

        this.fut2.poll(cx)
    }
}
