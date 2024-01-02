use crate::japrotocol::JaResponse;
use crate::prelude::JaHandle;
use crate::prelude::JaResult;
use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio::task::AbortHandle;

pub trait PluginTask {
    fn assign_abort(&mut self, abort_handle: AbortHandle);
    fn abort_plugin(&mut self);
}

#[async_trait]
pub trait Attach {
    async fn attach(&self, plugin_id: &str) -> JaResult<(JaHandle, mpsc::Receiver<JaResponse>)>;
}
