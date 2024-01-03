# Jarust Make Plugin

A helper macro that creates a trait to extend `JaSession`.

## Requirements

The `make_plugin` will generate code containing `async_trait` and `tokio` with `sync` feature. And surely we need `jarust`.

```toml
[dependencies]
async-trait = "<version>"
jarust = "<version>"
tokio = { version = "<version>", features = ["sync"] }
```

## Usage

```rs
use jarust::prelude::*;
use jarust_make_plugin::make_plugin;

make_plugin!(EchoTest, "janus.plugin.echotest");
```

This will generate this trait:

```rs
#[async_trait::async_trait]
pub trait EchoTest: Attach {
    type Event: Send + Sync + 'static;
    type Handle: From<JaHandle> + std::ops::Deref<Target = JaHandle> + PluginTask;

    fn parse_echo_test_message(message: JaResponse) -> JaResult<Self::Event>;

    async fn attach_echo_test(
        &self,
    ) -> JaResult<(Self::Handle, tokio::sync::mpsc::Receiver<Self::Event>)> {
        let (handle, mut receiver) = self.attach("janus.plugin.echotest").await?;
        let (tx, rx) = tokio::sync::mpsc::channel(CHANNEL_BUFFER_SIZE);
        let join_handle = tokio::spawn(async move {
            while let Some(msg) = receiver.recv().await {
                let msg = Self::parse_echo_test_message(msg)?;
                let _ = tx.send(msg).await;
            }
            Ok::<(), JaError>(())
        });
        let abort_handle = join_handle.abort_handle();
        let mut handle: Self::Handle = handle.into();
        handle.assign_abort(abort_handle);
        Ok((handle, rx))
    }
}
```

Then we can extend the `JaSession` by providing the return Handle type (that should confirm to certain bounds), Event type, and how to parse the events.

```rs
impl EchoTest for JaSession {
    type Event = EchoTestPluginEvent;
    type Handle = EchoTestHandle;

    fn parse_echo_test_message(message: JaResponse) -> JaResult<Self::Event> {
        let msg = match message.janus {
            JaResponseProtocol::Event(JaEventProtocol::Event { plugin_data, .. }) => {
                serde_json::from_value::<EchoTestPluginData>(plugin_data)?.event
            }
            _ => {
                log::error!("unexpected response");
                return Err(JaError::UnexpectedResponse);
            }
        };
        Ok(msg)
    }
}
```
