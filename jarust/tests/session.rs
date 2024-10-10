mod fixtures;
mod mocks;

#[cfg(test)]
mod tests {
    use crate::mocks::mock_generate_transaction::MockGenerateTransaction;
    use crate::mocks::mock_interface::MockInterface;
    use jarust::jaconnection::CreateConnectionParams;
    use jarust::japlugin::AttachHandleParams;
    use jarust::prelude::Attach;
    use jarust::prelude::JaResponse;
    use jarust_interface::janus_interface::ConnectionParams;
    use jarust_interface::janus_interface::JanusInterface;
    use jarust_interface::japrotocol::ErrorResponse;
    use jarust_interface::japrotocol::JaData;
    use jarust_interface::japrotocol::JaSuccessProtocol;
    use jarust_interface::japrotocol::ResponseType;
    use std::time::Duration;

    #[tokio::test]
    async fn it_successfully_attach_to_handle() {
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
        let mut connection = jarust::custom_connect(interface.clone()).await.unwrap();

        let session_id = 73;

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
            .create_session(CreateConnectionParams {
                ka_interval: 10,
                timeout: Duration::from_secs(10),
            })
            .await
            .unwrap();

        let response = JaResponse {
            janus: ResponseType::Success(JaSuccessProtocol::Data {
                data: JaData { id: 3 },
            }),
            transaction: Some("mock-attach-plugin-transaction".to_string()),
            session_id: Some(session_id),
            sender: None,
            estproto: None,
        };
        interface.mock_attach_rsp(response).await;

        let _ = session
            .attach(AttachHandleParams {
                plugin_id: "mock.plugin.test".to_string(),
                timeout: Duration::from_secs(5),
            })
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn it_fails_to_attach_session() {
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
        let mut connection = jarust::custom_connect(interface.clone()).await.unwrap();

        let session_id = 73;

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
            .create_session(CreateConnectionParams {
                ka_interval: 10,
                timeout: Duration::from_secs(10),
            })
            .await
            .unwrap();

        let response = JaResponse {
            janus: ResponseType::Error {
                error: ErrorResponse {
                    code: 0,
                    reason: "".to_string(),
                },
            },
            transaction: Some("mock-attach-plugin-transaction".to_string()),
            session_id: Some(session_id),
            sender: None,
            estproto: None,
        };
        interface.mock_attach_rsp(response).await;

        let result = session
            .attach(AttachHandleParams {
                plugin_id: "mock.plugin.test".to_string(),
                timeout: Duration::from_secs(5),
            })
            .await;
        assert!(matches!(
            result,
            Err(jarust_interface::Error::JanusError { .. })
        ));
    }
}
