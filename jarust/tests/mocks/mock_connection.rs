use super::mock_transport::MockTransport;
use jarust::jaconfig::JaConfig;
use jarust::jaconnection::JaConnection;
use jarust::prelude::JaResult;
use crate::mocks::mock_generate_transaction::MockGenerateTransaction;

pub struct MockConnectionConfig {
    pub url: String,
    pub namespace: String,
}

#[allow(dead_code)]
pub async fn mock_connection(
    transport: MockTransport,
    config: MockConnectionConfig,
    generator: MockGenerateTransaction
) -> JaResult<JaConnection> {
    let config = JaConfig::builder()
        .url(&config.url)
        .namespace(&config.namespace)
        .build();

    let connection = jarust::connect_with_transport(config, transport, generator).await?;

    Ok(connection)
}
