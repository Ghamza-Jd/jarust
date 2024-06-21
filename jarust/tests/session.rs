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
    use jarust::japlugin::Attach;
    use jarust::japrotocol::ErrorResponse;
    use jarust::japrotocol::JaData;
    use jarust::japrotocol::JaResponse;
    use jarust::japrotocol::JaSuccessProtocol;
    use jarust::japrotocol::ResponseType;

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
            generator,
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
            "mock-transaction",
        )
        .await
        .unwrap();

        let attachment_msg = serde_json::to_string(&JaResponse {
            janus: ResponseType::Success(JaSuccessProtocol::Data {
                data: JaData { id: 3 },
            }),
            transaction: None,
            session_id: Some(FIXTURE_SESSION_ID),
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
            generator,
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
            transaction: None,
            session_id: Some(FIXTURE_SESSION_ID),
            sender: None,
            establishment_protocol: None,
        })
        .unwrap();

        server.mock_send_to_client(&error).await;
        let result = session.attach("mock.plugin.test").await;
        assert!(result.is_err());
    }
}
