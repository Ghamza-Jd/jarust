use async_trait::async_trait;
use jarust_rt::JaTask;
use jarust_transport::prelude::JaTransportResult;
use jarust_transport::trans::MessageStream;
use jarust_transport::trans::TransportProtocol;
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
    task: Option<JaTask>,
}

impl MockTransport {
    pub fn get_mock_server(&mut self) -> Option<MockServer> {
        self.server.take()
    }
}

#[async_trait]
impl TransportProtocol for MockTransport {
    fn create_transport() -> Self
    where
        Self: Sized,
    {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            rx: Some(rx),
            server: Some(MockServer { tx }),
            task: None,
        }
    }

    async fn connect(&mut self, _: &str) -> JaTransportResult<MessageStream> {
        let (tx, rx) = mpsc::unbounded_channel();

        if let Some(mut receiver) = self.rx.take() {
            let taks = jarust_rt::spawn(async move {
                while let Some(msg) = receiver.recv().await {
                    tx.send(msg).unwrap();
                }
            });
            self.task = Some(taks);
        };

        Ok(rx)
    }

    async fn send(&mut self, _: &[u8]) -> JaTransportResult<()> {
        Ok(())
    }
}

impl Drop for MockTransport {
    fn drop(&mut self) {
        if let Some(task) = self.task.take() {
            task.cancel();
        }
    }
}

impl Debug for MockTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Mock").finish()
    }
}
