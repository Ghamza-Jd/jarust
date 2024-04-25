use async_trait::async_trait;
use jarust::jatask;
use jarust::jatask::AbortHandle;
use jarust::prelude::*;
use jarust::transport::trans::Transport;
use std::fmt::Debug;
use tokio::sync::mpsc;

pub struct MockServer {
    tx: mpsc::UnboundedSender<String>,
}

impl MockServer {
    pub async fn mock_send_to_client(&self, msg: &str) {
        self.tx.send(msg.to_string()).unwrap();
    }
}

pub struct MockTransport {
    rx: Option<MessageStream>,
    server: Option<MockServer>,
    abort_handle: Option<AbortHandle>,
}

impl MockTransport {
    pub fn get_mock_server(&mut self) -> Option<MockServer> {
        self.server.take()
    }
}

#[async_trait]
impl Transport for MockTransport {
    fn create_transport() -> Self
    where
        Self: Sized,
    {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            rx: Some(rx),
            server: Some(MockServer { tx }),
            abort_handle: None,
        }
    }

    async fn connect(&mut self, _: &str) -> JaResult<MessageStream> {
        let (tx, rx) = mpsc::unbounded_channel();

        if let Some(mut receiver) = self.rx.take() {
            let abort_handle = jatask::spawn(async move {
                while let Some(msg) = receiver.recv().await {
                    tx.send(msg).unwrap();
                }
            });
            self.abort_handle = Some(abort_handle);
        };

        Ok(rx)
    }

    async fn send(&mut self, _: &[u8]) -> JaResult<()> {
        Ok(())
    }
}

impl Drop for MockTransport {
    fn drop(&mut self) {
        if let Some(abort_handle) = self.abort_handle.take() {
            abort_handle.abort();
        }
    }
}

impl Debug for MockTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Mock").finish()
    }
}
