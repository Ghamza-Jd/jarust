use super::mock_transport::MockServer;
use jarust::jaconnection::CreateConnectionParams;
use jarust::jaconnection::JaConnection;
use jarust::prelude::*;
use jarust_transport::japrotocol::JaData;
use jarust_transport::japrotocol::JaSuccessProtocol;
use jarust_transport::japrotocol::ResponseType;
use std::time::Duration;

pub struct MockSessionConfig {
    pub session_id: u64,
    pub ka_interval: u32,
    pub capacity: usize,
    pub timeout: Duration,
}

#[allow(dead_code)]
pub async fn mock_session(
    mut connection: JaConnection,
    server: &MockServer,
    config: MockSessionConfig,
    expected_transaction: &str,
) -> JaResult<JaSession> {
    let msg = serde_json::to_string(&JaResponse {
        janus: ResponseType::Success(JaSuccessProtocol::Data {
            data: JaData {
                id: config.session_id,
            },
        }),
        transaction: Some(expected_transaction.to_string()),
        session_id: None,
        sender: None,
        establishment_protocol: None,
    })?;

    server.mock_send_to_client(&msg).await;
    let session = connection
        .create(CreateConnectionParams {
            ka_interval: config.ka_interval,
            capacity: config.capacity,
            timeout: config.timeout,
        })
        .await?;

    Ok(session)
}
