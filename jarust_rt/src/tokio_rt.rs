use futures_util::Future;
use tokio::task::AbortHandle;

pub fn spawn<F>(future: F) -> JaTask
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let jh = tokio::spawn(future);
    JaTask {
        inner: jh.abort_handle(),
    }
}

#[derive(Debug)]
pub struct JaTask {
    inner: AbortHandle,
}

impl JaTask {
    pub fn cancel(&self) {
        self.inner.abort();
    }
}

impl Drop for JaTask {
    fn drop(&mut self) {
        self.cancel();
    }
}
