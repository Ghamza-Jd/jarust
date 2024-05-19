use super::mock_transport::MockServer;
use jarust::jaconnection::JaConnection;
use jarust::japrotocol::JaData;
use jarust::japrotocol::JaSuccessProtocol;
use jarust::japrotocol::ResponseType;
use jarust::prelude::*;

pub struct MockSessionConfig {
    pub session_id: u64,
    pub ka_interval: u32,
}

#[allow(dead_code)]
pub async fn mock_session(
    mut connection: JaConnection,
    server: &MockServer,
    config: MockSessionConfig,
) -> JaResult<JaSession> {
    let msg = serde_json::to_string(&JaResponse {
        janus: ResponseType::Success(JaSuccessProtocol::Data {
            data: JaData {
                id: config.session_id,
            },
        }),
        transaction: None,
        session_id: None,
        sender: None,
        establishment_protocol: None,
    })
    .unwrap();

    server.mock_send_to_client(&msg).await;
    let session = connection.create(config.ka_interval).await?;
    Ok(session)
}