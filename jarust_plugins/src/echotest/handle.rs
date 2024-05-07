use super::messages::StartMsg;
use jarust::japrotocol::EstablishmentProtocol;
use jarust::prelude::*;
use jarust_rt::AbortHandle;
use std::ops::Deref;
use std::time::Duration;

pub struct EchoTestHandle {
    handle: JaHandle,
    abort_handles: Option<Vec<AbortHandle>>,
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

impl From<JaHandle> for EchoTestHandle {
    fn from(handle: JaHandle) -> Self {
        Self {
            handle,
            abort_handles: None,
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
        self.abort_plugin();
    }
}

impl Clone for EchoTestHandle {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
            abort_handles: None,
        }
    }
}
