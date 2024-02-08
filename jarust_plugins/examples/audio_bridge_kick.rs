use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust_plugins::audio_bridge::events::AudioBridgePluginEvent;
use jarust_plugins::audio_bridge::messages::AudioBridgeCreateOptions;
use jarust_plugins::audio_bridge::messages::AudioBridgeJoinOptions;
use jarust_plugins::audio_bridge::messages::AudioBridgeKickOptions;
use jarust_plugins::audio_bridge::AudioBridge;
use log::LevelFilter;
use log::SetLoggerError;
use simple_logger::SimpleLogger;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    init_logger()?;

    let mut connection = jarust::connect(
        JaConfig::new("ws://localhost:8188/ws", None, "janus"),
        TransportType::Wss,
    )
    .await?;
    let session = connection.create(10).await?;
    let (handle, mut event_receiver) = session.attach_audio_bridge().await?;

    let (room, permanent) = handle
        .create_room_with_config(AudioBridgeCreateOptions {
            secret: Some("superdupersecret".to_string()),
            ..Default::default()
        })
        .await?;
    log::info!("Created Room {}, permanent: {}", room, permanent);

    let _ = handle
        .join_room(
            room,
            AudioBridgeJoinOptions {
                secret: Some("superdupersecret".to_string()),
                generate_offer: Some(true),
                ..Default::default()
            },
            None,
        )
        .await?;

    if let Some((event, ..)) = event_receiver.recv().await {
        match event {
            AudioBridgePluginEvent::JoinRoom { id, room, .. } => {
                log::info!("Joined room {}, Paricipant id: {}", room, id);

                let kick_result = handle
                    .kick(
                        room,
                        id,
                        AudioBridgeKickOptions {
                            secret: Some("superdupersecret".to_string()),
                        },
                    )
                    .await;
                if let Ok(()) = kick_result {
                    log::info!("Paricipant {} Kicked", id);
                }
            }
        }
    }

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
