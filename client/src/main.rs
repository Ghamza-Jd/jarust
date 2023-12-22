use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use log::LevelFilter;
use log::SetLoggerError;
use serde_json::json;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger()?;

    let mut connection = jarust::connect(JaConfig::new(
        "wss://janus.conf.meetecho.com/ws",
        None,
        TransportType::Wss,
        "janus",
    ))
    .await?;

    // let server_info = connection.server_info().await?;

    let session = connection.create(10).await?;
    let (handle, mut event_receiver) = session.attach("janus.plugin.echotest").await?;

    handle
        .message(json!({
            "audio": true,
            "video": true
        }))
        .await?;

    handle.detach().await?;

    while let Some(event) = event_receiver.recv().await {
        log::info!("{event}");
    }

    Ok(())
}

fn init_logger() -> Result<(), SetLoggerError> {
    let logger = SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .with_colors(true)
        .with_module_level("tokio_tungstenite", LevelFilter::Off)
        .with_module_level("tungstenite", LevelFilter::Off)
        .with_module_level("want", LevelFilter::Off)
        .init();

    logger
}
