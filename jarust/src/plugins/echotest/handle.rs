use super::messages::EchoTestStartMsg;
use crate::jahandle::JaHandle;
use crate::jasession::JaSession;
use crate::prelude::*;
use async_trait::async_trait;
use tokio::sync::mpsc;

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
        let (handle, mut receiver) = self.attach("janus.plugin.echotest").await?;
        let (tx, rx) = mpsc::channel(100);
        tokio::spawn(async move {
            while let Some(msg) = receiver.recv().await {
                let msg = format!("Todo: parse properly {msg:?}");
                let _ = tx.send(msg).await;
            }
        });
        Ok((handle.into(), rx))
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
