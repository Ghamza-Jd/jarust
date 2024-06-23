use super::mock_transport::MockTransport;
use crate::mocks::mock_generate_transaction::MockGenerateTransaction;
use jarust::jaconfig::JaConfig;
use jarust::jaconnection::JaConnection;
use jarust::prelude::JaResult;

pub struct MockConnectionConfig {
    pub url: String,
    pub namespace: String,
    pub capacity: usize,
}

#[allow(dead_code)]
pub async fn mock_connection(
    transport: MockTransport,
    config: MockConnectionConfig,
    generator: MockGenerateTransaction,
) -> JaResult<JaConnection> {
    let config = JaConfig::builder()
        .url(&config.url)
        .namespace(&config.namespace)
        .capacity(config.capacity)
        .build();

    let connection = jarust::custom_connect(config, transport, generator).await?;

    Ok(connection)
}
