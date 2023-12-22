use super::wss::WebsocketTransport;
use crate::prelude::*;
use async_trait::async_trait;
use tokio::sync::mpsc;

#[async_trait]
pub trait Transport {
    async fn connect(uri: &str) -> JaResult<(Box<Self>, mpsc::Receiver<String>)>
    where
        Self: Sized;
    async fn send(&mut self, data: &[u8]) -> JaResult<()>;
}

pub enum TransportProtocol {
    Wss(WebsocketTransport),
}

impl TransportProtocol {
    #[must_use]
    pub fn as_transport_mut(&mut self) -> &mut (dyn Transport + Send + Sync) {
        match self {
            Self::Wss(transport) => transport,
        }
    }
}
