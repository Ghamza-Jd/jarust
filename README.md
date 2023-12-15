# About Jarust

Jarust is a Rust adapter for [Janus WebRTC server](https://github.com/meetecho/janus-gateway)

Interally uses WebSockets to connect to Janus.

The library wraps the Janus core API and some of the most popular plugins APIs.

## Example Usage

This is just a pretty simple hello world for the echotest plugin.

```rs
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut connection = jarust::connect(JaConfig::new(
        "wss://janus.conf.meetecho.com/ws",
        None,
        TransportType::Wss,
        "janus",
    ))
    .await?;

    let session = connection.create(10).await?;
    let handle = session.attach("janus.plugin.echotest").await?;
}
```
