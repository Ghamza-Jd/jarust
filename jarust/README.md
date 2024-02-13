# Jarust

The core of jarust.

It handles:

- Connection
- Sessions
- Handles
- Send messages
- Receive events
- Demultiplexing

## Plugins

The recommended way to create a plugin is to use `make_plugin` macro from [jarust_make_plugin](https://crates.io/crates/jarust_make_plugin).

Checkout the existing plugins: [jarust_plugins](https://crates.io/crates/jarust_plugins)

## Example usage

```rust
use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust::japlugin::Attach;
use log::LevelFilter;
use log::SetLoggerError;
use serde_json::json;
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
    let (handle, mut event_receiver) = session.attach("janus.plugin.echotest").await?;

    handle
        .message(json!({
            "video": true,
            "audio": true,
        }))
        .await?;

    while let Some(event) = event_receiver.recv().await {
        tracing::info!("response: {event:?}");
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
