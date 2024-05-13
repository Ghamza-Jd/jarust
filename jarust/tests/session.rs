mod mocks;

use crate::mocks::mock_session::mock_session;
use jarust::japlugin::Attach;
use jarust::japrotocol::ErrorResponse;
use jarust::japrotocol::JaData;
use jarust::japrotocol::JaResponse;
use jarust::japrotocol::JaSuccessProtocol;
use jarust::japrotocol::ResponseType;

#[tokio::test]
async fn it_successfully_attach_to_handle() {
    let (session, server) = mock_session().await.unwrap();

    let attachment_msg = serde_json::to_string(&JaResponse {
        janus: ResponseType::Success(JaSuccessProtocol::Data {
            data: JaData { id: 3 },
        }),
        transaction: None,
        session_id: Some(2),
        sender: None,
        establishment_protocol: None,
    })
    .unwrap();
    server.mock_send_to_client(&attachment_msg).await;
    let result = session.attach("mock.plugin.test").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn it_fails_to_attach_session() {
    let (session, server) = mock_session().await.unwrap();

    let error = serde_json::to_string(&JaResponse {
        janus: ResponseType::Error {
            error: ErrorResponse {
                code: 0,
                reason: "".to_string(),
            },
        },
        transaction: None,
        session_id: Some(2),
        sender: None,
        establishment_protocol: None,
    })
    .unwrap();

    server.mock_send_to_client(&error).await;
    let result = session.attach("mock.plugin.test").await;
    assert!(result.is_err());
}
