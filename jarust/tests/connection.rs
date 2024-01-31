mod mocks;

use crate::mocks::mock_transport::MockTransport;
use jarust::jaconfig::JaConfig;
use jarust::transport::trans::Transport;

#[tokio::test]
async fn test_connection() {
    let config = JaConfig::new("mock://some.janus.com", None, "janus");
    let transport = MockTransport::new();
    jarust::connect_with_transport(config, transport)
        .await
        .unwrap();
}
