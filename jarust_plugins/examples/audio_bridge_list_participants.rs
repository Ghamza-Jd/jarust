use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust_plugins::audio_bridge::messages::AudioBridgeCreateOptions;
use jarust_plugins::audio_bridge::AudioBridge;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("jarust=trace".parse()?))
        .init();

    let mut connection = jarust::connect(
        JaConfig::new("ws://localhost:8188/ws", None, "janus"),
        TransportType::Ws,
    )
    .await?;
    let session = connection.create(10).await?;
    let (handle, ..) = session.attach_audio_bridge().await?;

    let (room, permanent) = handle
        .create_room_with_config(AudioBridgeCreateOptions {
            secret: Some("superdupersecret".to_string()),
            ..Default::default()
        })
        .await?;
    tracing::info!("Created Room {}, permanent: {}", room, permanent);

    let (room, participants) = handle.list_participants(room).await?;
    tracing::info!("Participants in room {}: {:#?}", room, participants);

    Ok(())
}
