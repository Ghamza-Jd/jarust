use crate::prelude::*;
use async_trait::async_trait;
use bytes::Bytes;
use std::fmt::Debug;
use tokio::sync::mpsc;

#[async_trait]
pub trait TransportProtocol: Debug + Send + Sync + 'static {
    /// Creates a new transport
    fn create_transport() -> Self
    where
        Self: Sized;

    /// Connect the transport with the server. Returns a channel receiver.
    async fn connect(&mut self, url: &str) -> JaTransportResult<mpsc::UnboundedReceiver<Bytes>>;

    /// Send a message over the transport.
    async fn send(&mut self, data: &[u8]) -> JaTransportResult<()>;

    /// Read-only str for the debug trait
    fn name(&self) -> Box<str> {
        "TransportProtocol".to_string().into_boxed_str()
    }
}

pub struct TransportSession {
    inner: Box<dyn TransportProtocol>,
}

impl TransportSession {
    pub async fn connect(
        mut transport: impl TransportProtocol,
        url: &str,
    ) -> JaTransportResult<(Self, mpsc::UnboundedReceiver<Bytes>)> {
        let rx = transport.connect(url).await?;
        let transport = Self {
            inner: Box::new(transport),
        };
        Ok((transport, rx))
    }

    pub async fn send(&mut self, data: &[u8]) -> JaTransportResult<()> {
        self.inner.send(data).await
    }
}

impl Debug for TransportSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Transport")
            .field(&self.inner.name())
            .finish()
    }
}
