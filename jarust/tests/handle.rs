mod fixtures;
mod mocks;

use crate::fixtures::FIXTURE_HANDLE_ID;
use crate::fixtures::FIXTURE_KA_INTERVAL;
use crate::fixtures::FIXTURE_NAMESPACE;
use crate::fixtures::FIXTURE_PLUGIN_ID;
use crate::fixtures::FIXTURE_SESSION_ID;
use crate::fixtures::FIXTURE_URL;
use crate::mocks::mock_connection::mock_connection;
use crate::mocks::mock_connection::MockConnectionConfig;
use crate::mocks::mock_handle::mock_handle;
use crate::mocks::mock_handle::MockHandleConfig;
use crate::mocks::mock_session::mock_session;
use crate::mocks::mock_session::MockSessionConfig;
use crate::mocks::mock_transport::MockTransport;
use fixtures::FIXTURE_TIMEOUT;
use jarust::japrotocol::GenericEvent;
use jarust::japrotocol::JaHandleEvent;
use jarust::japrotocol::JaResponse;
use jarust::japrotocol::ResponseType;

#[tokio::test]
async fn it_receives_incoming_handle_events() {
    let (transport, server) = MockTransport::transport_server_pair().unwrap();
    let connection = mock_connection(
        transport,
        MockConnectionConfig {
            url: FIXTURE_URL.to_string(),
            namespace: FIXTURE_NAMESPACE.to_string(),
        },
    )
    .await
    .unwrap();
    let session = mock_session(
        connection,
        &server,
        MockSessionConfig {
            session_id: FIXTURE_SESSION_ID,
            ka_interval: FIXTURE_KA_INTERVAL,
            timeout: FIXTURE_TIMEOUT,
        },
    )
    .await
    .unwrap();
    let (_handle, mut stream) = mock_handle(
        session,
        &server,
        MockHandleConfig {
            session_id: FIXTURE_SESSION_ID,
            handle_id: FIXTURE_HANDLE_ID,
            plugin_id: FIXTURE_PLUGIN_ID.to_string(),
        },
    )
    .await
    .unwrap();

    let event = serde_json::to_string(&JaResponse {
        janus: ResponseType::Event(JaHandleEvent::GenericEvent(GenericEvent::Detached)),
        transaction: None,
        session_id: Some(FIXTURE_SESSION_ID),
        sender: Some(FIXTURE_HANDLE_ID),
        establishment_protocol: None,
    })
    .unwrap();
    server.mock_send_to_client(&event).await;

    let incoming_event = stream.recv().await.unwrap();

    assert_eq!(
        incoming_event.janus,
        ResponseType::Event(JaHandleEvent::GenericEvent(GenericEvent::Detached))
    );
}
