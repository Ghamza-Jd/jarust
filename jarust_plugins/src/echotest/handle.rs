use super::messages::EchoTestStartMsg;
use jarust::prelude::*;
use std::ops::Deref;
use tokio::task::AbortHandle;

pub struct EchoTestHandle {
    handle: JaHandle,
    abort_handle: Option<AbortHandle>,
}

impl EchoTestHandle {
    pub async fn start(&self, mut request: EchoTestStartMsg) -> JaResult<()> {
        match request.jsep.take() {
            Some(jsep) => {
                self.handle
                    .message_with_jsep(serde_json::to_value(request)?, jsep)
                    .await?;
                Ok(())
            }
            None => self.handle.message(serde_json::to_value(request)?).await,
        }
    }
}

impl PluginTask for EchoTestHandle {
    fn assign_abort(&mut self, abort_handle: AbortHandle) {
        self.abort_handle = Some(abort_handle);
    }

    fn abort_plugin(&mut self) {
        if let Some(abort_handle) = self.abort_handle.take() {
            abort_handle.abort();
        };
    }
}

impl From<JaHandle> for EchoTestHandle {
    fn from(handle: JaHandle) -> Self {
        Self {
            handle,
            abort_handle: None,
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
