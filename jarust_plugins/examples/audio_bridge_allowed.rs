use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust_plugins::audio_bridge::messages::AudioBridgeAction;
use jarust_plugins::audio_bridge::messages::AudioBridgeAllowedOptions;
use jarust_plugins::audio_bridge::messages::AudioBridgeCreateOptions;
use jarust_plugins::audio_bridge::AudioBridge;
use log::LevelFilter;
use log::SetLoggerError;
use simple_logger::SimpleLogger;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    init_logger()?;

    let mut connection = jarust::connect(JaConfig::new(
        "ws://localhost:8188/ws",
        None,
        TransportType::Wss,
        "janus",
    ))
    .await?;
    let session = connection.create(10).await?;
    let (handle, ..) = session.attach_audio_bridge().await?;

    let (room, permanent) = handle
        .create_room_with_config(AudioBridgeCreateOptions {
            secret: Some("superdupersecret".to_string()),
            ..Default::default()
        })
        .await?;
    log::info!("Created Room {}, permanent: {}", room, permanent);

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

    log::info!(
        "Allowed participants in room {}: {:#?}",
        room,
        allowed_participants
    );

    Ok(())
}

fn init_logger() -> Result<(), SetLoggerError> {
    SimpleLogger::new()
        .with_level(LevelFilter::Trace)
        .with_colors(true)
        .with_module_level("tokio_tungstenite", LevelFilter::Off)
        .with_module_level("tungstenite", LevelFilter::Off)
        .with_module_level("want", LevelFilter::Off)
        .init()
}
