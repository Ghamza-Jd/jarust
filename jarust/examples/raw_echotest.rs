use jarust::jaconfig::JaConfig;
use jarust::jaconfig::JanusAPI;
use jarust::prelude::Attach;
use jarust_interface::tgenerator::RandomTransactionGenerator;
use serde_json::json;
use std::time::Duration;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("jarust=trace".parse()?))
        .init();
    let config = JaConfig {
        url: "ws://localhost:8188/ws".to_string(),
        apisecret: None,
        server_root: "janus".to_string(),
        capacity: 32,
    };
    let mut connection =
        jarust::connect(config, JanusAPI::WebSocket, RandomTransactionGenerator).await?;
    let timeout = Duration::from_secs(10);
    let session = connection
        .create_session(10, Duration::from_secs(10))
        .await?;
    let (handle, mut event_receiver) = session
        .attach("janus.plugin.echotest".to_string(), timeout)
        .await?;

    handle
        .send_waiton_ack(
            json!({
                "video": true,
                "audio": true,
            }),
            Duration::from_secs(2),
        )
        .await?;

    while let Some(event) = event_receiver.recv().await {
        tracing::info!("response: {event:#?}");
    }

    Ok(())
}
