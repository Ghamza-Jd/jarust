# Jarust Plugins

A list of predefined plugins.

Current plugins:

- [x] Echo test
- [x] Audio bridge
- [ ] Video room

## EchoTest Example

```rust
use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust_plugins::echotest::events::EchoTestPluginEvent;
use jarust_plugins::echotest::messages::EchoTestStartMsg;
use jarust_plugins::echotest::EchoTest;
use log::LevelFilter;
use log::SetLoggerError;
use simple_logger::SimpleLogger;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    init_logger()?;

    let mut connection = jarust::connect(
        JaConfig::new("ws://localhost:8188/ws", None, "janus"),
        TransportType::Ws,
    )
    .await?;
    let session = connection.create(10).await?;
    let (handle, mut event_receiver, ..) = session.attach_echo_test().await?;

    handle
        .start(EchoTestStartMsg {
            audio: true,
            video: true,
            ..Default::default()
        })
        .await?;

    while let Some(event) = event_receiver.recv().await {
        match event {
            EchoTestPluginEvent::Result { result, .. } => {
                tracing::info!("result: {result}");
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
        .with_module_level("rustls", LevelFilter::Off)
        .init()
}
```

## AudioBridge Example

```rust
use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust_plugins::audio_bridge::AudioBridge;
use log::LevelFilter;
use log::SetLoggerError;
use simple_logger::SimpleLogger;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    init_logger()?;

    let mut connection = jarust::connect(
        JaConfig::new("ws://localhost:8188/ws", None, "janus"),
        TransportType::Ws,
    )
    .await?;
    let session = connection.create(10).await?;
    let (handle, ..) = session.attach_audio_bridge().await?;

    let result = handle.list().await?;
    tracing::info!("Rooms {:#?}", result);

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
```
