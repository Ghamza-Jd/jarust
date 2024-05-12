mod mocks;

use crate::mocks::mock_transport::MockTransport;
use jarust::error::JaError;
use jarust::jaconfig::JaConfig;
use jarust::japrotocol::ErrorResponse;
use jarust::japrotocol::JaData;
use jarust::japrotocol::JaResponse;
use jarust::japrotocol::JaSuccessProtocol;
use jarust::japrotocol::ResponseType;
use jarust_transport::trans::TransportProtocol;

#[tokio::test]
async fn it_successfully_connects() {
    let config = JaConfig::builder()
        .url("mock://some.janus.com")
        .namespace("mock")
        .build();
    let transport = MockTransport::create_transport();
    let connection = jarust::connect_with_transport(config, transport).await;
    assert!(connection.is_ok());
}

#[tokio::test]
async fn it_successfully_creates_session() {
    let config = JaConfig::builder()
        .url("mock://some.janus.com")
        .namespace("mock")
        .build();
    let (transport, server) = MockTransport::transport_server_pair();

    let msg = serde_json::to_string(&JaResponse {
        janus: ResponseType::Success(JaSuccessProtocol::Data {
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
async fn it_fails_to_create_session_with_janus_error() {
    let config = JaConfig::builder()
        .url("mock://some.janus.com")
        .namespace("mock")
        .build();
    let (transport, server) = MockTransport::transport_server_pair();

    let msg = serde_json::to_string(&JaResponse {
        janus: ResponseType::Error {
            error: ErrorResponse {
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
