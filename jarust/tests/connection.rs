mod fixtures;
mod mocks;

#[cfg(test)]
mod tests {
    use crate::fixtures::FIXTURE_CAPACITY;
    use crate::fixtures::FIXTURE_NAMESPACE;
    use crate::fixtures::FIXTURE_URL;
    use crate::mocks::mock_connection::mock_connection;
    use crate::mocks::mock_connection::MockConnectionConfig;
    use crate::mocks::mock_generate_transaction::MockGenerateTransaction;
    use crate::mocks::mock_transport::MockTransport;
    use jarust::error::JaError;
    use jarust::jaconfig::JaConfig;
    use jarust::japrotocol::ErrorResponse;
    use jarust::japrotocol::JaData;
    use jarust::japrotocol::JaResponse;
    use jarust::japrotocol::JaSuccessProtocol;
    use jarust::japrotocol::ResponseType;
    use jarust_transport::trans::TransportProtocol;
    use std::time::Duration;

    #[tokio::test]
    async fn it_successfully_connects() {
        let config = JaConfig::builder()
            .url("mock://some.janus.com")
            .namespace("mock")
            .capacity(10)
            .build();
        let transport = MockTransport::create_transport();
        let generator = MockGenerateTransaction::new();
        let connection = jarust::custom_connect(config, transport, generator).await;
        assert!(connection.is_ok());
    }

    #[tokio::test]
    async fn it_successfully_creates_session() {
        let (transport, server) = MockTransport::transport_server_pair().unwrap();
        let mut generator = MockGenerateTransaction::new();
        generator.next_transaction("abc123");
        let mut connection = mock_connection(
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

        let msg = serde_json::to_string(&JaResponse {
            janus: ResponseType::Success(JaSuccessProtocol::Data {
                data: JaData { id: 2 },
            }),
            transaction: Some("abc123".to_string()),
            session_id: None,
            sender: None,
            establishment_protocol: None,
        })
        .unwrap();

        server.mock_send_to_client(&msg).await;
        let session = connection.create(10, 32, Duration::from_secs(10)).await;

        assert!(session.is_ok());
    }

    #[tokio::test]
    async fn it_fails_to_create_session_with_janus_error() {
        let (transport, server) = MockTransport::transport_server_pair().unwrap();
        let mut generator = MockGenerateTransaction::new();
        generator.next_transaction("abc123");
        let mut connection = mock_connection(
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

        let msg = serde_json::to_string(&JaResponse {
            janus: ResponseType::Error {
                error: ErrorResponse {
                    code: 0,
                    reason: "".to_string(),
                },
            },
            transaction: Some("abc123".to_string()),
            session_id: None,
            sender: None,
            establishment_protocol: None,
        })
        .unwrap();

        server.mock_send_to_client(&msg).await;
        let session = connection.create(10, 32, Duration::from_secs(10)).await;

        assert!(matches!(session.unwrap_err(), JaError::JanusError { .. }))
    }
}
