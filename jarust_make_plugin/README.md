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

```rust
use jarust::prelude::*;
use jarust_make_plugin::make_plugin;

make_plugin!(EchoTest, "janus.plugin.echotest");
```

This will generate this trait:

```rust
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

We'll start with creating the `EchoTestHandle`.

```rust
pub struct EchoTestHandle {
    handle: JaHandle,
    abort_handle: Option<AbortHandle>,
}
```

Then we can implement the required trait for `EchoTestHandle` to be able to use it as a plugin handle.

```rust
// This trait is used to assign an abort handle so we can abort the task on drop
impl PluginTask for EchoTestHandle {
    fn assign_abort(&mut self, abort_handle: AbortHandle) {
        self.abort_handle = Some(abort_handle);
    }

    fn abort_plugin(&mut self) {
        if let Some(abort_handle) = self.abort_handle.take() {
            abort_handle.abort();
        };
    }
}

// Create EchoTestHandle from JaHandle
impl From<JaHandle> for EchoTestHandle {
    fn from(handle: JaHandle) -> Self {
        Self {
            handle,
            abort_handle: None,
        }
    }
}

// Dereference EchoTestHandle to JaHandle to get the existing fucntionalities from JaHandle
impl Deref for EchoTestHandle {
    type Target = JaHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

// On drop abort task
impl Drop for EchoTestHandle {
    fn drop(&mut self) {
        self.abort_plugin();
    }
}
```

And add the `EchoTestHandle` specific requests. (The audio bridge for example could have `mute` and `unmute` requests)

```rust
// EchoTestHandle specific requests
impl EchoTestHandle {
    pub async fn start(&self, request: EchoTestStartMsg) -> JaResult<()> {
        self.handle.message(serde_json::to_value(request)?).await
    }
}
```

Finally, we implement the `EchoTest` trait that was created from `make_plugin!` macro on `JaSession` to extend the
`JaSession`'s functionality. And provide the Handle type, Event type, and how to parse the incoming events.

```rust
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
