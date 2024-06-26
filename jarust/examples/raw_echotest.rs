use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust::japlugin::Attach;
use jarust::TransactionGenerationStrategy;
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
    let mut connection = jarust::connect(
        config,
        TransportType::Ws,
        TransactionGenerationStrategy::Random,
    )
    .await?;
    let timeout = Duration::from_secs(10);
    let session = connection.create(10, capacity, timeout).await?;
    let (handle, mut event_receiver) = session
        .attach("janus.plugin.echotest", capacity, timeout)
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
