mod mocks;

use crate::mocks::mock_transport::MockTransport;
use jarust::error::JaError;
use jarust::jaconfig::JaConfig;
use jarust::japrotocol::JaData;
use jarust::japrotocol::JaResponse;
use jarust::japrotocol::JaResponseError;
use jarust::japrotocol::JaResponseProtocol;
use jarust::transport::trans::Transport;

#[tokio::test]
async fn test_connection() {
    let config = JaConfig::new("mock://some.janus.com", None, "janus");
    let transport = MockTransport::new();
    let connection = jarust::connect_with_transport(config, transport).await;
    assert!(connection.is_ok());
}

#[tokio::test]
async fn test_session_creation_success() {
    let config = JaConfig::new("mock://some.janus.com", None, "janus");
    let transport = MockTransport::new();

    let msg = serde_json::to_string(&JaResponse {
        janus: JaResponseProtocol::Success {
            data: JaData { id: 0 },
        },
        transaction: None,
        session_id: None,
        sender: None,
    })
    .unwrap();
    transport.mock_recv_msg(&msg).await;

    let mut connection = jarust::connect_with_transport(config, transport)
        .await
        .unwrap();

    let session = connection.create(10).await;

    assert!(session.is_ok());
}

#[tokio::test]
async fn test_session_creation_failure() {
    let config = JaConfig::new("mock://some.janus.com", None, "janus");
    let transport = MockTransport::new();

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
    })
    .unwrap();
    transport.mock_recv_msg(&msg).await;

    let mut connection = jarust::connect_with_transport(config, transport)
        .await
        .unwrap();

    let session = connection.create(10).await;

    assert!(matches!(session.unwrap_err(), JaError::JanusError { .. }))
}
