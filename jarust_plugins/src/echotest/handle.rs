use super::messages::EchoTestStartMsg;
use jarust::jatask::AbortHandle;
use jarust::prelude::*;
use std::ops::Deref;

pub struct EchoTestHandle {
    handle: JaHandle,
    abort_handles: Option<Vec<AbortHandle>>,
}

impl EchoTestHandle {
    pub async fn start(&self, request: EchoTestStartMsg) -> JaResult<()> {
        self.handle
            .fire_and_forget(serde_json::to_value(request)?)
            .await
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
