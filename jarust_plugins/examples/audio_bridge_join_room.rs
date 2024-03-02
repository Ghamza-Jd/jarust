use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust_plugins::audio_bridge::messages::AudioBridgeCreateOptions;
use jarust_plugins::audio_bridge::messages::AudioBridgeJoinOptions;
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
    let (handle, mut event_receiver) = session.attach_audio_bridge().await?;

    let (room, permanent) = handle
        .create_room_with_config(
            AudioBridgeCreateOptions {
                secret: Some("superdupersecret".to_string()),
                ..Default::default()
            },
            timeout,
        )
        .await?;
    tracing::info!("Created Room {}, permanent: {}", room, permanent);

    let _ = handle
        .join_room(
            room,
            AudioBridgeJoinOptions {
                secret: Some("superdupersecret".to_string()),
                generate_offer: Some(true),
                ..Default::default()
            },
            None,
            timeout,
        )
        .await?;

    if let Some((event, protocol)) = event_receiver.recv().await {
        tracing::info!("Joined Room {}, {:#?}, {:#?}", room, event, protocol);
    }

    Ok(())
}
