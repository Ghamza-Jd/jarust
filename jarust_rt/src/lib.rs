#[cfg(not(any(feature = "tokio-rt")))]
compile_error!("Feature \"tokio-rt\" must be enabled for this crate.");

#[cfg(feature = "tokio-rt")]
#[path = "tokio_rt.rs"]
pub mod jatask;

use futures_util::Future;
pub use jatask::JaTask;

pub fn spawn<F>(future: F) -> JaTask
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    jatask::spawn(future)
}
