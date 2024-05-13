use super::trans::Transport;
use crate::prelude::*;
use async_trait::async_trait;
use bytes::Bytes;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct WasmWsTransport;

#[async_trait]
impl Transport for WasmWsTransport {
    fn create_transport() -> Self {
        Self
    }

    async fn connect(&mut self, _uri: &str) -> JaResult<mpsc::UnboundedReceiver<Bytes>> {
        tracing::error!("WASM support is WIP!");
        todo!("WASM support is WIP!")
    }

    async fn send(&mut self, _data: &[u8]) -> JaResult<()> {
        tracing::error!("WASM support is WIP!");
        todo!("WASM support is WIP!")
    }
}
