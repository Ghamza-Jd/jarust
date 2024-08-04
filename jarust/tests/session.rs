mod fixtures;
mod mocks;

#[cfg(test)]
mod tests {
    use crate::fixtures::FIXTURE_CAPACITY;
    use crate::fixtures::FIXTURE_KA_INTERVAL;
    use crate::fixtures::FIXTURE_NAMESPACE;
    use crate::fixtures::FIXTURE_SESSION_ID;
    use crate::fixtures::FIXTURE_TIMEOUT;
    use crate::fixtures::FIXTURE_URL;
    use crate::mocks::mock_connection::mock_connection;
    use crate::mocks::mock_connection::MockConnectionConfig;
    use crate::mocks::mock_generate_transaction::MockGenerateTransaction;
    use crate::mocks::mock_session::mock_session;
    use crate::mocks::mock_session::MockSessionConfig;
    use crate::mocks::mock_transport::MockTransport;
    use jarust::error::JaError;
    use jarust::japlugin::Attach;
    use jarust::params::AttachHandleParams;
    use jarust_transport_next::japrotocol::ErrorResponse;
    use jarust_transport_next::japrotocol::JaData;
    use jarust_transport_next::japrotocol::JaResponse;
    use jarust_transport_next::japrotocol::JaSuccessProtocol;
    use jarust_transport_next::japrotocol::ResponseType;

    #[tokio::test]
    async fn it_successfully_attach_to_handle() {
        let (transport, server) = MockTransport::transport_server_pair().unwrap();
        let mut generator = MockGenerateTransaction::new();
        generator.next_transaction("mock-transaction");
        let connection = mock_connection(
            transport,
            MockConnectionConfig {
                url: FIXTURE_URL.to_string(),
                namespace: FIXTURE_NAMESPACE.to_string(),
                capacity: FIXTURE_CAPACITY,
            },
            generator.clone(),
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
                capacity: FIXTURE_CAPACITY,
            },
            "mock-transaction",
        )
        .await
        .unwrap();

        let attachment_msg = serde_json::to_string(&JaResponse {
            janus: ResponseType::Success(JaSuccessProtocol::Data {
                data: JaData { id: 3 },
            }),
            transaction: Some("mock-attach-plugin-transaction".to_string()),
            session_id: Some(FIXTURE_SESSION_ID),
            sender: None,
            establishment_protocol: None,
        })
        .unwrap();
        server.mock_send_to_client(&attachment_msg).await;

        generator.next_transaction("mock-attach-plugin-transaction");
        // no need for assertion, if unwrap fails the test will fail
        let _ = session
            .attach(AttachHandleParams {
                plugin_id: "mock.plugin.test".to_string(),
                timeout: FIXTURE_TIMEOUT,
            })
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn it_fails_to_attach_session() {
        let (transport, server) = MockTransport::transport_server_pair().unwrap();
        let mut generator = MockGenerateTransaction::new();
        generator.next_transaction("mock-transaction");
        let connection = mock_connection(
            transport,
            MockConnectionConfig {
                url: FIXTURE_URL.to_string(),
                namespace: FIXTURE_NAMESPACE.to_string(),
                capacity: FIXTURE_CAPACITY,
            },
            generator.clone(),
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
                capacity: FIXTURE_CAPACITY,
            },
            "mock-transaction",
        )
        .await
        .unwrap();

        let error = serde_json::to_string(&JaResponse {
            janus: ResponseType::Error {
                error: ErrorResponse {
                    code: 0,
                    reason: "".to_string(),
                },
            },
            transaction: Some("mock-attach-plugin-transaction".to_string()),
            session_id: Some(FIXTURE_SESSION_ID),
            sender: None,
            establishment_protocol: None,
        })
        .unwrap();

        server.mock_send_to_client(&error).await;
        generator.next_transaction("mock-attach-plugin-transaction");
        let result = session
            .attach(AttachHandleParams {
                plugin_id: "mock.plugin.test".to_string(),
                timeout: FIXTURE_TIMEOUT,
            })
            .await;
        assert!(matches!(
            result,
            Err(JaError::JanusError { code: _, reason: _ })
        ));
    }
}
