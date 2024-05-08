use super::events::PluginEvent;
use super::handle::AudioBridgeHandle;
use jarust::prelude::*;
use std::ops::Deref;
use tokio::sync::mpsc;

#[async_trait::async_trait]
pub trait AudioBridge: Attach {
    type Event: TryFrom<JaResponse, Error = JaError> + Send + Sync + 'static;
    type Handle: From<JaHandle> + Deref<Target = JaHandle> + PluginTask;

    async fn attach_audio_bridge(
        &self,
    ) -> JaResult<(Self::Handle, mpsc::UnboundedReceiver<Self::Event>)> {
        let (handle, mut receiver) = self.attach("janus.plugin.audiobridge").await?;
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let abort_handle = jarust_rt::spawn(async move {
            while let Some(rsp) = receiver.recv().await {
                if let Ok(event) = rsp.try_into() {
                    let _ = tx.send(event);
                };
            }
        });
        let mut handle: Self::Handle = handle.into();
        handle.assign_cancellation(abort_handle);
        Ok((handle, rx))
    }
}

impl AudioBridge for JaSession {
    type Event = PluginEvent;
    type Handle = AudioBridgeHandle;
}
