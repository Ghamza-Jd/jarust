use super::events::Events;
use super::handle::EchoTestHandle;
use jarust::japrotocol::ResponseType;
use jarust::prelude::*;
use std::ops::Deref;
use tokio::sync::mpsc;

#[async_trait::async_trait]
pub trait EchoTest: Attach {
    type Event: Send + Sync + 'static;
    type Handle: From<JaHandle> + Deref<Target = JaHandle> + PluginTask;

    fn parse_echotest_event(message: JaResponse) -> JaResult<Self::Event>;

    async fn attach_echotest(
        &self,
    ) -> JaResult<(Self::Handle, mpsc::UnboundedReceiver<Self::Event>)> {
        let (handle, mut receiver) = self.attach("janus.plugin.echotest").await?;
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let abort_handle = jatask::spawn(async move {
            while let Some(rsp) = receiver.recv().await {
                if let Ok(event) = Self::parse_echotest_event(rsp) {
                    let _ = tx.send(event);
                };
            }
        });
        let mut handle: Self::Handle = handle.into();
        handle.assign_aborts(vec![abort_handle]);
        Ok((handle, rx))
    }
}

impl EchoTest for JaSession {
    type Event = Events;
    type Handle = EchoTestHandle;

    fn parse_echotest_event(message: JaResponse) -> JaResult<Self::Event> {
        let msg = match message.janus {
            ResponseType::Event(event) => event.try_into()?,
            _ => {
                tracing::error!("unexpected response {message:#?}");
                return Err(JaError::UnexpectedResponse);
            }
        };
        Ok(msg)
    }
}
