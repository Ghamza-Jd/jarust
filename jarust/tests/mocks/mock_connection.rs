use super::mock_transport::MockServer;
use super::mock_transport::MockTransport;
use jarust::jaconfig::JaConfig;
use jarust::jaconnection::JaConnection;
use jarust::prelude::JaResult;

#[allow(dead_code)]
pub async fn mock_connection() -> JaResult<(JaConnection, MockServer)> {
    let config = JaConfig::builder()
        .url("mock://some.janus.com")
        .namespace("mock")
        .build();
    let (transport, server) = MockTransport::transport_server_pair();

    let connection = jarust::connect_with_transport(config, transport).await?;

    Ok((connection, server))
}
