use crate::params::AttachHandleParams;
use crate::prelude::*;
use async_trait::async_trait;
use jarust_rt::JaTask;

pub trait PluginTask {
    fn assign_task(&mut self, task: JaTask);
    fn cancel_task(&mut self);
}

#[async_trait]
pub trait Attach {
    async fn attach(&self, params: AttachHandleParams) -> JaResult<(JaHandle, JaResponseStream)>;
}
