use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust::japrotocol::Jsep;
use jarust::japrotocol::JsepType;
use log::LevelFilter;
use log::SetLoggerError;
use serde_json::json;
use simple_logger::SimpleLogger;
use std::time::Duration;
use tokio::time;

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

    let session = connection.create(10).await?;
    let handle = session.attach("janus.plugin.echotest").await?;

    handle
        .message_with_jsep(
            json!({
                "request": "",
                "audio": false,
                "video": true
            }),
            Jsep {
                jsep_type: JsepType::Offer,
                sdp: "".into(),
            },
        )
        .await?;
    let mut interval = time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
    }
}

fn init_logger() -> Result<(), SetLoggerError> {
    let logger = SimpleLogger::new()
        .with_level(LevelFilter::Trace)
        .with_colors(true)
        .with_module_level("tokio_tungstenite", LevelFilter::Debug)
        .with_module_level("tungstenite", LevelFilter::Debug)
        .with_module_level("want", LevelFilter::Debug)
        .init();

    logger
}
