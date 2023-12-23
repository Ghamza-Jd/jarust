use crate::prelude::*;
use async_trait::async_trait;
use tokio::sync::mpsc;

#[async_trait]
pub trait Transport: Send + Sync + 'static {
    fn new() -> Self
    where
        Self: Sized;
    async fn connect(&mut self, uri: &str) -> JaResult<mpsc::Receiver<String>>;
    async fn send(&mut self, data: &[u8]) -> JaResult<()>;
}

pub struct TransportProtocol(Box<dyn Transport + Send + Sync>);

impl TransportProtocol {
    pub async fn connect(
        mut transport: impl Transport,
        uri: &str,
    ) -> JaResult<(Self, mpsc::Receiver<String>)> {
        let rx = transport.connect(uri).await?;
        let transport = Self(Box::new(transport));
        Ok((transport, rx))
    }

    pub async fn send(&mut self, data: &[u8]) -> JaResult<()> {
        self.0.send(data).await
    }
}
