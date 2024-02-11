use async_trait::async_trait;
use jarust::jatask;
use jarust::jatask::AbortHandle;
use jarust::prelude::*;
use jarust::transport::trans::Transport;
use tokio::sync::mpsc;

pub struct MockServer {
    tx: mpsc::Sender<String>,
}

impl MockServer {
    pub async fn mock_send_to_client(&self, msg: &str) {
        self.tx.send(msg.to_string()).await.unwrap();
    }
}

pub struct MockTransport {
    rx: Option<mpsc::Receiver<String>>,
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
    fn new() -> Self
    where
        Self: Sized,
    {
        let (tx, rx) = mpsc::channel(32);
        Self {
            rx: Some(rx),
            server: Some(MockServer { tx }),
            abort_handle: None,
        }
    }

    async fn connect(&mut self, _: &str) -> JaResult<mpsc::Receiver<String>> {
        let (tx, rx) = mpsc::channel(32);

        if let Some(mut receiver) = self.rx.take() {
            let abort_handle = jatask::spawn(async move {
                while let Some(msg) = receiver.recv().await {
                    tx.send(msg).await.unwrap();
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
