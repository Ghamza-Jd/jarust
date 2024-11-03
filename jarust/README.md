# Jarust

Jarust is a memory safe and high-performance Rust adapter for [Janus WebRTC server](https://github.com/meetecho/janus-gateway).

Inspired by [Janode](https://github.com/meetecho/janode), jarust offers similar functionalities but it's designed
to be customizable, for exmaple, you could use the built-in WebSocket transport or provide your own RabbitMQ transport implementation.

The crate wraps the Janus core API and some of the most popular plugins APIs.

## Example Usage

```rust
use jarust::core::jaconfig::JaConfig;
use jarust::core::jaconfig::JanusAPI;
use jarust::core::prelude::Attach;
use jarust::interface::tgenerator::RandomTransactionGenerator;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let config = JaConfig {
        url: "ws://localhost:8188/ws".to_string(),
        apisecret: None,
        server_root: "janus".to_string(),
        capacity: 32,
    };
    let mut connection =
        jarust::core::connect(config, JanusAPI::WebSocket, RandomTransactionGenerator)
            .await
            .unwrap();

    let info = connection
        .server_info(Duration::from_secs(5))
        .await
        .unwrap();
    println!("{:#?}", info);

    let session = connection
        .create_session(10, Duration::from_secs(5))
        .await
        .unwrap();

    let (handle, _) = session
        .attach("janus.plugin.echotest".to_string(), Duration::from_secs(5))
        .await
        .unwrap();
}
```
