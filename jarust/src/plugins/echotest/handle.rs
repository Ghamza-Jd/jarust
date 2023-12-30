use super::events::EchoTestPluginData;
use super::messages::EchoTestStartMsg;
use crate::jaconfig::CHANNEL_BUFFER_SIZE;
use crate::jahandle::JaHandle;
use crate::japrotocol::JaEventProtocol;
use crate::japrotocol::JaResponseProtocol;
use crate::jasession::JaSession;
use crate::prelude::*;
use async_trait::async_trait;
use tokio::sync::mpsc;

const PLUGIN_ID: &str = "janus.plugin.echotest";

#[async_trait]
pub trait EchoTest {
    async fn attach_echotest(
        &self,
    ) -> JaResult<(EchoTestHandle, mpsc::Receiver<EchoTestPluginData>)>;
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
    async fn attach_echotest(
        &self,
    ) -> JaResult<(EchoTestHandle, mpsc::Receiver<EchoTestPluginData>)> {
        let (handle, mut receiver) = self.attach(PLUGIN_ID).await?;
        let (tx, rx) = mpsc::channel(CHANNEL_BUFFER_SIZE);
        tokio::spawn(async move {
            while let Some(msg) = receiver.recv().await {
                let msg = match msg.janus {
                    JaResponseProtocol::Event(JaEventProtocol::Event { plugin_data, .. }) => {
                        serde_json::from_value::<EchoTestPluginData>(plugin_data).unwrap()
                    }
                    _ => continue,
                };
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
