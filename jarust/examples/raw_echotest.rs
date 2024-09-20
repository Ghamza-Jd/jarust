use jarust::jaconfig::ApiInterface;
use jarust::jaconfig::JaConfig;
use jarust::jaconnection::CreateConnectionParams;
use jarust::japlugin::AttachHandleParams;
use jarust::prelude::Attach;
use jarust_transport::transaction_gen::RandomTransactionGenerator;
use serde_json::json;
use std::time::Duration;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("jarust=trace".parse()?))
        .init();
    let capacity = 32;
    let config = JaConfig::builder()
        .url("ws://localhost:8188/ws")
        .capacity(capacity)
        .build();
    let mut connection =
        jarust::connect(config, ApiInterface::WebSocket, RandomTransactionGenerator).await?;
    let timeout = Duration::from_secs(10);
    let session = connection
        .create(CreateConnectionParams {
            ka_interval: 10,
            timeout,
        })
        .await?;
    let (handle, mut event_receiver) = session
        .attach(AttachHandleParams {
            plugin_id: "janus.plugin.echotest".to_string(),
            timeout,
        })
        .await?;

    handle
        .send_waiton_ack(
            json!({
                "video": true,
                "audio": true,
            }),
            std::time::Duration::from_secs(2),
        )
        .await?;

    while let Some(event) = event_receiver.recv().await {
        tracing::info!("response: {event:#?}");
    }

    Ok(())
}
