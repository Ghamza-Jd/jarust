mod mocks;

use crate::mocks::mock_handle::mock_handle;
use jarust::japrotocol::GenericEvent;
use jarust::japrotocol::JaHandleEvent;
use jarust::japrotocol::JaResponse;
use jarust::japrotocol::ResponseType;

#[tokio::test]
async fn it_receives_incoming_handle_events() {
    let (_handle, mut stream, server) = mock_handle().await.unwrap();
    let event = serde_json::to_string(&JaResponse {
        janus: ResponseType::Event(JaHandleEvent::GenericEvent(GenericEvent::Detached)),
        transaction: None,
        session_id: Some(2),
        sender: Some(3),
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
