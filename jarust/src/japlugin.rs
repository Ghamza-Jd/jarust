use crate::prelude::*;
use async_trait::async_trait;
use jarust_rt::JaTask;
use std::time::Duration;
use tokio::sync::mpsc;

pub trait PluginTask {
    fn assign_task(&mut self, task: JaTask);
    fn cancel_task(&mut self);
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AttachHandleParams {
    /// Janus plugin identifier
    pub plugin_id: String,
    /// Request timeout
    pub timeout: Duration,
}

#[async_trait]
pub trait Attach {
    async fn attach(
        &self,
        params: AttachHandleParams,
    ) -> JaResult<(JaHandle, mpsc::UnboundedReceiver<JaResponse>)>;
}
