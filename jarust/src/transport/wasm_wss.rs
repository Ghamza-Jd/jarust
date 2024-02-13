use super::trans::Transport;
use crate::prelude::*;
use async_trait::async_trait;
use tokio::sync::mpsc;

pub struct WasmWsTransport;

#[async_trait]
impl Transport for WasmWsTransport {
    fn new() -> Self {
        Self
    }

    async fn connect(&mut self, uri: &str) -> JaResult<mpsc::Receiver<String>> {
        tracing::error!("WASM support is WIP!");
        todo!("WASM support is WIP!")
    }

    async fn send(&mut self, _data: &[u8]) -> JaResult<()> {
        tracing::error!("WASM support is WIP!");
        todo!("WASM support is WIP!")
    }
}
