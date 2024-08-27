use async_trait::async_trait;
use bytes::BufMut;
use bytes::Bytes;
use bytes::BytesMut;
use jarust::error::JaError;
use jarust::prelude::JaResult;
use jarust_rt::JaTask;
use jarust_transport_next::legacy::trans::TransportProtocol;
use jarust_transport_next::prelude::JaTransportResult;
use std::fmt::Debug;
use tokio::sync::mpsc;

pub struct MockServer {
    tx: mpsc::UnboundedSender<Bytes>,
}

impl MockServer {
    pub async fn mock_send_to_client(&self, msg: &str) {
        let mut bytes = BytesMut::new();
        bytes.put_slice(msg.as_bytes());
        self.tx.send(bytes.into()).unwrap();
    }
}

pub struct MockTransport {
    rx: Option<mpsc::UnboundedReceiver<Bytes>>,
    server: Option<MockServer>,
    task: Option<JaTask>,
}

impl MockTransport {
    pub fn get_mock_server(&mut self) -> Option<MockServer> {
        self.server.take()
    }

    pub fn transport_server_pair() -> JaResult<(Self, MockServer)> {
        let mut transport = Self::create_transport();
        match transport.get_mock_server() {
            Some(server) => Ok((transport, server)),
            None => Err(JaError::TransportNotOpened),
        }
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

    async fn connect(&mut self, _: &str) -> JaTransportResult<mpsc::UnboundedReceiver<Bytes>> {
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

    async fn send(&mut self, _: &[u8], _: &str) -> JaTransportResult<()> {
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
