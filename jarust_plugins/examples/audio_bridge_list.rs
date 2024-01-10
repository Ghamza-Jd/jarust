use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust::japrotocol::JaData;
use jarust::japrotocol::JaEventProtocol;
use jarust::japrotocol::JaResponse;
use jarust::japrotocol::JaResponseProtocol;
use jarust::japrotocol::JaSuccessProtocol;
use jarust_plugins::audio_bridge::events::AudioBridgePluginEvent;
use jarust_plugins::audio_bridge::AudioBridge;
use jarust_plugins::echotest::events::EchoTestPluginEvent;
use jarust_plugins::echotest::messages::EchoTestStartMsg;
use jarust_plugins::echotest::EchoTest;
use log::LevelFilter;
use log::SetLoggerError;
use simple_logger::SimpleLogger;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    init_logger()?;

    let mut connection = jarust::connect(JaConfig::new(
        "wss://janus.conf.meetecho.com/ws",
        None,
        TransportType::Wss,
        "janus",
    ))
    .await?;
    let session = connection.create(10).await?;
    let (handle, ..) = session.attach_audio_bridge().await?;

    let result = handle.list().await?;
    log::info!("Rooms {:#?}", result);

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
