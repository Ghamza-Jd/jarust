use super::mock_transport::MockServer;
use super::mock_transport::MockTransport;
use jarust::jaconfig::JaConfig;
use jarust::jaconnection::JaConnection;
use jarust::prelude::JaResult;

pub struct MockConnectionConfig {
    pub url: String,
    pub namespace: String,
}

#[allow(dead_code)]
pub async fn mock_connection(config: MockConnectionConfig) -> JaResult<(JaConnection, MockServer)> {
    let config = JaConfig::builder()
        .url(&config.url)
        .namespace(&config.namespace)
        .build();
    let (transport, server) = MockTransport::transport_server_pair();

    let connection = jarust::connect_with_transport(config, transport).await?;

    Ok((connection, server))
}
