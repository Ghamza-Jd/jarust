use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust_plugins::echotest::events::EchoTestPluginEvent;
use jarust_plugins::echotest::messages::EchoTestStartMsg;
use jarust_plugins::echotest::EchoTest;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("jarust=trace".parse()?))
        .init();
    let timeout = std::time::Duration::from_secs(10);

    let mut connection = jarust::connect(
        JaConfig::new("ws://localhost:8188/ws", None, "janus"),
        TransportType::Ws,
    )
    .await?;
    let session = connection.create(10).await?;
    let (handle, mut event_receiver, ..) = session.attach_echo_test().await?;
    let handle = handle.clone();

    handle
        .start(
            EchoTestStartMsg {
                audio: true,
                video: true,
                ..Default::default()
            },
            timeout,
        )
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
