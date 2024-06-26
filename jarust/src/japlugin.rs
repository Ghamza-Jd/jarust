use crate::prelude::*;
use async_trait::async_trait;
use jarust_rt::JaTask;
use std::time::Duration;

pub trait PluginTask {
    fn assign_task(&mut self, task: JaTask);
    fn cancel_task(&mut self);
}

#[async_trait]
pub trait Attach {
    async fn attach(
        &self,
        plugin_id: &str,
        capacity: usize,
        timeout: Duration,
    ) -> JaResult<(JaHandle, JaResponseStream)>;
}
