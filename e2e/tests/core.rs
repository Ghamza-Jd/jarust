use jarust::jaconfig::JaConfig;
use jarust::jaconfig::JanusAPI;
use jarust_interface::tgenerator::RandomTransactionGenerator;
use std::time::Duration;

#[tokio::test]
async fn it_gets_server_info() {
    let config = JaConfig {
        url: "ws://localhost:8188/ws".to_string(),
        apisecret: None,
        server_root: "janus".to_string(),
        capacity: 32,
    };
    let mut connection = jarust::connect(config, JanusAPI::WebSocket, RandomTransactionGenerator)
        .await
        .unwrap();
    let timeout = Duration::from_secs(10);
    let info = connection.server_info(timeout).await.unwrap();
    assert_eq!(info.server_name, "Jarust".to_string());
}
