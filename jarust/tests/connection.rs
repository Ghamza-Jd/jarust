mod fixtures;
mod mocks;

#[cfg(test)]
mod tests {
    use crate::mocks::mock_connection::mock_connection;
    use crate::mocks::mock_generate_transaction::MockGenerateTransaction;
    use crate::mocks::mock_interface::MockInterface;
    use jarust::error::JaError;
    use jarust::jaconnection::CreateConnectionParams;
    use jarust_transport::error::JaTransportError;
    use jarust_transport::interface::janus_interface::ConnectionParams;
    use jarust_transport::interface::janus_interface::JanusInterface;
    use jarust_transport::japrotocol::ErrorResponse;
    use jarust_transport::japrotocol::JaData;
    use jarust_transport::japrotocol::JaResponse;
    use jarust_transport::japrotocol::JaSuccessProtocol;
    use jarust_transport::japrotocol::ResponseType;
    use std::time::Duration;

    #[tokio::test]
    async fn it_successfully_connects() {
        let conn_params = ConnectionParams {
            url: "mock://some.janus.com".to_string(),
            capacity: 10,
            apisecret: None,
            namespace: "mock".to_string(),
        };
        let transaction_generator = MockGenerateTransaction::new();
        let interface = MockInterface::make_interface(conn_params, transaction_generator)
            .await
            .unwrap();
        let connection = jarust::custom_connect(interface).await;
        assert!(connection.is_ok());
    }

    #[tokio::test]
    async fn it_successfully_creates_session() {
        let (interface, server) = MockInterface::interface_server_pair().await.unwrap();
        let mut generator = MockGenerateTransaction::new();
        generator.next_transaction("abc123");
        let mut connection = mock_connection(interface).await.unwrap();

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
        let session = connection
            .create(CreateConnectionParams {
                ka_interval: 10,
                timeout: Duration::from_secs(10),
            })
            .await;

        assert!(session.is_ok());
    }

    #[tokio::test]
    async fn it_fails_to_create_session_with_janus_error() {
        let (interface, server) = MockInterface::interface_server_pair().await.unwrap();
        let mut generator = MockGenerateTransaction::new();
        generator.next_transaction("abc123");
        let mut connection = mock_connection(interface).await.unwrap();

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
        let session = connection
            .create(CreateConnectionParams {
                ka_interval: 10,
                timeout: Duration::from_secs(10),
            })
            .await;

        assert!(matches!(
            session.unwrap_err(),
            JaError::JanusTransport(JaTransportError::JanusError { .. })
        ))
    }
}
