use async_trait::async_trait;
use jarust::jaconfig::JaConfig;
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

    async fn connect(&mut self, _: &str) -> JaResult<mpsc::Receiver<String>> {
        let (_, rx) = mpsc::channel(32);
        Ok(rx)
    }

    async fn send(&mut self, _: &[u8]) -> JaResult<()> {
        Ok(())
    }
}

#[tokio::test]
async fn test_connection() {
    let config = JaConfig::new("mock://some.janus.com", None, "janus");
    let transport = MockTransport::new();
    jarust::connect_with_transport(config, transport)
        .await
        .unwrap();
}
