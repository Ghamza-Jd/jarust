use super::messages::{AudioBridgeCreateMsg, AudioBridgeListMsg};
use jarust::prelude::*;
use std::ops::Deref;
use tokio::task::AbortHandle;

pub struct AudioBridgeHandle {
    handle: JaHandle,
    abort_handle: Option<AbortHandle>,
}

impl AudioBridgeHandle {
    // pub async fn start(&self, request: AudioBridgeStartMsg) -> JaResult<()> {
    //     self.handle.message(serde_json::to_value(request)?).await
    // }

    // pub async fn create(&self, request: AudioBridgeCreateMsg) {}
    pub async fn list(&self) -> JaResult<()> {
        self.handle
            .message(serde_json::to_value(AudioBridgeListMsg::default())?)
            .await
    }
}

impl PluginTask for AudioBridgeHandle {
    fn assign_abort(&mut self, abort_handle: AbortHandle) {
        self.abort_handle = Some(abort_handle);
    }

    fn abort_plugin(&mut self) {
        if let Some(abort_handle) = self.abort_handle.take() {
            abort_handle.abort();
        };
    }
}

impl From<JaHandle> for AudioBridgeHandle {
    fn from(handle: JaHandle) -> Self {
        Self {
            handle,
            abort_handle: None,
        }
    }
}

impl Deref for AudioBridgeHandle {
    type Target = JaHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl Drop for AudioBridgeHandle {
    fn drop(&mut self) {
        self.abort_plugin();
    }
}
