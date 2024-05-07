use crate::prelude::*;
use async_trait::async_trait;
use std::fmt::Debug;
use tokio::sync::mpsc;

pub type MessageStream = mpsc::UnboundedReceiver<String>;

#[async_trait]
pub trait TransportProtocol: Debug + Send + Sync + 'static {
    /// Creates a new transport
    fn create_transport() -> Self
    where
        Self: Sized;

    /// Connect the transport with the server. Returns a channel receiver.
    async fn connect(&mut self, uri: &str) -> JaResult<MessageStream>;

    /// Send a message over the transport.
    async fn send(&mut self, data: &[u8]) -> JaResult<()>;
}

pub struct TransportSession(Box<dyn TransportProtocol + Send + Sync>);

impl TransportSession {
    pub async fn connect(
        mut transport: impl TransportProtocol,
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

impl Debug for TransportSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TransportProtocol").finish()
    }
}
