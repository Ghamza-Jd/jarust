use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust_plugins::audio_bridge::messages::AudioBridgeDestroyOptions;
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

    let list = handle.list().await?;

    log::info!("Rooms to delete {:#?}", list);

    for item in list {
        if let Ok((room, ..)) = handle
            .destroy_room(
                item.room,
                AudioBridgeDestroyOptions {
                    secret: Some("superdupersecret".to_string()),
                    ..Default::default()
                },
            )
            .await
        {
            log::info!("Deleted Room {}", room);
        };
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
