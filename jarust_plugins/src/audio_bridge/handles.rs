use super::{
    messages::AudioBridgeListMsg,
    results::{AudioBridgePluginData, AudioBridgePluginEvent, Room},
};
use jarust::prelude::*;
use std::ops::Deref;
use tokio::task::AbortHandle;

pub struct AudioBridgeHandle {
    handle: JaHandle,
    abort_handles: Option<Vec<AbortHandle>>,
}

impl AudioBridgeHandle {
    // pub async fn start(&self, request: AudioBridgeStartMsg) -> JaResult<()> {
    //     self.handle.message(serde_json::to_value(request)?).await
    // }

    // pub async fn create(&self, request: AudioBridgeCreateMsg) {}

    pub async fn list(&self) -> JaResult<Vec<Room>> {
        let response = self
            .handle
            .message_with_result::<AudioBridgePluginData>(serde_json::to_value(
                AudioBridgeListMsg::default(),
            )?)
            .await?;

        let result = match response.event {
            AudioBridgePluginEvent::List { list, .. } => list,
        };
        Ok(result)
    }
}

impl PluginTask for AudioBridgeHandle {
    fn assign_aborts(&mut self, abort_handles: Vec<AbortHandle>) {
        self.abort_handles = Some(abort_handles);
    }

    fn abort_plugin(&mut self) {
        if let Some(abort_handles) = self.abort_handles.take() {
            for abort_handle in abort_handles {
                abort_handle.abort();
            }
        };
    }
}

impl From<JaHandle> for AudioBridgeHandle {
    fn from(handle: JaHandle) -> Self {
        Self {
            handle,
            abort_handles: None,
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
