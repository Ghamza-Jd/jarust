mod mocks;

use crate::mocks::mock_transport::MockTransport;
use jarust::error::JaError;
use jarust::jaconfig::JaConfig;
use jarust::japrotocol::JaData;
use jarust::japrotocol::JaResponse;
use jarust::japrotocol::JaResponseError;
use jarust::japrotocol::JaResponseProtocol;
use jarust::japrotocol::JaSuccessProtocol;
use jarust::transport::trans::Transport;

#[tokio::test]
async fn test_connection() {
    let config = JaConfig::builder()
        .url("mock://some.janus.com")
        .namespace("mock")
        .build();
    let transport = MockTransport::create_transport();
    let connection = jarust::connect_with_transport(config, transport).await;
    assert!(connection.is_ok());
}

#[tokio::test]
async fn test_session_creation_success() {
    let config = JaConfig::builder()
        .url("mock://some.janus.com")
        .namespace("mock")
        .build();
    let mut transport = MockTransport::create_transport();
    let server = transport.get_mock_server().unwrap();

    let msg = serde_json::to_string(&JaResponse {
        janus: JaResponseProtocol::Success(JaSuccessProtocol::Data {
            data: JaData { id: 0 },
        }),
        transaction: None,
        session_id: None,
        sender: None,
        establishment_protocol: None,
    })
    .unwrap();

    let mut connection = jarust::connect_with_transport(config, transport)
        .await
        .unwrap();

    server.mock_send_to_client(&msg).await;
    let session = connection.create(10).await;

    assert!(session.is_ok());
}

#[tokio::test]
async fn test_session_creation_failure() {
    let config = JaConfig::builder()
        .url("mock://some.janus.com")
        .namespace("mock")
        .build();
    let mut transport = MockTransport::create_transport();
    let server = transport.get_mock_server().unwrap();

    let msg = serde_json::to_string(&JaResponse {
        janus: JaResponseProtocol::Error {
            error: JaResponseError {
                code: 0,
                reason: "".to_string(),
            },
        },
        transaction: None,
        session_id: None,
        sender: None,
        establishment_protocol: None,
    })
    .unwrap();

    let mut connection = jarust::connect_with_transport(config, transport)
        .await
        .unwrap();

    server.mock_send_to_client(&msg).await;
    let session = connection.create(10).await;

    assert!(matches!(session.unwrap_err(), JaError::JanusError { .. }))
}
