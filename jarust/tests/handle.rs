mod mocks;

use crate::mocks::mock_transport::MockTransport;
use jarust::jaconfig::JaConfig;
use jarust::japlugin::Attach;
use jarust::japrotocol::GenericEvent;
use jarust::japrotocol::JaData;
use jarust::japrotocol::JaHandleEvent;
use jarust::japrotocol::JaResponse;
use jarust::japrotocol::JaSuccessProtocol;
use jarust::japrotocol::ResponseType;

#[tokio::test]
async fn it_receives_incoming_handle_events() {
    let config = JaConfig::builder()
        .url("mock://some.janus.com")
        .namespace("mock")
        .build();
    let (transport, server) = MockTransport::transport_server_pair();

    let creation_msg = serde_json::to_string(&JaResponse {
        janus: ResponseType::Success(JaSuccessProtocol::Data {
            data: JaData { id: 2 },
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

    server.mock_send_to_client(&creation_msg).await;
    let session = connection.create(10).await.unwrap();

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
    let (_handle, mut stream) = session.attach("mock.plugin.test").await.unwrap();

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
