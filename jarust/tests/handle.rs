mod fixtures;
mod mocks;

#[cfg(test)]
mod tests {
    use crate::mocks::mock_generate_transaction::MockGenerateTransaction;
    use crate::mocks::mock_interface::MockInterface;
    use jarust::core::custom_connect;
    use jarust::core::prelude::Attach;
    use jarust::core::prelude::JaResponse;
    use jarust::interface::janus_interface::ConnectionParams;
    use jarust::interface::janus_interface::JanusInterface;
    use jarust::interface::japrotocol::GenericEvent;
    use jarust::interface::japrotocol::JaData;
    use jarust::interface::japrotocol::JaHandleEvent;
    use jarust::interface::japrotocol::JaSuccessProtocol;
    use jarust::interface::japrotocol::ResponseType;
    use std::time::Duration;

    #[tokio::test]
    async fn it_receives_incoming_handle_events() {
        let conn_params = ConnectionParams {
            url: "mock://some.janus.com".to_string(),
            capacity: 10,
            apisecret: None,
            server_root: "mock".to_string(),
        };
        let transaction_generator = MockGenerateTransaction::new();
        let interface = MockInterface::make_interface(conn_params, transaction_generator)
            .await
            .unwrap();
        let mut connection = custom_connect(interface.clone()).await.unwrap();

        let session_id = 73;
        let handle_id = 77;

        let response = JaResponse {
            janus: ResponseType::Success(JaSuccessProtocol::Data {
                data: JaData { id: session_id },
            }),
            transaction: Some("abc123".to_string()),
            session_id: None,
            sender: None,
            estproto: None,
        };
        interface.mock_create_rsp(response).await;

        let session = connection
            .create_session(10, Duration::from_secs(10))
            .await
            .unwrap();

        let response = JaResponse {
            janus: ResponseType::Success(JaSuccessProtocol::Data {
                data: JaData { id: 77 },
            }),
            transaction: Some("mock-attach-plugin-transaction".to_string()),
            session_id: Some(session_id),
            sender: None,
            estproto: None,
        };
        interface.mock_attach_rsp(response).await;

        let (_handle, mut stream) = session
            .attach("mock.plugin.test".to_string(), Duration::from_secs(5))
            .await
            .unwrap();
        interface
            .mock_event(
                77,
                JaResponse {
                    janus: ResponseType::Event(JaHandleEvent::GenericEvent(GenericEvent::Detached)),
                    transaction: Some("mock-event-transaction".to_string()),
                    session_id: Some(session_id),
                    sender: Some(handle_id),
                    estproto: None,
                },
            )
            .await;

        let incoming_event = stream.recv().await.unwrap();
        assert_eq!(
            incoming_event.janus,
            ResponseType::Event(JaHandleEvent::GenericEvent(GenericEvent::Detached))
        );
    }
}
