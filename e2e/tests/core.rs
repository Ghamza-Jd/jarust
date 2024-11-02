use jarust::core::jaconfig::JaConfig;
use jarust::core::jaconfig::JanusAPI;
use jarust::core::prelude::Attach;
use jarust::interface::tgenerator::RandomTransactionGenerator;
use std::time::Duration;

#[tokio::test]
async fn it_websocket_core_tests() {
    let config = JaConfig {
        url: "ws://localhost:8188/ws".to_string(),
        apisecret: None,
        server_root: "janus".to_string(),
        capacity: 32,
    };
    let mut connection =
        jarust::core::connect(config, JanusAPI::WebSocket, RandomTransactionGenerator)
            .await
            .unwrap();

    let info = connection
        .server_info(Duration::from_secs(5))
        .await
        .unwrap();
    assert_eq!(info.server_name, "Jarust".to_string());

    let session = connection
        .create_session(10, Duration::from_secs(5))
        .await
        .unwrap();

    let (_, _) = session
        .attach("janus.plugin.echotest".to_string(), Duration::from_secs(5))
        .await
        .unwrap();
}
