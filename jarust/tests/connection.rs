mod fixtures;
mod mocks;

#[cfg(test)]
mod tests {
    use crate::mocks::mock_generate_transaction::MockGenerateTransaction;
    use crate::mocks::mock_interface::MockInterface;
    use jarust::prelude::JaResponse;
    use jarust_interface::janus_interface::ConnectionParams;
    use jarust_interface::janus_interface::JanusInterface;
    use jarust_interface::japrotocol::ErrorResponse;
    use jarust_interface::japrotocol::JaData;
    use jarust_interface::japrotocol::JaSuccessProtocol;
    use jarust_interface::japrotocol::ResponseType;
    use jarust_interface::japrotocol::ServerInfoRsp;
    use std::collections::HashMap;
    use std::time::Duration;

    #[tokio::test]
    async fn it_successfully_connects() {
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
        let connection = jarust::custom_connect(interface).await;
        assert!(connection.is_ok());
    }

    #[tokio::test]
    async fn it_successfully_creates_session() {
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

        let response = JaResponse {
            janus: ResponseType::Success(JaSuccessProtocol::Data {
                data: JaData { id: 73 },
            }),
            transaction: Some("abc123".to_string()),
            session_id: None,
            sender: None,
            estproto: None,
        };

        interface.mock_create_rsp(response).await;

        let session = connection.create_session(10, Duration::from_secs(10)).await;

        assert!(session.is_ok());
    }

    #[tokio::test]
    async fn it_successfully_receives_server_info() {
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

        let server_info = ServerInfoRsp {
            name: "Mock server name".to_string(),
            version: 0,
            version_string: "0.1.0".to_string(),
            author: "John Doe".to_string(),
            commit_hash: "abc123".to_string(),
            compile_time: "2021-01-01".to_string(),
            log_to_stdout: true,
            log_to_file: true,
            data_channels: true,
            accepting_new_sessions: true,
            session_timeout: 90,
            reclaim_session_timeout: 60,
            candidates_timeout: 60,
            server_name: "Mock server".to_string(),
            local_ip: "127.0.0.1".to_string(),
            ipv6: true,
            ice_lite: true,
            ice_tcp: true,
            ice_nomination: "".to_string(),
            ice_keepalive_conncheck: true,
            full_trickle: true,
            mdns_enabled: true,
            min_nack_queue: 10,
            twcc_period: 60,
            dtls_mtu: 1300,
            static_event_loops: 10,
            api_secret: false,
            auth_token: false,
            event_handlers: true,
            opaqueid_in_api: true,
            dependencies: HashMap::new(),
            transports: HashMap::new(),
            plugins: HashMap::new(),
        };

        interface.mocker_server_info_rsp(server_info).await;

        let rsp = connection.server_info(Duration::from_secs(5)).await;

        assert!(rsp.is_ok());
    }

    #[tokio::test]
    async fn it_fails_to_create_session_with_janus_error() {
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

        let response = JaResponse {
            janus: ResponseType::Error {
                error: ErrorResponse {
                    code: 0,
                    reason: "".to_string(),
                },
            },
            transaction: Some("abc123".to_string()),
            session_id: None,
            sender: None,
            estproto: None,
        };

        interface.mock_create_rsp(response).await;

        let session = connection.create_session(10, Duration::from_secs(10)).await;

        assert!(matches!(
            session.unwrap_err(),
            jarust_interface::Error::JanusError { .. }
        ))
    }
}
