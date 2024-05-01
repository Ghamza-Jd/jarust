use super::events::EchoTestPluginEvent;
use super::handle::EchoTestHandle;
use jarust::japrotocol::JaEventProtocol;
use jarust::japrotocol::JaResponseProtocol;
use jarust::prelude::*;
use tokio::sync::mpsc;

#[async_trait::async_trait]
pub trait EchoTest: Attach {
    type Event: Send + Sync + 'static;
    type Handle: From<JaHandle> + std::ops::Deref<Target = JaHandle> + PluginTask;

    fn parse_echotest_message(message: JaResponse) -> JaResult<Self::Event>;

    async fn attach_echotest(
        &self,
    ) -> JaResult<(Self::Handle, mpsc::UnboundedReceiver<Self::Event>)> {
        let (handle, mut receiver) = self.attach("janus.plugin.echotest").await?;
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let abort_handle = jatask::spawn(async move {
            while let Some(msg) = receiver.recv().await {
                let msg = Self::parse_echotest_message(msg)?;
                let _ = tx.send(msg);
            }
            Ok::<(), JaError>(())
        });
        let mut handle: Self::Handle = handle.into();
        handle.assign_aborts(vec![abort_handle]);
        Ok((handle, rx))
    }
}

impl EchoTest for JaSession {
    type Event = EchoTestPluginEvent;
    type Handle = EchoTestHandle;

    fn parse_echotest_message(message: JaResponse) -> JaResult<Self::Event> {
        let msg = match message.janus {
            JaResponseProtocol::Event(JaEventProtocol::Event { plugin_data, .. }) => {
                serde_json::from_value::<EchoTestPluginEvent>(plugin_data.data)?
            }
            _ => {
                tracing::error!("unexpected response {message:#?}");
                return Err(JaError::UnexpectedResponse);
            }
        };
        Ok(msg)
    }
}
