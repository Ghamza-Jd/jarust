use crate::jahandle::JaHandle;
use crate::jasession::JaSession;
use crate::prelude::*;
use async_trait::async_trait;
use tokio::sync::mpsc;

use super::messages::EchoTestStartMsg;

#[async_trait]
pub trait EchoTest {
    async fn attach_echotest(&self) -> JaResult<(EchoTestHandle, mpsc::Receiver<String>)>;
}

pub struct EchoTestHandle {
    handle: JaHandle,
}

impl From<JaHandle> for EchoTestHandle {
    fn from(handle: JaHandle) -> Self {
        Self { handle }
    }
}

#[async_trait]
impl EchoTest for JaSession {
    async fn attach_echotest(&self) -> JaResult<(EchoTestHandle, mpsc::Receiver<String>)> {
        let (handle, receiver) = self.attach("janus.plugin.echotest").await?;
        // intercept the messages and serialize them to plugin specific messages
        Ok((handle.into(), receiver))
    }
}

impl EchoTestHandle {
    pub async fn start(&self, request: EchoTestStartMsg) -> JaResult<()> {
        self.handle.message(serde_json::to_value(request)?).await
    }
}

impl std::ops::Deref for EchoTestHandle {
    type Target = JaHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}
