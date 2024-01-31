use async_trait::async_trait;
use jarust::prelude::*;
use jarust::transport::trans::Transport;
use tokio::sync::mpsc;

pub struct MockTransport {
    tx: mpsc::Sender<String>,
    rx: Option<mpsc::Receiver<String>>,
}

impl MockTransport {
    pub async fn mock_recv_msg(&self, msg: &str) {
        self.tx.send(msg.to_string()).await.unwrap();
    }
}

#[async_trait]
impl Transport for MockTransport {
    fn new() -> Self
    where
        Self: Sized,
    {
        let (tx, rx) = mpsc::channel(32);
        Self { tx, rx: Some(rx) }
    }

    async fn connect(&mut self, _: &str) -> JaResult<mpsc::Receiver<String>> {
        let (tx, rx) = mpsc::channel(32);

        if let Some(mut receiver) = self.rx.take() {
            tokio::spawn(async move {
                while let Some(msg) = receiver.recv().await {
                    tx.send(msg).await.unwrap();
                }
            });
        };

        Ok(rx)
    }

    async fn send(&mut self, _: &[u8]) -> JaResult<()> {
        Ok(())
    }
}

impl Drop for MockTransport {
    fn drop(&mut self) {}
}
