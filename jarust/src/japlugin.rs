use crate::prelude::*;
use async_trait::async_trait;
use jarust_rt::AbortHandle;

pub trait PluginTask {
    fn assign_cancellation(&mut self, cancellable: AbortHandle);
    fn invoke_cancellation(&mut self);
}

#[async_trait]
pub trait Attach {
    async fn attach(&self, plugin_id: &str) -> JaResult<(JaHandle, JaResponseStream)>;
}
