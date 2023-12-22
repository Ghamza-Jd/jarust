use crate::prelude::*;
use async_trait::async_trait;
use tokio::sync::mpsc;

#[async_trait]
pub trait Transport {
    async fn connect(&self) -> JaResult<(Box<dyn Transport>, mpsc::Receiver<String>)>;
    async fn send(&self, data: &[u8]) -> JaResult<()>;
}
