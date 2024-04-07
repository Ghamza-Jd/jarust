#[cfg(not(any(feature = "tokio-rt")))]
compile_error!("Feature \"tokio-rt\" must be enabled for this crate.");

#[cfg(feature = "tokio-rt")]
#[path = "tokio_rt.rs"]
pub mod jatask_rt;

use futures_util::Future;
pub use jatask_rt::AbortHandle;

pub fn spawn<F>(future: F) -> AbortHandle
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    jatask_rt::spawn(future)
}
