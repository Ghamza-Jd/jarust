use async_trait::async_trait;
use jarust::jaconfig::{JaConfig, TransportType};
use jarust::prelude::*;
use jarust::transport::trans::Transport;
use tokio::sync::mpsc;

pub struct MockTransport;

#[async_trait]
impl Transport for MockTransport {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }

    async fn connect(&mut self, _uri: &str) -> JaResult<mpsc::Receiver<String>> {
        let (tx, rx) = mpsc::channel(32);
        tokio::spawn(async move {
            tx.send("Hello".to_string()).await.unwrap();
        });
        Ok(rx)
    }

    async fn send(&mut self, _data: &[u8]) -> JaResult<()> {
        Ok(())
    }
}

#[tokio::test]
async fn test_connection() {
    let config = JaConfig::new(
        "wss://janus.conf.meetecho.com/ws",
        None,
        TransportType::Wss,
        "janus",
    );
    let transport = MockTransport::new();
    jarust::connect_with_transport(config, transport)
        .await
        .unwrap();
}
