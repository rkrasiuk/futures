mod or;
use or::*;

/// An extension trait for `Future`s that provides a variety of convenient adapters.
pub trait FutureExt: Future {
    /// Returns the result of the first resolved future.
    fn or<F>(self, fut2: F) -> OrFuture<Self, F>
    where
        Self: Sized,
    {
        OrFuture::new(self, fut2)
    }
}
