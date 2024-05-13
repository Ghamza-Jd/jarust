use super::mock_transport::MockServer;
use crate::mocks::mock_connection::mock_connection;
use jarust::japrotocol::JaData;
use jarust::japrotocol::JaSuccessProtocol;
use jarust::japrotocol::ResponseType;
use jarust::prelude::*;

#[allow(dead_code)]
pub async fn mock_session() -> JaResult<(JaSession, MockServer)> {
    let (mut connection, server) = mock_connection().await?;

    let msg = serde_json::to_string(&JaResponse {
        janus: ResponseType::Success(JaSuccessProtocol::Data {
            data: JaData { id: 2 },
        }),
        transaction: None,
        session_id: None,
        sender: None,
        establishment_protocol: None,
    })
    .unwrap();

    server.mock_send_to_client(&msg).await;
    let session = connection.create(10).await?;
    Ok((session, server))
}
