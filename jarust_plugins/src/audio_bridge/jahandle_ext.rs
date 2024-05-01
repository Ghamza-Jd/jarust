use super::events::AudioBridgePluginData;
use super::events::AudioBridgePluginEvent;
use super::handle::AudioBridgeHandle;
use jarust::japrotocol::EstablishmentProtocol;
use jarust::japrotocol::JaEventProtocol;
use jarust::japrotocol::JaResponseProtocol;
use jarust::prelude::*;
use std::ops::Deref;
use tokio::sync::mpsc;

#[async_trait::async_trait]
pub trait AudioBridge: Attach {
    type Event: Send + Sync + 'static;
    type Handle: From<JaHandle> + Deref<Target = JaHandle> + PluginTask;

    fn parse_audio_bridge_message(message: JaResponse) -> JaResult<Self::Event>;

    async fn attach_audio_bridge(
        &self,
    ) -> JaResult<(Self::Handle, mpsc::UnboundedReceiver<Self::Event>)> {
        let (handle, mut receiver) = self.attach("janus.plugin.audiobridge").await?;
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let abort_handle = jatask::spawn(async move {
            while let Some(msg) = receiver.recv().await {
                let msg = Self::parse_audio_bridge_message(msg)?;
                let _ = tx.send(msg);
            }
            Ok::<(), JaError>(())
        });
        let mut handle: Self::Handle = handle.into();
        handle.assign_aborts(vec![abort_handle]);
        Ok((handle, rx))
    }
}

impl AudioBridge for JaSession {
    type Event = (AudioBridgePluginEvent, Option<EstablishmentProtocol>);
    type Handle = AudioBridgeHandle;

    fn parse_audio_bridge_message(message: JaResponse) -> JaResult<Self::Event> {
        let msg = match message.janus {
            JaResponseProtocol::Event(JaEventProtocol::Event { plugin_data }) => (
                serde_json::from_value::<AudioBridgePluginData>(plugin_data.data)?.event,
                message.establishment_protocol,
            ),
            _ => {
                tracing::error!("unexpected response");
                return Err(JaError::UnexpectedResponse);
            }
        };
        Ok(msg)
    }
}
