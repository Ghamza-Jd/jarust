use crate::prelude::*;
use async_trait::async_trait;
use jarust_rt::JaTask;
use std::time::Duration;
use tokio::sync::mpsc;

pub trait PluginTask {
    fn assign_task(&mut self, task: JaTask);
    fn cancel_task(&mut self);
}

#[async_trait]
pub trait Attach {
    async fn attach(
        &self,
        plugin_id: String,
        timeout: Duration,
    ) -> Result<(JaHandle, mpsc::UnboundedReceiver<JaResponse>), jarust_interface::Error>;
}
