use crate::jatask::AbortHandle;
use crate::prelude::*;
use async_trait::async_trait;

pub trait PluginTask {
    fn assign_aborts(&mut self, abort_handles: Vec<AbortHandle>);
    fn abort_plugin(&mut self);
}

#[async_trait]
pub trait Attach {
    async fn attach(&self, plugin_id: &str) -> JaResult<(JaHandle, JaResponseStream)>;
}
