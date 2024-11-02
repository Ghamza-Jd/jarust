use crate::streaming::params::*;
use crate::streaming::responses::*;
use crate::JanusId;
use jarust_core::prelude::*;
use jarust_rt::JaTask;
use serde_json::json;
use serde_json::Value;
use std::ops::Deref;
use std::time::Duration;

pub struct StreamingHandle {
    handle: JaHandle,
    task: Option<JaTask>,
}

//
// synchronous methods
//
impl StreamingHandle {
    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn create_mountpoint(
        &self,
        params: StreamingCreateParams,
        timeout: Duration,
    ) -> Result<MountpointCreatedRsp, jarust_interface::Error> {
        tracing::info!(plugin = "streaming", "Sending create");
        let mut message: Value = params.try_into()?;
        message["request"] = "create".into();

        self.handle
            .send_waiton_rsp::<MountpointCreatedRsp>(message, timeout)
            .await
    }

    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn destroy_mountpoint(
        &self,
        params: StreamingDestroyParams,
        timeout: Duration,
    ) -> Result<MountpointDestroyedRsp, jarust_interface::Error> {
        tracing::info!(plugin = "streaming", "Sending destroy");
        let mut message: Value = params.try_into()?;
        message["request"] = "destroy".into();

        self.handle
            .send_waiton_rsp::<MountpointDestroyedRsp>(message, timeout)
            .await
    }

    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn list(
        &self,
        timeout: Duration,
    ) -> Result<Vec<MountpointListed>, jarust_interface::Error> {
        tracing::info!(plugin = "streaming", "Sending list");
        let response = self
            .handle
            .send_waiton_rsp::<ListMountpointsRsp>(json!({"request": "list"}), timeout)
            .await?;

        Ok(response.list)
    }

    #[tracing::instrument(level = tracing::Level::DEBUG, skip_all)]
    pub async fn info(
        &self,
        mountpoint: JanusId,
        secret: Option<String>,
        timeout: Duration,
    ) -> Result<MountpointInfo, jarust_interface::Error> {
        tracing::info!(plugin = "streaming", "Sending info");
        let mut message: Value = json!({
            "request": "info",
            "id": mountpoint,
        });
        if let Some(secret) = secret {
            message["secret"] = secret.into();
        }
        let response = self
            .handle
            .send_waiton_rsp::<MountpointInfoRsp>(message, timeout)
            .await?;

        Ok(response.info)
    }

    // TODO:
    // edit
    // enable
    // disable
    // recording
}

// TODO: async methods

impl PluginTask for StreamingHandle {
    fn assign_task(&mut self, task: JaTask) {
        self.task = Some(task);
    }

    fn cancel_task(&mut self) {
        if let Some(task) = self.task.take() {
            task.cancel()
        };
    }
}

impl From<JaHandle> for StreamingHandle {
    fn from(handle: JaHandle) -> Self {
        Self { handle, task: None }
    }
}

impl Deref for StreamingHandle {
    type Target = JaHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl Drop for StreamingHandle {
    fn drop(&mut self) {
        self.cancel_task();
    }
}
