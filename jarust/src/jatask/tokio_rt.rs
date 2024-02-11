use futures_util::Future;
use tokio::task::AbortHandle as AbortHandleInner;

pub fn spawn<F>(future: F) -> AbortHandle
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let jh = tokio::spawn(future);
    AbortHandle {
        inner: jh.abort_handle(),
    }
}

#[derive(Debug)]
pub struct AbortHandle {
    inner: AbortHandleInner,
}

impl AbortHandle {
    pub fn abort(&self) {
        self.inner.abort();
    }
}

impl Drop for AbortHandle {
    fn drop(&mut self) {
        self.abort();
    }
}
