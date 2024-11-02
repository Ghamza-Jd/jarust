# Jarust

The core of jarust.

It under the hood it uses [jarust_interface](https://crates.io/crates/jarust_interface) to provide an abstract api
for connecting, creating a session, attaching to a plugin, and then communicate with the plugin handle.

It's also the building block for the plugin crate [jarust_plugins](https://crates.io/crates/jarust_plugins)

## Example usage

```rust
use jarust_core::jaconfig::JaConfig;
use jarust_core::jaconfig::TransportType;
use jarust_core::japlugin::Attach;
use serde_json::json;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("jarust_core=trace".parse()?))
        .init();

    let mut connection = jarust_core::connect(
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
        tracing::info!("response: {event:#?}");
    }

    Ok(())
}
```
