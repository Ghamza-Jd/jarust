use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust::japlugin::Attach;
use serde_json::json;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("jarust=trace".parse()?))
        .init();

    let mut connection = jarust::connect(
        JaConfig::new("wss://janus.conf.meetecho.com/ws", None, "janus"),
        TransportType::Ws,
    )
    .await?;
    let session = connection.create(10).await?;
    let (handle, mut event_receiver) = session.attach("janus.plugin.echotest").await?;

    handle
        .message(json!({
            "video": true,
            "audio": true,
        }))
        .await?;

    while let Some(event) = event_receiver.recv().await {
        tracing::info!("response: {event:#?}");
    }

    Ok(())
}
