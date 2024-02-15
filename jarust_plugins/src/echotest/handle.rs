use super::messages::EchoTestStartMsg;
use jarust::japrotocol::{EstablishmentProtocol, JsepType};
use jarust::jatask::AbortHandle;
use jarust::prelude::*;
use std::ops::Deref;
use std::time::Duration;

pub struct EchoTestHandle {
    handle: JaHandle,
    abort_handles: Option<Vec<AbortHandle>>,
}

impl EchoTestHandle {
    pub async fn start(&self, mut request: EchoTestStartMsg, timeout: Duration) -> JaResult<()> {
        let Some(jsep) = request.jsep.take() else {
            return self.handle.message(serde_json::to_value(request)?).await;
        };
        if jsep.jsep_type != JsepType::Offer {
            let err = JaError::InvalidJanusRequest {
                reason: "jsep must be an offer".to_owned(),
            };
            tracing::error!("{err}");
            return Err(err);
        }
        self.handle
            .message_with_establishment_protocol(
                serde_json::to_value(request)?,
                EstablishmentProtocol::JSEP(jsep),
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
