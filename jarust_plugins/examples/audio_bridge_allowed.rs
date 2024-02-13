use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust_plugins::audio_bridge::messages::AudioBridgeAction;
use jarust_plugins::audio_bridge::messages::AudioBridgeAllowedOptions;
use jarust_plugins::audio_bridge::messages::AudioBridgeCreateOptions;
use jarust_plugins::audio_bridge::AudioBridge;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

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

    let (room, allowed_participants) = handle
        .allowed(
            room,
            AudioBridgeAction::Add,
            vec![],
            AudioBridgeAllowedOptions {
                secret: Some("superdupersecret".to_string()),
            },
        )
        .await?;

    tracing::info!(
        "Allowed participants in room {}: {:#?}",
        room,
        allowed_participants
    );

    Ok(())
}
