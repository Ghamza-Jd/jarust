use jarust::core::connect;
use jarust::core::jaconfig::JaConfig;
use jarust::core::jaconfig::JanusAPI;
use jarust::core::prelude::Attach;
use jarust::interface::tgenerator::RandomTransactionGenerator;
use std::time::Duration;

#[allow(unused_labels)]
#[tokio::test]
async fn it_websocket_core_tests() {
    e2e::init_tracing_subscriber();
    let config = JaConfig {
        url: "ws://localhost:8188/ws".to_string(),
        apisecret: None,
        server_root: "janus".to_string(),
        capacity: 32,
    };
    let mut connection = connect(config, JanusAPI::WebSocket, RandomTransactionGenerator)
        .await
        .unwrap();

    'server_info: {
        let info = connection
            .server_info(Duration::from_secs(5))
            .await
            .unwrap();
        assert_eq!(
            info.server_name,
            "Jarust".to_string(),
            "Server name should match the one in server_config/janus.cfg"
        );
    }

    'destroyed_session: {
        let session = connection
            .create_session(10, Duration::from_secs(5))
            .await
            .unwrap();

        session.destroy(Duration::from_secs(5)).await.unwrap();

        let result = session
            .attach("janus.plugin.echotest".to_string(), Duration::from_secs(5))
            .await;
        assert!(
            matches!(
                result,
                Err(jarust::interface::error::Error::JanusError { code: _, reason: _ })
            ),
            "No such session after destroying it"
        )
    }

    let session = connection
        .create_session(10, Duration::from_secs(5))
        .await
        .unwrap();

    let (_, _) = session
        .attach("janus.plugin.echotest".to_string(), Duration::from_secs(5))
        .await
        .unwrap();
}
