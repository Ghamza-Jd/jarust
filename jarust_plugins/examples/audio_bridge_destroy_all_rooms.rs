use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust_plugins::audio_bridge::messages::AudioBridgeDestroyOptions;
use jarust_plugins::audio_bridge::AudioBridge;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("jarust=trace".parse()?))
        .init();
    let timeout = std::time::Duration::from_secs(10);

    let config = JaConfig::builder().url("ws://localhost:8188/ws").build();
    let mut connection = jarust::connect(config, TransportType::Ws).await?;
    let session = connection.create(10).await?;
    let (handle, ..) = session.attach_audio_bridge().await?;

    let list = handle.list(timeout).await?;

    tracing::info!("Rooms to destroy {:#?}", list);

    for item in list {
        if let Ok((room, ..)) = handle
            .destroy_room(
                item.room,
                AudioBridgeDestroyOptions {
                    secret: Some("superdupersecret".to_string()),
                    ..Default::default()
                },
                timeout,
            )
            .await
        {
            tracing::info!("Destroyed Room {}", room);
        };
    }

    Ok(())
}
