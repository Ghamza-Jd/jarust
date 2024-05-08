use super::events::PluginEvent;
use super::handle::EchoTestHandle;
use jarust::prelude::*;
use std::ops::Deref;
use tokio::sync::mpsc;

#[async_trait::async_trait]
pub trait EchoTest: Attach {
    type Event: TryFrom<JaResponse, Error = JaError> + Send + Sync + 'static;
    type Handle: From<JaHandle> + Deref<Target = JaHandle> + PluginTask;

    async fn attach_echotest(
        &self,
    ) -> JaResult<(Self::Handle, mpsc::UnboundedReceiver<Self::Event>)> {
        let (handle, mut receiver) = self.attach("janus.plugin.echotest").await?;
        let (tx, rx) = mpsc::unbounded_channel();
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

impl EchoTest for JaSession {
    type Event = PluginEvent;
    type Handle = EchoTestHandle;
}
