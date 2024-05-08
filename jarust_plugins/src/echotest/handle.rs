use super::messages::StartMsg;
use jarust::japrotocol::EstablishmentProtocol;
use jarust::prelude::*;
use jarust_rt::AbortHandle;
use std::ops::Deref;
use std::time::Duration;

pub struct EchoTestHandle {
    handle: JaHandle,
    cancellable: Option<AbortHandle>,
}

impl EchoTestHandle {
    pub async fn start(&self, request: StartMsg) -> JaResult<()> {
        self.handle
            .fire_and_forget(serde_json::to_value(request)?)
            .await
    }

    pub async fn start_with_establishment(
        &self,
        request: StartMsg,
        establishment: EstablishmentProtocol,
        timeout: Duration,
    ) -> JaResult<()> {
        self.send_waiton_ack_with_establishment(
            serde_json::to_value(request)?,
            establishment,
            timeout,
        )
        .await?;
        Ok(())
    }
}

impl PluginTask for EchoTestHandle {
    fn assign_cancellation(&mut self, cancellable: AbortHandle) {
        self.cancellable = Some(cancellable);
    }

    fn invoke_cancellation(&mut self) {
        if let Some(cancellation) = self.cancellable.take() {
            cancellation.abort();
        };
    }
}

impl From<JaHandle> for EchoTestHandle {
    fn from(handle: JaHandle) -> Self {
        Self {
            handle,
            cancellable: None,
        }
    }
}

impl Deref for EchoTestHandle {
    type Target = JaHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl Drop for EchoTestHandle {
    fn drop(&mut self) {
        self.invoke_cancellation();
    }
}

impl Clone for EchoTestHandle {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
            cancellable: None,
        }
    }
}
