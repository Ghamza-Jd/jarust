use crate::prelude::*;
use async_trait::async_trait;
use std::fmt::Debug;
use tokio::sync::mpsc;

pub type MessageStream = mpsc::Receiver<String>;

#[async_trait]
pub trait Transport: Debug + Send + Sync + 'static {
    /// Creates a new transport
    fn create_transport() -> Self
    where
        Self: Sized;

    /// Connect the transport with the server. Returns a channel receiver.
    async fn connect(&mut self, uri: &str) -> JaResult<MessageStream>;

    /// Send a message over the transport.
    async fn send(&mut self, data: &[u8]) -> JaResult<()>;
}

pub struct TransportProtocol(Box<dyn Transport + Send + Sync>);

impl TransportProtocol {
    pub async fn connect(
        mut transport: impl Transport,
        uri: &str,
    ) -> JaResult<(Self, MessageStream)> {
        let rx = transport.connect(uri).await?;
        let transport = Self(Box::new(transport));
        Ok((transport, rx))
    }

    pub async fn send(&mut self, data: &[u8]) -> JaResult<()> {
        self.0.send(data).await
    }
}

impl Debug for TransportProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TransportProtocol").finish()
    }
}
