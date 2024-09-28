use super::events::PluginEvent;
use super::handle::AudioBridgeHandle;
use jarust::japlugin::AttachHandleParams;
use jarust::prelude::*;
use std::ops::Deref;
use std::time::Duration;
use tokio::sync::mpsc;

#[async_trait::async_trait]
pub trait AudioBridge: Attach {
    type Event: TryFrom<JaResponse, Error = jarust_interface::Error> + Send + Sync + 'static;
    type Handle: From<JaHandle> + Deref<Target = JaHandle> + PluginTask;

    async fn attach_audio_bridge(
        &self,
        timeout: Duration,
    ) -> Result<(Self::Handle, mpsc::UnboundedReceiver<Self::Event>), jarust_interface::Error> {
        let (handle, mut receiver) = self
            .attach(AttachHandleParams {
                plugin_id: "janus.plugin.audiobridge".to_string(),
                timeout,
            })
            .await?;
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let task = jarust_rt::spawn("audiobridge listener", async move {
            while let Some(rsp) = receiver.recv().await {
                if let Ok(event) = rsp.try_into() {
                    let _ = tx.send(event);
                };
            }
        });
        let mut handle: Self::Handle = handle.into();
        handle.assign_task(task);
        Ok((handle, rx))
    }
}

impl AudioBridge for JaSession {
    type Event = PluginEvent;
    type Handle = AudioBridgeHandle;
}
