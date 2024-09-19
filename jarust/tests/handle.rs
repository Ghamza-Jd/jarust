mod fixtures;
mod mocks;

#[cfg(test)]
mod tests {
    use crate::fixtures::FIXTURE_HANDLE_ID;
    use crate::fixtures::FIXTURE_KA_INTERVAL;
    use crate::fixtures::FIXTURE_PLUGIN_ID;
    use crate::fixtures::FIXTURE_SESSION_ID;
    use crate::fixtures::FIXTURE_TIMEOUT;
    use crate::mocks::mock_connection::mock_connection;
    use crate::mocks::mock_generate_transaction::MockGenerateTransaction;
    use crate::mocks::mock_handle::mock_handle;
    use crate::mocks::mock_handle::MockHandleConfig;
    use crate::mocks::mock_interface::MockInterface;
    use crate::mocks::mock_session::mock_session;
    use crate::mocks::mock_session::MockSessionConfig;
    use jarust_transport::japrotocol::GenericEvent;
    use jarust_transport::japrotocol::JaHandleEvent;
    use jarust_transport::japrotocol::JaResponse;
    use jarust_transport::japrotocol::ResponseType;

    #[tokio::test]
    async fn it_receives_incoming_handle_events() {
        let (interface, server) = MockInterface::interface_server_pair().await.unwrap();
        let mut generator = MockGenerateTransaction::new();
        generator.next_transaction("mock-connection-transaction");
        let connection = mock_connection(interface).await.unwrap();

        generator.next_transaction("mock-session-transaction");
        let session = mock_session(
            connection,
            &server,
            MockSessionConfig {
                session_id: FIXTURE_SESSION_ID,
                ka_interval: FIXTURE_KA_INTERVAL,
                timeout: FIXTURE_TIMEOUT,
            },
            "mock-session-transaction",
        )
        .await
        .unwrap();

        generator.next_transaction("mock-handle-transaction");
        let (_handle, mut stream) = mock_handle(
            session,
            &server,
            MockHandleConfig {
                session_id: FIXTURE_SESSION_ID,
                handle_id: FIXTURE_HANDLE_ID,
                plugin_id: FIXTURE_PLUGIN_ID.to_string(),
                timeout: FIXTURE_TIMEOUT,
            },
            "mock-handle-transaction",
        )
        .await
        .unwrap();

        let event = serde_json::to_string(&JaResponse {
            janus: ResponseType::Event(JaHandleEvent::GenericEvent(GenericEvent::Detached)),
            transaction: Some("mock-event-transaction".to_string()),
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
}
